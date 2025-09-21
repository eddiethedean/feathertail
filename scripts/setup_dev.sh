#!/bin/bash

# Feathertail Development Environment Setup Script
# This script sets up a complete development environment for feathertail

set -e

echo "🚀 Setting up Feathertail development environment..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -f "pyproject.toml" ]; then
    echo "❌ Error: Please run this script from the feathertail root directory"
    exit 1
fi

# Check for required tools
echo "🔍 Checking for required tools..."

# Check Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check Python
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 not found. Please install Python 3.7+"
    exit 1
fi

# Check pip
if ! command -v pip &> /dev/null; then
    echo "❌ pip not found. Please install pip"
    exit 1
fi

echo "✅ All required tools found"

# Install Rust dependencies
echo "📦 Installing Rust dependencies..."
cargo build

# Install Python dependencies
echo "📦 Installing Python dependencies..."
pip install -r requirements.txt
pip install -r docs/requirements.txt

# Install development dependencies
echo "📦 Installing development dependencies..."
pip install black flake8 mypy pytest pytest-cov pre-commit

# Install pre-commit hooks
echo "🔧 Setting up pre-commit hooks..."
pre-commit install

# Build the project
echo "🔨 Building the project..."
maturin develop

# Run tests to verify setup
echo "🧪 Running tests to verify setup..."
python -m pytest tests/python/unit/ -v --tb=short

echo "✅ Development environment setup complete!"
echo ""
echo "Next steps:"
echo "1. Run 'make test' to run all tests"
echo "2. Run 'make docs' to build documentation"
echo "3. Run 'make benchmark' to run performance tests"
echo "4. Check out CONTRIBUTING.md for contribution guidelines"
echo ""
echo "Happy coding! 🎉"
