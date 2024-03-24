# Liberated chat
Liberated chat is a dead simple chatroom. That's that.

Features:
- Auth
- Persistance (sqlite3 database)
- Included dockerfile
- Simplicity

Built with:
- Rust
- Actix
- Leptos
- Rusqlite

# Building:
Be sure to have Rust installed and avalible.

Install Just (build manager):
```sh
cargo install just
```

Install dependencies with Just:
```sh
just install-dependencies
```

Make any changes you wish to .env

Bundle the entire project (this may take a while):
```sh
just bundle
```

Done! Bundle is located in ./bundle
Run the executable in there and the server will start.

# Docker:
I am rather new to docker, so there is no guarantee that the docker build will work.
Make any changes you wish to .env then run:
```sh
docker build -t liberated-chat .
```

Docker image should successfully build.
Run and supply the port from .env as an argument.