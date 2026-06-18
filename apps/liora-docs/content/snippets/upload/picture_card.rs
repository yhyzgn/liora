//! Upload picture-card list type.

use liora_components::{Upload, UploadFile, UploadStatus};

pub fn upload_picture_card() -> Upload {
    Upload::new()
        .picture_card()
        .button_text("上传图片")
        .multiple(true)
        .accept("image/*")
        .max_size(2 * 1024 * 1024)
        .width_lg()
        .files([
            UploadFile::new("cover", "cover.png")
                .size(512_000)
                .status(UploadStatus::Success),
            UploadFile::new("banner", "banner.jpg")
                .size(1_240_000)
                .status(UploadStatus::Uploading)
                .progress(42),
        ])
}
