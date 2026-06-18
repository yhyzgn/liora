#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Category {
    Basic,
    Form,
    Data,
    Navigation,
    Feedback,
    Others,
}

#[allow(dead_code)]
impl Category {
    pub const ALL: &'static [Category] = &[
        Category::Basic,
        Category::Form,
        Category::Data,
        Category::Navigation,
        Category::Feedback,
        Category::Others,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Category::Basic => "Basic 基础",
            Category::Form => "Form 表单",
            Category::Data => "Data 数据",
            Category::Navigation => "Navigation 导航",
            Category::Feedback => "Feedback 反馈",
            Category::Others => "Others 其他",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Category::Basic => "⊞",
            Category::Form => "☰",
            Category::Data => "⊟",
            Category::Navigation => "☈",
            Category::Feedback => "⚡",
            Category::Others => "⋯",
        }
    }

    #[allow(dead_code)]
    pub fn order(&self) -> usize {
        match self {
            Category::Basic => 0,
            Category::Form => 1,
            Category::Data => 2,
            Category::Navigation => 3,
            Category::Feedback => 4,
            Category::Others => 5,
        }
    }
}
