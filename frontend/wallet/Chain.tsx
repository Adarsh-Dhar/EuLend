import { useEffect, useMemo, useState } from 'react';
import { Chain } from '@chain-registry/types';
import { matchSorter } from 'match-sorter';
import {
  Avatar,
  Box,
  Combobox,
  Skeleton,
  Stack,
  Text,
  ThemeProvider,
  useTheme,
} from '@interchain-ui/react';

export type ChainSelectProps = {
  chains: Chain;
  chainName?: string;
  onChange?: (chainName?: string) => void;
};

function ChainOption({ logo, label }: { logo: string; label: string }) {
  return (
    <Stack
      direction="horizontal"
      space="$4"
      attributes={{ alignItems: 'center' }}
    >
      <Avatar
        name={label}
        getInitials={(name : any) => name[0]}
        size="xs"
        src={logo}
        fallbackMode="bg"
      />
      <Text fontSize="$md" fontWeight="$normal" color="$text">
        {label}
      </Text>
    </Stack>
  );
}


export function ChainSelect({
  chainName,
  //@ts-ignore
  chains = [],
  onChange = () => {},
}: ChainSelectProps) {
  const { themeClass } = useTheme();
  const [value, setValue] = useState<string>();
  const [input, setInput] = useState<string>('');

  const cache = useMemo(
    () =>
    //@ts-ignore
      chains.reduce(
        (cache : any, chain : any) => ((cache[chain.chain_name] = chain), cache),
        // @ts-ignore
        {} as Record<string, Chain[number]>
      ),
    [chains]
  );

  const options = useMemo(
    () =>
      matchSorter(
        chains
        //@ts-ignore
          .filter((chain : any) => 
            ['Nibiru', 'Archway Testnet', 'Coreum', 'Neutron Testnet', 'Injective', 'Stargaze Testnet'].includes(chain.pretty_name)
          )
          .map((chain : any) => ({
            logo: chain.logo_URIs?.png || chain.logo_URIs?.svg || '',
            value: chain.chain_name,
            label: chain.pretty_name,
          }))
          .filter((chain : any) => chain.value && chain.label),
        input,
        { keys: ['value', 'label'] }
      ),
    [chains, input]
  );
  

  useEffect(() => {
    if (!chainName) setValue(undefined);
// @ts-ignore
    if (chainName && chains.length > 0) {
      const chain = cache[chainName];

      if (chain) {
        setValue(chain.chain_name);
        setInput(chain.pretty_name);
      }
    }
  }, [chains, chainName]);

  const avatar = cache[value!]?.logo_URIs?.png || cache[value!]?.logo_URIs?.svg;

  return (
    <ThemeProvider>
      <Box
        display="flex"
        justifyContent="center"
        alignItems="center"
        className={themeClass}
      >
        <Combobox
          selectedKey={value}
          inputValue={input}
          onInputChange={(input : any) => {
            setInput(input);
            if (!input) setValue(undefined);
          }}
          onSelectionChange={(value : any) => {
            const selectedChain = cache[value as string];
            if (selectedChain) {
              setValue(selectedChain.chain_name);
              onChange(selectedChain.chain_name); // Only calls onChange with a valid chain name
            }
          }}
          inputAddonStart={
            value && avatar ? (
              <Avatar
                name={value as string}
                getInitials={(name : any) => name[0]}
                size="xs"
                src={avatar}
                fallbackMode="bg"
                attributes={{
                  paddingX: '$4',
                }}
              />
            ) : (
              <Box
                display="flex"
                justifyContent="center"
                alignItems="center"
                px="$4"
              >
                <Skeleton width="24px" height="24px" borderRadius="$full" />
              </Box>
            )
          }
          styleProps={{
            width: {
              mobile: '100%',
              mdMobile: '350px',
            },
          }}
        >
          {options.map((option : any) => (
            <Combobox.Item key={option.value} textValue={option.label}>
              <ChainOption logo={option.logo ?? ''} label={option.label} />
            </Combobox.Item>
          ))}
        </Combobox>
      </Box>
    </ThemeProvider>
  );
}
