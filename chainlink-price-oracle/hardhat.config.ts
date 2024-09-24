
require("@nomiclabs/hardhat-ethers");

const fs = require('fs');
// const infuraId = fs.readFileSync(".infuraid").toString().trim() || "";

module.exports = {
  defaultNetwork: "hardhat",
  networks: {
    hardhat:{
      chainId : 11155111
    },
    sepolia: {
      url: `https://sepolia.infura.io/v3/dc10a4b3a75349aab5abdf2314cbad35`, // Replace with the actual RPC endpoint for the Sepolia testnet
      accounts: [ `56a96c57ac92b2d35cd073ce2a81415ab31fbb7181df929069a125384c83b7fd` ]
    }
  },
  solidity: {
    version: "0.8.20",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  }
};