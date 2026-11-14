//! Keyed, variable-height transcript viewport backed by GPUI's native tail following.
//!
//! Hosts own content and session storage. Keep this view as an entity, call
//! `notify_item_changed` when a row's height changes, and notify its context
//! after changing the item roster or restoring a saved position.

use gpui::{
    AnyElement, App, Context, FollowMode, IntoElement, ListAlignment, ListOffset, ListState,
    Render, SharedString, Window, div, list, prelude::*, px,
};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, fmt, ops::Range, rc::Rc};

type ItemRenderer = dyn Fn(&SharedString, &mut Window, &mut App) -> AnyElement;

/// Portable session position, anchored to item identity rather than an index.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TranscriptPosition {
    /// First visible item, or no item for an empty/following viewport.
    pub item: Option<String>,
    /// Distance into the first visible item in logical pixels.
    pub offset: f32,
    /// Resume following the tail rather than restoring an old item.
    pub following_tail: bool,
}

/// Invalid roster or position. An error leaves the existing viewport unchanged.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TranscriptError {
    /// Item identities must be unique within a transcript.
    DuplicateItem(SharedString),
    /// The requested anchor or changed item is not in the current roster.
    MissingItem(SharedString),
    /// Offsets must be finite and non-negative.
    InvalidOffset,
}

impl fmt::Display for TranscriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateItem(id) => write!(f, "duplicate transcript item: {id}"),
            Self::MissingItem(id) => write!(f, "unknown transcript item: {id}"),
            Self::InvalidOffset => f.write_str("transcript offset must be finite and non-negative"),
        }
    }
}

impl std::error::Error for TranscriptError {}

/// Native virtual viewport with stable item IDs, prepend anchors and tail following.
pub struct Transcript {
    items: Rc<[SharedString]>,
    indices: HashMap<SharedString, usize>,
    state: ListState,
    render_item: Rc<ItemRenderer>,
    pending_restore: Rc<RefCell<Option<TranscriptPosition>>>,
}

