# Plugin System

Piri supports a plugin system that allows you to extend functionality. Plugins run automatically in daemon mode.

## Available Plugins

### [Scratchpads](scratchpads.md)

Quick show/hide windows, cross-workspace/monitor support.

**Key Features**:
- Quick show/hide of frequently used applications
- Cross-workspace and cross-monitor support
- Customizable appearance direction and size

### [Empty Plugin](empty.md)

Empty workspace automation, auto-executes commands on switch.

**Key Features**:
- Automatic command execution on empty workspaces
- Workspace-based configuration
- Similar to Hyprland's `on-created-empty` workspace rule

### [Singleton Plugin](singleton.md)

Manages single-instance windows for global uniqueness.

**Key Features**:
- Smart detection with auto App ID extraction
- Window registry for fast lookup
- Supports post-creation commands (`on_created_command`)

### [Window Rule Plugin](window_rule.md)

Regex-based window placement to specified workspaces, with focus-triggered commands.

**Key Features**:
- Regex matching (`app_id`/`title`, lists supported, OR logic)
- Workspace name/index matching
- Focus-triggered commands with de-duplication

### [Workspace Rule Plugin](workspace_rule.md)

Workspace layout management: auto-width, tiling, alignment, maximization. Built-in EdgePulse indicators.

**Key Features**:
- Auto-width, tiling, alignment, maximization
- EdgePulse edge indicators with animation
- Workspace-aware, independent configuration

### [Window Order Plugin](window_order.md)

Weight-based window reordering. Larger weight = further left.

**Key Features**:
- Intelligent sorting, minimizes moves
- Manual/event-driven trigger support
- Supports `app_id` partial matching

### [Swallow Plugin](swallow.md)

Window swallowing, child replaces parent in layout.

**Key Features**:
- PID-based parent-child matching (default)
- Rule-based matching (`app_id`/`title`/`pid`)
- Intelligent focus window queue

### [Mark Plugin](mark.md)

Named marks for windows via `piri mark …`: bind/focus windows. Bindings in daemon memory only.

**Key features**:
- `toggle`, `add`, and `delete` operations
- Works well with Niri `spawn` keybindings or a launcher
- Optional refocus: toggle same mark to jump back to previous window

### [Sticky Plugin](sticky.md)

Pin floating window to follow focused workspace. Ideal for utility windows.

**Key features**:
- `add` and `delete` commands
- `--cross` controls cross-monitor behavior
- Floating windows only

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
sticky = true
```

**Default Behavior**:
- If not explicitly specified, plugins are **disabled** by default (`false`)
- Set each plugin to `true` explicitly to enable it
- Exception: `window_rule` is enabled by default if window rules are configured (unless explicitly set to `false`)
