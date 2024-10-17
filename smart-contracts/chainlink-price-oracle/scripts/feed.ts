

// ABI for the contract
const ABI = [
    {
      "inputs": [],
      "stateMutability": "nonpayable",
      "type": "constructor"
    },
    {
      "inputs": [],
      "name": "getBTCtoUSD",
      "outputs": [
        {
          "internalType": "int256",
          "name": "",
          "type": "int256"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "getETHtoUSD",
      "outputs": [
        {
          "internalType": "int256",
          "name": "",
          "type": "int256"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    }
  ];

// Address of the deployed contract on Sepolia
const contractAddress = "0xEd4dDEa35aB840F1F73e6163b876BA7729F172AA";

// Hardcoded Infura URL and private key for the signer
const INFURA_RPC_URL = "https://sepolia.infura.io/v3/dc10a4b3a75349aab5abdf2314cbad35";
const PRIVATE_KEY = "56a96c57ac92b2d35cd073ce2a81415ab31fbb7181df929069a125384c83b7fd";

// Function to initialize Ethereum provider and signer
const initEthereum = async () => {
    // Create a provider using Infura
    const provider = new ethers.providers.JsonRpcProvider(INFURA_RPC_URL);

    // Create a signer from the private key
    const signer = new ethers.Wallet(PRIVATE_KEY, provider);

    console.log(`Signer address: ${signer.address}`);
    console.log(`Provider URL: ${provider.connection.url}`);

    return { signer, provider };
};

// Function to interact with the smart contract
const getContract = async () => {
    const { signer, provider } = await initEthereum();
    
    // Initialize the contract with the provider and ABI
    const contract = new ethers.Contract(contractAddress, ABI, provider);

    console.log("Contract initialized:", contract);
    
    return contract;
};

// Function to call the contract's getChainlinkDataFeedLatestAnswer function
const getBTCtoUSDdata = async () => {
    const contract = await getContract();
    
    try {
        // Call the read-only function from the contract
        const latestAnswer = await contract.getBTCtoUSD();
        console.log("Latest Chainlink BTC/USD Price:", latestAnswer.toString());
    } catch (error) {
        console.error("Error fetching Chainlink data:", error);
    }
};

const getETHtoUSDdata = async () => {
    const contract = await getContract();
    
    try {
        // Call the read-only function from the contract
        const latestAnswer = await contract.getETHtoUSD();
        console.log("Latest Chainlink ETH/USD Price:", latestAnswer.toString());
    } catch (error) {
        console.error("Error fetching Chainlink data:", error);
    }
};

// Fetch the Chainlink BTC/USD price
getBTCtoUSDdata();
getETHtoUSDdata();
