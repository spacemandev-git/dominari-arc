import { ComponentIndex, Dominari, Registry } from "../dominari-sdk/dominari-sdk-nodejs";
import glob from "glob";
import toml from 'toml';
import * as fs from 'fs';
import { ixPack, ixWasmToJs, randomU64 } from './util';
import { Connection, Keypair, TransactionMessage, VersionedTransaction } from '@solana/web3.js';
import { bs58 } from '@coral-xyz/anchor/dist/cjs/utils/bytes';

const CONNSTRING = "http://localhost:8899";
const connection = new Connection(CONNSTRING, "finalized");
main();

/**
 * Called with arguments
 * ts-node admin.ts privateKey registryID dominariID
 */
async function main(){
    await prepareFiles();
    console.log("Files prepared!");
    const privateKey = Keypair.fromSecretKey(bs58.decode(process.argv[2]));
    const registryID = process.argv[3];
    const dominariID = process.argv[4];

    await initRegistry(privateKey,registryID,dominariID);
    console.log("Registry Initialized");
    await initDominari(privateKey,registryID,dominariID);
    console.log("Dominari Initialized");
}


/**
 * Compiles Blueprints into single JSON
 */
async function prepareFiles(){
    let allBlueprints = {};
    glob("../client-nextjs/public/blueprints/**/*.toml", (err, files) => {
        for(let file of files){
            let name = file.split('/').pop().split('.toml')[0];
            let blueprint = toml.parse((fs.readFileSync(file).toString()));
            allBlueprints[name] = blueprint;
        }
        fs.writeFileSync('../client-nextjs/public/blueprints/blueprints.json', JSON.stringify(allBlueprints, null, 4));
    });
}

/**
 * 
 */
async function initRegistry(privateKey: Keypair, registryID: string, dominariID:string) {
    const registry = new Registry(registryID);
    let instructions = [];

    // Initialize Tx
    instructions.push(
        ixWasmToJs(registry.initialize(privateKey.publicKey.toString()))
    );

    // Register Components
    const componentIndex = fs.readFileSync('./ComponentIndex.txt').toString().split("\n");
    for(let component of componentIndex){
        instructions.push(
            ixWasmToJs(registry.register_component(privateKey.publicKey.toString(), component))
        );
    };

    // Register AB
    instructions.push(
        ixWasmToJs(registry.register_action_bundle(privateKey.publicKey.toString(), dominariID))
    );

    // Register Components w/ AB
    for(let component of componentIndex){
        instructions.push(
            ixWasmToJs(registry.add_components_for_action_bundle(privateKey.publicKey.toString(), dominariID, [component]))
        );
    };
    const ixGroups = await ixPack(instructions);
    for(let ixGroup of ixGroups){
        const msg = new TransactionMessage({
            payerKey: privateKey.publicKey,
            recentBlockhash: (await connection.getLatestBlockhash()).blockhash,
            instructions: ixGroup
        }).compileToLegacyMessage();
        const tx = new VersionedTransaction(msg);
        tx.sign([privateKey]);
        const sig = await connection.sendRawTransaction(tx.serialize());
        await connection.confirmTransaction(sig);
        console.log("TX Submitted: ", sig);
    };
}

async function initDominari(privateKey: Keypair, registryID:string, dominariID:string){
    // Create ComponentIndex for Relevant Keys Arg
    const componentIndex = fs.readFileSync('./ComponentIndex.txt').toString().split("\n");
    let CI = new ComponentIndex(registryID);
    for(let url of componentIndex){
        CI.insert_component_url(url);
    }

    // Initialize (consume CI)
    const dominari = new Dominari(dominariID);
    const initDomIx = ixWasmToJs(dominari.initalize(privateKey.publicKey.toString(), CI));
    const tx = new VersionedTransaction(new TransactionMessage({
        payerKey: privateKey.publicKey,
        recentBlockhash: (await connection.getLatestBlockhash()).blockhash,
        instructions: [initDomIx]
    }).compileToLegacyMessage());
    tx.sign([privateKey]);
    const sig = await connection.sendTransaction(tx)
    await connection.confirmTransaction(sig);

    // Register Blueprints
    const blueprintJson = JSON.parse(fs.readFileSync("../client-nextjs/public/blueprints/blueprints.json").toString());
    let blueprintIxs = [];
    for(let blueprintName of Object.keys(blueprintJson)){
        let ix = ixWasmToJs(dominari.register_blueprint(
            privateKey.publicKey.toString(),
            blueprintName,
            CI,
            blueprintJson[blueprintName]
        ));
        blueprintIxs.push(ix);
    };
    const ixG = await ixPack(blueprintIxs);
    for(let group of ixG){
        const msg = new TransactionMessage({
            payerKey: privateKey.publicKey,
            recentBlockhash: (await connection.getLatestBlockhash()).blockhash,
            instructions: group
        }).compileToLegacyMessage();
        const tx = new VersionedTransaction(msg);
        tx.sign([privateKey]);
        const sig = await connection.sendTransaction(tx);
        await connection.confirmTransaction(sig);
        console.log("TX Submitted: ", sig); 
    };

}

/* Registry
1. Deploy 3 Programs
2. Initalize Registry (program_id, payer)
3. Register Components with Registry (schema, payer)
4. Register Action Bundle (ab_program_id, payer)
5. Register AB w/ Components (vec of all component pubkeys)

Dominari
6. Generate Game Instance -> Add instance to AB Registration & Create New Instance
*/

/* Dominari
Initialization
0. Create a ComponentIndex and fill with URLs
1. Initalize (Consumes reference to ComponentIndex)
    -> Create AB Signer
2. Register Blueprints

Game Loop
1. Create Game
    -> Create Instance Index
    -> Initalize Map
    -> Initalize Tiles
2. Toggle Game State
3. Game Actions
*/