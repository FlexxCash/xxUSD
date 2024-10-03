import { PublicKey, Signer, Transaction, Keypair } from '@solana/web3.js';
import * as anchor from '@project-serum/anchor';
import {
  getOrCreateAssociatedTokenAccount,
  createTransferInstruction,
} from '@solana/spl-token';
import { getConnection, TXN_OPTS } from './connection';
import {
  JUPSOL_DEVNET,
  JUPSOL_DEVNET_DECIMALS,
  USDC_DEVNET,
  USDC_DEVNET_DECIMALS,
} from './constants';
import BN from 'bn.js';

export async function sendAndConfirmTransaction(
  transaction: Transaction,
  signers: Signer[]
): Promise<string> {
  const connection = getConnection();
  const txid = await connection.sendTransaction(transaction, signers, TXN_OPTS);
  await connection.confirmTransaction(txid, TXN_OPTS.commitment);
  return txid;
}

export async function transferJupSOL(
  amount: number,
  from: Signer,
  to: PublicKey
): Promise<string> {
  const connection = getConnection();
  const fromATA = await getOrCreateAssociatedTokenAccount(
    connection,
    from,
    JUPSOL_DEVNET,
    from.publicKey
  );
  const toATA = await getOrCreateAssociatedTokenAccount(
    connection,
    from,
    JUPSOL_DEVNET,
    to
  );

  const transferIx = createTransferInstruction(
    fromATA.address,
    toATA.address,
    from.publicKey,
    amount * Math.pow(10, JUPSOL_DEVNET_DECIMALS)
  );

  const transaction = new Transaction().add(transferIx);
  return sendAndConfirmTransaction(transaction, [from]);
}

export function uiToNative(amount: number, decimals: number): BN {
  return new BN(amount * Math.pow(10, decimals));
}

export function nativeToUi(amount: BN, decimals: number): number {
  return amount.toNumber() / Math.pow(10, decimals);
}

export async function getJupSOLBalance(wallet: PublicKey): Promise<number> {
  const connection = getConnection();
  const payer = Keypair.generate(); // 創建一個臨時的 payer
  const ata = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    JUPSOL_DEVNET,
    wallet
  );
  const balance = await connection.getTokenAccountBalance(ata.address);
  return balance.value.uiAmount || 0;
}

export async function getUSDCBalance(wallet: PublicKey): Promise<number> {
  const connection = getConnection();
  const payer = Keypair.generate(); // 創建一個臨時的 payer
  const ata = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    USDC_DEVNET,
    wallet
  );
  const balance = await connection.getTokenAccountBalance(ata.address);
  return balance.value.uiAmount || 0;
}

// Add more utility functions as needed