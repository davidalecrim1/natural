# Natural

macOS menubar app to toggle natural scrolling with one click.

## Architecture

Tray-only Tauri v2 app. No visible window, no Dock icon. All logic runs on the Rust side.

- `src-tauri/src/main.rs` -- Tray setup, menu construction, event handling
- `src-tauri/src/scroll.rs` -- Read/write/apply scroll direction
- `src-tauri/tauri.conf.json` -- Tauri config (no windows, bundle settings)
- `src/index.html` -- Empty placeholder (Tauri requirement, never shown)

## Toggle mechanism

1. `defaults write -g com.apple.swipescrolldirection -bool true/false` persists the preference
2. `libloading` loads Apple's private `PreferencePanesSupport.framework` and calls `setSwipeScrollDirection` for instant effect
3. Fallback: posts `SwipeScrollDirectionDidChangeNotification` via Swift CLI if the private framework is unavailable

## Commands

```
make dev                        # Run in development mode
make build                      # Production build (.app + .dmg)
make install                    # Build and copy to /Applications
make clean                      # Clean Rust build artifacts
make lint                       # Check formatting and run Clippy
make test                       # Run Rust unit tests
make release VERSION=x.y.z      # Bump versions, build, tag, push, create GitHub release
```

## Prerequisites

- Rust (rustup)
- Node.js + npm
- Xcode Command Line Tools

## Workflow

After completing any code change, always run:

```
make lint
make test
```

## Conventions

- No frontend framework -- plain HTML, all logic in Rust
- Tray icon is managed entirely in Rust code (not in tauri.conf.json) to avoid duplicate icons
- Icon uses `icon_as_template(true)` for macOS light/dark menubar adaptation
- App hides from Dock via `set_activation_policy(Accessory)`
