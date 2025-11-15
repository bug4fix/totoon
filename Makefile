.PHONY: install test lint format clean build

install:
	pip install -e ".[dev]"

test:
	pytest python/tests/ -v

lint:
	ruff check python python/tests python/examples

format:
	black python python/tests python/examples

clean:
	rm -rf build/
	rm -rf dist/
	rm -rf *.egg-info
	find . -type d -name __pycache__ -exec rm -r {} +
	find . -type f -name "*.pyc" -delete

build:
	python -m build

all: format lint test

