use ztar_rod::asm::Instruction;

#[test]
#[ignore = "takes a long time (2 min) --- checks every possible 4-byte encoding"]
fn decode_then_encode_works() {
    // for every possible byte sequence:
    // if it is a valid instruction, then it should encode back to the same bytes
    for i in 0..=0xffff_ffffu32 {
        let bytes = i.to_be_bytes();

        match Instruction::decode(bytes) {
            None => (),
            Some(inst) => assert_eq!(inst.encode(), bytes, "decoded as {}", inst),
        }
    }
}

#[allow(non_snake_case)]
mod encode_then_decode {
    use itertools::iproduct;
    use std::convert::TryFrom;

    // some iterators to make it easier to generate instructions

    use ztar_rod::asm::{Coprocessor, Instruction, Register};

    fn reg() -> impl Iterator<Item = Register> + Clone {
        (0..=31).into_iter().map(|r| Register::try_from(r).unwrap())
    }
    fn target() -> impl Iterator<Item = u32> + Clone {
        [0, 4, 8, 0x0fff_fffc].into_iter().cloned()
    }
    fn imm() -> impl Iterator<Item = i16> + Clone {
        [-32768, -32767, -256, -128, -1, 0, 1, 128, 256, 32766, 32767]
            .into_iter()
            .cloned()
    }
    fn cofun() -> impl Iterator<Item = u32> + Clone {
        [0, 1, 0x1ff_fffe, 0x1ff_ffff].into_iter().cloned()
    }
    fn cacheop() -> impl Iterator<Item = u8> + Clone {
        (0..=31).into_iter()
    }
    fn cop() -> impl Iterator<Item = Coprocessor> + Clone {
        [Coprocessor::COP1, Coprocessor::COP2].into_iter().cloned()
    }
    fn shift() -> impl Iterator<Item = u8> + Clone {
        (0..=31).into_iter()
    }
    fn code10() -> impl Iterator<Item = u16> + Clone {
        [0, 1, 2, 0x3fe, 0x3ff].into_iter().cloned()
    }
    fn code20() -> impl Iterator<Item = u32> + Clone {
        [0, 1, 2, 0xf_fffe, 0xf_ffff].into_iter().cloned()
    }

    macro_rules! with_args {
        ($( $inst:ident $($arg:ident),+ <= $($gen:expr),+ ; )*) => {
            $(
                #[allow(unused_parens)]
                #[test]
                fn $inst() {
                    for ($($arg),+) in iproduct!($($gen),+) {
                        let i = Instruction::$inst($($arg),+);
                        let encoded = i.encode();
                        let decoded = Instruction::decode(encoded).unwrap();
                        assert_eq!(decoded, i, "encoded as {:02X?}", encoded);
                    }
                }
            )*
        }
    }

