use anchor_lang::prelude::*;

// Replace with your ID: 2aLWR5XmvASspmZuYZ8YcSD2fxRsJiQwd1eWg6cxQQ2i
declare_id!("2aLWR5XmvASspmZuYZ8YcSD2fxRsJiQwd1eWg6cxQQ2i");

#[program]
pub mod guardian_hook {
    use super::*;

    // 1. Initialize the Security Guard for a token/treasury
    pub fn initialize_guard(ctx: Context<InitializeGuard>, safety_limit: u64) -> Result<()> {
        let config = &mut ctx.accounts.security_config;
        config.authority = ctx.accounts.authority.key();
        config.safety_limit = safety_limit;
        config.is_authorized = false;
        msg!(
            "Guardian initialized. Safety Limit: {} tokens.",
            safety_limit
        );
        Ok(())
    }

    // 2. The function that checks if a transfer is allowed
    // In a full Token-2022 setup, this is linked to the Transfer Hook
    pub fn check_transfer(ctx: Context<CheckTransfer>, amount: u64) -> Result<()> {
        let config = &ctx.accounts.security_config;

        if amount > config.safety_limit {
            // If the amount is too high, check if we have an override 'authorization'
            require!(config.is_authorized, ErrorCode::LimitExceeded);
            msg!("High-value transfer authorized via security override.");
        } else {
            msg!("Transfer within safety limits. Proceeding...");
        }
        Ok(())
    }

    // 3. Allow the authority to toggle authorization for the next big transfer
    pub fn toggle_auth(ctx: Context<UpdateGuard>, status: bool) -> Result<()> {
        let config = &mut ctx.accounts.security_config;
        config.is_authorized = status;
        msg!("Security authorization status updated to: {}", status);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeGuard<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 1)]
    pub security_config: Account<'info, SecurityConfig>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CheckTransfer<'info> {
    pub security_config: Account<'info, SecurityConfig>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateGuard<'info> {
    #[account(mut, has_one = authority)]
    pub security_config: Account<'info, SecurityConfig>,
    pub authority: Signer<'info>,
}

#[account]
pub struct SecurityConfig {
    pub authority: Pubkey,
    pub safety_limit: u64,
    pub is_authorized: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("GuardianHook: Transfer amount exceeds safety limit. Security override required.")]
    LimitExceeded,
}
