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
    instruction::{approve as spl_approve, transfer as spl_transfer},
    state::Account,
};

pub const WCALL_SEED: &[u8] = b"wcall";
pub const TOKEN_PROG_ID: &'static str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const MALLOC_PROG_ID: &'static str = "ma11ocdevdevdevdevdevdevdevdevdevdevdevdevd";

solana_program::declare_id!("ma11ocwca1111111111111111111111111111111111");

/// call "Transfer" for SPL token
/// three account infos: source, destination, signer (delegate / owner)
pub fn transfer(
    amount: u64,
    prog_id: &Pubkey,
    account_infos: &[AccountInfo],
) -> Result<(), ProgError> {
    let wcall_pubkey = Pubkey::create_program_address(&[WCALL_SEED], prog_id).map_err(|e| {
        msg!("error finding program-derived address!");
        ProgError::ProgDerivedAddrError
    })?;

    let insn = spl_transfer(
        &Pubkey::from_str(TOKEN_PROG_ID).unwrap(),
        &account_infos[0].key,
        &account_infos[1].key,
        &wcall_pubkey,
        &[&wcall_pubkey],
        amount,
    )
    .map_err(|e| {
        msg!("error constructing SPL transfer: {}", e);
        ProgError::TransferError
    })?;
    invoke_signed(&insn, account_infos, &[&[WCALL_SEED]]).map_err(|e| {
        msg!("error in SPL transfer: {}", e);
        ProgError::TransferError
    })?;

    Ok(())
}

/// call "Approve" for SPL token
/// three account infos: source, delegate, owner of source
pub fn approve(
    amount: u64,
    prog_id: &Pubkey,
    account_infos: &[AccountInfo],
) -> Result<(), ProgError> {
    let wcall_pubkey = Pubkey::create_program_address(&[WCALL_SEED], prog_id).map_err(|e| {
        msg!("error finding program-derived address!");
        ProgError::ProgDerivedAddrError
    })?;
    let insn = spl_approve(
        &Pubkey::from_str(TOKEN_PROG_ID).unwrap(),
        &account_infos[0].key,
        &account_infos[1].key,
        &wcall_pubkey,
        &[&wcall_pubkey],
        amount,
    )
    .map_err(|e| {
        msg!("error constructing SPL approve: {}", e);
        ProgError::ApproveError
    })?;

    invoke_signed(&insn, account_infos, &[&[WCALL_SEED]]).map_err(|e| {
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
