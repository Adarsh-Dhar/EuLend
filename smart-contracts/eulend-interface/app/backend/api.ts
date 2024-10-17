import express from 'express';
import { MongoClient, Db } from 'mongodb';


const app = express();
let db: Db;

// Connect to MongoDB
MongoClient.connect('mongodb://localhost:27017').then(client => {
  db = client.db('lending_protocol');
  console.log('Connected to database');
}).catch(err => console.error('Failed to connect to database', err));

// REST API Endpoints

// Get all lending markets
app.get('/api/v1/markets', async (req, res) => {
  const markets = await db.collection('markets').find().toArray();
  res.json(markets);
});

// Get user’s lending and borrowing positions
app.get('/api/v1/user/:address/positions', async (req, res) => {
  const userAddress = req.params.address;
  const positions = await db.collection('positions').find({ user: userAddress }).toArray();
  res.json(positions);
});

// Get current token prices
app.get('/api/v1/prices', async (req, res) => {
  const prices = await db.collection('prices').find().toArray();
  res.json(prices);
});

// Start the server
app.listen(3000, () => {
  console.log('API Server running on port 3000');
});
