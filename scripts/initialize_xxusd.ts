import { getConnection, getXxusdClient, getWallet, TXN_OPTS } from './common';
import { PublicKey, Transaction } from '@solana/web3.js';

async function main() {
  const connection = getConnection();
  const xxusdClient = getXxusdClient();
  const wallet = getWallet();

  console.log('Initializing xxUSD controller...');

  try {
    const [controllerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('controller')],
      xxusdClient.programId
    );

    const tx = new Transaction();
    const initializeIx = await xxusdClient.createInitializeControllerInstruction(wallet.publicKey, 6); // 6 decimals for xxUSD
    tx.add(initializeIx);

    const latestBlockhash = await connection.getLatestBlockhash();
    tx.recentBlockhash = latestBlockhash.blockhash;
    tx.feePayer = wallet.publicKey;

    const signedTx = await wallet.signTransaction(tx);
    const txId = await connection.sendRawTransaction(signedTx.serialize(), TXN_OPTS);
    await connection.confirmTransaction(txId, TXN_OPTS.commitment);

    console.log(`xxUSD controller initialized. Transaction ID: ${txId}`);
    console.log(`Controller PDA: ${controllerPda.toBase58()}`);
  } catch (error) {
    console.error('Error initializing xxUSD controller:', error);
  }
}

main().then(
  () => process.exit(),
  (err) => {
    console.error(err);
    process.exit(-1);
  }
);