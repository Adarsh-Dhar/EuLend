// components/Navbar.tsx
"use client"
import Link from "next/link";
import  ConnectWallet  from "./ConnectWallet";


const Navbar: React.FC = () => {
  return (

<nav className="bg-gray-800 p-4">
      <div className="container mx-auto flex justify-between items-center">
      <Link href="/" className="text-white font-bold text-xl">
            EuLend
          </Link>

        <div className="flex space-x-4">
          <Link href="/Borrow" className="my-2 text-gray-300 hover:text-white transition">
            Borrow
          </Link>
          <Link href="/Repay" className="my-2 text-gray-300 hover:text-white transition">
            Repay
          </Link>
          <Link href="/ProvideLiquidity" className="my-2 text-gray-300 hover:text-white transition">
            Liquidity
          </Link>
          <ConnectWallet />
        </div>
      </div>
    </nav>

    
  );
};

export default Navbar;
