// use anchor_lang::prelude::*;
// use anchor_spl::token_interface::{
//     transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
// };

// pub fn transfer_tokens<'info>(
//     from: &InterfaceAccount<'info, TokenAccount>,
//     to: &InterfaceAccount<'info, TokenAccount>,
//     amount: &u64,
//     mint: &InterfaceAccount<'info, Mint>,
//     authority: &Signer<'info>,
//     token_program: &Interface<'info, TokenInterface>,
// ) -> Result<()> {
//     let transfer_account_options = TransferChecked {
//         from: from.to_account_info(),
//         to: to.to_account_info(),
//         mint: mint.to_account_info(),
//         authority: authority.to_account_info(),
//     };
//     let cpi_context = CpiContext::new(token_program.to_account_info(), transfer_account_options);

//     let _ = transfer_checked(cpi_context, *amount, mint.decimals);
//     Ok(())
// }

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

pub fn tranfer_token<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: &u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface>
) -> Result<()> {
    let tranfer_checked_option = TransferChecked {
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };
    let cpi_context = CpiContext::new(token_program.to_account_info(), tranfer_checked_option);
    let _ = transfer_checked(cpi_context, *amount, mint.decimals);
    Ok(())
}
