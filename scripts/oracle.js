const { SigningArchwayClient } = require("@archwayhq/arch3.js");
const axios = require('axios');

async function fetchAndUpdatePrice(token, contractAddress) {
    try {
        // Validate input
        if (!token || typeof token !== 'string') {
            throw new Error('Token parameter must be a non-empty string');
        }

        // Fetch price from CoinGecko
        const res = await axios.get(
            `https://api.coingecko.com/api/v3/simple/price?ids=${token}&vs_currencies=usd`,
            {
                headers: {
                    accept: 'application/json',
                    'x-cg-api-key': 'CG-Ko72T3vZvSHxDtrs4TNakp3x'
                }
            }
        );

        // Check if token exists in response
        if (!res.data[token]) {
            throw new Error(`Price not found for token: ${token}`);
        }

        const price = res.data[token].usd;
        console.log(`Fetched ${token} price: $${price}`);

        // Connect to the RPC endpoint
        const rpcEndpoint = "https://rpc.constantine.archway.io";
        const client = await SigningArchwayClient.connect(rpcEndpoint);

        // Prepare message for contract
        const msg = {
            update_price: {
                token: token,
                // Convert price to microdollars (6 decimal places)
                price: Math.round(price * 1_000_000).toString()
            }
        };

        console.log('Executing contract with message:', msg);

        // Execute contract (ensure the contract allows interaction without wallet)
        // const result = await client.execute(
        //     "your-address-here",   // Contract owner or authorized address to execute the transaction
        //     contractAddress,
        //     msg,
        //     "auto",
        //     "", // memo
        //     {
        //         amount: [], // no funds sent
        //         gas: "500000" // increased gas limit
        //     }
        // );

        // console.log(`Updated ${token} price! Hash: ${result.transactionHash}`);
        // return result;

    } catch (error) {
        console.error('Detailed error:', {
            message: error.message,
            stack: error.stack,
            response: error.response?.data
        });
        throw error;
    }
}

// Example usage:
async function updateTokenPrice() {
    try {
        await fetchAndUpdatePrice("ethereum", "archway1..."); // Pass the token and contract address
    } catch (error) {
        console.error('Failed to update price:', error);
    }
}

// Run the function
updateTokenPrice();
