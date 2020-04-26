# securethebox_server_rs
## Overview
- frontend framework: [yew](https://github.com/yewstack/yew/)
- web framework: [warp](https://github.com/seanmonstar/warp)
- graphql service: [juniper](https://github.com/graphql-rust/juniper)
- database service: [mongodb atlas](https://github.com/mongodb/mongo-rust-driver)

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
## Running
```
cargo run
```
## Testing
- test default (execution=1-thread) all tests should succeed
```
cargo test -- --test-threads=1
```
- test (excluds ignored tests), (execution=parallel) some tests may fail
```
cargo test
```
- test include ignored (high cpu tests)
```
cargo test -- --ignored
```

## Recommended Cargo Tools
- [cargo-watch](https://github.com/passcod/cargo-watch)
- [cargo-outdated](https://github.com/kbknapp/cargo-outdated)

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
```