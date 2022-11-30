use crate::error::StudentIntroError;
use crate::instruction::StudentIntroInstruction;
use crate::state::StudentIntroAccountState;
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use std::convert::TryInto;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = StudentIntroInstruction::unpack(instruction_data)?;
    match instruction {
        StudentIntroInstruction::AddStudentIntro { name, msg } => {
            add_student_intro(program_id, accounts, name, msg)
        }
        StudentIntroInstruction::UpdateStudentIntro { name, msg } => {
            update_student_intro(program_id, accounts, name, msg)
        }
    }
}

pub fn add_student_intro(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    msg: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if !initializer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), name.as_bytes().as_ref()],
        program_id,
    );
    msg!("Found PDA: {}", pda);
    if pda != *pda_account.key {
        return Err(StudentIntroError::InvalidPda.into());
    }
    let data_len = 1000;
    if 1 + 4 + name.len() + 4 + msg.len() > data_len {
        return Err(StudentIntroError::InvalidDataLength.into());
    }
    let rent_amt = Rent::get()?.minimum_balance(data_len);

    invoke_signed(
        &system_instruction::create_account(
            &initializer.key,
            &pda_account.key,
            rent_amt,
            data_len.try_into().unwrap(),
            program_id,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[initializer.key.as_ref(), name.as_bytes().as_ref(), &[bump]]],
    )?;
    msg!("Created PDA account successfully");
    msg!("Deserializing account data");
    msg!("Name: {}", name.clone());
    msg!("Msg: {}", msg.clone());
    let mut account_data =
        try_from_slice_unchecked::<StudentIntroAccountState>(&pda_account.data.borrow()).unwrap();
    account_data.name = name;
    account_data.msg = msg;
    account_data.is_initialized = true;
    msg!("Serializing account data");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("Serialization successful");
    Ok(())
}

pub fn update_student_intro(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    msg: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    if !initializer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if pda_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }
    msg!("Deserializing account data");
    let mut account_data =
        try_from_slice_unchecked::<StudentIntroAccountState>(&pda_account.data.borrow()).unwrap();
    let (pda, _bump) = Pubkey::find_program_address(
        &[
            initializer.key.as_ref(),
            account_data.name.as_bytes().as_ref(),
        ],
        program_id,
    );
    if pda != *pda_account.key {
        return Err(StudentIntroError::InvalidPda.into());
    }
    if !account_data.is_initialized() {
        return Err(StudentIntroError::UninitializedAccount.into());
    }
    if account_data.name != name {
        return Err(StudentIntroError::InvalidStudentName.into());
    }
    if 1 + 4 + name.len() + 4 + msg.len() > 1000 {
        return Err(StudentIntroError::InvalidDataLength.into());
    }
    account_data.msg = msg;
    msg!("Serializing account data");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    Ok(())
}
