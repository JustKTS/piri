# Workspace Rule 插件

Workspace Rule 插件为工作区提供自动化的窗口布局管理功能，包括自动宽度调整、自动平铺、自动对齐和自动最大化等功能。该插件整合了原本的 Autofill 功能，提供了更全面的工作区窗口管理能力。

## 功能特性

Workspace Rule 插件提供以下功能：

- **自动宽度调整** (`auto_width`): 根据窗口数量自动调整窗口宽度
- **自动平铺** (`auto_tile`): 自动将新窗口合并到现有列中（第一列除外）
- **自动对齐** (`auto_fill`): 自动将最后一列窗口对齐到最右侧（原 Autofill 功能）
- **自动最大化** (`auto_maximize`): 当只有一个窗口时自动最大化，多个窗口时自动取消最大化
- **EdgePulse 边缘提示** (`edge_pulse`): 当当前聚焦列到达工作区左/右边缘时，触发对应侧边缘提示

## 演示视频

![Autofill 演示视频](../assets/autofill.mp4)

![Autofill 演示视频 1](../assets/autofill_1.mp4)

![Autofill 演示视频 2](../assets/autofill_2.mp4)

![自动平铺演示视频](../assets/auto_tile.mp4)

## 配置

### 基本配置

在配置文件中启用插件：

```toml
[piri.plugins]
workspace_rule = true
```

### 默认配置

使用 `[piri.workspace_rule]` 配置默认设置，这些设置会应用到所有没有特定配置的工作区：

```toml
[piri.workspace_rule]
# 自动宽度配置：数组索引对应窗口数量（从 1 开始）
# 每个元素可以是字符串（所有窗口相同宽度）或数组（每个窗口不同宽度）
auto_width = ["100%", "50%", "33.33%", "25%", "20%"]
# 自动平铺：允许每列最多 2 个窗口（第一列除外）
auto_tile = false
# 自动对齐：自动将最后一列对齐到最右侧
auto_fill = false
# 自动最大化：只有一个窗口时最大化，多个窗口时取消最大化
auto_maximize = false
```

### 工作区特定配置

使用 `[workspace_rule.{workspace}]` 为特定工作区配置规则，工作区标识符可以是名称（如 `"browser"`）或索引（如 `"1"`）：

```toml
# 工作区索引配置
[workspace_rule.1]
auto_width = ["100%", "50%", "33.33%", "25%", "20%"]

# 工作区名称配置
[workspace_rule.browser]
# 1 个窗口：100%，2 个窗口：45% 和 55%，3 个窗口：33.33% 每个
auto_width = ["100%", ["45%", "55%"], "33.33%"]

# 启用自动最大化
[workspace_rule.main]
auto_maximize = true

# 启用自动对齐（原 Autofill 功能）
[workspace_rule.dev]
auto_fill = true
```

### 配置参数说明

| 参数 | 类型 | 说明 |
| :--- | :--- | :--- |
| `auto_width` | `Vec<Vec<String>>` | 自动宽度配置数组，索引对应窗口数量（从 1 开始）。每个元素可以是字符串（所有窗口相同宽度）或数组（每个窗口不同宽度）。宽度值必须是百分比格式（如 `"50%"`） |
| `auto_tile` | `bool` | 如果为 `true`，自动将新窗口合并到现有列中（第一列除外）。当某个非第一列只有一个窗口时，新窗口会被合并到该列 |
| `auto_fill` | `bool` | 如果为 `true`，自动将最后一列窗口对齐到最右侧（原 Autofill 功能） |
| `auto_maximize` | `bool` | 如果为 `true`，当工作区只有一个窗口时自动最大化到边缘，多个窗口时自动取消最大化 |
| `edge_pulse.enabled` | `bool` | 是否启用 EdgePulse 工作区边缘检测与提示 |
| `edge_pulse.show_left` | `bool` | 到达工作区左边缘时是否显示左侧提示 |
| `edge_pulse.show_right` | `bool` | 到达工作区右边缘时是否显示右侧提示 |
| `edge_pulse.width` | `u32` | 提示宽度（像素） |
| `edge_pulse.height_ratio` | `f64` | 提示高度占输出高度的比例（0.0-1.0） |
| `edge_pulse.left_gradient_start` | `String` | 左侧渐变起始色（如 `#68d8ff`） |
| `edge_pulse.left_gradient_end` | `String` | 左侧渐变结束色 |
| `edge_pulse.right_gradient_start` | `String` | 右侧渐变起始色 |
| `edge_pulse.right_gradient_end` | `String` | 右侧渐变结束色 |
| `edge_pulse.alpha` | `f64` | 全局透明度（0.0-1.0） |

## 工作原理

### 自动宽度调整 (`auto_width`)

插件根据工作区中的**列数**（而非窗口数）来应用宽度配置：

1. 统计当前工作区中平铺窗口的列数
2. 根据列数查找对应的宽度配置（数组索引 = 列数 - 1）
3. 为每一列设置对应的宽度百分比

**示例**：
- 配置 `auto_width = ["100%", "50%", ["30%", "35%", "35%"]]`
- 1 列：所有窗口宽度为 100%
- 2 列：每列宽度为 50%
- 3 列：第一列 30%，第二列 35%，第三列 35%

