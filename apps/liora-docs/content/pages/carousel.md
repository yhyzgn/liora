# Carousel

`Carousel` 用于展示功能亮点、图片横幅或可切换的营销内容。它保持 GPUI 原生渲染，支持方向、指示器位置、箭头、自动播放与自定义内容。

## 基础轮播

多项轮播显示箭头时，内容区会为两侧导航预留空间，避免遮住标题、描述或自定义内容；隐藏箭头或只有一项时不占用这部分空间。

### 效果

::Demo{component="CarouselBasic"}::

### 代码

```rust src="carousel/basic.rs"
```

## 外置指示器与自动播放

`autoplay(true)` 由组件自动请求切换，无需上层创建定时器。默认间隔为 3000ms，`interval_ms` 会限制到 250–9,223,372,036,854ms，以保持各原生调度器可表示的范围。`pause_on_hover` 默认为 `true`：鼠标位于整个轮播区域（包括外置指示器）时暂停，移开后重新等待一个完整间隔再恢复。

此示例为受控轮播：创建一次 `autoplay_carousel(cx)`，将返回的实体保存在父视图字段中，并在渲染时使用 `.child(entity.clone())`。示例通过弱引用处理 `on_change`，更新保留的索引并调用 `cx.notify()`；箭头、指示器和自动播放共用此回调。

### 效果

::Demo{component="CarouselAutoplay"}::

### 代码

```rust src="carousel/autoplay.rs"
```

## 自定义内容

### 效果

::Demo{component="CarouselCustom"}::

### 代码

```rust src="carousel/custom.rs"
```

## 状态与回调

- 不传入 `active_index` 时，组件自行保留当前索引；基础示例中的箭头和指示器无需额外状态即可切换，自动播放同样可独立工作。
- 传入 `active_index` 时，该值为权威状态。`on_change(index, window, cx)` 仅请求切换，调用方须保存新索引并触发重绘；忽略回调不会自动改变受控索引。
- 自动播放每个间隔只触发一次切换请求；手动箭头和指示器使用相同的 `on_change` 回调，不要另外添加应用层定时器。
- 渲染位置稳定时使用默认身份；对于动态排列的轮播，为每个实例提供稳定且互不相同的 `.id(...)`，不要在每次渲染时生成新 ID。
