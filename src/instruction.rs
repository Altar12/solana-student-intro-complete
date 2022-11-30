use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

#[derive(BorshDeserialize)]
struct StudentIntroInstructionPayload {
    name: String,
    msg: String,
}

pub enum StudentIntroInstruction {
    AddStudentIntro { name: String, msg: String },
    UpdateStudentIntro { name: String, msg: String },
}

impl StudentIntroInstruction {
    pub fn unpack(data: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = data
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        let payload = StudentIntroInstructionPayload::try_from_slice(rest).unwrap();
        Ok(match variant {
            0 => Self::AddStudentIntro {
                name: payload.name,
                msg: payload.msg,
            },
            1 => Self::UpdateStudentIntro {
                name: payload.name,
                msg: payload.msg,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
