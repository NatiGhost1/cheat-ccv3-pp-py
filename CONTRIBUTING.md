# Contributing

Thank you for your interest in contributing to `cheat-ccv3-pp-py`! This document provides guidelines and instructions for contributing.

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on the code, not the person
- Help others succeed

## Getting Started

### Prerequisites

- Rust 1.70+ ([Install](https://rustup.rs/))
- Python 3.11+ ([Install](https://www.python.org/downloads/))
- Maturin ([Install](https://www.maturin.rs/installation))

### Development Setup

```bash
# Clone the repository
git clone https://github.com/NatiGhost1/cheat-ccv3-pp-py.git
cd cheat-ccv3-pp-py

# Create a Python virtual environment
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install development dependencies
pip install maturin

# Build in development mode
maturin develop
```

### Running Tests

```bash
# Build and test
maturin develop
python3 -m pytest tests/
```

## Development Workflow

1. **Fork the repository** on GitHub
2. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes** following the style guide
4. **Document your changes** with comments and docstrings
5. **Test thoroughly** (see Testing section)
6. **Commit with clear messages**:
   ```bash
   git commit -m "Add feature: description of what was added"
   ```
7. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```
8. **Open a Pull Request** with a clear description

## Style Guide

### Rust Code

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting:
  ```bash
  cargo fmt
  ```
- Use `cargo clippy` for linting:
  ```bash
  cargo clippy -- -D warnings
  ```

### Documentation

- Write clear docstrings for all public items
- Use examples in docstrings when appropriate
- Keep README and documentation updated
- Document breaking changes in CHANGELOG.md

### Python API

- Maintain Pythonic naming conventions
- Use type hints in docstrings
- Follow [PEP 8](https://pep8.org/) style guide
- Test with multiple Python versions (3.11+)

## Testing

### Adding Tests

Create tests in `tests/` directory:

```python
from cheat_ccv3_pp_py import Beatmap, Difficulty, Performance

def test_basic_calculation():
    beatmap = Beatmap(path="test_maps/test.osu")
    diff = Difficulty(beatmap, mods=0)
    result = diff.calculate()
    assert result.stars > 0
```

### Running Tests

```bash
# Run all tests
maturin develop
python3 -m pytest tests/ -v

# Run specific test file
python3 -m pytest tests/test_beatmap.py

# Run with coverage
pip install pytest-cov
pytest --cov=cheat_ccv3_pp_py tests/
```

## Building Release Binaries

```bash
# Release build
maturin build --release

# Build for distribution
maturin sdist
maturin bdist pyo3
```

## Common Tasks

### Updating Dependencies

```bash
# Update Cargo dependencies
cargo update

# Update Python dependencies
pip install --upgrade pip setuptools wheel
```

### Debugging

```bash
# Build with debug symbols
maturin develop

# Use print debugging
println!("Debug value: {:?}", variable);

# Use Python debugger
import pdb; pdb.set_trace()
```

## Pull Request Process

1. **Ensure all tests pass**
2. **Update documentation** if needed
3. **Add entry to CHANGELOG.md**
4. **Request review** from maintainers
5. **Address feedback** promptly
6. **Squash commits** if requested

## Reporting Issues

### Bug Reports

Include:
- Python version
- Operating system
- Minimal reproduction code
- Expected vs actual behavior
- Traceback/error message

### Feature Requests

Include:
- Use case description
- Proposed API
- Examples if applicable
- Alternative approaches considered

## Questions?

- Open an issue for questions
- Check existing issues first
- Join discussions in pull requests

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Happy Contributing!** 🎉
