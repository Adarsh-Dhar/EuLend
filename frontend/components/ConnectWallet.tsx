"use client"
import { useState } from "react";
import { useStore } from "../states/state";
import { SigningArchwayClient } from "@archwayhq/arch3.js";
const contractAddress = 'archway1z5krv4lgwh883fv840gupe4mtwjfnmfw0d86l9yrrlj7s4rj9hdsp8c87s';

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

const WalletConnect = () => {

  const [error, setError] = useState<string | null>(null);
  const [isConnecting, setIsConnecting] = useState(false);
  const userAddress = useStore((state) => state.address);
  const updateAddress = useStore((state) => state.changeAddress);
  const offlineSignerState = useStore((state) => state.offlineSigner);
  const updateOfflineSigner = useStore((state) => state.changeOfflineSigner);

  // Function to truncate address for display
  const truncateAddress = (addr: string) => {
    if (!addr) return "";
    const start = addr.slice(0, 8);
    const end = addr.slice(-6);
    return `${start}...${end}`;
  };

  const connectWallet = async () => {
    setIsConnecting(true);
    setError(null);
    try {
      const chainId = "constantine-3";
      if (!window.keplr) {
        throw new Error("Keplr wallet not found! Please install Keplr extension.");
      }
      
      await window.keplr.enable(chainId);
      const offlineSigner = window.keplr.getOfflineSigner(chainId);
      updateOfflineSigner(offlineSigner);
      console.log("offlineSigner", offlineSigner);
      const accounts = await offlineSigner.getAccounts();
      console.log("state of offlineSigner", await offlineSignerState);
      
      if (!accounts || accounts.length === 0) {
        throw new Error("No accounts found");
      }
      
      updateAddress(accounts[0].address);
      console.log("user address", userAddress);
    } catch (err: any) {
      setError(err.message || "Failed to connect to wallet");
    } finally {
      setIsConnecting(false);
    }
  };

  const createAccount = async () => {
    try {
      if (!(await offlineSignerState) || !userAddress) {
        throw new Error("Please connect wallet first");
      }

      const cwClient = await SigningArchwayClient.connectWithSigner(
        "https://rpc.constantine.archway.io",
        offlineSignerState
      );

      console.log("userAddress", userAddress);
      console.log("cwClient", cwClient);
      const accounts = await offlineSignerState.getAccounts();

      const msg = { create_account: {} };
      const response = await cwClient.execute(
          accounts[0].address,
          contractAddress,
          msg,
          "auto"
      );
      console.log('Create Account Response:', response);
      return response;
    } catch (error) {
      console.error('Error creating account:', error);
      throw error;
    }
  }

  const borrow = async (borrowAmount: string, collateralDenom: string, collateralAmount: string, funds: any[]) => {
    try {
      if (!(await offlineSignerState) || !userAddress) {
        throw new Error("Please connect wallet first");
      }

      const cwClient = await SigningArchwayClient.connectWithSigner(
        "https://rpc.constantine.archway.io",
        offlineSignerState
      );

      const accounts = await offlineSignerState.getAccounts();

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
        funds
      );
      console.log('Borrow Response:', response);
      return response;
    } catch (error) {
      console.error('Error borrowing:', error);
      throw error;
    }
  }

  const repay = async (withdrawDenom: string, withdrawAmount: string, repaymentFunds: any[]) => {
    try {
      if (!(await offlineSignerState) || !userAddress) {
        throw new Error("Please connect wallet first");
      }

      const cwClient = await SigningArchwayClient.connectWithSigner(
        "https://rpc.constantine.archway.io",
        offlineSignerState
      );

      const accounts = await offlineSignerState.getAccounts();

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
        repaymentFunds
      );
      console.log('Repay Response:', response);
      return response;
    } catch (error) {
      console.error('Error repaying:', error);
      throw error;
    }
  }

  return (

      <div className="flex flex-col items-center gap-4">
      <button
        onClick={connectWallet}
        disabled={isConnecting}
        className="relative inline-flex items-center justify-center p-0.5 mb-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-500 to-pink-500 group-hover:from-purple-500 group-hover:to-pink-500 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-purple-200 dark:focus:ring-purple-800"
      >
        <span className="relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0">
          {isConnecting ? (
            "Connecting..."
          ) : userAddress ? (
            truncateAddress(userAddress)
          ) : (
            "Connect Wallet"
          )}
        </span>
      </button>
      <button
        onClick={createAccount}
        disabled={isConnecting}
        className="relative inline-flex items-center justify-center p-0.5 mb-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-500 to-pink-500 group-hover:from-purple-500 group-hover:to-pink-500 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-purple-200 dark:focus:ring-purple-800"
      >
        <span className="relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0">
          {isConnecting ? (
            "Connecting..."
          ) : userAddress ? (
            truncateAddress(userAddress)
          ) : (
            "Create Account"
          )}
        </span>
      </button>
      
      {error && (
        <p className="text-red-500 text-sm">{error}</p>
      )}
    </div>

    
  );
};

export default WalletConnect;