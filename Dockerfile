FROM rust:1.79

WORKDIR /usr/src/myapp

COPY . .

RUN apt-get update -y
RUN cargo install tauri-cli@^2.0.0-beta
RUN apt-get install javascriptcoregtk-4.1 libsoup-3.0 webkit2gtk-4.1 -y  \
    && apt-get install libgtk-3-dev
