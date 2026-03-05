set shell := ["bash", "-cu"]

# Start a release branch, bump version, and push the branch
release-start VERSION:
	git checkout main
	git pull --ff-only || true
	git checkout -b release/v{{VERSION}}
	perl -0pi -e 's/^version = "[^"]+"/version = "{{VERSION}}"/m' Cargo.toml
	git add Cargo.toml
	git commit -m "release: v{{VERSION}}"
	git push -u origin release/v{{VERSION}}

# Create and push a tag after the release branch is merged to main
release-tag VERSION:
	git checkout main
	git pull --ff-only || true
	grep -q "^version = \"{{VERSION}}\"" Cargo.toml
	git tag v{{VERSION}}
	git push origin v{{VERSION}}
