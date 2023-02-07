#!/usr/bin/env bash

aqua --no-relay -i install.aqua -o air --air > /dev/null

aqua run -i install.aqua \
    -f 'upstall_spell_to_relay(script, app_config)' \
    --plugin ./plugins \
    --addr /ip4/127.0.0.1/tcp/9999/ws/p2p/12D3KooWLwQvL1tPesXUgYoWF1dGgea5YSS3WipCHd6GAAErbvRR \
    --data '{
        "script": "/Users/folex/Development/fluencelabs/spell/src/aqua/installation-spell/air/install.install.air",
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