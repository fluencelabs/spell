#!/usr/bin/env bash

CWD="$(dirname "$0")"
# go to the project root
cd "$CWD/../.."

aqua --no-relay -i src/aqua -o src/air --air > /dev/null

# node sk: 3LGQSnGbAor5i1sSNFAIBJw76AENxYV/at7VgRv4pI4=
aqua run -i src/aqua/install.aqua \
    -f 'upstall_spell_to_relay(script, app_config)' \
    --plugin src/plugins \
    --addr /ip4/127.0.0.1/tcp/9999/ws/p2p/12D3KooWJDdHPGakMsVH4UytjjxeV6qjgB1ci6qTFjrrM6fDRyUs \
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