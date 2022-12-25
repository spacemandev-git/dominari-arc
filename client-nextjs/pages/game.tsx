import type { NextPage } from 'next';
import React, {createContext, useState, useMemo} from 'react';
import {useEffectOnce} from 'usehooks-ts';
import { Keypair, PublicKey } from '@solana/web3.js';
import { DOMINARI_PROGRAM_ID, DUMMY_PRIV_KEY } from '../util/constants';
import Menu from '../components/menu';
import GameBoard from '../components/gameboard';
import Header from '../components/header';
import {encode, decode} from 'bs58';

/**
 * Create Game
 *  -> Takes a Config.toml
 * 
 * Join Game
 *  -> Takes an instance ID
 * 
 * Game Loop
 *  -> Hand for Units, 
 *  -> Tile Map for Board
 */

export const GameContext = createContext({} as GameContextInterface);

const GamePage: NextPage = (props) => {
    const [nav, changeNav] = useState(NavEnum.Settings);
    const [privateKey, changePrivateKey] = useState(Keypair.fromSecretKey(decode(DUMMY_PRIV_KEY)));

    useEffectOnce(() => {
        // bs58 encoded secret key
        const previousPrivateKey = localStorage.getItem('privateKey');
        console.log("Local Storage Key: ", previousPrivateKey);

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
            instance: 0,
            dominariProgramId: DOMINARI_PROGRAM_ID,
            nav,
            changeNav,
            privateKey,
            changePrivateKey
        }}>
            <Header></Header>
            <div className="grid grid-col-2">
                <Menu></Menu>
                <div className="ml-36 mt-48 mr-20 border-white border-2">
                    <GameBoard></GameBoard>
                </div>
            </div>
        </GameContext.Provider>
    )
}
export default GamePage;

export interface GameContextInterface {
    // Game Metainformation
    instance: number,
    dominariProgramId: PublicKey,

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