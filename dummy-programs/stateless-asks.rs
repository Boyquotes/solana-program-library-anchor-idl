// This code is only used to generate the IDL for the stateless asks program.

use anchor_lang::prelude::*;

declare_id!("StatelessXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod stateless_asks {
    use super::*;

    pub fn accept_offer(ctx: Context<AcceptOffer>, has_metadata: bool, maker_size: u64, taker_size: u64, bump_seed: u8) -> Result<()> {
        Ok(())
    }
}


#[derive(Accounts)]
pub struct AcceptOffer<'info> {
}
