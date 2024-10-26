import { useState } from 'react';
import { assets } from 'chain-registry';
import { OverviewTransfer, AvailableItem, OverviewTransferProps, Box } from '@interchain-ui/react';

const symbols = ['ATOM', 'JUNO', 'STARS', 'BLD', 'STRD', 'CRO', 'AKT', 'MARS'];

const dropdownList = symbols.map((symbol) => {
  const asset = assets.find((assetList) => assetList.assets[0].symbol === symbol)!.assets[0];

  return {
    imgSrc: asset.logo_URIs?.png || asset.logo_URIs?.jpeg || asset.logo_URIs?.svg,
    name: asset.name,
    symbol: asset.symbol,
    denom: asset.base,
    available: Number((Math.random() * 100).toFixed(6)),
    priceDisplayAmount: Math.floor(Math.random() * 10) + 1,
  } as AvailableItem;
});

function Demo() {
  const [selected, setSelected] = useState<AvailableItem>(dropdownList[0]);

  const onChange: OverviewTransferProps['onChange'] = (selectedItem, value) => {
    console.log('onChange', selectedItem, value);
    setSelected(selectedItem);
  };

  return (
    <Box display='flex' justifyContent='center'>
      <OverviewTransfer
        onChange={onChange}
        dropdownList={dropdownList}
        selectedItem={selected}
        timeEstimateLabel='~ 20 seconds'
        onTransfer={() => {
          console.log('onTransfer');
        }}
        onCancel={() => {
          console.log('onCancel');
        }}
        fromChainLogoUrl={selected.imgSrc}
        toChainLogoUrl='https://raw.githubusercontent.com/cosmos/chain-registry/master/osmosis/images/osmo.svg'
      />
    </Box>
  );
}