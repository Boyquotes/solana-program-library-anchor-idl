// This code is only used to generate the IDL for the token lending program.

use anchor_lang::prelude::*;

declare_id!("6TvznH3B2e3p2mbhufNBpgSrLx6UkgvxtVQvopEZ2kuH");

#[program]
pub mod token_lending {
    use super::*;

    pub fn init_lending_market(ctx: Context<InitLendingMarket>, owner: Pubkey, quote_currency: [u8; 32]) -> Result<()> {
        Ok(())
    }

    pub fn set_lending_market_owner(ctx: Context<SetLendingMarketOwner>, new_owner: Pubkey) -> Result<()> {
        Ok(())
    }

    pub fn init_reserve(ctx: Context<InitReserve>, liquidity_amount: u64, config: ReserveConfig) -> Result<()> {
        Ok(())
    }

    pub fn refresh_reserve(ctx: Context<RefreshReserve>) -> Result<()> {
        Ok(())
    }

    pub fn deposit_reserve_liquidity(ctx: Context<DepositReserveLiquidity>, liquidity_amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn redeem_reserve_collateral(ctx: Context<RedeemReserveCollateral>, collateral_amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn init_obligation(ctx: Context<InitObligation>) -> Result<()> {
        Ok(())
    }

    pub fn refresh_obligation(ctx: Context<RefreshObligation>) -> Result<()> {
        Ok(())
    }

    pub fn deposit_obligation_collateral(ctx: Context<DepositObligationCollateral>, collateral_amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn withdraw_obligation_collateral(ctx: Context<WithdrawObligationCollateral>, collateral_amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn borrow_obligation_liquidity(ctx: Context<BorrowObligationLiquidity>, liquidity_amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn repay_obligation_liquidity(ctx: Context<RepayObligationLiquidity>, liquidity_amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn liquidate_obligation(ctx: Context<LiquidateObligation>, liquidity: u64) -> Result<()> {
        Ok(())
    }

    pub fn flash_loan(ctx: Context<FlashLoan>, amount: u64) -> Result<()> {
        Ok(())
    }
}


#[derive(Accounts)]
pub struct InitLendingMarket<'info> {
    #[account(mut)]
    lending_market: AccountInfo<'info>,
    rent: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    oracle: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetLendingMarketOwner<'info> {
    #[account(mut)]
    lending_market: AccountInfo<'info>,
    #[account(signer)]
    current_owner: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitReserve<'info> {
    #[account(mut)]
    source_liquidity_token: AccountInfo<'info>,
    #[account(mut)]
    dest_collateral_token: AccountInfo<'info>,
    #[account(mut)]
    reserve: AccountInfo<'info>,
    reserve_liquidity_spl_token_mint: AccountInfo<'info>,
    #[account(mut)]
    reserve_liquidity_supply_spl_token: AccountInfo<'info>,
    #[account(mut)]
    reserve_liquidity_fee_receiver: AccountInfo<'info>,
    #[account(mut)]
    reserve_collateral_spl_token_mint: AccountInfo<'info>,
    #[account(mut)]
    reserve_collateral_token_supply: AccountInfo<'info>,
    pyth_product: AccountInfo<'info>,
    pyth_price: AccountInfo<'info>,
    lending_market: AccountInfo<'info>,
    derived_lending_auth: AccountInfo<'info>,
    #[account(signer)]
    lending_market_owner: AccountInfo<'info>,
    #[account(signer)]
    user_transfer_auth: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    rent: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RefreshReserve<'info> {
    #[account(mut)]
    reserve: AccountInfo<'info>,
    reserve_liquidity_oracle: AccountInfo<'info>,
    clock: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositReserveLiquidity<'info> {
    #[account(mut)]
    source_liquidity_token: AccountInfo<'info>,
    #[account(mut)]
    dest_collateral_token: AccountInfo<'info>,
    #[account(mut)]
    reserve: AccountInfo<'info>,
    #[account(mut)]
    reserve_liquidity_supply_spl_token: AccountInfo<'info>,
    #[account(mut)]
    reserve_collateral_spl_token_mint: AccountInfo<'info>,
    lending_market: AccountInfo<'info>,
    derived_lending_auth: AccountInfo<'info>,
    #[account(signer)]
    user_transfer_auth: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RedeemReserveCollateral<'info> {
    #[account(mut)]
    source_collateral_token: AccountInfo<'info>,
    #[account(mut)]
    dest_liquidity_token: AccountInfo<'info>,
    #[account(mut)]
    reserve: AccountInfo<'info>,
    #[account(mut)]
    reserve_collateral_spl_token_mint: AccountInfo<'info>,
    #[account(mut)]
    reserve_liquidity_supply_spl_token: AccountInfo<'info>,
    lending_market: AccountInfo<'info>,
    derived_lending_auth: AccountInfo<'info>,
    #[account(signer)]
    user_transfer_auth: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitObligation<'info> {
    #[account(mut)]
    obligation: AccountInfo<'info>,
    lending_market: AccountInfo<'info>,
    #[account(signer)]
    obligation_owner: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    rent: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RefreshObligation<'info> {
    #[account(mut)]
    obligation: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    collateral_deposit_reserve: AccountInfo<'info>,
    liquidity_borrow_reserve: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositObligationCollateral<'info> {
    #[account(mut)]
    source_collateral_token: AccountInfo<'info>,
    #[account(mut)]
    dest_deposit_reserve_collateral_supply_spl_token: AccountInfo<'info>,
    deposit_reserve: AccountInfo<'info>,
    #[account(mut)]
    obligation: AccountInfo<'info>,
    lending_market: AccountInfo<'info>,
    #[account(signer)]
    obligation_owner: AccountInfo<'info>,
    #[account(signer)]
    user_transfer_auth: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawObligationCollateral<'info> {
    #[account(mut)]
    source_withdraw_reserve_collateral_supply_spl_token: AccountInfo<'info>,
    #[account(mut)]
    dest_collateral_token: AccountInfo<'info>,
    withdraw_reserve: AccountInfo<'info>,
    #[account(mut)]
    obligation: AccountInfo<'info>,
    lending_market: AccountInfo<'info>,
    derived_lending_auth: AccountInfo<'info>,
    #[account(signer)]
    obligation_owner: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct BorrowObligationLiquidity<'info> {
    #[account(mut)]
    source_borrow_reserve_liquidity_supply_spl_token: AccountInfo<'info>,
    #[account(mut)]
    dest_liquidity_token: AccountInfo<'info>,
    #[account(mut)]
    borrow_reserve: AccountInfo<'info>,
    #[account(mut)]
    borrow_reserve_liquidity: AccountInfo<'info>,
    #[account(mut)]
    obligation: AccountInfo<'info>,
    lending_market: AccountInfo<'info>,
    derived_lending_auth: AccountInfo<'info>,
    #[account(signer)]
    obligation_owner: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
    /* OPTIONAL ACCOUNTS **THESE DO NOT APPEAR IN THE IDL**
    #[account(mut)]
    host_fee_receiver: AccountInfo<'info>,
    */
    remaining_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RepayObligationLiquidity<'info> {
    #[account(mut)]
    source_liquidity_token: AccountInfo<'info>,
    #[account(mut)]
    dest_repay_reserve_liquidity_supply_spl_token: AccountInfo<'info>,
    #[account(mut)]
    repay_reserve: AccountInfo<'info>,
    #[account(mut)]
    obligation: AccountInfo<'info>,
    lending_market: AccountInfo<'info>,
    #[account(signer)]
    user_transfer_auth: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct LiquidateObligation<'info> {
    #[account(mut)]
    source_liquidity_token: AccountInfo<'info>,
    #[account(mut)]
    dest_collateral_token: AccountInfo<'info>,
    #[account(mut)]
    repay_reserve: AccountInfo<'info>,
    #[account(mut)]
    repay_reserve_liquidity_supply_spl_token: AccountInfo<'info>,
    withdraw_reserve: AccountInfo<'info>,
    #[account(mut)]
    withdraw_reserve_collateral_supply_spl_token: AccountInfo<'info>,
    #[account(mut)]
    obligation: AccountInfo<'info>,
    lending_market: AccountInfo<'info>,
    derived_lending_auth: AccountInfo<'info>,
    #[account(signer)]
    user_transfer_auth: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct FlashLoan<'info> {
    #[account(mut)]
    source_liquidity_token: AccountInfo<'info>,
    #[account(mut)]
    dest_liquidity_token: AccountInfo<'info>,
    #[account(mut)]
    reserve: AccountInfo<'info>,
    #[account(mut)]
    flash_loan_fee_receiver: AccountInfo<'info>,
    #[account(mut)]
    host_fee_receiver: AccountInfo<'info>,
    lending_market: AccountInfo<'info>,
    derived_lending_auth: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
    flash_loan_receiver_program_id: AccountInfo<'info>,
    /* OPTIONAL ACCOUNTS **THESE DO NOT APPEAR IN THE IDL**
    Additional accounts expected by the receiving program's 'ReceiveFlashLoan' instruction
    */
    remaining_accounts: AccountInfo<'info>,
}
