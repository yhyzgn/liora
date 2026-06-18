use liora_components::{Upload, UploadFile, UploadRejectReason};

#[test]
fn detects_when_upload_can_accept_more_files() {
    assert!(Upload::can_accept_more_len(0, Some(1), false));
    assert!(!Upload::can_accept_more_len(1, Some(1), false));
    assert!(!Upload::can_accept_more_len(0, None, true));
}

#[test]
fn clamps_upload_progress_to_percent() {
    let file = UploadFile::new("large", "large.bin").progress(142);

    assert_eq!(file.progress, 100);
}

#[test]
fn validates_file_type_by_extension_and_mime_group() {
    assert!(Upload::matches_accept_name("avatar.PNG", Some(".png,.jpg")));
    assert!(Upload::matches_accept_name("cover.webp", Some("image/*")));
    assert!(!Upload::matches_accept_name("archive.zip", Some("image/*")));
}

#[test]
fn rejects_files_over_max_size() {
    let result =
        Upload::validate_file_name_size("avatar.png", Some(2048), Some(".png"), Some(1024));

    assert_eq!(result, Err(UploadRejectReason::TooLarge));
}
