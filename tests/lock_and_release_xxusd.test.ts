import * as anchor from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import { expect } from 'chai';
import { authority, user, xxusdProgramId, xxusdClient } from './constants';
import { getConnection } from './connection';
import { uiToNative } from './utils';

describe('Lock and Release xxUSD', () => {
  const connection = getConnection();

  before(async () => {
    // Mint some xxUSD for the user to lock
    await xxusdClient.mint(user.publicKey, uiToNative(12, 9).toNumber(), uiToNative(10, 6).toNumber());
  });

  it('should lock xxUSD', async () => {
    const lockAmount = uiToNative(5, 6).toNumber(); // Lock 5 xxUSD
    const lockPeriod = 7 * 24 * 60 * 60; // 7 days in seconds

    await xxusdClient.lockXxusd(user.publicKey, lockAmount, lockPeriod);

    // TODO: Add checks for locked xxUSD balance
    // This will require implementing a function to get locked xxUSD balance
  });

  it('should release xxUSD after lock period', async () => {
    // Fast-forward time (this is a simplified approach, you might need to use a different method in your actual testing environment)
    await new Promise(resolve => setTimeout(resolve, 7 * 24 * 60 * 60 * 1000));

    await xxusdClient.releaseXxusd(user.publicKey);

    // TODO: Add checks for released xxUSD balance
    // This will require implementing a function to get xxUSD balance
  });
});