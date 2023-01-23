## How to run

### Local container
```shell
docker run --platform linux/amd64 -p7777 -p9999 --rm docker.fluence.dev/rust-peer:ipfs_master_584_1 --local
```

### Upload App Config
```shell
aqua run -i install.aqua \
    -f 'upload_to_relay(app_config)' \
    --plugin ./plugins \
    --addr /ip4/127.0.0.1/tcp/9999/ws/p2p/12D3KooWBh6FcSJBQTCFLfiorVUuYKBiasyJgKLGdT7Ecp4HUyaz \
    --data '{
        "app_config": {
            "basedir": "/Users/folex/Development/fluencelabs/examples/marine-examples/url-downloader/artifacts/",
            "services": [
                {
                    "name": "url-downloader",
                    "modules": [
                        {
                            "wasm": "curl_adapter.wasm",
                            "config": "curl_adapter.json"
                        },
                        {
                            "wasm": "local_storage.wasm",
                            "config": "local_storage.json"
                        },
                        {
                            "wasm": "facade.wasm",
                            "config": "facade.json"
                        }
                    ]
                }
            ]
        }
    }'
```
