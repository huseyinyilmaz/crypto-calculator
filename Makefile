# https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
.PHONY: help
help: ## This outputs help information.
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

test: ## run tests and wait watch for changes.
	cargo watch -x test

lint: ## run clippy
	cargo clippy


fmt-check: ## run format check
	cargo fmt --all -- --check

build:
	cargo build

release:
	cargo build --release

install: release
	cargo install --path .
