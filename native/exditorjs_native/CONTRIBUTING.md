# Contributing to EditorJS Converter

We'd love to have your contributions! Here's how you can help:

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/test-editorjs.git`
3. Create a feature branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Push to your fork and submit a pull request

## Development Setup

### Requirements
- Rust 1.70+ (with Cargo)

### Building
```bash
cargo build
```

### Running Tests
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_html_paragraph
```

### Running Examples
```bash
cargo run --example basic
```

### Code Style
- Follow standard Rust conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Add tests for new features

## Types of Contributions

### Bug Reports
- Use the issue tracker
- Include a minimal example that reproduces the issue
- Specify your Rust version and OS

### Feature Requests
- Describe the use case
- Provide examples of desired behavior
- Discuss implementation approach if possible

### Code Contributions
- New block type support
- Improved HTML/Markdown parsing
- Performance optimizations
- Better error messages
- Documentation improvements
- Test coverage

## Pull Request Process

1. Ensure all tests pass: `cargo test`
2. Update documentation if needed
3. Add tests for new functionality
4. Use clear, descriptive commit messages
5. Reference any related issues

## Code Guidelines

### Naming Conventions
- Use snake_case for functions and variables
- Use PascalCase for types and structs
- Use UPPER_SNAKE_CASE for constants

### Documentation
- Document public APIs with doc comments
- Include examples in doc comments where helpful
- Explain complex algorithms

### Error Handling
- Use the `Result<T>` type for fallible operations
- Provide clear error messages
- Use the `Error` enum for custom errors

## Areas for Contribution

1. **Improved HTML Parsing**: Better support for complex HTML structures
2. **Inline Formatting**: Bold, italic, underline, links in content
3. **Nested Lists**: Support for nested list items
4. **Custom Blocks**: More flexible block type handling
5. **Performance**: Optimize parsing algorithms
6. **Documentation**: Expand examples and guides
7. **Testing**: Increase test coverage
8. **Tooling**: CLI utilities for batch conversion

## Questions?

Feel free to open an issue with the `question` label or start a discussion.

## Code of Conduct

Be respectful and constructive. We're all here to help make this project better!

Thank you for contributing! 🎉
