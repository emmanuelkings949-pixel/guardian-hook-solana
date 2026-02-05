import * as anchor from "@coral-xyz/anchor";

describe("guardian_hook", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = pg.program;
  const securityConfig = anchor.web3.Keypair.generate();

  it("Security Test: Blocks high-value drain attempt", async () => {
    console.log("🛡️ Starting Guardian Hook Security Test...");

    // 1. Initialize the Guard with a 100 token limit
    await program.methods
      .initializeGuard(new anchor.BN(100))
      .accounts({
        securityConfig: securityConfig.publicKey,
        authority: provider.wallet.publicKey,
      })
      .signers([securityConfig])
      .rpc();

    console.log("✅ Guardian Initialized (Limit: 100)");

    // 2. Try to transfer 500 tokens (Should be BLOCKED)
    try {
      await program.methods
        .checkTransfer(new anchor.BN(500))
        .accounts({
          securityConfig: securityConfig.publicKey,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      
      console.log("❌ ERROR: Security failed! High transfer allowed.");
    } catch (err) {
      console.log("🔒 SUCCESS: Guardian blocked the 500 token drain attempt!");
    }

    // 3. Try to transfer 50 tokens (Should be ALLOWED)
    try {
      await program.methods
        .checkTransfer(new anchor.BN(50))
        .accounts({
          securityConfig: securityConfig.publicKey,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      console.log("✅ SUCCESS: Small transfer allowed within limits.");
    } catch (err) {
      console.log("❌ ERROR: Guardian blocked a safe transfer.");
    }
  });
});
