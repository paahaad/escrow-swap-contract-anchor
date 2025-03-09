use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{instructions::shared::tranfer_token, state::offer::Offer};

#[derive(Accounts)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = 8 + Offer::INIT_SPACE,
        seeds = [b"offer", maker.key().as_ref()],
        bump
        
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program
    )]
    pub valut: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>


}

pub fn send_offered_to_vault(ctx: &Context<MakeOffer>, token_a_offered_amount: u64) -> Result<()> {
    msg!("Welcome {}", ctx.program_id);
    tranfer_token(
        &ctx.accounts.maker_ata_a, 
        &ctx.accounts.valut, 
        &token_a_offered_amount, 
        &ctx.accounts.token_mint_a, 
        &ctx.accounts.maker, 
        &ctx.accounts.token_program
    )
}

pub fn save_offer(ctx: Context<MakeOffer>, id: i64, token_b_wanted_amount: u64) -> Result<()>{
    ctx.accounts.offer.set_inner(Offer{
        id,
        maker: ctx.accounts.maker.key(),
        token_mint_a: ctx.accounts.token_mint_a.key(),
        token_mint_b: ctx.accounts.token_mint_b.key(),
        wannted_amount: token_b_wanted_amount,
        bump: ctx.bumps.offer
    });
    Ok(())
}