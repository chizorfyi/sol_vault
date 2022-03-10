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
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import bip39 from "bip39";
import * as dotenv from "dotenv";
dotenv.config();
import { devpair } from "../keypair";
import { bs58 } from "@project-serum/anchor/dist/cjs/utils/bytes";

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
    console.log("devpair:", devpair);

    const depositor = Keypair.fromSecretKey(devpair);

    console.log("depositor:", depositor);

    const transfer_amount = 100000;

    const mintAuthority = Keypair.generate();
    const vault = Keypair.generate();
    const pair = Keypair.generate();
    const userBank = Keypair.generate();

    // const txone = await program.provider.connection.confirmTransaction(
    //   await program.provider.connection.requestAirdrop(depositor.publicKey, 1),
    //   "confirmed"
    // );

    const balance = await program.provider.connection.getBalance(
      depositor.publicKey
    );

    console.log("------------------------------------------------------------");
    // console.log("tx one:", txone);
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
      transfer_amount * 10,
      9
    );

    console.log("------------------------------------------------------------");
    console.log("tx two:", txTwo);

    const txThree = await transferChecked(
      program.provider.connection,
      depositor,
      mainTokenAcct,
      token,
      vaultTokenAcct,
      depositor.publicKey,
      transfer_amount,
      9
    );

    const vaultTokenInfo3 = await getAccount(
      program.provider.connection,
      pair.publicKey
    );

    const mainTokenAcctInfo3 = await getAccount(
      program.provider.connection,
      mainTokenAcct
    );

    console.log("------------------------------------------------------------");
    console.log("tx three:", txThree);
    console.log("vault token info:", vaultTokenInfo3);
    console.log("main token info:", mainTokenAcctInfo3);

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

    const vaultTokenInfo = await getAccount(
      program.provider.connection,
      pair.publicKey
    );

    const mainTokenAcctInfo = await getAccount(
      program.provider.connection,
      mainTokenAcct
    );

    console.log("-----------------------------------------");
    console.log("txSix:", txSix);

    const user2 = await program.account.userBank.fetch(userBank.publicKey);
    const vaultInfo = await program.account.vault.fetch(vault.publicKey);

    console.log("vault token info:", vaultTokenInfo);
    console.log("main token info:", mainTokenAcctInfo);

    console.log("vault info: ", vaultInfo);

    console.log("user:", user2);

    const [pdaAccount, bump] = await findProgramAddressSync(
      [depositor.publicKey.toBuffer()],
      program.programId
    );

    console.log("pda account:", pdaAccount);

    // const tx_ = await program.provider.connection.confirmTransaction(
    //   await program.provider.connection.requestAirdrop(pdaAccount, 100),
    //   "confirmed"
    // );

    // console.log("tx_:", tx_);
    // let balance2 = await program.provider.connection.getBalance(pdaAccount);
    // console.log("balance_:", balance2);

    // const tx_ = new anchor.web3.Transaction().add(
    //   anchor.web3.SystemProgram.transfer({
    //     fromPubkey: depositor.publicKey,
    //     toPubkey: pdaAccount,
    //     lamports: anchor.web3.LAMPORTS_PER_SOL * 2,
    //   })
    // );

    // const sign_ = await anchor.web3.sendAndConfirmTransaction(
    //   program.provider.connection,
    //   tx_,
    //   [depositor]
    // );

    // balance2 = await program.provider.connection.getBalance(pdaAccount);

    // console.log("tx_:", sign_);
    // console.log("balance_:", balance2);

    const txSeven = await program.rpc.withdrawFromVault({
      accounts: {
        depositor: depositor.publicKey,
        depositorTokenAcct: mainTokenAcct,
        vaultTokenAcct: vaultTokenAcct,
        pdaAccount: pdaAccount,
        vault: vault.publicKey,
        userBank: userBank.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [depositor],
    });

    console.log("-----------------------------------------");
    console.log("txSeven:", txSeven);

    let user3 = await program.account.userBank.fetch(userBank.publicKey);
    // const vaultInfo2 = await program.account.vault.fetch(vault.publicKey);

    const vaultTokenInfo2 = await getAccount(
      program.provider.connection,
      pair.publicKey
    );

    const mainTokenAcctInfo2 = await getAccount(
      program.provider.connection,
      mainTokenAcct
    );

    // console.log("vault info: ", vaultInfo2);

    user3 = await program.account.userBank.fetch(userBank.publicKey);

    console.log("user:", user3);

    console.log("vault token info:", vaultTokenInfo2);
    console.log("main token info:", mainTokenAcctInfo2);
  });
});
