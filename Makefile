.PHONY: dev build install clean

dev:
	npm run tauri dev

build:
	npm run tauri build

install: build
	cp -r src-tauri/target/release/bundle/macos/Natural.app /Applications/

clean:
	cd src-tauri && cargo clean
