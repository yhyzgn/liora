//! Per-view bounded native formula/diagram image cache.
use gpui::{RenderImage, SharedString};
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum AssetKind {
    Math(bool, [u8; 4]),
    Mermaid([u8; 3], [u8; 3]),
}

type Rendered = Result<Arc<RenderImage>, SharedString>;
#[derive(Clone, Default)]
pub(super) struct AssetCache(Rc<RefCell<HashMap<(AssetKind, SharedString), Rendered>>>);

impl AssetCache {
    pub(super) fn render(&self, kind: AssetKind, source: &str) -> Rendered {
        let key = (kind, SharedString::from(source.to_owned()));
        if let Some(result) = self.0.borrow().get(&key) {
            return result.clone();
        }
        let result = match kind {
            AssetKind::Math(display, foreground) => {
                super::math::render_math(source, display, foreground, Default::default())
                    .map(|output| {
                        debug_assert_eq!(output.png.len(), output.diagnostic.png_bytes);
                        output.image
                    })
                    .map_err(|error| SharedString::from(error.to_string()))
            }
            AssetKind::Mermaid(foreground, background) => {
                super::diagram::render_diagram(source, foreground, background)
                    .map(|output| {
                        debug_assert_eq!(output.png.len(), output.diagnostic.png_bytes);
                        output.image
                    })
                    .map_err(|error| SharedString::from(error.to_string()))
            }
        };
        let mut cache = self.0.borrow_mut();
        if cache.len() >= 4 {
            if let Some(oldest) = cache.keys().next().cloned() {
                cache.remove(&oldest);
            }
        }
        cache.insert(key, result.clone());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::{Arc, AssetCache, AssetKind};

    #[test]
    fn diagram_cache_identity_includes_host_foreground_and_background() {
        let cache = AssetCache::default();
        let source = "flowchart LR\nA[Native] --> B[GPUI]\n";
        let light = AssetKind::Mermaid([20, 20, 20], [255, 255, 255]);
        let dark = AssetKind::Mermaid([240, 240, 240], [16, 16, 18]);
        let light_image = cache.render(light, source).unwrap();
        let dark_image = cache.render(dark, source).unwrap();
        assert!(!Arc::ptr_eq(&light_image, &dark_image));
        assert!(Arc::ptr_eq(
            &dark_image,
            &cache.render(dark, source).unwrap()
        ));
        let same_foreground = cache
            .render(AssetKind::Mermaid([240, 240, 240], [255, 255, 255]), source)
            .unwrap();
        assert!(!Arc::ptr_eq(&dark_image, &same_foreground));
    }

    #[test]
    fn math_cache_identity_includes_alpha() {
        let cache = AssetCache::default();
        let opaque = cache
            .render(AssetKind::Math(false, [24, 48, 72, 255]), "x")
            .unwrap();
        let translucent = cache
            .render(AssetKind::Math(false, [24, 48, 72, 128]), "x")
            .unwrap();
        assert!(!Arc::ptr_eq(&opaque, &translucent));
        assert!(Arc::ptr_eq(
            &translucent,
            &cache
                .render(AssetKind::Math(false, [24, 48, 72, 128]), "x")
                .unwrap()
        ));
    }

    #[test]
    fn caches_real_math_and_diagram_images_and_evicts_one_entry_at_capacity() {
        let cache = AssetCache::default();
        let math = cache
            .render(AssetKind::Math(true, [0, 0, 0, 255]), r"\frac{1}{2}")
            .unwrap();
        let same_math = cache
            .render(AssetKind::Math(true, [0, 0, 0, 255]), r"\frac{1}{2}")
            .unwrap();
        assert!(std::sync::Arc::ptr_eq(&math, &same_math));
        let dark_math = cache
            .render(AssetKind::Math(true, [255, 255, 255, 255]), r"\frac{1}{2}")
            .unwrap();
        assert!(!std::sync::Arc::ptr_eq(&math, &dark_math));
        assert!(std::sync::Arc::ptr_eq(
            &dark_math,
            &cache
                .render(AssetKind::Math(true, [255, 255, 255, 255]), r"\frac{1}{2}")
                .unwrap()
        ));

        let diagram = cache
            .render(
                AssetKind::Mermaid([0, 0, 0], [255, 255, 255]),
                "flowchart LR\nA[Start] --> B[Done]\n",
            )
            .unwrap();
        assert!(diagram.size(0).width.0 > 0);
        assert!(
            cache
                .render(
                    AssetKind::Mermaid([0, 0, 0], [255, 255, 255]),
                    "not a diagram"
                )
                .is_err()
        );

        let bounded = AssetCache::default();
        for source in ["bad 1", "bad 2", "bad 3", "bad 4", "bad 5"] {
            assert!(
                bounded
                    .render(AssetKind::Mermaid([0, 0, 0], [255, 255, 255]), source)
                    .is_err()
            );
        }
        assert_eq!(bounded.0.borrow().len(), 4);
    }
}
