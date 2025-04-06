use solana_program::pubkey::PubkeyError;
use steel::*;

use crate::prelude::*;

// pub fn initialize(signer: Pubkey) -> Instruction {
//     Instruction {
//         program_id: crate::ID,
//         accounts: vec![
//             AccountMeta::new(signer, true),
//             AccountMeta::new(counter_pda().0, false),
//             AccountMeta::new_readonly(system_program::ID, false),
//         ],
//         data: Initialize {}.to_bytes()
//     }
// }

// pub fn add(signer: Pubkey, amount: u64) -> Instruction {
//     Instruction {
//         program_id: crate::ID,
//         accounts: vec![
//             AccountMeta::new(signer, true),
//             AccountMeta::new(counter_pda().0, false),
//         ],
//         data: Add {
//             amount: amount.to_le_bytes(),
//         }
//         .to_bytes(),
//     }
// }

pub fn create_pool(
    signer: Pubkey,
    token_a_mint: Pubkey,
    token_b_mint: Pubkey,
    lp_mint: Pubkey,
    creator_token_a_ata: Pubkey,
    creator_token_b_ata: Pubkey,
    creator_token_lp_ata: Pubkey,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(config_pda().0, false),
            AccountMeta::new(pool_pda().0, false),
            AccountMeta::new(token_a_mint, false),
            AccountMeta::new(token_b_mint, false),
            AccountMeta::new(lp_mint, false),
            AccountMeta::new(vault_pda(&token_a_mint).0, false), //token_a_vault
            AccountMeta::new(vault_pda(&token_b_mint).0, false), //token_b_vault
            AccountMeta::new(creator_token_a_ata, false),
            AccountMeta::new(creator_token_b_ata, false),
            AccountMeta::new(creator_token_lp_ata, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(mpl_token_metadata::ID, false),
        ],
        data: CreatePool {}.to_bytes(),
    }
}

/// Derive the PDA of the config account.
pub fn config_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CONFIG], &crate::id())
}

/// Derive the PDA of the pool account.
pub fn pool_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[POOL], &crate::id())
}

pub fn vault_pda(mint_address: &Pubkey) -> (Pubkey, u8) {
    // Using the mint address as part of the PDA seed
    Pubkey::find_program_address(&[TOKEN_VAULT, mint_address.as_ref()], &crate::id())
}
