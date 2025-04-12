use steel::*;
use steel_amm_api::prelude::*;

pub fn process_create_pool(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Parse args.
    //   let args = Add::try_from_bytes(data)?;
    // let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [signer_info, config_info, pool_info, token_a_mint_info, token_b_mint_info, lp_mint_info, token_a_vault_info, token_b_vault_info, creator_token_a_ata_, creator_token_b_ata, creator_token_lp_ata, system_program, token_program, associated_token_program, token_metadata_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;

    //config checks?

    //pool init and checks.
    pool_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[POOL], &steel_amm_api::ID)?;

    // token_a_mint_info.as_mint()?.assert( |m|m < &token_b_mint_info.key);
    token_a_mint_info.as_mint()?;
    token_b_mint_info.as_mint()?;

    token_program.is_program(&spl_token::ID)?;
    system_program.is_program(&system_program::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;
    token_metadata_program.is_program(&mpl_token_metadata::ID)?;

    // let counter = counter_info
    //     .as_account_mut::<Counter>(&steel_amm_api::ID)?
    //     .assert_mut(|c| c.value < 100)?;

    // // Update state
    // counter.value += amount;

    create_program_account::<Pool>(
        pool_info,
        system_program,
        signer_info,
        &steel_amm_api::ID,
        &[
            POOL,
            config_info.key.as_ref(),
            token_a_mint_info.key.as_ref(),
            token_b_mint_info.key.as_ref(),
        ],
    )?;
    let pool = pool_info.as_account_mut::<Pool>(&steel_amm_api::ID)?;
    pool.amm_config = *config_info.key;
    pool.pool_creator = *signer_info.key;
    pool.token_a_vault = *token_a_vault_info.key;
    pool.token_b_vault = *token_b_vault_info.key;
    pool.lp_mint = *lp_mint_info.key;
    pool.token_a_mint = *token_a_mint_info.key;
    pool.token_b_mint = *token_b_mint_info.key;
    pool.lp_supply = 0;

    Ok(())
}
