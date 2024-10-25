import { useEffect } from "react";
import {
  Box,
  ClipboardCopyText,
  Stack,
  useColorModeValue,
} from "@interchain-ui/react";
import { WalletStatus } from "@cosmos-kit/core";
import { useChain } from "@cosmos-kit/react";
import { chains } from "chain-registry";
import { User } from "./User";
import { Warning } from "./Warning";
import { ChainSelect } from "./Chain";
import { CHAIN_NAME, CHAIN_NAME_STORAGE_KEY } from "@/frontend/config";
import {
  ButtonConnect,
  ButtonConnected,
  ButtonConnecting,
  ButtonDisconnected,
  ButtonError,
  ButtonNotExist,
  ButtonRejected,
} from "./Connect";

export type WalletProps = {
  chainName?: string;
  onChainChange?: (chainName?: string) => void;
};

export function Wallet({
  chainName = CHAIN_NAME,
  onChainChange = () => {},
}: WalletProps) {
  const {
    chain,
    status,
    wallet,
    username,
    address,
    message,
    connect,
    openView,
  } = useChain(chainName);

  const ConnectButton = {
    // @ts-ignore
    [WalletStatus.Connected]: <ButtonConnected onClick={openView} />,
    // @ts-ignore
    [WalletStatus.Connecting]: <ButtonConnecting />,
    // @ts-ignore
    [WalletStatus.Disconnected]: <ButtonDisconnected onClick={connect} />,
    // @ts-ignore
    [WalletStatus.Error]: <ButtonError onClick={openView} />,
    // @ts-ignore
    [WalletStatus.Rejected]: <ButtonRejected onClick={connect} />,
    // @ts-ignore
    [WalletStatus.NotExist]: <ButtonNotExist onClick={openView} />,
    // @ts-ignore
  }[status] || <ButtonConnect onClick={connect} />;

  function handleChainChange(chainName?: string) {
    if (chainName) {
      onChainChange(chainName);
      localStorage.setItem(CHAIN_NAME_STORAGE_KEY, chainName!);
    }
  }

  useEffect(() => {
    const selected = localStorage.getItem(CHAIN_NAME_STORAGE_KEY);
    if (selected && selected !== chainName) {
      onChainChange(selected);
    }
  }, []);

  return (
    // @ts-ignore
    <Box py="$16">
          {/* @ts-ignore */}

      <Box mx="auto" maxWidth="28rem" attributes={{ mb: "$12" }}>
          {/* @ts-ignore */}

        <ChainSelect
          chains={chains}
          chainName={chain.chain_name}
          onChange={handleChainChange}
        />
      </Box>
      <Stack
        direction="vertical"
        attributes={{
          mx: "auto",
          px: "$8",
          py: "$15",
          maxWidth: "21rem",
          borderRadius: "$lg",
          justifyContent: "center",
          backgroundColor: useColorModeValue("$white", "$blackAlpha500"),
          boxShadow: useColorModeValue(
            "0 0 2px #dfdfdf, 0 0 6px -2px #d3d3d3",
            "0 0 2px #363636, 0 0 8px -2px #4f4f4f",
          ),
        }}
      >
          {/* @ts-ignore */}

        {username ? <User name={username} /> : null}
        {address
          ? <ClipboardCopyText text={address} truncate="middle" />
          : null}
          {/* @ts-ignore */}

        <Box
          my="$8"
          flex="1"
          width="full"
          display="flex"
          height="$16"
          overflow="hidden"
          justifyContent="center"
          px={{ mobile: "$8", tablet: "$10" }}
        >
          {ConnectButton}
        </Box>

        {message &&
            [WalletStatus.Error, WalletStatus.Rejected].includes(status)
            // @ts-ignore
          ? <Warning text={`${wallet?.prettyName}: ${message}`} />
          : null}
      </Stack>
    </Box>
  );
}
