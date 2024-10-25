import React from 'react';

const CreateAccount: React.FC = () => {
  return (
    <div className="fixed inset-0 flex items-center justify-center bg-gray-200">
      <div className="p-6 bg-white rounded-md shadow-lg w-96">
        <h1 className="text-2xl font-bold mb-4 text-center">Create an Account</h1>
        <form>
          {/* Uncomment when SelectToken component is available */}
          {/* <SelectToken /> */}
          {/* Input box for amount */}
          <input
            type="number"
            placeholder="Amount"
            className="p-2 border rounded-md w-full mb-4 mt-4"
          />
          {/* Submit button */}
          <button
            type="submit"
            className="bg-blue-500 text-white py-2 px-4 rounded-md w-full hover:bg-blue-600 transition duration-200"
          >
            Deposit
          </button>
        </form>
      </div>
    </div>
  );
};

export default CreateAccount;