use anchor_lang::prelude::*;

declare_id!("CCtBeJGySCrjtXrVMeqYVgG4NKEPDqKPqPm72Vo8vjk4");

#[program]
pub mod pda_account {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let pda_account = &mut ctx.accounts.pda_account;
        pda_account.user = *ctx.accounts.user.key;
        pda_account.bump = ctx.bumps.pda_account;

        msg!("Initialized PDA account for user: {:?}", pda_account.user);
        msg!("Bump seed: {:?}", pda_account.bump);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        seeds = [b"user", user.key().as_ref()], 
        bump,                                  
        payer = user,
        space = 8 + DataAccount::INIT_SPACE
    )]
    pub pda_account: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct DataAccount {
    pub user: Pubkey,
    pub bump: u8,     
}
