//! Native workbench layout with serializable tab groups and host-owned content.

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use gpui::{
    AnyElement, App, Context, Entity, IntoElement, ParentElement, Pixels, Render, RenderOnce,
    SharedString, Styled, ViewElement as Component, Window, div, prelude::*, px, relative,
};
use liora_core::Config;
use serde::{Deserialize, Serialize};

/// Edge used by dock panels around the center document area.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DockEdge {
    /// Left side of the workbench.
    Left,
    /// Right side of the workbench.
    Right,
    /// Top side of the workbench.
    Top,
    /// Bottom side of the workbench.
    Bottom,
    /// Beside the center document groups.
    Center,
}

/// Panel placement persisted by [`DockState`]. Content and titles remain host-owned.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DockPanelPlacement {
    /// Stable panel key.
    pub key: String,
    /// Workbench edge.
    pub edge: DockEdge,
    /// Preferred edge-axis size in pixels.
    pub size: Option<u32>,
}

/// Serializable layout descriptor. It contains stable identities and geometry only, never UI elements.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DockState {
    root: DockNode,
    known_tabs: Vec<String>,
    panels: Vec<DockPanelPlacement>,
    next_group_id: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum DockNode {
    Group {
        id: String,
        tabs: Vec<String>,
        active: Option<String>,
    },
    Split {
        axis: DockSplitAxis,
        ratio_permille: u16,
        first: Box<DockNode>,
        second: Box<DockNode>,
    },
}

/// Axis used to split a dock group.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DockSplitAxis {
    /// Creates left/right groups.
    Horizontal,
    /// Creates top/bottom groups.
    Vertical,
}

/// Side on which a new group is created during a tab drop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DockSplitSide {
    /// Create a group to the left of the drop target.
    Left,
    /// Create a group to the right of the drop target.
    Right,
    /// Create a group above the drop target.
    Top,
    /// Create a group below the drop target.
    Bottom,
}

/// Errors produced by layout restoration or validation.
#[derive(Debug)]
pub enum DockStateError {
    /// The JSON data could not be parsed.
    Json(serde_json::Error),
    /// The JSON was valid but described an impossible layout.
    Invalid(&'static str),
}

impl std::fmt::Display for DockStateError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "invalid dock JSON: {error}"),
            Self::Invalid(reason) => write!(formatter, "invalid dock layout: {reason}"),
        }
    }
}

impl std::error::Error for DockStateError {}

impl DockState {
    /// Creates a single tab group from stable tab keys.
    pub fn from_tabs(tabs: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let known_tabs = unique_strings(tabs.into_iter().map(Into::into));
        let active = known_tabs.first().cloned();
        Self {
            root: DockNode::Group {
                id: "center".to_owned(),
                tabs: known_tabs.clone(),
                active,
            },
            known_tabs,
            panels: Vec::new(),
            next_group_id: 1,
        }
    }

    /// Parses and validates a serialized layout descriptor.
    pub fn from_json(json: &str) -> Result<Self, DockStateError> {
        let state: Self = serde_json::from_str(json).map_err(DockStateError::Json)?;
        state.validate()?;
        Ok(state)
    }

    /// Serializes a validated descriptor. Business content is never included.
    pub fn to_json(&self) -> Result<String, DockStateError> {
        self.validate()?;
        serde_json::to_string(self).map_err(DockStateError::Json)
    }

    /// Returns the number of center tab groups.
    pub fn group_count(&self) -> usize {
        fn count(node: &DockNode) -> usize {
            match node {
                DockNode::Group { .. } => 1,
                DockNode::Split { first, second, .. } => count(first) + count(second),
            }
        }
        count(&self.root)
    }

    /// Returns true when a tab remains open in any group.
    pub fn contains_tab(&self, key: &str) -> bool {
        group_for_tab(&self.root, key).is_some()
    }

    /// Returns an active key for the first group in tree order.
    pub fn active_tab(&self) -> Option<&str> {
        fn find(node: &DockNode) -> Option<&str> {
            match node {
                DockNode::Group { active, .. } => active.as_deref(),
                DockNode::Split { first, second, .. } => find(first).or_else(|| find(second)),
            }
        }
        find(&self.root)
    }

    /// Selects a tab in its owning group.
    pub fn select_tab(&mut self, key: &str) -> bool {
        let Some(group) = group_for_tab_mut(&mut self.root, key) else {
            return false;
        };
        if group_active(group).as_deref() == Some(key) {
            return false;
        }
        set_group_active(group, Some(key.to_owned()));
        true
    }

    /// Closes a tab and collapses any group made empty by the close.
    pub fn close_tab(&mut self, key: &str) -> bool {
        if !remove_tab(&mut self.root, key) {
            return false;
        }
        self.root = prune_empty(
            std::mem::replace(&mut self.root, empty_group("center")),
            true,
        )
        .unwrap_or_else(|| empty_group("center"));
        true
    }

