//! An IDO pool program implementing the Mango Markets token sale design here:
//! https://docs.mango.markets/litepaper#token-sale.
// #![warn(clippy::all)]

use anchor_lang::prelude::*;
use anchor_spl::token::{
    self,
    Burn,
    CloseAccount,
    Mint,
    MintTo,
    Token,
    TokenAccount,
    Transfer,
    FreezeAccount,
    ThawAccount,
};

use std::ops::Deref;

// the pubkey of the program, this needs to match or else no contract calls work
declare_id!("v8N37RZ3w484F5XXnxeQnKEoARLZaqtJ31ibYwxkCRM");

const DECIMALS: u8 = 6;

#[program]
pub mod ido_pool {
    use super::*;

    #[access_control(validate_ido_times(ido_times))]
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        ido_name: String,
        bumps: PoolBumps,
        num_ido_tokens: u64,
        ido_times: IdoTimes,
    ) -> ProgramResult {
        msg!("INITIALIZE POOL");

        let ido_account = &mut ctx.accounts.ido_account;

        let name_bytes = ido_name.as_bytes();
        let mut name_data = [b' '; 10];
        name_data[..name_bytes.len()].copy_from_slice(name_bytes);

        ido_account.ido_name = name_data;
        ido_account.bumps = bumps;
        ido_account.ido_authority = ctx.accounts.ido_authority.key();

        ido_account.usdc_mint = ctx.accounts.usdc_mint.key();
        ido_account.redeemable_mint = ctx.accounts.redeemable_mint.key();
        ido_account.watermelon_mint = ctx.accounts.watermelon_mint.key();
        ido_account.pool_usdc = ctx.accounts.pool_usdc.key();
        ido_account.pool_watermelon = ctx.accounts.pool_watermelon.key();

        ido_account.num_options = 0;
        ido_account.num_ido_tokens = num_ido_tokens;
        ido_account.ido_times = ido_times;

        ido_account.exchange_num = 1;
        ido_account.exchange_denom = 2;

        // Transfer Watermelon from ido_authority to pool account.
        let cpi_accounts = Transfer {
            from: ctx.accounts.ido_authority_watermelon.to_account_info(),
            to: ctx.accounts.pool_watermelon.to_account_info(),
            authority: ctx.accounts.ido_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, num_ido_tokens)?;

        Ok(())
    }

    #[access_control(unrestricted_phase(&ctx.accounts.ido_account))]
    pub fn init_user_redeemable(ctx: Context<InitUserRedeemable>) -> ProgramResult {
        msg!("INIT USER REDEEMABLE");
        let ido_name = ctx.accounts.ido_account.ido_name.as_ref();
        let seeds = &[
            ido_name.trim_ascii_whitespace(),
            &[ctx.accounts.ido_account.bumps.ido_account],
        ];
        let signer = &[&seeds[..]];
        let cpi_accounts = FreezeAccount {
            account: ctx.accounts.user_redeemable.to_account_info(),
            authority: ctx.accounts.ido_account.to_account_info(),
            mint: ctx.accounts.redeemable_mint.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::freeze_account(cpi_ctx)?;

        Ok(())
    }

    #[access_control(unrestricted_phase(&ctx.accounts.ido_account))]
    pub fn exchange_usdc_for_redeemable(
        ctx: Context<ExchangeUsdcForRedeemable>,
        amount: u64,
    ) -> ProgramResult {
        msg!("EXCHANGE USDC FOR REDEEMABLE");
        // While token::transfer will check this, we prefer a verbose err msg.
        if ctx.accounts.user_usdc.amount < amount {
            return Err(ErrorCode::LowUsdc.into());
        }

        // Transfer user's USDC to pool USDC account.
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_usdc.to_account_info(),
            to: ctx.accounts.pool_usdc.to_account_info(),
            authority: ctx.accounts.user_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        // Mint Redeemable to user Redeemable account.
        let ido_name = ctx.accounts.ido_account.ido_name.as_ref();
        let seeds = &[
            ido_name.trim_ascii_whitespace(),
            &[ctx.accounts.ido_account.bumps.ido_account],
        ];
        let signer = &[&seeds[..]];

        // thaw the user's account
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = ThawAccount {
            account: ctx.accounts.user_redeemable.to_account_info(),
            mint: ctx.accounts.redeemable_mint.to_account_info(),
            authority: ctx.accounts.ido_account.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::thaw_account(cpi_ctx)?;

        let cpi_accounts = MintTo {
            mint: ctx.accounts.redeemable_mint.to_account_info(),
            to: ctx.accounts.user_redeemable.to_account_info(),
            authority: ctx.accounts.ido_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::mint_to(cpi_ctx, amount)?;

        // freeze the user's account again
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = FreezeAccount {
            account: ctx.accounts.user_redeemable.to_account_info(),
            mint: ctx.accounts.redeemable_mint.to_account_info(),
            authority: ctx.accounts.ido_account.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::freeze_account(cpi_ctx)?;

        ctx.accounts.ido_account.num_options += amount;

        Ok(())
    }

    #[access_control(withdraw_phase(&ctx.accounts.ido_account))]
    pub fn init_escrow_usdc(ctx: Context<InitEscrowUsdc>) -> ProgramResult {
        msg!("INIT ESCROW USDC");
        Ok(())
    }

    #[access_control(withdraw_phase(&ctx.accounts.ido_account))]
    pub fn exchange_redeemable_for_usdc(
        ctx: Context<ExchangeRedeemableForUsdc>,
        amount: u64,
    ) -> ProgramResult {
        msg!("EXCHANGE REDEEMABLE FOR USDC");
        // While token::burn will check this, we prefer a verbose err msg.
        if ctx.accounts.user_redeemable.amount < amount {
            return Err(ErrorCode::LowRedeemable.into());
        }

        let ido_name = ctx.accounts.ido_account.ido_name.as_ref();
        let seeds = &[
            ido_name.trim_ascii_whitespace(),
            &[ctx.accounts.ido_account.bumps.ido_account],
        ];
        let signer = &[&seeds[..]];

        // Burn the user's redeemable tokens.
        // thaw the user's account so we can burn from it
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = ThawAccount {
            account: ctx.accounts.user_redeemable.to_account_info(),
            mint: ctx.accounts.redeemable_mint.to_account_info(),
            authority: ctx.accounts.ido_account.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::thaw_account(cpi_ctx)?;

        let cpi_accounts = Burn {
            mint: ctx.accounts.redeemable_mint.to_account_info(),
            to: ctx.accounts.user_redeemable.to_account_info(),
            authority: ctx.accounts.ido_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::burn(cpi_ctx, amount)?;

        // Transfer USDC from pool account to the user's escrow account.
        // take our 5% tax
        let usdc_amount = amount.checked_sub(amount.checked_div(20).unwrap()).unwrap();
        let cpi_accounts = Transfer {
            from: ctx.accounts.pool_usdc.to_account_info(),
            to: ctx.accounts.escrow_usdc.to_account_info(),
            authority: ctx.accounts.ido_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, usdc_amount)?;

        // Send rent back to user if account is empty
        ctx.accounts.user_redeemable.reload()?;
        if ctx.accounts.user_redeemable.amount == 0 {
            let cpi_accounts = CloseAccount {
                account: ctx.accounts.user_redeemable.to_account_info(),
                destination: ctx.accounts.user_authority.to_account_info(),
                authority: ctx.accounts.ido_account.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::close_account(cpi_ctx)?;
        } else {
            // otherwise, freeze the user's options account again
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_accounts = FreezeAccount {
                account: ctx.accounts.user_redeemable.to_account_info(),
                mint: ctx.accounts.redeemable_mint.to_account_info(),
                authority: ctx.accounts.ido_account.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::freeze_account(cpi_ctx)?;
        }

        ctx.accounts.ido_account.num_options -= amount;

        Ok(())
    }

    #[access_control(ido_over(&ctx.accounts.ido_account))]
    pub fn exchange_redeemable_for_watermelon(
        ctx: Context<ExchangeRedeemableForWatermelon>,
        amount: u64,
    ) -> ProgramResult {
        msg!("EXCHANGE REDEEMABLE FOR WATERMELON");
        // While token::burn will check this, we prefer a verbose err msg.
        if ctx.accounts.user_redeemable.amount < amount {
            return Err(ErrorCode::LowRedeemable.into());
        }

        // Calculate watermelon tokens due.
        // amount * (exchange rate) * (total gov tokens / total options) * (1/2)
        let watermelon_amount = (amount as u128)
            .checked_mul(ctx.accounts.ido_account.num_ido_tokens as u128)
            .unwrap()
            .checked_div(2)
            .unwrap()
            .checked_mul(ctx.accounts.ido_account.exchange_num as u128)
            .unwrap()
            .checked_div(ctx.accounts.ido_account.num_options as u128)
            .unwrap()
            .checked_div(ctx.accounts.ido_account.exchange_denom as u128)
            .unwrap();

        let ido_name = ctx.accounts.ido_account.ido_name.as_ref();
        let seeds = &[
            ido_name.trim_ascii_whitespace(),
            &[ctx.accounts.ido_account.bumps.ido_account],
        ];
        let signer = &[&seeds[..]];

        // Burn the user's redeemable tokens.
        // thaw the user's account so we can burn from it
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = ThawAccount {
            account: ctx.accounts.user_redeemable.to_account_info(),
            mint: ctx.accounts.redeemable_mint.to_account_info(),
            authority: ctx.accounts.ido_account.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::thaw_account(cpi_ctx)?;

        let cpi_accounts = Burn {
            mint: ctx.accounts.redeemable_mint.to_account_info(),
            to: ctx.accounts.user_redeemable.to_account_info(),
            authority: ctx.accounts.ido_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::burn(cpi_ctx, amount)?;

        // Transfer Watermelon from pool account to user.
        let cpi_accounts = Transfer {
            from: ctx.accounts.pool_watermelon.to_account_info(),
            to: ctx.accounts.user_watermelon.to_account_info(),
            authority: ctx.accounts.ido_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, watermelon_amount as u64)?;

        // Send rent back to user if account is empty
        ctx.accounts.user_redeemable.reload()?;
        if ctx.accounts.user_redeemable.amount == 0 {
            let cpi_accounts = CloseAccount {
                account: ctx.accounts.user_redeemable.to_account_info(),
                destination: ctx.accounts.user_authority.clone(),
                authority: ctx.accounts.ido_account.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::close_account(cpi_ctx)?;
        } else {
            // otherwise, freeze the user's options account again
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_accounts = FreezeAccount {
                account: ctx.accounts.user_redeemable.to_account_info(),
                mint: ctx.accounts.redeemable_mint.to_account_info(),
                authority: ctx.accounts.ido_account.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::freeze_account(cpi_ctx)?;
        }
        // ido_account.num_options represents the _total_ number of options given out, so we don't decrement
        // it here

        Ok(())
    }

    #[access_control(ido_over(&ctx.accounts.ido_account))]
    pub fn withdraw_pool_usdc(ctx: Context<WithdrawPoolUsdc>) -> ProgramResult {
        msg!("WITHDRAW POOL USDC");
        // Transfer total USDC from pool account to ido_authority account.
        let ido_name = ctx.accounts.ido_account.ido_name.as_ref();
        let seeds = &[
            ido_name.trim_ascii_whitespace(),
            &[ctx.accounts.ido_account.bumps.ido_account],
        ];
        let signer = &[&seeds[..]];
        let cpi_accounts = Transfer {
            from: ctx.accounts.pool_usdc.to_account_info(),
            to: ctx.accounts.ido_authority_usdc.to_account_info(),
            authority: ctx.accounts.ido_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, ctx.accounts.pool_usdc.amount)?;

        Ok(())
    }

    #[access_control(escrow_over(&ctx.accounts.ido_account))]
    pub fn withdraw_from_escrow(ctx: Context<WithdrawFromEscrow>, amount: u64) -> ProgramResult {
        msg!("WITHDRAW FROM ESCROW");
        // While token::transfer will check this, we prefer a verbose err msg.
        if ctx.accounts.escrow_usdc.amount < amount {
            return Err(ErrorCode::LowUsdc.into());
        }

        let ido_name = ctx.accounts.ido_account.ido_name.as_ref();
        let seeds = &[
            ido_name.trim_ascii_whitespace(),
            &[ctx.accounts.ido_account.bumps.ido_account],
        ];
        let signer = &[&seeds[..]];

        // Transfer USDC from user's escrow account to user's USDC account.
        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_usdc.to_account_info(),
            to: ctx.accounts.user_usdc.to_account_info(),
            authority: ctx.accounts.ido_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, amount)?;

        // Send rent back to user if account is empty
        ctx.accounts.escrow_usdc.reload()?;
        if ctx.accounts.escrow_usdc.amount == 0 {
            let cpi_accounts = CloseAccount {
                account: ctx.accounts.escrow_usdc.to_account_info(),
                destination: ctx.accounts.user_authority.clone(),
                authority: ctx.accounts.ido_account.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::close_account(cpi_ctx)?;
        }

        Ok(())
    }

    pub fn update_exchange_rate(ctx: Context<UpdateExchangeRate>, num: u64, denom: u64) -> ProgramResult {
        msg!("UPDATING EXCHANGE RATE");
        let ido = &mut ctx.accounts.ido_account;
        ido.exchange_num = num;
        ido.exchange_denom = denom;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(ido_name: String, bumps: PoolBumps)]
pub struct InitializePool<'info> {
    // IDO Authority accounts
    #[account(mut)]
    pub ido_authority: Signer<'info>,
    // Watermelon Doesn't have to be an ATA because it could be DAO controlled
    #[account(mut,
        constraint = ido_authority_watermelon.owner == ido_authority.key(),
        constraint = ido_authority_watermelon.mint == watermelon_mint.key())]
    pub ido_authority_watermelon: Box<Account<'info, TokenAccount>>,
    // IDO Accounts
    #[account(init,
        seeds = [ido_name.as_bytes()],
        bump = bumps.ido_account,
        payer = ido_authority)]
    pub ido_account: Box<Account<'info, IdoAccount>>,
    // TODO Confirm USDC mint address on mainnet or leave open as an option for other stables
    #[account(constraint = usdc_mint.decimals == DECIMALS)]
    pub usdc_mint: Box<Account<'info, Mint>>,
    #[account(init,
        mint::decimals = DECIMALS,
        mint::authority = ido_account,
        mint::freeze_authority = ido_account,
        seeds = [ido_name.as_bytes(), b"redeemable_mint".as_ref()],
        bump = bumps.redeemable_mint,
        payer = ido_authority)]
    pub redeemable_mint: Box<Account<'info, Mint>>,
    #[account(constraint = watermelon_mint.key() == ido_authority_watermelon.mint)]
    pub watermelon_mint: Box<Account<'info, Mint>>,
    #[account(init,
        token::mint = watermelon_mint,
        token::authority = ido_account,
        seeds = [ido_name.as_bytes(), b"pool_watermelon"],
        bump = bumps.pool_watermelon,
        payer = ido_authority)]
    pub pool_watermelon: Box<Account<'info, TokenAccount>>,
    #[account(init,
        token::mint = usdc_mint,
        token::authority = ido_account,
        seeds = [ido_name.as_bytes(), b"pool_usdc"],
        bump = bumps.pool_usdc,
        payer = ido_authority)]
    pub pool_usdc: Box<Account<'info, TokenAccount>>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitUserRedeemable<'info> {
    // User Accounts
    #[account(mut)]
    pub user_authority: Signer<'info>,
    #[account(init,
        token::mint = redeemable_mint,
        token::authority = ido_account,
        seeds = [user_authority.key().as_ref(),
            ido_account.ido_name.as_ref().trim_ascii_whitespace(),
            b"user_redeemable"],
        bump,
        payer = user_authority)]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,
    // IDO Accounts
    #[account(seeds = [ido_account.ido_name.as_ref().trim_ascii_whitespace()],
        bump = ido_account.bumps.ido_account)]
    pub ido_account: Box<Account<'info, IdoAccount>>,
    #[account(seeds = [ido_account.ido_name.as_ref().trim_ascii_whitespace(), b"redeemable_mint"],
        bump = ido_account.bumps.redeemable_mint)]
    pub redeemable_mint: Box<Account<'info, Mint>>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ExchangeUsdcForRedeemable<'info> {
    // User Accounts
    pub user_authority: Signer<'info>,
    // TODO replace these with the ATA constraints when possible
    #[account(mut,
        constraint = user_usdc.owner == user_authority.key(),
        constraint = user_usdc.mint == usdc_mint.key())]
    pub user_usdc: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        seeds = [user_authority.key().as_ref(),
            ido_account.ido_name.as_ref().trim_ascii_whitespace(),
            b"user_redeemable"],
        bump)]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,
    // IDO Accounts
    #[account(mut,
        seeds = [ido_account.ido_name.as_ref().trim_ascii_whitespace()],
        bump = ido_account.bumps.ido_account,
        has_one = usdc_mint)]
    pub ido_account: Box<Account<'info, IdoAccount>>,
    pub usdc_mint: Box<Account<'info, Mint>>,
    #[account(mut,
        seeds = [ido_account.ido_name.as_ref().trim_ascii_whitespace(), b"redeemable_mint"],
        bump = ido_account.bumps.redeemable_mint)]
    pub redeemable_mint: Box<Account<'info, Mint>>,
    #[account(mut,
        seeds = [ido_account.ido_name.as_ref().trim_ascii_whitespace(), b"pool_usdc"],
        bump = ido_account.bumps.pool_usdc)]
    pub pool_usdc: Box<Account<'info, TokenAccount>>,
    // Programs and Sysvars
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct InitEscrowUsdc<'info> {
    // User Accounts
    #[account(mut)]
    pub user_authority: Signer<'info>,
    #[account(init,
        token::mint = usdc_mint,
        token::authority = ido_account,
        seeds =  [user_authority.key().as_ref(),
            ido_account.ido_name.as_ref().trim_ascii_whitespace(),
            b"escrow_usdc"],
        bump,
        payer = user_authority)]
    pub escrow_usdc: Box<Account<'info, TokenAccount>>,
    #[account(seeds = [ido_account.ido_name.as_ref().trim_ascii_whitespace()],
        bump = ido_account.bumps.ido_account,
        has_one = usdc_mint)]
    pub ido_account: Box<Account<'info, IdoAccount>>,
    pub usdc_mint: Box<Account<'info, Mint>>,
    // Programs and Sysvars
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ExchangeRedeemableForUsdc<'info> {
    // User Accounts
    pub user_authority: Signer<'info>,
    #[account(mut,
        seeds = [user_authority.key().as_ref(),
            ido_account.ido_name.as_ref().trim_ascii_whitespace(),
            b"escrow_usdc"],
        bump)]
    pub escrow_usdc: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        seeds = [user_authority.key().as_ref(),
            ido_account.ido_name.as_ref().trim_ascii_whitespace(),
            b"user_redeemable"],
        bump)]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,
    // IDO Accounts
    #[account(mut,
        seeds = [ido_account.ido_name.as_ref().trim_ascii_whitespace()],
        bump = ido_account.bumps.ido_account,
        has_one = usdc_mint)]
    pub ido_account: Box<Account<'info, IdoAccount>>,
    pub usdc_mint: Box<Account<'info, Mint>>,
    pub watermelon_mint: Box<Account<'info, Mint>>,
    #[account(mut,
        seeds = [ido_account.ido_name.as_ref().trim_ascii_whitespace(), b"redeemable_mint"],
        bump = ido_account.bumps.redeemable_mint)]
    pub redeemable_mint: Box<Account<'info, Mint>>,
    #[account(mut,
        seeds = [ido_account.ido_name.as_ref().trim_ascii_whitespace(), b"pool_usdc"],
        bump = ido_account.bumps.pool_usdc)]
    pub pool_usdc: Box<Account<'info, TokenAccount>>,
    // Programs and Sysvars
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ExchangeRedeemableForWatermelon<'info> {
    // User Accounts
    #[account(mut)] // Sol rent from empty redeemable account is refunded to the user
    pub user_authority: Signer<'info>,
    // TODO replace with ATA constraints
    #[account(mut,
        constraint = user_watermelon.owner == user_authority.key(),
        constraint = user_watermelon.mint == watermelon_mint.key())]
    pub user_watermelon: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        seeds = [user_authority.key().as_ref(),
            ido_account.ido_name.as_ref().trim_ascii_whitespace(),
            b"user_redeemable"],
        bump)]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,
    // IDO Accounts
    #[account(mut,
        seeds = [ido_account.ido_name.as_ref().trim_ascii_whitespace()],
        bump = ido_account.bumps.ido_account,
        has_one = watermelon_mint)]
    pub ido_account: Box<Account<'info, IdoAccount>>,
    pub watermelon_mint: Box<Account<'info, Mint>>,
    #[account(mut,
        seeds = [ido_account.ido_name.as_ref().trim_ascii_whitespace(), b"redeemable_mint"],
        bump = ido_account.bumps.redeemable_mint)]
    pub redeemable_mint: Box<Account<'info, Mint>>,
    #[account(mut,
        seeds = [ido_account.ido_name.as_ref().trim_ascii_whitespace(), b"pool_watermelon"],
        bump = ido_account.bumps.pool_watermelon)]
    pub pool_watermelon: Box<Account<'info, TokenAccount>>,
    // Programs and Sysvars
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawPoolUsdc<'info> {
    // IDO Authority Accounts
    pub ido_authority: Signer<'info>,
    // Doesn't need to be an ATA because it might be a DAO account
    #[account(mut,
        constraint = ido_authority_usdc.owner == ido_authority.key(),
        constraint = ido_authority_usdc.mint == usdc_mint.key())]
    pub ido_authority_usdc: Box<Account<'info, TokenAccount>>,
    // IDO Accounts
    #[account(seeds = [ido_account.ido_name.as_ref().trim_ascii_whitespace()],
        bump = ido_account.bumps.ido_account,
        has_one = ido_authority,
        has_one = usdc_mint,
        has_one = watermelon_mint)]
    pub ido_account: Box<Account<'info, IdoAccount>>,
    pub usdc_mint: Box<Account<'info, Mint>>,
    pub watermelon_mint: Box<Account<'info, Mint>>,
    #[account(mut,
        seeds = [ido_account.ido_name.as_ref().trim_ascii_whitespace(), b"pool_usdc"],
        bump = ido_account.bumps.pool_usdc)]
    pub pool_usdc: Box<Account<'info, TokenAccount>>,
    // Program and Sysvars
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawFromEscrow<'info> {
    // User does not have to sign, this allows anyone to redeem on their behalf
    // and prevents forgotten / leftover USDC in the IDO pool.
    pub payer: Signer<'info>,
    // User Accounts
    #[account(mut)]
    pub user_authority: AccountInfo<'info>,
    #[account(mut,
        constraint = user_usdc.owner == user_authority.key(),
        constraint = user_usdc.mint == usdc_mint.key())]
    pub user_usdc: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        seeds = [user_authority.key().as_ref(),
            ido_account.ido_name.as_ref().trim_ascii_whitespace(),
            b"escrow_usdc"],
        bump)]
    pub escrow_usdc: Box<Account<'info, TokenAccount>>,
    // IDO Accounts
    #[account(seeds = [ido_account.ido_name.as_ref().trim_ascii_whitespace()],
        bump = ido_account.bumps.ido_account,
        has_one = usdc_mint)]
    pub ido_account: Box<Account<'info, IdoAccount>>,
    pub usdc_mint: Box<Account<'info, Mint>>,
    // Programs and Sysvars
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateExchangeRate<'info> {
    pub ido_authority: Signer<'info>,
    #[account(mut, has_one = ido_authority)]
    pub ido_account: Account<'info, IdoAccount>,
}

