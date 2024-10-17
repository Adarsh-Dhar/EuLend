require("@nomicfoundation/hardhat-toolbox");

module.exports = {
  solidity: "0.8.9",
  networks: {
    ganache: {
      url: "http://127.0.0.1:8545", // URL of the Ganache network
      accounts: [
        // Use the private keys provided by Ganache or copy the mnemonic to generate the keys
        "0xff30fd287c8ec1d078e888dfca3d3740f019ea646bfb50aefcbf3dd2ff745138",
        
      ],
    },
  },
};