    /// Moves a tab to a group at the requested insertion index.
    pub fn move_tab(&mut self, key: &str, target_group: &str, index: usize) -> bool {
        let Some(source_group) = group_for_tab(&self.root, key).map(group_id) else {
            return false;
        };
        if group_for_id(&self.root, target_group).is_none() {
            return false;
        }
        if source_group == target_group {
            let Some(group) = group_for_id_mut(&mut self.root, target_group) else {
                return false;
            };
            let tabs = group_tabs_mut(group);
            let Some(old_index) = tabs.iter().position(|tab| tab == key) else {
                return false;
            };
            let tab = tabs.remove(old_index);
            let target_index = if index > old_index { index - 1 } else { index }.min(tabs.len());
            if target_index == old_index {
                tabs.insert(old_index, tab);
                return false;
            }
            tabs.insert(target_index, tab);
            set_group_active(group, Some(key.to_owned()));
            return true;
        }

        if !remove_tab(&mut self.root, key) {
            return false;
        }
        self.root = prune_empty(
            std::mem::replace(&mut self.root, empty_group("center")),
            true,
        )
        .unwrap_or_else(|| empty_group("center"));
        let Some(group) = group_for_id_mut(&mut self.root, target_group) else {
            return false;
        };
        let tabs = group_tabs_mut(group);
        tabs.insert(index.min(tabs.len()), key.to_owned());
        set_group_active(group, Some(key.to_owned()));
        true
    }

    /// Moves a tab into a newly split group adjacent to the requested target group.
    pub fn split_tab(&mut self, key: &str, target_group: &str, side: DockSplitSide) -> bool {
        let Some(source_group) = group_for_tab(&self.root, key)
            .map(group_id)
            .map(str::to_owned)
        else {
            return false;
        };
        if group_for_id(&self.root, target_group).is_none() {
            return false;
        }
        let mut next_group_id = self.next_group_id;
        let split_id = loop {
            let candidate = format!("dock-group-{next_group_id}");
            let Some(next) = next_group_id.checked_add(1) else {
                return false;
            };
            next_group_id = next;
            if group_for_id(&self.root, &candidate).is_none() {
                if next_group_id == u64::MAX {
                    return false;
                }
                break candidate;
            }
        };
        if !remove_tab(&mut self.root, key) {
            return false;
        }
        self.next_group_id = next_group_id;
        if source_group != target_group {
            self.root = prune_empty(
                std::mem::replace(&mut self.root, empty_group("center")),
                true,
            )
            .unwrap_or_else(|| empty_group("center"));
        }
        let moved = DockNode::Group {
            id: split_id,
            tabs: vec![key.to_owned()],
            active: Some(key.to_owned()),
        };
        replace_group_with_split(&mut self.root, target_group, moved, side)
    }

    fn from_inputs(
        tabs: &[DockTab],
        active_tab: Option<&SharedString>,
        panels: &[DockPanel],
    ) -> Self {
        let mut state = Self::from_tabs(tabs.iter().map(|tab| tab.key.to_string()));
        if let Some(active) = active_tab.map(ToString::to_string)
            && state.contains_tab(&active)
        {
            state.select_tab(&active);
        }
        state.panels = panels.iter().map(DockPanelPlacement::from_panel).collect();
        state
    }

    fn reconcile(&mut self, tabs: &[DockTab], panels: &[DockPanel]) {
        let incoming_tabs = unique_strings(tabs.iter().map(|tab| tab.key.to_string()));
        let incoming: HashSet<_> = incoming_tabs.iter().cloned().collect();
        for removed in self
            .known_tabs
            .iter()
            .filter(|key| !incoming.contains(*key))
        {
            remove_tab(&mut self.root, removed);
        }
        if self.known_tabs.iter().any(|key| !incoming.contains(key)) {
            self.root = prune_empty(
                std::mem::replace(&mut self.root, empty_group("center")),
                true,
            )
            .unwrap_or_else(|| empty_group("center"));
        }
        self.known_tabs.retain(|key| incoming.contains(key));
        let new_tabs: Vec<_> = incoming_tabs
            .into_iter()
            .filter(|key| !self.known_tabs.contains(key))
            .collect();
        for key in new_tabs {
            self.known_tabs.push(key.clone());
            let group = first_group_mut(&mut self.root);
            group_tabs_mut(group).push(key.clone());
            if group_active(group).is_none() {
                set_group_active(group, Some(key));
            }
        }

        let incoming_panels: Vec<_> = panels.iter().map(DockPanelPlacement::from_panel).collect();
        let panel_keys: HashSet<_> = incoming_panels
            .iter()
            .map(|panel| panel.key.clone())
            .collect();
        self.panels.retain(|panel| panel_keys.contains(&panel.key));
        for panel in incoming_panels {
            if self.panels.iter().all(|entry| entry.key != panel.key) {
                self.panels.push(panel);
            }
        }
    }

