use gpui::{ElementId, Pixels, SharedString};

pub trait PixelsExt {
    fn as_f32(self) -> f32;
}

impl PixelsExt for Pixels {
    fn as_f32(self) -> f32 {
        f32::from(self)
    }
}

pub fn element_id(id: impl Into<SharedString>) -> ElementId {
    ElementId::from(id.into())
}
