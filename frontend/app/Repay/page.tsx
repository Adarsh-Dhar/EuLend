"use client"
import Repay from "@/components/Repay";
import Navbar from "@/components/Navbar";
import { RecoilRoot } from "recoil";

export default function Home() {
  return (
    <div>
      <RecoilRoot>
            <Navbar />
          <Repay />
      </RecoilRoot>
    </div>
  );
}
