use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("ExyjeDqAnvh7civmJpKGwV4HtNSmP4xSDVM6nskARWYK");

#[program]
pub mod w7 {
    use super::*;

    pub fn ext_mint(ctx: Context<ExtMint>, info: MintMetaInfo) -> Result<()> {
        let mint_meta_data = &mut ctx.accounts.mint_meta_data;
        let mint_account = &ctx.accounts.mint_account;
        mint_meta_data.mint = mint_account.to_account_info().key();
        mint_meta_data.name = info.name;
        mint_meta_data.symbol = info.symbol;
        mint_meta_data.icon = info.icon;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExtMint<'info> {
    #[account(
        init,
        payer=user,
        space=512,
        seeds=[token_program.key().as_ref(), mint_account.key().as_ref()],
        bump
    )]
    pub mint_meta_data: Account<'info, MintMetaData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub mint_account: Account<'info, Mint>,
}

#[account]
pub struct MintMetaData {
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub icon: String,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MintMetaInfo {
    pub name: String,
    pub symbol: String,
    pub icon: String,
}
