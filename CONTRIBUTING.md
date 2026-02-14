# Contributing to BBEAN Engine

Thank you for your interest in contributing to BBEAN Engine. This document provides guidelines for contributing to the project.

## Development Setup

1. Clone the repository:
```bash
git clone https://github.com/BBEAN-gm/bbean-engine.git
cd bbean-engine
```

2. Run the setup script:
```bash
chmod +x scripts/setup.sh
./scripts/setup.sh
```

3. Verify everything works:
```bash
cargo test --workspace
```

## Code Style

- Follow standard Rust formatting (`cargo fmt`)
- Run clippy before submitting: `cargo clippy --workspace`
- Write tests for new functionality
- Keep functions focused and small
- Use meaningful variable and function names

## Commit Messages

We use conventional commits:

- `feat:` New feature
- `fix:` Bug fix
- `refactor:` Code refactoring
- `docs:` Documentation changes
- `test:` Adding or updating tests
- `chore:` Maintenance tasks
- `perf:` Performance improvements

## Pull Request Process

1. Fork the repository and create a feature branch
2. Make your changes with appropriate tests
3. Ensure all tests pass: `cargo test --workspace`
4. Ensure code is formatted: `cargo fmt --check`
5. Ensure clippy is clean: `cargo clippy --workspace`
6. Submit a pull request with a clear description

## Testing

- Unit tests live alongside the code they test
- Integration tests are in the `tests/` directory
- Run all tests: `cargo test --workspace`
- Run specific crate tests: `cargo test -p bbean-core`

## Architecture

See [docs/architecture.md](docs/architecture.md) for an overview of the system design.

## Reporting Issues

- Use the issue templates provided
- Include reproduction steps for bugs
- Provide system information (OS, Rust version, etc.)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
