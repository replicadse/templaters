SHELL=/bin/bash

CONTAINER_NAME ?= replicadse/templaters
CONTAINER_TAG ?= latest
CONTAINER_FQ := $(CONTAINER_NAME):$(CONTAINER_TAG)

.PHONY: default
default:
	@echo make rules are:
	@echo + build: building the application
	@echo + override-version: replacing the version information for this application
	@echo --+ VERSION in format "x.y.z"
	@echo + fmt: formatting the code
	@echo + scan: scanning the code

.PHONY: build
build:
	@cargo build --release

.PHONY: override-version
override-version:
	@sed 's/version = "0.0.0"/version = "'$(VERSION)'"/g' Cargo.toml > Cargo.toml.tmp
	@mv Cargo.toml.tmp Cargo.toml

.PHONY: fmt
fmt:
	@cargo fmt --all

.PHONY: scan
scan:
	@cargo fmt --all -- --check

