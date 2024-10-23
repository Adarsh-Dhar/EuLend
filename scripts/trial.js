const { SigningArchwayClient } = require('@archwayhq/arch3.js');
const { DirectSecp256k1HdWallet } = require("@cosmjs/proto-signing");
const mnemonic = "test barely daughter cotton echo brain penalty price hood bargain venture mix ostrich obscure supreme fee roast expire arch fiscal govern term fantasy mesh"

async function trial() {
    const network = {  
        chainId: 'constantine-3',  
        endpoint: 'https://rpc.constantine.archway.io', 
        prefix: 'constantine',
    };

    const mnemonic = process.env.MNEMONIC;
    const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, { prefix: network.prefix });
    const accounts = await wallet.getAccounts();
    const accountAddress = accounts[0].address;
    const destinationAddress = process.env.COSMOS_ADDRESS;

    const signingClient = await SigningArchwayClient.connectWithSigner(network.endpoint, wallet);

    const msgIBCTransfer = {  
        typeUrl: "/ibc.applications.transfer.v1.MsgTransfer",  
        value: {    
            sourcePort: 'transfer',    
            sourceChannel: 'channel-0', // channel of the bridge    
    token: {      
        denom: 'aconst', 
        amount: '1000000000000000000'    
    },    
    sender: accountAddress,    
    receiver: destinationAddress,    
    // Timeout is in nanoseconds, you can also just send Long.UZERO for default timeout    
    timeoutTimestamp: Long.fromNumber(Date.now() + 600_000).multiply(1_000_000), 
 },
};

const broadcastResult = await signingClient.signAndBroadcast(
    accountAddress, 
    [msgIBCTransfer],
    "auto",
    "IBC Transfer",
    );


    if (broadcastResult.code !== undefined && broadcastResult.code !== 0) {  
        console.log("Transaction failed:", broadcastResult.log || broadcastResult.rawLog);
    } else {  
        console.log("Transaction successful:", broadcastResult.transactionHash);
    }
}

trial()