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
        </div>
    );
}

export default AdminPage;