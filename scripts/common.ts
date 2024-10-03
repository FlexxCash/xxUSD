import {
  PublicKey,
  Connection,
  ConnectionConfig,
  ConfirmOptions,
  Keypair,
  Transaction,
} from '@solana/web3.js';
import { XxusdClient } from '../sdk/xxusd-client';
import * as anchor from '@project-serum/anchor';
import * as fs from 'fs';
import * as os from 'os';
import * as path from 'path';

const TXN_COMMIT = 'confirmed';
const connectionConfig = {
  commitment: TXN_COMMIT,
  disableRetryOnRateLimit: false,
  confirmTransactionInitialTimeout: 10000,
} as ConnectionConfig;
export const TXN_OPTS = {
  commitment: TXN_COMMIT,
  preflightCommitment: TXN_COMMIT,
  skipPreflight: true,
} as ConfirmOptions;

function getOrCreateProgramId(): PublicKey {
  const programIdFile = './program-id.json';
  let programId: PublicKey;

  if (fs.existsSync(programIdFile)) {
    const data = JSON.parse(fs.readFileSync(programIdFile, 'utf8'));
    programId = new PublicKey(data.programId);
  } else {
    const newKeypair = Keypair.generate();
    programId = newKeypair.publicKey;
    fs.writeFileSync(programIdFile, JSON.stringify({
      programId: programId.toBase58(),
      secretKey: Array.from(newKeypair.secretKey)
    }));
  }

  return programId;
}

export const xxusdProgramId = getOrCreateProgramId();
export const jupsolMint = new PublicKey('7eS55f4LP5xj4jqRp24uv5aPFak4gzue8jwb5949KDzP');
export const usdcMint = new PublicKey('EneKhgmdLQgfLtqC9aE52B1bMcFtjob6qMkDc5Q3mHx7');

export function getConnection() {
  const connection = new Connection(
    'https://api.devnet.solana.com',
    connectionConfig
  );
  return connection;
}

export function getXxusdClient() {
  const connection = getConnection();
  const wallet = getWallet();
  const provider = new anchor.AnchorProvider(connection, wallet, {});
  return new XxusdClient(xxusdProgramId, provider);
}

export function getWallet() {
  const walletPath = process.env.SOLANA_WALLET_PATH || path.join(os.homedir(), '.config', 'solana', 'new_id.json');
  try {
    const walletKeyPair = Keypair.fromSecretKey(
      Uint8Array.from(JSON.parse(fs.readFileSync(walletPath, 'utf-8')))
    );
    
    return {
      publicKey: walletKeyPair.publicKey,
      signTransaction: async (tx: Transaction) => {
        tx.partialSign(walletKeyPair);
        return tx;
      },
      signAllTransactions: async (txs: Transaction[]) => {
        return txs.map((tx) => {
          tx.partialSign(walletKeyPair);
          return tx;
        });
      },
    };
  } catch (error) {
    console.error(`Error reading wallet file at ${walletPath}:`, error);
    throw new Error('Failed to read wallet file. Please check the file path and permissions.');
  }
}

// Add more utility functions as needed