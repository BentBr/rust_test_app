# Rust Test Web App
Playground to get started with rust.

## Useful commands
- `cargo run` to have cargo built the app
  - `cargo run --release`
- `cargo fmt` to format all files
- `cargo clippy` to have a phpstan analogous function of linting
- `cargo audit` to check for security issues of packages
- `cargo nextest run` to test via better test framework than default one of cargo
- node must be used from within the container: `docker-compose exec node sh` -> `npm run dev` \
_You don't have to run it manually as the docker container automatically runs the dev server!_
- Or use the docker-compose setup via dinghy router: `docker-compose exec node npm run dev` \
It's setup to serve on `rusty.docker` (On default port 80). See _vite.config.js_
- `diesel migration generate create_to_do_items` Using diesel to create a new migration
- Running the migration: `diesel migration run`
- Undoing and running again the migration: `diesel migration redo`

## Todos and ideas
- proper feedback during deletion
- adding description and possibility to change it
- Check for benchmarking
- Check for git cliff + Semver +  Conventional Commits
- Add codecov.io (makers of sentry)

## Initial Setup
Postgres stuff:
- `brew install libpq postgresql@15` + `brew link postgresql@1`
- `cargo install diesel_cli --no-default-features --features postgres`

Testing stuff:
- `cargo install grcov` Install the coverage report generator
- `rustup component add llvm-tools-preview` component to check usages during test
- `rustup install nightly && rustup default nightly` nightly build for instrument-coverage usage
