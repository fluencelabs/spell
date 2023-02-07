#!/usr/bin/env bash

SCRIPT_DIR="$(dirname $0)"

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