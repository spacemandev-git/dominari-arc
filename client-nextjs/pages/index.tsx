import { NextPage } from "next"
import { FaGamepad, FaWrench } from 'react-icons/fa';
import { createContext, useRef, useState, useEffect, useContext, ReactNode } from "react";
import { Dominari, GameState } from "dominari-sdk";
import { DOMINARI_PROGRAM_ID, LOCAL_STORAGE_GAMEINSTANCES, LOCAL_STORAGE_PRIVATEKEY, REGISTRY_PROGRAM_ID } from "../util/constants";
import { Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { useLocalStorage } from "usehooks-ts";
import {encode, decode} from 'bs58';
import toml from 'toml';
import {randomU64, ixPack, ixWasmToJs} from '../util/util';
import { Stage, Container } from 'react-pixi-fiber'
import { WasmTile, WasmPlayer } from '../util/interfaces';
import * as PIXI from 'pixi.js';

//@ts-ignore
import NoSSR from 'react-no-ssr';
import { ConfigFileInterface } from "../util/interfaces";

export const DominariContext = createContext({} as DominariContextInterface);
export interface DominariContextInterface {
    connection: Connection,
    setRPC: Function,
    privateKey: Keypair,
    setPrivateKeyStr: Function,
    gamestate: GameState,
    setInstance: Function,
    dominari: Dominari,
}

enum NavEnum {
    Settings,
    Map
}

const Home: NextPage = () => {
    // state stuff
    const [nav, setNav] = useState(NavEnum.Settings);
    const [rpc, setRPC] = useState("http://localhost:8899");
    const [connection, setConnection] = useState(new Connection(rpc));
    const [dominari, setDominari] = useState(new Dominari(DOMINARI_PROGRAM_ID.toBase58()));
    const [instance, setInstance] = useState(BigInt("0"));
    const [privateKey, setPrivateKey] = useState(new Keypair());
    const [privateKeyStr, setPrivateKeyStr] = useLocalStorage(LOCAL_STORAGE_PRIVATEKEY, "");

    let gamestate = useRef<GameState>(new GameState(
        rpc,
        DOMINARI_PROGRAM_ID.toString(),
        REGISTRY_PROGRAM_ID.toString(),
        instance
    ));

    useEffect(() => {
        console.log(`Changing connection to ${rpc}`);
        setConnection(new Connection(rpc))
    }, [rpc])

    useEffect(() => {
        gamestate.current = new GameState(
            rpc, 
            DOMINARI_PROGRAM_ID.toString(),
            REGISTRY_PROGRAM_ID.toString(),
            instance
        )
    }, [instance])

    // Updates whenever PrivateKeyStr is update
    useEffect(() => {
        if(privateKeyStr == "" || privateKeyStr == null) {
            setPrivateKeyStr(encode(privateKey.secretKey))
        } else {
            console.log(privateKeyStr);
            setPrivateKey(Keypair.fromSecretKey(decode(privateKeyStr)));
        }
    }, [privateKeyStr])

    return (
        <NoSSR>
            <DominariContext.Provider value={{
                    connection,
                    setRPC,
                    privateKey,
                    setPrivateKeyStr,
                    gamestate: gamestate.current,
                    setInstance,
                    dominari,
                }}>
                <div className="grid grid-col-2">
                    <div className="fixed top-0 left-0 h-screen w-16 flex flex-col
                        bg-white dark:bg-gray-900 shadow-lg gap-4 items-center">
                        <div className="sidebar-icon group mt-36">
                            <FaWrench size="48" onClick={() => {setNav(NavEnum.Settings)}} />
                        </div>
                        <div className="sidebar-icon group">
                            <FaGamepad size="48" onClick={() => {setNav(NavEnum.Map)}} />
                        </div>
                    </div>
                    <div className="ml-32 mt-6 mr-20 border-white border-2">
                        {nav == NavEnum.Settings && <Settings></Settings>}
                        {nav == NavEnum.Map && <Map></Map>}
                    </div>
                </div>
            </DominariContext.Provider>
        </NoSSR>
    )
}
export default Home;

const Settings = () => {
    // Context Imports
    const { 
        connection,
        setRPC,
        privateKey,
        setPrivateKeyStr,
        gamestate,
        setInstance,
        dominari
    } = useContext(DominariContext);


    // HTML REFs
    const privKeyRef = useRef<HTMLInputElement>(null);
    const instanceRef = useRef<HTMLInputElement>(null);
    const selectInstanceRef = useRef<HTMLSelectElement>(null);
    const configFileRef = useRef<HTMLInputElement>(null);
    const rpcRef = useRef<HTMLInputElement>(null);


    // Local Component State
    const [balance, updateBalance] = useState(0);
    const getBalance = async () => {
        console.log("Refreshing balance");
        updateBalance(await connection.getBalance(privateKey.publicKey!));
    }
    useEffect(() => {getBalance()});
    const airdrop = async () => {
        await connection.requestAirdrop(privateKey.publicKey!, 1e9);
        alert("Requested! Please wait a while to refresh...")
    }

    // LocalStorage
    const [instanceListStr, setInstanceListStr] = useLocalStorage(LOCAL_STORAGE_GAMEINSTANCES, "[]");

    //Create Game Function
    const getInstanceOptions = () => {
        try{
            let instanceStrList: string[] = JSON.parse(instanceListStr);
            if(instanceListStr) {
                return instanceStrList.map((instanceID:string) => { return  <option key={instanceID} value={instanceID}>{instanceID}</option>})    
            } else {
                return []
            }    
        } catch (e) {
            throw e;
        }
    }


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
                return dominari.get_blueprint_key(val);
            }
        })
        console.log("Config File Post Transform", configFile);
        
        let ixGroup = [];
        // Create Game Instance
        const newInstanceId = randomU64();
        setInstance(newInstanceId);
        const createGameInstanceIx = dominari.create_game_instance(privateKey.publicKey.toString(), newInstanceId, configFile.config);
        ixGroup.push(ixWasmToJs(createGameInstanceIx));

        // Init Map 
        const mapId = randomU64();
        const initMapIx = dominari.init_map(privateKey.publicKey.toString(), newInstanceId, mapId, configFile.map.mapmeta.max_x, configFile.map.mapmeta.max_y);
        ixGroup.push(ixWasmToJs(initMapIx));
        const recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
        ixGroup.forEach((ixG) => {
            let tx = new Transaction();
            tx.add(ixG);
            tx.recentBlockhash = recentBlockhash,
            tx.feePayer = privateKey.publicKey;
            tx.sign(privateKey);
            connection.sendRawTransaction(tx.serialize(), {skipPreflight: true}).then( (sig) => {
                console.log("Create Instance & Map: ", sig);
            })
            
        })


        let tileIxGroup = [];
        // Init Tiles
        for(let x=0; x<configFile.map.mapmeta.max_x; x++){
            for(let y=0; y<configFile.map.mapmeta.max_y; y++){
                let tileId = randomU64();
                const initTileTx = dominari.init_tile(privateKey.publicKey.toString(), newInstanceId, tileId, x, y, BigInt(configFile.map.cost_per_tile.toString()))
                tileIxGroup.push(ixWasmToJs(initTileTx));
            }
        }
        let ixPacked = await ixPack(tileIxGroup);
        let txGroup = []
        for (let ixP of ixPacked){
            const tx = new Transaction();
            tx.add(...ixP);
            tx.recentBlockhash = recentBlockhash;
            tx.feePayer = privateKey.publicKey;
            tx.sign(privateKey)
            txGroup.push(tx);
        }
        console.log("Creating Game Instance, Map, and Tiles TX: ", txGroup);
        let promises = txGroup.map((tx) => {
            return connection.sendRawTransaction(tx.serialize(), {skipPreflight: true})
        });
        let sigs = await Promise.all(promises)
        let confirmations = sigs.map((sig) => {
            return connection.confirmTransaction(sig);
        });
        await Promise.all(confirmations);
        console.log("All tiles created!");

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
            let featureIx = ixWasmToJs(dominari.init_feature(
                privateKey.publicKey.toString(),
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
            tx.feePayer = privateKey.publicKey;
            tx.sign(privateKey);
            return tx;
        })
        .forEach(async (tx) => {
            const sig = await connection.sendRawTransaction(tx.serialize(), {skipPreflight: true})
            console.log("Feature Tx: ", sig);
            await connection.confirmTransaction(sig);
        })

        let oldInstanceStr:string[] | null | undefined = JSON.parse(instanceListStr);
        if(oldInstanceStr) {
            oldInstanceStr.push(newInstanceId.toString());
            setInstanceListStr(JSON.stringify(oldInstanceStr));
        } else {
            setInstanceListStr(JSON.stringify([newInstanceId.toString]));
        }
        alert(`Instance ${newInstanceId} finished initializing.`)
    }

    return(
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
                    onClick={() => {
                        setRPC(rpcRef.current?.value!);
                    }}
                >Set RPC</button>
            </div>
            <div className="mt-12 flex flex-row gap-4">
                <label>Private Key: </label>
                <input type='text' className="w-96" ref={privKeyRef} defaultValue={encode(privateKey.secretKey)}></input>
                <button className="bg-slate-600" onClick={()=>{
                    setPrivateKeyStr(privKeyRef.current?.value!);
                }}> Load </button>
            </div>
            <div className="mt-12 flex flex-row gap-4">
                <label>Public Key: {privateKey.publicKey.toString()}</label>
                <label> Balance: {balance} Lamports ({balance/1e9} SOL)</label>
                <button className="bg-slate-600" onClick={getBalance}>Refresh </button>
                <button className="bg-slate-600" onClick={airdrop}>Airdrop 1 SOL</button>
            </div>
            <div className="mt-12 flex flex-row gap-4">
                <label>Game ID</label>
                <input type="text" defaultValue={gamestate.instance.toString()} ref={instanceRef}></input>
                <button className="bg-slate-600" onClick={() => {
                    if(BigInt(instanceRef.current?.value!) >= 0) {
                        setInstance(BigInt(instanceRef.current?.value!));
                    }
                }}>Change Game ID</button>
                <label>Created Game IDs</label>
                <select onChange={(selection) => {
                    //console.log(selection.target.value);
                    // Update Game Instance when Selection Made
                    setInstance(BigInt(selection.target.value));
                }}>
                    <option>Select Instance</option>
                    {getInstanceOptions()}
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

const Map = () => {
    // Context Imports
    const {
        connection,
        privateKey,
        gamestate,
        dominari
    } = useContext(DominariContext);

    // Refs
    let containerRef = useRef<Container>(null);
    let stageRef = useRef<Stage>(null);

    // State
    const [player, setPlayer] = useState({} as WasmPlayer);
    const [selectedTile, selectTile] = useState("");

    // Render Steps
    // Load State and setup listenrs
    useEffect(() => {
        const setup = async () => {
            await gamestate.load_state();
            const blueprintJson = await (await fetch('blueprints/blueprints.json')).json()
            gamestate.add_blueprints(blueprintJson);
            setPlayer(gamestate.get_player_info(privateKey.publicKey.toString()));
        }

        setup().then(() => {
            // Setup Listeners (Call Render Map on each Change)

            // Render Map
            renderMap();
        });
    }, [])

    
    useEffect(() => {
        if(gamestate.is_state_loaded){
            renderMap();            
        }
    }, [selectedTile])
    

    // Constants
    const TILE_SIZE = 125;
    const UNSLECTED_TILE_COLOR = 0x000000;
    const SELECTED_TILE_COLOR = 0xee6363;

    // Functions
    const renderMap = () => {
        console.log("Rendering Map!");
        let grid:WasmTile[][] = gamestate.get_map();

                for(let rowNum=0; rowNum<grid.length; rowNum++){
            let row = grid[rowNum];
            for(let colNum=0; colNum<row.length; colNum++){
                let box = new PIXI.Graphics();
                box.name = `${colNum},${rowNum}`;
                if(selectedTile == box.name) {
                    box.beginFill(0xee6363);
                } else {
                    box.beginFill(0x000000);
                }
                box.drawRect(5+(colNum*TILE_SIZE), 5+(rowNum*TILE_SIZE), TILE_SIZE-5, TILE_SIZE-5);
                containerRef.current?.addChild!(box);

                // XY Coordinate on Top Left
                let text = new PIXI.Text(`${colNum},${rowNum}`, {
                    fontFamily: 'Sans-Serif',
                    fontSize: 12,
                    fill: 0xFFFFFF,
                    align: 'center'
                });    
                text.position.x = 10 + (TILE_SIZE * colNum)
                text.position.y = 10 + (TILE_SIZE * rowNum)
                containerRef.current?.addChild!(text);
                
                // TileInfo
                let tileInfo:WasmTile = grid[rowNum][colNum];

                // Add Feature Icon
                if(tileInfo.feature){
                    let featureSprite = PIXI.Sprite.from(`assets/features/${tileInfo.feature.name.toLowerCase()}.png`);
                    featureSprite.anchor.x = 0;
                    featureSprite.anchor.y = 0;
                    featureSprite.width = 50;
                    featureSprite.height = 50;
                    featureSprite.position.x = 80 + (TILE_SIZE * colNum);
                    featureSprite.position.y = 10 + (TILE_SIZE * rowNum);
                    containerRef.current?.addChild!(featureSprite);
                }
                // Add Troop Icon
                let troopSprite = PIXI.Sprite.from(`assets/add_unit.png`);
                troopSprite.interactive = true;

                if(tileInfo.troop){
                    let box:PIXI.Graphics | undefined = containerRef.current?.getChildByName!(`${colNum},${rowNum}`);
                    troopSprite = PIXI.Sprite.from(`assets/troops/${tileInfo.troop.name.toLowerCase()}.png`);
                    troopSprite.on("mousedown", () => {
                        if(selectedTile == `${colNum},${rowNum}`){
                            selectTile("");
                            box!.drawRect(5+(colNum*TILE_SIZE), 5+(rowNum*TILE_SIZE), TILE_SIZE-5, TILE_SIZE-5);            
                        } else {
                            box!.drawRect(5+(colNum*TILE_SIZE), 5+(rowNum*TILE_SIZE), TILE_SIZE-5, TILE_SIZE-5);
                            selectTile(`${colNum},${rowNum}`);
                        }
                        console.log("Move Troop")
                    })
                } else {
                    troopSprite.on("mousedown", () => {
                        console.log("Add Unit!")
                        if(selectedTile == `${colNum},${rowNum}`){
                            console.log("Tile currently selected")
                            selectTile("");
                        } else {
                            console.log("Tile not selected")
                            selectTile(`${colNum},${rowNum}`);
                        }
                    })
                }
                troopSprite.anchor.x = 0;
                troopSprite.anchor.y = 0;
                troopSprite.width = 50;
                troopSprite.height = 50;
                troopSprite.position.x = 10 + (TILE_SIZE * colNum);
                troopSprite.position.y = 70 + (TILE_SIZE * rowNum);
                containerRef.current?.addChild!(troopSprite);
            }
        }

    }

    return(
        <div>
            <div className="h-10 gap-4">
                {player?.name && <PlayerFragment></PlayerFragment>}
                {!player?.name && <CreatePlayerFragment></CreatePlayerFragment>}
            </div>
            <Stage options={{height: 125*8 +5, width: 125*8 +5, backgroundColor: 0xFFFFFF}} ref={stageRef}>
                <Container ref={containerRef}></Container>
            </Stage>
        </div>
    )
}

const PlayerFragment = () => {
    return(
        <div>
            <label>Player Logged In</label>
        </div>
    )
}

const CreatePlayerFragment = () => {
    return(
        <div>
            <label>Create Player</label>
        </div>
    )
}