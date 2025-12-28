# tb-router Makefile

# Cross-compilation target for OpenWRT router (musl libc)
TARGET := aarch64-unknown-linux-musl
BINARY := tb-router
RELEASE_DIR := target/$(TARGET)/release

.PHONY: all build build-router clean install-deps strip size

# Default: build for host
all: build

# Build for development (host)
build:
	cargo build

# Build release for router (cross-compile)
build-router:
	cargo build --release --target $(TARGET)
	@echo "Binary: $(RELEASE_DIR)/$(BINARY)"
	@ls -lh $(RELEASE_DIR)/$(BINARY)

# Install cross-compilation dependencies (Arch Linux)
install-deps:
	rustup target add $(TARGET)
	sudo pacman -S --needed musl-aarch64 aarch64-linux-gnu-binutils

# Strip binary for smaller size
# Requires: sudo pacman -S aarch64-linux-gnu-binutils
strip: build-router
	aarch64-linux-gnu-strip $(RELEASE_DIR)/$(BINARY)
	@echo "Stripped binary:"
	@ls -lh $(RELEASE_DIR)/$(BINARY)

# Clean build artifacts
clean:
	cargo clean

# Show all compiled binaries
size:
	@echo "Host binary:"
	@ls -lh target/debug/$(BINARY) 2>/dev/null || echo "  Not found"
	@echo "Router binary (release):"
	@ls -lh $(RELEASE_DIR)/$(BINARY) 2>/dev/null || echo "  Not found"

# Deploy to router (set ROUTER_HOST env var)
# Uses -O for legacy SCP protocol (OpenWRT has no sftp-server)
REMOTE_DIR := /root/tb-router

deploy: build-router
	@if [ -z "$(ROUTER_HOST)" ]; then echo "Usage: make deploy ROUTER_HOST=root@192.168.1.1"; exit 1; fi
	ssh $(ROUTER_HOST) "mkdir -p $(REMOTE_DIR)"
	scp -O $(RELEASE_DIR)/$(BINARY) $(ROUTER_HOST):$(REMOTE_DIR)/
	scp -O config $(ROUTER_HOST):$(REMOTE_DIR)/
	@echo ""
	@echo "Deployed to $(ROUTER_HOST):$(REMOTE_DIR)"
	@echo "Run on router: cd $(REMOTE_DIR) && ./$(BINARY)"

