#!/usr/bin/env bash

CWD="$(dirname "$0")"
# go to the project root
cd "$CWD/../.."

aqua --no-relay -i src/aqua -o src/air --air > /dev/null

# node sk: bL8RRGuBJEWSj4JKzLCUgR/EY8+lit2g1LE2BE1oF/U=
aqua run -i src/aqua/install.aqua \
    -f 'upstall_spell_to_relay(script, app_config)' \
    --plugin src/plugins \
    --addr /ip4/127.0.0.1/tcp/9999/ws/p2p/12D3KooWB7fEjubgmpJAtzTKCdWeLmXadR1VP2mYCNZMzgBWkKef \
    --timeout 60000 \
    --data '{
        "script": "/Users/folex/Development/fluencelabs/spell/src/aqua/installation-spell/src/air/spell.install.air",
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