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
- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust#build-system)
- [cargo-watch](https://github.com/passcod/cargo-watch)
- [cargo-outdated](https://github.com/kbknapp/cargo-outdated)
- [pier](https://github.com/pier-cli/pier)
  - pier config is already in pier.toml file
- DO NOT USE rust-clippy its very buggy

## Recommended Aliases
- pier
```
alias p="pier run"
alias pl="pier list"
alias pg="pier list | grep "
```

## Recommended Terminal Split
```
mainTerm (cargo run watch):
p wr

term1 (cargo test watch):
p wt

term2 (espanso watch):
p we

term3 (docker mongodb);
p du
```

## Rust Primatives
- [STD Libraries](https://github.com/brson/stdx)
- [DataTypes Cheatsheet](https://cheat.rs)
- [Rust for Pros](https://overexact.com/rust-for-professionals/)