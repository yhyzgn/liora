use liora_tray::{TrayError, icon_from_rgba};
use std::io::ErrorKind;

#[test]
fn unrepresentable_rgba_byte_count_returns_invalid_input() {
    match icon_from_rgba(vec![0; 4], u32::MAX, u32::MAX) {
        Err(TrayError::BadIcon(tray_icon::BadIcon::OsError(error))) => {
            assert_eq!(error.kind(), ErrorKind::InvalidInput);
        }
        result => panic!("expected an unrepresentable-dimensions error, got {result:?}"),
    }
}

#[test]
fn wrapped_zero_byte_count_cannot_accept_an_empty_icon() {
    match icon_from_rgba(Vec::new(), 1 << 31, 1 << 31) {
        Err(TrayError::BadIcon(tray_icon::BadIcon::OsError(error))) => {
            assert_eq!(error.kind(), ErrorKind::InvalidInput);
        }
        result => panic!("expected an unrepresentable-dimensions error, got {result:?}"),
    }
}

#[test]
fn representable_rgba_mismatches_preserve_exact_counts() {
    for actual in [0, 3, 5] {
        match icon_from_rgba(vec![0; actual], 1, 1) {
            Err(TrayError::InvalidRgba {
                width: 1,
                height: 1,
                expected: 4,
                actual: reported,
            }) => assert_eq!(reported, actual),
            result => panic!("expected an exact RGBA byte-count error, got {result:?}"),
        }
    }
}

#[cfg(target_pointer_width = "64")]
#[test]
fn representable_byte_count_does_not_use_u32_pixel_arithmetic() {
    match icon_from_rgba(vec![0; 4], 65_536, 65_536) {
        Err(TrayError::InvalidRgba {
            width: 65_536,
            height: 65_536,
            expected: 17_179_869_184,
            actual: 4,
        }) => {}
        result => panic!("expected an exact wide RGBA byte-count error, got {result:?}"),
    }
}

#[cfg(target_pointer_width = "32")]
#[test]
fn representable_pixel_count_with_unrepresentable_bytes_returns_invalid_input() {
    match icon_from_rgba(vec![0; 4], 1 << 30, 1) {
        Err(TrayError::BadIcon(tray_icon::BadIcon::OsError(error))) => {
            assert_eq!(error.kind(), ErrorKind::InvalidInput);
        }
        result => panic!("expected an unrepresentable-dimensions error, got {result:?}"),
    }
}
