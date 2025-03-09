use crate::emulable::Instruction;

pub fn parse_instruction(buf: &[u8]) -> Option<Instruction> {
    // Imaginons un format binaire minimal :
    // Byte 0 = opcode (0x01 = Add, 0x02 = Load)
    // Bytes 1.. = params
    let opcode = buf.get(0)?;
    match opcode {
        0x01 => {
            let rd = *buf.get(1)? as usize;
            let rs1 = *buf.get(2)? as usize;
            let rs2 = *buf.get(3)? as usize;
            Some(Instruction::Add { rd, rs1, rs2 })
        }
        0x02 => {
            let rd = *buf.get(1)? as usize;
            let addr = *buf.get(2)? as u32; // simpliste...
            Some(Instruction::Load { rd, addr })
        }
        _ => None,
    }
}
