.PHONY: dev build install clean release lint test

dev:
	npm run tauri dev

build:
	npm run tauri build

install: build
	cp -r src-tauri/target/release/bundle/macos/Natural.app /Applications/

clean:
	cd src-tauri && cargo clean

lint:
	cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings

test:
	cd src-tauri && cargo test

release:
ifndef VERSION
	$(error Usage: make release VERSION=0.2.0)
endif
	@echo "Releasing v$(VERSION)..."
	sed -i '' 's/"version": ".*"/"version": "$(VERSION)"/' src-tauri/tauri.conf.json
	sed -i '' 's/^version = ".*"/version = "$(VERSION)"/' src-tauri/Cargo.toml
	sed -i '' 's/"version": ".*"/"version": "$(VERSION)"/' package.json
	$(MAKE) build
	git checkout -- src-tauri/tauri.conf.json src-tauri/Cargo.toml package.json
	git tag v$(VERSION)
	git push origin v$(VERSION)
	gh release create v$(VERSION) \
		src-tauri/target/release/bundle/dmg/Natural_$(VERSION)_aarch64.dmg \
		--title "v$(VERSION)" \
		--generate-notes
