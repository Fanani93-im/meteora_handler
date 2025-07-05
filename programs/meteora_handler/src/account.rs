use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DlmmSwap<'info> {
    #[account(mut)]
    /// CHECK: Pool DLMM utama
    pub lb_pair: UncheckedAccount<'info>,

    /// CHECK: Bin array extension
    pub bin_array_bitmap_extension: Option<UncheckedAccount<'info>>,

    #[account(mut)]
    pub reserve_x: UncheckedAccount<'info>,
    #[account(mut)]
    pub reserve_y: UncheckedAccount<'info>,

    #[account(mut)]
    pub user_token_in: UncheckedAccount<'info>,
    #[account(mut)]
    pub user_token_out: UncheckedAccount<'info>,

    /// CHECK: Mint X & Y
    pub token_x_mint: UncheckedAccount<'info>,
    pub token_y_mint: UncheckedAccount<'info>,

    #[account(mut)]
    pub oracle: UncheckedAccount<'info>,
    pub host_fee_in: Option<UncheckedAccount<'info>>,

    pub user: Signer<'info>,

    #[account(address = crate::dlmm::ID)]
    pub dlmm_program: UncheckedAccount<'info>,

    pub event_authority: UncheckedAccount<'info>,
    pub token_x_program: UncheckedAccount<'info>,
    pub token_y_program: UncheckedAccount<'info>,
    // Akun bin ticks via remaining_accounts[]
}
