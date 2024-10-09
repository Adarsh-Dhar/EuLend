import { ApolloServer, gql } from 'apollo-server';
import { MongoClient, Db } from 'mongodb';

let db: Db;

// Connect to MongoDB
MongoClient.connect('mongodb://localhost:27017').then(client => {
  db = client.db('lending_protocol');
  console.log('Connected to database');
}).catch(err => console.error('Failed to connect to database', err));

// GraphQL Schema
const typeDefs = gql`
  type Market {
    id: ID!
    name: String!
    interestRate: Float!
    totalLiquidity: Float!
  }

  type Position {
    user: String!
    amountLent: Float!
    amountBorrowed: Float!
  }

  type Query {
    markets: [Market]
    userPositions(user: String!): [Position]
  }
`;

// GraphQL Resolvers
const resolvers = {
  Query: {
    markets: async () => {
      return await db.collection('markets').find().toArray();
    },
    userPositions: async (_: any, { user }: { user: string }) => {
      return await db.collection('positions').find({ user }).toArray();
    },
  },
};

// Create Apollo Server
const server = new ApolloServer({ typeDefs, resolvers });

server.listen().then(({ url }) => {
  console.log(`🚀 GraphQL Server ready at ${url}`);
});
