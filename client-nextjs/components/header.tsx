import dynamic from 'next/dynamic';
import { FC, useRef } from "react";
import { useConnection, useWallet } from '@solana/wallet-adapter-react';

const WalletDisconnectButtonDynamic = dynamic(
    async () => (await import('@solana/wallet-adapter-react-ui')).WalletDisconnectButton,
    { ssr: false }
);
const WalletMultiButtonDynamic = dynamic(
    async () => (await import('@solana/wallet-adapter-react-ui')).WalletMultiButton,
    { ssr: false }
);

const Header: FC = (props:any) => {
    const {connection} = useConnection();
    const rpcRef = useRef<HTMLInputElement>(null);

    const changeConnection = (evt:any) => {
        props.setEndpoint(rpcRef.current?.value)
    }

    return(
        <div>
            <div className="fixed left-0 self-center text-xl ml-4 mt-4">
                <label>Dominari</label>
            </div>
            <div className="fixed right-0 flex mt-4 mr-4">
                <label
                    className="self-center w-64"
                >
                RPC: {connection.rpcEndpoint}
                </label>
                <input 
                    ref={rpcRef}
                    className="self-center w-64" 
                    defaultValue={connection.rpcEndpoint}
                ></input>
                <button
                    className="ml-2 bg-purple-800 w-24"
                    onClick={changeConnection}
                >Set RPC</button>
                <WalletMultiButtonDynamic />
                <WalletDisconnectButtonDynamic />
            </div>
        </div>
    )
}

export default Header;