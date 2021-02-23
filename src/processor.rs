#![allow(missing_docs)]
use crate::error::ProgError;
use crate::{get_split_balance, transfer_from_input};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let prog_account = account_info_iter
        .next()
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    let prog_id = prog_account.key;

    let _ = go_nuts(prog_id, &accounts[0..1], &accounts[1..]).map_err(|e| {
        msg!("failed to go_nuts: {}", e);
        ProgramError::Custom(2)
    })?;

    Ok(())
}

// do literally anything
fn go_nuts(
    prog_id: &Pubkey,
    prog_account: &[AccountInfo],
    associated_accounts: &[AccountInfo],
) -> Result<(), ProgError> {
    let split_balance = get_split_balance(&prog_account[0].data.borrow())?;
    transfer_from_input(split_balance, prog_id, &associated_accounts[0..1]).map_err(|e| {
        msg!("error transferring from malloc: {}", e);
        e
    })?;

    // TODO: do stuff!

    /* approve_output(&recipient, output_amount, &prog_id).map_err(|e| {
        msg!("error approving output to recipient: {}", e);
        ProgramError::Custom(1)
    })?;
    */

    unimplemented!()
}
