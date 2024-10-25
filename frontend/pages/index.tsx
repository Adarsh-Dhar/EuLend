import { useState } from "react";
import { Layout, Wallet } from "@/components";
import { CHAIN_NAME } from "@/frontend/config";

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
