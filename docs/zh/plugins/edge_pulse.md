# EdgePulse 边缘提示

EdgePulse 是 [Workspace Rule](workspace_rule.md) 插件的子功能。当当前聚焦列到达工作区左/右边缘时，在屏幕对应边缘渲染提示指示器，提供导航边界的视觉提示。

![edge_pulse_1.png](../assets/edge_pulse_1.png)
![edge_pulse_2.png](../assets/edge_pulse_2.png)

## 配置

在配置文件的 workspace rule 段启用 EdgePulse：

```toml
[piri.plugins]
workspace_rule = true

[piri.workspace_rule.edge_pulse]
enabled = true
show_left = true
show_right = true
width = 14
height_ratio = 0.42
left_gradient_start = "#68d8ff"
left_gradient_end = "#1f4fff"
right_gradient_start = "#ffd36a"
right_gradient_end = "#ff7a1f"
alpha = 0.85
```

### 工作区级别配置

可以为特定工作区覆盖 EdgePulse 设置：

```toml
[workspace_rule.main.edge_pulse]
enabled = true
show_left = true
show_right = true
width = 10
height_ratio = 0.5
left_gradient_start = "#80e0ff"
left_gradient_end = "#2060ff"
alpha = 0.9
```

## 配置参数

| 参数 | 类型 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- |
| `enabled` | `bool` | `false` | 是否启用 EdgePulse 左右侧缺邻居检测与提示 |
| `show_left` | `bool` | `true` | 到达工作区左边缘时是否显示左侧提示 |
| `show_right` | `bool` | `true` | 到达工作区右边缘时是否显示右侧提示 |
| `width` | `u32` | `14` | 提示宽度（像素） |
| `height_ratio` | `f64` | `0.42` | 提示高度占输出高度的比例（0.0-1.0） |
| `left_gradient_start` | `String` | `#68d8ff` | 左侧提示渐变起始色 |
| `left_gradient_end` | `String` | `#1f4fff` | 左侧提示渐变结束色 |
| `right_gradient_start` | `String` | `#ffd36a` | 右侧提示渐变起始色 |
| `right_gradient_end` | `String` | `#ff7a1f` | 右侧提示渐变结束色 |
| `alpha` | `f64` | `0.85` | 全局透明度（0.0-1.0） |

## 工作原理

1. 在窗口焦点变化、工作区切换、窗口打开/关闭时，插件评估当前聚焦列在滚动布局中的位置。
2. 如果聚焦列到达工作区左（或右）边缘（即该方向没有其他平铺列），则在对应屏幕边缘渲染彩色指示器。
3. 指示器使用垂直渐变配合水平 alpha 衰减，呈现柔和的边缘效果。
4. 所有指示器均以 Wayland layer-shell overlay surface 渲染（通过 `zwlr_layer_shell_v1`），确保显示在所有窗口上方且不拦截输入事件。

## 行为说明

- 仅当工作区中存在 **2 个或更多列**时才显示指示器，单列布局不显示。
- 聚焦浮动窗口时保持当前指示器状态不变。
- 切换工作区时强制重新评估；每个工作区可使用独立的 EdgePulse 样式。
- 指示器状态会被缓存，避免冗余渲染。
- 禁用 EdgePulse 或无聚焦窗口时，指示器自动隐藏。

## 系统要求

- 基于 wlroots 的合成器（如 Niri），需提供 `zwlr_layer_shell_v1`。
- 必须设置 `WAYLAND_DISPLAY` 环境变量。