### 自动平铺 (`auto_tile`)

当新窗口打开时：

1. 检查当前工作区中非第一列的列
2. 查找只有一个窗口的列
3. 将新窗口合并到该列中（使用 swallow 机制）

**注意**：第一列不会被合并，新窗口会创建新列。

### 自动对齐 (`auto_fill`)

当窗口关闭或布局改变时：

1. 保存当前聚焦的窗口
2. 聚焦第一列，然后聚焦最后一列（使所有列对齐到最右侧）
3. 恢复之前聚焦的窗口

这是原 Autofill 插件的功能，现已整合到 Workspace Rule 插件中。

### 自动最大化 (`auto_maximize`)

当窗口数量变化时：

1. **只有一个窗口**：自动最大化到边缘
2. **多个窗口**：自动取消最大化，恢复正常的宽度调整

**注意**：插件会跟踪已最大化的窗口，避免重复处理。

## 事件处理

插件监听以下事件：

- `WindowOpenedOrChanged`: 处理新窗口打开和窗口状态变化
- `WindowClosed`: 处理窗口关闭
- `WindowLayoutsChanged`: 处理布局变化（用于自动对齐）

### 窗口状态跟踪

插件会跟踪以下状态：

- **已见窗口** (`seen_windows`): 区分新窗口和已存在的窗口
- **窗口浮动状态** (`window_floating_state`): 检测浮动/平铺状态变化
- **已最大化窗口** (`maximized_windows`): 跟踪由 `auto_maximize` 最大化的窗口

### 节流机制

`apply_widths` 函数使用 400ms 的节流机制，避免频繁触发：

- 第一个请求立即执行
- 400ms 内的后续请求会被忽略

## 配置示例

### 示例 1: 基础宽度配置

```toml
[piri.plugins]
workspace_rule = true

[piri.workspace_rule]
# 1 个窗口：100%，2 个窗口：各 50%，3 个窗口：各 33.33%
auto_width = ["100%", "50%", "33.33%"]
```

### 示例 2: 自定义宽度配置

```toml
[workspace_rule.dev]
# 1 个窗口：100%，2 个窗口：45% 和 55%，3 个窗口：30%, 35%, 35%
auto_width = ["100%", ["45%", "55%"], ["30%", "35%", "35%"]]
```

### 示例 3: 启用自动最大化

```toml
[workspace_rule.main]
auto_maximize = true
```

### 示例 4: 启用自动对齐（原 Autofill）

```toml
[workspace_rule.browser]
auto_fill = true
```

### 示例 5: 启用自动平铺

```toml
[workspace_rule.work]
auto_tile = true
```

### 示例 6: 组合配置

```toml
[piri.workspace_rule]
# 默认配置
auto_width = ["100%", "50%", "33.33%"]
auto_fill = true

[workspace_rule.main]
# 主工作区：启用自动最大化
auto_maximize = true

[workspace_rule.dev]
# 开发工作区：自定义宽度 + 自动平铺
auto_width = ["100%", ["45%", "55%"], ["30%", "35%", "35%"]]
auto_tile = true
```

## 从 Autofill 迁移

如果你之前使用 Autofill 插件，可以按以下方式迁移：

**旧配置**：
```toml
[piri.plugins]
autofill = true
```

**新配置**：
```toml
[piri.plugins]
workspace_rule = true

# 全局启用自动对齐（原 Autofill 功能）
[piri.workspace_rule]
auto_fill = true

# 或为特定工作区启用
[workspace_rule.main]
auto_fill = true
```

## 特性

- ✅ **工作区感知**: 每个工作区可以独立配置
- ✅ **灵活配置**: 支持默认配置和工作区特定配置
- ✅ **事件驱动**: 实时响应窗口变化
- ✅ **节流优化**: 避免频繁触发，提升性能
- ✅ **状态跟踪**: 智能跟踪窗口状态，避免重复处理
- ✅ **整合功能**: 整合了原 Autofill 插件的功能

## 使用场景

- **多窗口布局管理**: 自动调整窗口宽度，保持整洁的布局
- **单窗口最大化**: 只有一个窗口时自动最大化，提升专注度
- **自动对齐**: 窗口关闭后自动对齐，保持界面整洁
- **智能平铺**: 自动将新窗口合并到现有列，优化空间利用

## 技术细节

### 列数统计

插件通过 `pos_in_scrolling_layout` 字段来统计列数：

- 只统计平铺窗口（非浮动窗口）
- 每个列只需要一个窗口 ID 即可设置宽度
- 列索引从 1 开始

### 宽度解析

宽度值必须是百分比格式（如 `"50%"`），范围 0-100%：

- 支持小数（如 `"33.33%"`）
- 必须以 `%` 结尾
- 解析失败会记录警告并跳过

### 窗口匹配

插件使用工作区名称或 ID 来匹配窗口：

- 优先使用 `workspace` 字段（名称）
- 如果不存在，使用 `workspace_id` 字段（ID）

## 限制

- 宽度配置最多支持 5 列（索引 0-4）
- 超过 5 列的布局不会应用宽度调整
- 浮动窗口不会参与宽度调整和自动平铺
- 自动最大化功能只适用于平铺窗口

