// This code is only used to generate the IDL for the token swap program.

use anchor_lang::prelude::*;

declare_id!("SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8");

#[program]
pub mod token_swap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fees: Fees, swap_curve: SwapCurve) -> Result<()> {
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
        Ok(())
    }

    pub fn deposit_all_token_types(ctx: Context<DepositAllTokenTypes>, pool_token_amount: u64, maximum_token_a_amount: u64, maximum_token_b_amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn withdraw_all_token_types(ctx: Context<WithdrawAllTokenTypes>, pool_token_amount: u64, minimum_token_a_amount: u64, minimum_token_b_amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn deposit_single_token_type_exact_amount_in(ctx: Context<DepositSingleTokenTypeExactAmountIn>, source_token_amount: u64, minimum_pool_token_amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn withraw_single_token_type_exact_amount_out(ctx: Context<WithdrawSingleTokenTypeExactAmountOut>, destination_token_amount: u64, maximum_pool_token_amount: u64) -> Result<()> {
        Ok(())
    }
}
    

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut, signer)]
    new_token_swap: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_a_account: AccountInfo<'info>,
    token_b_account: AccountInfo<'info>,
    #[account(mut)]
    pool_token_mint: AccountInfo<'info>,
    pool_token_account_a: AccountInfo<'info>,
    #[account(mut)]
    pool_token_account_b: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    swap: AccountInfo<'info>,
    swap_authority: AccountInfo<'info>,
    transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    source: AccountInfo<'info>,
    #[account(mut)]
    account_into: AccountInfo<'info>,
    #[account(mut)]
    account_from: AccountInfo<'info>,
    #[account(mut)]
    destination: AccountInfo<'info>,
    #[account(mut)]
    pool_token_mint: AccountInfo<'info>,
    #[account(mut)]
    fee_account: AccountInfo<'info>,
    program_id: AccountInfo<'info>,
    /* OPTIONAL ACCOUNTS **THESE DO NOT APPEAR IN THE IDL**
    #[account(mut)]
    host_fee_account: AccountInfo<'info>,
    */
    remaining_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositAllTokenTypes<'info> {
    swap: AccountInfo<'info>,
    swap_authority: AccountInfo<'info>,
    transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    token_a_transfer_auth: AccountInfo<'info>,
    #[account(mut)]
    token_b_transfer_auth: AccountInfo<'info>,
    #[account(mut)]
    token_a_account: AccountInfo<'info>,
    #[account(mut)]
    token_b_account: AccountInfo<'info>,
    #[account(mut)]
    pool_mint_account: AccountInfo<'info>,
    #[account(mut)]
    pool_deposit_account: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawAllTokenTypes<'info> {
    swap: AccountInfo<'info>,
    swap_authority: AccountInfo<'info>,
    transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    pool_mint_account: AccountInfo<'info>,
    #[account(mut)]
    pool_source_account: AccountInfo<'info>,
    #[account(mut)]
    token_a_swap_account: AccountInfo<'info>,
    #[account(mut)]
    token_b_swap_account: AccountInfo<'info>,
    #[account(mut)]
    token_a_user_account: AccountInfo<'info>,
    #[account(mut)]
    token_b_user_account: AccountInfo<'info>,
    #[account(mut)]
    fee_account: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositSingleTokenTypeExactAmountIn<'info> {
    swap: AccountInfo<'info>,
    swap_authority: AccountInfo<'info>,
    transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    token_a_source_account: AccountInfo<'info>,
    #[account(mut)]
    token_a_swap_account: AccountInfo<'info>,
    #[account(mut)]
    token_b_swap_account: AccountInfo<'info>,
    #[account(mut)]
    pool_mint_account: AccountInfo<'info>,
    #[account(mut)]
    pool_deposit_account: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawSingleTokenTypeExactAmountOut<'info> {
    swap: AccountInfo<'info>,
    swap_authority: AccountInfo<'info>,
    transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    pool_mint_account: AccountInfo<'info>,
    #[account(mut)]
    pool_source_account: AccountInfo<'info>,
    #[account(mut)]
    token_a_swap_account: AccountInfo<'info>,
    #[account(mut)]
    token_b_swap_account: AccountInfo<'info>,
    #[account(mut)]
    credit_account: AccountInfo<'info>,
    #[account(mut)]
    fee_account: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
}
