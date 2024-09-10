import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import chalk from "chalk";
import { OylpusDaoSmartContract } from "../target/types/oylpus_dao_smart_contract";
import { PublicKey } from "@solana/web3.js";

describe("oylpus-dao-smart-contract", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .OylpusDaoSmartContract as Program<OylpusDaoSmartContract>;

  // derive program authority PDA
  let [treasuryAuthority, treasuryAuthBump] =
    await PublicKey.findProgramAddress(
      [Buffer.from("treasury_authority_seed")],
      program.programId
    );

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initializeDapp()
      .accounts({ treasuryAuthority: treasuryAuthority })
      .rpc();

    console.log("\nYour transaction signature");
    explorerLinkLog(tx, "tx", "testnet");
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
