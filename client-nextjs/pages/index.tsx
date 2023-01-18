import { NextPage } from "next"
import { FaGamepad, FaWrench } from 'react-icons/fa';
import { createContext, useRef, useState, useEffect, useContext } from "react";
import { Dominari, GameState } from "dominari-sdk";
import { COLORS, DOMINARI_PROGRAM_ID, LOCAL_STORAGE_GAMEINSTANCES, LOCAL_STORAGE_PRIVATEKEY, REGISTRY_PROGRAM_ID } from "../util/constants";
import { ComputeBudgetProgram, Connection, Keypair, PublicKey, Transaction, TransactionMessage, VersionedTransaction } from "@solana/web3.js";
import { useLocalStorage } from "usehooks-ts";
import {encode, decode} from 'bs58';
import toml from 'toml';
import {randomU64, ixPack, ixWasmToJs, getDistance} from '../util/util';
import { Stage, Container } from 'react-pixi-fiber'
import { WasmTile, WasmPlayer, NavEnum, Blueprints, PlayPauseState, WasmTroop } from '../util/interfaces';
import * as PIXI from 'pixi.js';
import { Observable } from "rxjs";

// Events Decoder Imports
import {Layout} from "@solana/buffer-layout";
import { Idl, IdlTypeDef, IdlEvent } from "@project-serum/anchor/dist/cjs/idl";
import { IdlCoder } from "@project-serum/anchor/dist/cjs/coder/borsh/idl";
const DominariIDL:Idl = require("../../target/idl/dominari.json");
import { sha256 } from "js-sha256";
import { Event, EventData } from "@project-serum/anchor";

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
    blueprints: Blueprints,
    setNav: Function, // for debug to instantly switch after game state loaded
    playpause: PlayPauseState,
    setPlayPause: Function,
    slot: number,
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
    const [blueprints, setBlueprints] = useState({} as Blueprints);
    const [playpause, setPlayPause] = useState("Paused" as PlayPauseState);
    const [slot,setSlot] = useState(0);

    let gamestate = useRef<GameState>(new GameState(
        rpc,
        DOMINARI_PROGRAM_ID.toString(),
        REGISTRY_PROGRAM_ID.toString(),
        instance
    ));

    // Load Blueprints
    useEffect(() => {
        const blueprintSetup = async () => {
            return await (await fetch('blueprints/blueprints.json')).json()
            
        };
        blueprintSetup().then((blueprintJson: Blueprints) => {
            setBlueprints(blueprintJson);
        })
    }, [])

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
        );
    }, [instance])

    // Updates whenever PrivateKeyStr is update
    useEffect(() => {
        if(privateKeyStr == "" || privateKeyStr == null) {
            setPrivateKeyStr(encode(privateKey.secretKey))
        } else {
            console.log(privateKeyStr);
            setPrivateKey(Keypair.fromSecretKey(decode(privateKeyStr)));
        }
    }, [privateKeyStr]);


    useEffect(() => {
        const interval = setInterval(async () => setSlot(await connection.getSlot()), 1000);
        return () => {
          clearInterval(interval);
        };
    }, []);

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
                    blueprints,
                    setNav,
                    playpause,
                    setPlayPause,
                    slot,
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
                        {nav == NavEnum.Map && <MapPage></MapPage>}
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
        dominari,
        setPlayPause
    } = useContext(DominariContext);


    // HTML REFs
    const privKeyRef = useRef<HTMLInputElement>(null);
    const instanceRef = useRef<HTMLInputElement>(null);
    const selectInstanceRef = useRef<HTMLSelectElement>(null);
    const configFileRef = useRef<HTMLInputElement>(null);
    const rpcRef = useRef<HTMLInputElement>(null);


    // Local Component State
    const [balance, updateBalance] = useState(0);
    const getBalance = () => {
        connection.getBalance(privateKey.publicKey!).then((balance) => {
            updateBalance(balance)
        });
    }
    const airdrop = async () => {
        await connection.requestAirdrop(privateKey.publicKey!, 100e9);
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
        setPlayPause(gamestate.get_play_phase());

        
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
                <button className="bg-slate-600" onClick={airdrop}>Airdrop 100 SOL</button>
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
                    instanceRef.current!.value = selection.target.value;
                    alert("Game Loaded, switch to Map!");
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

const MapPage = () => {
    // Context Imports
    const {
        connection,
        privateKey,
        gamestate,
        dominari,
        blueprints,
        setPlayPause,
        slot
    } = useContext(DominariContext);

    // Refs
    let containerRef = useRef<Container>(null);
    let stageRef = useRef<Stage>(null);

    // State
    const [player, setPlayer] = useState({} as WasmPlayer);
    const [selectedTroopTile, selectTroopTile] = useState({} as WasmTile);
    const [hoverTroopTile, setHoverTroopTile] = useState({} as WasmTile);
    const [showAddTroopModal, setTroopModal] = useState(false);
    const [showUseFeatureModal, setFeatureModal] = useState(false);

    // Non React State
    let eventsObservable:Observable<{
        slot: number,
        name: string,
        data: any
    }>;

    // Render Steps
    // Load State and setup listenrs`
    useEffect(() => {
        const setup = async () => {
            await gamestate.load_state();
            gamestate.add_blueprints(Object.keys(blueprints));
            setPlayer(gamestate.get_player_info(privateKey.publicKey.toString()));
        }

        setup().then(() => {
            // Setup Listeners (Call Render Map on each Change)
            //const program = new Program<DominariTypes>(DominariIDL, DOMINARI_PROGRAM_ID, {connection});
            const LOG_START_INDEX = "Program data: ".length;

            // EVENT DECODER LOGIC
            // TODO: Move this somewhere else
            
            const layoutList: [string, Layout<any>][] = DominariIDL.events!.map((event) => {
                let eventTypeDef: IdlTypeDef = {
                    name: event.name,
                    type: {
                      kind: "struct",
                      fields: event.fields.map((f) => {
                        return { name: f.name, type: f.type };
                      }),
                    },
                  };
                  return [event.name, IdlCoder.typeDefLayout(eventTypeDef, DominariIDL.types!)];
            })
            const eventDiscriminator = (name:string) => {
                return Buffer.from(sha256.digest(`event:${name}`)).subarray(0,8);
            }
            const layouts = new Map(layoutList);
            const descriminators = new Map<string, string>(
                DominariIDL.events === undefined
                  ? []
                  : DominariIDL.events.map((e) => [
                      eventDiscriminator(e.name).toString('base64'),
                      e.name,
                    ])
            );

            const decode = <E extends IdlEvent = IdlEvent, T = Record<string, never>>(log:string):Event<E, T>|null=> {
                let logArr: Buffer = Buffer.from(log, 'base64');
                const disc = logArr.subarray(0,8).toString('base64')
                const eventName = descriminators.get(disc);
                if(!eventName){
                    return null;
                }
                const layout = layouts.get(eventName!);
                const data = layout!.decode(logArr.slice(8)) as EventData<
                    E["fields"][number],
                    T
                >;

                return {data, name:eventName!};
            }

            eventsObservable = new Observable((subscriber) => {
                connection.onLogs(DOMINARI_PROGRAM_ID, (logs, ctx) => {
                    for(let log of logs.logs){
                        if(log.startsWith("Program data:")){
                            const logStr = log.slice(LOG_START_INDEX);
                            const event = decode(logStr);
                            if(event) {
                                subscriber.next({
                                    slot: ctx.slot,
                                    name: event.name,
                                    data: event.data
                                })
                            }
                        }
                    }
                },
                "confirmed"
                )
            })

            eventsObservable.subscribe(async (event) => {
                console.log("EVENT: ", event);
                if(event.data.instance != gamestate.instance.toString()){
                    return;
                }

                switch(event.name) {
                    case "GameStateChanged": 
                        if(event.data.newState.play){
                            setPlayPause("Play");
                        } else if (event.data.newState.paused){
                            setPlayPause("Paused");
                        } else if (event.data.newState.lobby){
                            setPlayPause("Lobby")
                        } else if (event.data.newState.build){
                            setPlayPause("Build")
                        } else if (event.data.newState.finished) {
                            setPlayPause("Finished")
                        }
                        break;
                    case "NewUnitSpawned":
                        console.log("New Unit Spawned!");
                        // Update Tile Entity (update occupant)
                        await gamestate.update_entity(BigInt(event.data.tile));
                        // Update Unit Entity (Create if doesn't exist)
                        await gamestate.update_entity(BigInt(event.data.unit));
                        // Update Entity Player (reduce cards)
                        await  gamestate.update_entity(BigInt(event.data.player));
                        setPlayer(gamestate.get_player_info(privateKey.publicKey.toString()));
                        // Update instance index
                        await gamestate.update_instance_index();
                        renderTile(gamestate.get_wasm_tile(BigInt(event.data.tile)));
                        break;
                    case "TroopMovement": 
                        await gamestate.update_entity(BigInt(event.data.from));
                        await gamestate.update_entity(BigInt(event.data.to));
                        await gamestate.update_entity(BigInt(event.data.unit));
                        renderMap();
                        break;
                    case "TileAttacked": 
                        await gamestate.update_entity(BigInt(event.data.attacker));
                        await gamestate.update_entity(BigInt(event.data.defender));
                        await gamestate.update_entity(BigInt(event.data.defendingTile));
                        renderMap();
                        break;
                }
            });

            // Render Map
            renderMap();
        });
    }, [])

    
    useEffect(() => {
        if(gamestate.is_state_loaded){
            renderMap();
            
            // After map is rendered, check if selected tile is a troop
            // Then renderTile() for it's movement           
            if(!selectedTroopTile.troop){ // && selectedTroopTile.x){
                // Add troop modal if there's no troop on the tile
                setTroopModal(true);
            }
        }
    }, [selectedTroopTile])
    

    // Constants
    const TILE_SIZE = 125;
    const UNSLECTED_TILE_COLOR = COLORS.BLACK;
    const SELECTED_TILE_COLOR = COLORS.WHITE;

    const renderTile = (tile: WasmTile) => {
        let box = new PIXI.Graphics();
        box.name = `${tile.x},${tile.y}`;
        if(selectedTroopTile.x == tile.x && selectedTroopTile.y == tile.y) {
            box.beginFill(SELECTED_TILE_COLOR);
        } else {
            //Check if tile is within movement distance of selected tile
            if(
                selectedTroopTile.troop &&
                selectedTroopTile.troop.troop_owner_player_key == privateKey.publicKey.toString() && 
                getDistance(selectedTroopTile.x, selectedTroopTile.y, tile.x, tile.y) <= selectedTroopTile.troop.movement){
                box.beginFill(COLORS.GREEN);
            } else {
                box.beginFill(UNSLECTED_TILE_COLOR);
            }
        }
        box.drawRect(5+(tile.x*TILE_SIZE), 5+(tile.y*TILE_SIZE), TILE_SIZE-5, TILE_SIZE-5);
        containerRef.current?.addChild!(box);

        // XY Coordinate on Top Left
        let text = new PIXI.Text(`${tile.x},${tile.y}`, {
            fontFamily: 'Sans-Serif',
            fontSize: 12,
            fill: 0xFFFFFF,
            align: 'center'
        });    
        text.position.x = 10 + (TILE_SIZE * tile.x);
        text.position.y = 10 + (TILE_SIZE * tile.y);
        containerRef.current?.addChild!(text);
        
        // Add Feature Icon
        if(tile.feature){
            let featureSprite = PIXI.Sprite.from(`assets/features/${tile.feature.name.toLowerCase()}.png`);
            featureSprite.anchor.x = 0;
            featureSprite.anchor.y = 0;
            featureSprite.width = 50;
            featureSprite.height = 50;
            featureSprite.position.x = 70 + (TILE_SIZE * tile.x);
            featureSprite.position.y = 10 + (TILE_SIZE * tile.y);
            containerRef.current?.addChild!(featureSprite);
        }
        // Add Troop Icon
        let troopSprite;
        if(tile.troop){
            troopSprite = PIXI.Sprite.from(`assets/troops/${tile.troop.name.toLowerCase()}.png`);
        } else {
            troopSprite = PIXI.Sprite.from(`assets/add_unit.png`);
        }

        troopSprite.interactive = true;
        troopSprite.on("mousedown", async () => {
            if(selectedTroopTile.x == tile.x && selectedTroopTile.y == tile.y){
                // This tile is currently selected
                selectTroopTile({} as WasmTile);
            } else {
                // This tile is not selected
                // IF there is a selectedTile, check if this new tile is eligible for move or attack
                // ELSE select this tile
                if(selectedTroopTile.troop && tile.troop && getDistance(selectedTroopTile.x, selectedTroopTile.y, tile.x, tile.y) <= selectedTroopTile.troop.attack_range ){
                    // This tile is within attack range of the selected tile's troop
                    console.log("Attacking Unit!");
                    const attackIx = ixWasmToJs(dominari.attack_unit(
                        privateKey.publicKey.toString(),
                        gamestate.instance,
                        BigInt(selectedTroopTile.troop.id),
                        BigInt(tile.troop.id),
                        BigInt(gamestate.get_tile_id(tile.x, tile.y)),
                    ));

                    const tx = new VersionedTransaction(new TransactionMessage({
                        payerKey: privateKey.publicKey,
                        recentBlockhash: (await connection.getLatestBlockhash()).blockhash,
                        instructions: [
                            ComputeBudgetProgram.setComputeUnitLimit({units: 1400000}),
                            attackIx
                        ]
                    }).compileToLegacyMessage());
                    tx.sign([privateKey]);
                    try{
                        const sig = await connection.sendRawTransaction(tx.serialize(), {skipPreflight: true});
                        console.log(sig);
                    } catch (e){
                        console.log(e);
                    }

                } else if(selectedTroopTile.troop && getDistance(selectedTroopTile.x, selectedTroopTile.y, tile.x, tile.y) <= selectedTroopTile.troop.movement) {
                    // Tile doesn't have a troop and is within movement range of the selected Troop

                    const moveIx = ixWasmToJs(dominari.move_unit(
                        privateKey.publicKey.toString(),
                        gamestate.instance,
                        BigInt(selectedTroopTile.troop.id),
                        BigInt(gamestate.get_tile_id(selectedTroopTile.x, selectedTroopTile.y)),
                        BigInt(gamestate.get_tile_id(tile.x, tile.y)),
                    ));

                    const tx = new VersionedTransaction(new TransactionMessage({
                        payerKey: privateKey.publicKey,
                        recentBlockhash: (await connection.getLatestBlockhash()).blockhash,
                        instructions: [
                            moveIx
                        ]
                    }).compileToLegacyMessage());
                    tx.sign([privateKey]);
                    const sig = await connection.sendTransaction(tx, {skipPreflight: true});
                } else {
                    selectTroopTile(tile);
                }
            }
        })

        troopSprite.on("mouseover", () => {
            // Show info about selected unit
            //console.log("Troop Health: ", tile.troop?.health);
            setHoverTroopTile(tile);
        });

        troopSprite.anchor.x = 0;
        troopSprite.anchor.y = 0;
        troopSprite.width = 50;
        troopSprite.height = 50;
        troopSprite.position.x = 10 + (TILE_SIZE * tile.x);
        troopSprite.position.y = 70 + (TILE_SIZE * tile.y);
        troopSprite.name = `${tile.x},${tile.y}:UNIT`;
        let attackSprite = PIXI.Sprite.from(`assets/attack.png`);
        containerRef.current?.addChild!(troopSprite);

        if(tile.troop?.troop_owner_player_key == privateKey.publicKey.toString()) {
            troopSprite.tint = COLORS.GREEN;
        } else if (tile.troop && selectedTroopTile.troop && getDistance(tile.x, tile.y, selectedTroopTile.x, selectedTroopTile.y) <= selectedTroopTile.troop.attack_range) {
            // Draw a little cross swords on the sprite to show it's within attacking range
            attackSprite.width = 35;
            attackSprite.height = 35;
            attackSprite.position.x = 10 + (TILE_SIZE * tile.x);
            attackSprite.position.y = 70 + (TILE_SIZE * tile.y);
            containerRef.current?.addChild!(attackSprite);
        }        
    }

    // Functions
    const renderMap = () => {
        let grid: WasmTile[] = gamestate.get_map();
        for(let tile of grid){
            renderTile(tile);
        }
    }

    return(
        <div>
            <div className="h-10 gap-4 items-center mt-4 ml-2">
                {player?.name && <PlayerFragment {...{player}}></PlayerFragment>}
                {!player?.name && <CreatePlayerFragment {...{setPlayer}}></CreatePlayerFragment>}
            </div>
            {showAddTroopModal && <AddTroopModal {...{setShowModal: setTroopModal, player: player, selectedTroopTile: selectedTroopTile}}></AddTroopModal>}
            <div className="flex flex-row">
                <Stage options={{height: 125*8 +5, width: 125*8 +5, backgroundColor: 0xFFFFFF}} ref={stageRef}>
                    <Container ref={containerRef}></Container>
                </Stage>
                <div className="flex flex-col ml-10">
                    <h1 className="text-3xl">Selected Troop</h1>
                    <TroopInfo troop={selectedTroopTile.troop}></TroopInfo>
                    <br/><br/><br/><br/>
                    <h1 className="text-3xl">Hover Troop</h1>
                    <TroopInfo troop={hoverTroopTile.troop}></TroopInfo>
                </div>
            </div>
        </div>
    )
}

const TroopInfo = ({troop}: {troop:WasmTroop}) => {
    const {
        connection,
        slot
    } = useContext(DominariContext);

    if(!troop){
        return(<></>)
    } else {
        return(
            <div className="flex flex-col">
                <label>Troop: {troop.name} | Class {troop.class} </label>
                <label>ID: {troop.id}</label>
                <label>Owner: {troop.troop_owner_player_key}</label>
                <label>Health: {troop.health}</label>
                <label>Damage {troop.min_damage}..{troop.max_damage} </label>
                <div className="flex flex-row">
                    <label className="p-1">INF+  {troop.bonus_infantry}</label>
                    <label className="p-1">ARMR+ {troop.bonus_armor}</label>
                    <label className="p-1">AIR+  {troop.bonus_aircraft}</label>
                    <label className="p-1">FEAT+ {troop.bonus_feature}</label>
                </div>
                <label>Move: {troop.movement} | ATK Range: {troop.attack_range}</label>
                <label>Last Used Slot: {troop.last_used} | Recovery: {troop.recovery} </label>
                <label>Current Slot: {slot} | Ready In: {( slot-(parseInt(troop.last_used) + parseInt(troop.recovery)))/-2}s</label>
            </div>
        )       
    }
}

const PlayerFragment = ({player}: {player:WasmPlayer}) => {
    const {
        dominari,
        connection,
        privateKey,
        gamestate,
        playpause,
    } = useContext(DominariContext);

    return(
        <div className="flex gap-4">
            <p>{player.name}</p>
            <label>Score</label>
            <p>{player.score}</p>
            <label>Kills</label>
            <p>{player.kills}</p>
            <label>Game State: {playpause}</label>
            <button onClick={async ()=>{
                const changeStateIx = ixWasmToJs(dominari.change_game_state(
                    privateKey.publicKey.toString(),
                    gamestate.instance,
                    BigInt((gamestate.get_player_info(privateKey.publicKey.toString())).id),
                    playpause == "Play" ? "Paused" : "Play",
                ));
                let tx = new Transaction();
                tx.add(changeStateIx);
                const recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
                tx.recentBlockhash = recentBlockhash;
                tx.feePayer = privateKey.publicKey;
                tx.sign(privateKey);
                connection.sendRawTransaction(tx.serialize(), {skipPreflight: true}).then((sig) => {
                    console.log("Game State Tx: ", sig);
                })
            }}>Pause/Play</button>  
        </div>    
    )
}

const CreatePlayerFragment = ({setPlayer}: {setPlayer:Function}) => {
    //Game Context 
    const {
        dominari,
        privateKey,
        gamestate,
        connection
    } = useContext(DominariContext);

    // Refs
    const nameref = useRef<HTMLInputElement>(null);
    const imageref = useRef<HTMLInputElement>(null);

    // Create Player Function
    const createPlayer = async () => {
        let player_id = randomU64();
        let createPlayerIx = ixWasmToJs(dominari.init_player(
            privateKey.publicKey.toString(),
            gamestate.instance,
            player_id,
            nameref.current?.value!,
            imageref.current?.value!
        ));
        let tx = new Transaction();
        tx.add(createPlayerIx);
        tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
        tx.feePayer = privateKey.publicKey;
        tx.sign(privateKey);
        const sig = await connection.sendRawTransaction(tx.serialize(), {skipPreflight: true});
        await connection.confirmTransaction(sig);
        console.log("TX Confirmed: ", sig);

        // TODO: Something here causes an error!
        gamestate.update_instance_index();
        gamestate.update_entity(player_id);
        setPlayer(gamestate.get_player_info(privateKey.publicKey.toString()));
    }

    return(
    <div className="flex gap-4">
        <label>Name</label>
        <input ref={nameref}></input>
        <label>Image</label>
        <input ref={imageref}></input>
        <button onClick={createPlayer}>Join Game</button>
    </div>
    )
}

const AddTroopModal = ({setShowModal, player, selectedTroopTile}: {setShowModal:Function, player:WasmPlayer, selectedTroopTile: WasmTile}) => {
    // Context
    const {
        connection,
        dominari,
        privateKey,
        gamestate
    } = useContext(DominariContext);
    let cards = player.cards;

    const [selectedUnit, setSelectedUnit] = useState("");
    const getUnitCards = () => {
        let componentArray:JSX.Element[] = []
        for(let i=0; i<cards.length; i++){
            componentArray.push(
                <CardComponent 
                    blueprintName={cards[i]} 
                    key={randomU64().toString()}
                    id={`${cards[i]}-${i}`}  // name-id
                    setSelected={setSelectedUnit}
                    selected={selectedUnit}    
                ></CardComponent>
            )
        }
        return componentArray;
    }

    return(
        <>
          <div
            className="justify-center items-center flex overflow-x-hidden overflow-y-auto fixed inset-0 z-50 outline-none focus:outline-none"
          >
            <div className="relative w-auto my-6 mx-auto max-w-3xl">
              {/*content*/}
              <div className="border-0 rounded-lg shadow-lg relative flex flex-col w-full bg-black outline-none focus:outline-none">
                {/*header*/}
                <div className="flex items-start justify-between p-5 border-b border-solid border-slate-200 rounded-t">
                  <h3 className="text-3xl font-semibold">
                    My Troops
                  </h3>
                  <button
                    className="p-1 ml-auto bg-transparent border-0 text-black opacity-5 float-right text-3xl leading-none font-semibold outline-none focus:outline-none"
                    onClick={() => setShowModal(false)}
                  >
                    <span className="bg-transparent text-black opacity-5 h-6 w-6 text-2xl block outline-none focus:outline-none">
                      x
                    </span>
                  </button>
                </div>
                {/*body*/}
                <div className="relative p-6 grid grid-cols-2">
                {getUnitCards()}
                </div>
                {/*footer*/}
                <div className="flex items-center justify-end p-6 border-t border-solid border-slate-200 rounded-b">
                  <button
                    className="text-red-500 background-transparent font-bold uppercase px-6 py-2 text-sm outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150"
                    type="button"
                    onClick={() => setShowModal(false)}
                  >
                    Close
                  </button>
                  <button
                    className="bg-emerald-500 text-white active:bg-emerald-600 font-bold uppercase text-sm px-6 py-3 rounded shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150"
                    type="button"
                    onClick={async () => {
                        console.log(`Spawning ${selectedUnit}`)
                        let tile = selectedTroopTile;
                        let tileID = BigInt(gamestate.get_tile_id(tile.x, tile.y))
                        let playerID:bigint = BigInt((gamestate.get_player_info(privateKey.publicKey.toString())).id);
                        
                        console.log(playerID);
                        // Spawn Unit
                        let ix = ixWasmToJs(dominari.spawn_unit(
                            privateKey.publicKey.toString(), 
                            gamestate.instance,
                            playerID,
                            randomU64(),
                            tileID,
                            selectedUnit.split("-")[0]
                        ));
                        
                        let spawnUnitTx = new Transaction();
                        let recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
                        spawnUnitTx.add(ComputeBudgetProgram.setComputeUnitLimit({units:1400000}));
                        spawnUnitTx.add(ix);
                        spawnUnitTx.recentBlockhash = recentBlockhash;
                        spawnUnitTx.feePayer = privateKey.publicKey;
                        spawnUnitTx.sign(privateKey);
                        let sig =  await connection.sendRawTransaction(spawnUnitTx.serialize(), {skipPreflight: true});
                        console.log(`Spawn Unit Tx: ${sig}`);
                        setShowModal(false)
                    }}
                  >
                    Spawn Unit
                  </button>
                </div>
              </div>
            </div>
          </div>
          <div className="opacity-25 fixed inset-0 z-40 bg-black"></div>
        </>
    )
}

const CardComponent = ({blueprintName, id, setSelected, selected}: {blueprintName:string, id:string, setSelected: Function, selected:string}) => {
    const {
        blueprints
    } = useContext(DominariContext);

    if(!blueprints[blueprintName]){
        return(
            <div>
                <label>{blueprintName}</label>
                <label>Blueprint not found locally!</label>
            </div>
        )
    }

    const unitOrMod:any = blueprints[blueprintName];
    const imageSrc = unitOrMod.metadata.entity_type == "Unit" ? `assets/troops/${blueprintName}.png` : `assets/mods/${blueprintName}.png`
    const [bgColor, setBgColor] = useState("black");

    useEffect(() => {
        if(selected == id) {
            setBgColor("#ee6363");
        }
    }, [selected])

    return (
    <div onClick={() => {setSelected(id)}} style={{backgroundColor: bgColor}} className="w-[150]">
        <label className="ml-4">{blueprintName}</label>
        <div className="mx-4 my-2 flex gap-2">
            <img src={imageSrc} className="border-2 border-white w-20 h-20"></img>
            <div className="grid grid-cols-2">
                <label>Health: {unitOrMod.health.health}</label>
                <label>Class: {unitOrMod.troop_class.class}</label>
                <label>Movement: {unitOrMod.range.movement}</label>
                <label>Attack Range: {unitOrMod.range.attack_range}</label>
                <label>Damage: {unitOrMod.damage.min_damage}-{unitOrMod.damage.max_damage}</label>
                <label>Recovery: {unitOrMod.last_used.recovery}</label>
                <label>Value: {unitOrMod.value.value}</label>
            </div>
        </div>
    </div>
    )
}