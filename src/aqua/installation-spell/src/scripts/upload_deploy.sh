#!/usr/bin/env bash

CWD="$(dirname "$0")"
# go to the project root
PROJECT="$CWD/../.."

aqua -i src/aqua -o src/air --air > /dev/null

NOW=$(date +%s)

# node sk: bL8RRGuBJEWSj4JKzLCUgR/EY8+lit2g1LE2BE1oF/U=
aqua run -i "$PROJECT/src/aqua/cli.aqua" \
    --sk "8wJyRzI3K8NPlCGSf9E+6ExB5MBdXQAz7m0jjjjjjNg=" \
    --show-config \
    -f 'upload_deploy(deploy_config)' \
    --plugin "$PROJECT/src/plugins" \
    --addr /ip4/127.0.0.1/tcp/9990/ws/p2p/12D3KooWHBG9oaVx4i3vi6c1rSBUm7MLBmyGmmbHoZ23pmjDCnvK \
    --timeout 60000 \
    --data '{
        "deploy_config": {
            "installation_script": '"$(cat /Users/folex/Development/fluencelabs/spell/src/aqua/installation-spell/src/air/deal_spell.deal_install.air | jq -Rs)"',
            "installation_trigger": {
                "clock": { "start_sec": 1676293670, "end_sec": 0, "period_sec": 600 },
                "connections": { "connect": false, "disconnect": false },
                "blockchain": { "start_block": 0, "end_block": 0 }
            },
            "workers": [
                {
                    "name": "lampert",
                    "hosts": ["12D3KooWHBG9oaVx4i3vi6c1rSBUm7MLBmyGmmbHoZ23pmjDCnvK"],
                    "config": {
                        "services": [
                            {
                                "name": "url-downloader",
                                "modules": [
                                    {
                                        "wasm": "/Users/folex/Development/fluencelabs/examples/marine-examples/url-downloader/artifacts/curl_adapter.wasm",
                                        "config": '"$(cat /Users/folex/Development/fluencelabs/examples/marine-examples/url-downloader/artifacts/curl_adapter.json | jq -Rs)"'
                                    },
                                    {
                                        "wasm": "/Users/folex/Development/fluencelabs/examples/marine-examples/url-downloader/artifacts/local_storage.wasm",
                                        "config": '"$(cat /Users/folex/Development/fluencelabs/examples/marine-examples/url-downloader/artifacts/local_storage.json | jq -Rs)"'
                                    },
                                    {
                                        "wasm": "/Users/folex/Development/fluencelabs/examples/marine-examples/url-downloader/artifacts/facade.wasm",
                                        "config": '"$(cat /Users/folex/Development/fluencelabs/examples/marine-examples/url-downloader/artifacts/facade.json | jq -Rs)"'
                                    }
                                ]
                            }
                        ],
                        "spells": [
                            {
                                "name": "test-spell",
                                "script": '"$(cat /Users/aleksey/Documents/dev/fluencelabs/spell/src/aqua/installation-spell/src/air/test_spell.main.air | jq -Rs)"',
                                "init_args": {
                                    "test_arg": "alex folex"
                                },
                                "config": {
                                    "clock": { "start_sec": 1, "end_sec": 0, "period_sec": 30 },
                                    "connections": { "connect": false, "disconnect": false },
                                    "blockchain": { "start_block": 0, "end_block": 0 }
                                }
                            }
                        ]
                    }
                }
            ]
        }
    }'