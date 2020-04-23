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
- test default
```
cargo test -- --ignored
```
- test include ignored
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
alias cwc="cargo-watch -i .travis-openssl-keys -i secrets/travis-openssl-keys-values.txt -i test.rs -i test.yml -i .travis.yml -i target/* -i secrets.tar.gz -i secrets.tar.gz.enc -i -x check"
alias cwt="cargo-watch -i .travis-openssl-keys -i secrets/travis-openssl-keys-values.txt -i test.rs -i test.yml -i .travis.yml -i target/* -i secrets.tar.gz -i secrets.tar.gz.enc -i -x 'test -- --nocapt
ure'"
alias cwr="cargo-watch  -i .travis-openssl-keys -i secrets/travis-openssl-keys-values.txt -i test.rs -i test.yml -i .travis.yml -i target/* -i secrets.tar.gz -i secrets.tar.gz.enc -i -x run"
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