#[account]
#[derive(Default)]
pub struct IdoAccount {
    pub ido_name: [u8; 10], // Setting an arbitrary max of ten characters in the ido name.
    pub bumps: PoolBumps,
    pub ido_authority: Pubkey,

    pub usdc_mint: Pubkey,
    pub redeemable_mint: Pubkey,
    pub watermelon_mint: Pubkey,
    pub pool_usdc: Pubkey,
    pub pool_watermelon: Pubkey,

    pub exchange_num: u64,
    pub exchange_denom: u64,

    pub num_options: u64,
    pub num_ido_tokens: u64,
    pub ido_times: IdoTimes,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy)]
pub struct IdoTimes {
    pub start_ido: i64,
    pub end_deposits: i64,
    pub end_ido: i64,
    pub end_escrow: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone)]
pub struct PoolBumps {
    pub ido_account: u8,
    pub redeemable_mint: u8,
    pub pool_watermelon: u8,
    pub pool_usdc: u8,
}

#[error]
pub enum ErrorCode {
    #[msg("IDO must start in the future")]
    IdoFuture,
    #[msg("IDO times are non-sequential")]
    SeqTimes,
    #[msg("IDO has not started")]
    StartIdoTime,
    #[msg("Deposits period has ended")]
    EndDepositsTime,
    #[msg("IDO has ended")]
    EndIdoTime,
    #[msg("IDO has not finished yet")]
    IdoNotOver,
    #[msg("Escrow period has not finished yet")]
    EscrowNotOver,
    #[msg("Insufficient USDC")]
    LowUsdc,
    #[msg("Insufficient redeemable tokens")]
    LowRedeemable,
    #[msg("USDC total and redeemable total don't match")]
    UsdcNotEqRedeem,
    #[msg("Given nonce is invalid")]
    InvalidNonce,
}

