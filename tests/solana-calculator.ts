import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import * as assert from 'assert';
import { SolanaCalculator } from '../target/types/solana_calculator';

const { SystemProgram } = anchor.web3;

describe('solana-calculator', () => {
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  const calculator = anchor.web3.Keypair.generate();
  const program = (anchor as any).workspace
    .SolanaCalculator as Program<SolanaCalculator>;

  async function getCalculatorAccount() {
    const account = await program.account.calculator.fetch(
      calculator.publicKey
    );
    return account;
  }

  it('Creates a calculator', async () => {
    await program.rpc.create('Welcome to Solana', {
      accounts: {
        calculator: calculator.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [calculator],
    });

    const account = await getCalculatorAccount();

    assert.ok(account.greeting === 'Welcome to Solana');
  });

  it('Adds two numbers', async () => {
    await program.rpc.add(new anchor.BN(2), new anchor.BN(3), {
      accounts: {
        calculator: calculator.publicKey,
      },
    });

    const account = await getCalculatorAccount();

    assert.ok(account.result.eq(new anchor.BN(5)));
    assert.ok(account.greeting === 'Welcome to Solana');
  });

  it('Multiplies two numbers', async () => {
    await program.rpc.multiply(new anchor.BN(2), new anchor.BN(3), {
      accounts: {
        calculator: calculator.publicKey,
      },
    });

    const account = await getCalculatorAccount();

    assert.ok(account.result.eq(new anchor.BN(6)));
    assert.ok(account.greeting === 'Welcome to Solana');
  });

  it('Subtracts two numbers', async () => {
    await program.rpc.subtract(new anchor.BN(32), new anchor.BN(33), {
      accounts: {
        calculator: calculator.publicKey,
      },
    });

    const account = await getCalculatorAccount();

    assert.ok(account.result.eq(new anchor.BN(-1)));
    assert.ok(account.greeting === 'Welcome to Solana');
  });

  it('Divides two numbers', async () => {
    await program.rpc.divide(new anchor.BN(10), new anchor.BN(3), {
      accounts: {
        calculator: calculator.publicKey,
      },
    });

    const account = await getCalculatorAccount();

    assert.ok(account.result.eq(new anchor.BN(3)));
    assert.ok(account.remainder.eq(new anchor.BN(1)));
    assert.ok(account.greeting === 'Welcome to Solana');
  });
});
