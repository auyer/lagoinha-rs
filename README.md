# lagoinha-rs

> Rust project inspired by https://github.com/IgorHalfeld/lagoinha used to retrieve Addresses from the Brazilian Postal Code (CEP)

❌ - This project is in the first stages of development, and should not be used in production.

✔️ - Contributions and reviews are appreciated !

---

<p align="center">
  <!-- <img src="assets/logo.png" width="100px" /> -->
  <h3 align="center">
    Lagoinha-rs
  </h3>
  <p align="center">
    Rust library that returns addresses from the Brazilian Postal Code (CEP) <br/>
    using the following APIs: Correios, ViaCEP, Cepla
  </p>
  <p align="center">
    Readme in <a href="README-pt.md">Português</a>
  </p>
</p>

---

![CI](https://github.com/auyer/lagoinha-rs/workflows/CI/badge.svg)
[![crates.io](https://meritbadge.herokuapp.com/lagoinha)](https://crates.io/crates/lagoinha)
[![API docs](https://docs.rs/lagoinha/badge.svg)](https://docs.rs/lagoinha)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Lagoinha is a package that uses public APIs to fetch addresses using the Brazilian Postal Code (CEP). This package cuncurrenlty calls all the supported APIs and returns the first result.

### Diagram:

![lagoinha call fluxogram](.github/assets/lagoinha-calls.png)

### Why this name ?

It means "little pond". It is a Brazillian meme ! Check the video[vídeo](https://www.youtube.com/watch?v=C1Sd_RWF5ks)!

### Instalation

```toml
lagoinha-rs = "0.1"
```

### How to use it

```rust
extern crate lagoinha;
extern crate tokio;

#[tokio::main]
async fn main() {
    let addr = lagoinha::get_address("CEP_GOES_HERE").await;
    println!("{:#?}", addr);
}
```

### Run Examples

Check the [examples folder](examples/) !
To run them, use the commands below.

```bash
# these examples can be run with a specific CEP (or leave blank for default value)
cargo run --example get_address 20940040
cargo run --example standalone_services 20940040

```

---

## Todo

- [x] Get Started
- [x] Viacep service
- [x] Correios service
- [x] CepLá service
- [x] Separate Two languages in README.md
- [ ] Documentation
- [x] Invest in better error handling
- [ ] Unhappy path testing
- [ ] Validate input
- [ ] Different compilation features
- [ ] Abstractions: this will allow for mocking, and testing all paths without calls to the APIs
- [ ] Allow user to implement custom services, and opt out of any of the defaults

<!-- logo by [@nelsonsecco](https://twitter.com/nelsonsecco) -->
