const hre = require("hardhat");

async function main() {
    const Oracle = await hre.ethers.getContractFactory("DataConsumerV3");
    const oracle = await Oracle.deploy();
    await oracle.deployed();
    console.log("Price feeds is deployed to:", oracle.address);
}

main()
    .then(() => process.exit(0))
    .catch(error => {
        console.error(error);
        process.exit(1);
    });