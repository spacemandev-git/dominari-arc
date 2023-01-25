import { NextPage } from "next";
import { useEffect } from "react";

const NFTI: NextPage = () => {
    const submitMint = async (mint: string) => {
        const res = await fetch('/api/nfti', {
            method: "post",
            body: JSON.stringify({mint})
        });
        console.log(res.json());
    }


    useEffect(() => {
        
        submitMint('EdvGbLQbs2dNhvwt3AFcE5PHRWsrMZu9fzvxqVhmnQsg').then(() => {
            return;
        })
    }, [])

    return(
        <div className="p-40">
            <h1 className="text-3xl">NFT Importer</h1>
            <div>
                <label>Mint Address: </label><input></input><button>Submit Mint</button>
            </div>
        </div>
    )
}

export default NFTI;