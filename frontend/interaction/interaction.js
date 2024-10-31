const { ArchwayClient } = require('@archwayhq/arch3.js');
const { SigningArchwayClient } = require('@archwayhq/arch3.js');
const { Coin } = require('@cosmjs/stargate');
const { SigningCosmWasmClient } = require('@cosmjs/cosmwasm-stargate');

// Contract address on Constantine testnet
const contractAddress = 'archway1z5krv4lgwh883fv840gupe4mtwjfnmfw0d86l9yrrlj7s4rj9hdsp8c87s';

// Dummy sender address for testing
const DUMMY_SENDER = "archway1t00mqwm46hmvkgj4ysyh0ykyjln3yw2fvt92wj";

class Contract {
    constructor(contractAddress) {
        this.contractAddress = contractAddress;
        this.cwClient = null;
        this.accounts = [{address: DUMMY_SENDER}];
    }

    async connectWallet() {
        try {
            // Using dummy client and account for testing
            this.cwClient = {
                execute: async (sender, contract, msg, fee, memo, funds) => {
                    console.log('Simulated execution:', {sender, contract, msg, funds});
                    return {
                        transactionHash: 'dummy_tx_hash',
                        logs: []
                    };
                },
                queryContractSmart: async (contract, msg) => {
                    console.log('Simulated query:', {contract, msg});
                    return {};
                }
            };

            console.log("Connected with dummy wallet", {
                client: this.cwClient,
                accounts: this.accounts
            });

            return this.accounts[0];

        } catch (error) {
            console.error("Error connecting dummy wallet:", error);
            throw error;
        }
    }

    async createAccount() {
        try {
            if (!this.cwClient || !this.accounts) {
                throw new Error("Please connect wallet first");
            }

            const msg = { create_account: {} };
            const response = await this.cwClient.execute(
                this.accounts[0].address,
                this.contractAddress,
                msg,
                "auto"
            );
            console.log('Create Account Response:', response);
            return response;
        } catch (error) {
            console.error('Error creating account:', error);
            throw error;
        }
    }

    async borrow(borrowAmount, collateralDenom, collateralAmount, funds) {
        try {
            if (!this.cwClient || !this.accounts) {
                throw new Error("Please connect wallet first");
            }

            const msg = {
                borrow: {
                    borrow_amount: borrowAmount,
                    collateral_denom: collateralDenom,
                    collateral_amount: collateralAmount
                }
            };

            const response = await this.cwClient.execute(
                this.accounts[0].address,
                this.contractAddress,
                msg,
                "auto",
                undefined,
                funds
            );
            console.log('Borrow Response:', response);
            return response;
        } catch (error) {
            console.error('Error borrowing:', error);
            throw error;
        }
    }

    async repay(withdrawDenom, withdrawAmount, repaymentFunds) {
        try {
            if (!this.cwClient || !this.accounts) {
                throw new Error("Please connect wallet first");
            }

            const msg = {
                repay: {
                    withdraw_denom: withdrawDenom,
                    withdraw_amount: withdrawAmount
                }
            };

            const response = await this.cwClient.execute(
                this.accounts[0].address,
                this.contractAddress,
                msg,
                "auto",
                undefined,
                repaymentFunds
            );
            console.log('Repay Response:', response);
            return response;
        } catch (error) {
            console.error('Error repaying:', error);
            throw error;
        }
    }

    async getAccount(address) {
        try {
            if (!this.cwClient) {
                throw new Error("Please connect wallet first");
            }

            const response = await this.cwClient.queryContractSmart(
                this.contractAddress,
                {
                    get_account: { address }
                }
            );
            console.log('Account Information:', response);
            return response;
        } catch (error) {
            console.error('Error querying account:', error);
            throw error;
        }
    }

    async getMaxWithdrawableAmount(tokenDenom) {
        try {
            if (!this.cwClient) {
                throw new Error("Please connect wallet first");
            }

            const response = await this.cwClient.queryContractSmart(
                this.contractAddress,
                {
                    max_withdrawable_amount: { token_denom: tokenDenom }
                }
            );
            console.log('Max Withdrawable Amount:', response);
            return response;
        } catch (error) {
            console.error('Error querying max withdrawable amount:', error);
            throw error;
        }
    }
}

// Example usage:
async function main() {
    try {
        const contract = new Contract(contractAddress);
        
        // Connect wallet
        const account = await contract.connectWallet();
        console.log("Connected account:", account.address);

        // Create lending account
        await contract.createAccount();

        // Borrow example
        const collateralFunds = [{
            denom: "atom",
            amount: "1000"
        }];
        await contract.borrow(
            "500",
            "atom", 
            "1000",
            collateralFunds
        );

        // Query account status
        await contract.getAccount(account.address);

        // Repay example
        const repayFunds = [{
            denom: "usdc",
            amount: "500"
        }];
        await contract.repay(
            "atom",
            "1000",
            repayFunds
        );

    } catch (error) {
        console.error("Error:", error);
    }
}

// Run example
main().catch(console.error);