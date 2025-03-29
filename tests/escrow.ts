import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import BN from "bn.js";
import { expect } from "chai";

describe("escrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.local());

  const program = anchor.workspace.Escrow as Program<Escrow>;
  const recipient = Keypair.generate();

  const paymentIndex = new BN(0);

    const [lockup] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("lockup"),
        paymentIndex.toArrayLike(Buffer, "le", 8)
      ],
      program.programId
    );

    const [condition] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("condition"),
        paymentIndex.toArrayLike(Buffer, "le", 8)
      ],
      program.programId
    );

  it("Initializes payment", async () => {
    await program
      .methods
      .initializePayment(
        paymentIndex,
        "placeholder",
        new BN(1 * LAMPORTS_PER_SOL)
      )
      .accounts({
        systemProgram: SystemProgram.programId,
        condition,
        lockup,
        payer: program.provider.publicKey,
        recipient: recipient.publicKey
      })
      .rpc();

      const lockupBalance = await program
        .provider
        .connection
        .getBalance(lockup);

      expect(lockupBalance).eq(1 * LAMPORTS_PER_SOL);

      const {
        content,
        payer,
        recipient: setRecipient
      } = await program
        .account
        .condition
        .fetch(condition);
      
      expect(content).eq("placeholder");
      expect(payer.toString()).eq(program.provider.publicKey.toString());
      expect(setRecipient.toString()).eq(recipient.publicKey.toString());
  });

  it("Finalizes payment", async () => {
    await program
      .methods
      .finalizePayment(
        paymentIndex
      )
      .accounts({
        condition,
        lockup,
        payer: program.provider.publicKey,
        recipient: recipient.publicKey,
        systemProgram: SystemProgram.programId
      })
      .rpc();

      const lockupBalance = await program
        .provider
        .connection
        .getBalance(lockup);

      expect(lockupBalance).eq(0);

      const recipientBalance = await program
        .provider
        .connection
        .getBalance(recipient.publicKey);

      expect(recipientBalance).eq(1 * LAMPORTS_PER_SOL);

      let conditionNotFound = false;
      const conditionAccount = await program
        .account
        .condition
        .fetch(condition)
        .catch(err => conditionNotFound = true);

      expect(conditionNotFound).eq(true);
  });
});
