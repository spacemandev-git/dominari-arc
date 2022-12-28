import { useConnection, useLocalStorage } from '@solana/wallet-adapter-react';
import { Keypair, PublicKey, Transaction } from '@solana/web3.js';
import {FC, useContext, useRef, useState, useEffect} from 'react';
import { GameContext } from '../pages/game';
import {encode, decode} from 'bs58';
import toml from 'toml';
import { ixPack, ixWasmToJs, randomU64 } from '../util/util';
import { ConfigFileInterface } from '../util/interfaces';
import { GameState } from 'dominari-sdk';
import { DOMINARI_PROGRAM_ID, LS_GAMEINSTANCES, REGISTRY_PROGRAM_ID } from '../util/constants';

const Settings: FC = () => {
    const {connection} = useConnection();
    let gameContext = useContext(GameContext);
    const privKeyRef = useRef<HTMLInputElement>(null);
    const instanceRef = useRef<HTMLInputElement>(null);
    const selectInstanceRef = useRef<HTMLSelectElement>(null);
    const configFileRef = useRef<HTMLInputElement>(null);

    const [balance, updateBalance] = useState(0);
    const getBalance = async () => {
        console.log("Refreshing balance");
        updateBalance(await connection.getBalance(gameContext.privateKey.publicKey!));
    }
    const airdrop = async () => {
        await connection.requestAirdrop(gameContext.privateKey.publicKey!, 1e9);
        alert("Requested! Please wait a while to refresh...")
    }

    useEffect(() => {
        getBalance();
    }, [gameContext.privateKey])

    const createGame = async () => {
        /*
        Game Loop
        1. Create Game
            -> Create Game Instance
            -> Initialize Map
            -> Initialize Tiles
            -> Initialize Features
        2. Toggle Game State
        3. Game Actions
        */

        // Read Config File
        let configFile:ConfigFileInterface = toml.parse(await configFileRef.current?.files?.item(0)?.text() as string)
        configFile.config.starting_cards = configFile.config.starting_cards.map((val:string) => {
            // it can either be turned into a PublicKey, in which case, leave it as is
            // or get blueprint key from the name
            try{
                let nkey = new PublicKey(val);
                return val; //if this passes, the val was a pubkey to begin with
            } catch (e) {
                return gameContext.dominari.get_blueprint_key(val);
            }
        })
        console.log("Config File Post Transform", configFile);
        
        let ixGroup = [];
        // Create Game Instance
        const newInstanceId = randomU64();
        gameContext.changeInstance(newInstanceId);
        const createGameInstanceIx = gameContext.dominari.create_game_instance(gameContext.privateKey.publicKey.toString(), newInstanceId, configFile.config);
        ixGroup.push(ixWasmToJs(createGameInstanceIx));

        // Init Map 
        const mapId = randomU64();
        const initMapIx = gameContext.dominari.init_map(gameContext.privateKey.publicKey.toString(), newInstanceId, mapId, configFile.map.mapmeta.max_x, configFile.map.mapmeta.max_y);
        ixGroup.push(ixWasmToJs(initMapIx));

        // Init Tiles
        for(let x=0; x<configFile.map.mapmeta.max_x; x++){
            for(let y=0; y<configFile.map.mapmeta.max_y; y++){
                let tileId = randomU64();
                const initTileTx = gameContext.dominari.init_tile(gameContext.privateKey.publicKey.toString(), newInstanceId, tileId, x, y, BigInt(configFile.map.cost_per_tile.toString()))
                ixGroup.push(ixWasmToJs(initTileTx));
            }
        }
        let ixPacked = await ixPack(ixGroup);
        const recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
        let txGroup = []
        for (let ixP of ixPacked){
            const tx = new Transaction();
            tx.add(...ixP);
            tx.recentBlockhash = recentBlockhash;
            tx.feePayer = gameContext.privateKey.publicKey;
            tx.sign(gameContext.privateKey)
            txGroup.push(tx);
        }
        console.log("Creating Game Instance, Map, and Tiles TX: ", txGroup);
        for(let tx of txGroup){
            const sig = await connection.sendRawTransaction(tx.serialize(), {skipPreflight: true})
            console.log("Create Game Tx: ", sig);
            await connection.confirmTransaction(sig);
        }

        // Init Features
            // Build Index
            // Fetch Entites
            // Sort through them to Find Feature Tiles

        let gamestate = new GameState(
            connection.rpcEndpoint,
            DOMINARI_PROGRAM_ID.toString(),
            REGISTRY_PROGRAM_ID.toString(),
            newInstanceId //not using the full form cause the update doesn't happen til after the rerender
        );

        await gamestate.load_state();
        
        let featureIxG = [];
        for(let feature of configFile.map.features) {
            let tile_id = BigInt(gamestate.get_tile_id(feature.x, feature.y));
            let featureIx = ixWasmToJs(gameContext.dominari.init_feature(
                gameContext.privateKey.publicKey.toString(),
                newInstanceId,
                randomU64(),
                tile_id,
                feature.feature
            ));
            featureIxG.push(featureIx);
        }
        (await ixPack(featureIxG))
        .map((ixG) => {
            const tx = new Transaction();
            tx.add(...ixG);
            tx.recentBlockhash = recentBlockhash;
            tx.feePayer = gameContext.privateKey.publicKey;
            tx.sign(gameContext.privateKey);
            return tx;
        })
        .forEach(async (tx) => {
            const sig = await connection.sendRawTransaction(tx.serialize(), {skipPreflight: true})
            console.log("Feature Tx: ", sig);
            await connection.confirmTransaction(sig);
        })

        alert(`Instance ${newInstanceId} finished initializing.`)
    }

    const getInstanceList = () => {
        const instanceList = ["280357192616367311"]
        return instanceList.map((instance:string) => {
            return (<option key={instance} value={instance}>{instance}</option>)
        })
    }

    const rpcRef = useRef<HTMLInputElement>(null);

    const changeConnection = (evt:any) => {
        gameContext.changeConnection(rpcRef.current?.value)
    }

    return (
        <div>
            <h1 className="text-3xl"> Settings </h1>
            <div className="mt-12 flex flex-row gap-4">
                <label>RPC: </label>
                <input 
                    ref={rpcRef}
                    className="self-center w-64" 
                    defaultValue={connection.rpcEndpoint}
                ></input>
                <button
                    className="bg-slate-600"
                    onClick={changeConnection}
                >Set RPC</button>
            </div>
            <div className="mt-12 flex flex-row gap-4">
                <label>Private Key: </label>
                <input type='text' className="w-96" ref={privKeyRef} defaultValue={encode(gameContext.privateKey.secretKey)}></input>
                <button className="bg-slate-600" onClick={()=>{
                    gameContext.changePrivateKey(Keypair.fromSecretKey(decode(privKeyRef.current?.value!)));
                    localStorage.setItem("privateKey", privKeyRef.current?.value!)
                }}> Load </button>
            </div>
            <div className="mt-12 flex flex-row gap-4">
                <label>Public Key: {gameContext.privateKey.publicKey!.toString()}</label>
                <label> Balance: {balance} Lamports ({balance/1e9} SOL)</label>
                <button className="bg-slate-600" onClick={getBalance}>Refresh </button>
                <button className="bg-slate-600" onClick={airdrop}>Airdrop 1 SOL</button>
            </div>
            <div className="mt-12 flex flex-row gap-4">
                <label>Game ID</label>
                <input type="text" defaultValue={gameContext.instance.toString()} ref={instanceRef}></input>
                <button className="bg-slate-600" onClick={() => {
                    if(parseInt(instanceRef.current?.value!) >= 0) {
                        gameContext.changeInstance(BigInt(instanceRef.current?.value!));
                    }
                }}>Change Game ID</button>
                <label>Created Game IDs</label>
                <select onSelect={() => {
                    if(parseInt(instanceRef.current?.value!) >= 0) {
                        gameContext.changeInstance(BigInt(selectInstanceRef.current?.value!));
                        console.log(gameContext.instance);
                    }
                }}>
                    {getInstanceList()}
                </select>
            </div>
            <div className="mt-12 flex flex-row gap-4">
                <label>Game Config</label>
                <input type="file" ref={configFileRef}></input>
                <button className="bg-slate-600" onClick={createGame}>Create Game w/ Config</button>
            </div>
        </div>
    )
}

export default Settings;

//13712002795722475028