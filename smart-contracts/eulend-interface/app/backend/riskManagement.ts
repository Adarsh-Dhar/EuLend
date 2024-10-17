import { MongoClient, Db } from 'mongodb';

interface Position {
  user: string;
  amountLent: number;
  amountBorrowed: number;
  collateral: string;
  debt: string;
}

async function monitorHealthFactors() {
  const client = await MongoClient.connect('mongodb://localhost:27017');
  const db: Db = client.db('lending_protocol');

  // Periodically check positions
  setInterval(async () => {
    //@ts-ignore
    const positions: Position[] = await db.collection('positions').find().toArray();

    positions.forEach(async (position) => {
      const healthFactor = calculateHealthFactor(position);
      if (healthFactor < 1) {
        console.log(`Liquidating position for user: ${position.user}`);
        await triggerLiquidation(position.user);
      }
    });
  }, 60000); // check every minute
}

function calculateHealthFactor(position: Position): number {
  const collateralValue = position.amountLent * getCollateralPrice(position.collateral);
  const debtValue = position.amountBorrowed * getDebtPrice(position.debt);
  return collateralValue / debtValue;
}

function getCollateralPrice(collateral: string): number {
  // Fetch price from DB or external service
  // For simplicity, returning a fixed value
  return 1.0;
}

function getDebtPrice(debt: string): number {
  // Fetch price from DB or external service
  // For simplicity, returning a fixed value
  return 1.0;
}

async function triggerLiquidation(user: string) {
  // Interact with smart contract to liquidate the user's position
  console.log(`Triggering liquidation for user: ${user}`);
}

monitorHealthFactors().catch(console.error);
