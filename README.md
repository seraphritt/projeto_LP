
### rpcjson

https://github.com/paritytech/jsonrpc

https://crates.io/crates/jsonrpc-http-server

https://www.youtube.com/watch?v=FaPrnPMY_po

https://substrate.stackexchange.com/questions/2435/invalid-params-for-rpc

### Example of client invocation using cURL:

curl -X POST -H "Content-Type: application/json" -d '{"id": 13, "jsonrpc": "2.0", "method": "ping" }' 127.0.0.1:8333

curl -X POST -H "Content-Type: application/json" -d '{"id": 13, "jsonrpc": "2.0", "method": "add", "params":[1,2] }' 127.0.0.1:8333