import React from 'react';
import SelectToken from './SelectToken'; // Import the SelectToken component

const Repay: React.FC = () => {
  const maxRepayable = 1000; // Example: max Repayable amount (this can be dynamic)

  return (
    <div className="p-4">
      <h1 className="text-2xl font-bold mb-4">Repay</h1>
      <form>
        {/* SelectToken component */}
        <SelectToken />

        {/* Display the max Repayable amount */}
        <p className="text-gray-700 mb-2">
          Max Repayable: <span className="font-bold">{maxRepayable}</span>
        </p>

        {/* Input box for amount */}
        <input
          type="number"
          placeholder="Amount"
          className="p-2 border rounded-md w-full mb-4 mt-2"
        />

        {/* Submit button */}
        <button type="submit" className="bg-blue-500 text-white py-2 px-4 rounded-md w-full">
          Repay
        </button>
      </form>
    </div>
  );
};

export default Repay;
