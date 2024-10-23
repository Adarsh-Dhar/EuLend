// components/Withdraw.tsx
import React from 'react';

const Withdraw: React.FC = () => {
  return (
    <div className="p-4">
      <h1 className="text-2xl font-bold mb-4">Withdraw</h1>
      <form>
        <input
          type="number"
          placeholder="Amount"
          className="p-2 border rounded-md w-full mb-4"
        />
        <button type="submit" className="bg-blue-500 text-white py-2 px-4 rounded-md">
          Withdraw
        </button>
      </form>
    </div>
  );
};

export default Withdraw;
