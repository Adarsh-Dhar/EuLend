"use client"
import React, { useState } from 'react';
import ChainList from "./ChainList";

const BorrowConfirmation = ({ amount, onClose }: { amount: number, onClose: () => void }) => (
  <div className="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center z-50">
    <div className="bg-white rounded-xl p-6 w-full max-w-sm mx-4">
      <div className="text-center mb-6">
        <h3 className="text-lg font-semibold mb-2">Borrow Confirmation</h3>
        <p className="text-gray-600">
          You will receive: <span className="font-bold">{amount || '--'} USDC</span>
        </p>
      </div>
      
      <div className="flex justify-end space-x-3">
      <button
          onClick={onClose}
          className="px-4 py-2 green-500 rounded-lg border border-gray-300 text-gray-700 hover:bg-gray-50"
        >
          Accept
        </button>
        <button
          onClick={onClose}
          className="px-4 py-2 red-500 rounded-lg border border-gray-300 text-gray-700 hover:bg-gray-50"
        >
          Reject
        </button>
      </div>
    </div>
  </div>
);

const Borrow = () => {
  const [collateralAmount, setCollateralAmount] = useState("");
  const [showConfirmation, setShowConfirmation] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);

  const handleBorrow = async () => {
    try {
      setIsProcessing(true);
      // Here you would add your backend call to process the borrow
      // await borrowUSDC(collateralAmount);
      setShowConfirmation(true);
    } catch (error) {
      console.error("Borrow failed:", error);
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div className="flex justify-center items-center min-h-screen bg-gray-50 p-4">
      <div className="w-full max-w-md bg-white rounded-xl shadow-lg p-6 space-y-6">
        <h2 className="text-2xl font-semibold text-gray-800">Borrow</h2>
        
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Select Collateral Token
            </label>
            <ChainList />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Enter Collateral Amount
            </label>
            <input
              type="number"
              placeholder="0.00"
              value={collateralAmount}
              onChange={(e) => setCollateralAmount(e.target.value)}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none"
            />
          </div>

          <button
            onClick={handleBorrow}
            disabled={!collateralAmount || isProcessing}
            className={`w-full py-3 px-4 rounded-lg text-white font-medium transition-colors
              ${collateralAmount && !isProcessing
                ? 'bg-blue-600 hover:bg-blue-700' 
                : 'bg-gray-400 cursor-not-allowed'}`}
          >
            {isProcessing ? (
              <span className="flex items-center justify-center">
                Processing...
              </span>
            ) : (
              'Confirm Borrow'
            )}
          </button>
        </div>
      </div>

      {showConfirmation && (
        <BorrowConfirmation 
          amount = {Number(collateralAmount)}
          onClose={() => {
            setShowConfirmation(false);
            setCollateralAmount("");
          }}
        />
      )}
    </div>
  );
};

export default Borrow;