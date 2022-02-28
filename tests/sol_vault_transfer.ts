import * as anchor from "@project-serum/anchor";
import { IdlAccounts, Program } from "@project-serum/anchor";
import { SolVaultTransfer } from "../target/types/sol_vault_transfer";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import {
  createAssociatedTokenAccount,
  createMint,
  mintToChecked,
} from "@solana/spl-token";

type UserBank = IdlAccounts<SolVaultTransfer>["userBank"];
type Vault = IdlAccounts<SolVaultTransfer>["vault"];

describe("sol_vault_transfer", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace
    .SolVaultTransfer as Program<SolVaultTransfer>;

  // let token, depositor, pda_account, mainTokenAcct, vaultTokenAcct;

  const depositor = Keypair.generate();

  const transfer_amount = 100000;

  const mintAuthority = Keypair.generate();
  const userBank = Keypair.generate();
  const vault = Keypair.generate();

  it("deposits", async () => {
    // Add your test here.

    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        depositor.publicKey,
        10000000000
      ),
      "confirmed"
    );

    const token = await createMint(
      program.provider.connection,
      depositor,
      mintAuthority.publicKey,
      null,
      9
    );

    const mainTokenAcct = await createAssociatedTokenAccount(
      program.provider.connection,
      depositor,
      token,
      depositor.publicKey
    );

    const vaultTokenAcct = await createAssociatedTokenAccount(
      program.provider.connection,
      depositor,
      token,
      depositor.publicKey
    );

    let tx = await mintToChecked(
      program.provider.connection,
      depositor,
      token,
      mainTokenAcct,
      depositor,
      transfer_amount,
      9
    );

    console.log(tx);

    // const tx = await program.rpc.initialize({});
    // console.log("Your transaction signature", tx);
  });
});
