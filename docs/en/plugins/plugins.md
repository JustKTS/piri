# Plugin System

Piri supports a plugin system that allows you to extend functionality. Plugins run automatically in daemon mode.

## Available Plugins

### [Scratchpads](scratchpads.md)

A powerful window management feature that allows you to quickly show and hide windows of frequently used applications. Supports cross-workspace and cross-monitor functionality.

**Key Features**:
- Quick show/hide of frequently used applications
- Cross-workspace and cross-monitor support
- Customizable appearance direction and size

### [Empty Plugin](empty.md)

Executes commands when switching to specific empty workspaces. Useful for automating workflows, such as automatically launching applications in empty workspaces.

**Key Features**:
- Automatic command execution on empty workspaces
- Workspace-based configuration
- Similar to Hyprland's `on-created-empty` workspace rule

### [Mark Plugin](mark.md)

Named marks for windows via `piri mark …`: bind the focused window or jump back to a marked window. Bindings are kept in daemon memory only; no per-mark tables in the config.

**Key features**:
- `toggle`, `add`, and `delete` operations
- Works well with Niri `spawn` keybindings or a launcher

### [Window Rule Plugin](window_rule.md)

Automatically moves windows to specified workspaces based on their `app_id` or `title`. Useful for automating window management and organizing applications.

**Key Features**:
- Automatic window assignment to workspaces
- Match by `app_id` or `title` (with regex support)
- Similar to Hyprland's window rules

### [Autofill Plugin](autofill.md)

Automatically aligns the last column of windows to the rightmost position when windows are closed or layout changes. Helps maintain a clean and organized window layout.

**Key Features**:
- Pure event-driven, real-time alignment
- Zero configuration required
- Focus preservation - maintains user's focused window
- Workspace-aware operation

## Plugin Control

You can control which plugins are enabled or disabled in the configuration file:

```toml
[piri.plugins]
scratchpads = true
empty = true
window_rule = true
workspace_rule = true
singleton = true
window_order = true
swallow = true
mark = true
```

**Default Behavior**:
- If not explicitly specified, plugins are **disabled** by default (`false`)
- Set each plugin to `true` explicitly to enable it
- Exception: `window_rule` is enabled by default if window rules are configured (unless explicitly set to `false`)
