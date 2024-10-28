"use client"
import { useState } from "react";
import { useStore } from "../states/state";


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
      const accounts = await offlineSigner.getAccounts();
      
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
      
      {error && (
        <p className="text-red-500 text-sm">{error}</p>
      )}
    </div>

    
  );
};

export default WalletConnect;