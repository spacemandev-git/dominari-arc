import type { NextApiRequest, NextApiResponse } from 'next'

export default async function handler(req:NextApiRequest, res:NextApiResponse) {
    const body = JSON.parse(req.body);
    console.log("Mint recieved on server: ", body.mint);

    const metadata = await (await fetch(`https://api.helius.xyz/v0/tokens/metadata?api-key=${process.env.HELIUS_API_KEY}`, {
        method: 'post',
        body: JSON.stringify({
            'mintAccounts': [body.mint]
        })
    })).json();
    console.log("Metadata: ", JSON.stringify(metadata, null, 2));
    // metadata[0]
        //.mint
        //.onChainData
        //.offCHainData



    res.status(200).json({});
}


export interface DominariBlueprint {
    metadata: {
        name: string,
        entity_type: "Unit"
    },
    offchain_metdata: {
        image: string,
        link: string
    },
    damage: {
        min_damage: number,
        max_damage: number,
        bonus_infantry: number,
        bonus_armor: number,
        bonus_aircraft: number,
        bonus_feature: number
    },
    health: {
        health: number
    },
    troop_class: {
        class: "Infantry" | "Armor" | "Air",
    },
    range: {
        movement: number,
        attack_range: number,
    },
    last_used: {
        last_used: 0,
        recovery: number, //slots
    },
    value: {
        value: number
    }
}

export interface HeliusMetadata {
    mint: string,
    onChainData: OnChainData,
    offChainData: OffChainData
}

export interface OnChainData {
    collection:          Collection;
    collectionDetails:   null;
    data:                Data;
    editionNonce:        number;
    isMutable:           boolean;
    key:                 string;
    mint:                string;
    primarySaleHappened: boolean;
    tokenStandard:       null;
    updateAuthority:     string;
    uses:                null;
}

export interface Collection {
    key:      string;
    verified: boolean;
}

export interface Data {
    creators:             Creator[];
    name:                 string;
    sellerFeeBasisPoints: number;
    symbol:               string;
    uri:                  string;
}

export interface Creator {
    address:  string;
    share:    number;
    verified: boolean;
}

export interface OffChainData {
    attributes:           Attribute[];
    description:          string;
    image:                string;
    name:                 string;
    properties:           Properties;
    sellerFeeBasisPoints: number;
    symbol:               string;
}

export interface Attribute {
    traitType: string;
    value:     number | string;
}

export interface Properties {
    category: string;
    creators: Creator[];
    files:    File[];
}

export interface Creator {
    address: string;
    share:   number;
}

export interface File {
    type: string;
    uri:  string;
}
