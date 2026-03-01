# Natural

A macOS menubar app to toggle natural scrolling with one click. No more digging through Settings > Trackpad every time you switch between an external mouse and trackpad. Intentionally minimal -- one feature, no bloat.

## How it works

Natural lives in your menubar. Click the icon, hit "Toggle Scrolling", done. The change applies instantly -- no logout required.

## Keyboard shortcut

Press **Cmd+Ctrl+N** anywhere to toggle natural scrolling without opening the menu. A native macOS notification confirms the new state (ON/OFF).

## Compatibility

| macOS Version | Status |
|---|---|
| macOS 26 (Tahoe) | Tested, works |
| macOS 15 (Sequoia) | Expected to work |
| macOS 14 (Sonoma) | Expected to work |
| macOS 13 (Ventura) | Expected to work |

The app uses Apple's private `PreferencePanesSupport.framework` for instant toggle. If Apple removes this in a future release, the app falls back to a `DistributedNotification`, and the `defaults write` preference is always persisted regardless.

**Architecture:** Apple Silicon (arm64). Build from source for Intel.

## Install

### From source

```
git clone <repo-url>
cd natural
npm install
make install
```

This builds the app and copies `Natural.app` to `/Applications`.

### From .dmg

Download the latest `.dmg` from releases, open it, and drag `Natural.app` to Applications.

### Gatekeeper note

Since the app is not code-signed for distribution, macOS may block it on first launch. To allow it:

```
xattr -cr /Applications/Natural.app
```

Or right-click the app > Open > Open.

## Development

### Prerequisites

- [Rust](https://rustup.rs/)
- Node.js + npm
- Xcode Command Line Tools (`xcode-select --install`)

### Commands

```
make dev       # Run in development mode
make build     # Production build (.app + .dmg)
make install   # Build and copy to /Applications
make clean     # Clean build artifacts
```

## How the toggle works

1. Reads current state via `defaults read -g com.apple.swipescrolldirection`
2. Writes the new value via `defaults write -g com.apple.swipescrolldirection -bool <value>`
3. Applies immediately by calling `setSwipeScrollDirection` from Apple's private `PreferencePanesSupport.framework` via Rust FFI
4. Falls back to posting `SwipeScrollDirectionDidChangeNotification` if the framework is unavailable

## Built with

- [Tauri v2](https://v2.tauri.app/) -- lightweight native app framework
- Rust -- all app logic
- [libloading](https://crates.io/crates/libloading) -- dynamic framework loading for instant toggle

## License

MIT
