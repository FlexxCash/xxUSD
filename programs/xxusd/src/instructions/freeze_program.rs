use anchor_lang::prelude::*;
use crate::state::Controller;

#[derive(Accounts)]
pub struct FreezeProgram<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
    )]
    pub controller: Account<'info, Controller>,
}

pub fn handler(ctx: Context<FreezeProgram>, freeze: bool) -> Result<()> {
    let controller = &mut ctx.accounts.controller;
    controller.is_frozen = freeze;
    Ok(())
}