import type { NextPage } from 'next';
import React, {createContext, useState, useMemo} from 'react';
import {useEffectOnce} from 'usehooks-ts';
import { Keypair, PublicKey } from '@solana/web3.js';
import { DOMINARI_PROGRAM_ID, DUMMY_PRIV_KEY } from '../util/constants';
import Menu from '../components/menu';
import GameBoard from '../components/gameboard';
import Header from '../components/header';
import {encode, decode} from 'bs58';
import {Dominari, GameState} from 'dominari-sdk';
/**
 * Create Game
 *  -> Takes a Config.toml
 * 
 * Join Game
 *  -> Takes an instance ID
 *  -> Create Player if it doesnt' Exist 
 *  -> 
 * Game Loop
 *  -> Hand for Units, 
 *  -> Tile Map for Board
 */

export const GameContext = createContext({} as GameContextInterface);

const GamePage: NextPage = (props:any) => {
    const [nav, changeNav] = useState(NavEnum.GameBoard);
    const [privateKey, changePrivateKey] = useState(Keypair.fromSecretKey(decode(DUMMY_PRIV_KEY)));
    const [instance, changeInstance] = useState(BigInt("280357192616367311"));
    const [dominari, updateDominari] = useState(new Dominari(DOMINARI_PROGRAM_ID.toBase58()))
    //const [gamestate, updateGameState] = useState(null);

    useEffectOnce(() => {
        // bs58 encoded secret key
        const previousPrivateKey = localStorage.getItem('privateKey');
        if(previousPrivateKey == null || previousPrivateKey == "null"){
            let pKey = new Keypair();
            localStorage.setItem('privateKey', encode(pKey.secretKey));
            changePrivateKey(pKey);
        } else {
            let pKey = Keypair.fromSecretKey(decode(previousPrivateKey))
            changePrivateKey(pKey);
        }
    })

    return (
        <GameContext.Provider value={{
            changeConnection: props.setEndpoint,
            dominari,
            instance,
            changeInstance,
            nav,
            changeNav,
            privateKey,
            changePrivateKey,
        }}>
            <div className="grid grid-col-2">
                <Menu></Menu>
                <div className="ml-32 mt-6 mr-20 border-white border-2">
                    <GameBoard></GameBoard>
                </div>
            </div>
        </GameContext.Provider>
    )
}
export default GamePage;

export interface GameContextInterface {
    // Connection
    changeConnection: Function,

    //Dominari Obj
    dominari: Dominari,

    // Game Metainformation
    instance: bigint,
    changeInstance: Function,

    //Nav Info
    nav: NavEnum,
    changeNav: Function,

    //Private Key
    privateKey: Keypair,
    changePrivateKey: Function,
}

export const enum NavEnum{
    Settings,
    GameBoard,
    Hand
}