# KekAccount
![](https://tokei.rs/b1/github/KotwOSS/kekaccount)
![](https://tokei.rs/b1/github/KotwOSS/kekaccount?category=blanks)
![](https://tokei.rs/b1/github/KotwOSS/kekaccount?category=code)
![](https://tokei.rs/b1/github/KotwOSS/kekaccount?category=comments)
![](https://tokei.rs/b1/github/KotwOSS/kekaccount?category=files)
<br>

KekHost is an application written in [rust](https://www.rust-lang.org/) for hosting files. 

<br>

**Comming soon**
[Documentation](https://oss.kotw.dev/kekaccount/docs/)

<br>

## License
This project is licensed under the [Mit License](https://mit-license.org/)

<hr>
<br>

## Usage

### Web

#### Testing
```
cd frontend && npm run dev
```

#### Formating
```
npm run format
```

#### Building
```
npm run build
```

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
./dev.sh
```

<hr>
<br>

## Goals

- TCP api - Second level priority
- REST api - Finished
- Frontend - Doing

<br>

If you have aditional ideas how to make this tool better please create a feature request in the issues tab.

<hr>
<br>

## Contributing
More information [here](https://oss.kotw.dev/kekaccount/CONTRIBUTE).
