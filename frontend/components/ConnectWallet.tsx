"use client"
import { useState } from "react";

// Define minimal types we need for Keplr wallet
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
  const [address, setAddress] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isConnecting, setIsConnecting] = useState(false);

  const connectWallet = async () => {
    setIsConnecting(true);
    setError(null);
    
    try {
      const chainId = "constantine-3";
      
      if (!window.keplr) {
        throw new Error("Keplr wallet not found! Please install Keplr extension.");
      }
      
      // Enable Keplr for the chain
      await window.keplr.enable(chainId);
      
      // Get the offline signer
      const offlineSigner = window.keplr.getOfflineSigner(chainId);

      // Get user accounts
      const accounts = await offlineSigner.getAccounts();
      if (!accounts || accounts.length === 0) {
        throw new Error("No accounts found");
      }

      // Set the user's address in the state
      setAddress(accounts[0].address);
      
    } catch (err: any) {
      setError(err.message || "Failed to connect to wallet");
    } finally {
      setIsConnecting(false);
    }
  };

  return (
    <div className="flex flex-col items-center gap-4">
      <button
        onClick={connectWallet}
        disabled={isConnecting}
        className="relative inline-flex items-center justify-center p-0.5 mb-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-500 to-pink-500 group-hover:from-purple-500 group-hover:to-pink-500 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-purple-200 dark:focus:ring-purple-800"
      >
        <span className="relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0">
          {isConnecting ? "Connecting..." : address ? "Connected" : "Connect Wallet"}
        </span>
      </button>

      {error && (
        <p className="text-red-500 text-sm">{error}</p>
      )}
      
      {address && (
        <p className="text-sm text-gray-600 dark:text-gray-300">
          Connected address: {address}
        </p>
      )}
    </div>
  );
};

export default WalletConnect;