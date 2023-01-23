#!/usr/bin/env bash 
set -e

cd ../../jumpcrypto-solarc
anchor build

cd ../dominari/arc2
anchor build

clockwork localnet \
    --bpf-program ../../jumpcrypto-solarc/localhost_keypairs/core-ds-keypair.json ../../jumpcrypto-solarc/target/deploy/core_ds.so \
    --bpf-program localhost_keypairs/registry-keypair.json target/deploy/registry.so \
    --bpf-program localhost_keypairs/dominari-keypair.json target/deploy/dominari.so
