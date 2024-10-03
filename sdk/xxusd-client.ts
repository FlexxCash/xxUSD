import { Connection, PublicKey, Transaction, TransactionInstruction } from '@solana/web3.js';
import { Program, Provider, BN } from '@project-serum/anchor';
import idl from '../target/idl/xxusd.json';

export class XxusdClient {
  public program: Program;
  public provider: Provider;

  constructor(public programId: PublicKey, provider: Provider) {
    this.provider = provider;
    this.program = new Program(idl as any, programId, provider);
  }

  async createInitializeControllerInstruction(authority: PublicKey, redeemableMintDecimals: number): Promise<TransactionInstruction> {
    const [controllerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('controller')],
      this.programId
    );

    return this.program.methods['initializeController'](redeemableMintDecimals)
      .accounts({
        authority,
        controller: controllerPda,
        systemProgram: PublicKey.default,
      })
      .instruction();
  }

  async initializeController(authority: PublicKey, redeemableMintDecimals: number): Promise<string> {
    const [controllerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('controller')],
      this.programId
    );

    const tx = await this.program.methods['initializeController'](redeemableMintDecimals)
      .accounts({
        authority,
        controller: controllerPda,
        systemProgram: PublicKey.default,
      })
      .rpc();

    return tx;
  }

  async mint(user: PublicKey, collateralAmount: number, mintAmount: number): Promise<string> {
    const [controllerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('controller')],
      this.programId
    );

    const tx = await this.program.methods
      .mint(new BN(collateralAmount), new BN(mintAmount))
      .accounts({
        user,
        controller: controllerPda,
        // Add other necessary accounts here
      })
      .rpc();

    return tx;
  }

  async lockXxusd(user: PublicKey, amount: number, lockPeriod: number): Promise<string> {
    const [controllerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('controller')],
      this.programId
    );

    const tx = await this.program.methods
      .lockXxusd(new BN(amount), new BN(lockPeriod))
      .accounts({
        user,
        controller: controllerPda,
        // Add other necessary accounts here
      })
      .rpc();

    return tx;
  }

  async releaseXxusd(user: PublicKey): Promise<string> {
    const [controllerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('controller')],
      this.programId
    );

    const tx = await this.program.methods
      .releaseXxusd()
      .accounts({
        user,
        controller: controllerPda,
        // Add other necessary accounts here
      })
      .rpc();

    return tx;
  }

  async setProductPrice(authority: PublicKey, productId: number, price: number): Promise<string> {
    const [controllerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('controller')],
      this.programId
    );

    const tx = await this.program.methods
      .setProductPrice(new BN(productId), new BN(price))
      .accounts({
        authority,
        controller: controllerPda,
        // Add other necessary accounts here
      })
      .rpc();

    return tx;
  }

  async getProductPrice(productId: number): Promise<BN> {
    const [controllerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('controller')],
      this.programId
    );

    const price = await this.program.methods
      .getProductPrice(new BN(productId))
      .accounts({
        controller: controllerPda,
      })
      .view();

    return price;
  }

  async depositToHedgingStrategy(authority: PublicKey, amount: number): Promise<string> {
    const [controllerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('controller')],
      this.programId
    );

    const tx = await this.program.methods
      .depositToHedgingStrategy(new BN(amount))
      .accounts({
        authority,
        controller: controllerPda,
        // Add other necessary accounts here
      })
      .rpc();

    return tx;
  }

  async withdrawFromHedgingStrategy(authority: PublicKey, amount: number): Promise<string> {
    const [controllerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('controller')],
      this.programId
    );

    const tx = await this.program.methods
      .withdrawFromHedgingStrategy(new BN(amount))
      .accounts({
        authority,
        controller: controllerPda,
        // Add other necessary accounts here
      })
      .rpc();

    return tx;
  }

  async updateHedgingStrategyParameters(authority: PublicKey, collateralRatio: number): Promise<string> {
    const [controllerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('controller')],
      this.programId
    );

    const tx = await this.program.methods
      .updateHedgingStrategyParameters(new BN(collateralRatio))
      .accounts({
        authority,
        controller: controllerPda,
        // Add other necessary accounts here
      })
      .rpc();

    return tx;
  }
}