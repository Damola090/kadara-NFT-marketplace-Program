use anchor_lang::prelude::*;

declare_id!("9hrbqDbjc9Vn8hwAWtRMmFPuwqPpZdKzqhsjucgqxfAM");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
