use anchor_lang::prelude::*;

declare_id!(Pubkey::from_str_const(env!("PUBLIC_KEY")));

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorites(
        ctx: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>) -> Result<()> {

        msg!("Greetings from {}", ctx.program_id);
        let user = ctx.accounts.user.key();

        msg!("User {user}'s favorite number is {number} and his favorite color is {color} and his favorite hobbies are {hobbies:?}");

        ctx.accounts.favorites.set_inner(Favorites {
            user,
            number,
            color,
            hobbies,
        });
        
        Ok(())
    }

    #[account]
    #[derive(InitSpace)]
    pub struct Favorites {
        pub user: Pubkey,

        pub number: u64,

        #[max_len(50)]
        pub color: String,

        #[max_len(5, 50)]
        pub hobbies: Vec<String>,
    }

    #[derive(Accounts)]
    pub struct SetFavorites<'info> {
        #[account(mut)]
        pub user: Signer<'info>,

        #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump,
        constraint = *favorites.to_account_info().owner == id()
    )]
        pub favorites: Account<'info, Favorites>,

        pub system_program: Program<'info, System>,
    }
}