impl Transcript {
    /// Creates an empty transcript. Only visible/overdraw rows invoke the renderer.
    pub fn new(
        render_item: impl Fn(&SharedString, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        let state = ListState::new(0, ListAlignment::Top, px(640.0));
        state.set_follow_mode(FollowMode::Tail);
        Self {
            items: Rc::from([]),
            indices: HashMap::new(),
            state,
            render_item: Rc::new(render_item),
            pending_restore: Rc::new(RefCell::new(None)),
        }
    }

    /// Returns the current item roster in display order.
    pub fn items(&self) -> &[SharedString] {
        &self.items
    }

    /// Replaces the roster, retaining measurement and scroll anchors where possible.
    ///
    /// Appends/prepends splice only the inserted span. A surviving visible item
    /// remains the anchor even when other items are reordered or removed.
    pub fn set_items(&mut self, items: Vec<SharedString>) -> Result<(), TranscriptError> {
        if self.items.as_ref() == items.as_slice() {
            return Ok(());
        }
        let mut indices = HashMap::with_capacity(items.len());
        for (index, key) in items.iter().enumerate() {
            if indices.insert(key.clone(), index).is_some() {
                return Err(TranscriptError::DuplicateItem(key.clone()));
            }
        }
        let old_offset = self.state.logical_scroll_top();
        let anchor = self.items.get(old_offset.item_ix);
        let following = self.state.is_following_tail();
        let (removed, inserted) = changed_span(&self.items, &items);
        self.state.splice(removed, inserted);
        if !following && let Some(new_index) = anchor.and_then(|key| indices.get(key)) {
            self.state.scroll_to(ListOffset {
                item_ix: *new_index,
                offset_in_item: old_offset.offset_in_item,
            });
        }
        self.items = items.into();
        self.indices = indices;
        if self
            .pending_restore
            .borrow()
            .as_ref()
            .is_some_and(|position| {
                position
                    .item
                    .as_deref()
                    .is_some_and(|id| !self.indices.contains_key(id))
            })
        {
            self.pending_restore.borrow_mut().take();
        }
        Ok(())
    }

    /// Invalidates one growing/expanded row without resetting selection or scroll.
    pub fn notify_item_changed(&self, id: &str) -> Result<(), TranscriptError> {
        let index = self.index_of(id)?;
        self.state.remeasure_items(index..index + 1);
        Ok(())
    }

    /// Replaces a host renderer and invalidates existing row measurements.
    pub fn set_render_item(
        &mut self,
        render_item: impl Fn(&SharedString, &mut Window, &mut App) -> AnyElement + 'static,
    ) {
        self.render_item = Rc::new(render_item);
        self.state.remeasure_items(0..self.items.len());
    }

    /// Returns whether native scrolling is currently following newly appended content.
    pub fn is_following_tail(&self) -> bool {
        self.state.is_following_tail()
    }

    /// Jumps to the latest content and re-enables native tail following.
    pub fn follow_tail(&self) {
        self.pending_restore.borrow_mut().take();
        self.state.set_follow_mode(FollowMode::Tail);
    }

    /// Freezes the current position; scrolling back to the bottom resumes following.
    pub fn pause_following(&self) {
        self.state.pause_following_tail();
    }

    /// Scrolls to a named row, leaving tail-following until the user returns to bottom.
    pub fn reveal_item(&self, id: &str) -> Result<(), TranscriptError> {
        let index = self.index_of(id)?;
        self.pending_restore.borrow_mut().take();
        self.state.scroll_to(ListOffset {
            item_ix: index,
            offset_in_item: px(0.0),
        });
        Ok(())
    }

    /// Captures the current viewport for host-owned session persistence.
    pub fn position(&self) -> TranscriptPosition {
        let offset = self.state.logical_scroll_top();
        let following_tail = self.state.is_following_tail();
        TranscriptPosition {
            item: if following_tail {
                None
            } else {
                self.items.get(offset.item_ix).map(ToString::to_string)
            },
            offset: if following_tail {
                0.0
            } else {
                offset.offset_in_item.into()
            },
            following_tail,
        }
    }

    /// Restores a validated session position after its item roster has been loaded.
    /// The saved pixel offset is clamped to the anchor's current measured height on draw.
    pub fn restore_position(&self, position: &TranscriptPosition) -> Result<(), TranscriptError> {
        if !position.offset.is_finite() || position.offset < 0.0 {
            return Err(TranscriptError::InvalidOffset);
        }
        let index = match position.item.as_deref() {
            Some(key) => Some(self.index_of(key)?),
            None => None,
        };
        if position.following_tail {
            self.follow_tail();
        } else {
            self.state.pause_following_tail();
            self.state.scroll_to(ListOffset {
                item_ix: index.unwrap_or(0),
                offset_in_item: px(position.offset),
            });
            *self.pending_restore.borrow_mut() = Some(position.clone());
        }
        Ok(())
    }

    /// Exposes GPUI's native state for scroll observers and viewport geometry.
    pub fn list_state(&self) -> ListState {
        self.state.clone()
    }

    fn index_of(&self, id: &str) -> Result<usize, TranscriptError> {
        self.indices
            .get(id)
            .copied()
            .ok_or_else(|| TranscriptError::MissingItem(id.to_owned().into()))
    }
}

impl Render for Transcript {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let items = self.items.clone();
        let render_item = self.render_item.clone();
        let pending_restore = self.pending_restore.clone();
        let restore_items = items.clone();
        let restore_renderer = render_item.clone();
        let state = self.state.clone();
        div()
            .relative()
            .size_full()
            .child(
                gpui::canvas(
                    move |bounds, window, cx| {
                        let Some(position) = pending_restore.borrow_mut().take() else {
                            return;
                        };
                        let index = match position.item.as_deref() {
                            Some(id) => restore_items.iter().position(|key| key.as_ref() == id),
                            None => (!restore_items.is_empty()).then_some(0),
                        };
                        if let Some(index) = index {
                            // Measure at the current viewport width before the list lays out its anchor.
                            let mut row = div()
                                .id(restore_items[index].clone())
                                .w_full()
                                .child(restore_renderer(&restore_items[index], window, cx))
                                .into_any_element();
                            let size = row.layout_as_root(
                                gpui::size(
                                    bounds.size.width.into(),
                                    gpui::AvailableSpace::MinContent,
                                ),
                                window,
                                cx,
                            );
                            state.splice(index..index + 1, 1);
                            state.scroll_to(ListOffset {
                                item_ix: index,
                                offset_in_item: px(position.offset).min(size.height),
                            });
                        }
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full(),
            )
            .child(
                list(self.state.clone(), move |index, window, cx| {
                    let key = &items[index];
                    div()
                        .id(key.clone())
                        .w_full()
                        .child(render_item(key, window, cx))
                        .into_any_element()
                })
                .size_full(),
            )
            .child(crate::VirtualScrollbar::new(self.state.clone()))
    }
}

fn changed_span(old: &[SharedString], new: &[SharedString]) -> (Range<usize>, usize) {
    let prefix = old
        .iter()
        .zip(new)
        .take_while(|(left, right)| left == right)
        .count();
    let suffix = old[prefix..]
        .iter()
        .rev()
        .zip(new[prefix..].iter().rev())
        .take_while(|(left, right)| left == right)
        .count();
    (prefix..old.len() - suffix, new.len() - prefix - suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys(values: &[&str]) -> Vec<SharedString> {
        values
            .iter()
            .map(|value| (*value).to_owned().into())
            .collect()
    }

    fn transcript() -> Transcript {
        Transcript::new(|_, _, _| div().into_any_element())
    }

    #[test]
    fn prepend_and_reorder_preserve_visible_item_and_pixel_offset() {
        let mut view = transcript();
        view.set_items(keys(&["a", "b", "c"])).unwrap();
        view.restore_position(&TranscriptPosition {
            item: Some("b".into()),
            offset: 7.0,
            following_tail: false,
        })
        .unwrap();
        view.set_items(keys(&["history", "c", "b", "a"])).unwrap();
        assert_eq!(view.position().item.as_deref(), Some("b"));
        assert_eq!(view.position().offset, 7.0);
        assert_eq!(view.list_state().logical_scroll_top().item_ix, 2);
        assert!(!view.is_following_tail());
    }

    #[test]
    fn invalid_roster_or_position_does_not_mutate_viewport() {
        let mut view = transcript();
        view.set_items(keys(&["a", "b"])).unwrap();
        let before = view.position();
        assert!(matches!(
            view.set_items(keys(&["b", "b"])),
            Err(TranscriptError::DuplicateItem(_))
        ));
        assert_eq!(view.items(), keys(&["a", "b"]));
        assert_eq!(view.position(), before);
        for offset in [f32::NAN, f32::INFINITY, -1.0] {
            assert_eq!(
                view.restore_position(&TranscriptPosition {
                    item: Some("a".into()),
                    offset,
                    following_tail: false,
                }),
                Err(TranscriptError::InvalidOffset)
            );
        }
        assert!(matches!(
            view.restore_position(&TranscriptPosition {
                item: Some("gone".into()),
                offset: 0.0,
                following_tail: false,
            }),
            Err(TranscriptError::MissingItem(_))
        ));
        assert_eq!(view.position(), before);
    }

    #[test]
    fn append_and_height_invalidation_preserve_follow_state() {
        let mut view = transcript();
        view.set_items(keys(&["a"])).unwrap();
        view.set_items(keys(&["a", "b"])).unwrap();
        view.notify_item_changed("b").unwrap();
        assert!(view.is_following_tail());
        view.pause_following();
        view.notify_item_changed("a").unwrap();
        assert!(!view.is_following_tail());
        view.follow_tail();
        assert!(view.is_following_tail());
        assert!(matches!(
            view.notify_item_changed("missing"),
            Err(TranscriptError::MissingItem(_))
        ));
    }

    #[test]
    fn changed_span_retains_unmodified_measurement_regions() {
        assert_eq!(
            changed_span(&keys(&["a", "b"]), &keys(&["x", "a", "b"])),
            (0..0, 1)
        );
        assert_eq!(
            changed_span(&keys(&["a", "b"]), &keys(&["a", "b", "x"])),
            (2..2, 1)
        );
        assert_eq!(
            changed_span(&keys(&["a", "x", "b"]), &keys(&["a", "y", "b"])),
            (1..2, 1)
        );
    }
}
