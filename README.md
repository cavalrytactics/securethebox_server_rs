# securethebox_server_rs
## Overview
- frontend framework: [yew](https://github.com/yewstack/yew)
- web framework: [actix-web](https://github.com/actix/actix-web)
- graphql service: [juniper](https://github.com/graphql-rust/juniper)
- database service: [mongodb-rust-driver](https://github.com/mongodb/mongo-rust-driver)

## Requirements for local dev
- [travis-ci cli: latest](https://github.com/travis-ci/travis.rb#mac-os-x-via-homebrew)
- [docker desktop for mac](https://hub.docker.com/editions/community/docker-ce-desktop-mac/)
- [terraform cli: v12.24](https://www.terraform.io/downloads.html)
- [gcloud sdk cli](https://cloud.google.com/sdk/docs/downloads-interactive)
- [rust](https://www.rust-lang.org/tools/install)

## Building
```
cargo build
```
## Database + Seeding
```
docker-compose up
cargo run --bin seed
```
## Database purging
```
docker-compose down -v
```
## Running
```
cargo run
```
## Testing
- test default (execution=1-thread) all tests should succeed
```
cargo test -- --test-threads=1
```
- test (excludes ignored tests), (execution=parallel) some tests may fail
```
cargo test
```
- test include ignored (excludes high cpu/memory tests)
```
cargo test -- --ignored
```
## CI/CD
- [securethebox_server_rs](https://travis-ci.org/github/cavalrytactics/securethebox_server_rs)
- src/controllers/travis.rs
- travis_template.yml <-- Edit
- travis.yml <-- Do not edit

## Recommended Cargo Tools
- [cargo-watch](https://github.com/passcod/cargo-watch)
- [cargo-outdated](https://github.com/kbknapp/cargo-outdated)
- DO NOT USE rust-clippy its very buggy

## Recommended Aliases
```
alias cb="cargo build"
alias cbr="cargo build --release"
alias cr="cargo run"
alias cc="cargo check"
alias ct="cargo test"
alias cwc="cargo-watch -w src -x check"
alias cwt="cargo-watch -w src -x 'test -- --nocapture --test-threads=1'"
alias cwr="cargo-watch -w src -x run"
```

## Recommended Terminal Split
```
mainTerm:
cwr

term1:
cwc

term2:
cwt

term3:
docker-compose up
```