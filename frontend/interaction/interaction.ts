import { ArchwayClient } from '@archwayhq/arch3.js';
const client = await ArchwayClient.connect('https://rpc.mainnet.archway.io');
const contractAddress = 'archway1z5krv4lgwh883fv840gupe4mtwjfnmfw0d86l9yrrlj7s4rj9hdsp8c87s';
const msg = {  
    get_count: {},
};
const { count } = await client.queryContractSmart(  
    contractAddress,  
    msg
    );
console.log("Counter: ", count);