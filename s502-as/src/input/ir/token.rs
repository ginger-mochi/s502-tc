// TODO make token struct with position, change enum to tokentype
pub enum Token {
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
    Dfb,
    Dfw,
    Equ,
    Hcf,
    Hlt,
    Inl,
    Kil,
    Org,
    Sct,
    Txt,
    Period,
    VisFile,
    VisGlobal,
    Colon,
    Pound,
    Comma,
    Lparen,
    Rparen,
    Newline,
    Langle,
    Rangle,
    A,
    X,
    Y,
    EndFile,
    Ident(String),
    Str(String),
    Byte(u8),
    Word(u16),
}
