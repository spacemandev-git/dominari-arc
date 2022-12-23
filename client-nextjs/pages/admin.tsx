import type { NextPage } from 'next';
import dynamic from 'next/dynamic';
import Head from 'next/head';
import React from 'react';
import Header from '../components/header';

/**
 * Connect Wallet
 * Initialize Registry & AB
 * Registr Blueprints
 */


const AdminPage: NextPage = (props) => {
    return (
        <div>
            <Header {...props}></Header>
            <div>
                <label>Registry Contract</label>
                <input type="text"></input>
                <button>Initalize Contract</button>
                <button>Register Components</button>
            </div>
            <div>
                <label>Action Bundle Contract</label>
                <input type="text"></input>
                <button>Initalize</button>
                <button>Register Blueprints</button>
            </div>
        </div>
    );
}

export default AdminPage;