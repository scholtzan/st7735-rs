use crate::st7735::Instruction;

/// System function command.
pub struct Command {
    pub instruction: Instruction,
    pub arguments: Vec<u8>,
    pub delay: Option<u64>,
}
