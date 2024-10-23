// components/Header.tsx
"use client"
import React from 'react';
import Link from 'next/link';

const Header: React.FC = () => {
  return (
    <header className="bg-gray-800 p-4 shadow-md">
      <nav className="flex justify-around">
        <Link href="/deposit">
          <a className="text-white text-lg hover:underline">Deposit</a>
        </Link>
        <Link href="/withdraw">
          <a className="text-white text-lg hover:underline">Withdraw</a>
        </Link>
        <Link href="/borrow">
          <a className="text-white text-lg hover:underline">Borrow</a>
        </Link>
        <Link href="/repay">
          <a className="text-white text-lg hover:underline">Repay</a>
        </Link>
      </nav>
    </header>
  );
};

export default Header;
