import { MongoClient, Db } from 'mongodb';
import { SigningStargateClient } from '@cosmjs/stargate';

interface LendingEvent {
  eventType: string;
  user: string;
  amount: number;
  collateral: string;
  debt: string;
}

async function indexChainEvents() {
  const client = await MongoClient.connect('mongodb://localhost:27017');
  const db: Db = client.db('lending_protocol');
  
  const rpcEndpoint = "https://archway-node-url.com";
  const chainClient = await SigningStargateClient.connect(rpcEndpoint);

  // Listening to relevant events such as "borrow", "repay", "liquidate", etc.
  // Example structure; use a proper event handler in your implementation
  //@ts-ignore
  chainClient.subscribeTx({ events: ['lending_event'] }, async (event : any) => {
    const lendingEvent: LendingEvent = {
      eventType: event.events[0].type, // assuming event structure
      user: event.events[0].attributes[0].value,
      amount: parseFloat(event.events[0].attributes[1].value),
      collateral: event.events[0].attributes[2].value,
      debt: event.events[0].attributes[3].value,
    };

    await db.collection('events').insertOne(lendingEvent);
    console.log("Indexed new lending event", lendingEvent);
  });

  console.log("Listening for lending events...");
}

indexChainEvents().catch(console.error);
