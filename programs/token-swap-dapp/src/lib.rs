use anchor_lang::prelude::*;

declare_id!("CDvQtfPPKVkAfrcC9PJuQYFdj46Zk2fje2q4rmpgT6dh");

#[program]
pub mod token_swap_dapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
