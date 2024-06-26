use crate::common::abi::*;
use crate::common::build::*;

use self::{alu::AluBuilder, branch::BranchBuilder, forward::ForwardBuilder};
mod alu;
mod branch;
mod forward;
use alu::Alloc as AluAlloc;
use alu::Connect as AluConnect;
use branch::Alloc as BranchAlloc;
use branch::Connect as BranchConnect;
use forward::Alloc as ForwardAlloc;
use forward::Connect as ForwardConnect;

pub enum Alloc {
    BranchSel = 0,
    AluRes = 1,
    Rs2Data = 2,
}
pub enum Connect {
    Jal_ = 0,
    BranchEn = 1,
    PcSel = 2,
    ImmSel = 3,
    AluCtrl = 4,
    BranchType = 5,
    Pc = 6,
    Rs1Data = 7,
    Rs2Data = 8,
    Imm = 9,
    Rs1 = 10,
    Rs2 = 11,
    RdMem = 12,
    RdMemWrite = 13,
    RdMemData = 14,
    RdWb = 15,
    RdWbWrite = 16,
    RdWbData = 17,
}
pub struct ExStageBuilder {
    pub fwd_mux_1: MuxBuilder,
    pub fwd_mux_2: MuxBuilder,
    pub pc_sel: MuxBuilder,
    pub imm_sel: MuxBuilder,
    pub branch: BranchBuilder,
    pub forward: ForwardBuilder,
    pub alu: AluBuilder,
}
impl ExStageBuilder {
    pub fn new() -> Self {
        let mut fwd_mux_1 = MuxBuilder::default();
        let mut fwd_mux_2 = MuxBuilder::default();
        let mut pc_sel = MuxBuilder::default();
        let mut imm_sel = MuxBuilder::default();
        let mut branch = BranchBuilder::default();
        let mut forward = ForwardBuilder::default();
        let mut alu = AluBuilder::default();
        fwd_mux_1.connect(forward.alloc(ForwardAlloc::Forward1), MuxConnect::Select);
        branch.connect(fwd_mux_1.alloc(MuxAlloc::Out), BranchConnect::Op1);
        branch.connect(fwd_mux_2.alloc(MuxAlloc::Out), BranchConnect::Op2);
        pc_sel.connect(fwd_mux_1.alloc(MuxAlloc::Out), MuxConnect::In(0));
        alu.connect(pc_sel.alloc(MuxAlloc::Out), AluConnect::Op1);
        alu.connect(imm_sel.alloc(MuxAlloc::Out), AluConnect::Op2);
        imm_sel.connect(fwd_mux_2.alloc(MuxAlloc::Out), MuxConnect::In(0));
        fwd_mux_2.connect(forward.alloc(ForwardAlloc::Forward2), MuxConnect::Select);
        ExStageBuilder {
            fwd_mux_1,
            fwd_mux_2,
            pc_sel,
            imm_sel,
            branch,
            forward,
            alu,
        }
    }
}
impl Default for ExStageBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ControlBuilder for ExStageBuilder {
    fn build(self) -> ControlRef {
        Exstage {
            fwd_mux_1: self.fwd_mux_1.build(),
            fwd_mux_2: self.fwd_mux_2.build(),
            pc_sel: self.pc_sel.build(),
            imm_sel: self.imm_sel.build(),
            branch: self.branch.build(),
            forward: self.forward.build(),
            alu: self.alu.build(),
        }
        .into()
    }
}
impl PortBuilder for ExStageBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, id: Alloc) -> PortRef {
        match id {
            Alloc::BranchSel => self.branch.alloc(BranchAlloc::BK),
            Alloc::AluRes => self.alu.alloc(AluAlloc::Res),
            Alloc::Rs2Data => self.fwd_mux_2.alloc(MuxAlloc::Out),
        }
    }
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::Jal_ => self.branch.connect(pin, BranchConnect::Jal_),
            Connect::BranchEn => self.branch.connect(pin, BranchConnect::BranchSel),
            Connect::PcSel => self.pc_sel.connect(pin, MuxConnect::Select),
            Connect::ImmSel => self.imm_sel.connect(pin, MuxConnect::Select),
            Connect::AluCtrl => self.alu.connect(pin, AluConnect::Ctrl),
            Connect::BranchType => self.branch.connect(pin, BranchConnect::BranchType),
            Connect::Pc => self.pc_sel.connect(pin, MuxConnect::In(1)),
            Connect::Rs1Data => self.fwd_mux_1.connect(pin, MuxConnect::In(0)),
            Connect::Rs2Data => self.fwd_mux_2.connect(pin, MuxConnect::In(0)),
            Connect::Imm => self.imm_sel.connect(pin, MuxConnect::In(1)),
            Connect::Rs1 => self.forward.connect(pin, ForwardConnect::Rs1),
            Connect::Rs2 => self.forward.connect(pin, ForwardConnect::Rs2),
            Connect::RdMem => self.forward.connect(pin, ForwardConnect::RdMem),
            Connect::RdMemWrite => self.forward.connect(pin, ForwardConnect::RdMemWrite),
            Connect::RdMemData => {
                self.fwd_mux_1.connect(pin.clone(), MuxConnect::In(1));
                self.fwd_mux_2.connect(pin, MuxConnect::In(1));
            }
            Connect::RdWb => self.forward.connect(pin, ForwardConnect::RdWb),
            Connect::RdWbWrite => self.forward.connect(pin, ForwardConnect::RdWbWrite),
            Connect::RdWbData => {
                self.fwd_mux_1.connect(pin.clone(), MuxConnect::In(2));
                self.fwd_mux_2.connect(pin, MuxConnect::In(2));
            }
        }
    }
}
#[derive(Debug)]
pub struct Exstage {
    pub fwd_mux_1: ControlRef,
    pub fwd_mux_2: ControlRef,
    pub pc_sel: ControlRef,
    pub imm_sel: ControlRef,
    pub branch: ControlRef,
    pub forward: ControlRef,
    pub alu: ControlRef,
}
impl Control for Exstage {
    fn output(&self) -> Vec<(&'static str, u32)> {
        let mut res = Vec::new();
        res.extend(self.branch.output());
        res.extend(self.alu.output());
        res.extend(self.forward.output());
        res.extend(self.fwd_mux_1.output());
        res.extend(self.fwd_mux_2.output());
        res
    }
    fn inner_signal(&self) -> Vec<(&'static str, u32)> {
        let mut res = vec![
            ("fwd1_data", self.fwd_mux_1.output()[0].1),
            ("fwd2_data", self.fwd_mux_2.output()[0].1),
            ("op1", self.pc_sel.output()[0].1),
            ("op2", self.imm_sel.output()[0].1),
            ("npc_en", self.branch.output()[0].1),
        ];
        res.extend(self.forward.output());
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    pub struct TestConnect {
        pub jal_: u32,
        pub branch_sel: u32,
        pub pc_sel: u32,
        pub imm_sel: u32,
        pub alu_ctrl: u32,
        pub branch_type: u32,
        pub pc: u32,
        pub rs1_data: u32,
        pub rs2_data: u32,
        pub imm: u32,
        pub rs1: u32,
        pub rs2: u32,
        pub rd_mem: u32,
        pub rd_mem_write: u32,
        pub rd_mem_data: u32,
        pub rd_wb: u32,
        pub rd_wb_write: u32,
        pub rd_wb_data: u32,
    }
    pub struct TestAlloc {
        pub branch_sel: u32,
        pub alu_result: u32,
        pub fwd_1: u32,
        pub fwd_2: u32,
        pub fwd_mux_1: u32,
        pub fwd_mux_2: u32,
        pub alu_op1: u32,
        pub alu_op2: u32,
    }
    fn run_test(connect: TestConnect, alloc: TestAlloc) {
        let mut tb = ExStageBuilder::new();
        let bk = tb.alloc(Alloc::BranchSel);
        let alu_result = tb.alloc(Alloc::AluRes);
        let fwd_1 = tb.forward.alloc(ForwardAlloc::Forward1);
        let fwd_2 = tb.forward.alloc(ForwardAlloc::Forward2);
        let fwd_mux_1 = tb.fwd_mux_1.alloc(MuxAlloc::Out);
        let fwd_mux_2 = tb.fwd_mux_2.alloc(MuxAlloc::Out);
        let alu_op1 = tb.pc_sel.alloc(MuxAlloc::Out);
        let alu_op2 = tb.imm_sel.alloc(MuxAlloc::Out);
        let mut constant = ConstsBuilder::default();
        tb.connect(
            constant.alloc(ConstsAlloc::Out(connect.jal_)),
            Connect::Jal_,
        );
        tb.connect(
            constant.alloc(ConstsAlloc::Out(connect.branch_sel)),
            Connect::BranchEn,
        );
        tb.connect(
            constant.alloc(ConstsAlloc::Out(connect.pc_sel)),
            Connect::PcSel,
        );
        tb.connect(
            constant.alloc(ConstsAlloc::Out(connect.imm_sel)),
            Connect::ImmSel,
        );
        tb.connect(
            constant.alloc(ConstsAlloc::Out(connect.alu_ctrl)),
            Connect::AluCtrl,
        );
        tb.connect(
            constant.alloc(ConstsAlloc::Out(connect.branch_type)),
            Connect::BranchType,
        );
        tb.connect(constant.alloc(ConstsAlloc::Out(connect.pc)), Connect::Pc);
        tb.connect(
            constant.alloc(ConstsAlloc::Out(connect.rs1_data)),
            Connect::Rs1Data,
        );
        tb.connect(
            constant.alloc(ConstsAlloc::Out(connect.rs2_data)),
            Connect::Rs2Data,
        );
        tb.connect(constant.alloc(ConstsAlloc::Out(connect.imm)), Connect::Imm);
        tb.connect(constant.alloc(ConstsAlloc::Out(connect.rs1)), Connect::Rs1);
        tb.connect(constant.alloc(ConstsAlloc::Out(connect.rs2)), Connect::Rs2);
        tb.connect(
            constant.alloc(ConstsAlloc::Out(connect.rd_mem)),
            Connect::RdMem,
        );
        tb.connect(
            constant.alloc(ConstsAlloc::Out(connect.rd_mem_write)),
            Connect::RdMemWrite,
        );
        tb.connect(
            constant.alloc(ConstsAlloc::Out(connect.rd_mem_data)),
            Connect::RdMemData,
        );
        tb.connect(
            constant.alloc(ConstsAlloc::Out(connect.rd_wb)),
            Connect::RdWb,
        );
        tb.connect(
            constant.alloc(ConstsAlloc::Out(connect.rd_wb_write)),
            Connect::RdWbWrite,
        );
        tb.connect(
            constant.alloc(ConstsAlloc::Out(connect.rd_wb_data)),
            Connect::RdWbData,
        );
        assert_eq!(bk.read(), alloc.branch_sel);
        assert_eq!(fwd_1.read(), alloc.fwd_1);
        assert_eq!(fwd_2.read(), alloc.fwd_2);
        assert_eq!(fwd_mux_1.read(), alloc.fwd_mux_1);
        assert_eq!(fwd_mux_2.read(), alloc.fwd_mux_2);
        assert_eq!(alu_op1.read(), alloc.alu_op1);
        assert_eq!(alu_op2.read(), alloc.alu_op2);
        assert_eq!(alu_result.read(), alloc.alu_result);
    }
    #[test]
    fn test_ex0() {
        //e5010113        addi x2 x2 -432
        run_test(
            TestConnect {
                jal_: 0,
                branch_sel: 0,
                pc_sel: 0,
                imm_sel: 1,
                alu_ctrl: 1,
                branch_type: 0,
                pc: 0x10054,
                rs1_data: 0x7ffffff0,
                rs2_data: 0,
                imm: 0xfffffe50,
                rs1: 2,
                rs2: 0x10,
                rd_mem: 0,
                rd_mem_write: 0,
                rd_mem_data: 0xdeadbeef,
                rd_wb: 0,
                rd_wb_write: 0,
                rd_wb_data: 0,
            },
            TestAlloc {
                branch_sel: 0,
                alu_result: 0x7ffffe40,
                fwd_1: 0,
                fwd_2: 0,
                fwd_mux_1: 0x7ffffff0,
                fwd_mux_2: 0,
                alu_op1: 0x7ffffff0,
                alu_op2: 0xfffffe50,
            },
        );
    }
    #[test]
    fn test_ex1() {
        //1a812623        sw x8 428 x2
        run_test(
            TestConnect {
                jal_: 0,
                branch_sel: 0,
                pc_sel: 0,
                imm_sel: 1,
                alu_ctrl: 1,
                branch_type: 0,
                pc: 0x10058,
                rs1_data: 0x7ffffff0,
                rs2_data: 0,
                imm: 0x1ac,
                rs1: 2,
                rs2: 8,
                rd_mem: 2,
                rd_mem_write: 1,
                rd_mem_data: 0x7ffffe40,
                rd_wb: 0,
                rd_wb_write: 0,
                rd_wb_data: 0xdeadbeef,
            },
            TestAlloc {
                branch_sel: 0,
                alu_result: 0x7fffffec,
                fwd_1: 1,
                fwd_2: 0,
                fwd_mux_1: 0x7ffffe40,
                fwd_mux_2: 0,
                alu_op1: 0x7ffffe40,
                alu_op2: 0x1ac,
            },
        );
    }
    #[test]
    fn test_ex2() {
        //0280006f        jal x0 40
        run_test(
            TestConnect {
                jal_: 1,
                branch_sel: 0,
                pc_sel: 1,
                imm_sel: 1,
                alu_ctrl: 1,
                branch_type: 0,
                pc: 0x10064,
                rs1_data: 0,
                rs2_data: 0,
                imm: 0x28,
                rs1: 0,
                rs2: 8,
                rd_mem: 0xc,
                rd_mem_write: 0,
                rd_mem_data: 0x7fffffdc,
                rd_wb: 8,
                rd_wb_write: 1,
                rd_wb_data: 0x7ffffff0,
            },
            TestAlloc {
                branch_sel: 1,
                alu_result: 0x1008c,
                fwd_1: 0,
                fwd_2: 2,
                fwd_mux_1: 0,
                fwd_mux_2: 0x7ffffff0,
                alu_op1: 0x10064,
                alu_op2: 0x28,
            },
        );
    }
}
