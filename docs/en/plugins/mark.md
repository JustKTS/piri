# Mark Plugin

The Mark plugin provides **named quick-focus targets**: bind a string (often a single letter, e.g. `a`) to the current window, then jump back to that window using the same name. Bindings live in the daemon’s memory only; **nothing is written to the config file**.

## Configuration

Enable the plugin:

```toml
[piri.plugins]
mark = true
```

No extra `[mark.*]` tables are required; all marks are created or removed at runtime via the CLI.

## Command Line

```bash
# If the mark points to a window that still exists → focus it; else bind the focused window to this mark
piri mark {name} toggle

# Remove this mark (success even if it did not exist)
piri mark {name} delete

# Bind the focused window to this mark (replaces any previous binding)
piri mark {name} add
```

Examples:

```bash
piri mark a toggle   # First time: mark current window as a; later: jump to a
piri mark a add      # Force re-bind current window to a
piri mark a delete   # Clear a
```

## Behavior

1. **`toggle`**: If `name` is bound and the window still exists, **focus** that window; otherwise (unbound or window closed) **bind** the currently focused window to `name`.
2. **`add`**: Always sets `name` to the current focus, overwriting any previous binding without checking the old window.
3. **`delete`**: Removes the binding for `name`; idempotent.

To **change** which window a mark points to while the old binding is still valid, use **`add`**, or **`delete`** then **`toggle`**.

## Niri Keybindings

Piri cannot listen for the “next key” by itself. In Niri you typically add one `spawn` per mark you care about, for example:

```kdl
binds {
    Mod+Shift+A { spawn "piri" "mark" "a" "toggle"; }
    Mod+Shift+B { spawn "piri" "mark" "b" "toggle"; }
}
```

If Niri gains multi-key sequences or binding modes, you can group these under one prefix. See the main README for context.

## Limitations

- **Not persistent**: All marks are lost when `piri daemon` restarts.
- **No on-window labels**: Niri IPC does not provide drawing mark letters on window decorations; use a bar, notifications, or an external script if you need a visible list.
- **Requires focus**: `toggle` / `add` use the **currently focused** window; focus the target before invoking.

## Use Cases

- Short-lived bookmarks for a few windows you switch between often.
- Pairing with a launcher (e.g. `fuzzel` listing `a`–`z` then calling `piri mark …`) so you do not type the mark name manually every time.
