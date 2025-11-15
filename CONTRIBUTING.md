# Contributing to totoon

Thank you for your interest in contributing to totoon! We welcome contributions from the community.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/totoon.git`
3. Create a virtual environment: `python -m venv venv`
4. Activate it: `source venv/bin/activate` (or `venv\Scripts\activate` on Windows)
5. Install dependencies: `pip install -e ".[dev]"`

## Development Setup

```bash
# Install the package in development mode
pip install -e ".[dev]"

# Run tests
pytest

# Format code
black totoon tests

# Lint code
ruff check totoon tests
```

## Making Changes

1. Create a new branch: `git checkout -b feature/your-feature-name`
2. Make your changes
3. Add tests for new functionality
4. Ensure all tests pass: `pytest`
5. Format and lint your code: `black . && ruff check .`
6. Commit your changes: `git commit -m "Add feature: description"`
7. Push to your fork: `git push origin feature/your-feature-name`
8. Open a Pull Request

## Code Style

- Follow PEP 8 style guidelines
- Use type hints where appropriate
- Write docstrings for all public functions
- Keep functions focused and small
- Add tests for new features

## Testing

- Write tests for all new features
- Ensure test coverage doesn't decrease
- Run tests before submitting: `pytest --cov=totoon`

## Reporting Issues

When reporting issues, please include:
- Python version
- totoon version
- Steps to reproduce
- Expected vs actual behavior
- Any error messages

## Feature Requests

We welcome feature requests! Please open an issue describing:
- The problem you're trying to solve
- Your proposed solution
- Use cases

Thank you for contributing! 🚀

