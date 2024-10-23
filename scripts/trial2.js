const { SigningArchwayClient } = require('@archwayhq/arch3.js');
const { DirectSecp256k1HdWallet } = require("@cosmjs/proto-signing");
const Long = require("long");
const dotenv = require("dotenv");

dotenv.config();

console.log(process.env.MNEMONIC);

async function main() {
    const network = {
        chainId: 'archway-1',
        endpoint: 'https://rpc.mainnet.archway.io',
        prefix: 'archway',
    };

    const mnemonic = process.env.MNEMONIC;

    // Initialize wallet using mnemonic
    const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, { prefix: network.prefix });
    const accounts = await wallet.getAccounts();
    const accountAddress = accounts[0].address;
    const destinationAddress = process.env.COSMOS_ADDRESS;

    // Initialize the signing client with the wallet
    const signingClient = await SigningArchwayClient.connectWithSigner(network.endpoint, wallet);

    // Construct IBC transfer message
    const msgIBCTransfer = {
        typeUrl: "/ibc.applications.transfer.v1.MsgTransfer",
        value: {
            sourcePort: 'transfer',
            sourceChannel: 'channel-0', // channel of the bridge
            token: {
                denom: 'aarch',
                amount: '1000000000000000000' // 1 aarch in smallest unit
            },
            sender: accountAddress,
            receiver: destinationAddress,
            // Timeout is in nanoseconds, using current time + 10 minutes
            timeoutTimestamp: Long.fromNumber(Date.now() + 600_000).multiply(1_000_000),
        },
    };

    // Broadcast the transaction
    const broadcastResult = await signingClient.signAndBroadcast(
        accountAddress,
        [msgIBCTransfer],
        'auto',
        'IBC Transfer' // optional memo
    );

    // Check if the transaction succeeded or failed
    if (broadcastResult.code !== undefined && broadcastResult.code !== 0) {
        console.log("Transaction failed:", broadcastResult.log || broadcastResult.rawLog);
    } else {
        console.log("Transaction successful:", broadcastResult.transactionHash);
    }
}

// Run the main function
main();
