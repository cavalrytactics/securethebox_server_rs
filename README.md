# securethebox_server_rs
## Overview
- 
- web framework: [warp](https://github.com/seanmonstar/warp)
  - supports juniper/graphql

## Requirements
- [travis-ci cli: latest](https://github.com/travis-ci/travis.rb#mac-os-x-via-homebrew)
- [docker desktop for mac](https://hub.docker.com/editions/community/docker-ce-desktop-mac/)
- [terraform cli: v12.24](https://www.terraform.io/downloads.html)
- [Rust](https://www.rust-lang.org/tools/install)

- Building
```
cargo build
```
- Running
```
cargo run
```
- Testing
```
cargo test
```

- Recommended Cargo Tools
- [cargo-watch](https://github.com/passcod/cargo-watch)
- [cargo-outdated](https://github.com/kbknapp/cargo-outdated)

- Recommended Aliases
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

- Recommended Terminal Split
```
mainTerm:
cwr

term1:
cwc

term2:
cwt
```