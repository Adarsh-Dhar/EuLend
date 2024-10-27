// /scripts/oracle.js
const { SigningArchwayClient } = require("@archwayhq/arch3.js");
const axios = require('axios');

async function fetchAndUpdatePrice(token) {
    try {
        // Validate the token parameter
        if (!token || typeof token !== 'string') {
            throw new Error('Token parameter must be a non-empty string');
        }

        // Fetch the price from CoinGecko
        const res = await axios.get(
            `https://api.coingecko.com/api/v3/simple/price?ids=${token}&vs_currencies=usd`,
            {
                headers: {
                    accept: 'application/json',
                    'x-cg-api-key': 'CG-Ko72T3vZvSHxDtrs4TNakp3x' // Replace with your actual API key
                }
            }
        );

        // Check if the token price was returned
        if (!res.data[token]) {
            throw new Error(`Price not found for token: ${token}`);
        }

        const price = res.data[token].usd;
        console.log(`Fetched ${token} price: $${price}`);

        // Connect to the Archway RPC endpoint
        const rpcEndpoint = "https://rpc.constantine.archway.io";
        const client = await SigningArchwayClient.connect(rpcEndpoint);

        // Prepare the message to update the price in the contract
        const msg = {
            update_price: {
                token: token,
                price: Math.round(price * 1_000_000).toString() // Convert price to an appropriate format
            }
        };

        console.log('Executing contract with message:', msg);

        // Send the message to your contract (this part will require your contract's address and the user's wallet)
        // For example:
        // const contractAddress = "<your_contract_address>"; // Replace with your contract address
        // const walletAddress = "<user_wallet_address>"; // Replace with the user's wallet address
        // const response = await client.execute(walletAddress, contractAddress, msg);
        // console.log('Contract executed with response:', response);

        return msg; // Return the prepared message (or the response if you execute the contract)
    } catch (error) {
        console.error('Detailed error:', {
            message: error.message,
            stack: error.stack,
            response: error.response?.data
        });
        throw error;
    }
}




module.exports = { fetchAndUpdatePrice };