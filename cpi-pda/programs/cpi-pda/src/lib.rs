use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("AnagZg5Xc8aqEFhNo1UEuggvpSLVwTVovZHuUK5QqLfG");

#[program]
pub mod cpi_pda {
    use super::*;

    pub fn sol_transfer_one(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.pda_account.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        let seed = to_pubkey.key();
        let bump_seed = ctx.bumps.pda_account;
        let signer_seeds: &[&[&[u8]]] = &[&[b"pda", seed.as_ref(), &[bump_seed]]];

        let cpi_context = CpiContext::new(
            program_id,
            Transfer {
                from: from_pubkey,
                to: to_pubkey,
            },
        ).with_signer(signer_seeds);

        transfer(cpi_context, amount)?;
        Ok(())
    }

    pub fn sol_transfer_two(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.pda_account.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        let seed = to_pubkey.key();
        let bump_seed = ctx.bumps.pda_account;
        let signer_seeds: &[&[&[u8]]] = &[&[b"pda", seed.as_ref(), &[bump_seed]]];

        let instruction =
            &system_instruction::transfer(&from_pubkey.key(), &to_pubkey.key(), amount);

        invoke_signed(instruction, &[from_pubkey, to_pubkey, program_id], signer_seeds)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(
        mut,
        seeds = [b"pda", recipient.key().as_ref()], 
        bump, 
    )]
    pda_account: SystemAccount<'info>,
    #[account(mut)]
    recipient: SystemAccount<'info>,
    system_program: Program<'info, System>,
}
