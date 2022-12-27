import { useEffect, useRef, useContext, useMemo } from 'react';
import { WasmTile } from '../util/interfaces';
import { Stage, Container } from 'react-pixi-fiber'
import * as PIXI from 'pixi.js';
import { DOMINARI_PROGRAM_ID, REGISTRY_PROGRAM_ID } from '../util/constants';
import { useConnection } from '@solana/wallet-adapter-react';
import { GameState } from 'dominari-sdk';
import { GameContext } from '../pages/game';


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

    let gamestate = useMemo(() => {}, [gameContext.instance])

    useEffect(() => {
        const setup = async() => {
            /*
             let grid = [
                [1,2,3,4,5,6,7,8],
                [1,2,3,4,5,6,7,8],
                [1,2,3,4,5,6,7,8],
                [1,2,3,4,5,6,7,8],
                [1,2,3,4,5,6,7,8],
                [1,2,3,4,5,6,7,8],
                [1,2,3,4,5,6,7,8],
                [1,2,3,4,5,6,7,8]
            ];
            */

            let gamestate = new GameState(
                connection.rpcEndpoint,
                DOMINARI_PROGRAM_ID.toString(),
                REGISTRY_PROGRAM_ID.toString(),
                BigInt("280357192616367311")
            );
            await gamestate.load_state();
            

            let grid:WasmTile[][] = gamestate.get_map();
            console.log("Grid: ", grid);

            let boxSize = 125;    
            //rowNum = y, colNum = x
            for(let rowNum=0; rowNum<grid.length; rowNum++){
                let row = grid[rowNum];
                for(let colNum=0; colNum<row.length; colNum++){
                    let box = new PIXI.Graphics();
                    box.beginFill(0x000000);
                    box.drawRect(5+(colNum*boxSize), 5+(rowNum*boxSize), boxSize-5, boxSize-5);
                    containerRef.current?.addChild!(box);

                    let text = new PIXI.Text(`${colNum},${rowNum}`, {
                        fontFamily: 'Arial',
                        fontSize: 12,
                        fill: 0xFFFFFF,
                        align: 'center'
                    });
    
                    text.position.x = 10 + (boxSize * colNum)
                    text.position.y = 10 + (boxSize * rowNum)
                    containerRef.current?.addChild!(text);
                }
            }
            
        }

        setup().then(() => {return;})
    }, [gameContext.instance])

    return(
        <div className="grid">
            <div className="h-12">Create Player | Player Stats</div>
            <Stage options={{height: 125*8 +5, width: 125*8 +5, backgroundColor: 0xFFFFFF}} ref={stageRef}>
                <Container ref={containerRef}>
                </Container>
            </Stage>
        </div>  
    )    
}


export default Map;

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