import * as anchor from '@project-serum/anchor';
import { PublicKey, Keypair } from '@solana/web3.js';
import { expect } from 'chai';
import { getXxusdClient, getWallet, TXN_OPTS } from '../scripts/common';
import { createMint } from '@solana/spl-token';

describe('Initialize Controller', () => {
  const xxusdClient = getXxusdClient();
  const wallet = getWallet();

  it('should initialize the controller', async () => {
    const [controllerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('controller')],
      xxusdClient.programId
    );

    // Create a new mint for xxUSD
    const xxusdMint = await createMint(
      xxusdClient.provider.connection,
      wallet,
      wallet.publicKey,
      null,
      6
    );

    const tx = await xxusdClient.initializeController(wallet.publicKey, 6, xxusdMint);
    await xxusdClient.provider.connection.confirmTransaction(tx, TXN_OPTS.commitment);

    const controllerAccount = await xxusdClient.program.account.controller.fetch(controllerPda);
    expect(controllerAccount).to.not.be.null;
    expect(controllerAccount.authority.toBase58()).to.equal(wallet.publicKey.toBase58());
    expect(controllerAccount.redeemableMintDecimals).to.equal(6);
    expect(controllerAccount.xxusdMint.toBase58()).to.equal(xxusdMint.toBase58());
    expect(controllerAccount.frozen).to.be.false;
  });
});