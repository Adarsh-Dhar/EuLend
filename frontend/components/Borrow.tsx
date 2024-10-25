import React, { useState, useEffect } from 'react';
// Import the SelectToken component

const Borrow: React.FC = () => {
  const [collateralAmount, setCollateralAmount] = useState<number>(0);
  const [borrowAmount, setBorrowAmount] = useState<number>(0);

  // Mock fetch function to simulate getting the borrow amount from the backend
  useEffect(() => {
    // Simulate backend calculation based on collateral amount
    if (collateralAmount > 0) {
      // Assume the backend gives borrow amount as 50% of collateral amount
      setBorrowAmount(collateralAmount * 0.5);
    } else {
      setBorrowAmount(0);
    }
  }, [collateralAmount]);

  const handleCollateralChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setCollateralAmount(parseFloat(e.target.value) || 0);
  };

  return (
    <div className="p-4">
      <h1 className="text-2xl font-bold mb-4">Borrow</h1>

      <form>
        {/* Collateral Section */}
        <div className="mb-6">
          <h2 className="text-xl font-semibold mb-2">Collateral</h2>

          {/* SelectToken component for collateral */}


          {/* Input box for collateral amount */}
          <input
            type="number"
            placeholder="Collateral Amount"
            value={collateralAmount}
            onChange={handleCollateralChange}
            className="p-2 border rounded-md w-full mb-4 mt-2"
          />
        </div>

        {/* Borrow Section */}
        <div>
          <h2 className="text-xl font-semibold mb-2">Borrow</h2>

          {/* Display the calculated borrow amount from the backend */}
          <p className="text-gray-700 mb-2">
            Amount you can borrow: <span className="font-bold">{borrowAmount}</span>
          </p>
        </div>

        {/* Borrow button */}
        <button type="submit" className="bg-blue-500 text-white py-2 px-4 rounded-md w-full">
          Borrow
        </button>
      </form>
    </div>
  );
};

export default Borrow;
