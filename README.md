# Rust Test Web App
Playground to get started with rust.

## Useful commands
- `cargo run` to have cargo built the app
  - `cargo run --release`
- `cargo fmt` to format all files
- `cargo clippy` to have a phpstan analogous function of linting
- `cargo audit` to check for security issues of packages
- `cargo nextest run` to test via better test framework than default one of cargo

### Git cliff
Using conventional commits we want to make sure having a proper CHANGELOG.md file being created and maintained. \
See `cliff.toml` for configuration and keys
- `git cliff -o CHANGELOG.md` generates latest changelog file based on conventional commits
- `git cliff --bump` to calculate next semantic version (based on Semver)

## Todos and ideas
- Check for benchmarking