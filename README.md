# Rust

### Useful links

[Learn Rust | official](https://www.rust-lang.org/learn)

[*The Book*](https://doc.rust-lang.org/book/title-page.html)

[Rust Std Library](https://doc.rust-lang.org/std/index.html)

[Rust Reference](https://doc.rust-lang.org/reference/notation.html)

[Rust Documentation](https://doc.rust-lang.org/stable/)

[Rust by example](https://doc.rust-lang.org/stable/rust-by-example/index.html)

##### wasm

[wasm-bingen lib/cli | official](https://rustwasm.github.io/docs/wasm-bindgen/#introduction)

[Rust and WebAssembly | official](https://rustwasm.github.io/docs/book/introduction.html)

[wasm-pack doc | official](https://rustwasm.github.io/wasm-pack/book/introduction.html)

##### tools

[cargo-generate | GitHub](https://github.com/ashleygwilliams/cargo-generate)

##### tuto

[Rust for Javascripters | Coding Tech Youtube](https://youtu.be/ohuTy8MmbLc)

[learn Rust 30 minutes a day](https://fasterthanli.me/articles/a-half-hour-to-learn-rust) - good entry or refresh point

##### server in Rust

[auth web microservice in rust](https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/)

[How to Build a REST API in Rust | Medium](https://medium.com/better-programming/rest-api-in-rust-step-by-step-guide-b8a6c5fcbff0)

[web and api auth server in Rust | Medium](https://medium.com/swlh/creating-a-web-and-api-authentication-service-in-rust-55d0b0a848d) (1)

[sending mails with Rust](https://medium.com/swlh/sending-emails-with-rust-162667ee40f6) (2)

[User auth in Rust](https://medium.com/swlh/user-authentication-in-rust-ee8116934d73) (3)

##### wasm

[debugging Rust-generated WASM | official](https://rustwasm.github.io/docs/wasm-bindgen/#introduction)

## TODO

##### the book

* [ ] [1.3 Hello, Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) (official doc)

##### rust by example

* [ ] [1.2.2.1 Testcase liste | official](https://doc.rust-lang.org/stable/rust-by-example/hello/print/print_display/testcase_list.html)

##### rust 30 min a day

* [ ] An `impl` block is always *for* a type, so, inside that block

##### rust & wasm

project : `./wasm/wasm-game-of-life/`

* [X] [4.4 Game of Live WASM](https://rustwasm.github.io/docs/book/game-of-life/implementing.html#rust-implementation)
* [X] [4.5 GoL Testing Life](https://rustwasm.github.io/docs/book/game-of-life/testing.html)
* [ ] [4.8 GoL Time Profiling](https://rustwasm.github.io/docs/book/game-of-life/time-profiling.html) stopped there. Good place to go for harsh optimisation. Not a priority.

## commands

`rustup update` : update rust

##### cargo

`cargo new --vcs=none [NAME]` : creates a new cargo project and directory, the `--vcs=none` flag disable the creation of a git repo.

`cargo check` : check that the project compile.

`cargo build` : compile. The executable is located at `./target/debug/executable` , it is possibel to run directly from it.

`cargo run` : compile and run

`cargo build --release` : build for release, removing code for error logging.

##### wasm-pack

`wasm-pack build` : compile the rust module to wasm.

`wasm-pack test --chrome --headless` : run tests. You can also use the `--firefox`, `--safari`, and `--node` options to test your code in those browsers.

`wasm-pack build --debug` : build package for debug.

### General info

```shell
 ❮ onyr ★  kenzae❯ ❮ rust❯❯ rustc --version
rustc 1.48.0 (7eac88abb 2020-11-16)
```

## Installation

Installing `rust` and `rustup`, the package and language manager is trivial. Follow instructions [here](https://www.rust-lang.org/tools/install). It's basically a one liner that does all the job.

## VSCode

Use a `rust-toolchain` file to specify `stable` or `nightly` rust toolchain to be used by VSCode and Rust. More info [here | StackOverflow](https://stackoverflow.com/questions/58226545/how-to-switch-between-rust-toolchains).

You can also set VSCode user settings for using stable, [read this | GitHub issue](https://github.com/rust-lang/vscode-rust/issues/237#issuecomment-359639894).
