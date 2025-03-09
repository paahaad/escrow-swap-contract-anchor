pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("56Qg3yXVwG7wnWT7DzsGzVXRt1RAiiQAXFftr5xsbToh");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>, id: i64, token_a_offered_amount: u64, token_b_wanted_amount: u64) -> Result<()> {

        instructions::make_offer::send_offered_to_vault(&ctx, token_a_offered_amount)?;
        instructions::make_offer::save_offer(ctx, id, token_b_wanted_amount)
    }

    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
        instructions::taker::send_wanted_token_to_maker(&ctx)?;
        instructions::taker::withdraw_and_close(ctx)
    }
}
