// This code is only used to generate the IDL for the record program.

use anchor_lang::prelude::*;

declare_id!("ReciQBw6sQKH9TVVJQDnbnJ5W7FP539tPHjZhRF4E9r");

#[program]
pub mod record {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn write(ctx: Context<Write>, offset: u64, data: Vec<u8>) -> Result<()> {
        Ok(())
    }

    pub fn set_authority(ctx: Context<SetAuthority>) -> Result<()> {
        Ok(())
    }

    pub fn close_account(ctx: Context<CloseAccount>) -> Result<()> {
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    record: AccountInfo<'info>,
    authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Write<'info> {
    #[account(mut)]
    record: AccountInfo<'info>,
    #[account(signer)]
    authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetAuthority<'info> {
    #[account(mut)]
    record: AccountInfo<'info>,
    #[account(signer)]
    authority: AccountInfo<'info>,
    new_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(mut)]
    record: AccountInfo<'info>,
    #[account(signer)]
    authority: AccountInfo<'info>,
    receiver: AccountInfo<'info>,
}
