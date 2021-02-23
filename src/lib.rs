#![allow(missing_docs)]

//! Template for Malloc Protocol's wrapped program calls

mod entrypoint;
mod error;
pub mod processor;

// Export current sdk types for downstream users building with a different sdk version
use core::str::FromStr;
use error::ProgError;
use serde::{Deserialize, Serialize};
pub use solana_program;
use solana_program::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::{
    instruction::{approve, transfer},
    state::Account,
};

// TODO figure out how to have static pubkeys
solana_program::declare_id!("ma11ocwca1111111111111111111111111111111111");
const TOKEN_PROG_ID: &'static str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const MALLOC_PROG_ID: &'static str = "ma11ocdevdevdevdevdevdevdevdevdevdevdevdevd";

/// transfer input SPL token to current token
pub fn transfer_from_input(
    amount: u64,
    prog_id: &Pubkey,
    recipient_account: &[AccountInfo],
) -> Result<(), ProgError> {
    let malloc_token_account =
        Pubkey::create_program_address(&[b"malloc"], &Pubkey::from_str(MALLOC_PROG_ID).unwrap())
            .map_err(|e| {
                msg!("error finding program-derived address!");
                ProgError::ProgDerivedAddrError
            })?;

    let wcall_token_account =
        Pubkey::create_program_address(&[b"wcall"], prog_id).map_err(|e| {
            msg!("error finding program-derived address!");
            ProgError::ProgDerivedAddrError
        })?;

    let insn = transfer(
        &Pubkey::from_str(TOKEN_PROG_ID).unwrap(),
        &malloc_token_account,
        &wcall_token_account,
        &wcall_token_account,
        &[&wcall_token_account],
        amount,
    )
    .map_err(|e| {
        msg!("error constructing SPL transfer: {}", e);
        ProgError::TransferError
    })?;
    invoke_signed(&insn, recipient_account, &[&[b"wcall"]]).map_err(|e| {
        msg!("error in SPL transfer: {}", e);
        ProgError::TransferError
    })?;

    Ok(())
}

/// call "Approve" on output SPL token for malloc contract to be able to enact next basket
pub fn approve_output(
    amount: u64,
    prog_id: &Pubkey,
    delegate_pubkey: &Pubkey,
    delegate_account: &[AccountInfo],
) -> Result<(), ProgError> {
    let wcall_pubkey = Pubkey::create_program_address(&[b"wcall"], prog_id).map_err(|e| {
        msg!("error finding program-derived address!");
        ProgError::ProgDerivedAddrError
    })?;
    let insn = approve(
        &Pubkey::from_str(TOKEN_PROG_ID).unwrap(),
        &wcall_pubkey,
        &delegate_pubkey,
        &wcall_pubkey,
        &[&wcall_pubkey],
        amount,
    )
    .map_err(|e| {
        msg!("error constructing SPL approve: {}", e);
        ProgError::ApproveError
    })?;

    invoke_signed(&insn, delegate_account, &[&[b"wcall"]]).map_err(|e| {
        msg!("error in SPL approve: {}", e);
        ProgError::ApproveError
    })?;

    Ok(())
}

pub fn get_split_balance(split_account_data: &[u8]) -> Result<u64, ProgError> {
    let state = Account::unpack_from_slice(split_account_data).map_err(|e| {
        msg!("falid to unpack split account data: {}", e);
        ProgError::InvalidState
    })?;
    Ok(state.amount)
}
