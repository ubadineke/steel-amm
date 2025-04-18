mod add;
mod create_pool;
mod initialize;

use add::*;
use create_pool::*;
use initialize::*;

use steel::*;
use steel_amm_api::prelude::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&steel_amm_api::ID, program_id, data)?;

    match ix {
        // SteelAmmInstruction::Initialize => process_initialize(accounts, data)?,
        // SteelAmmInstruction::Add => process_add(accounts, data)?,
        SteelAmmInstruction::CreatePool => process_create_pool(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
