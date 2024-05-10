use anchor_lang::prelude::*;

declare_id!("2XiEkhPw8uGnDMEMF1D6A2BWjnQSiNHQhmhqwUpQT9Nr");

#[program]
pub mod pocnft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
