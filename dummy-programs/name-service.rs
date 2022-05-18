use anchor_lang::prelude::*;

declare_id!("namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX");

#[program]
pub mod name_service {
    use super::*;

    pub fn create(ctx: Context<Create>, hashed_name: Vec<u8>, lamports: u64, space: u32) -> Result<()> {
        Ok(())
    }

    pub fn update(ctx: Context<Update>, offset: u32, data: Vec<u8>) -> Result<()> {
        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, new_owner: Pubkey) -> Result<()> {
        Ok(())
    }

    pub fn delete(ctx: Context<Delete>) -> Result<()> {
        Ok(())
    }
}
    

#[derive(Accounts)]
pub struct Create<'info> {
    system_program: AccountInfo<'info>,
    #[account(mut, signer)]
    func_account: AccountInfo<'info>,
    #[account(mut)]
    name_record: AccountInfo<'info>,
    account_owner: AccountInfo<'info>,
    #[account(signer)]
    account_class: AccountInfo<'info>,
    parent_name_record: AccountInfo<'info>,
    #[account(signer)]
    parent_owner: AccountInfo<'info>,
    
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    name_record: AccountInfo<'info>,
    #[account(signer)]
    account_owner: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    name_record: AccountInfo<'info>,
    #[account(signer)]
    account_owner: AccountInfo<'info>,
    
}

#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(mut)]
    name_record: AccountInfo<'info>,
    #[account(signer)]
    account_owner: AccountInfo<'info>,
    #[account(mut)]
    refund_account: AccountInfo<'info>,
    
}
