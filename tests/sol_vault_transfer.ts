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
  ZO_DEX_DEVNET_PROGRAM_ID,
  ZO_FUTURE_TAKER_FEE,
  ZO_OPTION_TAKER_FEE,
  ZO_SQUARE_TAKER_FEE,
} from "@zero_one/client";
import {
  getOrCreateAssociatedTokenAccount,
  getAccount,
} from "../node_modules/@solana/spl-token";

import assert from "assert";

describe("sol_vault_transfer", () => {
  // Configure the client to use the local cluster.
  const depositor = Keypair.fromSecretKey(devpair);

  const depositor2 = Keypair.fromSecretKey(devpair2);

  const depositor3 = Keypair.fromSecretKey(devpair3);

  console.log("depositor:", depositor.publicKey.toBase58());

  const provider = anchor.Provider.env();

  anchor.setProvider(provider);

  const zoDexId = new PublicKey("ZDxUi178LkcuwdxcEqsSo2E7KATH99LAAXN5LcSVMBC");

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
    const provider2 = new anchor.Provider(
      new anchor.web3.Connection("https://api.devnet.solana.com"),
      // @ts-ignore
      new anchor.Wallet(depositor2),
      {
        preflightCommitment: "confirmed",
        commitment: "confirmed",
      }
    );

    // console.log("depositor pubkey:", depositor.publicKey.toBase58());
    // console.log("depositor2 pubkey:", depositor2.publicKey.toBase58());

    const zoProgram = createProgram(provider, Cluster.Devnet);

    console.log("zo program:", zoProgram.programId.toBase58());

    console.log("---------------------------------------");

    const zoState = await State.load(zoProgram, zoStateId);

    console.log("---------------------------------------");

    const depositorMargin = await Margin.load(zoProgram, zoState);
    const depositorControl = await Control.load(
      zoProgram,
      depositorMargin.data.control
    );

    // const [zoUsdcVault, vaultInfo] = zoState.getVaultCollateralByMint(usdcMint);

    // console.log("---------------------------");
    // console.log("zo usdc vault:", zoUsdcVault.toBase58());
    // console.log("zo usdc vault info:", vaultInfo);

    const [merPda, merNonce] = await PublicKey.findProgramAddress(
      [Buffer.from("msvault")],
      program.programId
    );

    console.log("merpda:", merPda.toBase58());

    const depUsdc = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      depositor,
      usdcMint,
      depositor.publicKey
    );

    const dep2Usdc = await getOrCreateAssociatedTokenAccount(
      provider2.connection,
      depositor2,
      usdcMint,
      depositor2.publicKey
    );

    const vaultkey3 = Keypair.generate();

    // const vaultkey = new PublicKey(
    //   "EqahQki9pwbZw8b57QfHfdScgZMaqtjrA7CzFagoeyDH"
    // );

    // const vaultkey2 = new PublicKey(
    //   "3jNdwjQbuRxvNES686Xvf7QiWe2jys2TBudPR6JMWmUy"
    // );

    // console.log("vault key:", vaultkey.publicKey.toBase58());
    // console.log("vault key:", vaultkey.toBase58());

    // console.log("vault2 key:", vaultkey2.toBase58());
    // console.log("vault key:", vaultkey2.toBase58());

    // const msUsdc = await createTokenAccount(provider, usdcMint, merPda);

    const msUsdc = new PublicKey(
      "BJ2ebUEyz4diV1HFm2PZdupJnfvkNZdgnxMQVMfojYcV"
    );

    const msUsdcInfo = await getAccount(provider.connection, msUsdc);

    console.log("msUsdc account:", msUsdc.toBase58());
    console.log("msUsdc account address:", msUsdcInfo.owner.toBase58());

    console.log("---------------------------------------");

    /// Create Vault

    const tx1 = await program.rpc.createVault({
      accounts: {
        depositor: depositor.publicKey,
        vault: vaultkey3.publicKey,
        vaultTokenAcct: msUsdc,
        depositorTokenAcct: depUsdc.address,
        mint: usdcMint,
        systemProgram: SystemProgram.programId,
      },
      signers: [depositor, vaultkey3],
    });

    const vault_info = await program.account.vault.fetch(vaultkey3.publicKey);
    console.log("vault info:", vault_info.vaultTokenAccount.toBase58());

    console.log("==================================");
    // console.log("tx1:", tx1);

    /// Deposit to Vault
    const tx2 = await program.rpc.depositToVault(new anchor.BN("1000"), {
      accounts: {
        depositor: depositor.publicKey,
        depositorTokenAcct: depUsdc.address,
        vaultTokenAcct: msUsdc,
        vault: vaultkey3.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [depositor],
    });

    console.log("===================================");
    console.log("tx2:", tx2);

    // const msUsdcInfo2 = await getAccount(provider.connection, msUsdc);

    const vault_info2 = await program.account.vault.fetch(vaultkey3.publicKey);

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
    console.log(
      "compare:",
      vault_info2.vaultTokenAccount.toBase58(),
      "to",
      msUsdc.toBase58()
    );

    console.log("==================================================");

    /// Withdraw from Vault
    const tx3 = await program.rpc.withdrawFromVault(new anchor.BN("999"), {
      accounts: {
        depositor: depositor.publicKey,
        depositorTokenAcct: depUsdc.address,
        pdaAccount: merPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        vault: vaultkey3.publicKey,
        vaultTokenAcct: msUsdc,
      },
      signers: [depositor],
    });

    console.log("===================================");
    console.log("tx3:", tx3);

    const msUsdcInfo3 = await getAccount(provider.connection, msUsdc);

    const vault_info3 = await program.account.vault.fetch(vaultkey3.publicKey);

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
    // console.log("dep2Usdc:", dep2Usdc);

    // const [[margin, nonce], control, controlLamports] = await Promise.all([
    //   PublicKey.findProgramAddress(
    //     [
    //       depositor.publicKey.toBuffer(),
    //       zoState.pubkey.toBuffer(),
    //       Buffer.from("marginv1"),
    //     ],
    //     zoProgram.programId
    //   ),
    //   Keypair.generate(),
    //   provider.connection.getMinimumBalanceForRentExemption(
    //     CONTROL_ACCOUNT_SIZE
    //   ),
    // ]);

    // console.log("======================================");
    // console.log("key:", margin.toBase58());
    // console.log("nonce:", nonce);
    // console.log("control lamports", controlLamports);

    // const info = await program.provider.connection.getAccountInfo(margin);

    // if (info) {
    //   console.log("Margin account already exists");
    // } else {
    //   //calling CreateMargin through CPI call

    //   const tx = await program.rpc.createZoMargin(nonce, {
    //     accounts: {
    //       authority: depositor.publicKey,
    //       zoProgramState: zoState.pubkey,
    //       zoMargin: margin,
    //       zoProgram: zoProgram.programId,
    //       control: control.publicKey,
    //       rent: SYSVAR_RENT_PUBKEY,
    //       systemProgram: SystemProgram.programId,
    //     },
    //     preInstructions: [
    //       SystemProgram.createAccount({
    //         fromPubkey: depositor.publicKey,
    //         newAccountPubkey: control.publicKey,
    //         lamports: controlLamports,
    //         space: CONTROL_ACCOUNT_SIZE,
    //         programId: zoProgram.programId,
    //       }),
    //     ],
    //     signers: [control, depositor],
    //   });

    //   const txTwo = await provider.connection.confirmTransaction(
    //     tx,
    //     "confirmed"
    //   );

    //   console.log("tx two:", txTwo);
    // }

    //

    //

    // console.log("zo margin:", zoMargin);
    // console.log("zo control:", zoControl);
    // const markets = zoState.markets;

    // console.log("dex markets:", markets["BTC-PERP"]);

    // //deposit

    // const depositAmount = new anchor.BN("100000000000");
    // console.log("depositing amount: ", depositAmount.toString());

    // const fetchBalanceBefore = await provider.connection.getTokenAccountBalance(
    //   depUsdc.address
    // );
    // console.log(
    //   "user USDC balance before deposit: ",
    //   fetchBalanceBefore.value.amount
    // );

    // await depositorMargin.refresh();
    // console.log(
    //   "user Margin USDC balance before deposit: ",
    //   depositorMargin.balances.USDC.n.toString()
    // );

    // const fetchVaultBalanceBefore =
    //   await program.provider.connection.getTokenAccountBalance(zoUsdcVault);
    // console.log(
    //   "state vault USDC balance before deposit: ",
    //   fetchVaultBalanceBefore.value.amount
    // );

    // const tx3 = await program.rpc.zoDeposit(depositAmount, {
    //   accounts: {
    //     authority: depositor.publicKey,
    //     zoProgramState: zoState.pubkey,
    //     zoProgramMargin: depositorMargin.pubkey,
    //     zoProgram: zoProgram.programId,
    //     cache: zoState.cache.pubkey,
    //     stateSigner: depositorMargin.state.signer,
    //     tokenAccount: depUsdc.address,
    //     zoProgramVault: zoUsdcVault,
    //     tokenProgram: TOKEN_PROGRAM_ID,
    //   },
    //   signers: [depositor],
    // });

    // const tx2 = await program.provider.connection.confirmTransaction(
    //   tx3,
    //   "finalized"
    // );

    // console.log("=======================================");
    // console.log("tx2:", tx2);

    // const fetchBalanceAfter2 =
    //   await program.provider.connection.getTokenAccountBalance(depUsdc.address);
    // console.log(
    //   "user USDC balance after deposit: ",
    //   fetchBalanceAfter2.value.amount
    // );

    // await depositorMargin.refresh();

    // console.log(
    //   "user Margin USDC balance after deposit: ",
    //   depositorMargin.balances.USDC.n.toString()
    // );

    // const fetchVaultBalanceAfter3 =
    //   await program.provider.connection.getTokenAccountBalance(zoUsdcVault);
    // console.log(
    //   "state vault USDC balance after deposit: ",
    //   fetchVaultBalanceAfter3.value.amount
    // );

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

    // //
    // const balance = await provider.connection.getTokenAccountBalance(
    //   depUsdc.address
    // );

    // console.log("Balance:", balance);

    // const btcPerpMarketKey = zoState.markets["BTC-PERP"].pubKey;

    // const [openOrdersAcct] = await PublicKey.findProgramAddress(
    //   [depositorControl.pubkey.toBuffer(), btcPerpMarketKey.toBuffer()],
    //   zoDexId
    // );

    // const [ookey] = await depositorMargin.getOpenOrdersKeyBySymbol(
    //   "BTC-PERP",
    //   Cluster.Devnet
    // );

    // console.log("ookey:", ookey.toBase58());
    // console.log("ookey:", openOrdersAcct.toBase58());

    // const theOpenOrders = await depositorMargin.getOpenOrdersInfoBySymbol(
    //   "BTC-PERP"
    // );

    // console.log("open orders info:", theOpenOrders);

    // //

    // console.log("zo state signer:", zoState.signer.toBase58());

    // const tx10 = await program.rpc.createZoPerpOrder({
    //   accounts: {
    //     state: zoState.pubkey,
    //     stateSigner: depositorMargin.state.signer,
    //     authority: depositor.publicKey,
    //     margin: depositorMargin.pubkey,
    //     control: depositorControl.pubkey,
    //     openOrders: ookey,
    //     dexMarket: btcPerpMarketKey,
    //     dexProgram: ZO_DEX_DEVNET_PROGRAM_ID,
    //     rent: SYSVAR_RENT_PUBKEY,
    //     systemProgram: SystemProgram.programId,
    //     zoProgram: ZERO_ONE_DEVNET_PROGRAM_ID,
    //   },
    //   signers: [depositor],
    // });

    /// Place Perp Orders

    // const OpenOrders = await depositorMargin.getOpenOrdersInfoBySymbol(
    //   "BTC-PERP"
    // );

    // console.log("open orders info:", OpenOrders);
    // console.log("ookey:", ookey);

    // const btcPerpMarket = await zoState.getMarketBySymbol("BTC-PERP");

    // const ordertype: OrderType = { postOnly: {} };

    // const isLong = true;

    // const limitPriceBn = btcPerpMarket.priceNumberToLots(35000);

    // const maxBaseQtyBn = btcPerpMarket.baseSizeNumberToLots(1000);

    // const takerFee =
    //   btcPerpMarket.decoded.perpType.toNumber() === 1
    //     ? ZO_FUTURE_TAKER_FEE
    //     : btcPerpMarket.decoded.perpType.toNumber() === 2
    //     ? ZO_OPTION_TAKER_FEE
    //     : ZO_SQUARE_TAKER_FEE;
    // const feeMultiplier = isLong ? 1 + takerFee : 1 - takerFee;
    // const maxQuoteQtyBn = new anchor.BN(
    //   Math.round(
    //     limitPriceBn
    //       .mul(maxBaseQtyBn)
    //       .mul(btcPerpMarket.decoded["quoteLotSize"])
    //       .toNumber() * feeMultiplier
    //   )
    // );

    // const tx11 = await program.rpc.placeZoPerpOrder(
    //   isLong,
    //   limitPriceBn,
    //   maxBaseQtyBn,
    //   maxQuoteQtyBn,
    //   ordertype,
    //   10,
    //   new anchor.BN("0"),
    //   {
    //     accounts: {
    //       state: depositorMargin.state.pubkey,
    //       stateSigner: depositorMargin.state.signer,
    //       authority: depositor.publicKey,
    //       cache: depositorMargin.state.cache.pubkey,
    //       control: depositorControl.pubkey,
    //       eventQ: btcPerpMarket.eventQueueAddress,
    //       reqQ: btcPerpMarket.requestQueueAddress,
    //       margin: depositorMargin.pubkey,
    //       marketAsks: btcPerpMarket.asksAddress,
    //       marketBids: btcPerpMarket.bidsAddress,
    //       openOrders: ookey,
    //       rent: SYSVAR_RENT_PUBKEY,
    //       dexMarket: btcPerpMarketKey,
    //       dexProgram: ZO_DEX_DEVNET_PROGRAM_ID,
    //       zoProgram: ZERO_ONE_DEVNET_PROGRAM_ID,
    //     },
    //   }
    // );

    // console.log("======================================");
    // console.log("tx11:", tx11);

    // const theOpenOrders3 = await depositorMargin.getOpenOrdersInfoBySymbol(
    //   "BTC-PERP"
    // );

    // console.log("open orders info:", theOpenOrders3);

    //

    //
  });
});
