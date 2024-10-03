import * as anchor from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import { expect } from 'chai';
import { authority, xxusdProgramId, xxusdClient } from './constants';
import { getConnection } from './connection';
import { uiToNative } from './utils';

describe('Manage Hedging Strategy', () => {
  const connection = getConnection();

  it('should deposit to hedging strategy', async () => {
    const depositAmount = uiToNative(1000, 6).toNumber(); // 1000 xxUSD

    await xxusdClient.depositToHedgingStrategy(authority.publicKey, depositAmount);

    // TODO: Add checks for hedging strategy balance
    // This will require implementing a function to get hedging strategy balance
  });

  it('should withdraw from hedging strategy', async () => {
    const withdrawAmount = uiToNative(500, 6).toNumber(); // 500 xxUSD

    await xxusdClient.withdrawFromHedgingStrategy(authority.publicKey, withdrawAmount);

    // TODO: Add checks for hedging strategy balance
    // This will require implementing a function to get hedging strategy balance
  });

  it('should update hedging strategy parameters', async () => {
    const newCollateralRatio = 120; // 120%

    await xxusdClient.updateHedgingStrategyParameters(authority.publicKey, newCollateralRatio);

    // TODO: Add checks for updated hedging strategy parameters
    // This will require implementing a function to get hedging strategy parameters
  });
});