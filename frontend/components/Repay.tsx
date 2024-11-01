"use client"
import React, { useState } from 'react';
import ChainList from "./ChainList";
import { repay } from '../interaction';
import { useStore } from '../states/state';

type RepayConfirmationProps = {
  amount: number;
  onClose: () => void;
  onRepay: () => void;
};

const RepayConfirmation: React.FC<RepayConfirmationProps> = ({ amount, onClose, onRepay }) => (
  <div className="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center z-50">
    <div className="bg-white rounded-xl p-6 w-full max-w-sm mx-4">
      <div className="text-center mb-6">
        <h3 className="text-lg font-semibold mb-4">Withdraw Token Selection</h3>
        <p className="text-gray-600 mb-4">
          Repaying: <span className="font-bold">{amount} USDC</span>
        </p>
        
        <div className="mb-4">
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Select Token to Receive
          </label>
          <ChainList />
        </div>
      </div>
      
      <div className="flex justify-end space-x-3">
        <button
          onClick={onClose}
          className="px-4 py-2 rounded-lg border border-gray-300 text-gray-700 hover:bg-gray-50"
        >
          Cancel
        </button>
        <button
          onClick={onRepay}
          className="px-4 py-2 rounded-lg bg-blue-600 text-white hover:bg-blue-700"
        >
          Confirm Repay
        </button>
      </div>
    </div>
  </div>
);

const TransferConfirmation = ({ onClose }: { onClose: () => void }) => (
  <div className="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center z-50">
    <div className="bg-white rounded-xl p-6 w-full max-w-sm mx-4">
      <div className="text-center mb-6">
        <h3 className="text-lg font-semibold mb-2">Transfer Complete</h3>
        <p className="text-gray-600">
          Your tokens have been transferred to your account
        </p>
      </div>
      
      <div className="flex justify-end">
        <button
          onClick={onClose}
          className="px-4 py-2 rounded-lg bg-blue-600 text-white hover:bg-blue-700"
        >
          Close
        </button>
      </div>
    </div>
  </div>
);

const Repay = () => {
  const [usdcAmount, setUsdcAmount] = useState("");
  const [showRepayConfirmation, setShowRepayConfirmation] = useState(false);
  const [showTransferConfirmation, setShowTransferConfirmation] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  const tokenSelected = useStore((state) => state.tokenSelected);

  const handleInitialRepay = () => {
    setShowRepayConfirmation(true);
  };

  const handleFinalRepay = async () => {
    try {
      setIsProcessing(true);
      // Here you would add your backend call to process the repay and token transfer
      // await processRepay(usdcAmount, selectedToken);
      setShowRepayConfirmation(false);
      setShowTransferConfirmation(true);
      
      repay(tokenSelected, usdcAmount)
    } catch (error) {
      console.error("Repay failed:", error);
    } finally {
      setIsProcessing(false);
    }
  };

  const handleClose = () => {
    setShowTransferConfirmation(false);
    setUsdcAmount("");
    setShowRepayConfirmation(false);
  };

  return (
    <div className="flex justify-center items-center min-h-screen bg-gray-50 p-4">
      <div className="w-full max-w-md bg-white rounded-xl shadow-lg p-6 space-y-6">
        <h2 className="text-2xl font-semibold text-gray-800">Repay</h2>
        
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Enter USDC Amount to Repay
            </label>
            <input
              type="number"
              placeholder="0.00"
              value={usdcAmount}
              onChange={(e) => setUsdcAmount(e.target.value)}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none"
            />
          </div>

          <button
            onClick={handleInitialRepay}
            disabled={!usdcAmount || isProcessing}
            className={`w-full py-3 px-4 rounded-lg text-white font-medium transition-colors
              ${usdcAmount && !isProcessing
                ? 'bg-blue-600 hover:bg-blue-700' 
                : 'bg-gray-400 cursor-not-allowed'}`}
          >
            {isProcessing ? 'Processing...' : 'Repay'}
          </button>
        </div>
      </div>

      {showRepayConfirmation && (
        <RepayConfirmation 
          amount={Number(usdcAmount)}
          onClose={() => setShowRepayConfirmation(false)}
          onRepay={handleFinalRepay}
        />
      )}

      {showTransferConfirmation && (
        <TransferConfirmation 
          onClose={handleClose}
        />
      )}
    </div>
  );
};

export default Repay;