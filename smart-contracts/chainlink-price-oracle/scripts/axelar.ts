require("dotenv").config();
const { ethers } = require("ethers");

const GANACHE_RPC_URL = "http://127.0.0.1:8545";


// ABI for your Axelar contract
const AxelarABI =[
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "gateway_",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "gasReceiver_",
          "type": "address"
        },
        {
          "internalType": "string",
          "name": "chainName_",
          "type": "string"
        },
        {
          "internalType": "address",
          "name": "dataConsumerAddress",
          "type": "address"
        }
      ],
      "stateMutability": "nonpayable",
      "type": "constructor"
    },
    {
      "inputs": [],
      "name": "InvalidAddress",
      "type": "error"
    },
    {
      "inputs": [],
      "name": "NotApprovedByGateway",
      "type": "error"
    },
    {
      "inputs": [],
      "name": "chainName",
      "outputs": [
        {
          "internalType": "string",
          "name": "",
          "type": "string"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "dataConsumer",
      "outputs": [
        {
          "internalType": "contract DataConsumerV3",
          "name": "",
          "type": "address"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "bytes32",
          "name": "commandId",
          "type": "bytes32"
        },
        {
          "internalType": "string",
          "name": "sourceChain",
          "type": "string"
        },
        {
          "internalType": "string",
          "name": "sourceAddress",
          "type": "string"
        },
        {
          "internalType": "bytes",
          "name": "payload",
          "type": "bytes"
        }
      ],
      "name": "execute",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "bytes32",
          "name": "commandId",
          "type": "bytes32"
        },
        {
          "internalType": "string",
          "name": "sourceChain",
          "type": "string"
        },
        {
          "internalType": "string",
          "name": "sourceAddress",
          "type": "string"
        },
        {
          "internalType": "bytes",
          "name": "payload",
          "type": "bytes"
        },
        {
          "internalType": "string",
          "name": "tokenSymbol",
          "type": "string"
        },
        {
          "internalType": "uint256",
          "name": "amount",
          "type": "uint256"
        }
      ],
      "name": "executeWithToken",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "gasService",
      "outputs": [
        {
          "internalType": "contract IAxelarGasService",
          "name": "",
          "type": "address"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "gateway",
      "outputs": [
        {
          "internalType": "contract IAxelarGateway",
          "name": "",
          "type": "address"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "string",
          "name": "destinationChain",
          "type": "string"
        },
        {
          "internalType": "string",
          "name": "destinationAddress",
          "type": "string"
        }
      ],
      "name": "sendBTCtoUSD",
      "outputs": [],
      "stateMutability": "payable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "string",
          "name": "destinationChain",
          "type": "string"
        },
        {
          "internalType": "string",
          "name": "destinationAddress",
          "type": "string"
        }
      ],
      "name": "sendETHtoUSD",
      "outputs": [],
      "stateMutability": "payable",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "storedMessage",
      "outputs": [
        {
          "internalType": "string",
          "name": "sender",
          "type": "string"
        },
        {
          "internalType": "string",
          "name": "message",
          "type": "string"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    }
  ];

// Address of the deployed Axelar contract on Sepolia
const axelarcontractAddress = "0x5fEdc9087B0B5Bac2710BDb289AD93B7966a8550";

// Infura RPC URL for Sepolia and private key
const INFURA_RPC_URL_1 = "https://sepolia.infura.io/v3/dc10a4b3a75349aab5abdf2314cbad35";
const PRIVATE_KEY_1 = "56a96c57ac92b2d35cd073ce2a81415ab31fbb7181df929069a125384c83b7fd";

// Function to initialize Ethereum provider and signer
const init = async () => {
    // Create a provider using Infura
    const provider = new ethers.providers.JsonRpcProvider(GANACHE_RPC_URL);



    // Create a signer from the private key
    const signer = new ethers.Wallet(PRIVATE_KEY_1, provider);

    console.log(`Signer address: ${signer.address}`);
    console.log(`Provider URL: ${provider.connection.url}`);

    return { signer, provider };
};

// Function to interact with the Axelar contract
const getAxelarContract = async () => {
    const { signer } = await init();
    
    // Initialize the contract with the signer and ABI
    const axelarContract = new ethers.Contract(axelarcontractAddress, AxelarABI, signer);

    console.log("Axelar Contract initialized:", axelarContract);
    
    return axelarContract;
};

// Function to send BTC/USD price to Archway
const sendBTCtoUSD = async () => {
    const axelarContract = await getAxelarContract();
    
    try {
        const destinationChain = "archway"; // Name of the destination chain
        const destinationAddress = "ARCHWAY_CONTRACT_ADDRESS"; // Archway contract address

        // Call the sendBTCtoUSD function from the contract
        const tx = await axelarContract.sendBTCtoUSD(destinationChain, destinationAddress, {
            value: ethers.utils.parseEther("0.01") // Gas payment
        });
        
        console.log("Sent BTC/USD price to Archway:", tx.hash);
    } catch (error) {
        console.error("Error sending BTC/USD price:", error);
    }
};

// Function to send ETH/USD price to Archway
const sendETHtoUSD = async () => {
    const axelarContract = await getAxelarContract();
    
    try {
        const destinationChain = "archway";
        const destinationAddress = "ARCHWAY_CONTRACT_ADDRESS";

        // Call the sendETHtoUSD function from the contract
        const tx = await axelarContract.sendETHtoUSD(destinationChain, destinationAddress, {
            value: ethers.utils.parseEther("0.01")
        });
        
        console.log("Sent ETH/USD price to Archway:", tx.hash);
    } catch (error) {
        console.error("Error sending ETH/USD price:", error);
    }
};

// Send the prices
sendBTCtoUSD();
sendETHtoUSD();
