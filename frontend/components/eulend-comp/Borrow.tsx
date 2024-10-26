import { useState, useEffect } from 'react';
import { assets } from 'chain-registry';
import { OverviewTransfer, AvailableItem, OverviewTransferProps, Box } from '@interchain-ui/react';
import {fetchAndUpdatePrice} from "../../scripts/oracle"


const symbols = ['STARS','ARCH', 'NIBI','COREUM', 'INJ', 'NTRN'];
const stargaze_price = fetchAndUpdatePrice('stargaze');
const archway_price = fetchAndUpdatePrice('archway');
const nibiru_price = fetchAndUpdatePrice('nibiru');




const dropdownList = symbols.map((symbol) => {
  const asset = assets.find((assetList) => assetList.assets[0].symbol === symbol)!.assets[0];

  console.log('asset', asset);
  console.log('asset name', asset.name);
    console.log('asset symbol', asset.symbol);
    console.log('asset base', asset.base);

    console.log('stargaze_price', stargaze_price);  
console.log('archway_price', archway_price);
console.log('nibiru_price', nibiru_price);


  return {
    imgSrc: asset.logo_URIs?.png || asset.logo_URIs?.jpeg || asset.logo_URIs?.svg,
    name: asset.name,
    symbol: asset.symbol,
    denom: asset.base,
    available: 23,
    priceDisplayAmount: 21,
  } as AvailableItem;
});

function Borrow() {

    useEffect(() => {
          try {
            // Call your async function here
            
            const token = selected.name.toLowerCase()
            console.log('token', token);
            // const price = fetchAndUpdatePrice(token);
            // console.log('price', price);
          } catch (error) {
            console.error("Error fetching data:", error);
          }
        })


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
        fromChainLogoUrl='https://raw.githubusercontent.com/cosmos/chain-registry/master/osmosis/images/osmo.svg'
        toChainLogoUrl={selected.imgSrc}
      />
    </Box>
  );
}

export default Borrow;  