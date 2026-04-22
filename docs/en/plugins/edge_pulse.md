# EdgePulse Edge Indicator

EdgePulse is a sub-feature of the [Workspace Rule](workspace_rule.md) plugin. It renders left/right edge hints on the screen when the focused column reaches the workspace edge, providing a visual cue for navigation boundaries.

![edge_pulse_1.png](../assets/edge_pulse_1.png)
![edge_pulse_2.png](../assets/edge_pulse_2.png)

## Configuration

Enable EdgePulse in your configuration file under the workspace rule section:

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

### Workspace-Specific Configuration

You can override EdgePulse settings per workspace:

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

## Configuration Parameters

| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `enabled` | `bool` | `false` | Enable EdgePulse missing-neighbor detection and hints |
| `show_left` | `bool` | `true` | Show left hint when reaching the workspace left edge |
| `show_right` | `bool` | `true` | Show right hint when reaching the workspace right edge |
| `width` | `u32` | `14` | Hint width in pixels |
| `height_ratio` | `f64` | `0.42` | Hint height ratio against output height (0.0-1.0) |
| `left_gradient_start` | `String` | `#68d8ff` | Left hint gradient start color |
| `left_gradient_end` | `String` | `#1f4fff` | Left hint gradient end color |
| `right_gradient_start` | `String` | `#ffd36a` | Right hint gradient start color |
| `right_gradient_end` | `String` | `#ff7a1f` | Right hint gradient end color |
| `alpha` | `f64` | `0.85` | Global alpha (0.0-1.0) |

## How It Works

1. On window focus change, workspace switch, window open/close, the plugin evaluates the focused column's position in the scrolling layout.
2. If the focused column reaches the workspace left (or right) edge — meaning no other tiled columns exist in that direction — a colored indicator is rendered on the corresponding screen edge.
3. The indicator uses a vertical gradient with horizontal alpha falloff for a soft edge appearance.
4. All indicators are rendered as Wayland layer-shell overlay surfaces (via `zwlr_layer_shell_v1`), ensuring they appear above all windows without intercepting input events.

## Behavior Details

- Indicators only appear when there are **2 or more columns** in the workspace. A single-column layout shows no indicators.
- Focusing a floating window preserves the current indicator state.
- Switching workspaces forces re-evaluation; each workspace may have its own EdgePulse style.
- Indicator state is cached to avoid redundant renders.
- Indicators are automatically hidden when EdgePulse is disabled or no focused window exists.

## Requirements

- A wlroots-based compositor (e.g., Niri) that provides `zwlr_layer_shell_v1`.
- The `WAYLAND_DISPLAY` environment variable must be set.
