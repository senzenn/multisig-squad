use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFp1Jg");

#[program]
pub mod multisig_squad {
    use super::*;

    /// Simple hello world function
    pub fn hello(ctx: Context<Hello>) -> Result<()> {
        msg!("Hello, World! 👋");

        let user = &ctx.accounts.user;
        msg!("Called by user: {}", user.key());

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello<'info> {
    /// The user calling the function
    pub user: Signer<'info>,
}

