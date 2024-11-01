"use client"
import React, { useState } from 'react';

import { createAccount } from '../interaction';


const CreateAccount = () => {
  const [isProcessing, setIsProcessing] = useState(false);
 

  const handleCreateAccount = async () => {
    try {
      setIsProcessing(true);
      // Here you would add your backend call to create account
      await createAccount();
    } catch (error) {
      console.error("Account creation failed:", error);
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div className="flex justify-center items-center min-h-screen bg-gray-50 p-4">
      <div className="w-full max-w-md bg-white rounded-xl shadow-lg p-6 space-y-6">
        <h2 className="text-2xl font-semibold text-gray-800">Create Account</h2>
        
        <div className="space-y-4">
          <button
            onClick={handleCreateAccount}
            disabled={isProcessing}
            className={`w-full py-3 px-4 rounded-lg text-white font-medium transition-colors
              ${!isProcessing 
                ? 'bg-blue-600 hover:bg-blue-700' 
                : 'bg-gray-400 cursor-not-allowed'}`}
          >
            {isProcessing ? (
              <span className="flex items-center justify-center">
                Processing...
              </span>
            ) : (
              'Create Account'
            )}
          </button>
        </div>
      </div>
    </div>
  );
};

export default CreateAccount;
