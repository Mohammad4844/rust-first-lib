SHELL := /bin/bash
.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort

clean: ## Clean the project using cargo
	cargo clean

build: ## Build the project using cargo
	cargo build

lint: ## Run clippy on the project
	cargo clippy

format: ## Format the project using rustfmt
	@rustup component add rustfmt 2> /dev/null
	cargo fmt

test: ## Run the tests
	cargo test
