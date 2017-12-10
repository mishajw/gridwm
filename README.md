# gridwm

Variables:
- `DIR`: `left`, `right`, `up`, `down`

Operations:
- `go <DIR>` go to the workspace in some direction
- `move-ws <DIR>` move the workspace in some direction by switching
- `move-win <DIR>` move the current window to the workspace in some direction

Decompose command into series of commands, for example:
- `go <DIR>`
  - current x, y <- `[pri] get-current`
  - new x, y <- current x, y + direction
  - exists <- `[pri] exists x y`
    - if not exists, `[pri] create x y`
  - `[pri] set-current x y`

To do:
- [x] Implement operations for BSPWM
- [ ] Support multiple workspaces
- [ ] Make workspace status subscribable
- [ ] Put `bspc` subscribing into background thread

