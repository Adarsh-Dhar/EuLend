import { SigningArchwayClient } from "@archwayhq/arch3.js";
import { useStore } from "../states/state";

const contractAddress = 'archway1auuygyvu7nmhy99g96lvavgj88335fe6cwgrf5dmgnj3jyj302kqcyzhnh';



interface KeplrWindow {
  keplr?: {
    enable: (chainId: string) => Promise<void>;
    getOfflineSigner: (chainId: string) => {
      getAccounts: () => Promise<{ address: string; }[]>;
    };
  };
}

declare global {
  interface Window extends KeplrWindow {}
}



export const createAccount = async () => {
  const offlineSigner = useStore.getState().offlineSigner;
  const userAddress = useStore.getState().address;
  console.log("offlineSigner", offlineSigner);
  console.log("userAddress", userAddress);
  try {
    
    if (!offlineSigner || !userAddress) {
      throw new Error("Please connect wallet first");
    }

    const cwClient = await SigningArchwayClient.connectWithSigner(
      "https://rpc.constantine.archway.io",
      offlineSigner
    );

    console.log("userAddress", userAddress);
    console.log("cwClient", cwClient);
    const accounts = await offlineSigner.getAccounts();

    const msg = { create_account: {} };
    const response = await cwClient.execute(
        accounts[0].address,
        contractAddress,
        msg,
        "auto",

    );
    console.log('Create Account Response:', response);
    return response;
  } catch (error) {
    console.error('Error creating account:', error);
    throw error;
  }
}

export const deleteAccount = async () => {
  const offlineSigner = useStore.getState().offlineSigner;
  const userAddress = useStore.getState().address;
  console.log("offlineSigner", offlineSigner);
  console.log("userAddress", userAddress);
  try {
    
    if (!offlineSigner || !userAddress) {
      throw new Error("Please connect wallet first");
    }

    const cwClient = await SigningArchwayClient.connectWithSigner(
      "https://rpc.constantine.archway.io",
      offlineSigner
    );

    console.log("userAddress", userAddress);
    console.log("cwClient", cwClient);
    const accounts = await offlineSigner.getAccounts();

    const msg = { delete_account: {} };
    const response = await cwClient.execute(
        accounts[0].address,
        contractAddress,
        msg,
        "auto"
    );
    console.log('Delete Account Response:', response);
    return response;
  } catch (error) {
    console.error('Error deleting account:', error);
    throw error;
  }
}

export const borrow = async (
  borrowAmount: number,
  collateralDenom: string,
  collateralAmount: number,
) => {
  const offlineSigner = useStore.getState().offlineSigner;
  const userAddress = useStore.getState().address;
  try {
    
    if (!offlineSigner || !userAddress) {
      throw new Error("Please connect wallet first");
    }

    const cwClient = await SigningArchwayClient.connectWithSigner(
      "https://rpc.constantine.archway.io",
      offlineSigner
    );

    const accounts = await offlineSigner.getAccounts();

    const msg = {
      borrow: {
        borrow_amount: borrowAmount,
        collateral_denom: collateralDenom,
        collateral_amount: collateralAmount
      }
    };

    const response = await cwClient.execute(
      accounts[0].address,
      contractAddress,
      msg,
      "auto",
      undefined,
    
    );
    console.log('Borrow Response:', response);
    return response;
  } catch (error) {
    console.error('Error borrowing:', error);
    throw error;
  }
}

export const repay = async (
  withdrawDenom: string,
  withdrawAmount: string,
  
) => {
  const offlineSigner = useStore.getState().offlineSigner;
  const userAddress = useStore.getState().address;
  try {
   
    if (!offlineSigner || !userAddress) {
      throw new Error("Please connect wallet first");
    }

    const cwClient = await SigningArchwayClient.connectWithSigner(
      "https://rpc.constantine.archway.io",
      offlineSigner
    );

    const accounts = await offlineSigner.getAccounts();

    const msg = {
      repay: {
        withdraw_denom: withdrawDenom,
        withdraw_amount: withdrawAmount
      }
    };

    const response = await cwClient.execute(
      accounts[0].address,
      contractAddress,
      msg,
      "auto",
      undefined,
    
    );
    console.log('Repay Response:', response);
    return response;
  } catch (error) {
    console.error('Error repaying:', error);
    throw error;
  }
}

export const provideLiquidity = async (amount: number) => {
  const offlineSigner = useStore.getState().offlineSigner;
  const userAddress = "noble1t00mqwm46hmvkgj4ysyh0ykyjln3yw2f3rvxut";
  console.log("offlineSigner", offlineSigner);
  console.log("userAddress", userAddress);
  try {
    
    if (!offlineSigner || !userAddress) {
      throw new Error("Please connect wallet first");
    }

    const cwClient = await SigningArchwayClient.connectWithSigner(
      "https://rpc.constantine.archway.io",
      offlineSigner
    );

    console.log("userAddress", userAddress);
    console.log("cwClient", cwClient);
    const accounts = await offlineSigner.getAccounts();
    const funds = [
      {
        denom: "uusdc",
        amount: amount.toString() // amount should be string
      }
    ];
      
    

    const msg = { provide_liquidity: {} };
    const response = await cwClient.execute(
        accounts[0].address,
        contractAddress,
        msg,
        "auto",
        "provide liquidity",
        funds
    );
    console.log('Delete Account Response:', response);
    return response;
  } catch (error) {
    console.error('Error deleting account:', error);
    throw error;
  }
}