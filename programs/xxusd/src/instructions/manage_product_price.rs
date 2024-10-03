use anchor_lang::prelude::*;
use crate::state::controller::Controller;
use crate::core::Amount;
use crate::error::XxusdError;

pub const CONTROLLER_SEED: &[u8] = b"controller";

#[derive(Accounts)]
pub struct ManageProductPrice<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [CONTROLLER_SEED],
        bump,
        has_one = authority,
    )]
    pub controller: Box<Account<'info, Controller>>,
}

pub fn handler(ctx: Context<ManageProductPrice>, product_id: u64, price: Amount) -> Result<()> {
    set_product_price(ctx, product_id, price)
}

fn set_product_price(ctx: Context<ManageProductPrice>, product_id: u64, price: Amount) -> Result<()> {
    // Ensure the price is not zero
    require!(price.value() > 0, XxusdError::InvalidProductPrice);

    // Ensure the product_id is valid (you might want to define a valid range)
    require!(product_id > 0, XxusdError::InvalidProductId);

    let controller = &mut ctx.accounts.controller;

    // Update or add the product price
    if let Some(existing_price) = controller.product_prices.iter_mut().find(|p| p.0 == product_id) {
        existing_price.1 = price;
    } else {
        // Check if we're not exceeding a maximum number of products
        require!(
            controller.product_prices.len() < controller.max_products as usize,
            XxusdError::MaxProductsReached
        );
        controller.product_prices.push((product_id, price));
    }

    // Emit an event for the price update
    emit!(ProductPriceUpdated {
        product_id,
        price,
    });

    Ok(())
}

pub fn get_product_price(ctx: &Context<ManageProductPrice>, product_id: u64) -> Result<Amount> {
    let controller = &ctx.accounts.controller;

    // Find the product price
    controller.product_prices
        .iter()
        .find(|p| p.0 == product_id)
        .map(|p| p.1)
        .ok_or(XxusdError::ProductNotFound.into())
}

#[event]
pub struct ProductPriceUpdated {
    pub product_id: u64,
    pub price: Amount,
}