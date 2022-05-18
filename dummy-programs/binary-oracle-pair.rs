// This code is only used to generate the IDL for the binary oracle pair program.

use anchor_lang::prelude::*;

declare_id!("Fd7btgySsrjuo25CJCj7oE7VPMyezDhnx7pZkj2v69Nk");

#[program]
pub mod binary_oracle_pair {
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>, mint_end_slot: Slot, decide_end_slot: Slot, bump_seed: u8) -> Result<()> {
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn decide(ctx: Context<Decide>, decision: bool) -> Result<()> {
        Ok(())
    }
}
    

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(mut)]
    pool: AccountInfo<'info>,
    auth: AccountInfo<'info>,
    decider_auth: AccountInfo<'info>,
    deposit_currency: AccountInfo<'info>,
    #[account(mut)]
    deposit_token: AccountInfo<'info>,
    #[account(mut)]
    token_pass_mint: AccountInfo<'info>,
    #[account(mut)]
    token_fail_mint: AccountInfo<'info>,
    rent: AccountInfo<'info>,
    token_id: AccountInfo<'info>, 
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    pool: AccountInfo<'info>,
    auth: AccountInfo<'info>,
    #[account(signer)]
    user_transfer_auth: AccountInfo<'info>,
    #[account(mut)]
    token_source: AccountInfo<'info>,
    #[account(mut)]
    deposit_token: AccountInfo<'info>,
    #[account(mut)]
    token_p_pass_mint: AccountInfo<'info>,
    #[account(mut)]
    token_f_fail_mint: AccountInfo<'info>,
    #[account(mut)]
    token_p_dest: AccountInfo<'info>,
    #[account(mut)]
    token_f_dest: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    token_id: AccountInfo<'info>,
    
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pool: AccountInfo<'info>,
    auth: AccountInfo<'info>,
    #[account(signer)]
    user_transfer_auth: AccountInfo<'info>,
    #[account(mut)]
    deposit_token: AccountInfo<'info>,
    #[account(mut)]
    token_p_pass_source: AccountInfo<'info>,
    #[account(mut)]
    token_f_fail_source: AccountInfo<'info>,
    #[account(mut)]
    token_p_pass_mint: AccountInfo<'info>,
    #[account(mut)]
    token_f_fail_mint: AccountInfo<'info>,
    #[account(mut)]
    deposit_dest: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    token_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Decide<'info> {
    pool: AccountInfo<'info>,
    #[account(signer)]
    decider_pubkey: AccountInfo<'info>,
    clock: AccountInfo<'info>,
}
