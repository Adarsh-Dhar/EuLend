import Image from "next/image";
import Borrow from "@/components/Borrow";
import Navbar from "@/components/Navbar";
import { RecoilRoot } from "recoil";

export default function Home() {
  return (
    <div>
      <RecoilRoot>
            <Navbar />
          <Borrow />
      </RecoilRoot>
      

    </div>
  );
}
