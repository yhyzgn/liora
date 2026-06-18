//! Upload drag-style drop zone.

use liora_components::{Upload, UploadFile, UploadStatus};

pub fn upload_drag() -> Upload {
    Upload::new()
        .drag(true)
        .multiple(true)
        .accept(".png,.jpg,.jpeg,.pdf")
        .max_size(2 * 1024 * 1024)
        .button_text("拖拽文件到这里上传")
        .tip("真实拖放接入可由宿主扩展；组件提供原生拖拽区域样式。")
        .width_lg()
        .add_file(
            UploadFile::new("error", "合同扫描件.jpg")
                .size(820_000)
                .status(UploadStatus::Error)
                .description("网络中断"),
        )
}
