// This code is only used to generate the IDL for the associated token account program.

use anchor_lang::prelude::*;

declare_id!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

#[program]
pub mod associated_token_account {
    use super::*;

    pub fn create(ctx: Context<Create>) -> Result<()> {
        Ok(())
    }

    pub fn create_idempotent(ctx: Context<CreateIdempotent>) -> Result<()> {
        Ok(())
    }

    pub fn recover_nested(ctx: Context<RecoverNested>) -> Result<()> {
        Ok(())
    }
}
    

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(mut, signer)]
    funding: AccountInfo<'info>,
    #[account(mut)]
    associated_token: AccountInfo<'info>,
    wallet: AccountInfo<'info>,
    token_mint_new: AccountInfo<'info>,
    system: AccountInfo<'info>,
    spl_token: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateIdempotent<'info> {
    #[account(mut, signer)]
    funding: AccountInfo<'info>,
    #[account(mut)]
    associated_token: AccountInfo<'info>,
    wallet: AccountInfo<'info>,
    token_mint_new: AccountInfo<'info>,
    system: AccountInfo<'info>,
    spl_token: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RecoverNested<'info> {
    #[account(mut)]
    nested_associated_token: AccountInfo<'info>,
    token_mint_nested: AccountInfo<'info>,
    #[account(mut)]
    wallet_associated: AccountInfo<'info>,
    owner: AccountInfo<'info>,
    token_mint_owner: AccountInfo<'info>,
    #[account(mut, signer)]
    wallet_owner: AccountInfo<'info>,
    spl_token: AccountInfo<'info>,
}

