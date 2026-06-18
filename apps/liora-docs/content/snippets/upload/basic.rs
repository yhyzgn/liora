//! Basic Upload file list.

use liora_components::{Upload, UploadFile, UploadStatus};

pub fn upload_basic() -> Upload {
    Upload::new()
        .button_text("选择文件")
        .accept(".pdf,.fig,.txt")
        .max_size(5 * 1024 * 1024)
        .tip("仅接受 pdf/fig/txt，单文件 ≤ 5MB。")
        .width_lg()
        .add_file(
            UploadFile::new("spec", "产品需求说明.pdf")
                .size(428_000)
                .status(UploadStatus::Success),
        )
        .add_file(
            UploadFile::new("draft", "设计稿-v2.fig")
                .size(2_480_000)
                .status(UploadStatus::Uploading)
                .progress(68),
        )
}

fn main() {
    let _ = upload_basic();
}
