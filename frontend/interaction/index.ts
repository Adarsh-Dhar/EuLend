import { SigningArchwayClient } from "@archwayhq/arch3.js";
import { useStore } from "../states/state";
import Long from "long";
import {ibc,chains} from "chain-registry"
import { Height } from "cosmjs-types/ibc/core/client/v1/client";
import { useState } from "react";

const contractAddress = 'archway1auuygyvu7nmhy99g96lvavgj88335fe6cwgrf5dmgnj3jyj302kqcyzhnh';

const channels = ibc.filter(
  (channel) => channel.chain_1.chain_name === "archwaytestnet"
)


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
      const balance = await cwClient.getBalance(userAddress, "ausdc");
      console.log("Balance:", balance);
  
      const accounts = await offlineSigner.getAccounts();

      console.log("accounts", accounts);
      let coin = {
        denom : "ausdc",
        amount : amount
      }
      // Create IBC transfer message
      // const msgIBCTransfer = {
      //   userAddress, // sender address
      //   contractAddress, // recipient address
      //   coin, // transfer amount
      //   "transfer", // source port
      //   "channel-50", // source channel
      //   undefined, // timeout height
      //   undefined, // timeout timestamp 
      //   "auto", // fee
      //   "Coreum IBC Transfer" // memo
      // };

      const msgIBCTransfer = {
        typeUrl: "/ibc.applications.transfer.v1.MsgTransfer",
        value: {
          sender: userAddress,
          receiver: contractAddress,
          token: {
            denom: "uausdc",
            amount: amount
          },
          sourcePort: "transfer",
          sourceChannel: "channel-449",
          timeoutTimestamp: Long.fromNumber(Date.now() + 600_000).multiply(1_000_000),
        }
      }

      console.log("msgIBCTransfer", msgIBCTransfer);
  
      // Execute IBC transfer
      const ibcResponse = await cwClient.signAndBroadcast(
        accounts[0].address,
        [msgIBCTransfer],
        'auto',
        'IBC Transfer'
      );
  
      if (ibcResponse !== undefined && ibcResponse.code !== 0) {
        throw new Error(`IBC Transfer failed: ${ibcResponse}`);
      }
      
      console.log('IBC Transfer Response:', ibcResponse);
      return ibcResponse;
    } catch (error) {
      console.error('Error providing liquidity:', error);
      throw error;
    }
  }

