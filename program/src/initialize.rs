// use steel_amm_api::prelude::*;
// use steel::*;

// pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
//     // Load accounts.
//     let [signer_info, counter_info, system_program] = accounts else {
//         return Err(ProgramError::NotEnoughAccountKeys);
//     };
//     signer_info.is_signer()?;
//     counter_info.is_empty()?.is_writable()?.has_seeds(
//         &[COUNTER],
//         &steel_amm_api::ID
//     )?;
//     system_program.is_program(&system_program::ID)?;

//     // Initialize counter.
//     create_program_account::<Counter>(
//         counter_info,
//         system_program,
//         signer_info,
//         &steel_amm_api::ID,
//         &[COUNTER],
//     )?;
//     let counter = counter_info.as_account_mut::<Counter>(&steel_amm_api::ID)?;

//     counter.value = 0;
//     let state = true;
//     counter.running = match val_run {
//         1 => true,
//          0 => false,
//          _ => ProgramError::InvalidAccountData

//       };

//     Ok(())
// }
