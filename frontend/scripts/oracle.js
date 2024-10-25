const { SigningArchwayClient } = require("@archwayhq/arch3.js");
const axios = require('axios');

async function fetchAndUpdatePrice(token) {
    try {
        if (!token || typeof token !== 'string') {
            throw new Error('Token parameter must be a non-empty string');
        }

        const res = await axios.get(
            `https://api.coingecko.com/api/v3/simple/price?ids=${token}&vs_currencies=usd`,
            {
                headers: {
                    accept: 'application/json',
                    'x-cg-api-key': 'CG-Ko72T3vZvSHxDtrs4TNakp3x'
                }
            }
        );

        if (!res.data[token]) {
            throw new Error(`Price not found for token: ${token}`);
        }

        const price = res.data[token].usd;
        console.log(`Fetched ${token} price: $${price}`);

        const rpcEndpoint = "https://rpc.constantine.archway.io";
        const client = await SigningArchwayClient.connect(rpcEndpoint);

        const msg = {
            update_price: {
                token: token,
                price: Math.round(price * 1_000_000).toString()
            }
        };

        console.log('Executing contract with message:', msg);

        return msg

    } catch (error) {
        console.error('Detailed error:', {
            message: error.message,
            stack: error.stack,
            response: error.response?.data
        });
        throw error;
    }
}

// Export using CommonJS
module.exports = { fetchAndUpdatePrice };
