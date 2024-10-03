import * as anchor from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import { expect } from 'chai';
import { authority, xxusdProgramId, xxusdClient } from './constants';
import { getConnection } from './connection';
import { uiToNative } from './utils';

describe('Manage Product Price', () => {
  const connection = getConnection();

  it('should set and get product price', async () => {
    const productId = 1;
    const price = uiToNative(100, 6).toNumber(); // 100 xxUSD

    // Set product price
    await xxusdClient.setProductPrice(authority.publicKey, productId, price);

    // Get product price
    const retrievedPrice = await xxusdClient.getProductPrice(productId);

    expect(retrievedPrice.toNumber()).to.equal(price);
  });

  it('should update product price', async () => {
    const productId = 1;
    const newPrice = uiToNative(150, 6).toNumber(); // 150 xxUSD

    // Update product price
    await xxusdClient.setProductPrice(authority.publicKey, productId, newPrice);

    // Get updated product price
    const retrievedPrice = await xxusdClient.getProductPrice(productId);

    expect(retrievedPrice.toNumber()).to.equal(newPrice);
  });
});