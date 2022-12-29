import { useState, useRef, useContext, useMemo, useEffect } from 'react';
import { WasmTile, WasmPlayer } from '../util/interfaces';
import { Stage, Container, render } from 'react-pixi-fiber'
import * as PIXI from 'pixi.js';
import { DOMINARI_PROGRAM_ID, REGISTRY_PROGRAM_ID } from '../util/constants';
import { useConnection } from '@solana/wallet-adapter-react';
import { GameState } from 'dominari-sdk';
import { GameContext } from '../pages/game';
import { ixWasmToJs, randomU64 } from '../util/util';
import { Transaction } from '@solana/web3.js';

/*
if (window.gamestate == undefined || window.gamestate == null) {
    return(
    <div>
        <p> Please load a game instance from settings first! </p>
    </div>
    )
}
*/

const Map = () => {
    const {connection} = useConnection();
    let containerRef = useRef<Container>(null);
    let stageRef = useRef<Stage>(null);
    const gameContext = useContext(GameContext);
    const [player, setPlayer] = useState({} as WasmPlayer);
    let gamestate: GameState =  new GameState(
        connection.rpcEndpoint,
        DOMINARI_PROGRAM_ID.toString(),
        REGISTRY_PROGRAM_ID.toString(),
        gameContext.instance
    );
    const [selectedTile, selectTile] = useState("");

    const setup = async () => {
        await gamestate.load_state();
        const blueprintJson = await (await fetch('blueprints/blueprints.json')).json()
        gamestate.add_blueprints(blueprintJson);
        renderMap(gamestate);
        console.log(gameContext.privateKey.publicKey.toString());
        setPlayer(gamestate.get_player_info(gameContext.privateKey.publicKey.toString()));
    }


    let renderMap = (gamestate: GameState) => {
        let grid:WasmTile[][] = gamestate.get_map();
        let boxSize = 125;    
        //rowNum = y, colNum = x
        for(let rowNum=0; rowNum<grid.length; rowNum++){
            let row = grid[rowNum];
            for(let colNum=0; colNum<row.length; colNum++){
                let box = new PIXI.Graphics();
                box.name = `${colNum},${rowNum}`;
                box.beginFill(0x000000);
                box.drawRect(5+(colNum*boxSize), 5+(rowNum*boxSize), boxSize-5, boxSize-5);
                containerRef.current?.addChild!(box);

                // XY Coordinate on Top Left
                let text = new PIXI.Text(`${colNum},${rowNum}`, {
                    fontFamily: 'Sans-Serif',
                    fontSize: 12,
                    fill: 0xFFFFFF,
                    align: 'center'
                });    
                text.position.x = 10 + (boxSize * colNum)
                text.position.y = 10 + (boxSize * rowNum)
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
                    featureSprite.position.x = 80 + (boxSize * colNum);
                    featureSprite.position.y = 10 + (boxSize * rowNum);
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
                            box!.beginFill(0x000000);
                            box!.drawRect(5+(colNum*boxSize), 5+(rowNum*boxSize), boxSize-5, boxSize-5);            
                        } else {
                            box!.beginFill(0xee6363);
                            box!.drawRect(5+(colNum*boxSize), 5+(rowNum*boxSize), boxSize-5, boxSize-5);
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
                            box!.beginFill(0x000000);
                            box!.drawRect(5+(colNum*boxSize), 5+(rowNum*boxSize), boxSize-5, boxSize-5);            
                        } else {
                            console.log("Tile not selected")
                            selectTile(`${colNum},${rowNum}`);
                            box!.beginFill(0xee6363);
                            box!.drawRect(5+(colNum*boxSize), 5+(rowNum*boxSize), boxSize-5, boxSize-5);
                        }
                    })
                }
                troopSprite.anchor.x = 0;
                troopSprite.anchor.y = 0;
                troopSprite.width = 50;
                troopSprite.height = 50;
                troopSprite.position.x = 10 + (boxSize * colNum);
                troopSprite.position.y = 70 + (boxSize * rowNum);
                containerRef.current?.addChild!(troopSprite);
            
            }
        }
    }

    useEffect(() => {
        setup()
    }, [])

    useEffect(()=>{
        console.log(gamestate.is_state_loaded)
        if(gamestate.is_state_loaded){
            console.log("State loaded");
            renderMap(gamestate);
        } else {
            console.log("State not loaded");
        }
    }, [selectedTile])

    return(
        <div className="flex flex-col">
            <div className="h-10 items-center mt-4">
                <PlayerFragment player={player} setPlayer={setPlayer} gamestate={gamestate}></PlayerFragment>
            </div>
            <Stage options={{height: 125*8 +5, width: 125*8 +5, backgroundColor: 0xFFFFFF}} ref={stageRef}>
                <Container ref={containerRef}></Container>
            </Stage>
        </div>  
    )    
}


export default Map;

const PlayerFragment = ({player, setPlayer, gamestate}: {player: WasmPlayer, setPlayer: Function, gamestate: GameState}) => {
    const gameContext = useContext(GameContext);
    const nameref = useRef<HTMLInputElement>(null);
    const imageref = useRef<HTMLInputElement>(null);
    const {connection} = useConnection();

    const createPlayer = async () => {
        let player_id = randomU64();
        let createPlayerIx = ixWasmToJs(gameContext.dominari.init_player(
            gameContext.privateKey.publicKey.toString(),
            gameContext.instance,
            player_id,
            nameref.current?.value!,
            imageref.current?.value!
        ));
        let tx = new Transaction();
        tx.add(createPlayerIx);
        tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
        tx.feePayer = gameContext.privateKey.publicKey;
        tx.sign(gameContext.privateKey);
        const sig = await connection.sendRawTransaction(tx.serialize(), {skipPreflight: true});
        await connection.confirmTransaction(sig);
        console.log("TX Confirmed: ", sig);
        gamestate.update_instance_index();
        gamestate.update_entity(player_id);
        setPlayer(gamestate.get_player_info(gameContext.privateKey.publicKey.toString()));
    }

    useEffect(() => {}, [player])

    if(player){
        console.log(player);
        return(
            <div className="flex gap-4">
                <p>{player.name}</p>
                <label>Score</label>
                <p>{player.score}</p>
                <label>Kills</label>
                <p>{player.kills}</p>
            </div>    
        )
    } else {
        console.log("No Player!");
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
}


/**
 *     const getTiles = () => {
        console.log(`Loading state for instance: ${window.gamestate.instance}`)
        window.gamestate.load_state()
        .then(() => {
            let map:WasmTile[][] = window.gamestate.get_map();
            console.log(map);
            return <>Test</>    
        })
        return <>Not Loaded State</>
    }
 */

//https://codepen.io/manofthelake/pen/pojZxqP?editors=1001