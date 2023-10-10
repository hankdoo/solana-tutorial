use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("2sqNTobLRXDtMZxemNY3HXWanz1aZAGnmQH2f35saPei");

#[program]
mod restaurant_review {
    use super::*;
    pub fn post_review(
        _ctx: Context<ReviewAccounts>,
        restaurant: String,
        review: String,
        rating: u8,
    ) -> Result<()> {
        let new_review = &mut _ctx.accounts.review;
        new_review.reviewer = _ctx.accounts.signer.key();
        new_review.restaurant = restaurant;
        new_review.review = review;
        new_review.rating = rating;
        msg!(
            "Restaurant review for {} - {} stars",
            new_review.restaurant,
            new_review.rating
        );
        msg!("Review: {}", new_review.review);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(restaurant: String)]
pub struct ReviewAccounts<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        space = (32 + 4 + 4 + 1) * 5,
        seeds=[restaurant.as_bytes().as_ref(),signer.key().as_ref()],
        bump
    )]
    pub review: Account<'info, Review>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Review {
    pub reviewer: Pubkey,
    pub restaurant: String,
    pub review: String,
    pub rating: u8,
}

