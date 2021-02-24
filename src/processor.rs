#![allow(missing_docs)]
use crate::error::ProgError;
use crate::{get_split_balance, transfer};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Instruction processor
/// expects two accounts (aside from program account): source, dest
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

    let _ = go_nuts(prog_id, accounts).map_err(|e| {
        msg!("failed to go_nuts: {}", e);
        ProgramError::Custom(2)
    })?;

    Ok(())
}

// do literally anything
fn go_nuts(prog_id: &Pubkey, accounts: &[AccountInfo]) -> Result<(), ProgError> {
    let split_balance = get_split_balance(&accounts[1].data.borrow())?;
    transfer(
        split_balance,
        prog_id,
        &[
            accounts[1].to_owned(),
            accounts[2].to_owned(),
            accounts[0].to_owned(),
        ],
    )
    .map_err(|e| {
        msg!("error transferring from malloc: {}", e);
        e
    })?;

    /* approve_output(&recipient, output_amount, &prog_id).map_err(|e| {
        msg!("error approving output to recipient: {}", e);
        ProgramError::Custom(1)
    })?;
    */
    Ok(())
}
