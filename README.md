# securethebox_server_rs
## Overview
- web framework: [actix-web](https://github.com/actix/actix-web)
- graphql service: [async-graphql](https://github.com/async-graphql/async-graphql)
- database service: [mongodb-rust-driver](https://github.com/mongodb/mongo-rust-driver)
- frontend: [securethebox_client_rs](https://github.com/cavalrytactics/securethebox_client_rs)

## Requirements for local dev
- [travis-ci cli: latest](https://github.com/travis-ci/travis.rb#mac-os-x-via-homebrew)
- [rust & rustup](https://www.rust-lang.org/tools/install)
- [rustsym](https://github.com/trixnz/rustsym)

## Updating Rust
```
rustup update
```
## Rust Nightly | Stable
```
rustup set profile 
rustup default nightly
rustup default stable
```
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
- [ff](https://github.com/vishaltelangre/ff))
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

## TODO
- [x] P0 - Dev: Standardize Tests
- [x] P0 - Dev: Encrypted/Decrypted Secrets
- [x] P0 - Dev: Travis Test/Deploy
- [x] P0 - Feat: Travis Controller
- [x] P0 - Feat: Terraform Controller
- [x] P0 - Feat: Terraform apply/destroy Kubernetes Cluster
- [x] P0 - Dev: Seed Database
- [ ] P0 - Feat: GraphQL Schema 
- [x] P0 - Feat: GraphQL API (async-graphql)
- [ ] P0 - Feat: Authentication GraphQL API 
- [x] P0 - Feat: Websocket/Subscriptions
- [x] P0 - Feat: Web Server Actix-Web
- [x] P0 - Dev: Dockerfile
- [x] P0 - Dev: Docker-Compose
- [ ] P0 - Feat: Challenge Creation
- [ ] P0 - Feat: Challenge Start
- [ ] P0 - Feat: Challenge Report
- [ ] P0 - Feat: Challenge End 
- [ ] P0 - Feat: Proxy user environment
- [ ] P0 - Feat: Load Balancer
- [ ] P0 - Feat: SSH Access to each POD 
- [ ] P0 - Feat: Configuration Management of Containers (ConfigMaps)
- [ ] P0 - Feat: Challenge Vulnerable App (juice-shop)
- [ ] P0 - Feat: Challenge Logging
- [ ] P0 - Feat: Challenge SIEM
- [ ] P0 - Feat: Challenge 
- [ ] P2 - Feat: RBAC - Casbin-rs 