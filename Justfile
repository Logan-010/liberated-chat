set dotenv-load

default:
  just -l

test:
  cargo test
  cargo clippy

install-dependencies:
  cargo install --locked trunk

clean:
  cargo clean
  rm -f "$DATABASE_PATH"/"$DATABASE_NAME"

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
  cp ./target/release/liberated-chat-server* ./bundle
  rm ./bundle/liberated-chat-server.d
  cp -R ./liberated-chat-frontend/dist/* ./bundle/"$FRONTEND_PATH"

bundle-debug: build-all-debug
  mkdir -p ./bundle
  mkdir -p ./bundle/"$DATABASE_PATH"
  mkdir -p ./bundle/"$FRONTEND_PATH"
  cp ./target/debug/liberated-chat-server* ./bundle
  rm ./bundle/liberated-chat-server.d
  cp -R ./liberated-chat-frontend/dist/* ./bundle/"$FRONTEND_PATH"

