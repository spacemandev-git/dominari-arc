
// Fetch Helius Metadata in TS
// Setup private queue with local switchboard + helius
// 

import fetch from 'node-fetch';

const degenApeMint = "6vtWwjMdCoMyeoqk2iWyvAFvEaHJLZFeLoS6MSKA7Tdf";
const smbMint = "CWsXEP62dCcqkhFxDpy5wTLSLskp7ieRtDcRJhoS3hvs";
const heliusKey = "1b21b073-a222-47bb-8628-564145e58f4e"

fetchNFTMetadata();

async function fetchNFTMetadata(){
    const url = `https://api.helius.xyz/v0/tokens/metadata?api-key=${heliusKey}`
    const data = await fetch(url, {
        method: 'post',
        body: JSON.stringify({
            mintAccounts: [smbMint]
        })
    });

    console.log(JSON.stringify(await data.json(), null, 2));
}

