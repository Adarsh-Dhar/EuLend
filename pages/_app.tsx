import '../styles/globals.css';
import '@interchain-ui/react/styles';

import type { AppProps } from 'next/app';
import { SignerOptions, wallets } from 'cosmos-kit';
import { ChainProvider } from '@cosmos-kit/react';
import { assets, chains } from 'chain-registry';
import {
  Box,
  ThemeProvider,
  useColorModeValue,
  useTheme,
} from '@interchain-ui/react';
import Header from "../components/frontend-components/Headers"
import CreateAccount from '@/components/frontend-components/CreateAccount';
import Borrow from '@/components/frontend-components/Borrow';
import Repay from '@/components/frontend-components/Repay';

function CreateCosmosApp({ Component, pageProps }: AppProps) {
  const { themeClass } = useTheme();

  const signerOptions: SignerOptions = {
    // signingStargate: () => {
    //   return getSigningCosmosClientOptions();
    // }
  };

  return (
    <ThemeProvider>
      <Header />
      <ChainProvider
      //@ts-ignore
        chains={chains}
        assetLists={assets}
        wallets={wallets}
        walletConnectOptions={{
          signClient: {
            projectId: 'a8510432ebb71e6948cfd6cde54b70f7',
            relayUrl: 'wss://relay.walletconnect.org',
            metadata: {
              name: 'Cosmos Kit dApp',
              description: 'Cosmos Kit dApp built by Create Cosmos App',
              url: 'https://docs.cosmology.zone/cosmos-kit/',
              icons: [],
            },
          },
        }}
        // @ts-ignore
        signerOptions={signerOptions}
      >
          {/* @ts-ignore */}

        <Box
          className={themeClass}
          minHeight="100dvh"
          backgroundColor={useColorModeValue('$white', '$background')}
        >
          {/* @ts-ignore */}
          <Component {...pageProps} />


        </Box>
      </ChainProvider>

      <CreateAccount />
      <Borrow />
      <Repay />

    </ThemeProvider>
  );
}

export default CreateCosmosApp;
