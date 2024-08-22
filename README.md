## Rest-api with autogenerated wasm client library

Actix-web rest-api with [progenitor](https://github.com/oxidecomputer/progenitor) and [openapi-generator](https://github.com/OpenAPITools/openapi-generator) generated client library from OpenAPI specification, using [wasm-pack](https://github.com/rustwasm/wasm-pack).

### Building OpenAPI

First we build our `openapi.json` specification which will later be served at http://127.0.0.1:8082/swagger :

```
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
nvm install 20
npm install --global yarn
npm install --global @redocly/cli@latest

yarn bundle
```

### Generating web client with Progenitor

Then we generate javascript library using a `wasm-pack` :

```
# install web-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

cd progenitor
wasm-pack build --target web
```

### Generating web client with OpenAPI generator

Then we generate javascript library using a `wasm-pack` :

```
cd openapi-gen

java -jar openapi-generator-cli.jar generate \
    -i ../openapi.json \
    -g rust \
    -o ./rust2 \
    --additional-properties=packageName=openapi-gen,library=reqwest

cd rust
wasm-pack build --target web
```

### Running Actix-web http server

```
cargo run
```

```
curl -X POST http://127.0.0.1:8082/blocks -H "Content-Type: application/json" -d '{"block_id":"frog", "height":1}'
curl -X POST http://127.0.0.1:8082/addresses -H "Content-Type: application/json" -d '{"address":"croco", "balance":2}'
```

```
curl http://127.0.0.1:8082/blocks
curl http://127.0.0.1:8082/addresses
```

Now you can access : 
  - swagger at http://127.0.0.1:8082/swagger 
  - progenitor demo at http://127.0.0.1:8082/progenitor
  - openapi-gen demo at http://127.0.0.1:8082/openapi-gen
