import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PdaAccount } from "../target/types/pda_account";
import { PublicKey } from "@solana/web3.js";

describe("pda-account", () => {
  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.PdaAccount as Program<PdaAccount>;

  const user = provider.wallet as anchor.Wallet;

  // Derive the PDA address using the seeds expected by the program
  const [PDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("user"), user.publicKey.toBuffer()],
    program.programId
  );

  it("Is initialized!", async () => {
    // Initialize the account
    const transactionSignature = await program.methods
      .initialize()
      .accounts({
        user: user.publicKey,
        pdaAccount: PDA,
      })
      .rpc();

    // Confirm the transaction
    await program.provider.connection.confirmTransaction(
      transactionSignature,
      "confirmed"
    );
    // Fetch the transaction details
    const transactionResponse =
      await program.provider.connection.getTransaction(transactionSignature, {
        commitment: "confirmed",
      });
    console.log(transactionResponse.meta.logMessages);

    // Fetch the created account
    const pdaAccount = await program.account.dataAccount.fetch(PDA);
    console.log(JSON.stringify(pdaAccount, null, 2));
  });
});