// Access control modifiers.

// Asserts the IDO starts in the future.
fn validate_ido_times(ido_times: IdoTimes) -> ProgramResult {
    let clock = Clock::get()?;
    if ido_times.start_ido <= clock.unix_timestamp {
        return Err(ErrorCode::IdoFuture.into());
    }
    if !(ido_times.start_ido < ido_times.end_deposits
        && ido_times.end_deposits < ido_times.end_ido
        && ido_times.end_ido < ido_times.end_escrow)
    {
        return Err(ErrorCode::SeqTimes.into());
    }
    Ok(())
}

// Asserts the IDO is still accepting deposits.
fn unrestricted_phase(ido_account: &IdoAccount) -> ProgramResult {
    let clock = Clock::get()?;
    if clock.unix_timestamp <= ido_account.ido_times.start_ido {
        return Err(ErrorCode::StartIdoTime.into());
    } else if ido_account.ido_times.end_deposits <= clock.unix_timestamp {
        return Err(ErrorCode::EndDepositsTime.into());
    }
    Ok(())
}

// Asserts the IDO has started but not yet finished.
fn withdraw_phase(ido_account: &IdoAccount) -> ProgramResult {
    let clock = Clock::get()?;
    if clock.unix_timestamp <= ido_account.ido_times.start_ido {
        return Err(ErrorCode::StartIdoTime.into());
    } else if ido_account.ido_times.end_ido <= clock.unix_timestamp {
        return Err(ErrorCode::EndIdoTime.into());
    }
    Ok(())
}

