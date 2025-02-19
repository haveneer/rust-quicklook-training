pub struct Machine {
    regs: [u32; 32],
    memory: Vec<u8>,
}

pub enum Instruction {
    Add { rd: usize, rs1: usize, rs2: usize },
    Load { rd: usize, addr: u32 },
    // etc...
}

// Un trait pour "exécuter" une instruction :
pub trait Emulable {
    fn execute(&mut self, instr: &Instruction);
}

// Implémentation du trait pour Machine
impl Emulable for Machine {
    fn execute(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Add { rd, rs1, rs2 } => {
                self.regs[*rd] = self.regs[*rs1] + self.regs[*rs2];
            }
            Instruction::Load { rd, addr } => {
                // Simpliste : pas de vérification d'out-of-bounds
                let val = self.memory[*addr as usize];
                self.regs[*rd] = val as u32;
            }
        }
    }
}