    fn validate(&self) -> Result<(), DockStateError> {
        if self.next_group_id == u64::MAX {
            return Err(DockStateError::Invalid(
                "group identity counter is exhausted",
            ));
        }
        if has_duplicate_or_empty(&self.known_tabs) {
            return Err(DockStateError::Invalid(
                "tab catalog keys must be non-empty and unique",
            ));
        }
        let mut groups = HashSet::new();
        let mut tabs = HashSet::new();
        validate_node(&self.root, &mut groups, &mut tabs)?;
        if tabs.iter().any(|tab| !self.known_tabs.contains(tab)) {
            return Err(DockStateError::Invalid(
                "open tabs must exist in the tab catalog",
            ));
        }
        if has_duplicate_or_empty(
            &self
                .panels
                .iter()
                .map(|panel| panel.key.clone())
                .collect::<Vec<_>>(),
        ) {
            return Err(DockStateError::Invalid(
                "panel keys must be non-empty and unique",
            ));
        }
        if self
            .panels
            .iter()
            .any(|panel| panel.size.is_some_and(|size| size < 48))
        {
            return Err(DockStateError::Invalid(
                "panel sizes must be at least 48 pixels",
            ));
        }
        Ok(())
    }
}

fn validate_node(
    node: &DockNode,
    groups: &mut HashSet<String>,
    tabs: &mut HashSet<String>,
) -> Result<(), DockStateError> {
    match node {
        DockNode::Group {
            id,
            tabs: group_tabs,
            active,
        } => {
            if id.is_empty() || !groups.insert(id.clone()) {
                return Err(DockStateError::Invalid(
                    "group ids must be non-empty and unique",
                ));
            }
            if has_duplicate_or_empty(group_tabs) {
                return Err(DockStateError::Invalid(
                    "group tab keys must be non-empty and unique",
                ));
            }
            for key in group_tabs {
                if !tabs.insert(key.clone()) {
                    return Err(DockStateError::Invalid(
                        "a tab cannot occur in multiple groups",
                    ));
                }
            }
            if active
                .as_ref()
                .is_some_and(|active| !group_tabs.contains(active))
            {
                return Err(DockStateError::Invalid(
                    "active tab must belong to its group",
                ));
            }
            if !group_tabs.is_empty() && active.is_none() {
                return Err(DockStateError::Invalid(
                    "non-empty groups must have an active tab",
                ));
            }
        }
        DockNode::Split {
            ratio_permille,
            first,
            second,
            ..
        } => {
            if !(100..=900).contains(ratio_permille) {
                return Err(DockStateError::Invalid(
                    "split ratio must be between 100 and 900 permille",
                ));
            }
            validate_node(first, groups, tabs)?;
            validate_node(second, groups, tabs)?;
        }
    }
    Ok(())
}

fn has_duplicate_or_empty(items: &[String]) -> bool {
    let mut seen = HashSet::new();
    items
        .iter()
        .any(|item| item.is_empty() || !seen.insert(item))
}

fn unique_strings(items: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut seen = HashSet::new();
    items
        .into_iter()
        .filter(|item| !item.is_empty() && seen.insert(item.clone()))
        .collect()
}

fn empty_group(id: impl Into<String>) -> DockNode {
    DockNode::Group {
        id: id.into(),
        tabs: Vec::new(),
        active: None,
    }
}

fn group_id(group: &DockNode) -> &str {
    match group {
        DockNode::Group { id, .. } => id,
        DockNode::Split { .. } => unreachable!("group_id requires a tab group"),
    }
}

fn group_tabs_mut(group: &mut DockNode) -> &mut Vec<String> {
    match group {
        DockNode::Group { tabs, .. } => tabs,
        DockNode::Split { .. } => unreachable!("group_tabs_mut requires a tab group"),
    }
}

fn group_active(group: &DockNode) -> &Option<String> {
    match group {
        DockNode::Group { active, .. } => active,
        DockNode::Split { .. } => unreachable!("group_active requires a tab group"),
    }
}

fn set_group_active(group: &mut DockNode, active: Option<String>) {
    match group {
        DockNode::Group {
            active: current, ..
        } => *current = active,
        DockNode::Split { .. } => unreachable!("set_group_active requires a tab group"),
    }
}

fn group_for_tab<'a>(node: &'a DockNode, key: &str) -> Option<&'a DockNode> {
    match node {
        DockNode::Group { tabs, .. } if tabs.iter().any(|tab| tab == key) => Some(node),
        DockNode::Group { .. } => None,
        DockNode::Split { first, second, .. } => {
            group_for_tab(first, key).or_else(|| group_for_tab(second, key))
        }
    }
}

