import type { NextPage } from 'next';
import dynamic from 'next/dynamic';
import Head from 'next/head';
import React, {useRef} from 'react';
import Header from '../components/header';
import { Dominari, ComponentIndex, Registry } from 'dominari-sdk';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { ixWasmToJs, ixPack } from '../util/util';
import { Transaction } from '@solana/web3.js';
import toml from 'toml';

/**
 * Connect Wallet
 * Initialize Registry & AB
 * Registr Blueprints
 */


const AdminPage: NextPage = (props) => {
    const {connection} = useConnection();

    let registry: Registry;
    let dominari: Dominari;
    const registryIDref = useRef<HTMLInputElement>(null);
    const dominariIDref = useRef<HTMLInputElement>(null);
    const schemaFileRef = useRef<HTMLInputElement>(null);
    const blueprintFilesRef = useRef<HTMLInputElement>(null);

    const { publicKey, sendTransaction, signAllTransactions, signTransaction } = useWallet();

    const initRegistry = async () => {
        if(!publicKey){
            alert("Please sign in first!");
            return;
        }
        //Initalize IX
        registry = new Registry(registryIDref.current!.value);
        let instructions = []
        instructions.push(
            ixWasmToJs(registry.initialize(publicKey?.toBase58() as string))
        )
        //console.log("Post Registry Initialization", instructions);

        // Register Components
        const schemaFiles = schemaFileRef.current?.files;
        if(schemaFiles?.length != 1){
            alert("Please select a component schema list file!");
            return;
        }
        const componentSchemas = await schemaFiles[0].text()
        for(let url of componentSchemas.split('\n')) {
            console.log(`Pushing ${url} Registration IX`);
            instructions.push(
                ixWasmToJs(registry.register_component(publicKey?.toBase58(), url))
            )
        }
        //console.log("Post Component Registration IX: ", instructions);

        // Register Action Bundle
        instructions.push(
            ixWasmToJs(registry.register_action_bundle(publicKey?.toBase58(), dominariIDref.current?.value as string))
        )
        //console.log("Post AB Registration IX: ", instructions);

        // Register Components w/ Action Bundle
        for(let url of componentSchemas.split("\n")){
            instructions.push(
                ixWasmToJs(registry.add_components_for_action_bundle(publicKey?.toBase58(), dominariIDref.current?.value as string, [url]))
            )
        }
        console.log("Final list of instructions for initializing Registry: ", instructions);
        const ixGroups = await ixPack(instructions);
        let txGroup: Transaction[] = []; 
        const recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
        for (let ixG of ixGroups){
            const tx = new Transaction();
            tx.add(...ixG);
            tx.recentBlockhash = recentBlockhash;
            tx.feePayer = publicKey;
            txGroup.push(tx);
        }

        signAllTransactions ? await signAllTransactions(txGroup) : alert("Sign in first!");

        for(let tx of txGroup) {
            const sig = await connection.sendRawTransaction(tx.serialize(), {skipPreflight: true});
            console.log("Sent TX: ", sig);
            await connection.confirmTransaction(sig);
        }
    }

    const initDominari = async () => {
        const recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

        // Create Component Index (Relevant Keys)
        const schemaFiles = schemaFileRef.current?.files;
        if(schemaFiles?.length != 1){
            alert("Please select a component schema list file!");
            return;
        }
        const componentSchemas = await schemaFiles[0].text();
        let componentIndex = new ComponentIndex(dominariIDref.current?.value as string);
        for(let comp of componentSchemas.split("\n")){
            componentIndex.insert_component_url(comp);
        }

        // Initialize
        dominari = new Dominari(dominariIDref.current?.value as string);
        const initDomIx = ixWasmToJs(dominari.initalize(publicKey?.toBase58() as string, componentIndex));
        console.log("Init Dom Ix: ", initDomIx);
        const initDomTx = new Transaction();
        initDomTx.add(initDomIx);
        initDomTx.recentBlockhash = recentBlockhash;
        initDomTx.feePayer = publicKey!;
        
        // Register Blueprints
        const blueprintFiles = blueprintFilesRef.current?.files;
        console.log(blueprintFiles);
        let blueprintIxs = []
        for(let i=0; i<blueprintFiles?.length!; i++){
            let blueprintFile = blueprintFiles?.item(i);
            let ix = ixWasmToJs(dominari.register_blueprint(
                publicKey?.toBase58() as string, 
                blueprintFile?.name as string, 
                componentIndex,
                toml.parse(await blueprintFile?.text() as string)) 
            );
            blueprintIxs.push(ix);
        }
        console.log("Blueprint Ixs: ", blueprintIxs);
        
        const ixG = await ixPack(blueprintIxs);
        let blueprintTxs = [initDomTx];
        for (let g of ixG){
            const tx = new Transaction();
            tx.add(...g);
            tx.recentBlockhash = recentBlockhash;
            tx.feePayer = publicKey!;
            blueprintTxs.push(tx);
        }

        signAllTransactions ? await signAllTransactions(blueprintTxs) : alert("sign in");
        for(let tx of blueprintTxs) {
            const sig = await connection.sendRawTransaction(tx.serialize(), {skipPreflight: true});
            console.log("Sent TX: ", sig);
            await connection.confirmTransaction(sig);
        }
    }

    return (
        <div className="grid grid-rows-3">
            <Header {...props}></Header>
            <div className="relative flex flex-col gap-2 mx-36">
                <label className="text-2xl">Registry Initialization</label>
                <div className="flex flex-row gap-4">
                    <label>Registry ID</label>
                    <input type="text" className="w-[30rem]" defaultValue={"H5mieGWWK6qukHoNzbR6ysLxReeQC4JHZcNM6JkPQnm3"} ref={registryIDref}></input>
                </div>
                <div className="flex flex-row gap-4">
                    <label>Dominari ID</label>
                    <input type="text" className="w-[30rem]" defaultValue={"3YdayPtujByJ1g1DWEUh7vpg78gZL49FWyD5rDGyof9T"} ref={dominariIDref}></input>
                </div>
                <div className="flex flex-row gap-4">
                    <label>Schema URL Text File</label>
                    <input type="file" ref={schemaFileRef}></input> 
                </div>
                <button className="bg-teal-700 mr-72" onClick={initRegistry}>Init Registry</button>
            </div>
            <div className='relative flex flex-col gap-2 mx-36'>
                <label className="text-2xl">Dominari Initalization</label>
                <div className="flex flex-row gap-4">
                    <label>Blueprint Files</label>
                    <input type="file" ref={blueprintFilesRef} multiple></input> 
                </div>
                <button className="bg-teal-700 mr-72" onClick={initDominari}>Init Dominari</button>
            </div>
        </div>
    );
}

export default AdminPage;

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
    -> Add Instance to AB Registration
    -> Initalize Map
    -> Initalize Tiles
2. Toggle Game State
3. Game Actions
*/