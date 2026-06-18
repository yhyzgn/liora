//! Upload quantity limits and disabled state.

use liora_components::{Upload, UploadFile, UploadStatus};

pub fn upload_limited() -> Upload {
    Upload::new()
        .limit(1)
        .accept(".zip,.txt")
        .max_size(10 * 1024 * 1024)
        .button_text("达到数量限制")
        .tip("limit=1 时已有文件，入口自动禁用。")
        .width_lg()
        .add_file(
            UploadFile::new("only", "唯一附件.zip")
                .size(5_120_000)
                .status(UploadStatus::Ready),
        )
}

pub fn upload_disabled() -> Upload {
    Upload::new()
        .disabled(true)
        .button_text("禁用上传")
        .tip("禁用状态下入口不可用。")
        .width_lg()
}
