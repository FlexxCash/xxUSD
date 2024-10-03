import * as anchor from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import { expect } from 'chai';
import { authority, user, xxusdProgramId, xxusdClient, JUPSOL_DEVNET, JUPSOL_DEVNET_DECIMALS } from './constants';
import { getConnection } from './connection';
import { getJupSOLBalance, transferJupSOL, uiToNative } from './utils';

describe('Mint xxUSD', () => {
  const connection = getConnection();

  before(async () => {
    // Transfer some jupSOL to the user for testing
    await transferJupSOL(100, authority, user.publicKey);
  });

  it('should mint xxUSD using jupSOL as collateral', async () => {
    const mintAmount = 10; // Amount of xxUSD to mint
    const jupsolAmount = 12; // Amount of jupSOL to use as collateral (assuming 1.2x collateralization ratio)

    const userJupSOLBalanceBefore = await getJupSOLBalance(user.publicKey);

    // Mint xxUSD
    await xxusdClient.mint(user.publicKey, uiToNative(jupsolAmount, JUPSOL_DEVNET_DECIMALS).toNumber(), uiToNative(mintAmount, 6).toNumber());

    const userJupSOLBalanceAfter = await getJupSOLBalance(user.publicKey);

    // Check that the user's jupSOL balance has decreased
    expect(userJupSOLBalanceAfter).to.be.lessThan(userJupSOLBalanceBefore);
    expect(userJupSOLBalanceBefore - userJupSOLBalanceAfter).to.equal(jupsolAmount);

    // TODO: Add checks for xxUSD balance increase
    // This will require implementing a function to get xxUSD balance
  });
});