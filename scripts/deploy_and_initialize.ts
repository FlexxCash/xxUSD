import { getConnection, getXxusdClient, getWallet, TXN_OPTS, xxusdProgramId } from './common';
import { PublicKey, Transaction, SystemProgram, Keypair } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, MINT_SIZE, createInitializeMintInstruction, createMintToInstruction, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress } from '@solana/spl-token';
import * as fs from 'fs';
import * as path from 'path';
import { execSync } from 'child_process';

async function main() {
  const xxusdClient = getXxusdClient();

  console.log('Deploying and initializing xxUSD...');

  try {
    // 讀取程序密鑰
    const programIdFile = './program-id.json';
    console.log(`Reading program ID from ${programIdFile}`);
    const programData = JSON.parse(fs.readFileSync(programIdFile, 'utf8'));
    const programId = new PublicKey(programData.programId);
    console.log(`Program ID: ${programId.toBase58()}`);

    // 更新 Anchor.toml 文件
    const anchorTomlPath = path.join(__dirname, '..', 'Anchor.toml');
    console.log(`Updating Anchor.toml at ${anchorTomlPath}`);
    let anchorToml = fs.readFileSync(anchorTomlPath, 'utf8');
    console.log('Original Anchor.toml content:', anchorToml);
    anchorToml = anchorToml.replace(
      /xxusd = "[^"]*"/,
      `xxusd = "${programId.toBase58()}"`
    );
    fs.writeFileSync(anchorTomlPath, anchorToml);
    console.log('Updated Anchor.toml content:', anchorToml);

    // 使用 Anchor 部署程序
    const deployCmd = `anchor deploy --provider.cluster devnet --provider.wallet ~/.config/solana/new_id.json`;
    console.log(`Executing: ${deployCmd}`);
    try {
      const output = execSync(deployCmd, { stdio: 'pipe' });
      console.log('Anchor deploy output:', output.toString());
    } catch (error) {
      console.error('Error during Anchor deploy:', error);
      if (error && typeof error === 'object' && 'stdout' in error && 'stderr' in error) {
        if (error.stdout) console.error('Stdout:', error.stdout.toString());
        if (error.stderr) console.error('Stderr:', error.stderr.toString());
      }
      throw error;
    }

    console.log('Program deployed successfully.');

    const connection = getConnection();
    const wallet = getWallet();

    // Create xxUSD mint account
    const xxusdMint = Keypair.generate();
    const xxusdMintRent = await connection.getMinimumBalanceForRentExemption(MINT_SIZE);

    const createMintAccountIx = SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: xxusdMint.publicKey,
      space: MINT_SIZE,
      lamports: xxusdMintRent,
      programId: TOKEN_PROGRAM_ID,
    });

    const initializeMintIx = createInitializeMintInstruction(
      xxusdMint.publicKey,
      6, // 6 decimals
      wallet.publicKey,
      wallet.publicKey
    );

    // Initialize the controller
    const [controllerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('controller')],
      xxusdProgramId
    );

    const initializeControllerIx = await xxusdClient.createInitializeControllerInstruction(wallet.publicKey, 6);

    const tx = new Transaction().add(
      createMintAccountIx,
      initializeMintIx,
      initializeControllerIx
    );

    const latestBlockhash = await connection.getLatestBlockhash();
    tx.recentBlockhash = latestBlockhash.blockhash;
    tx.feePayer = wallet.publicKey;

    const signedTx = await wallet.signTransaction(tx);
    signedTx.partialSign(xxusdMint);
    const txId = await connection.sendRawTransaction(signedTx.serialize(), TXN_OPTS);
    await connection.confirmTransaction(txId, TXN_OPTS.commitment);

    console.log(`xxUSD mint and controller initialized. Transaction ID: ${txId}`);
    console.log(`xxUSD Mint: ${xxusdMint.publicKey.toBase58()}`);
    console.log(`Controller PDA: ${controllerPda.toBase58()}`);
    console.log(`Program ID: ${xxusdProgramId.toBase58()}`);

    // Optional: Create a mock collateral account and mint some xxUSD for testing
    // This step is for testing purposes only and should not be used in production
    if (process.env['MINT_TEST_XXUSD']) {
      const mockCollateralMint = Keypair.generate();
      const mockCollateralMintRent = await connection.getMinimumBalanceForRentExemption(MINT_SIZE);

      const createMockCollateralMintIx = SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: mockCollateralMint.publicKey,
        space: MINT_SIZE,
        lamports: mockCollateralMintRent,
        programId: TOKEN_PROGRAM_ID,
      });

      const initializeMockCollateralMintIx = createInitializeMintInstruction(
        mockCollateralMint.publicKey,
        9, // 9 decimals for jupSOL
        wallet.publicKey,
        wallet.publicKey
      );

      const userXxusdAta = await getAssociatedTokenAddress(xxusdMint.publicKey, wallet.publicKey);
      const createUserXxusdAtaIx = createAssociatedTokenAccountInstruction(
        wallet.publicKey,
        userXxusdAta,
        wallet.publicKey,
        xxusdMint.publicKey
      );

      const mintTestXxusdIx = createMintToInstruction(
        xxusdMint.publicKey,
        userXxusdAta,
        wallet.publicKey,
        1000000000 // Mint 1000 xxUSD (with 6 decimals)
      );

      const testTx = new Transaction().add(
        createMockCollateralMintIx,
        initializeMockCollateralMintIx,
        createUserXxusdAtaIx,
        mintTestXxusdIx
      );

      const testLatestBlockhash = await connection.getLatestBlockhash();
      testTx.recentBlockhash = testLatestBlockhash.blockhash;
      testTx.feePayer = wallet.publicKey;

      const signedTestTx = await wallet.signTransaction(testTx);
      signedTestTx.partialSign(mockCollateralMint);
      const testTxId = await connection.sendRawTransaction(signedTestTx.serialize(), TXN_OPTS);
      await connection.confirmTransaction(testTxId, TXN_OPTS.commitment);

      console.log(`Mock collateral and test xxUSD minted. Transaction ID: ${testTxId}`);
      console.log(`Mock Collateral Mint: ${mockCollateralMint.publicKey.toBase58()}`);
    }

  } catch (error) {
    console.error('Error deploying and initializing xxUSD:', error);
  }
}

main().catch(console.error);