fn group_for_tab_mut<'a>(node: &'a mut DockNode, key: &str) -> Option<&'a mut DockNode> {
    match node {
        DockNode::Group { tabs, .. } if tabs.iter().any(|tab| tab == key) => Some(node),
        DockNode::Group { .. } => None,
        DockNode::Split { first, second, .. } => {
            group_for_tab_mut(first, key).or_else(|| group_for_tab_mut(second, key))
        }
    }
}

fn group_for_id<'a>(node: &'a DockNode, id: &str) -> Option<&'a DockNode> {
    match node {
        DockNode::Group { id: group_id, .. } if group_id == id => Some(node),
        DockNode::Group { .. } => None,
        DockNode::Split { first, second, .. } => {
            group_for_id(first, id).or_else(|| group_for_id(second, id))
        }
    }
}

fn group_for_id_mut<'a>(node: &'a mut DockNode, id: &str) -> Option<&'a mut DockNode> {
    match node {
        DockNode::Group { id: group_id, .. } if group_id == id => Some(node),
        DockNode::Group { .. } => None,
        DockNode::Split { first, second, .. } => {
            group_for_id_mut(first, id).or_else(|| group_for_id_mut(second, id))
        }
    }
}

fn first_group_mut(node: &mut DockNode) -> &mut DockNode {
    match node {
        DockNode::Group { .. } => node,
        DockNode::Split { first, .. } => first_group_mut(first),
    }
}

fn remove_tab(node: &mut DockNode, key: &str) -> bool {
    match node {
        DockNode::Group { tabs, active, .. } => {
            let Some(index) = tabs.iter().position(|tab| tab == key) else {
                return false;
            };
            tabs.remove(index);
            if active.as_deref() == Some(key) {
                *active = tabs.get(index.min(tabs.len().saturating_sub(1))).cloned();
            }
            true
        }
        DockNode::Split { first, second, .. } => remove_tab(first, key) || remove_tab(second, key),
    }
}

fn prune_empty(node: DockNode, is_root: bool) -> Option<DockNode> {
    match node {
        DockNode::Group { tabs, .. } if tabs.is_empty() && !is_root => None,
        DockNode::Split {
            axis,
            ratio_permille,
            first,
            second,
        } => match (prune_empty(*first, false), prune_empty(*second, false)) {
            (Some(first), Some(second)) => Some(DockNode::Split {
                axis,
                ratio_permille,
                first: Box::new(first),
                second: Box::new(second),
            }),
            (Some(node), None) | (None, Some(node)) => Some(node),
            (None, None) => None,
        },
        node => Some(node),
    }
}

fn replace_group_with_split(
    node: &mut DockNode,
    id: &str,
    moved: DockNode,
    side: DockSplitSide,
) -> bool {
    match node {
        DockNode::Group { id: current_id, .. } if current_id == id => {
            let old = std::mem::replace(node, empty_group("replaced"));
            let (axis, first, second) = match side {
                DockSplitSide::Left => (DockSplitAxis::Horizontal, moved, old),
                DockSplitSide::Right => (DockSplitAxis::Horizontal, old, moved),
                DockSplitSide::Top => (DockSplitAxis::Vertical, moved, old),
                DockSplitSide::Bottom => (DockSplitAxis::Vertical, old, moved),
            };
            *node = DockNode::Split {
                axis,
                ratio_permille: 500,
                first: Box::new(first),
                second: Box::new(second),
            };
            true
        }
        DockNode::Group { .. } => false,
        DockNode::Split { first, second, .. } => {
            if group_for_id(first, id).is_some() {
                replace_group_with_split(first, id, moved, side)
            } else {
                replace_group_with_split(second, id, moved, side)
            }
        }
    }
}

/// Dock panel metadata and host-owned native content.
pub struct DockPanel {
    /// Stable panel key.
    pub key: SharedString,
    /// User-facing panel title.
    pub title: SharedString,
    /// Preferred edge, overridden by restored layout state when present.
    pub edge: DockEdge,
    /// Preferred edge-axis size, overridden by restored layout state when present.
    pub size: Option<Pixels>,
    /// Whether a header strip is rendered above the panel content.
    pub header: bool,
    /// Content supplied and owned by the host application.
    pub content: AnyElement,
}

impl DockPanel {
    /// Creates a dock panel from a stable key, title, edge, and native content.
    pub fn new(
        key: impl Into<SharedString>,
        title: impl Into<SharedString>,
        edge: DockEdge,
        content: impl IntoElement,
    ) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            edge,
            size: None,
            header: true,
            content: content.into_any_element(),
        }
    }

    /// Sets a preferred panel size.
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into().max(px(48.0)));
        self
    }

    /// Hides or shows the panel header strip.
    pub fn header(mut self, header: bool) -> Self {
        self.header = header;
        self
    }
}

