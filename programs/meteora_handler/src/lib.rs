use anchor_lang::prelude::*;

pub mod dlmm;         // Berisi fungsi handler CPI swap DLMM
pub mod accounts;     // Berisi layout akun DlmmSwap

use dlmm::*;

/// Program utama smart contract kamu
#[program]
pub mod meteora_handler {
    use super::*;

    /// Fungsi entry untuk swap CPI
    pub fn swap_meteora(
        ctx: Context<DlmmSwap>,
        amount_in: u64,
        min_out: u64,
    ) -> Result<()> {
        handle_dlmm_swap(ctx, amount_in, min_out)
    }
}
