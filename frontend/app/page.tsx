import Image from "next/image";
import Navbar from "@/components/Navbar";
import ChainList from "@/components/ChainList";
import { RecoilRoot } from "recoil";




export default function Home() {
  return (
    <div>
      <RecoilRoot>
      <Navbar />

      </RecoilRoot>

    </div>
    

  );
}
