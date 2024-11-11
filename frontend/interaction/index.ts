import { SigningArchwayClient } from "@archwayhq/arch3.js";
import { useStore } from "../states/state";
import Long from "long";
import {ibc,chains} from "chain-registry"
import { Height } from "cosmjs-types/ibc/core/client/v1/client";
import { useState } from "react";
import { Coin } from "@archwayhq/arch3.js";

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

      const coin : Coin = {
        denom: "ibc/88A17FBD38C49E984596ED9FA650B1683E08D89F3D276334F69E4F83989F9492'",
        amount: amount.toString()
      };
      

      const msgIBCTransfer = {
        typeUrl: "/ibc.applications.transfer.v1.MsgTransfer",
        value: {
          sender: userAddress,
          receiver: contractAddress,
          token: coin,
          sourcePort: "transfer",
          sourceChannel: "channel-91",
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


  export const ibcTrial = async () => {
    const channels = ibc.filter(
      (channel) => channel.chain_1.chain_name === "archwaytestnet"
    );
  
  
  
    const IBCEnableChains = channels.map((channel) => {
      const chain = chains.find(
        (chain) => chain.chain_name === channel.chain_2.chain_name
      );
  
      return {
        imgSrc: chain?.logo_URIs?.png,
        name: chain?.pretty_name,
        chainId: chain?.chain_id,
        base: chain?.bech32_prefix,
        channel: channel.channels[0].chain_1.channel_id,
        channel2: channel.channels[0].chain_2.channel_id
      };
  })

  console.log("IBCEnableChains", IBCEnableChains);
}


