import axios from 'axios';
import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { DirectSecp256k1HdWallet } from '@cosmjs/proto-signing';
import { makeSignDoc, AminoSignResponse } from '@cosmjs/amino';
import { sha256 } from '@cosmjs/crypto';
import { coins } from '@cosmjs/proto-signing';

interface PriceData {
    coin_id: string;
    price: string;
    timestamp: number;
    signature: string;
    feeder_public_key: string;
}

interface StdFee {
    amount: {
        amount: string;
        denom: string;
    }[];
    gas: string;
}

class PriceFeeder {
    private client: SigningCosmWasmClient | undefined;
    private wallet: DirectSecp256k1HdWallet | undefined;
    private publicKey: string | undefined;
    private chainId: string;
    private sequence: number;
    private accountNumber: number;
    
    constructor(
        private readonly contractAddress: string,
        private readonly mnemonic: string,
        private readonly rpcEndpoint: string,
        private readonly apiKey: string,
        chainId: string
    ) {
        this.chainId = chainId;
        this.sequence = 0;
        this.accountNumber = 0;
    }

    async initialize() {
        this.wallet = await DirectSecp256k1HdWallet.fromMnemonic(
            this.mnemonic,
            { prefix: 'cosmos' } // Adjust prefix based on your chain
        );
        
        this.client = await SigningCosmWasmClient.connectWithSigner(
            this.rpcEndpoint,
            // @ts-ignore
            this.wallet
        );
        
        // Get account details
        const [account] = await this.wallet.getAccounts();
        this.publicKey = Buffer.from(account.pubkey).toString('base64');
        
        // Get account number and sequence
        const { accountNumber, sequence } = await this.client.getSequence(account.address);
        this.accountNumber = accountNumber;
        this.sequence = sequence;
    }

    private async fetchPrices(coinIds: string[]): Promise<Record<string, any>> {
        try {
            const response = await axios.get(
                'https://pro-api.coingecko.com/api/v3/simple/price',
                {
                    params: {
                        ids: coinIds.join(','),
                        vs_currencies: 'usd',
                        include_last_updated_at: true,
                    },
                    headers: {
                        'X-Cg-Pro-Api-Key': this.apiKey,
                    },
                }
            );
            return response.data;
        } catch (error) {
            console.error('Error fetching prices:', error);
            throw error;
        }
    }

    private async signPriceData(
        priceUpdates: Omit<PriceData, 'signature' | 'feeder_public_key'>[],
        address: string
    ): Promise<AminoSignResponse> {
        // Construct the message for contract execution
        const msg = {
            type: 'wasm/MsgExecuteContract',
            value: {
                sender: address,
                contract: this.contractAddress,
                msg: {
                    update_prices: {
                        prices: priceUpdates
                    }
                },
                funds: []
            }
        };

        // Construct the fee
        const fee: StdFee = {
            amount: coins(5000, 'uatom'), // Adjust based on your chain's denomination
            gas: '200000',
        };

        // Create the sign doc with all required parameters
        const signDoc = makeSignDoc(
            [msg],                     // msgs: readonly AminoMsg[]
            fee,                       // fee: StdFee
            this.chainId,              // chainId: string
            '',                        // memo: string (empty in this case)
            this.accountNumber.toString(), // accountNumber: string
            this.sequence.toString()    // sequence: string
        );

        // Sign the document
        if(!this.wallet) {
            throw new Error('Wallet not initialized');
        }
        const [account] = await this.wallet.getAccounts();
        const signer = (this.wallet as any).signerFunction;
        return await signer(account.address, signDoc);
    }

    async updatePrices() {
        try {
            if(!this.wallet) {
                throw new Error('Wallet not initialized');
            }
            const [account] = await this.wallet.getAccounts();
            const coinIds = ['bitcoin', 'ethereum', 'atom'];
            const prices = await this.fetchPrices(coinIds);
            
            const priceUpdates: Omit<PriceData, 'signature' | 'feeder_public_key'>[] = [];
            
            for (const coinId of coinIds) {
                const priceData = prices[coinId];
                if (priceData) {
                    priceUpdates.push({
                        coin_id: coinId,
                        price: Math.floor(priceData.usd * 100_000_000).toString(),
                        timestamp: priceData.last_updated_at,
                    });
                }
            }
            
            if (priceUpdates.length > 0) {
                // Sign the price updates
                const signResponse = await this.signPriceData(priceUpdates, account.address);
//@ts-ignore
                const signatureString = this.createSignatureString(signature);
                
                // Construct the final price updates with signatures
                const signedPriceUpdates = priceUpdates.map(update => ({
                    ...update,
                    signature: signatureString,
                    feeder_public_key: this.publicKey,
                }));
                
                // Execute the contract
                const msg = {
                    update_prices: {
                        prices: signedPriceUpdates,
                    },
                };
                if(!this.client) {
                    throw new Error('Client not initialized');
                }
                
                const tx = await this.client.execute(
                    account.address,
                    this.contractAddress,
                    msg,
                    'auto',
                    'Update prices from CoinGecko'
                );
                
                console.log('Price update transaction hash:', tx.transactionHash);
                
                // Update sequence for next transaction
                this.sequence += 1;
                
                // Verify transaction success
                const txResponse = await this.client.getTx(tx.transactionHash);
                if (txResponse?.code !== 0) {
                    throw new Error(`Transaction failed: ${txResponse?.rawLog}`);
                }
                
                console.log(`Successfully updated ${signedPriceUpdates.length} prices`);
            }
        } catch (error) {
            console.error('Error in price update cycle:', error);
            // Implement retry logic or alerting system here
        }
    }

    async start() {
        await this.initialize();
        await this.updatePrices();
        setInterval(() => this.updatePrices(), 30000);
    }
}

// Usage
const priceFeeder = new PriceFeeder(
    'contract-address',
    'your-mnemonic',
    'your-rpc-endpoint',
    'your-coingecko-api-key',
    'cosmoshub-4'  // or your specific chain ID
);

priceFeeder.start().catch(console.error);