    with_args! {
        ADD     rd, rs, rt            <= reg(), reg(), reg();
        ADDI    rt, rs, immediate     <= reg(), reg(), imm();
        ADDIU   rt, rs, immediate     <= reg(), reg(), imm();
        ADDU    rd, rs, rt            <= reg(), reg(), reg();
        AND     rd, rs, rt            <= reg(), reg(), reg();
        ANDI    rt, rs, immediate     <= reg(), reg(), imm();
        BCzF    cop, offset           <= cop(), imm();
        BCzFL   cop, offset           <= cop(), imm();
        BCzT    cop, offset           <= cop(), imm();
        BCzTL   cop, offset           <= cop(), imm();
        BEQ     rs, rt, offset        <= reg(), reg(), imm();
        BEQL    rs, rt, offset        <= reg(), reg(), imm();
        BGEZ    rs, offset            <= reg(), imm();
        BGEZAL  rs, offset            <= reg(), imm();
        BGEZALL rs, offset            <= reg(), imm();
        BGEZL   rs, offset            <= reg(), imm();
        BGTZ    rs, offset            <= reg(), imm();
        BGTZL   rs, offset            <= reg(), imm();
        BLEZ    rs, offset            <= reg(), imm();
        BLEZL   rs, offset            <= reg(), imm();
        BLTZ    rs, offset            <= reg(), imm();
        BLTZAL  rs, offset            <= reg(), imm();
        BLTZALL rs, offset            <= reg(), imm();
        BLTZL   rs, offset            <= reg(), imm();
        BNE     rs, rt, offset        <= reg(), reg(), imm();
        BNEL    rs, rt, offset        <= reg(), reg(), imm();
        BREAK   code                  <= code20();
        CACHE   op, offset, base      <= cacheop(), imm(), reg();
        CFCz    cop, rt, rd           <= cop(), reg(), reg();
        COPz    cop, cofun            <= cop(), cofun();
        CTCz    cop, rt, rd           <= cop(), reg(), reg();
        DADD    rd, rs, rt            <= reg(), reg(), reg();
        DADDI   rt, rs, immediate     <= reg(), reg(), imm();
        DADDIU  rt, rs, immediate     <= reg(), reg(), imm();
        DADDU   rd, rs, rt            <= reg(), reg(), reg();
        DDIV    rs, rt                <= reg(), reg();
        DDIVU   rs, rt                <= reg(), reg();
        DIV     rs, rt                <= reg(), reg();
        DIVU    rs, rt                <= reg(), reg();
        DMFC0   rt, rd                <= reg(), reg();
        DMTC0   rt, rd                <= reg(), reg();
        DMULT   rs, rt                <= reg(), reg();
        DMULTU  rs, rt                <= reg(), reg();
        DSLL    rd, rt, sa            <= reg(), reg(), shift();
        DSLLV   rt, rs, rd            <= reg(), reg(), reg();
        DSLL32  rd, rt, sa            <= reg(), reg(), shift();
        DSRA    rd, rt, sa            <= reg(), reg(), shift();
        DSRAV   rt, rs, rd            <= reg(), reg(), reg();
        DSRA32  rd, rt, sa            <= reg(), reg(), shift();
        DSRL    rd, rt, sa            <= reg(), reg(), shift();
        DSRLV   rt, rs, rd            <= reg(), reg(), reg();
        DSRL32  rd, rt, sa            <= reg(), reg(), shift();
        DSUB    rd, rs, rt            <= reg(), reg(), reg();
        DSUBU   rd, rs, rt            <= reg(), reg(), reg();
        J       target                <= target();
        JAL     target                <= target();
        JALR    rd, rs                <= reg(), reg();
        JR      rs                    <= reg();
        LB      rt, offset, base      <= reg(), imm(), reg();
        LBU     rt, offset, base      <= reg(), imm(), reg();
        LD      rt, offset, base      <= reg(), imm(), reg();
        LDCz    cop, rt, offset, base <= cop(), reg(), imm(), reg();
        LDL     rt, offset, base      <= reg(), imm(), reg();
        LDR     rt, offset, base      <= reg(), imm(), reg();
        LH      rt, offset, base      <= reg(), imm(), reg();
        LHU     rt, offset, base      <= reg(), imm(), reg();
        LL      rt, offset, base      <= reg(), imm(), reg();
        LLD     rt, offset, base      <= reg(), imm(), reg();
        LUI     rt, immediate         <= reg(), imm();
        LW      rt, offset, base      <= reg(), imm(), reg();
        LWCz    cop, rt, offset, base <= cop(), reg(), imm(), reg();
        LWL     rt, offset, base      <= reg(), imm(), reg();
        LWR     rt, offset, base      <= reg(), imm(), reg();
        LWU     rt, offset, base      <= reg(), imm(), reg();
        MFC0    rt, rd                <= reg(), reg();
        MFCz    cop, rt, rd           <= cop(), reg(), reg();
        MFHI    rd                    <= reg();
        MFLO    rd                    <= reg();
        MTC0    rt, rd                <= reg(), reg();
        MTCz    cop, rt, rd           <= cop(), reg(), reg();
        MTHI    rs                    <= reg();
        MTLO    rs                    <= reg();
        MULT    rs, rt                <= reg(), reg();
        MULTU   rs, rt                <= reg(), reg();
        NOR     rd, rs, rt            <= reg(), reg(), reg();
        OR      rd, rs, rt            <= reg(), reg(), reg();
        ORI     rt, rs, immediate     <= reg(), reg(), imm();
        SB      rt, offset, base      <= reg(), imm(), reg();
        SC      rt, offset, base      <= reg(), imm(), reg();
        SCD     rt, offset, base      <= reg(), imm(), reg();
        SD      rt, offset, base      <= reg(), imm(), reg();
        SDCz    cop, rt, offset, base <= cop(), reg(), imm(), reg();
        SDL     rt, offset, base      <= reg(), imm(), reg();
        SDR     rt, offset, base      <= reg(), imm(), reg();
        SH      rt, offset, base      <= reg(), imm(), reg();
        SLL     rd, rt, sa            <= reg(), reg(), shift();
        SLLV    rt, rs, rd            <= reg(), reg(), reg();
        SLT     rd, rs, rt            <= reg(), reg(), reg();
        SLTI    rt, rs, immediate     <= reg(), reg(), imm();
        SLTIU   rt, rs, immediate     <= reg(), reg(), imm();
        SLTU    rd, rs, rt            <= reg(), reg(), reg();
        SRA     rd, rt, sa            <= reg(), reg(), shift();
        SRAV    rt, rs, rd            <= reg(), reg(), reg();
        SRL     rd, rt, sa            <= reg(), reg(), shift();
        SRLV    rt, rs, rd            <= reg(), reg(), reg();
        SUB     rd, rs, rt            <= reg(), reg(), reg();
        SUBU    rd, rs, rt            <= reg(), reg(), reg();
        SW      rt, offset, base      <= reg(), imm(), reg();
        SWCz    cop, rt, offset, base <= cop(), reg(), imm(), reg();
        SWL     rt, offset, base      <= reg(), imm(), reg();
        SWR     rt, offset, base      <= reg(), imm(), reg();
        SYSCALL code                  <= code20();
        TEQ     rs, rt, code          <= reg(), reg(), code10();
        TEQI    rs, immediate         <= reg(), imm();
        TGE     rs, rt, code          <= reg(), reg(), code10();
        TGEI    rs, immediate         <= reg(), imm();
        TGEIU   rs, immediate         <= reg(), imm();
        TGEU    rs, rt, code          <= reg(), reg(), code10();
        TLT     rs, rt, code          <= reg(), reg(), code10();
        TLTI    rs, immediate         <= reg(), imm();
        TLTIU   rs, immediate         <= reg(), imm();
        TLTU    rs, rt, code          <= reg(), reg(), code10();
        TNE     rs, rt, code          <= reg(), reg(), code10();
        TNEI    rs, immediate         <= reg(), imm();
        XOR     rd, rs, rt            <= reg(), reg(), reg();
        XORI    rt, rs, immediate     <= reg(), reg(), imm();
    }

    #[test]
    fn ERET() {
        let i = Instruction::ERET;
        let decoded = Instruction::decode(i.encode()).unwrap();
        assert_eq!(decoded, i);
    }

    #[test]
    fn SYNC() {
        let i = Instruction::SYNC;
        let decoded = Instruction::decode(i.encode()).unwrap();
        assert_eq!(decoded, i);
    }

    #[test]
    fn TLBP() {
        let i = Instruction::TLBP;
        let decoded = Instruction::decode(i.encode()).unwrap();
        assert_eq!(decoded, i);
    }

    #[test]
    fn TLBR() {
        let i = Instruction::TLBR;
        let decoded = Instruction::decode(i.encode()).unwrap();
        assert_eq!(decoded, i);
    }

    #[test]
    fn TLBWI() {
        let i = Instruction::TLBWI;
        let decoded = Instruction::decode(i.encode()).unwrap();
        assert_eq!(decoded, i);
    }

    #[test]
    fn TLBWR() {
        let i = Instruction::TLBWR;
        let decoded = Instruction::decode(i.encode()).unwrap();
        assert_eq!(decoded, i);
    }

}
