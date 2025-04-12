use anchor_lang::prelude::*;
// Our program's address!
// This matches the key in the target/deploy directory
declare_id!("6N3L2oAurRvcormHiWetSEyQb5S6jDyWbBC4taNvh4G2");

// Anchor programs always use 8 bits for the discriminator
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

// program
#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorites(
        context: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        let user_public_key = context.accounts.user.key();
        msg!("Greetings from {}", context.program_id);
        msg!(
            "User {}'s favorite number is {}, favorite color is: {}",
            user_public_key,
            number,
            color
        );

        msg!("User's hobbies are: {:?}", hobbies);

        context.accounts.favorites.set_inner(Favorites {
            user: user_public_key,
            number,
            color,
            hobbies,
        });
        Ok(())
    }

    pub fn log_favorites(ctx: Context<LogFavorites>) -> Result<()> {
        let favorites = &ctx.accounts.favorites;

        msg!("Favorite number: {}", favorites.number);
        msg!("Favorite color: {}", favorites.color);
        msg!("Favorites hobbies:");
        for hobby in favorites.hobbies.iter() {
            msg!("{}", hobby);
        }

        Ok(())
    }
}

// what thign we will put inside the Favorites PDA
#[account]
#[derive(InitSpace)] //specifying the size of the struct
pub struct Favorites {
    pub user: Pubkey,

    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}
// When people call the set_favorites instruction,
// they  need to provide the accounts that will be modifed.
#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed, 
        payer = user, 
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE, 
        seeds=[b"favorites_v2", user.key().as_ref()],
    bump)]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LogFavorites<'info> {
    #[account(
        seeds = [b"favorites_v2", user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub favorites: Account<'info, Favorites>,
    pub user: Signer<'info>,
}
