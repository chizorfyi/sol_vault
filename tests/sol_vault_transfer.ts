import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { SolVaultTransfer } from '../target/types/sol_vault_transfer';

describe('sol_vault_transfer', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolVaultTransfer as Program<SolVaultTransfer>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
