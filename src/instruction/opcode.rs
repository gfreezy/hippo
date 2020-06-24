pub const AALOAD: u8 = 0x32;
pub const AASTORE: u8 = 0x53;
pub const ACONST_NULL: u8 = 0x1;
pub const ALOAD: u8 = 0x19;
pub const ALOAD_0: u8 = 0x2a;
pub const ALOAD_1: u8 = 0x2b;
pub const ALOAD_2: u8 = 0x2c;
pub const ALOAD_3: u8 = 0x2d;
pub const ANEWARRAY: u8 = 0xbd;
pub const ARETURN: u8 = 0xb0;
pub const ARRAYLENGTH: u8 = 0xbe;
pub const ASTORE: u8 = 0x3a;
pub const ASTORE_0: u8 = 0x4b;
pub const ASTORE_1: u8 = 0x4c;
pub const ASTORE_2: u8 = 0x4d;
pub const ASTORE_3: u8 = 0x4e;
pub const ATHROW: u8 = 0xbf;
pub const BALOAD: u8 = 0x33;
pub const BASTORE: u8 = 0x54;
pub const BIPUSH: u8 = 0x10;
pub const BREAKPOINT: u8 = 0xca;
pub const CALOAD: u8 = 0x34;
pub const CASTORE: u8 = 0x55;
pub const CHECKCAST: u8 = 0xc0;
pub const D2F: u8 = 0x90;
pub const D2I: u8 = 0x8e;
pub const D2L: u8 = 0x8f;
pub const DADD: u8 = 0x63;
pub const DALOAD: u8 = 0x31;
pub const DASTORE: u8 = 0x52;
pub const DCMPG: u8 = 0x98;
pub const DCMPL: u8 = 0x97;
pub const DCONST_0: u8 = 0x0e;
pub const DCONST_1: u8 = 0x0f;
pub const DDIV: u8 = 0x6f;
pub const DLOAD: u8 = 0x18;
pub const DLOAD_0: u8 = 0x26;
pub const DLOAD_1: u8 = 0x27;
pub const DLOAD_2: u8 = 0x28;
pub const DLOAD_3: u8 = 0x29;
pub const DMUL: u8 = 0x6b;
pub const DNEG: u8 = 0x77;
pub const DREM: u8 = 0x73;
pub const DRETURN: u8 = 0xaf;
pub const DSTORE: u8 = 0x39;
pub const DSTORE_0: u8 = 0x47;
pub const DSTORE_1: u8 = 0x48;
pub const DSTORE_2: u8 = 0x49;
pub const DSTORE_3: u8 = 0x4a;
pub const DSUB: u8 = 0x67;
pub const DUP: u8 = 0x59;
pub const DUP_X1: u8 = 0x5a;
pub const DUP_X2: u8 = 0x5b;
pub const DUP2: u8 = 0x5c;
pub const DUP2_X1: u8 = 0x5d;
pub const DUP2_X2: u8 = 0x5e;
pub const F2D: u8 = 0x8d;
pub const F2I: u8 = 0x8b;
pub const F2L: u8 = 0x8c;
pub const FADD: u8 = 0x62;
pub const FALOAD: u8 = 0x30;
pub const FASTORE: u8 = 0x51;
pub const FCMPG: u8 = 0x96;
pub const FCMPL: u8 = 0x95;
pub const FCONST_0: u8 = 0x0b;
pub const FCONST_1: u8 = 0x0c;
pub const FCONST_2: u8 = 0x0d;
pub const FDIV: u8 = 0x6e;
pub const FLOAD: u8 = 0x17;
pub const FLOAD_0: u8 = 0x22;
pub const FLOAD_1: u8 = 0x23;
pub const FLOAD_2: u8 = 0x24;
pub const FLOAD_3: u8 = 0x25;
pub const FMUL: u8 = 0x6a;
pub const FNEG: u8 = 0x76;
pub const FREM: u8 = 0x72;
pub const FRETURN: u8 = 0xae;
pub const FSTORE: u8 = 0x38;
pub const FSTORE_0: u8 = 0x43;
pub const FSTORE_1: u8 = 0x44;
pub const FSTORE_2: u8 = 0x45;
pub const FSTORE_3: u8 = 0x46;
pub const FSUB: u8 = 0x66;
pub const GETFIELD: u8 = 0xb4;
pub const GETSTATIC: u8 = 0xb2;
pub const GOTO: u8 = 0xa7;
pub const GOTO_W: u8 = 0xc8;
pub const I2B: u8 = 0x91;
pub const I2C: u8 = 0x92;
pub const I2D: u8 = 0x87;
pub const I2F: u8 = 0x86;
pub const I2L: u8 = 0x85;
pub const I2S: u8 = 0x93;
pub const IADD: u8 = 0x60;
pub const IALOAD: u8 = 0x2e;
pub const IAND: u8 = 0x7e;
pub const IASTORE: u8 = 0x4f;
pub const ICONST_M1: u8 = 0x2;
pub const ICONST_0: u8 = 0x3;
pub const ICONST_1: u8 = 0x4;
pub const ICONST_2: u8 = 0x5;
pub const ICONST_3: u8 = 0x6;
pub const ICONST_4: u8 = 0x7;
pub const ICONST_5: u8 = 0x8;
pub const IDIV: u8 = 0x6c;
pub const IF_ACMPEQ: u8 = 0xa5;
pub const IF_ACMPNE: u8 = 0xa6;
pub const IF_ICMPEQ: u8 = 0x9f;
pub const IF_ICMPGE: u8 = 0xa2;
pub const IF_ICMPGT: u8 = 0xa3;
pub const IF_ICMPLE: u8 = 0xa4;
pub const IF_ICMPLT: u8 = 0xa1;
pub const IF_ICMPNE: u8 = 0xa0;
pub const IFEQ: u8 = 0x99;
pub const IFGE: u8 = 0x9c;
pub const IFGT: u8 = 0x9d;
pub const IFLE: u8 = 0x9e;
pub const IFLT: u8 = 0x9b;
pub const IFNE: u8 = 0x9a;
pub const IFNONNULL: u8 = 0xc7;
pub const IFNULL: u8 = 0xc6;
pub const IINC: u8 = 0x84;
pub const ILOAD: u8 = 0x15;
pub const ILOAD_0: u8 = 0x1a;
pub const ILOAD_1: u8 = 0x1b;
pub const ILOAD_2: u8 = 0x1c;
pub const ILOAD_3: u8 = 0x1d;
pub const IMPDEP1: u8 = 0xfe;
pub const IMPDEP2: u8 = 0xff;
pub const IMUL: u8 = 0x68;
pub const INEG: u8 = 0x74;
pub const INSTANCEOF: u8 = 0xc1;
pub const INVOKEDYNAMIC: u8 = 0xba;
pub const INVOKEINTERFACE: u8 = 0xb9;
pub const INVOKESPECIAL: u8 = 0xb7;
pub const INVOKESTATIC: u8 = 0xb8;
pub const INVOKEVIRTUAL: u8 = 0xb6;
pub const IOR: u8 = 0x80;
pub const IREM: u8 = 0x70;
pub const IRETURN: u8 = 0xac;
pub const ISHL: u8 = 0x78;
pub const ISHR: u8 = 0x7a;
pub const ISTORE: u8 = 0x36;
pub const ISTORE_0: u8 = 0x3b;
pub const ISTORE_1: u8 = 0x3c;
pub const ISTORE_2: u8 = 0x3d;
pub const ISTORE_3: u8 = 0x3e;
pub const ISUB: u8 = 0x64;
pub const IUSHR: u8 = 0x7c;
pub const IXOR: u8 = 0x82;
pub const JSR: u8 = 0xa8;
pub const JSR_W: u8 = 0xc9;
pub const L2D: u8 = 0x8a;
pub const L2F: u8 = 0x89;
pub const L2I: u8 = 0x88;
pub const LADD: u8 = 0x61;
pub const LALOAD: u8 = 0x2f;
pub const LAND: u8 = 0x7f;
pub const LASTORE: u8 = 0x50;
pub const LCMP: u8 = 0x94;
pub const LCONST_0: u8 = 0x9;
pub const LCONST_1: u8 = 0x0a;
pub const LDC: u8 = 0x12;
pub const LDC_W: u8 = 0x13;
pub const LDC2_W: u8 = 0x14;
pub const LDIV: u8 = 0x6d;
pub const LLOAD: u8 = 0x16;
pub const LLOAD_0: u8 = 0x1e;
pub const LLOAD_1: u8 = 0x1f;
pub const LLOAD_2: u8 = 0x20;
pub const LLOAD_3: u8 = 0x21;
pub const LMUL: u8 = 0x69;
pub const LNEG: u8 = 0x75;
pub const LOOKUPSWITCH: u8 = 0xab;
pub const LOR: u8 = 0x81;
pub const LREM: u8 = 0x71;
pub const LRETURN: u8 = 0xad;
pub const LSHL: u8 = 0x79;
pub const LSHR: u8 = 0x7b;
pub const LSTORE: u8 = 0x37;
pub const LSTORE_0: u8 = 0x3f;
pub const LSTORE_1: u8 = 0x40;
pub const LSTORE_2: u8 = 0x41;
pub const LSTORE_3: u8 = 0x42;
pub const LSUB: u8 = 0x65;
pub const LUSHR: u8 = 0x7d;
pub const LXOR: u8 = 0x83;
pub const MONITORENTER: u8 = 0xc2;
pub const MONITOREXIT: u8 = 0xc3;
pub const MULTIANEWARRAY: u8 = 0xc5;
pub const NEW: u8 = 0xbb;
pub const NEWARRAY: u8 = 0xbc;
pub const NOP: u8 = 0x0;
pub const POP: u8 = 0x57;
pub const POP2: u8 = 0x58;
pub const PUTFIELD: u8 = 0xb5;
pub const PUTSTATIC: u8 = 0xb3;
pub const RET: u8 = 0xa9;
pub const RETURN: u8 = 0xb1;
pub const SALOAD: u8 = 0x35;
pub const SASTORE: u8 = 0x56;
pub const SIPUSH: u8 = 0x11;
pub const SWAP: u8 = 0x5f;
pub const TABLESWITCH: u8 = 0xaa;
pub const WIDE: u8 = 0xc4;

