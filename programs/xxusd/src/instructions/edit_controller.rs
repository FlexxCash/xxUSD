use anchor_lang::prelude::*;
use crate::state::Controller;

#[derive(Accounts)]
pub struct EditController<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
    )]
    pub controller: Account<'info, Controller>,
}

pub fn handler(ctx: Context<EditController>, new_authority: Option<Pubkey>) -> Result<()> {
    let controller = &mut ctx.accounts.controller;
    
    if let Some(new_authority) = new_authority {
        controller.authority = new_authority;
    }

    Ok(())
}