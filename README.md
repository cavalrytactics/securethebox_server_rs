# securethebox_server_rs
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
alias cwc="cargo-watch -c -i test.rs -i test.yml -i .travis.yml -i target/* -i secrets.tar.gz -i secrets.tar.gz.enc -i secrets/* -x check"
alias cwt="cargo-watch -c -i test.rs -i test.yml -i .travis.yml -i target/* -i secrets.tar.gz -i secrets.tar.gz.enc -i secrets/* -x 'test -- --nocapture'"
alias cwr="cargo-watch -c -i test.rs -i test.yml -i .travis.yml -i target/* -i secrets.tar.gz -i secrets.tar.gz.enc -i secrets/* -x run"
```

- Recommended Terminal Split
```
term1:
cwc

term2:
cwt

mainTerm:
cwr
```