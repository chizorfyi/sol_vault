import * as anchor from "@project-serum/anchor";
import { IdlAccounts, Program } from "@project-serum/anchor";
import { SolVaultTransfer } from "../target/types/sol_vault_transfer";
import {
  PublicKey,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  createAccount,
  createAssociatedTokenAccount,
  getOrCreateAssociatedTokenAccount,
  createMint,
  getAccount,
  getMint,
  mintToChecked,
  transferChecked,
} from "../node_modules/@solana/spl-token";
// import * as spltoken from "@solana/spl-token";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { devpair } from "../keypair";
import {
  Cluster,
  CONTROL_ACCOUNT_SIZE,
  createProgram,
  State,
} from "@zero_one/client";
import { bs58 } from "@project-serum/anchor/dist/cjs/utils/bytes";

type UserBank = IdlAccounts<SolVaultTransfer>["userBank"];
type Vault = IdlAccounts<SolVaultTransfer>["vault"];

describe("sol_vault_transfer", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const zoPid = new PublicKey("Zo1ThtSHMh9tZGECwBDL81WJRL6s3QTHf733Tyko7KQ");

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

    const depositor = Keypair.fromSecretKey(devpair);

    const rawKeypair = Keypair.generate();
    const vault = Keypair.generate();

    const userBank = Keypair.generate();

    const provider = program.provider;

    console.log("depositor pubkey:", depositor.publicKey);

    const zoProgram = createProgram(provider, Cluster.Devnet);

    console.log("zo program:", zoProgram);

    console.log("---------------------------------------");

    const zoState = await State.load(zoProgram, zoStateId);
    console.log("zo state:", zoState);

    console.log("---------------------------------------");

    const zoUsdcVault = zoState.getVaultCollateralByMint(usdcMint)[0];

    console.log("---------------------------");
    console.log("zo usdc vault:", zoUsdcVault);

    const depUsdc = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      depositor,
      usdcMint,
      depositor.publicKey
    );

    const vaultUsdc = await createAccount(
      provider.connection,
      depositor,
      usdcMint,
      depositor.publicKey,
      rawKeypair
    );

    const depUsdcAcct = await getAccount(provider.connection, depUsdc.address);
    const vaultUsdcAcct = await getAccount(provider.connection, vaultUsdc);

    console.log("---------------------------------------");
    console.log("depositor usdc acct:", depUsdcAcct);
    console.log("vault usdc acct:", vaultUsdcAcct);

    const txFive = await program.rpc.createUserBank({
      accounts: {
        userBank: userBank.publicKey,
        depositor: depositor.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [depositor, userBank],
    });

    console.log("===========================================");
    console.log("tx five:", txFive);

    const txSix = await program.rpc.depositToVault(new anchor.BN("500"), {
      accounts: {
        depositor: depositor.publicKey,
        depositorTokenAcct: depUsdc.address,
        vaultTokenAcct: vaultUsdc,
        vault: vault.publicKey,
        userBank: userBank.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      },
      signers: [vault, depositor],
    });

    const [pdaAccount, bump] = await findProgramAddressSync(
      [depositor.publicKey.toBuffer()],
      program.programId
    );

    console.log("===========================================");
    console.log("tx five:", txFive);
    console.log("pda:", pdaAccount);

    const some = bs58.decode(pdaAccount.toString());
    //create Margin

    const [[key, nonce], control, controlLamports] = await Promise.all([
      PublicKey.findProgramAddress(
        [
          pdaAccount.toBuffer(),
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
    console.log("key:", key);

    if (await program.provider.connection.getAccountInfo(key)) {
      console.log("Margin account already exists");
    } else {
      //calling CreateMaergin through CPI call
      const tx = await program.rpc.createZoMargin(nonce, {
        accounts: {
          authority: depositor,
          zoProgramState: zoState.pubkey,
          zoMargin: key,
          zoProgram: zoPid,
          control: control.publicKey,
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
        },
        preInstructions: [
          SystemProgram.createAccount({
            fromPubkey: pdaAccount,
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
        "finalized"
      );

      console.log("tx two:", txTwo);
    }

    //

    //
  });
});
