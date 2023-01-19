#!/usr/bin/env bash 
cd ../../../sol-arc
anchor build

# Deploy Universe
solana program deploy --program-id localhost_keypairs/core-ds-keypair.json target/deploy/core_ds.so

cd ../dominari/arc2

anchor build
# Deploy Registry
solana program deploy --program-id localhost_keypairs/registry-keypair.json target/deploy/registry.so
# Deploy Dominari Action Bundle
solana program deploy --program-id localhost_keypairs/dominari-keypair.json target/deploy/dominari.so

