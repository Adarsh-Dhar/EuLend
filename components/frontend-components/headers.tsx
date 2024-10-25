// components/Header.tsx
"use client"
import React from 'react';
import Link from 'next/link';

const Header: React.FC = () => {
  return (
    <header className="bg-gray-800 p-4 shadow-md">
      <nav className="flex justify-end space-x-6 pr-8 mt-4">
        <Link href="/Deposit" className="text-white text-lg hover:underline">
          Deposit
        </Link>
        <Link href="/Withdraw" className="text-white text-lg hover:underline">
          Withdraw
        </Link>
        <Link href="/Borrow" className="text-white text-lg hover:underline">
          Borrow
        </Link>
        <Link href="/Repay" className="text-white text-lg hover:underline">
          Repay
        </Link>
      </nav>
    </header>
  );
};

export default Header;
