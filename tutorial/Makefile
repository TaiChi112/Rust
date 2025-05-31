# Variables
CARGO = cargo
TARGET = debug

# Default target
all: build

# Build the project
build:
	$(CARGO) build

# Run the project
run:
	$(CARGO) run

# Test the project
test:
	$(CARGO) test

# Clean the project
clean:
	$(CARGO) clean

# Format the code
format:
	$(CARGO) fmt

# Lint the code
lint:
	$(CARGO) clippy -- -D warnings

# Build the project in release mode
release:
	$(CARGO) build --release

# Custom target for building with specific features
build-with-feature:
	$(CARGO) build --features "feature_name"

# Help message
help:
	@echo "Usage: make [target]"
	@echo "Targets:"
	@echo "  all        - Default target (build)"
	@echo "  build      - Build the project"
	@echo "  run        - Run the project"
	@echo "  test       - Test the project"
	@echo "  clean      - Clean the build artifacts"
	@echo "  format     - Format the code"
	@echo "  lint       - Lint the code using clippy"
	@echo "  release    - Build the project in release mode"
	@echo "  build-with-feature - Build with specific features (replace feature_name)"
