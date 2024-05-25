use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use serum_dex::context::MarketContext;
use serum_dex::instruction::{NewOrderInstructionV3, SelfTradeBehavior};
use serum_dex::state::Market;
use serum_dex::matching::Side;

declare_id!("CDvQtfPPKVkAfrcC9PJuQYFdj46Zk2fje2q4rmpgT6dh"); // Replace with your program's actual ID

#[program]
pub mod token_swap {
    use super::*;

    pub fn swap(ctx: Context<Swap>, amount: u64) -> ProgramResult {
        let market = Market::load(&ctx.accounts.market, &program_id()).unwrap();

        // Prepare the Serum DEX new order instruction
        let new_order_instruction = NewOrderInstructionV3 {
            side: if ctx.accounts.input_token_account.mint == market.own_address {
                Side::Ask // Selling base token for quote
            } else {
                Side::Bid // Buying base token with quote
            },
            limit_price: 0, // market order (execute immediately at best price)
            max_coin_qty: amount, // Maximum amount of base token to buy/sell
            max_native_pc_qty_including_fees: u64::MAX, // Spend whatever quote is necessary
            self_trade_behavior: SelfTradeBehavior::DecrementTake,
            order_type: serum_dex::instruction::OrderType::ImmediateOrCancel,
            client_order_id: 0, // Optional
            limit: u16::MAX, // Optional
            // ... other fields (see Serum docs for details)
        };

        let market_ctx = MarketContext::new(
            &ctx.accounts.market,
            &ctx.accounts.open_orders,
            &ctx.accounts.request_queue,
            &ctx.accounts.event_queue,
            &ctx.accounts.bids,
            &ctx.accounts.asks,
            &ctx.accounts.coin_vault,
            &ctx.accounts.pc_vault,
            None,
        );
        market_ctx.new_order_v3(
            &ctx.accounts.payer, 
            ctx.accounts.user.key,
            new_order_instruction,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    #[account(mut)]
    pub input_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub output_token_account: Account<'info, TokenAccount>,
    pub market: AccountInfo<'info>, // Serum DEX market account
    pub request_queue: AccountInfo<'info>,
    pub event_queue: AccountInfo<'info>,
    pub bids: AccountInfo<'info>,
    pub asks: AccountInfo<'info>,
    #[account(mut)]
    pub coin_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pc_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>, // Optional for existing open orders
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
