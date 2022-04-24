# Setup

## Setup environment

1. Login into postgresql:
```sh
psql -U postgres
```
2. Setup database: [Instructions](SETUP_DATABASE)
3. Setup cleaner: [Instructions](SETUP_CLEANER)
4. Logout:
```sh
\q
```
5. Copy `default.env` to `.env`:
```sh
cp default.env .env
```
6. Open with `nano`:
```sh
nano .env
```
7. Add `DATABASE_URL=<url>`:
```sh
DATABASE_URL=postgres://account:<password>@localhost:<port>/account
```
8. Configure `.env`
9. Save:
```
Ctrl+O
```
10.  Close nano:
```sh
Ctrl+X
```
11. Install cargo:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
12. Install diesel cli:
```sh
cargo install diesel_cli:
```
13. Run migrations:
```sh
diesel migration run
```

14. Configure `frontend/src/lib/config.ts`

<br>

## Additional Steps

### Development

1. Install cargo watch:
```sh
cargo install cargo-watch
```

2. Run development backend:
```sh
chmod +x dev.sh
./dev.sh
```

3. Run development frontend:
```sh
cd frontend
npm install
npm run dev
```

<br>

### Production

1. Build production backend:
```sh
cargo build --release
```

2. Move executable to root directory:
```sh
cp target/release/kekaccount .
```

3. Run production backend:
```sh
./kekaccount
```

4. Build production frontend:
```sh
cd frontend
npm install
npm run build
```

5. Upload `frontend/build` to your webserver