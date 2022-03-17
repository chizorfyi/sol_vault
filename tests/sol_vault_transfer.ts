import * as anchor from "@project-serum/anchor";
import { IdlAccounts, IdlTypes, Program } from "@project-serum/anchor";
import { SolVaultTransfer } from "../target/types/sol_vault_transfer";
import {
  PublicKey,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";

import * as spltoken from "@solana/spl-token";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { devpair, devpair2, devpair3 } from "../keypair";
import {
  Cache,
  Cluster,
  Control,
  CONTROL_ACCOUNT_SIZE,
  createProgram,
  createTokenAccount,
  Margin,
  OrderType,
  State,
  TOKEN_PROGRAM_ID,
  ZERO_ONE_DEVNET_PROGRAM_ID,
  ZO_DEVNET_STATE_KEY,
  ZO_DEX_DEVNET_PROGRAM_ID,
  ZO_FUTURE_TAKER_FEE,
  ZO_OPTION_TAKER_FEE,
  ZO_SQUARE_TAKER_FEE,
  ZO_STATE_KEY,
} from "@zero_one/client";
import {
  getOrCreateAssociatedTokenAccount,
  getAccount,
} from "../node_modules/@solana/spl-token";

import assert from "assert";
import MarginWeb3 from "@zero_one/client/dist/esm/accounts/margin/MarginWeb3";

describe("sol_vault_transfer", () => {
  // Configure the client to use the local cluster.

  const depositor = Keypair.fromSecretKey(devpair);

  const depositor2 = Keypair.fromSecretKey(devpair2);

  const depositor3 = Keypair.fromSecretKey(devpair3);

  console.log("depositor:", depositor.publicKey.toBase58());

  const provider = anchor.Provider.env();

  anchor.setProvider(provider);

  const program = anchor.workspace
    .SolVaultTransfer as Program<SolVaultTransfer>;

  // const zoDexId = new PublicKey("ZDxUi178LkcuwdxcEqsSo2E7KATH99LAAXN5LcSVMBC");

  // const zoStateId = new PublicKey(
  //   "KwcWW7WvgSXLJcyjKZJBHLbfriErggzYHpjS9qjVD5F"
  // );
  const usdcMint = new PublicKey(
    "7UT1javY6X1M9R2UrPGrwcZ78SX3huaXyETff5hm5YdX"
  );

  const msUsdc = new PublicKey("BJ2ebUEyz4diV1HFm2PZdupJnfvkNZdgnxMQVMfojYcV");

  const zoProgram = createProgram(provider, Cluster.Devnet);

  it("deposits to vault", async () => {
    // Add your test here.

    // const provider2 = new anchor.Provider(
    //   new anchor.web3.Connection("https://api.devnet.solana.com"),
    //   // @ts-ignore
    //   new anchor.Wallet(depositor2),
    //   {
    //     preflightCommitment: "confirmed",
    //     commitment: "confirmed",
    //   }
    // );

    const [merPda, merNonce] = await PublicKey.findProgramAddress(
      [Buffer.from("msvault")],
      program.programId
    );

    const depUsdc = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      depositor,
      usdcMint,
      depositor.publicKey
    );

    const [vaultkey] = await PublicKey.findProgramAddress(
      [depositor.publicKey.toBuffer(), Buffer.from("vault")],
      program.programId
    );

    // const msUsdc = await createTokenAccount(provider, usdcMint, merPda);

    // const msUsdc = new PublicKey(
    //   "BJ2ebUEyz4diV1HFm2PZdupJnfvkNZdgnxMQVMfojYcV"
    // );

    const vInfo = provider.connection.getAccountInfo(vaultkey);

    if (!vInfo) {
      /// Create Vault

      const tx1 = await program.rpc.createVault({
        accounts: {
          depositor: depositor.publicKey,
          vault: vaultkey,
          depositorTokenAcct: depUsdc.address,
          vaultTokenAcct: msUsdc,
          systemProgram: SystemProgram.programId,
        },
      });

      const vault_info = await program.account.vault.fetch(vaultkey);
      console.log("vault info:", vault_info.vaultTokenAccount.toBase58());

      console.log("==================================");
      console.log("tx1:", tx1);
    }

    /// Deposit to Vault
    const tx2 = await program.rpc.depositToVault(new anchor.BN("100000000"), {
      accounts: {
        depositor: depositor.publicKey,
        depositorTokenAcct: depUsdc.address,
        vaultTokenAcct: msUsdc,
        vault: vaultkey,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [depositor],
    });

    console.log("===================================");
    console.log("tx2:", tx2);

    // const msUsdcInfo2 = await getAccount(provider.connection, msUsdc);

    const vault_info2 = await program.account.vault.fetch(vaultkey);

    const balance = await provider.connection.getTokenAccountBalance(
      depUsdc.address
    );
    const msbalance = await provider.connection.getTokenAccountBalance(msUsdc);

    console.log("vault token acct info after transfer:", msbalance);
    console.log("depositor token acct info after transfer:", balance);
    console.log(
      "vault info after transfer:",
      vault_info2.vaultAmount.toNumber()
    );

    // console.log("depositor pubkey:", depositor.publicKey.toBase58());
    // console.log("depositor2 pubkey:", depositor2.publicKey.toBase58());
  });

  it("withdraws from vault", async () => {
    /// Withdraw from Vault

    const [merPda, merNonce] = await PublicKey.findProgramAddress(
      [Buffer.from("msvault")],
      program.programId
    );

    const depUsdc = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      depositor,
      usdcMint,
      depositor.publicKey
    );

    const [vaultkey] = await PublicKey.findProgramAddress(
      [depositor.publicKey.toBuffer(), Buffer.from("vault")],
      program.programId
    );
    const tx3 = await program.rpc.withdrawFromVault(new anchor.BN("999"), {
      accounts: {
        depositor: depositor.publicKey,
        depositorTokenAcct: depUsdc.address,
        pdaAccount: merPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        vault: vaultkey,
        vaultTokenAcct: msUsdc,
      },
      signers: [depositor],
    });

    console.log("===================================");
    console.log("tx3:", tx3);
    const msUsdcInfo3 = await getAccount(provider.connection, msUsdc);
    const vault_info3 = await program.account.vault.fetch(vaultkey);
    const balance2 = await provider.connection.getTokenAccountBalance(
      depUsdc.address
    );
    const msbalance2 = await provider.connection.getTokenAccountBalance(msUsdc);
    console.log("vault token acct info after transfer:", msbalance2);
    console.log("depositor token acct info after withdrawal:", balance2);
    // console.log("vault token acct info after withdrawal:", msUsdcInfo3);
    console.log(
      "vault info after withdrawal:",
      vault_info3.vaultAmount.toNumber()
    );
    console.log("depUsdc:", depUsdc);
  });

  it("creates a zo margin account", async () => {
    const zoState = await State.load(zoProgram, ZO_DEVNET_STATE_KEY);
    console.log("zo state key:", ZO_DEVNET_STATE_KEY);

    const [merPda, merNonce] = await PublicKey.findProgramAddress(
      [Buffer.from("msvault")],
      program.programId
    );

    const [[marginKey, nonce], control, controlLamports] = await Promise.all([
      PublicKey.findProgramAddress(
        [merPda.toBuffer(), zoState.pubkey.toBuffer(), Buffer.from("marginv1")],
        zoProgram.programId
      ),
      Keypair.generate(),
      provider.connection.getMinimumBalanceForRentExemption(
        CONTROL_ACCOUNT_SIZE
      ),
    ]);

    console.log("======================================");
    console.log("key:", marginKey.toBase58());
    console.log("control lamports", controlLamports);

    const info = await program.provider.connection.getAccountInfo(marginKey);

    if (info) {
      console.log("Margin account already exists");
    } else {
      //calling CreateMargin through CPI call

      const tx = await program.rpc.createZoMargin(nonce, {
        accounts: {
          authority: merPda,
          payer: depositor.publicKey,
          zoProgramState: zoState.pubkey,
          zoMargin: marginKey,
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

      console.log("tx two:", tx);
    }
  });

  // // Deposit to ZO margin account

  it("deposits to margin account", async () => {
    const zoState = await State.load(zoProgram, ZO_DEVNET_STATE_KEY);
    console.log("zo state key:", ZO_DEVNET_STATE_KEY.toBase58());

    const [zoUsdcVault, vaultInfo] = zoState.getVaultCollateralByMint(usdcMint);

    const depUsdc = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      depositor,
      usdcMint,
      depositor.publicKey
    );

    const [merPda] = await PublicKey.findProgramAddress(
      [Buffer.from("msvault")],
      program.programId
    );

    const [marginKey] = await Margin.getMarginKey(zoState, merPda, zoProgram);
    const depositorMargin = await Margin.load(
      zoProgram,
      zoState,
      zoState.cache,
      merPda
    );
    const depositorControl = await Control.load(
      zoProgram,
      depositorMargin.data.control
    );

    console.log("Margin authority:", depositorMargin.data.authority.toBase58());
    console.log("Margin authority:", depositorMargin.owner.toBase58());

    const depositAmount = new anchor.BN("10000000");
    console.log("depositing amount: ", depositAmount.toString());

    const fetchBalanceBefore =
      await program.provider.connection.getTokenAccountBalance(msUsdc);

    const fetchBalanceBeforeDep =
      await program.provider.connection.getTokenAccountBalance(depUsdc.address);
    console.log(
      "user USDC balance before deposit: ",
      fetchBalanceBeforeDep.value.amount
    );

    await depositorMargin.refresh();
    console.log(
      "dep Margin USDC balance before deposit: ",
      depositorMargin.balances.USDC.n.toString()
    );

    const fetchVaultBalanceBefore =
      await program.provider.connection.getTokenAccountBalance(zoUsdcVault);
    console.log(
      "state vault USDC balance before deposit: ",
      fetchVaultBalanceBefore.value.amount
    );

    const tx3 = await program.rpc.zoDeposit(false, depositAmount, {
      accounts: {
        authority: merPda,
        zoProgramState: zoState.pubkey,
        zoProgramMargin: depositorMargin.pubkey,
        zoProgram: zoProgram.programId,
        cache: zoState.cache.pubkey,
        stateSigner: depositorMargin.state.signer,
        tokenAccount: msUsdc,
        zoProgramVault: zoUsdcVault,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      // signers: [depositor3],
    });
    //  await ts.program.provider.connection.confirmTransaction(tx, "finalized");

    const tx2 = await program.provider.connection.confirmTransaction(
      tx3,
      "finalized"
    );

    console.log("=======================================");
    console.log("tx2:", tx2);

    const fetchBalanceAfter2 =
      await program.provider.connection.getTokenAccountBalance(msUsdc);
    console.log(
      "user USDC balance after deposit: ",
      fetchBalanceAfter2.value.amount
    );
    const fetchBalanceAfter2Dep =
      await program.provider.connection.getTokenAccountBalance(depUsdc.address);
    console.log(
      "dep USDC balance after deposit: ",
      fetchBalanceAfter2Dep.value.amount
    );

    await depositorMargin.refresh();

    console.log(
      "user Margin USDC balance after deposit: ",
      depositorMargin.balances.USDC.n.toString()
    );

    const fetchVaultBalanceAfter3 =
      await program.provider.connection.getTokenAccountBalance(zoUsdcVault);
    console.log(
      "state vault USDC balance after deposit: ",
      fetchVaultBalanceAfter3.value.amount
    );
  });

  /// // WIthdraw from Zo margin account

  it("withdraws from margin account", async () => {
    const zoState = await State.load(zoProgram, ZO_DEVNET_STATE_KEY);
    console.log("zo state key:", ZO_DEVNET_STATE_KEY.toBase58());

    const [zoUsdcVault, vaultInfo] = zoState.getVaultCollateralByMint(usdcMint);

    const [merPda] = await PublicKey.findProgramAddress(
      [Buffer.from("msvault")],
      program.programId
    );

    const [marginKey] = await Margin.getMarginKey(zoState, merPda, zoProgram);
    const depositorMargin = await Margin.load(
      zoProgram,
      zoState,
      zoState.cache,
      merPda
    );
    const depositorControl = await Control.load(
      zoProgram,
      depositorMargin.data.control
    );

    console.log("Margin authority:", depositorMargin.data.authority.toBase58());
    console.log("Margin authority:", depositorMargin.owner.toBase58());

    const withdrawAmount = new anchor.BN("9990000");
    console.log("withdraw amount: ", withdrawAmount.toString());

    const fetchBalanceBefore =
      await program.provider.connection.getTokenAccountBalance(msUsdc);

    console.log(
      "user USDC balance after withdrawal: ",
      fetchBalanceBefore.value.amount
    );

    await depositorMargin.refresh();

    console.log(
      "dep Margin USDC balance before withdrawal: ",
      depositorMargin.balances.USDC.n.toString()
    );

    const fetchVaultBalanceBefore =
      await program.provider.connection.getTokenAccountBalance(zoUsdcVault);
    console.log(
      "state vault USDC balance before withdrawal: ",
      fetchVaultBalanceBefore.value.amount
    );

    const tx3 = await program.rpc.zoWithdrawal(false, withdrawAmount, {
      accounts: {
        authority: merPda,
        zoProgramState: zoState.pubkey,
        zoProgramMargin: depositorMargin.pubkey,
        zoProgram: zoProgram.programId,
        cache: zoState.cache.pubkey,
        control: depositorControl.pubkey,
        stateSigner: depositorMargin.state.signer,
        tokenAccount: msUsdc,
        zoProgramVault: zoUsdcVault,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      // signers: [depositor3],
    });
    //  await ts.program.provider.connection.confirmTransaction(tx, "finalized");

    const tx2 = await program.provider.connection.confirmTransaction(
      tx3,
      "finalized"
    );

    console.log("=======================================");
    console.log("tx2:", tx2);

    const fetchBalanceAfter2 =
      await program.provider.connection.getTokenAccountBalance(msUsdc);
    console.log(
      "user USDC balance after withdrawal: ",
      fetchBalanceAfter2.value.amount
    );

    await depositorMargin.refresh();

    console.log(
      "user Margin USDC balance after withdrawal: ",
      depositorMargin.balances.USDC.n.toString()
    );

    const fetchVaultBalanceAfter3 =
      await program.provider.connection.getTokenAccountBalance(zoUsdcVault);
    console.log(
      "state vault USDC balance after withdrawal: ",
      fetchVaultBalanceAfter3.value.amount
    );
  });
});
