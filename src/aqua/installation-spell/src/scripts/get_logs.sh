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
    -f 'get_logs(workers)' \
    --plugin "$PROJECT/src/plugins" \
    --addr /ip4/127.0.0.1/tcp/9990/ws/p2p/12D3KooWHBG9oaVx4i3vi6c1rSBUm7MLBmyGmmbHoZ23pmjDCnvK \
    --timeout 60000 \
    --data '{
        "workers": {

            "workers": [
                {
                    "name": "lampert",
                    "definition": "Qmc4EDxzCSeb7AUuXmVPHxXyxiS4wCCp5mLawbDGcqp3rC",
                    "installation_spells": [
                        {
                            "host_id": "12D3KooWHBG9oaVx4i3vi6c1rSBUm7MLBmyGmmbHoZ23pmjDCnvK",
                            "spell_id": "85b2138d-9ab0-4fbe-bc9e-af39f7303f11",
                            "worker_id": "TBD"
                        }
                    ]
                }
            ]
        }
    }'