# Grid

`Grid` 是 Liora 的二维布局控件，适合图标墙、卡片墙、设置入口、仪表盘摘要等场景。它不是数据表格；结构化数据请使用 `Table` 或 `VirtualizedTable`。

## 两种横向自适应

### 效果

::Demo{component="Grid"}::

### 代码

```rust src="gallery/grid_demo.rs"
```

## API 边界

- `Grid::fit_item(size)` / `fit_item_sm()` / `fit_item_md()` / `fit_item_lg()`：item 尺寸固定，容器宽度变化时自动改变列数。
- `Grid::fit_columns(n)`：列数固定，item 随容器宽度缩放。
- `GridItem` 默认正方形并居中内容，适合图标墙；调用 `rectangular()` 可变成内容高度自适应卡片。
- `GridItem::on_click(...)` 会自动启用 pointer + hover 反馈，适合“点击复制 IconName”这类网格交互。
- `GridItem::hover_group(group)` 可把卡片 hover 状态同步给内部 `Icon::group_hover_primary(group)` / `Text::group_hover_primary(group)`，适合图标墙 hover 时图标与名称一起变成主题 primary 色。