impl DockPanelPlacement {
    fn from_panel(panel: &DockPanel) -> Self {
        let size = panel
            .size
            .map(|size| (size / px(1.0)).max(48.0).round() as u32);
        Self {
            key: panel.key.to_string(),
            edge: panel.edge,
            size,
        }
    }
}

/// Host-provided content for a center tab.
pub struct DockTab {
    /// Stable key used by persisted layout state.
    pub key: SharedString,
    /// User-facing tab label.
    pub title: SharedString,
    /// Content supplied and owned by the host application.
    pub content: AnyElement,
    /// Whether the component exposes a close action.
    pub closable: bool,
}

impl DockTab {
    /// Creates a center tab from a stable key, title, and native content.
    pub fn new(
        key: impl Into<SharedString>,
        title: impl Into<SharedString>,
        content: impl IntoElement,
    ) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            content: content.into_any_element(),
            closable: true,
        }
    }

    /// Toggles whether the close affordance is shown.
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }
}

/// Workbench layout with side panels and interactive center tab groups.
pub struct DockLayout {
    id: SharedString,
    panels: Vec<DockPanel>,
    tabs: Vec<DockTab>,
    active_tab: Option<SharedString>,
    height: Option<Pixels>,
    bordered: bool,
    panel_gap: Pixels,
    layout_state: Option<DockState>,
    on_layout_change: Option<Arc<dyn Fn(DockState) + 'static>>,
}

impl DockLayout {
    /// Creates a dock layout with a stable identity at this call site.
    #[track_caller]
    pub fn new() -> Self {
        let caller = std::panic::Location::caller();
        Self {
            id: format!(
                "dock-layout:{}:{}:{}",
                caller.file(),
                caller.line(),
                caller.column()
            )
            .into(),
            panels: Vec::new(),
            tabs: Vec::new(),
            active_tab: None,
            height: None,
            bordered: true,
            panel_gap: px(0.0),
            layout_state: None,
            on_layout_change: None,
        }
    }

    /// Assigns a stable element identity. Supply distinct ids for repeated call sites.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Adds an edge panel.
    pub fn panel(mut self, panel: DockPanel) -> Self {
        self.panels.push(panel);
        self
    }

    /// Adds host-owned center tab content.
    pub fn tab(mut self, tab: DockTab) -> Self {
        if self.active_tab.is_none() {
            self.active_tab = Some(tab.key.clone());
        }
        self.tabs.push(tab);
        self
    }

    /// Sets the initial active key when creating a fresh layout state.
    pub fn active_tab(mut self, key: impl Into<SharedString>) -> Self {
        self.active_tab = Some(key.into());
        self
    }

    /// Restores a serialized descriptor; content is still supplied through [`Self::panel`] and [`Self::tab`].
    pub fn layout_state(mut self, state: DockState) -> Self {
        self.layout_state = Some(state);
        self
    }

    /// Receives serializable descriptor snapshots after selection, close, reorder, move, or split actions.
    pub fn on_layout_change(mut self, callback: impl Fn(DockState) + 'static) -> Self {
        self.on_layout_change = Some(Arc::new(callback));
        self
    }

    /// Sets the component height.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Applies a desktop workbench default height.
    pub fn height_lg(self) -> Self {
        self.height(px(520.0))
    }

    /// Toggles the outer border around the dock workbench.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Sets spacing between panel regions.
    pub fn panel_gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.panel_gap = gap.into().max(px(0.0));
        self
    }

    /// Returns the number of builder panels assigned to an edge.
    pub fn panel_count(&self, edge: DockEdge) -> usize {
        self.panels
            .iter()
            .filter(|panel| panel.edge == edge)
            .count()
    }

    /// Returns the initial active tab key configured by the builder.
    pub fn active_tab_key(&self) -> Option<&SharedString> {
        self.active_tab.as_ref()
    }
}

