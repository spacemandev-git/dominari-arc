# Setup


# Dependencies
- Solana Test Validator
- Rust
- Wasm-pack
- yarn
- ts-node and typescript installed globally

## Clone Repo
```sh
git clone git@github.com:spacemandev-git/dominari-arc.git
```

## Generating the WASM SDK

```sh
cd dominari-sdk
./bundler.sh
./nodejs.sh
```

## Solana Test Validator & Deploy
```sh
# Run this in a seperate terminal so it can run
solana-test-validator
```

```sh 
cd ../scripts
./deploy.sh
```


## Initialize and Register Stuff
Use a private key that has tokens on your local test validator (usually ~/.config/solana/id.json)

```sh
cd ./client-ts
ts-node admin.ts <privateKey> H5mieGWWK6qukHoNzbR6ysLxReeQC4JHZcNM6JkPQnm3 3YdayPtujByJ1g1DWEUh7vpg78gZL49FWyD5rDGyof9T
```

## Get into the Game
```sh
cd ./client-nextjs
yarn dev
```

This will create a new key for the player, airdrop a couple SOL into the key. 
Create a new game using 2playerconfig.toml found in public/configs in client-nextjs. 
After the game is created, there will be an alert telling you the game id. Refresh the page (TODO error).
Select the game from the drop down. Then switch over to the game tab. Then register a player at the top of the page. Refresh the page (TODO error). Load in again, select game from dropdown, then you should have the player on the top of the page. Hit Play/Pause to start the game, then click any + icon on the map to spawn units.