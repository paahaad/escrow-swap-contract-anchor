use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        Mint, 
        TokenAccount, 
        TokenInterface, 
        TransferChecked,
        transfer_checked,
        CloseAccount,
        close_account
    }
};

use crate::{instructions::shared::tranfer_token, state::offer::Offer};

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    pub token_mint_a: InterfaceAccount<'info, Mint>,

    pub token_mint_b: InterfaceAccount<'info, Mint>,


    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_a,
        associated_token::authority = taker,
        associated_token::token_program  = token_program
    )]
    pub taker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = taker_ata_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = token_mint_a,
        has_one = token_mint_b,
        seeds = [b"offer", maker.key().as_ref()],
        bump = offer.bump

    )]
    pub offer: Account<'info, Offer>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>
    
}

pub fn send_wanted_token_to_maker(ctx: &Context<TakeOffer>) -> Result<()> {
    tranfer_token(
        &ctx.accounts.taker_ata_b, 
        &ctx.accounts.maker_ata_b, 
        &ctx.accounts.offer.wannted_amount, 
        &ctx.accounts.token_mint_b, 
        &ctx.accounts.taker, 
        &ctx.accounts.token_program
    )
}

pub fn withdraw_and_close(ctx: Context<TakeOffer>) -> Result<()> {

    let seed = &[
        b"offer",
        ctx.accounts.maker.to_account_info().key.as_ref(),
        &[ctx.accounts.offer.bump]
    ];

    let signer_seed = [&seed[..]];

    let accounts = TransferChecked{
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.taker_ata_b.to_account_info(),
        mint: ctx.accounts.token_mint_a.to_account_info(),
        authority: ctx.accounts.offer.to_account_info()
    };
    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(), 
        accounts, 
        &signer_seed
    );
    
    transfer_checked(cpi_context, ctx.accounts.vault.amount, ctx.accounts.token_mint_a.decimals)?;

    // Now cloes the account
    let account = CloseAccount{
        account: ctx.accounts.vault.to_account_info(),
        destination: ctx.accounts.taker.to_account_info(),
        authority: ctx.accounts.offer.to_account_info()
    };
    
    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(), 
        account, 
        &signer_seed
    );

    close_account(cpi_context)

}
