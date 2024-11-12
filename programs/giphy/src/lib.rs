use anchor_lang::prelude::*;

declare_id!("5gk6gpU967nbGWXmeu7NS5ZetrJAmnhw9A44uYu1bfMH");

#[program]
pub mod giphy {
    use super::*;
    
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        ctx.accounts.base_account.total_gifs = 0;
        Ok(())
    }
    
    pub fn add_gifs(ctx: Context<AddGifs>, gif_link: String) -> Result<()> {
        
        let payer= &mut ctx.accounts.payer;
        let item = ItemStruct{
            gif_link: gif_link,
            user_address: *payer.to_account_info().key,
        };
        ctx.accounts.base_account.gif_list.push(item);
        ctx.accounts.base_account.total_gifs += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(
        init,
        payer = payer,
        // Manually calculate the space needed for BaseAccount
        space = 8 + 8 + 4 + (BaseAccount::MAX_GIFS * (32 + 4 + 100)), 
    )]
    pub base_account: Account<'info, BaseAccount>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGifs<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub payer: Signer<'info>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,  
}

impl BaseAccount {
    const MAX_GIFS: usize = 100;
}
