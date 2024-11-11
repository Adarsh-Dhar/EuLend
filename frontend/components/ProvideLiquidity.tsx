"use client"
import React, { useState } from 'react';
import { ibcTrial, provideLiquidity } from '../interaction';

const ProvideLiquidity = () => {
  const [isProcessing, setIsProcessing] = useState(false);
  const [amount, setAmount] = useState(0);

  const handleProvideLiquidity = async () => {
    try {
      setIsProcessing(true);
      await ibcTrial();
      await provideLiquidity( amount);

    } catch (error) {
      console.error("Failed to provide liquidity:", error);
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div className="flex justify-center items-center min-h-screen bg-gray-50 p-4">
      <div className="w-full max-w-md bg-white rounded-xl shadow-lg p-6 space-y-6">
        <h2 className="text-2xl font-semibold text-gray-800">Provide Liquidity</h2>
        
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Enter Amount (USDC)
            </label>
            <textarea
              placeholder="0.00"
              value={amount}
              onChange={(e) => setAmount(Number(e.target.value))}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none"
            />
          </div>

          <button
            onClick={handleProvideLiquidity}
            disabled={!amount || isProcessing}
            className={`w-full py-3 px-4 rounded-lg text-white font-medium transition-colors
              ${amount && !isProcessing
                ? 'bg-blue-600 hover:bg-blue-700' 
                : 'bg-gray-400 cursor-not-allowed'}`}
          >
            {isProcessing ? (
              <span className="flex items-center justify-center">
                Processing...
              </span>
            ) : (
              'Provide Liquidity'
            )}
          </button>
        </div>
      </div>
    </div>
  );
};

export default ProvideLiquidity;
