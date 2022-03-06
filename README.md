# KekAccount
KekHost is an application written in [rust](https://www.rust-lang.org/) for hosting files. 

<br>

[Documentation](https://oss.kotw.dev/kekaccount/docs/)

<br>

## License
This project is licensed under the [Mit License](https://mit-license.org/)

<hr>
<br>

## Usage

### Prerequirements

- Rust Nightly <br>
You need [rustup](https://rustup.rs/) to run this.

```sh
rustup default nightly
```

<br>

### Configuration
Copy default.env to .env and change the settings in .env.

<br>

### Building
```sh
cargo build --release
```

The executable will be located at `target/release/kekaccount`

<br>

### Testing
If you are developing and don't want to rebuild and run the client to release mode use
```sh
cargo run
```

<hr>
<br>

## Goals

- TCP api
- WS api
- REST api

<br>

If you have aditional ideas how to make this tool better please create a feature request in the issues tab.

<hr>
<br>

## Contributing
More information [here](https://oss.kotw.dev/kekaccount/CONTRIBUTE).
