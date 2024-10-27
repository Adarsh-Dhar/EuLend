import * as React from 'react';
 
import { ChainProvider } from '@cosmos-kit/react';
import { chains, assets } from 'chain-registry';
import { wallets } from '@cosmos-kit/keplr';
import Navbar from './Navbar';
 


 
function ChainList() {
    return (
        <ChainProvider 
        chains = {chains} 
        assetLists={assets}
        wallets={wallets} 
        >
            <Navbar />

        </ChainProvider>
    );
}

export default ChainList;