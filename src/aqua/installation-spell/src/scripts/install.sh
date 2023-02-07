#!/usr/bin/env bash

aqua run -i install.aqua \
    -f "install_to_relay(\"$1\")" \
    --plugin ./plugins \
    --addr /ip4/127.0.0.1/tcp/9999/ws/p2p/12D3KooWBh6FcSJBQTCFLfiorVUuYKBiasyJgKLGdT7Ecp4HUyaz \
