use steel::*;

use super::SteelAmmAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Pool {
    /// Which config the pool belongs
    pub amm_config: Pubkey,
    /// pool creator
    pub pool_creator: Pubkey,
    /// Token A
    pub token_a_vault: Pubkey,
    /// Token B
    pub token_b_vault: Pubkey,

    /// Pool tokens are issued when A or B tokens are deposited.
    /// Pool tokens can be withdrawn back to the original A or B token.
    pub lp_mint: Pubkey,
    /// Mint information for token A
    pub token_a_mint: Pubkey,
    /// Mint information for token B
    pub token_b_mint: Pubkey,

    /// token_a program
    pub token_a_program: Pubkey,
    /// token_b program
    pub token_b_program: Pubkey,

    /// True circulating supply without burns and lock ups
    pub lp_supply: u64,

    // pub lp_mint_decimals: u8,
    // mint0 and mint1 decimals
    // pub mint_a_decimals: u8,
    // pub mint_b_decimals: u8,

    //padding for future updates
    // pub padding: [u8; 5], // observation account to store oracle data
                          // pub observation_key: Pubkey,

                          // The amounts of token_0 and token_1 that are owed to the liquidity provider.
                          // pub protocol_fees_token_a: u64,
                          // pub protocol_fees_token_b: u64,

                          // pub fund_fees_token_a: u64,
                          // pub fund_fees_token_b: u64,

                          // The timestamp allowed for swap in the pool.
                          // pub open_time: u64,
                          // recent epoch
                          // pub recent_epoch: u64,
                          // padding for future updates
                          // pub padding: [u64; 31],
}

account!(SteelAmmAccount, Pool);
