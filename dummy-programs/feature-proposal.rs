// This code is only used to generate the IDL for the feature proposal program.

use anchor_lang::prelude::*;

declare_id!("Feat1YXHhH6t1juaWF74WLcfv4XoNocjXA6sPWHNgAse");

#[program]
pub mod feature_proposal {
    use super::*;

    pub fn propose(ctx: Context<Propose>, tokens_to_mint: u64, acceptance_criteria: AcceptanceCriteria) -> Result<()> {
        Ok(())
    }

    pub fn tally(ctx: Context<Tally>) -> Result<()> {
        Ok(())
    }
}
    

#[derive(Accounts)]
pub struct Propose<'info> {
    #[account(mut, signer)]
    funding: AccountInfo<'info>,
    #[account(mut, signer)]
    unallocated_fp: AccountInfo<'info>,
    #[account(mut)]
    token_mint: AccountInfo<'info>,
    #[account(mut)]
    distributor_token: AccountInfo<'info>,
    #[account(mut)]
    acceptance_token: AccountInfo<'info>,
    #[account(mut)]
    feature_id: AccountInfo<'info>,
    system: AccountInfo<'info>,
    spl_token: AccountInfo<'info>,
    rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Tally<'info> {
    #[account(mut)]
    feature_proposal: AccountInfo<'info>,
    acceptance_token: AccountInfo<'info>,
    #[account(mut)]
    derived_feature_id: AccountInfo<'info>,
    system: AccountInfo<'info>,
    clock: AccountInfo<'info>,
}
