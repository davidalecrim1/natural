.PHONY: dev build _build install clean release lint test

dev:
	npm run tauri dev

_build:
	npm run tauri build -- --bundles app

build:
	npm run tauri build -- --bundles app,dmg

install: _build
	cp -r src-tauri/target/release/bundle/macos/Natural.app /Applications/

clean:
	cd src-tauri && cargo clean

lint:
	cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings

test:
	cd src-tauri && cargo test

release:
ifndef VERSION
	$(error Usage: make release VERSION=x.y.z)
endif
	$(eval SEMVER := $(patsubst v%,%,$(VERSION)))
	@echo "Releasing v$(SEMVER)..."
	sed -i '' 's/"version": ".*"/"version": "$(SEMVER)"/' src-tauri/tauri.conf.json
	sed -i '' 's/^version = ".*"/version = "$(SEMVER)"/' src-tauri/Cargo.toml
	sed -i '' 's/"version": ".*"/"version": "$(SEMVER)"/' package.json
	$(MAKE) _build
	git add src-tauri/tauri.conf.json src-tauri/Cargo.toml package.json src-tauri/Cargo.lock
	git commit -m "chore: bump version to $(SEMVER)"
	git tag v$(SEMVER)
	git push origin main v$(SEMVER)
	gh release create v$(SEMVER) \
		--title "v$(SEMVER)" \
		--generate-notes
