# Natural - macOS Menubar Scroll Toggle

## Context

macOS ties natural scrolling for trackpad and mouse together under a single setting. Switching between an external mouse and trackpad forces the user to open Settings > Trackpad > Natural scrolling every time. This app provides a one-click toggle from the macOS menubar.

## Approach

Build a **tray-only Tauri v2 app** (no visible window, no Dock icon). The entire UI is a menubar dropdown menu. All logic runs on the Rust side.

**Toggle mechanism:** `defaults write -g com.apple.swipescrolldirection -bool true/false` + instant apply via Apple's private `PreferencePanesSupport.framework` using `libloading` (Rust FFI). Fallback to `SwipeScrollDirectionDidChangeNotification` via Swift CLI if the private framework is unavailable.

## File Structure

```
natural/
  src-tauri/
    src/
      main.rs          -- Tray setup, menu events, dock hiding
      scroll.rs         -- Read/write/apply scroll direction
    Cargo.toml
    tauri.conf.json     -- No windows, tray icon, bundle config
    capabilities/
      default.json      -- Minimal (no frontend permissions needed)
    icons/
      icon.png          -- 22x22 monochrome template icon for menubar
  src/
    index.html          -- Empty placeholder (Tauri requirement)
  package.json
  Makefile              -- dev, build, install targets
```

## Implementation Steps

### 1. Scaffold Tauri v2 project
- `npm create tauri-app@latest . -- --template vanilla --manager npm`
- Clean up generated defaults (remove demo content, simplify index.html to empty body)

### 2. Configure tray-only app (`src-tauri/tauri.conf.json`)
- `"windows": []` -- no visible window
- `"trayIcon": { "iconPath": "icons/icon.png", "iconAsTemplate": true }`
- Bundle targets: `["dmg", "app"]`
- Product name: `Natural`, identifier: `com.natural.scrolltoggle`
- No `beforeDevCommand`/`beforeBuildCommand` (plain HTML, no build step)

### 3. Add Rust dependencies (`src-tauri/Cargo.toml`)
- `tauri` with features `["tray-icon", "image-png"]`
- `tauri-plugin-shell = "2"` (scaffolded by default, can keep)
- `libloading = "0.8"` (dynamic framework loading)
- `serde`, `serde_json`

### 4. Implement scroll toggle logic (`src-tauri/src/scroll.rs`)
- `is_natural_scrolling()` -- runs `defaults read -g com.apple.swipescrolldirection`, returns bool
- `toggle_natural_scrolling()` -- flips state, returns new value
- `set_natural_scrolling(bool)` -- writes via `defaults write` then calls `apply_immediately()`
- `apply_immediately(bool)` -- loads `PreferencePanesSupport.framework` via `libloading`, calls `setSwipeScrollDirection`. On failure, falls back to posting `SwipeScrollDirectionDidChangeNotification` via `swift -e`

### 5. Implement tray and menu (`src-tauri/src/main.rs`)
- Build menu: status label (disabled/non-clickable), separator, "Toggle Scrolling", "Quit"
- `TrayIconBuilder` with `menu_on_left_click(true)` -- both left and right click show menu
- On "toggle" event: call `scroll::toggle_natural_scrolling()`, update status label text
- On "quit" event: `app.exit(0)`
- `set_activation_policy(Accessory)` -- hides app from Dock, menubar-only

### 6. Create tray icon
- Simple 22x22 monochrome PNG with transparency (up-down arrow or scroll symbol)
- `iconAsTemplate: true` makes macOS auto-adapt to light/dark menubar
- Generate app bundle icons with `npx tauri icon`

### 7. Create Makefile
```makefile
dev:
	npm run tauri dev
build:
	npm run tauri build
install: build
	cp -r src-tauri/target/release/bundle/macos/Natural.app /Applications/
clean:
	cd src-tauri && cargo clean
```

### 8. Build and test
- Run `npm run tauri dev`, verify tray icon appears
- Click toggle, verify scroll direction changes immediately
- Build .dmg with `npm run tauri build`

## Risks

| Risk | Mitigation |
|------|-----------|
| Private framework removed in future macOS | Fallback to DistributedNotification. `defaults write` still persists the pref. |
| `MenuItem::set_text()` doesn't visually update | Rebuild entire menu via `tray.set_menu()` as workaround. |
| Gatekeeper blocks unsigned app | For personal use: `xattr -cr Natural.app`. |

## Verification

1. `npm run tauri dev` -- tray icon appears in menubar, no Dock icon
2. Click tray -- menu shows current state ("Natural Scrolling: ON/OFF")
3. Click "Toggle Scrolling" -- scroll direction changes immediately (test with trackpad)
4. Menu label updates to reflect new state
5. `npm run tauri build` -- produces `.app` and `.dmg` in `src-tauri/target/release/bundle/`
6. Copy `.app` to `/Applications/`, launch, verify it works standalone
