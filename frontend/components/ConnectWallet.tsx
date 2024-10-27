"use client"

import { useState, useEffect } from "react";
import { SigningStargateClient } from "@cosmjs/stargate";
import { Window as KeplrWindow } from "@keplr-wallet/types";

declare global {
  interface Window extends KeplrWindow {}
}

const ConnectWallet = () => {
  const [address, setAddress] = useState<string | null>(null);
  const [cosmJS, setCosmJS] = useState<SigningStargateClient | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const connectWallet = async () => {
      try {
        const chainId = "constantine-3";

        // Enable Keplr for the specified chain
        await window.keplr?.enable(chainId);

        // Get the offline signer
        const offlineSigner = window.getOfflineSigner
          ? window.getOfflineSigner(chainId)
          : window.keplr?.getOfflineSigner!(chainId);

        // Get user accounts
        const accounts = await offlineSigner?.getAccounts();

        if (!accounts || accounts.length === 0) {
          throw new Error("No accounts found");
        }

        // Set the user's address in the state
        setAddress(accounts[0].address);

        // Initialize the SigningStargateClient with the signer
        const client = await SigningStargateClient.connectWithSigner(
          "https://rpc.constantine-3.archway.tech", // Replace with the actual RPC URL
          offlineSigner!
        );

        // Store the client in state to use it in other interactions
        setCosmJS(client);
      } catch (err: any) {
        setError(err.message || "Failed to connect to wallet");
      }
    };

    connectWallet();
  }, []);

  return (
    <div className="wallet-connection">
      {error && <p className="error-text">Error: {error}</p>}
      {address ? (
        <p className="connected-text">Connected to wallet: {address}</p>
      ) : (
        <p>Connecting to wallet...</p>
      )}
    </div>
  );
};

export default ConnectWallet;
