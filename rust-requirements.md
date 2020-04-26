# Planning
- Docker Image
  [RUST]
    - Rust
      - https://hub.docker.com/_/rust
    - Graphql
      - https://github.com/jayy-lmao/rust-graphql-docker
    - Web
      - https://github.com/seanmonstar/warp
    - Authentication
      - Google Outh2

- HTTP service
  [RUST]

- Testing Library
  [RUST]
    - native
      - https://doc.rust-lang.org/1.18.0/book/first-edition/testing.html

- Types
  [RUST]
    - native

- Travis support
  [RUST]
    - https://docs.travis-ci.com/user/languages/rust/

- GraphQL support
  [RUST]
    - https://github.com/graphql-rust/juniper
      - supports actix-web
  - Needs to support Subscriptions <<< Unstable
    - https://github.com/graphql-rust/juniper/blob/8bcd1e5bfea8ee73082ddc8220f4a5fb989f3f51/docs/book/content/advanced/subscriptions.md
  - Supports MongoDB client and change steams
    - https://github.com/mongodb/mongo-rust-driver
    - https://github.com/mongodb/mongo-rust-driver/pull/30
- System Commands
  [RUST]
    - Kubernetes
      - https://github.com/clux/kube-rs
- Authentication
  [RUST]

- Google Cloud Platform
  [RUST]
    - Cloud Run
    - Cloud DNS
    - Cloud Tasks

# TODO
- Seeding Database
- Functional Programming
- Stateless Functions
- Microservices
  - HTTP
  - GraphQL
  - Websockets
- Typed Functions
- Test cases
- Version Control / Releases
- Documentation - Data Flow - Action Diagrams

# Layer of Rust
1. mio = i/o to os
2. futures & async-await = async/promises
3. tokio = timers/scheduling
4. tower & tonic & hyper = http client/server