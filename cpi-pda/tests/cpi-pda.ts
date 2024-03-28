import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { CpiPda } from "../target/types/cpi_pda";
import {
  PublicKey,
  SystemProgram,
  LAMPORTS_PER_SOL,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";

describe("cpi-pda", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CpiPda as Program<CpiPda>;
  const connection = program.provider.connection;

  const wallet = provider.wallet as anchor.Wallet;
  const [PDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("pda"), wallet.publicKey.toBuffer()],
    program.programId
  );

  const transferAmount = 0.1 * LAMPORTS_PER_SOL;

  it("Fund PDA with SOL", async () => {
    const transferInstruction = SystemProgram.transfer({
      fromPubkey: wallet.publicKey,
      toPubkey: PDA,
      lamports: transferAmount,
    });

    const transaction = new Transaction().add(transferInstruction);

    const transactionSignature = await sendAndConfirmTransaction(
      connection,
      transaction,
      [wallet.payer] // signer
    );

    console.log("Transaction Signature:", transactionSignature);
  });

  it("PDA SOL Transfer Anchor", async () => {
    const transactionSignature = await program.methods
      .solTransferOne(new BN(transferAmount / 2))
      .accounts({
        pdaAccount: PDA,
        recipient: wallet.publicKey,
      })
      .rpc();

    console.log("Transaction Signature:", transactionSignature);
  });

  it("PDA SOL Transfer invoke_signed", async () => {
    const transactionSignature = await program.methods
      .solTransferTwo(new BN(transferAmount / 2))
      .accounts({
        pdaAccount: PDA,
        recipient: wallet.publicKey,
      })
      .rpc();

    console.log("Transaction Signature:", transactionSignature);
  });
});
