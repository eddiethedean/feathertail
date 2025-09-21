# Feathertail Makefile
# Provides convenient commands for development, testing, and building

.PHONY: help setup-dev build test test-python test-rust docs clean install lint format benchmark profile

# Default target
help:
	@echo "Feathertail Development Commands"
	@echo "================================"
	@echo ""
	@echo "Setup:"
	@echo "  setup-dev     Set up development environment"
	@echo "  install       Install the package in development mode"
	@echo ""
	@echo "Building:"
	@echo "  build         Build the Rust library and Python bindings"
	@echo "  clean         Clean build artifacts"
	@echo ""
	@echo "Testing:"
	@echo "  test          Run all tests (Rust + Python)"
	@echo "  test-python   Run Python tests only"
	@echo "  test-rust     Run Rust tests only"
	@echo "  test-coverage Run tests with coverage report"
	@echo ""
	@echo "Code Quality:"
	@echo "  lint          Run all linting tools"
	@echo "  format        Format all code"
	@echo "  check         Run all checks (lint + test)"
	@echo ""
	@echo "Documentation:"
	@echo "  docs          Build documentation"
	@echo "  docs-serve    Build and serve documentation locally"
	@echo ""
	@echo "Performance:"
	@echo "  benchmark     Run performance benchmarks"
	@echo "  profile       Run performance profiling"
	@echo ""
	@echo "Release:"
	@echo "  release       Build release version"
	@echo "  publish       Publish to PyPI"

# Setup development environment
setup-dev:
	@echo "🚀 Setting up development environment..."
	@./scripts/setup_dev.sh

# Install the package in development mode
install:
	@echo "📦 Installing feathertail in development mode..."
	@maturin develop

# Build the project
build:
	@echo "🔨 Building feathertail..."
	@maturin build

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean
	@rm -rf build/
	@rm -rf dist/
	@rm -rf *.egg-info/
	@rm -rf docs/_build/
	@find . -type d -name __pycache__ -exec rm -rf {} +
	@find . -type f -name "*.pyc" -delete

# Run all tests
test: test-rust test-python
	@echo "✅ All tests completed!"

# Run Python tests
test-python:
	@echo "🐍 Running Python tests..."
	@PYTHONPATH=. python -m pytest tests/python/ -v --tb=short

# Run Rust tests
test-rust:
	@echo "🦀 Running Rust tests..."
	@cargo test

# Run tests with coverage
test-coverage:
	@echo "📊 Running tests with coverage..."
	@PYTHONPATH=. python -m pytest tests/python/ --cov=feathertail --cov-report=html --cov-report=term

# Run all linting tools
lint:
	@echo "🔍 Running linting tools..."
	@echo "  - Python (flake8)..."
	@flake8 feathertail/ tests/python/
	@echo "  - Python (mypy)..."
	@mypy feathertail/
	@echo "  - Rust (clippy)..."
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "✅ Linting completed!"

# Format all code
format:
	@echo "🎨 Formatting code..."
	@echo "  - Python (black)..."
	@black feathertail/ tests/python/
	@echo "  - Rust (rustfmt)..."
	@cargo fmt
	@echo "✅ Formatting completed!"

# Run all checks (lint + test)
check: lint test
	@echo "✅ All checks passed!"

# Build documentation
docs:
	@echo "📚 Building documentation..."
	@cd docs && make html
	@echo "✅ Documentation built in docs/_build/html/"

# Build and serve documentation locally
docs-serve: docs
	@echo "🌐 Serving documentation at http://localhost:8000"
	@cd docs && make serve

# Run performance benchmarks
benchmark:
	@echo "⚡ Running performance benchmarks..."
	@cargo bench
	@PYTHONPATH=. python -m pytest tests/python/benchmarks/ -v

# Run performance profiling
profile:
	@echo "📊 Running performance profiling..."
	@cargo build --release
	@PYTHONPATH=. python -m pytest tests/python/benchmarks/ --profile

# Build release version
release:
	@echo "🚀 Building release version..."
	@cargo build --release
	@maturin build --release

# Publish to PyPI
publish:
	@echo "📦 Publishing to PyPI..."
	@maturin publish

# Run pre-commit hooks
pre-commit:
	@echo "🔧 Running pre-commit hooks..."
	@pre-commit run --all-files

# Install pre-commit hooks
install-hooks:
	@echo "🔧 Installing pre-commit hooks..."
	@pre-commit install

# Update dependencies
update-deps:
	@echo "🔄 Updating dependencies..."
	@cargo update
	@pip install --upgrade -r requirements.txt
	@pip install --upgrade -r docs/requirements.txt

# Security audit
audit:
	@echo "🔒 Running security audit..."
	@cargo audit
	@pip audit

# Generate API documentation
api-docs:
	@echo "📖 Generating API documentation..."
	@cd docs && sphinx-apidoc -o api/ ../feathertail/
	@cd docs && make html

# Run integration tests
test-integration:
	@echo "🔗 Running integration tests..."
	@PYTHONPATH=. python -m pytest tests/python/integration/ -v

# Run specific test file
test-file:
	@echo "🧪 Running specific test file..."
	@PYTHONPATH=. python -m pytest $(FILE) -v

# Run tests with specific pattern
test-pattern:
	@echo "🔍 Running tests matching pattern..."
	@PYTHONPATH=. python -m pytest tests/python/ -k $(PATTERN) -v

# Development server
dev-server:
	@echo "🌐 Starting development server..."
	@PYTHONPATH=. python -m http.server 8000

# Check code quality
quality:
	@echo "✨ Running code quality checks..."
	@make format
	@make lint
	@make test
	@echo "✅ Code quality checks completed!"

# Full development cycle
dev-cycle: clean install test docs
	@echo "🔄 Full development cycle completed!"

# Quick development check
quick-check: format lint test-python
	@echo "⚡ Quick development check completed!"

# Show project status
status:
	@echo "📊 Project Status"
	@echo "================"
	@echo "Rust version: $(shell cargo --version)"
	@echo "Python version: $(shell python3 --version)"
	@echo "Git branch: $(shell git branch --show-current)"
	@echo "Git status: $(shell git status --porcelain | wc -l) files changed"
	@echo "Test count: $(shell find tests/python/ -name "*.py" -exec grep -l "def test_" {} \; | wc -l) test files"
