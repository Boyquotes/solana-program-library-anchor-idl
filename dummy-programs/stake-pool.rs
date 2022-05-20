// This code is only used to generate the IDL for the stake pool program.

use anchor_lang::prelude::*;

declare_id!("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy");

#[program]
pub mod stake_pool {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee: Fee, withdrawal_fee: Fee, deposit_fee: Fee, referral_fee: u8, max_validators: u32) -> Result<()> {
        Ok(())
    }

    pub fn add_validator_to_pool(ctx: Context<AddValidatorToPool>) -> Result<()> {
        Ok(())
    }

    pub fn remove_validator_from_pool(ctx: Context<RemoveValidatorFromPool>) -> Result<()> {
        Ok(())
    }

    pub fn decrease_validator_stake(ctx: Context<DecreaseValidatorStake>, lamports: u64, transient_stake_seed: u64) -> Result<()> {
        Ok(())
    }

    pub fn increase_validator_stake(ctx: Context<IncreaseValidatorStake>, lamports: u64, transient_stake_seed: u64) -> Result<()> {
        Ok(())
    }

    pub fn set_preferred_validator(ctx: Context<SetPreferredValidator>, validator_type: PreferredValidatorType, validator_vote_address: Option<Pubkey>) -> Result<()> {
        Ok(())
    }

    pub fn update_validator_list_balance(ctx: Context<UpdateValidatorListBalance>, start_index: u32, no_merge: bool) -> Result<()> {
        Ok(())
    }

    pub fn update_stake_pool_balance(ctx: Context<UpdateStakePoolBalance>) -> Result<()> {
        Ok(())
    }

    pub fn cleanup_removed_validator_entries(ctx: Context<CleanupRemovedValidatorEntries>) -> Result<()> {
        Ok(())
    }

    pub fn deposit_stake(ctx: Context<DepositStake>) -> Result<()> {
        Ok(())
    }

    pub fn withdraw_stake(ctx: Context<WithdrawStake>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn set_manager(ctx: Context<SetManager>, lamports: u64, transient_stake_seed: u64) -> Result<()> {
        Ok(())
    }

    pub fn set_fee(ctx: Context<SetFee>, fee: FeeType) -> Result<()> {
        Ok(())
    }

    pub fn set_staker(ctx: Context<SetStaker>) -> Result<()> {
        Ok(())
    }

    pub fn deposit_sol(ctx: Context<DepositSol>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn set_funding_authority(ctx: Context<SetFundingAuthority>, data: FundingType) -> Result<()> {
        Ok(())
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    new_pool: AccountInfo<'info>,
    #[account(signer)]
    manager: AccountInfo<'info>,
    staker: AccountInfo<'info>,
    withdraw_auth: AccountInfo<'info>,
    #[account(mut)]
    stake_list_storage: AccountInfo<'info>,
    reserve_stake: AccountInfo<'info>,
    pool_token_mint: AccountInfo<'info>,
    manager_fee_pool: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
    /* OPTIONAL ACCOUNTS **THESE DO NOT APPEAR IN THE IDL**
    deposit_auth: AccountInfo<'info>,
    */
    remaining_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AddValidatorToPool<'info> {
    #[account(mut)]
    stake_pool: AccountInfo<'info>,
    #[account(signer)]
    staker: AccountInfo<'info>,
    #[account(mut, signer)]
    funding: AccountInfo<'info>,
    withdraw_auth: AccountInfo<'info>,
    #[account(mut)]
    stake_list_storage: AccountInfo<'info>,
    #[account(mut)]
    add_stake: AccountInfo<'info>,
    validator: AccountInfo<'info>,
    rent: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    history: AccountInfo<'info>,
    config: AccountInfo<'info>,
    system: AccountInfo<'info>,
    stake: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RemoveValidatorFromPool<'info> {
    #[account(mut)]
    stake_pool: AccountInfo<'info>,
    #[account(signer)]
    staker: AccountInfo<'info>,
    withdraw_auth: AccountInfo<'info>,
    new_withdraw_auth: AccountInfo<'info>,
    #[account(mut)]
    stake_list_storage: AccountInfo<'info>,
    #[account(mut)]
    remove_stake: AccountInfo<'info>,
    transient_stake: AccountInfo<'info>,
    #[account(mut)]
    destination_stake: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    stake_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DecreaseValidatorStake<'info> {
    stake_pool: AccountInfo<'info>,
    #[account(signer)]
    stake_pool_staker: AccountInfo<'info>,
    withdraw_auth: AccountInfo<'info>,
    #[account(mut)]
    validator_list: AccountInfo<'info>,
    #[account(mut)]
    canonical_stake: AccountInfo<'info>,
    #[account(mut)]
    transient_stake: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    rent: AccountInfo<'info>,
    system: AccountInfo<'info>,
    stake: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct IncreaseValidatorStake<'info> {
    stake_pool: AccountInfo<'info>,
    #[account(signer)]
    stake_pool_staker: AccountInfo<'info>,
    withdraw_auth: AccountInfo<'info>,
    #[account(mut)]
    validator_list: AccountInfo<'info>,
    #[account(mut)]
    reserve_stake: AccountInfo<'info>,
    #[account(mut)]
    transient_stake: AccountInfo<'info>,
    validator: AccountInfo<'info>,
    validator_vote: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    rent: AccountInfo<'info>,
    history: AccountInfo<'info>,
    config: AccountInfo<'info>,
    system: AccountInfo<'info>,
    stake: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetPreferredValidator<'info> {
    #[account(mut)]
    stake_pool: AccountInfo<'info>,
    #[account(signer)]
    stake_pool_staker: AccountInfo<'info>,
    validator_list: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateValidatorListBalance<'info> {
    stake_pool: AccountInfo<'info>,
    withdraw_auth: AccountInfo<'info>,
    #[account(mut)]
    stake_list_storage: AccountInfo<'info>,
    #[account(mut)]
    reserve_stake: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    history: AccountInfo<'info>,
    stake: AccountInfo<'info>,
    // ADDING REMAINING ACCOUNTS HERE SINCE THE FOLLOWING COULD BE ANY NUMBER OF ACCOUNTS
    // 7+N pairs of validator and transient stake accounts 
    remaining_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateStakePoolBalance<'info> {
    #[account(mut)]
    stake_pool: AccountInfo<'info>,
    withdraw_auth: AccountInfo<'info>,
    #[account(mut)]
    stake_list_storage: AccountInfo<'info>,
    reserve_stake: AccountInfo<'info>,
    #[account(mut)]
    fee_receive: AccountInfo<'info>,
    #[account(mut)]
    pool_mint: AccountInfo<'info>,
    pool_token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CleanupRemovedValidatorEntries<'info> {
    stake_pool: AccountInfo<'info>,
    #[account(mut)]
    stake_list_storage: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositStake<'info> {
    #[account(mut)]
    stake_pool: AccountInfo<'info>,
    #[account(mut)]
    stake_list_storage: AccountInfo<'info>,
    #[account(signer)]
    deposit_auth: AccountInfo<'info>,
    withdraw_auth: AccountInfo<'info>,
    #[account(mut)]
    join_stake: AccountInfo<'info>,
    #[account(mut)]
    validator_stake: AccountInfo<'info>,
    #[account(mut)]
    reserve_stake: AccountInfo<'info>,
    #[account(mut)]
    receive_user: AccountInfo<'info>,
    #[account(mut)]
    fee_receive: AccountInfo<'info>,
    #[account(mut)]
    referral_receive: AccountInfo<'info>,
    #[account(mut)]
    pool_token_mint: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    history: AccountInfo<'info>,
    pool_token_program_id: AccountInfo<'info>,
    stake_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawStake<'info> {
    #[account(mut)]
    stake_pool: AccountInfo<'info>,
    #[account(mut)]
    stake_list_storage: AccountInfo<'info>,
    withdraw_auth: AccountInfo<'info>,
    #[account(mut)]
    validator_or_reserve: AccountInfo<'info>,
    #[account(mut)]
    receive_withdrawal: AccountInfo<'info>,
    new_withdraw_auth: AccountInfo<'info>,
    #[account(signer)]
    transfer_auth: AccountInfo<'info>,
    #[account(mut)]
    burn_from: AccountInfo<'info>,
    #[account(mut)]
    fee_receive: AccountInfo<'info>,
    #[account(mut)]
    pool_token_mint: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    pool_token_program_id: AccountInfo<'info>,
    stake_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetManager<'info> {
    #[account(mut)]
    stake_pool: AccountInfo<'info>,
    #[account(signer)]
    manager: AccountInfo<'info>,
    #[account(signer)]
    new_manager: AccountInfo<'info>,
    new_manager_fee: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetFee<'info> {
    #[account(mut)]
    stake_pool: AccountInfo<'info>,
    #[account(signer)]
    manager: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetStaker<'info> {
    #[account(mut)]
    stake_pool: AccountInfo<'info>,
    #[account(signer)]
    manager: AccountInfo<'info>,
    new_staker_pubkey: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositSol<'info> {
    #[account(mut)]
    stake_pool: AccountInfo<'info>,
    withdraw_auth: AccountInfo<'info>,
    #[account(mut)]
    reserve_stake: AccountInfo<'info>,
    #[account(signer)]
    lamport_provider: AccountInfo<'info>,
    #[account(mut)]
    receive_user: AccountInfo<'info>,
    #[account(mut)]
    fee_receive: AccountInfo<'info>,
    #[account(mut)]
    referral_receive: AccountInfo<'info>,
    #[account(mut)]
    pool_token_mint: AccountInfo<'info>,
    system: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
    /* OPTIONAL ACCOUNTS **THESE DO NOT APPEAR IN THE IDL**
    #[account(signer)]
    stake_pool_deposit_auth: AccountInfo<'info>,
    */
    remaining_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetFundingAuthority<'info> {
    #[account(mut)]
    stake_pool: AccountInfo<'info>,
    #[account(signer)]
    manager: AccountInfo<'info>,
    new_auth_pubkey: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawSol<'info> {
    #[account(mut)]
    stake_pool: AccountInfo<'info>,
    withdraw_auth: AccountInfo<'info>,
    #[account(signer)]
    transfer_auth: AccountInfo<'info>,
    #[account(mut)]
    burn_from: AccountInfo<'info>,
    #[account(mut)]
    reserve_stake: AccountInfo<'info>,
    #[account(mut)]
    reserve_lamport_receive: AccountInfo<'info>,
    #[account(mut)]
    fee_receive: AccountInfo<'info>,
    #[account(mut)]
    pool_token_mint: AccountInfo<'info>,
    clock: AccountInfo<'info>,
    history: AccountInfo<'info>,
    stake: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
    /* OPTIONAL ACCOUNTS **THESE DO NOT APPEAR IN THE IDL**
    #[account(signer)]
    stake_pool_withdraw_auth: AccountInfo<'info>,
    */
    remaining_accounts: AccountInfo<'info>,
}
