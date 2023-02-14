#!/usr/bin/env bash 

anchor build
# Deploy Registry
solana program deploy --program-id localhost_keypairs/registry-keypair.json target/deploy/registry.so --url localhost
# Deploy Dominari Action Bundle
solana program deploy --program-id localhost_keypairs/dominari-keypair.json target/deploy/dominari.so --url localhost

