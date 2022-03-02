import * as anchor from "@project-serum/anchor";
import { IdlAccounts, Program } from "@project-serum/anchor";
import { SolVaultTransfer } from "../target/types/sol_vault_transfer";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  createAccount,
  createAssociatedTokenAccount,
  createMint,
  getAccount,
  getMint,
  mintToChecked,
  transferChecked,
} from "@solana/spl-token";

type UserBank = IdlAccounts<SolVaultTransfer>["userBank"];
type Vault = IdlAccounts<SolVaultTransfer>["vault"];

describe("sol_vault_transfer", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace
    .SolVaultTransfer as Program<SolVaultTransfer>;

  // let token, depositor, pda_account, mainTokenAcct, vaultTokenAcct;

  it("deposits", async () => {
    // Add your test here.
    const depositor = Keypair.generate();

    const transfer_amount = 100000;

    const mintAuthority = Keypair.generate();
    const vault = Keypair.generate();
    const pair = Keypair.generate();
    const userBank = Keypair.generate();

    const txone = await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        depositor.publicKey,
        10000000000
      ),
      "confirmed"
    );

    const balance = await program.provider.connection.getBalance(
      depositor.publicKey
    );

    console.log("------------------------------------------------------------");
    console.log("tx one:", txone);
    console.log("balance:", balance);

    const token = await createMint(
      program.provider.connection,
      depositor,
      mintAuthority.publicKey,
      null,
      9
    );

    console.log("------------------------------------------------------------");
    console.log("token:", token);

    let mintAccountInfo = await getMint(program.provider.connection, token);

    console.log("------------------------------------------------------------");
    console.log("token info:", mintAccountInfo);

    const mainTokenAcct = await createAssociatedTokenAccount(
      program.provider.connection,
      depositor,
      token,
      depositor.publicKey
    );

    console.log("------------------------------------------------------------");
    console.log("mainTokenAccount:", mainTokenAcct);

    const vaultTokenAcct = await createAccount(
      program.provider.connection,
      depositor,
      token,
      depositor.publicKey,
      pair
    );

    console.log("------------------------------------------------------------");
    console.log("vaultTokenAccount:", vaultTokenAcct);
    console.log("pair:", pair.publicKey);

    const txTwo = await mintToChecked(
      program.provider.connection,
      depositor,
      token,
      mainTokenAcct,
      mintAuthority,
      transfer_amount,
      9
    );

    console.log("------------------------------------------------------------");
    console.log("tx two:", txTwo);

    // const txThree = await transferChecked(
    //   program.provider.connection,
    //   depositor,
    //   mainTokenAcct,
    //   token,
    //   vaultTokenAcct,
    //   depositor.publicKey,
    //   100,
    //   9
    // );

    // const vaultTokenInfo = await getAccount(
    //   program.provider.connection,
    //   pair.publicKey
    // );

    // const mainTokenAcctInfo = await getAccount(
    //   program.provider.connection,
    //   mainTokenAcct
    // );

    // console.log("------------------------------------------------------------");
    // console.log("tx three:", txThree);
    // console.log("vault token info:", vaultTokenInfo);
    // console.log("main token info:", mainTokenAcctInfo);

    // const txFour = await transferChecked(
    //   program.provider.connection,
    //   depositor,
    //   vaultTokenAcct,
    //   token,
    //   mainTokenAcct,
    //   depositor.publicKey,
    //   1,
    //   9
    // );

    // const vaultTokenInfo2 = await getAccount(
    //   program.provider.connection,
    //   pair.publicKey
    // );

    // const mainTokenAcctInfo2 = await getAccount(
    //   program.provider.connection,
    //   mainTokenAcct
    // );

    // console.log("------------------------------------------------------------");
    // console.log("tx three:", txFour);
    // console.log("vault token info:", vaultTokenInfo2);
    // console.log("main token info:", mainTokenAcctInfo2);

    // const tx = await program.rpc.initialize({});
    // console.log("Your transaction signature", tx);

    const txFive = await program.rpc.createUserBank({
      accounts: {
        userBank: userBank.publicKey,
        depositor: depositor.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [depositor, userBank],
    });

    console.log("-----------------------------------------");
    console.log("tx five:", txFive);

    const user = await program.account.userBank.fetch(userBank.publicKey);

    console.log("-----------------------------------------");
    console.log("user:", user);

    const txSix = await program.rpc.depositToVault(transfer_amount, {
      accounts: {
        depositor: depositor.publicKey,
        depositorTokenAcct: mainTokenAcct,
        vaultTokenAcct: vaultTokenAcct,
        vault: vault.publicKey,
        userBank: userBank.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      },
      signers: [vault, depositor],
    });

    console.log("-----------------------------------------");
    console.log("txSix:", txSix);

    const user2 = await program.account.userBank.fetch(userBank.publicKey);
    const vaultInfo = await program.account.userBank.fetch(vault.publicKey);

    console.log("vault info: ", vaultInfo);

    console.log("user:", user2);
  });
});
