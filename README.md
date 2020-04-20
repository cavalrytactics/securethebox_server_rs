# securethebox_server_rs
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
```
cargo-watch
cargo-outdated
```

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