impl IntoElement for DockLayout {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for DockLayout {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let initial = self.layout_state.unwrap_or_else(|| {
            DockState::from_inputs(&self.tabs, self.active_tab.as_ref(), &self.panels)
        });
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, _| initial);
        state.update(cx, |layout, _| layout.reconcile(&self.tabs, &self.panels));
        let snapshot = state.read(cx).clone();
        let callback = self.on_layout_change.clone();
        let mut tab_contents = HashMap::new();
        for tab in self.tabs {
            let key = tab.key.to_string();
            tab_contents.entry(key).or_insert(tab);
        }
        let mut panel_contents: HashMap<_, _> = self
            .panels
            .into_iter()
            .map(|panel| (panel.key.to_string(), panel))
            .collect();
        let (mut top, mut bottom, mut left, mut center_panels, mut right) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for placement in &snapshot.panels {
            let Some(mut panel) = panel_contents.remove(&placement.key) else {
                continue;
            };
            panel.edge = placement.edge;
            panel.size = placement.size.map(|size| px(size as f32));
            let rendered = render_panel(panel, &theme);
            match placement.edge {
                DockEdge::Top => top.push(rendered),
                DockEdge::Bottom => bottom.push(rendered),
                DockEdge::Left => left.push(rendered),
                DockEdge::Right => right.push(rendered),
                DockEdge::Center => center_panels.push(rendered),
            }
        }
        let center = render_node(&snapshot.root, &mut tab_contents, &state, &callback, &theme);
        let middle = div()
            .flex()
            .flex_row()
            .flex_1()
            .min_h(px(0.0))
            .min_w(px(0.0))
            .gap(self.panel_gap)
            .children(left)
            .child(center)
            .children(center_panels)
            .children(right);
        div()
            .id(format!("{}-root", self.id))
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .overflow_hidden()
            .when_some(self.height, |style, height| style.h(height))
            .when(self.bordered, |style| {
                style
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.neutral.border)
            })
            .bg(theme.neutral.card)
            .gap(self.panel_gap)
            .children(top)
            .child(middle)
            .children(bottom)
    }
}

#[derive(Clone, Debug)]
struct DockTabDrag {
    source: gpui::EntityId,
    key: String,
    title: SharedString,
}

#[derive(Clone, Debug)]
enum DockDropTarget {
    Insert {
        group_id: String,
        index: usize,
    },
    Append {
        group_id: String,
    },
    Split {
        group_id: String,
        side: DockSplitSide,
    },
}

struct DockDragPreview {
    title: SharedString,
}

impl Render for DockDragPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        div()
            .px_3()
            .py_2()
            .rounded_md()
            .bg(theme.neutral.card)
            .border_1()
            .border_color(theme.primary.base)
            .child(self.title.clone())
    }
}

fn mutate_layout(
    state: &Entity<DockState>,
    cx: &mut App,
    callback: &Option<Arc<dyn Fn(DockState) + 'static>>,
    mutate: impl FnOnce(&mut DockState) -> bool,
) {
    let changed = state.update(cx, |layout, cx| {
        let changed = mutate(layout);
        if changed {
            cx.notify();
        }
        changed
    });
    if changed {
        if let Some(callback) = callback {
            callback(state.read(cx).clone());
        }
        cx.refresh_windows();
    }
}

fn handle_drop(
    state: &Entity<DockState>,
    cx: &mut App,
    callback: &Option<Arc<dyn Fn(DockState) + 'static>>,
    dragged: &DockTabDrag,
    target: &DockDropTarget,
) {
    if dragged.source != state.entity_id() {
        return;
    }
    mutate_layout(state, cx, callback, |layout| match target {
        DockDropTarget::Insert { group_id, index } => {
            layout.move_tab(&dragged.key, group_id, *index)
        }
        DockDropTarget::Append { group_id } => layout.move_tab(&dragged.key, group_id, usize::MAX),
        DockDropTarget::Split { group_id, side } => layout.split_tab(&dragged.key, group_id, *side),
    });
}

fn render_node(
    node: &DockNode,
    contents: &mut HashMap<String, DockTab>,
    state: &Entity<DockState>,
    callback: &Option<Arc<dyn Fn(DockState) + 'static>>,
    theme: &liora_theme::Theme,
) -> AnyElement {
    match node {
        DockNode::Group { id, tabs, active } => render_group(
            id,
            tabs,
            active.as_deref(),
            contents,
            state,
            callback,
            theme,
        ),
        DockNode::Split {
            axis,
            ratio_permille,
            first,
            second,
        } => {
            let fraction = *ratio_permille as f32 / 1000.0;
            let first = render_node(first, contents, state, callback, theme);
            let second = render_node(second, contents, state, callback, theme);
            let split = match axis {
                DockSplitAxis::Horizontal => div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .min_w(px(0.0))
                    .min_h(px(0.0))
                    .h_full()
                    .child(
                        div()
                            .w(relative(fraction))
                            .flex_shrink_0()
                            .h_full()
                            .child(first),
                    )
                    .child(div().w(px(1.0)).h_full().bg(theme.neutral.border))
                    .child(div().flex_1().min_w(px(0.0)).h_full().child(second)),
                DockSplitAxis::Vertical => div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w(px(0.0))
                    .min_h(px(0.0))
                    .w_full()
                    .child(
                        div()
                            .h(relative(fraction))
                            .flex_shrink_0()
                            .w_full()
                            .child(first),
                    )
                    .child(div().h(px(1.0)).w_full().bg(theme.neutral.border))
                    .child(div().flex_1().min_h(px(0.0)).w_full().child(second)),
            };
            split.into_any_element()
        }
    }
}

