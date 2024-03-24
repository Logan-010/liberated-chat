set dotenv-load

default:
  just -l

test:
  cargo test

clippy:
  cargo clippy

test-suite: test clippy

install-dependencies:
  cargo install --locked trunk

clear-users:
  cargo run --bin clear-users

clear-sessions:
  cargo run --bin clear-sessions

clear-messages:
  cargo run --bin clear-messages

clear-all: clear-users clear-sessions clear-messages

clean:
  cargo clean
  rm -f "$DATABASE_PATH"/"$DATABASE_NAME"
  rm -rf liberated-chat-frontend/dist

@build-frontend:
  cd liberated-chat-frontend && trunk build --release

build-frontend-debug:
  cd liberated-chat-frontend && trunk build

@build-backend:
  cd liberated-chat-server && cargo build --release

build-backend-debug:
  cd liberated-chat-server && cargo build

@build-all: build-frontend build-backend

build-all-debug: build-frontend-debug build-backend-debug

@run: build-all
  mkdir -p ./"$DATABASE_PATH"
  cargo run --bin liberated-chat-server --release

run-debug: build-all-debug
  mkdir -p ./"$DATABASE_PATH"
  cargo run --bin liberated-chat-server

@bundle: build-all
  mkdir -p ./bundle
  mkdir -p ./bundle/"$DATABASE_PATH"
  mkdir -p ./bundle/"$FRONTEND_PATH"
  mv ./target/release/liberated-chat-server* ./bundle
  rm ./bundle/liberated-chat-server.d
  cp -R ./liberated-chat-frontend/dist/* ./bundle/"$FRONTEND_PATH"

bundle-debug: build-all-debug
  mkdir -p ./bundle
  mkdir -p ./bundle/"$DATABASE_PATH"
  mkdir -p ./bundle/"$FRONTEND_PATH"
  mv ./target/debug/liberated-chat-server* ./bundle
  rm ./bundle/liberated-chat-server.d
  cp -R ./liberated-chat-frontend/dist/* ./bundle/"$FRONTEND_PATH"

