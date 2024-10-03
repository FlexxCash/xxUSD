use anchor_lang::prelude::*;
use crate::state::controller::Controller;
use crate::core::Amount;
use crate::error::XxusdError;

#[derive(Accounts)]
pub struct ManageProductPrice<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"controller"],
        bump = controller.bump,
        has_one = authority,
    )]
    pub controller: Account<'info, Controller>,
}

pub fn handler(ctx: Context<ManageProductPrice>, product_id: u64, price: Amount) -> Result<()> {
    set_product_price(ctx, product_id, price)
}

pub fn set_product_price(ctx: Context<ManageProductPrice>, product_id: u64, price: Amount) -> Result<()> {
    let controller = &mut ctx.accounts.controller;

    // Ensure the price is not zero
    require!(price.value() > 0, XxusdError::InvalidProductPrice);

    // Update or add the product price
    if let Some(existing_price) = controller.product_prices.iter_mut().find(|p| p.0 == product_id) {
        existing_price.1 = price;
    } else {
        controller.product_prices.push((product_id, price));
    }

    // Emit an event for the price update
    emit!(ProductPriceUpdated {
        product_id,
        price,
    });

    Ok(())
}

pub fn get_product_price(ctx: Context<ManageProductPrice>, product_id: u64) -> Result<Amount> {
    let controller = &ctx.accounts.controller;

    // Find the product price
    controller.product_prices
        .iter()
        .find(|p| p.0 == product_id)
        .map(|p| p.1)
        .ok_or_else(|| XxusdError::ProductNotFound.into())
}

#[event]
pub struct ProductPriceUpdated {
    pub product_id: u64,
    pub price: Amount,
}