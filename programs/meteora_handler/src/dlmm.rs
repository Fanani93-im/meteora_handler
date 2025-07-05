use anchor_lang::prelude::*;
use crate::accounts::DlmmSwap;

pub mod dlmm {
    declare_id!("DLMM_PROGRAM_ID_REPLACE_ME"); // ganti pakai ID DLMM asli
}

pub fn handle_dlmm_swap<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, DlmmSwap<'info>>,
    amount_in: u64,
    min_out: u64,
) -> Result<()> {
    let accounts = meteora_dlmm::cpi::accounts::Swap {
        lb_pair: ctx.accounts.lb_pair.to_account_info(),
        bin_array_bitmap_extension: ctx.accounts.bin_array_bitmap_extension.as_ref().map(|a| a.to_account_info()),
        reserve_x: ctx.accounts.reserve_x.to_account_info(),
        reserve_y: ctx.accounts.reserve_y.to_account_info(),
        user_token_in: ctx.accounts.user_token_in.to_account_info(),
        user_token_out: ctx.accounts.user_token_out.to_account_info(),
        token_x_mint: ctx.accounts.token_x_mint.to_account_info(),
        token_y_mint: ctx.accounts.token_y_mint.to_account_info(),
        oracle: ctx.accounts.oracle.to_account_info(),
        host_fee_in: ctx.accounts.host_fee_in.as_ref().map(|a| a.to_account_info()),
        user: ctx.accounts.user.to_account_info(),
        token_x_program: ctx.accounts.token_x_program.to_account_info(),
        token_y_program: ctx.accounts.token_y_program.to_account_info(),
        event_authority: ctx.accounts.event_authority.to_account_info(),
        program: ctx.accounts.dlmm_program.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.dlmm_program.to_account_info(), accounts)
        .with_remaining_accounts(ctx.remaining_accounts.to_vec());

    meteora_dlmm::cpi::swap(cpi_ctx, amount_in, min_out)
}