fn render_group(
    group_id: &str,
    tabs: &[String],
    active: Option<&str>,
    contents: &mut HashMap<String, DockTab>,
    state: &Entity<DockState>,
    callback: &Option<Arc<dyn Fn(DockState) + 'static>>,
    theme: &liora_theme::Theme,
) -> AnyElement {
    let mut headers = div()
        .flex()
        .flex_row()
        .items_center()
        .min_w(px(0.0))
        .gap(px(2.0));
    for (index, key) in tabs.iter().enumerate() {
        let Some(tab) = contents.get(key) else {
            continue;
        };
        let is_active = active == Some(key.as_str());
        let title = tab.title.clone();
        let closeable = tab.closable;
        let tab_key = key.clone();
        let group = group_id.to_owned();
        let mut header = div()
            .id(format!("dock-tab-{group_id}-{key}"))
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .px_3()
            .py_2()
            .text_sm()
            .w(px(120.0))
            .flex_shrink_0()
            .justify_between()
            .font_weight(if is_active {
                gpui::FontWeight::BOLD
            } else {
                gpui::FontWeight::NORMAL
            })
            .text_color(if is_active {
                theme.primary.base
            } else {
                theme.neutral.text_2
            })
            .bg(if is_active {
                theme.primary.light_9
            } else {
                theme.neutral.card
            })
            .cursor_pointer()
            .child(title.clone())
            .on_click({
                let state = state.clone();
                let callback = callback.clone();
                let key = tab_key.clone();
                move |_, _, cx| {
                    mutate_layout(&state, cx, &callback, |layout| layout.select_tab(&key))
                }
            });
        if closeable {
            header = header.child(
                div()
                    .id(format!("dock-tab-close-{group_id}-{key}"))
                    .px_1()
                    .text_xs()
                    .cursor_pointer()
                    .text_color(theme.neutral.text_3)
                    .hover(|style| style.text_color(theme.danger.base))
                    .child("×")
                    .on_click({
                        let state = state.clone();
                        let callback = callback.clone();
                        let key = tab_key.clone();
                        move |_, _, cx| {
                            mutate_layout(&state, cx, &callback, |layout| layout.close_tab(&key));
                            cx.stop_propagation();
                        }
                    }),
            );
        }
        let target = DockDropTarget::Insert {
            group_id: group.clone(),
            index,
        };
        let drag = DockTabDrag {
            source: state.entity_id(),
            key: tab_key,
            title,
        };
        headers = headers.child(
            header
                .on_drag(drag, |drag, _, _, cx| {
                    cx.new(|_| DockDragPreview {
                        title: drag.title.clone(),
                    })
                })
                .on_drop({
                    let state = state.clone();
                    let callback = callback.clone();
                    move |drag: &DockTabDrag, _, cx| {
                        handle_drop(&state, cx, &callback, drag, &target)
                    }
                }),
        );
    }
    let split_sides = [
        (DockSplitSide::Left, "Split ←"),
        (DockSplitSide::Right, "Split →"),
        (DockSplitSide::Top, "Split ↑"),
        (DockSplitSide::Bottom, "Split ↓"),
    ];
    let controls =
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .children(split_sides.into_iter().map(|(side, label)| {
                let target = DockDropTarget::Split {
                    group_id: group_id.to_owned(),
                    side,
                };
                div()
                    .id(format!("dock-split-{}-{side:?}", group_id))
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .text_xs()
                    .w(px(64.0))
                    .flex_shrink_0()
                    .text_color(theme.neutral.text_3)
                    .bg(theme.neutral.body)
                    .child(label)
                    .on_drop({
                        let state = state.clone();
                        let callback = callback.clone();
                        move |drag: &DockTabDrag, _, cx| {
                            handle_drop(&state, cx, &callback, drag, &target)
                        }
                    })
            }));
    let active_content = active
        .and_then(|key| contents.remove(key))
        .map(|tab| tab.content)
        .unwrap_or_else(|| {
            div()
                .flex()
                .items_center()
                .justify_center()
                .text_color(theme.neutral.text_3)
                .child("Drop a tab here")
                .into_any_element()
        });
    let body = div()
        .flex()
        .flex_col()
        .flex_1()
        .size_full()
        .min_w(px(0.0))
        .min_h(px(0.0))
        .overflow_hidden()
        .border_1()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.card)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .gap_2()
                .border_b_1()
                .border_color(theme.neutral.border)
                .child(headers)
                .child(controls),
        )
        .child(
            div()
                .flex_1()
                .min_w(px(0.0))
                .min_h(px(0.0))
                .overflow_hidden()
                .p_3()
                .child(active_content),
        )
        .on_drop({
            let state = state.clone();
            let callback = callback.clone();
            let target = DockDropTarget::Append {
                group_id: group_id.to_owned(),
            };
            move |drag: &DockTabDrag, _, cx| handle_drop(&state, cx, &callback, drag, &target)
        });
    body.into_any_element()
}

