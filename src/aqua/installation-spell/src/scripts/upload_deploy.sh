#!/usr/bin/env bash

CWD="$(dirname "$0")"
# go to the project root
PROJECT="$CWD/../.."

aqua --no-relay -i src/aqua -o src/air --air > /dev/null

NOW=$(date +%s)

# node sk: bL8RRGuBJEWSj4JKzLCUgR/EY8+lit2g1LE2BE1oF/U=
aqua run -i "$PROJECT/src/aqua/cli.aqua" \
    -f 'upload_deploy(deploy_config)' \
    --plugin "$PROJECT/src/plugins" \
    --addr /ip4/127.0.0.1/tcp/9999/ws/p2p/12D3KooWB7fEjubgmpJAtzTKCdWeLmXadR1VP2mYCNZMzgBWkKef \
    --timeout 60000 \
    --data '{
        "deploy_config": {
            "installation_script": '"$(cat /Users/folex/Development/fluencelabs/spell/src/aqua/installation-spell/src/air/spell.install.air | jq -Rs)"',
            "installation_trigger": {
                "clock": { "start_sec": 1676293670, "end_sec": 0, "period_sec": 600 },
                "connections": { "connect": false, "disconnect": false },
                "blockchain": { "start_block": 0, "end_block": 0 }
            },
            "workers": [
                {
                    "name": "lampert",
                    "hosts": ["12D3KooWB7fEjubgmpJAtzTKCdWeLmXadR1VP2mYCNZMzgBWkKef"],
                    "config": {
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
                        ],
                        "spells": []
                    }
                }
            ]
        }
    }'