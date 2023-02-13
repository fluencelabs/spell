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
            "hosts": ["12D3KooWB7fEjubgmpJAtzTKCdWeLmXadR1VP2mYCNZMzgBWkKef"],
            "worker": {
                "services": [
                    {
                        "name": "url-downloader",
                        "modules": [
                            {
                                "wasm": "/Users/folex/Development/fluencelabs/examples/marine-examples/url-downloader/artifacts/curl_adapter.wasm",
                                "config": "/Users/folex/Development/fluencelabs/examples/marine-examples/url-downloader/artifacts/curl_adapter.json"
                            },
                            {
                                "wasm": "/Users/folex/Development/fluencelabs/examples/marine-examples/url-downloader/artifacts/local_storage.wasm",
                                "config": "/Users/folex/Development/fluencelabs/examples/marine-examples/url-downloader/artifacts/local_storage.json"
                            },
                            {
                                "wasm": "/Users/folex/Development/fluencelabs/examples/marine-examples/url-downloader/artifacts/facade.wasm",
                                "config": "/Users/folex/Development/fluencelabs/examples/marine-examples/url-downloader/artifacts/facade.json"
                            }
                        ]
                    }
                ]
            }
        }
    }'