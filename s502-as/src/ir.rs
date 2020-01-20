use enum_map::Enum;

pub struct Program<'a> {
    pub format: OutputFormat,
    pub sections: Vec<Section>,
    pub sect: usize,
    pub position: (String, u32),
    pub code: &'a str,
}

pub enum OutputFormat {
    Raw,
    Object,
}

pub struct Section {
    name: String,
    code: [u8; 65536],
    pos: usize,
    addr: usize,
    labels: Vec<Label>,
    references: Vec<Reference>,
}

#[derive(PartialEq, Debug)]
pub enum OperandType {
    Byte(u8),
    Word(u16),
    Ref(String),
    Str(String),
}

#[derive(PartialEq, Debug)]
pub enum OperandPart {
    Both = 0,
    High = 1,
    Low = 2,
}

#[derive(PartialEq, Debug)]
pub struct OperandVal {
    pub op_type: OperandType,
    pub part: OperandPart,
}

#[derive(PartialEq, Debug)]
pub enum Visibility {
    Hidden = 0,
    File = 1,
    Global = 2,
}

#[derive(PartialEq, Debug)]
pub struct Macro {
    pub id: String,
    pub val: OperandVal,
}

#[derive(PartialEq, Debug)]
pub struct Label {
    pub pos: u16,
    pub parent: Option<String>,
    pub child: Option<String>,
}

#[derive(PartialEq, Debug)]
pub struct Reference {
    pub pos: u16,
    pub parent: Option<String>,
    pub child: Option<String>,
}

#[derive(PartialEq, Debug, Enum)]
pub enum AddressMode {
    Acc,
    Abs,
    AbsX,
    AbsY,
    Imme,
    Impl,
    Ind,
    Xind,
    IndY,
    Zpg,
    ZpgX,
    ZpgY,
}

#[derive(PartialEq, Debug, Enum)]
pub enum Mnemonic {
    Adc,
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bit,
    Bmi,
    Bne,
    Bpl,
    Brk,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
}
