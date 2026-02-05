# GuardianHook 🛡️
**Solana Security Primitive for Asset Protection**

GuardianHook is a smart contract designed to prevent wallet drains by enforcing on-chain transfer limits. 

### Features
- **Programmable Safety Limits:** Rejects any transaction exceeding a pre-set threshold.
- **Authority Override:** Allows a designated security authority to toggle authorization for large, legitimate moves.
- **Devnet Verified:** Successfully tested to block unauthorized drain attempts.

### Program Info
- **Program ID:** `2aLWR5XmvASspmZuYZ8YcSD2fxRsJiQwd1eWg6cxQQ2i`
- **Network:** Solana Devnet
- **Framework:** Anchor 0.30.1

### Security Logic
The program checks every `check_transfer` call. If the amount > `safety_limit`, it requires `is_authorized` to be true, or the transaction fails with `LimitExceeded`.# guardian-hook-solana