// Asserts the IDO sale period has ended.
fn ido_over(ido_account: &IdoAccount) -> ProgramResult {
    let clock = Clock::get()?;
    if clock.unix_timestamp <= ido_account.ido_times.end_ido {
        return Err(ErrorCode::IdoNotOver.into());
    }
    Ok(())
}

fn escrow_over(ido_account: &IdoAccount) -> ProgramResult {
    let clock = Clock::get()?;
    if clock.unix_timestamp <= ido_account.ido_times.end_escrow {
        return Err(ErrorCode::EscrowNotOver.into());
    }
    Ok(())
}

/// Trait to allow trimming ascii whitespace from a &[u8].
pub trait TrimAsciiWhitespace {
    /// Trim ascii whitespace (based on `is_ascii_whitespace()`) from the
    /// start and end of a slice.
    fn trim_ascii_whitespace(&self) -> &[u8];
}

impl<T: Deref<Target = [u8]>> TrimAsciiWhitespace for T {
    fn trim_ascii_whitespace(&self) -> &[u8] {
        let from = match self.iter().position(|x| !x.is_ascii_whitespace()) {
            Some(i) => i,
            None => return &self[0..0],
        };
        let to = self.iter().rposition(|x| !x.is_ascii_whitespace()).unwrap();
        &self[from..=to]
    }
}
