import { Keypair, PublicKey } from '@solana/web3.js';
import { XxusdClient } from '../sdk/xxusd-client';
import jsonIdl from '../target/idl/xxusd.json';
import * as anchor from '@project-serum/anchor';
import { getConnection } from './connection';
import * as fs from 'fs';

// 使用指定的錢包文件
const walletPath = '~/.config/solana/new_id.json';
const walletKeyPair = Keypair.fromSecretKey(
  Uint8Array.from(JSON.parse(fs.readFileSync(walletPath, 'utf-8')))
);

export const authority = walletKeyPair;
console.log(
  `CONTROLLER AUTHORITY => 🔗 https://solscan.io/account/${authority.publicKey}?cluster=devnet`
);

export const user = walletKeyPair;
console.log(
  `USER => 🔗https://solscan.io/account/${user.publicKey}?cluster=devnet`
);

export const CLUSTER = 'devnet';

export const xxusdProgramId: PublicKey = new PublicKey(
  (jsonIdl as any).metadata.address
);
console.debug(`xxUSD PROGRAM ID == ${xxusdProgramId}`);

const connection = getConnection();
const provider = new anchor.AnchorProvider(connection, authority as any, {});
export const xxusdClient = new XxusdClient(xxusdProgramId, provider);

// jupSOL 相關常量
export const JUPSOL_DEVNET = new PublicKey('7eS55f4LP5xj4jqRp24uv5aPFak4gzue8jwb5949KDzP');
export const JUPSOL_DEVNET_DECIMALS = 9;

// USDC 相關常量
export const USDC_DEVNET = new PublicKey('EneKhgmdLQgfLtqC9aE52B1bMcFtjob6qMkDc5Q3mHx7');
export const USDC_DEVNET_DECIMALS = 6;

export const slippageBase = 1000;