fn render_panel(panel: DockPanel, theme: &liora_theme::Theme) -> AnyElement {
    let vertical = matches!(
        panel.edge,
        DockEdge::Left | DockEdge::Right | DockEdge::Center
    );
    let header = panel.header.then(|| {
        div()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(theme.neutral.border)
            .text_xs()
            .font_weight(gpui::FontWeight::BOLD)
            .text_color(theme.neutral.text_2)
            .child(panel.title.clone())
    });
    div()
        .id(format!("dock-panel-{}", panel.key))
        .flex()
        .flex_col()
        .min_w(px(0.0))
        .min_h(px(0.0))
        .when(vertical, |style| style.h_full())
        .when(!vertical, |style| style.w_full())
        .when_some(panel.size, |style, size| {
            if vertical {
                style.w(size).flex_shrink_0()
            } else {
                style.h(size).flex_shrink_0()
            }
        })
        .border_1()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.card)
        .children(header)
        .child(div().flex_1().min_h(px(0.0)).p_3().child(panel.content))
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dock_state_reorders_moves_splits_and_closes_tabs() {
        let mut state = DockState::from_tabs(["main", "readme", "logs"]);
        assert!(state.move_tab("logs", "center", 0));
        assert!(state.split_tab("readme", "center", DockSplitSide::Right));
        assert_eq!(state.group_count(), 2);
        assert!(state.move_tab("main", "dock-group-1", 1));
        assert_eq!(
            group_for_id(&state.root, "dock-group-1").map(group_tabs_mut_read),
            Some(vec!["readme", "main"])
        );
        assert!(state.close_tab("logs"));
        assert!(!state.contains_tab("logs"));
        assert_eq!(state.group_count(), 1);
    }

    fn group_tabs_mut_read(group: &DockNode) -> Vec<&str> {
        match group {
            DockNode::Group { tabs, .. } => tabs.iter().map(String::as_str).collect(),
            DockNode::Split { .. } => Vec::new(),
        }
    }

    #[test]
    fn dock_state_json_round_trips_and_rejects_invalid_layouts() {
        let mut state = DockState::from_tabs(["main", "readme"]);
        assert!(state.split_tab("readme", "center", DockSplitSide::Bottom));
        let json = state.to_json().unwrap();
        assert_eq!(DockState::from_json(&json).unwrap(), state);
        assert!(matches!(
            DockState::from_json("{not json}"),
            Err(DockStateError::Json(_))
        ));
        let bad_ratio = json.replace("\"ratio_permille\":500", "\"ratio_permille\":999");
        assert!(matches!(
            DockState::from_json(&bad_ratio),
            Err(DockStateError::Invalid(_))
        ));
        let duplicate_tab = json.replace("\"tabs\":[\"main\"]", "\"tabs\":[\"main\",\"main\"]");
        assert!(matches!(
            DockState::from_json(&duplicate_tab),
            Err(DockStateError::Invalid(_))
        ));
    }

    #[test]
    fn restored_group_identity_exhaustion_leaves_descriptor_unchanged() {
        let json = format!(
            r#"{{"root":{{"kind":"split","axis":"horizontal","ratio_permille":500,"first":{{"kind":"group","id":"dock-group-{}","tabs":["main","other"],"active":"main"}},"second":{{"kind":"group","id":"dock-group-{}","tabs":["logs"],"active":"logs"}}}},"known_tabs":["main","other","logs"],"panels":[],"next_group_id":{}}}"#,
            u64::MAX - 1,
            u64::MAX,
            u64::MAX - 1,
        );
        let mut state = DockState::from_json(&json).expect("valid near-exhausted descriptor");
        let before = state.clone();
        assert!(!state.split_tab(
            "other",
            &format!("dock-group-{}", u64::MAX - 1),
            DockSplitSide::Right
        ));
        assert_eq!(
            state, before,
            "failed allocation must not remove tabs or advance the counter"
        );
        assert_eq!(state.to_json().unwrap(), before.to_json().unwrap());
    }

    #[test]
    fn dock_reconcile_does_not_reopen_a_closed_tab() {
        let mut state = DockState::from_tabs(["main", "readme"]);
        assert!(state.close_tab("readme"));
        let current_tabs = [
            DockTab::new("main", "main.rs", div()),
            DockTab::new("readme", "README.md", div()),
        ];
        state.reconcile(&current_tabs, &[]);
        assert!(!state.contains_tab("readme"));
        state.reconcile(&current_tabs[..1], &[]);
        state.reconcile(&current_tabs, &[]);
        assert!(state.contains_tab("readme"));
    }
}
