
### rpcjson

https://github.com/paritytech/jsonrpc

https://crates.io/crates/jsonrpc-http-server

https://www.youtube.com/watch?v=FaPrnPMY_po

https://substrate.stackexchange.com/questions/2435/invalid-params-for-rpc

### Example of client invocation using cURL:


Temporizar servidor -> Copiar Bloco auxiliar para BD -> Validar e Adicionar Bloco

Trasacao da wallet -> se validar jogar no Bloco auxiliar 

Mudar block geracao de ID

RUST_LOG=info cargo run --bin server
curl -X POST -H "Content-Type: application/json" -d '{"id": 13, "jsonrpc": "2.0", "method": "ping" }' 127.0.0.1:8333

curl -X POST -H "Content-Type: application/json" -d '{"id": 13, "jsonrpc": "2.0", "method": "add", "params":[1,2] }' 127.0.0.1:8333

### Initializing docker and installing Tauri

# Using Debian Linux:
### Build the DockerFile (it may take some time)

`xhost +local:docker`

`docker build -t <image_tag> .` 

`docker run -it --network host -e DISPLAY=$DISPLAY -v /tmp/.X11-unix:/tmp/.X11-unix <image_tag> bash`

### Run the application 

`cargo tauri dev`

### Logging

To run with logging info and above run as `RUST_LOG=info cargo run --bin server`

