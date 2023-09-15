# Simple JVM implementation using Rust

For details related to VSCode configuration check [VSCode Rust integration](https://code.visualstudio.com/docs/languages/rust)

As a linter use [Clippy](https://github.com/rust-lang/rust-clippy)

## Run JVM locally

* You should build first using:

```bash
cargo build
```

* To execute locally use:

```bash
target/debug/rust-vm -cp java com.max.Hello
```

Or you can just run the script which will do the same steps:

```bash
./run.sh
```
