import { useConnection } from '@solana/wallet-adapter-react';
import { Keypair } from '@solana/web3.js';
import {FC, useContext, useRef, useState, useEffect} from 'react';
import { GameContext } from '../pages/game';
import {encode, decode} from 'bs58';

const Settings: FC = () => {
    const {connection} = useConnection();
    let gameContext = useContext(GameContext);
    const privKeyRef = useRef<HTMLInputElement>(null);
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

    return (
        <div>
            <h1 className="text-3xl"> Settings </h1>
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
        </div>
    )
}

export default Settings;