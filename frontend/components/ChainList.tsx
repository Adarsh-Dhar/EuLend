"use client"
import React, { useState, useRef, useEffect } from 'react';
import { ChevronDown, Check } from 'lucide-react';

const ChainList = () => {
  const [open, setOpen] = useState(false);
  const [value, setValue] = useState("");
  const [searchTerm, setSearchTerm] = useState("");
  const dropdownRef = useRef(null);

  const tokens = [
    {
      value: "archway",
      label: "ARCH",
      chainId: "constantine-3",
      balance: "26471502",
      icon: "🏛️"
    },
    {
      value: "coreum",
      label: "CORE",
      chainId: "coreum-testnet-1",
      balance: "940757",
      icon: "💫"
    },
    {
      value: "injective",
      label: "INJ",
      chainId: "injective-888",
      balance: "849417981",
      icon: "📊"
    },
    {
      value: "neutron",
      label: "NTRN",
      chainId: "pion-1",
      balance: "0",
      icon: "⚛️"
    },
    {
      value: "nibiru",
      label: "NIBI",
      chainId: "nibiru-testnet-1",
      balance: "23950952004",
      icon: "🌌"
    },
    {
      value: "stargaze",
      label: "STARS",
      chainId: "elgafar-1",
      balance: "93464128623",
      icon: "⭐"
    }
  ];

  // Format balance to be more readable
  const formatBalance = (balance : any) => {
    const num = parseInt(balance);
    if (num >= 1000000) {
      return `${(num / 1000000).toFixed(2)}M`;
    } else if (num >= 1000) {
      return `${(num / 1000).toFixed(2)}K`;
    }
    return num.toString();
  };

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event : any) => {
        //@ts-ignore
      if (dropdownRef.current && !dropdownRef.current.contains(event.target)) {
        setOpen(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const filteredTokens = tokens.filter(token =>
    token.label.toLowerCase().includes(searchTerm.toLowerCase()) ||
    token.value.toLowerCase().includes(searchTerm.toLowerCase()) ||
    token.chainId.toLowerCase().includes(searchTerm.toLowerCase())
  );

  return (
    <div className="relative w-64" ref={dropdownRef}>
      <button
        onClick={() => setOpen(!open)}
        className="w-full flex items-center justify-between px-4 py-2 text-sm bg-white border rounded-lg hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        {value ? (
          <span className="flex items-center">
            <span className="mr-2">
              {tokens.find(token => token.value === value)?.icon}
            </span>
            <span className="font-medium">{tokens.find(token => token.value === value)?.label}</span>
            <span className="text-gray-500 ml-2">
              ({formatBalance(tokens.find(token => token.value === value)?.balance || "0")})
            </span>
          </span>
        ) : (
          "Select chain..."
        )}
        <ChevronDown className="w-4 h-4" />
      </button>

      {open && (
        <div className="absolute w-full mt-1 bg-white border rounded-lg shadow-lg z-10">
          <input
            type="text"
            placeholder="Search chains..."
            className="w-full px-4 py-2 text-sm border-b focus:outline-none"
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
          />
          <div className="max-h-64 overflow-auto">
            {filteredTokens.length === 0 ? (
              <div className="px-4 py-2 text-sm text-gray-500">
                No chains found
              </div>
            ) : (
              filteredTokens.map((token) => (
                <button
                  key={token.value}
                  className="w-full flex items-center justify-between px-4 py-2 text-sm hover:bg-gray-50"
                  onClick={() => {
                    setValue(token.value === value ? "" : token.value);
                    setOpen(false);
                    setSearchTerm("");
                  }}
                >
                  <div className="flex items-center">
                    <span className="w-4 h-4 mr-2">
                      {value === token.value && <Check className="w-4 h-4 text-blue-500" />}
                    </span>
                    <span className="mr-2">{token.icon}</span>
                    <span className="font-medium">{token.label}</span>
                  </div>
                  <div className="flex flex-col items-end">
                    <span className="text-sm text-gray-500">
                      {formatBalance(token.balance)}
                    </span>
                    <span className="text-xs text-gray-400">
                      {token.chainId}
                    </span>
                  </div>
                </button>
              ))
            )}
          </div>
        </div>
      )}
    </div>
  );
};

export default ChainList;