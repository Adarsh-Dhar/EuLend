import React, { useState } from 'react';

// Array of tokens with name, short code, and logo
const tokens = [
  { name: "Bitcoin", shortCode: "BTC", logo: "https://cryptologos.cc/logos/bitcoin-btc-logo.png" },
  { name: "Ethereum", shortCode: "ETH", logo: "https://cryptologos.cc/logos/ethereum-eth-logo.png" },
  { name: "Tether", shortCode: "USDT", logo: "https://cryptologos.cc/logos/tether-usdt-logo.png" },
  { name: "Dai", shortCode: "DAI", logo: "https://cryptologos.cc/logos/multi-collateral-dai-dai-logo.png" },
  { name: "Solana", shortCode: "SOL", logo: "https://cryptologos.cc/logos/solana-sol-logo.png" },
  { name: "Polygon", shortCode: "MATIC", logo: "https://cryptologos.cc/logos/polygon-matic-logo.png" },
];

const SelectToken: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false); // State to toggle dropdown
  const [selectedToken, setSelectedToken] = useState<string | null>(null); // Selected token

  const toggleDropdown = () => {
    setIsOpen(!isOpen); // Toggle dropdown open/close
  };

  const handleSelectToken = (token: string) => {
    setSelectedToken(token); // Set selected token
    setIsOpen(false); // Close the dropdown after selection
  };

  return (
    <div className="relative inline-block text-left">
      <div>
        <button
          onClick={toggleDropdown}
          className="inline-flex justify-center w-full rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none"
        >
          {selectedToken ? `Selected: ${selectedToken}` : "Select Token"}
          <svg
            className="-mr-1 ml-2 h-5 w-5"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 20 20"
            fill="currentColor"
            aria-hidden="true"
          >
            <path
              fillRule="evenodd"
              d="M10 3a1 1 0 01.707.293l5 5a1 1 0 01-1.414 1.414L10 5.414 5.707 9.707A1 1 0 014.293 8.293l5-5A1 1 0 0110 3z"
              clipRule="evenodd"
            />
          </svg>
        </button>
      </div>

      {isOpen && (
        <div className="origin-top-right absolute right-0 mt-2 w-56 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 focus:outline-none z-10">
          <div className="py-1">
            {tokens.map((token) => (
              <button
                key={token.shortCode}
                onClick={() => handleSelectToken(`${token.name} (${token.shortCode})`)}
                className="flex items-center w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 hover:text-gray-900"
              >
                <img src={token.logo} alt={token.shortCode} className="h-5 w-5 mr-2" />
                {token.name} ({token.shortCode})
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default SelectToken;
