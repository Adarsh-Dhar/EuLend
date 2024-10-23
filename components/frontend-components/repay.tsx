// components/Repay.tsx
import React from 'react';

const Repay: React.FC = () => {
  return (
    <div className="p-4">
      <h1 className="text-2xl font-bold mb-4">Repay</h1>
      <form>
        <input
          type="number"
          placeholder="Amount"
          className="p-2 border rounded-md w-full mb-4"
        />
        <button type="submit" className="bg-blue-500 text-white py-2 px-4 rounded-md">
          Repay
        </button>
      </form>
    </div>
  );
};

export default Repay;
