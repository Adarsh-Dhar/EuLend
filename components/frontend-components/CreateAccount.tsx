import React from 'react';
import SelectToken from './SelectToken'; // Import the SelectToken component

const CreateAccount: React.FC = () => {
  return (
    <div className="p-4">
      <h1 className="text-2xl font-bold mb-4">Deposit</h1>
      <form>
        {/* SelectToken component */}
        <SelectToken />

        {/* Input box for amount */}
        <input
          type="number"
          placeholder="Amount"
          className="p-2 border rounded-md w-full mb-4 mt-4"
        />

        {/* Submit button */}
        <button type="submit" className="bg-blue-500 text-white py-2 px-4 rounded-md w-full">
          Deposit
        </button>
      </form>
    </div>
  );
};

export default CreateAccount;
