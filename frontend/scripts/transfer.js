const dotenv = require("dotenv"); 
const { SigningArchwayClient } = require('@archwayhq/arch3.js'); 
const { DirectSecp256k1HdWallet } = require("@cosmjs/proto-signing"); 
const Long = require("long");
dotenv.config();
const {fetchAndUpdatePrice} = require('./oracle.js');


async function main() {
    const oracleMsg = await fetchAndUpdatePrice("ethereum");
    const oracleMemo = oracleMsg.update_price.price.toString();
    console.log("oracle msg", oracleMemo);
    const network = {  
        chainId: 'constantine-3',  
        endpoint: 'https://rpc.constantine.archway.io',  
        prefix: 'archway',
    };

    const mnemonic = "test barely daughter cotton echo brain penalty price hood bargain venture mix ostrich obscure supreme fee roast expire arch fiscal govern term fantasy mesh";
    const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, { prefix: network.prefix });
    const accounts = await wallet.getAccounts();

    const accountAddress = accounts[0].address;
    const destinationAddress = "archway1t00mqwm46hmvkgj4ysyh0ykyjln3yw2fvt92wj";

    console.log("Mnemonic:", mnemonic); // Add this
console.log("Cosmos Address:", destinationAddress); // Add this

    const signingClient = await SigningArchwayClient.connectWithSigner(network.endpoint, wallet);

    const msgIBCTransfer = {  
        typeUrl: "/ibc.applications.transfer.v1.MsgTransfer",  
        value: {    
            sourcePort: 'transfer',    
            sourceChannel: 'channel-225', // updated to active channel for Osmosis    
            token: {      
                denom: 'aconst',     
                amount: '10000000000'    
            },   
            sender: accountAddress,    
            receiver: destinationAddress,    
            // Timeout is in nanoseconds, you can also just send Long.UZERO for default timeout    
            timeoutTimestamp: Long.fromNumber(Date.now() + 600_000).multiply(1_000_000), 
            oracleMsg : oracleMsg.update_price.price 
         },
    };
    


        const broadcastResult = await signingClient.signAndBroadcast( 
             accountAddress,  
             [msgIBCTransfer],  
             'auto',  
             oracleMemo, // optional memo
        );

        if (broadcastResult.code !== undefined && broadcastResult.code !== 0) { 
             console.log("Transaction failed:", broadcastResult.log || broadcastResult.rawLog);
            } else {  
                console.log("Transaction successful:", broadcastResult.transactionHash);
            };
}

main()