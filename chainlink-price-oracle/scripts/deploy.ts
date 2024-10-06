const hre = require("hardhat");

async function main() {
    const Oracle = await hre.ethers.getContractFactory("DataConsumerV3");
    const oracle = await Oracle.deploy();
    await oracle.waitForDeployment();
    console.log("Price feeds is deployed to:", oracle.target);


    const AxelarGateway = "0xe432150cce91c13a887f7D836923d5597adD8E31"
    const gasReceiver = "0xbE406F0189A0B4cf3A05C286473D23791Dd44Cc6"
    const chainName = "ethereum-sepolia"
    const oracleContract = oracle.target


    const Axelar = await hre.ethers.getContractFactory("Axelar");
    const axelar = await Axelar.deploy(AxelarGateway, gasReceiver, chainName, oracleContract);
    await axelar.waitForDeployment();
    console.log("axlear send receive is deployed to:", axelar.target);


    
}

main()
    .then(() => process.exit(0))
    .catch(error => {
        console.error(error);
        process.exit(1);
    });