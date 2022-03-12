import * as anchor from "@project-serum/anchor";
import { IdlAccounts, Program } from "@project-serum/anchor";
import { SolVaultTransfer } from "../target/types/sol_vault_transfer";
import {
  PublicKey,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";

import * as spltoken from "@solana/spl-token";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { devpair } from "../keypair";
import {
  Cache,
  Cluster,
  Control,
  CONTROL_ACCOUNT_SIZE,
  createProgram,
  Margin,
  State,
  TOKEN_PROGRAM_ID,
} from "@zero_one/client";
import {
  getOrCreateAssociatedTokenAccount,
  getAccount,
} from "../node_modules/@solana/spl-token";

import assert from "assert";

describe("sol_vault_transfer", () => {
  // Configure the client to use the local cluster.
  const depositor = Keypair.fromSecretKey(devpair);

  console.log("depositor:", depositor.publicKey.toBase58());

  const provider = anchor.Provider.env();

  anchor.setProvider(provider);

  const zoDexId = new PublicKey("ZDxUi178LkcuwdxcEqsSo2E7KATH99LAAXN5LcSVMBC");

  const serumDexId = new PublicKey(
    "DESVgJVGajEgKGXhb6XmqDHGz3VjdgP7rEVESBgxmroY"
  );

  const zoStateId = new PublicKey(
    "KwcWW7WvgSXLJcyjKZJBHLbfriErggzYHpjS9qjVD5F"
  );
  const usdcMint = new PublicKey(
    "7UT1javY6X1M9R2UrPGrwcZ78SX3huaXyETff5hm5YdX"
  );

  const program = anchor.workspace
    .SolVaultTransfer as Program<SolVaultTransfer>;

  it("deposits", async () => {
    // Add your test here.

    console.log("depositor pubkey:", depositor.publicKey.toBase58());

    const zoProgram = createProgram(provider, Cluster.Devnet);

    console.log("zo program:", zoProgram.programId.toBase58());

    console.log("---------------------------------------");

    const zoState = await State.load(zoProgram, zoStateId);

    console.log("---------------------------------------");

    const zoUsdcVault = zoState.getVaultCollateralByMint(usdcMint)[0];

    console.log("---------------------------");
    console.log("zo usdc vault:", zoUsdcVault.toBase58());

    const depUsdc = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      depositor,
      usdcMint,
      depositor.publicKey
    );

    const depUsdcAcct = await getAccount(provider.connection, depUsdc.address);

    console.log("---------------------------------------");
    console.log("depUsdc:", depUsdc);
    console.log("depositor usdc acct:", depUsdcAcct);

    const [[margin, nonce], control, controlLamports] = await Promise.all([
      PublicKey.findProgramAddress(
        [
          depositor.publicKey.toBuffer(),
          zoState.pubkey.toBuffer(),
          Buffer.from("marginv1"),
        ],
        zoProgram.programId
      ),
      Keypair.generate(),
      provider.connection.getMinimumBalanceForRentExemption(
        CONTROL_ACCOUNT_SIZE
      ),
    ]);

    console.log("======================================");
    console.log("key:", margin.toBase58());
    console.log("nonce:", nonce);
    console.log("control lamports", controlLamports);

    const info = await program.provider.connection.getAccountInfo(margin);

    if (info) {
      console.log("Margin account already exists");
    } else {
      //calling CreateMargin through CPI call

      const tx = await program.rpc.createZoMargin(nonce, {
        accounts: {
          authority: depositor.publicKey,
          zoProgramState: zoState.pubkey,
          zoMargin: margin,
          zoProgram: zoProgram.programId,
          control: control.publicKey,
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
        },
        preInstructions: [
          SystemProgram.createAccount({
            fromPubkey: depositor.publicKey,
            newAccountPubkey: control.publicKey,
            lamports: controlLamports,
            space: CONTROL_ACCOUNT_SIZE,
            programId: zoProgram.programId,
          }),
        ],
        signers: [control, depositor],
      });

      const txTwo = await provider.connection.confirmTransaction(
        tx,
        "confirmed"
      );

      console.log("tx two:", txTwo);
    }

    const zoMargin = await Margin.load(zoProgram, zoState);
    const zoControl = await Control.load(zoProgram, zoMargin.data.control);

    console.log("zo margin:", zoMargin);
    console.log("zo control:", zoControl);

    //deposit

    const depositAmount = new anchor.BN("100000");
    console.log("depositing amount: ", depositAmount.toString());

    const fetchBalanceBefore = await provider.connection.getTokenAccountBalance(
      depUsdc.address
    );
    console.log(
      "user USDC balance before deposit: ",
      fetchBalanceBefore.value.amount
    );

    await zoMargin.refresh();
    console.log(
      "user Margin USDC balance before deposit: ",
      zoMargin.balances.USDC.n.toString()
    );

    const fetchVaultBalanceBefore =
      await program.provider.connection.getTokenAccountBalance(zoUsdcVault);
    console.log(
      "state vault USDC balance before deposit: ",
      fetchVaultBalanceBefore.value.amount
    );

    const tx3 = await program.rpc.zoDeposit(depositAmount, {
      accounts: {
        authority: depositor.publicKey,
        zoProgramState: zoState.pubkey,
        zoProgramMargin: zoMargin.pubkey,
        zoProgram: zoProgram.programId,
        cache: zoState.cache.pubkey,
        stateSigner: zoMargin.state.signer,
        tokenAccount: depUsdc.address,
        zoProgramVault: zoUsdcVault,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [depositor],
    });

    const tx2 = await program.provider.connection.confirmTransaction(
      tx3,
      "finalized"
    );

    console.log("=======================================");
    console.log("tx2:", tx2);

    const fetchBalanceAfter2 =
      await program.provider.connection.getTokenAccountBalance(depUsdc.address);
    console.log(
      "user USDC balance after deposit: ",
      fetchBalanceAfter2.value.amount
    );

    await zoMargin.refresh();

    console.log(
      "user Margin USDC balance after deposit: ",
      zoMargin.balances.USDC.n.toString()
    );

    const fetchVaultBalanceAfter3 =
      await program.provider.connection.getTokenAccountBalance(zoUsdcVault);
    console.log(
      "state vault USDC balance after deposit: ",
      fetchVaultBalanceAfter3.value.amount
    );

    //Withdrawal

    // const withdrawAmount = new anchor.BN("50");

    // console.log("withdrawing amount: ", withdrawAmount.toString());

    // const fetchBalanceBefore2 =
    //   await provider.connection.getTokenAccountBalance(depUsdc.address);
    // console.log(
    //   "user USDC balance before withdraw: ",
    //   fetchBalanceBefore2.value.amount
    // );

    // await zoMargin.refresh();
    // console.log(
    //   "user Margin USDC balance before withdraw: ",
    //   zoMargin.balances.USDC.n.toString()
    // );

    // const fetchVaultBalanceBefore3 =
    //   await provider.connection.getTokenAccountBalance(zoUsdcVault);
    // console.log(
    //   "state vault USDC balance before withdraw: ",
    //   fetchVaultBalanceBefore3.value.amount
    // );

    // const tx = await program.rpc.zoWithdrawal(withdrawAmount, {
    //   accounts: {
    //     authority: depositor.publicKey,
    //     zoProgramState: zoState.pubkey,
    //     zoProgramMargin: zoMargin.pubkey,
    //     zoProgram: zoProgram.programId,
    //     control: zoMargin.control.pubkey,
    //     cache: zoState.cache.pubkey,
    //     stateSigner: zoMargin.state.signer,
    //     tokenAccount: depUsdc.address,
    //     zoProgramVault: zoUsdcVault,
    //     tokenProgram: TOKEN_PROGRAM_ID,
    //   },
    // });
    // await provider.connection.confirmTransaction(tx, "finalized");

    // const fetchBalanceAfter = await provider.connection.getTokenAccountBalance(
    //   depUsdc.address
    // );
    // console.log(
    //   "user USDC balance after withdraw: ",
    //   fetchBalanceAfter.value.amount
    // );

    // await zoMargin.refresh();
    // console.log(
    //   "user Margin USDC balance after withdraw: ",
    //   zoMargin.balances.USDC.n.toString()
    // );

    // const fetchVaultBalanceAfter =
    //   await provider.connection.getTokenAccountBalance(zoUsdcVault);
    // console.log(
    //   "state vault USDC balance after withdraw: ",
    //   fetchVaultBalanceAfter.value.amount
    // );

    const ooKey = Keypair.generate();

    const tx10 = await program.rpc.createZoPerpOrder({
      accounts: {
        state: zoState.pubkey,
        stateSigner: zoMargin.state.signer,
        authority: depositor,
        margin: zoMargin.pubkey,
        control: zoControl.pubkey,
        openOrders: ooKey.publicKey,
        dexMarket: serumDexId,
        dexProgram: zoDexId,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
      },
    });

    //

    //
  });
});
