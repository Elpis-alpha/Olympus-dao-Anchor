import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import chalk from "chalk";
import { OylpusDaoSmartContract as ODSC } from "../target/types/oylpus_dao_smart_contract";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { createMint, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { delay, safeAirdrop } from "./utils/utils";
import { assert } from "chai";

describe("oylpus-dao-smart-contract", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.AnchorProvider.env();
  console.log(anchor.workspace);
  const program = anchor.workspace.OylpusDaoSmartContract as Program<ODSC>;
  const payer = anchor.web3.Keypair.generate();

  let tokenMint: anchor.web3.PublicKey;
  let treasuryState: anchor.web3.PublicKey;

  let [treasuryAuthority] = PublicKey.findProgramAddressSync(
    [Buffer.from("treasury_authority_seed")],
    program.programId
  );

  let [treasuryVault] = PublicKey.findProgramAddressSync(
    [treasuryAuthority.toBuffer(), Buffer.from("treasury_seed")],
    program.programId
  );

  const [treasuryVaultLamports, treasuryVaultLamportsBump] =
    PublicKey.findProgramAddressSync(
      [treasuryAuthority.toBuffer(), Buffer.from("treasury_lamports_seed")],
      program.programId
    );

  it("create mint and treasury state", async () => {
    await safeAirdrop(payer.publicKey, provider.connection);
    await safeAirdrop(treasuryAuthority, provider.connection);
    delay(2000);

    tokenMint = await createMint(
      provider.connection,
      payer,
      treasuryAuthority,
      undefined,
      6,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );
    console.log("tokenMint", tokenMint.toBase58());

    // treasury state pad
    [treasuryState] = PublicKey.findProgramAddressSync(
      [tokenMint.toBuffer(), Buffer.from("treasury_state_seed")],
      program.programId
    );
    console.log("treasuryState", treasuryState.toBase58());
  });

  it("Is initialized!", async () => {
    try {
      console.table([
        { name: "payer", value: payer.publicKey.toBase58().substring(0, 15) },
        {
          name: "treasuryAuthority",
          value: treasuryAuthority.toBase58().substring(0, 15),
        },
        { name: "tokenMint", value: tokenMint.toBase58().substring(0, 15) },
        {
          name: "treasuryState",
          value: treasuryState.toBase58().substring(0, 15),
        },
        {
          name: "treasuryVault",
          value: treasuryVault.toBase58().substring(0, 15),
        },
        {
          name: "treasuryVaultLamports",
          value: treasuryVaultLamports.toBase58().substring(0, 15),
        },
      ]);

      const tx = await program.methods
        .initializeDapp()
        .accounts({
          treasuryAuthority,
          tokenMint,
          treasuryState,
          treasuryVault,
          treasuryVaultLamports,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          payer: payer.publicKey,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([payer])
        .rpc();

      console.log("\nYour transaction signature");
      explorerLinkLog(tx, "tx", "testnet");

      const {
        tokenMint: _tokenMint,
        treasuryAuthority: _treasuryAuthority,
        treasuryVault: _treasuryVault,
        treasuryVaultLamports: _treasuryVaultLamports,
      } = await program.account.treasuryState.fetch(treasuryState);

      // assert vals
      assert.equal(_tokenMint.toBase58(), tokenMint.toBase58());
      assert.equal(_treasuryAuthority.toBase58(), treasuryAuthority.toBase58());
      assert.equal(_treasuryVault.toBase58(), treasuryVault.toBase58());
      assert.equal(
        _treasuryVaultLamports.toBase58(),
        treasuryVaultLamports.toBase58()
      );
    } catch (error) {
      // console.error(error);
      if (error.getLogs) console.warn(await error.getLogs());
      throw new Error(error);
    }
  });
});

type explorerLinkType = "tx" | "address" | "token" | "block";
export const explorerLinkLog = (
  signature: string,
  type: explorerLinkType = "tx",
  network = "devnet"
) => {
  console.log(
    chalk
      .hex("#478be6")
      .underline(
        `https://explorer.solana.com/${type}/${signature}?cluster=${network}`
      ) + ";"
  );
};
