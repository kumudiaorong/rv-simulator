use crate::common::abi::*;
use crate::common::build::*;

pub enum Alloc {
    RegWrite = 0,
    WbSel = 1,
    Npc = 2,
    AluRes = 3,
    MemData = 4,
    Rd = 5,
}

impl From<Alloc> for usize {
    fn from(id: Alloc) -> Self {
        match id {
            Alloc::RegWrite => 0,
            Alloc::WbSel => 1,
            Alloc::Npc => 2,
            Alloc::AluRes => 3,
            Alloc::MemData => 4,
            Alloc::Rd => 5,
        }
    }
}

pub enum Connect {
    RegWrite = 0,
    WbSel = 1,
    Npc = 2,
    AluRes = 3,
    MemData = 4,
    Rd = 5,
    Enable = 6,
    Clear = 7,
}

impl From<Connect> for usize {
    fn from(id: Connect) -> Self {
        match id {
            Connect::RegWrite => 0,
            Connect::WbSel => 1,
            Connect::Npc => 2,
            Connect::AluRes => 3,
            Connect::MemData => 4,
            Connect::Rd => 5,
            Connect::Enable => 6,
            Connect::Clear => 7,
        }
    }
}

#[derive(Default)]
pub struct MemWbBuilder {
    pub reg_write: RegBuilder,
    pub wb_sel: RegBuilder,
    pub npc: RegBuilder,
    pub alu_res: RegBuilder,
    pub mem_data: RegBuilder,
    pub rd: RegBuilder,
    pub asm: AsmRegBuilder,
}
impl AsmBuilder for MemWbBuilder {
    fn asm_connect(&mut self, pin: AsmPortRef, id: usize) {
        match id {
            0 => self.asm.asm_connect(pin, AsmRegConnect::In.into()),
            _ => panic!("MemWbBuilder: don't need to asm connect"),
        }
    }
    fn asm_alloc(&self, id: usize) -> AsmPortRef {
        self.asm.asm_alloc(id)
    }
}
impl ControlBuilder for MemWbBuilder {
    fn build(self) -> ControlRef {
        Some(ControlRef::from(ControlShared::new(MemWb {
            reg_write: self.reg_write.build().unwrap(),
            wb_sel: self.wb_sel.build().unwrap(),
            npc: self.npc.build().unwrap(),
            alu_res: self.alu_res.build().unwrap(),
            mem_data: self.mem_data.build().unwrap(),
            rd: self.rd.build().unwrap(),
            asm: self.asm.build(),
        })))
    }
}
impl PortBuilder for MemWbBuilder {
    fn alloc(&mut self, id: usize) -> PortRef {
        match id {
            0 => self.reg_write.alloc(RegAlloc::Out.into()),
            1 => self.wb_sel.alloc(RegAlloc::Out.into()),
            2 => self.npc.alloc(RegAlloc::Out.into()),
            3 => self.alu_res.alloc(RegAlloc::Out.into()),
            4 => self.mem_data.alloc(RegAlloc::Out.into()),
            5 => self.rd.alloc(RegAlloc::Out.into()),
            _ => panic!("Invalid id"),
        }
    }
    fn connect(&mut self, pin: PortRef, id: usize) {
        match id {
            0 => self.reg_write.connect(pin, RegConnect::In.into()),
            1 => self.wb_sel.connect(pin, RegConnect::In.into()),
            2 => self.npc.connect(pin, RegConnect::In.into()),
            3 => self.alu_res.connect(pin, RegConnect::In.into()),
            4 => self.mem_data.connect(pin, RegConnect::In.into()),
            5 => self.rd.connect(pin, RegConnect::In.into()),
            6 => {
                self.reg_write
                    .connect(pin.clone(), RegConnect::Enable.into());
                self.wb_sel.connect(pin.clone(), RegConnect::Enable.into());
                self.npc.connect(pin.clone(), RegConnect::Enable.into());
                self.alu_res.connect(pin.clone(), RegConnect::Enable.into());
                self.mem_data
                    .connect(pin.clone(), RegConnect::Enable.into());
                self.rd.connect(pin, RegConnect::Enable.into());
            }
            7 => {
                self.reg_write
                    .connect(pin.clone(), RegConnect::Clear.into());
                self.wb_sel.connect(pin.clone(), RegConnect::Clear.into());
                self.npc.connect(pin.clone(), RegConnect::Clear.into());
                self.alu_res.connect(pin.clone(), RegConnect::Clear.into());
                self.mem_data.connect(pin.clone(), RegConnect::Clear.into());
                self.rd.connect(pin, RegConnect::Clear.into());
            }
            _ => panic!("Invalid id"),
        }
    }
}

pub struct MemWb {
    pub reg_write: ControlRef,
    pub wb_sel: ControlRef,
    pub npc: ControlRef,
    pub alu_res: ControlRef,
    pub mem_data: ControlRef,
    pub rd: ControlRef,
    pub asm: ControlRef,
}

impl Control for MemWb {
    fn rasing_edge(&mut self) {
        self.reg_write.rasing_edge();
        self.wb_sel.rasing_edge();
        self.npc.rasing_edge();
        self.alu_res.rasing_edge();
        self.mem_data.rasing_edge();
        self.rd.rasing_edge();
        self.asm.rasing_edge();
    }
    fn falling_edge(&mut self) {
        self.reg_write.falling_edge();
        self.wb_sel.falling_edge();
        self.npc.falling_edge();
        self.alu_res.falling_edge();
        self.mem_data.falling_edge();
        self.rd.falling_edge();
        self.asm.falling_edge();
    }
    #[cfg(debug_assertions)]
    fn debug(&self) -> String {
        format!(
            "MEM/WB : {}\nREG_WRITE\t: {:8} WB_SEL\t: {:8} NPC\t\t: {:8} ALU_RES\t: {:8} MEM_DATA\t: {:8}\nRD\t\t: {}",
            self.asm.debug(),
            self.reg_write.debug(),
            self.wb_sel.debug(),
            self.npc.debug(),
            self.alu_res.debug(),
            self.mem_data.debug(),
            self.rd.debug()
        )
    }
}

pub mod build {
    pub use super::Alloc as MemWbAlloc;
    pub use super::Connect as MemWbConnect;
    pub use super::MemWbBuilder;
}