pub fn show_opcode(code: u8) -> &'static str {
    match code {
        AALOAD => "AALOAD",
        AASTORE => "AASTORE",
        ACONST_NULL => "ACONST_NULL",
        ALOAD => "ALOAD",
        ALOAD_0 => "ALOAD_0",
        ALOAD_1 => "ALOAD_1",
        ALOAD_2 => "ALOAD_2",
        ALOAD_3 => "ALOAD_3",
        ANEWARRAY => "ANEWARRAY",
        ARETURN => "ARETURN",
        ARRAYLENGTH => "ARRAYLENGTH",
        ASTORE => "ASTORE",
        ASTORE_0 => "ASTORE_0",
        ASTORE_1 => "ASTORE_1",
        ASTORE_2 => "ASTORE_2",
        ASTORE_3 => "ASTORE_3",
        ATHROW => "ATHROW",
        BALOAD => "BALOAD",
        BASTORE => "BASTORE",
        BIPUSH => "BIPUSH",
        BREAKPOINT => "BREAKPOINT",
        CALOAD => "CALOAD",
        CASTORE => "CASTORE",
        CHECKCAST => "CHECKCAST",
        D2F => "D2F",
        D2I => "D2I",
        D2L => "D2L",
        DADD => "DADD",
        DALOAD => "DALOAD",
        DASTORE => "DASTORE",
        DCMPG => "DCMPG",
        DCMPL => "DCMPL",
        DCONST_0 => "DCONST_0",
        DCONST_1 => "DCONST_1",
        DDIV => "DDIV",
        DLOAD => "DLOAD",
        DLOAD_0 => "DLOAD_0",
        DLOAD_1 => "DLOAD_1",
        DLOAD_2 => "DLOAD_2",
        DLOAD_3 => "DLOAD_3",
        DMUL => "DMUL",
        DNEG => "DNEG",
        DREM => "DREM",
        DRETURN => "DRETURN",
        DSTORE => "DSTORE",
        DSTORE_0 => "DSTORE_0",
        DSTORE_1 => "DSTORE_1",
        DSTORE_2 => "DSTORE_2",
        DSTORE_3 => "DSTORE_3",
        DSUB => "DSUB",
        DUP => "DUP",
        DUP_X1 => "DUP_X1",
        DUP_X2 => "DUP_X2",
        DUP2 => "DUP2",
        DUP2_X1 => "DUP2_X1",
        DUP2_X2 => "DUP2_X2",
        F2D => "F2D",
        F2I => "F2I",
        F2L => "F2L",
        FADD => "FADD",
        FALOAD => "FALOAD",
        FASTORE => "FASTORE",
        FCMPG => "FCMPG",
        FCMPL => "FCMPL",
        FCONST_0 => "FCONST_0",
        FCONST_1 => "FCONST_1",
        FCONST_2 => "FCONST_2",
        FDIV => "FDIV",
        FLOAD => "FLOAD",
        FLOAD_0 => "FLOAD_0",
        FLOAD_1 => "FLOAD_1",
        FLOAD_2 => "FLOAD_2",
        FLOAD_3 => "FLOAD_3",
        FMUL => "FMUL",
        FNEG => "FNEG",
        FREM => "FREM",
        FRETURN => "FRETURN",
        FSTORE => "FSTORE",
        FSTORE_0 => "FSTORE_0",
        FSTORE_1 => "FSTORE_1",
        FSTORE_2 => "FSTORE_2",
        FSTORE_3 => "FSTORE_3",
        FSUB => "FSUB",
        GETFIELD => "GETFIELD",
        GETSTATIC => "GETSTATIC",
        GOTO => "GOTO",
        GOTO_W => "GOTO_W",
        I2B => "I2B",
        I2C => "I2C",
        I2D => "I2D",
        I2F => "I2F",
        I2L => "I2L",
        I2S => "I2S",
        IADD => "IADD",
        IALOAD => "IALOAD",
        IAND => "IAND",
        IASTORE => "IASTORE",
        ICONST_M1 => "ICONST_M1",
        ICONST_0 => "ICONST_0",
        ICONST_1 => "ICONST_1",
        ICONST_2 => "ICONST_2",
        ICONST_3 => "ICONST_3",
        ICONST_4 => "ICONST_4",
        ICONST_5 => "ICONST_5",
        IDIV => "IDIV",
        IF_ACMPEQ => "IF_ACMPEQ",
        IF_ACMPNE => "IF_ACMPNE",
        IF_ICMPEQ => "IF_ICMPEQ",
        IF_ICMPGE => "IF_ICMPGE",
        IF_ICMPGT => "IF_ICMPGT",
        IF_ICMPLE => "IF_ICMPLE",
        IF_ICMPLT => "IF_ICMPLT",
        IF_ICMPNE => "IF_ICMPNE",
        IFEQ => "IFEQ",
        IFGE => "IFGE",
        IFGT => "IFGT",
        IFLE => "IFLE",
        IFLT => "IFLT",
        IFNE => "IFNE",
        IFNONNULL => "IFNONNULL",
        IFNULL => "IFNULL",
        IINC => "IINC",
        ILOAD => "ILOAD",
        ILOAD_0 => "ILOAD_0",
        ILOAD_1 => "ILOAD_1",
        ILOAD_2 => "ILOAD_2",
        ILOAD_3 => "ILOAD_3",
        IMPDEP1 => "IMPDEP1",
        IMPDEP2 => "IMPDEP2",
        IMUL => "IMUL",
        INEG => "INEG",
        INSTANCEOF => "INSTANCEOF",
        INVOKEDYNAMIC => "INVOKEDYNAMIC",
        INVOKEINTERFACE => "INVOKEINTERFACE",
        INVOKESPECIAL => "INVOKESPECIAL",
        INVOKESTATIC => "INVOKESTATIC",
        INVOKEVIRTUAL => "INVOKEVIRTUAL",
        IOR => "IOR",
        IREM => "IREM",
        IRETURN => "IRETURN",
        ISHL => "ISHL",
        ISHR => "ISHR",
        ISTORE => "ISTORE",
        ISTORE_0 => "ISTORE_0",
        ISTORE_1 => "ISTORE_1",
        ISTORE_2 => "ISTORE_2",
        ISTORE_3 => "ISTORE_3",
        ISUB => "ISUB",
        IUSHR => "IUSHR",
        IXOR => "IXOR",
        JSR => "JSR",
        JSR_W => "JSR_W",
        L2D => "L2D",
        L2F => "L2F",
        L2I => "L2I",
        LADD => "LADD",
        LALOAD => "LALOAD",
        LAND => "LAND",
        LASTORE => "LASTORE",
        LCMP => "LCMP",
        LCONST_0 => "LCONST_0",
        LCONST_1 => "LCONST_1",
        LDC => "LDC",
        LDC_W => "LDC_W",
        LDC2_W => "LDC2_W",
        LDIV => "LDIV",
        LLOAD => "LLOAD",
        LLOAD_0 => "LLOAD_0",
        LLOAD_1 => "LLOAD_1",
        LLOAD_2 => "LLOAD_2",
        LLOAD_3 => "LLOAD_3",
        LMUL => "LMUL",
        LNEG => "LNEG",
        LOOKUPSWITCH => "LOOKUPSWITCH",
        LOR => "LOR",
        LREM => "LREM",
        LRETURN => "LRETURN",
        LSHL => "LSHL",
        LSHR => "LSHR",
        LSTORE => "LSTORE",
        LSTORE_0 => "LSTORE_0",
        LSTORE_1 => "LSTORE_1",
        LSTORE_2 => "LSTORE_2",
        LSTORE_3 => "LSTORE_3",
        LSUB => "LSUB",
        LUSHR => "LUSHR",
        LXOR => "LXOR",
        MONITORENTER => "MONITORENTER",
        MONITOREXIT => "MONITOREXIT",
        MULTIANEWARRAY => "MULTIANEWARRAY",
        NEW => "NEW",
        NEWARRAY => "NEWARRAY",
        NOP => "NOP",
        POP => "POP",
        POP2 => "POP2",
        PUTFIELD => "PUTFIELD",
        PUTSTATIC => "PUTSTATIC",
        RET => "RET",
        RETURN => "RETURN",
        SALOAD => "SALOAD",
        SASTORE => "SASTORE",
        SIPUSH => "SIPUSH",
        SWAP => "SWAP",
        TABLESWITCH => "TABLESWITCH",
        WIDE => "WIDE",
        _ => unreachable!(),
    }
}
