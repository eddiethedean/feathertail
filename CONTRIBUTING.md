# Contributing to Feathertail

Thank you for your interest in contributing to feathertail! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Contributing Process](#contributing-process)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Release Process](#release-process)

## Code of Conduct

This project follows a code of conduct that we expect all contributors to follow. Please be respectful and constructive in all interactions.

## Getting Started

### Prerequisites

- Rust 1.70+ (for the core library)
- Python 3.7+ (for Python bindings)
- Git
- Make (for build automation)

### Development Setup

1. **Fork the repository** on GitHub
2. **Clone your fork**:
   ```bash
   git clone https://github.com/yourusername/feathertail.git
   cd feathertail
   ```

3. **Set up the development environment**:
   ```bash
   make setup-dev
   ```

4. **Install dependencies**:
   ```bash
   pip install -r requirements.txt
   pip install -r docs/requirements.txt
   ```

5. **Build the project**:
   ```bash
   make build
   ```

6. **Run tests**:
   ```bash
   make test
   ```

## Contributing Process

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes

- Write your code following the coding standards
- Add tests for new functionality
- Update documentation as needed
- Ensure all tests pass

### 3. Commit Your Changes

Use conventional commit messages:

```bash
git commit -m "feat: add new string operation method"
git commit -m "fix: resolve memory leak in groupby operation"
git commit -m "test: add comprehensive tests for join operations"
git commit -m "docs: update API documentation for new methods"
```

### 4. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a pull request on GitHub.

## Coding Standards

### Rust Code

- Follow Rust naming conventions (snake_case for functions, PascalCase for types)
- Use `cargo fmt` to format code
- Use `cargo clippy` to check for linting issues
- Add comprehensive documentation comments
- Use meaningful variable and function names

### Python Code

- Follow PEP 8 style guidelines
- Use type hints where appropriate
- Add docstrings for all public functions
- Use meaningful variable and function names

### Documentation

- Use clear, concise language
- Provide examples for complex operations
- Update both code comments and user documentation
- Follow the existing documentation style

## Testing

### Running Tests

```bash
# Run all tests
make test

# Run Rust tests only
cargo test

# Run Python tests only
make test-python

# Run specific test file
python -m pytest tests/python/unit/test_joins.py -v
```

### Writing Tests

- Write tests for all new functionality
- Include edge cases and error conditions
- Use descriptive test names
- Aim for high test coverage
- Follow the existing test patterns

### Test Structure

```
tests/
├── python/
│   ├── unit/           # Unit tests for individual functions
│   ├── integration/    # Integration tests
│   └── benchmarks/     # Performance benchmarks
└── rust/               # Rust-specific tests
```

## Documentation

### API Documentation

- All public functions must have docstrings
- Include parameter descriptions and return value information
- Provide usage examples
- Update the Sphinx documentation when adding new features

### User Documentation

- Update the getting started guide for new features
- Add tutorials for complex functionality
- Keep the advanced usage guide current
- Update the API reference

### Building Documentation

```bash
# Build documentation
make docs

# Serve documentation locally
make docs-serve
```

## Release Process

### Version Numbering

We follow semantic versioning (MAJOR.MINOR.PATCH):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

- [ ] All tests pass
- [ ] Documentation is updated
- [ ] Version number is updated
- [ ] CHANGELOG.md is updated
- [ ] Release notes are written
- [ ] Tag is created
- [ ] Package is published to PyPI

## Development Tools

### Pre-commit Hooks

We use pre-commit hooks to ensure code quality:

```bash
# Install pre-commit hooks
pre-commit install

# Run hooks manually
pre-commit run --all-files
```

### Code Quality Tools

- **Rust**: `cargo fmt`, `cargo clippy`, `cargo audit`
- **Python**: `black`, `flake8`, `mypy`, `pytest`
- **Documentation**: `sphinx-build`, `myst-parser`

### Performance Testing

```bash
# Run benchmarks
make benchmark

# Profile performance
make profile
```

## Getting Help

- **Issues**: Use GitHub issues for bug reports and feature requests
- **Discussions**: Use GitHub discussions for questions and general discussion
- **Documentation**: Check the documentation for detailed information
- **Code Review**: All changes require code review before merging

## Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project documentation

Thank you for contributing to feathertail! 🚀
