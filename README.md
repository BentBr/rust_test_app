# Rust Test Web App
Playground to get started with rust.

## Useful commands
- `cargo run` to have cargo built the app
  - `cargo run --release`
- `cargo fmt` to format all files
- `cargo clippy` to have a phpstan analogous function of linting
- `cargo audit` to check for security issues of packages
- `cargo nextest run` to test via better test framework than default one of cargo
- node must be used from within the container: `docker-compose exec node sh` -> `npm run dev`
- Or use the docker-compose setup via dhingy router: `docker-compose exec node npm run dev` \
It's setup to serve on `rusty.docker` (On default port 80). See _vite.config.js_

## Todos and ideas
- proper feedback during deletion
- adding description and possibility to change it
- Check for benchmarking
- Check for git cliff + Semver +  Conventional Commits
- Add codecov.io (makers of sentry)