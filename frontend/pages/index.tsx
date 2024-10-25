import { useState } from "react";
import { Wallet } from "../wallet";
import { CHAIN_NAME } from "../config";
import {Layout} from "../components/Layout";

export default function Home() {
  const [chainName, setChainName] = useState(CHAIN_NAME);

  function onChainChange(chainName?: string) {
    setChainName(chainName!);
  }

  return (
    // @ts-ignore
    <Layout>
          {/* @ts-ignore */}

      <Wallet chainName={chainName} onChainChange={onChainChange} />
    </Layout>
  );
}
