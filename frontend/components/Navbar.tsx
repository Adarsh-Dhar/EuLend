// components/Navbar.tsx
"use client"
import Link from "next/link";
import  ConnectWallet  from "./ConnectWallet";

const Navbar: React.FC = () => {
  return (
    <nav className="bg-gray-800 p-4">
      <div className="container mx-auto flex justify-between items-center">
        <h1 className="text-white font-bold text-xl">My App</h1>
        <div className="flex space-x-4">
          <Link href="/Borrow" className="text-gray-300 hover:text-white transition">
            Borrow
          </Link>
          <Link href="/Repay" className="text-gray-300 hover:text-white transition">
            Repay
          </Link>
          <ConnectWallet />
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
