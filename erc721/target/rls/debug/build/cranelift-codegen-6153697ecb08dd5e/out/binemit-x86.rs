/// Emit binary machine code for `inst` for the x86 ISA.
#[allow(unused_variables, unreachable_code)]
pub fn emit_inst<CS: CodeSink + ?Sized>(
    func: &Function,
    inst: Inst,
    divert: &mut RegDiversions,
    sink: &mut CS,
    isa: &dyn TargetIsa,
) {
    let encoding = func.encodings[inst];
    let bits = encoding.bits();
    let inst_data = &func.dfg[inst];
    match encoding.recipe() {
        // Recipe get_pinned_reg
        0 => {
            if let InstructionData::NullAry {
                opcode,
                ..
            } = *inst_data {
                return;
            }
        }
        // Recipe RexOp1set_pinned_reg
        1 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let r15 = RU::r15.into();
                put_rexop1(bits, rex2(r15, in_reg0), sink);
                modrm_rr(r15, in_reg0, sink);
                return;
            }
        }
        // Recipe DynRexOp1umr
        2 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_dynrexop1(bits, rex2(out_reg0, in_reg0), sink);
                modrm_rr(out_reg0, in_reg0, sink);
                return;
            }
        }
        // Recipe Op1umr
        3 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op1(bits, rex2(out_reg0, in_reg0), sink);
                modrm_rr(out_reg0, in_reg0, sink);
                return;
            }
        }
        // Recipe RexOp1umr
        4 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex2(out_reg0, in_reg0), sink);
                modrm_rr(out_reg0, in_reg0, sink);
                return;
            }
        }
        // Recipe Op1rmov
        5 => {
            if let InstructionData::RegMove {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                put_op1(bits, rex2(dst, src), sink);
                modrm_rr(dst, src, sink);
                return;
            }
        }
        // Recipe RexOp1rmov
        6 => {
            if let InstructionData::RegMove {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                put_rexop1(bits, rex2(dst, src), sink);
                modrm_rr(dst, src, sink);
                return;
            }
        }
        // Recipe Op1pu_id
        7 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // The destination register is encoded in the low bits of the opcode.
                // No ModR/M.
                put_op1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                return;
            }
        }
        // Recipe RexOp1pu_id
        8 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // The destination register is encoded in the low bits of the opcode.
                // No ModR/M.
                put_rexop1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                return;
            }
        }
        // Recipe RexOp1u_id
        9 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex1(out_reg0), sink);
                modrm_r_bits(out_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                return;
            }
        }
        // Recipe RexOp1pu_iq
        10 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                let imm: i64 = imm.into();
                sink.put8(imm as u64);
                return;
            }
        }
        // Recipe Op1pu_id_bool
        11 => {
            if let InstructionData::UnaryBool {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // The destination register is encoded in the low bits of the opcode.
                // No ModR/M.
                put_op1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                let imm: u32 = if imm { 1 } else { 0 };
                sink.put4(imm);
                return;
            }
        }
        // Recipe RexOp1pu_id_bool
        12 => {
            if let InstructionData::UnaryBool {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // The destination register is encoded in the low bits of the opcode.
                // No ModR/M.
                put_rexop1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                let imm: u32 = if imm { 1 } else { 0 };
                sink.put4(imm);
                return;
            }
        }
        // Recipe Op1u_id_z
        13 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op1(bits, rex2(out_reg0, out_reg0), sink);
                modrm_rr(out_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexOp1u_id_z
        14 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex2(out_reg0, out_reg0), sink);
                modrm_rr(out_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe null
        15 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                return;
            }
        }
        // Recipe Op2urm_noflags_abcd
        16 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexOp2urm_noflags
        17 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe Op2urm_noflags
        18 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexOp1urm_noflags
        19 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexOp1copysp
        20 => {
            if let InstructionData::CopySpecial {
                opcode,
                src,
                dst,
                ..
            } = *inst_data {
                put_rexop1(bits, rex2(dst, src), sink);
                modrm_rr(dst, src, sink);
                return;
            }
        }
        // Recipe Op1copysp
        21 => {
            if let InstructionData::CopySpecial {
                opcode,
                src,
                dst,
                ..
            } = *inst_data {
                put_op1(bits, rex2(dst, src), sink);
                modrm_rr(dst, src, sink);
                return;
            }
        }
        // Recipe Op1umr_reg_to_ssa
        22 => {
            if let InstructionData::CopyToSsa {
                opcode,
                src,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op1(bits, rex2(out_reg0, src), sink);
                modrm_rr(out_reg0, src, sink);
                return;
            }
        }
        // Recipe RexOp1umr_reg_to_ssa
        23 => {
            if let InstructionData::CopyToSsa {
                opcode,
                src,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex2(out_reg0, src), sink);
                modrm_rr(out_reg0, src, sink);
                return;
            }
        }
        // Recipe Mp2furm_reg_to_ssa
        24 => {
            if let InstructionData::CopyToSsa {
                opcode,
                src,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp2(bits, rex2(src, out_reg0), sink);
                modrm_rr(src, out_reg0, sink);
                return;
            }
        }
        // Recipe RexMp2furm_reg_to_ssa
        25 => {
            if let InstructionData::CopyToSsa {
                opcode,
                src,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexmp2(bits, rex2(src, out_reg0), sink);
                modrm_rr(src, out_reg0, sink);
                return;
            }
        }
        // Recipe Op1ldWithIndex
        26 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe RexOp1ldWithIndex
        27 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe Op2ldWithIndex
        28 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe RexOp2ldWithIndex
        29 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe Op1ldWithIndexDisp8
        30 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp8(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexOp1ldWithIndexDisp8
        31 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp8(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op2ldWithIndexDisp8
        32 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp8(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexOp2ldWithIndexDisp8
        33 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp8(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op1ldWithIndexDisp32
        34 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexOp1ldWithIndexDisp32
        35 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op2ldWithIndexDisp32
        36 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexOp2ldWithIndexDisp32
        37 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op1stWithIndex
        38 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe RexOp1stWithIndex
        39 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe Mp1stWithIndex
        40 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe RexMp1stWithIndex
        41 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe Op1stWithIndexDisp8
        42 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp8(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexOp1stWithIndexDisp8
        43 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp8(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Mp1stWithIndexDisp8
        44 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp8(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexMp1stWithIndexDisp8
        45 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp8(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op1stWithIndexDisp32
        46 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexOp1stWithIndexDisp32
        47 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Mp1stWithIndexDisp32
        48 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexMp1stWithIndexDisp32
        49 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op1stWithIndex_abcd
        50 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe RexOp1stWithIndex_abcd
        51 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe Op1stWithIndexDisp8_abcd
        52 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp8(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexOp1stWithIndexDisp8_abcd
        53 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp8(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op1stWithIndexDisp32_abcd
        54 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexOp1stWithIndexDisp32_abcd
        55 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op1st
        56 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else if needs_offset(in_reg1) {
                    modrm_disp8(in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe RexOp1st
        57 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else if needs_offset(in_reg1) {
                    modrm_disp8(in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe Mp1st
        58 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else if needs_offset(in_reg1) {
                    modrm_disp8(in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe RexMp1st
        59 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else if needs_offset(in_reg1) {
                    modrm_disp8(in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe Op1stDisp8
        60 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp8(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexOp1stDisp8
        61 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp8(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Mp1stDisp8
        62 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp8(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexMp1stDisp8
        63 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp8(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op1stDisp32
        64 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp32(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp32(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexOp1stDisp32
        65 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp32(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp32(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Mp1stDisp32
        66 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp32(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp32(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexMp1stDisp32
        67 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp32(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp32(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op1st_abcd
        68 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else if needs_offset(in_reg1) {
                    modrm_disp8(in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe Op1stDisp8_abcd
        69 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp8(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op1stDisp32_abcd
        70 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp32(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp32(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op1spillSib32
        71 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_stk0 = StackRef::masked(
                    divert.stack(results[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let base = stk_base(out_stk0.base);
                put_op1(bits, rex2(base, in_reg0), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(out_stk0.offset as u32);
                return;
            }
        }
        // Recipe RexOp1spillSib32
        72 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_stk0 = StackRef::masked(
                    divert.stack(results[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let base = stk_base(out_stk0.base);
                put_rexop1(bits, rex2(base, in_reg0), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(out_stk0.offset as u32);
                return;
            }
        }
        // Recipe Op1regspill32
        73 => {
            if let InstructionData::RegSpill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let dst = StackRef::sp(dst, &func.stack_slots);
                let base = stk_base(dst.base);
                put_op1(bits, rex2(base, src), sink);
                modrm_sib_disp32(src, sink);
                sib_noindex(base, sink);
                sink.put4(dst.offset as u32);
                return;
            }
        }
        // Recipe RexOp1regspill32
        74 => {
            if let InstructionData::RegSpill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let dst = StackRef::sp(dst, &func.stack_slots);
                let base = stk_base(dst.base);
                put_rexop1(bits, rex2(base, src), sink);
                modrm_sib_disp32(src, sink);
                sib_noindex(base, sink);
                sink.put4(dst.offset as u32);
                return;
            }
        }
        // Recipe Op1ld
        75 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else if needs_offset(in_reg0) {
                    modrm_disp8(in_reg0, out_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg0, out_reg0, sink);
                }
                return;
            }
        }
        // Recipe RexOp1ld
        76 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else if needs_offset(in_reg0) {
                    modrm_disp8(in_reg0, out_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg0, out_reg0, sink);
                }
                return;
            }
        }
        // Recipe Op2ld
        77 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else if needs_offset(in_reg0) {
                    modrm_disp8(in_reg0, out_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg0, out_reg0, sink);
                }
                return;
            }
        }
        // Recipe RexOp2ld
        78 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else if needs_offset(in_reg0) {
                    modrm_disp8(in_reg0, out_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg0, out_reg0, sink);
                }
                return;
            }
        }
        // Recipe Op1ldDisp8
        79 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp8(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexOp1ldDisp8
        80 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp8(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op2ldDisp8
        81 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp8(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexOp2ldDisp8
        82 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp8(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op1ldDisp32
        83 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op1(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp32(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp32(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexOp1ldDisp32
        84 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop1(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp32(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp32(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op2ldDisp32
        85 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp32(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp32(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexOp2ldDisp32
        86 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexop2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp32(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp32(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op1fillSib32
        87 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_stk0 = StackRef::masked(
                    divert.stack(args[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                let base = stk_base(in_stk0.base);
                put_op1(bits, rex2(base, out_reg0), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(in_stk0.offset as u32);
                return;
            }
        }
        // Recipe RexOp1fillSib32
        88 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_stk0 = StackRef::masked(
                    divert.stack(args[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                let base = stk_base(in_stk0.base);
                put_rexop1(bits, rex2(base, out_reg0), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(in_stk0.offset as u32);
                return;
            }
        }
        // Recipe Op1regfill32
        89 => {
            if let InstructionData::RegFill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                let src = StackRef::sp(src, &func.stack_slots);
                let base = stk_base(src.base);
                put_op1(bits, rex2(base, dst), sink);
                modrm_sib_disp32(dst, sink);
                sib_noindex(base, sink);
                sink.put4(src.offset as u32);
                return;
            }
        }
        // Recipe RexOp1regfill32
        90 => {
            if let InstructionData::RegFill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                let src = StackRef::sp(src, &func.stack_slots);
                let base = stk_base(src.base);
                put_rexop1(bits, rex2(base, dst), sink);
                modrm_sib_disp32(dst, sink);
                sib_noindex(base, sink);
                sink.put4(src.offset as u32);
                return;
            }
        }
        // Recipe fillnull
        91 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_stk0 = StackRef::masked(
                    divert.stack(args[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                return;
            }
        }
        // Recipe ffillnull
        92 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_stk0 = StackRef::masked(
                    divert.stack(args[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                return;
            }
        }
        // Recipe Op1pushq
        93 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                put_op1(bits | (in_reg0 & 7), rex1(in_reg0), sink);
                return;
            }
        }
        // Recipe RexOp1pushq
        94 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                put_rexop1(bits | (in_reg0 & 7), rex1(in_reg0), sink);
                return;
            }
        }
        // Recipe Op1popq
        95 => {
            if let InstructionData::NullAry {
                opcode,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                return;
            }
        }
        // Recipe RexOp1popq
        96 => {
            if let InstructionData::NullAry {
                opcode,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                return;
            }
        }
        // Recipe stacknull
        97 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_stk0 = StackRef::masked(
                    divert.stack(args[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                let results = [func.dfg.first_result(inst)];
                let out_stk0 = StackRef::masked(
                    divert.stack(results[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                return;
            }
        }
        // Recipe Op1adjustsp
        98 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_op1(bits, rex2(RU::rsp.into(), in_reg0), sink);
                modrm_rr(RU::rsp.into(), in_reg0, sink);
                return;
            }
        }
        // Recipe RexOp1adjustsp
        99 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_rexop1(bits, rex2(RU::rsp.into(), in_reg0), sink);
                modrm_rr(RU::rsp.into(), in_reg0, sink);
                return;
            }
        }
        // Recipe Op1adjustsp_ib
        100 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                put_op1(bits, rex1(RU::rsp.into()), sink);
                modrm_r_bits(RU::rsp.into(), bits, sink);
                let imm: i64 = imm.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe Op1adjustsp_id
        101 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                put_op1(bits, rex1(RU::rsp.into()), sink);
                modrm_r_bits(RU::rsp.into(), bits, sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                return;
            }
        }
        // Recipe RexOp1adjustsp_ib
        102 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                put_rexop1(bits, rex1(RU::rsp.into()), sink);
                modrm_r_bits(RU::rsp.into(), bits, sink);
                let imm: i64 = imm.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe RexOp1adjustsp_id
        103 => {
            if let InstructionData::UnaryImm {
                opcode,
                imm,
                ..
            } = *inst_data {
                put_rexop1(bits, rex1(RU::rsp.into()), sink);
                modrm_r_bits(RU::rsp.into(), bits, sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                return;
            }
        }
        // Recipe Mp2frurm
        104 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexMp2frurm
        105 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexmp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe Mp2rfumr
        106 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp2(bits, rex2(out_reg0, in_reg0), sink);
                modrm_rr(out_reg0, in_reg0, sink);
                return;
            }
        }
        // Recipe RexMp2rfumr
        107 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexmp2(bits, rex2(out_reg0, in_reg0), sink);
                modrm_rr(out_reg0, in_reg0, sink);
                return;
            }
        }
        // Recipe Op2furm
        108 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexOp2furm
        109 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe Op2frmov
        110 => {
            if let InstructionData::RegMove {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                put_op2(bits, rex2(src, dst), sink);
                modrm_rr(src, dst, sink);
                return;
            }
        }
        // Recipe RexOp2frmov
        111 => {
            if let InstructionData::RegMove {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                put_rexop2(bits, rex2(src, dst), sink);
                modrm_rr(src, dst, sink);
                return;
            }
        }
        // Recipe Mp2fld
        112 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else if needs_offset(in_reg0) {
                    modrm_disp8(in_reg0, out_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg0, out_reg0, sink);
                }
                return;
            }
        }
        // Recipe RexMp2fld
        113 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else if needs_offset(in_reg0) {
                    modrm_disp8(in_reg0, out_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg0, out_reg0, sink);
                }
                return;
            }
        }
        // Recipe Mp2fldDisp8
        114 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp8(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexMp2fldDisp8
        115 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp8(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Mp2fldDisp32
        116 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp32(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp32(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexMp2fldDisp32
        117 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp32(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp32(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Mp2fldWithIndex
        118 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe RexMp2fldWithIndex
        119 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(out_reg0, sink);
                    sib(0, in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe Mp2fldWithIndexDisp8
        120 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp8(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexMp2fldWithIndexDisp8
        121 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp8(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Mp2fldWithIndexDisp32
        122 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexMp2fldWithIndexDisp32
        123 => {
            if let InstructionData::LoadComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex3(in_reg0, out_reg0, in_reg1), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib(0, in_reg1, in_reg0, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Mp2fst
        124 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else if needs_offset(in_reg1) {
                    modrm_disp8(in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe RexMp2fst
        125 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else if needs_offset(in_reg1) {
                    modrm_disp8(in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe Mp2fstDisp8
        126 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp8(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexMp2fstDisp8
        127 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp8(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Mp2fstDisp32
        128 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp32(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp32(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexMp2fstDisp32
        129 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp32(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp32(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Mp2fstWithIndex
        130 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe RexMp2fstWithIndex
        131 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                // The else branch always inserts an SIB byte.
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(in_reg0, sink);
                    sib(0, in_reg2, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe Mp2fstWithIndexDisp8
        132 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp8(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe RexMp2fstWithIndexDisp8
        133 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp8(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Mp2fstWithIndexDisp32
        134 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_mp2(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe RexMp2fstWithIndexDisp32
        135 => {
            if let InstructionData::StoreComplex {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_rexmp2(bits, rex3(in_reg1, in_reg0, in_reg2), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib(0, in_reg2, in_reg1, sink);
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Mp2ffillSib32
        136 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_stk0 = StackRef::masked(
                    divert.stack(args[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                let base = stk_base(in_stk0.base);
                put_mp2(bits, rex2(base, out_reg0), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(in_stk0.offset as u32);
                return;
            }
        }
        // Recipe RexMp2ffillSib32
        137 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_stk0 = StackRef::masked(
                    divert.stack(args[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                let base = stk_base(in_stk0.base);
                put_rexmp2(bits, rex2(base, out_reg0), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(in_stk0.offset as u32);
                return;
            }
        }
        // Recipe Mp2fregfill32
        138 => {
            if let InstructionData::RegFill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                let src = StackRef::sp(src, &func.stack_slots);
                let base = stk_base(src.base);
                put_mp2(bits, rex2(base, dst), sink);
                modrm_sib_disp32(dst, sink);
                sib_noindex(base, sink);
                sink.put4(src.offset as u32);
                return;
            }
        }
        // Recipe RexMp2fregfill32
        139 => {
            if let InstructionData::RegFill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                let src = StackRef::sp(src, &func.stack_slots);
                let base = stk_base(src.base);
                put_rexmp2(bits, rex2(base, dst), sink);
                modrm_sib_disp32(dst, sink);
                sib_noindex(base, sink);
                sink.put4(src.offset as u32);
                return;
            }
        }
        // Recipe Mp2fspillSib32
        140 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_stk0 = StackRef::masked(
                    divert.stack(results[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let base = stk_base(out_stk0.base);
                put_mp2(bits, rex2(base, in_reg0), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(out_stk0.offset as u32);
                return;
            }
        }
        // Recipe RexMp2fspillSib32
        141 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_stk0 = StackRef::masked(
                    divert.stack(results[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let base = stk_base(out_stk0.base);
                put_rexmp2(bits, rex2(base, in_reg0), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(out_stk0.offset as u32);
                return;
            }
        }
        // Recipe Mp2fregspill32
        142 => {
            if let InstructionData::RegSpill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let dst = StackRef::sp(dst, &func.stack_slots);
                let base = stk_base(dst.base);
                put_mp2(bits, rex2(base, src), sink);
                modrm_sib_disp32(src, sink);
                sib_noindex(base, sink);
                sink.put4(dst.offset as u32);
                return;
            }
        }
        // Recipe RexMp2fregspill32
        143 => {
            if let InstructionData::RegSpill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let dst = StackRef::sp(dst, &func.stack_slots);
                let base = stk_base(dst.base);
                put_rexmp2(bits, rex2(base, src), sink);
                modrm_sib_disp32(src, sink);
                sib_noindex(base, sink);
                sink.put4(dst.offset as u32);
                return;
            }
        }
        // Recipe Op2f32imm_z
        144 => {
            if let InstructionData::UnaryIeee32 {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op2(bits, rex2(out_reg0, out_reg0), sink);
                modrm_rr(out_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe Mp2f64imm_z
        145 => {
            if let InstructionData::UnaryIeee64 {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp2(bits, rex2(out_reg0, out_reg0), sink);
                modrm_rr(out_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexOp2f32imm_z
        146 => {
            if let InstructionData::UnaryIeee32 {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop2(bits, rex2(out_reg0, out_reg0), sink);
                modrm_rr(out_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexMp2f64imm_z
        147 => {
            if let InstructionData::UnaryIeee64 {
                opcode,
                imm,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexmp2(bits, rex2(out_reg0, out_reg0), sink);
                modrm_rr(out_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe DynRexMp2frurm
        148 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_dynrexmp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe Mp2furm
        149 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexMp2furm
        150 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexmp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe Mp2rfurm
        151 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexMp2rfurm
        152 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexmp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe Mp3furmi_rnd
        153 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp3(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                sink.put1(match opcode {
                    Opcode::Nearest => 0b00,
                    Opcode::Floor => 0b01,
                    Opcode::Ceil => 0b10,
                    Opcode::Trunc => 0b11,
                    x => panic!("{} unexpected for furmi_rnd", opcode),
                });
                return;
            }
        }
        // Recipe RexMp3furmi_rnd
        154 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexmp3(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                sink.put1(match opcode {
                    Opcode::Nearest => 0b00,
                    Opcode::Floor => 0b01,
                    Opcode::Ceil => 0b10,
                    Opcode::Trunc => 0b11,
                    x => panic!("{} unexpected for furmi_rnd", opcode),
                });
                return;
            }
        }
        // Recipe Mp2fa
        155 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_mp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe RexMp2fa
        156 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexmp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe Op2fcscc
        157 => {
            if let InstructionData::FloatCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_op2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                // `setCC` instruction, no REX.
                use crate::ir::condcodes::FloatCC::*;
                let setcc = match cond {
                    Ordered                    => 0x9b, // EQ|LT|GT => setnp (P=0)
                    Unordered                  => 0x9a, // UN       => setp  (P=1)
                    OrderedNotEqual            => 0x95, // LT|GT    => setne (Z=0),
                    UnorderedOrEqual           => 0x94, // UN|EQ    => sete  (Z=1)
                    GreaterThan                => 0x97, // GT       => seta  (C=0&Z=0)
                    GreaterThanOrEqual         => 0x93, // GT|EQ    => setae (C=0)
                    UnorderedOrLessThan        => 0x92, // UN|LT    => setb  (C=1)
                    UnorderedOrLessThanOrEqual => 0x96, // UN|LT|EQ => setbe (Z=1|C=1)
                    Equal |                       // EQ
                    NotEqual |                    // UN|LT|GT
                    LessThan |                    // LT
                    LessThanOrEqual |             // LT|EQ
                    UnorderedOrGreaterThan |      // UN|GT
                    UnorderedOrGreaterThanOrEqual // UN|GT|EQ
                    => panic!("{} not supported by fcscc", cond),
                };
                sink.put1(0x0f);
                sink.put1(setcc);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe RexOp2fcscc
        158 => {
            if let InstructionData::FloatCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_rexop2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                // `setCC` instruction, no REX.
                use crate::ir::condcodes::FloatCC::*;
                let setcc = match cond {
                    Ordered                    => 0x9b, // EQ|LT|GT => setnp (P=0)
                    Unordered                  => 0x9a, // UN       => setp  (P=1)
                    OrderedNotEqual            => 0x95, // LT|GT    => setne (Z=0),
                    UnorderedOrEqual           => 0x94, // UN|EQ    => sete  (Z=1)
                    GreaterThan                => 0x97, // GT       => seta  (C=0&Z=0)
                    GreaterThanOrEqual         => 0x93, // GT|EQ    => setae (C=0)
                    UnorderedOrLessThan        => 0x92, // UN|LT    => setb  (C=1)
                    UnorderedOrLessThanOrEqual => 0x96, // UN|LT|EQ => setbe (Z=1|C=1)
                    Equal |                       // EQ
                    NotEqual |                    // UN|LT|GT
                    LessThan |                    // LT
                    LessThanOrEqual |             // LT|EQ
                    UnorderedOrGreaterThan |      // UN|GT
                    UnorderedOrGreaterThanOrEqual // UN|GT|EQ
                    => panic!("{} not supported by fcscc", cond),
                };
                sink.put1(0x0f);
                sink.put1(setcc);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe Mp2fcscc
        159 => {
            if let InstructionData::FloatCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_mp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                // `setCC` instruction, no REX.
                use crate::ir::condcodes::FloatCC::*;
                let setcc = match cond {
                    Ordered                    => 0x9b, // EQ|LT|GT => setnp (P=0)
                    Unordered                  => 0x9a, // UN       => setp  (P=1)
                    OrderedNotEqual            => 0x95, // LT|GT    => setne (Z=0),
                    UnorderedOrEqual           => 0x94, // UN|EQ    => sete  (Z=1)
                    GreaterThan                => 0x97, // GT       => seta  (C=0&Z=0)
                    GreaterThanOrEqual         => 0x93, // GT|EQ    => setae (C=0)
                    UnorderedOrLessThan        => 0x92, // UN|LT    => setb  (C=1)
                    UnorderedOrLessThanOrEqual => 0x96, // UN|LT|EQ => setbe (Z=1|C=1)
                    Equal |                       // EQ
                    NotEqual |                    // UN|LT|GT
                    LessThan |                    // LT
                    LessThanOrEqual |             // LT|EQ
                    UnorderedOrGreaterThan |      // UN|GT
                    UnorderedOrGreaterThanOrEqual // UN|GT|EQ
                    => panic!("{} not supported by fcscc", cond),
                };
                sink.put1(0x0f);
                sink.put1(setcc);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe RexMp2fcscc
        160 => {
            if let InstructionData::FloatCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_rexmp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                // `setCC` instruction, no REX.
                use crate::ir::condcodes::FloatCC::*;
                let setcc = match cond {
                    Ordered                    => 0x9b, // EQ|LT|GT => setnp (P=0)
                    Unordered                  => 0x9a, // UN       => setp  (P=1)
                    OrderedNotEqual            => 0x95, // LT|GT    => setne (Z=0),
                    UnorderedOrEqual           => 0x94, // UN|EQ    => sete  (Z=1)
                    GreaterThan                => 0x97, // GT       => seta  (C=0&Z=0)
                    GreaterThanOrEqual         => 0x93, // GT|EQ    => setae (C=0)
                    UnorderedOrLessThan        => 0x92, // UN|LT    => setb  (C=1)
                    UnorderedOrLessThanOrEqual => 0x96, // UN|LT|EQ => setbe (Z=1|C=1)
                    Equal |                       // EQ
                    NotEqual |                    // UN|LT|GT
                    LessThan |                    // LT
                    LessThanOrEqual |             // LT|EQ
                    UnorderedOrGreaterThan |      // UN|GT
                    UnorderedOrGreaterThanOrEqual // UN|GT|EQ
                    => panic!("{} not supported by fcscc", cond),
                };
                sink.put1(0x0f);
                sink.put1(setcc);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe Op2fcmp
        161 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_op2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe RexOp2fcmp
        162 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexop2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe Mp2fcmp
        163 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_mp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe RexMp2fcmp
        164 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexmp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe DynRexOp1rr
        165 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_dynrexop1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe DynRexOp1rout
        166 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_dynrexop1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe DynRexOp1rin
        167 => {
            if let InstructionData::Ternary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_dynrexop1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe DynRexOp1rio
        168 => {
            if let InstructionData::Ternary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_dynrexop1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe DynRexOp1r_ib
        169 => {
            if let InstructionData::BinaryImm {
                opcode,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_dynrexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe DynRexOp1r_id
        170 => {
            if let InstructionData::BinaryImm {
                opcode,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_dynrexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                return;
            }
        }
        // Recipe DynRexOp1ur
        171 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_dynrexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                return;
            }
        }
        // Recipe Op1rr
        172 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_op1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe RexOp1rr
        173 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexop1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe DynRexOp2rrx
        174 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_dynrexop2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe DynRexOp1div
        175 => {
            if let InstructionData::Ternary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg2 = divert.reg(args[2], &func.locations);
                sink.trap(TrapCode::IntegerDivisionByZero, func.srclocs[inst]);
                put_dynrexop1(bits, rex1(in_reg2), sink);
                modrm_r_bits(in_reg2, bits, sink);
                return;
            }
        }
        // Recipe DynRexOp1mulx
        176 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_dynrexop1(bits, rex1(in_reg1), sink);
                modrm_r_bits(in_reg1, bits, sink);
                return;
            }
        }
        // Recipe Op2fa
        177 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_op2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe RexOp2fa
        178 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexop2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe Op2fax
        179 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_op2(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe RexOp2fax
        180 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexop2(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe Op1rc
        181 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_op1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                return;
            }
        }
        // Recipe RexOp1rc
        182 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_rexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                return;
            }
        }
        // Recipe Mp2urm
        183 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe RexMp2urm
        184 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexmp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe DynRexOp2bsf_and_bsr
        185 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = func.dfg.inst_results(inst);
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_dynrexop2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe DynRexOp1icscc
        186 => {
            if let InstructionData::IntCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_dynrexop1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                // `setCC` instruction, no REX.
                let setcc = 0x90 | icc2opc(cond);
                sink.put1(0x0f);
                sink.put1(setcc as u8);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe DynRexOp1icscc_ib
        187 => {
            if let InstructionData::IntCompareImm {
                opcode,
                cond,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_dynrexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put1(imm as u8);
                // `setCC` instruction, no REX.
                let setcc = 0x90 | icc2opc(cond);
                sink.put1(0x0f);
                sink.put1(setcc as u8);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe DynRexOp1icscc_id
        188 => {
            if let InstructionData::IntCompareImm {
                opcode,
                cond,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_dynrexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                // `setCC` instruction, no REX.
                let setcc = 0x90 | icc2opc(cond);
                sink.put1(0x0f);
                sink.put1(setcc as u8);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe DynRexOp1rcmp
        189 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_dynrexop1(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe DynRexOp1rcmp_ib
        190 => {
            if let InstructionData::BinaryImm {
                opcode,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_dynrexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe DynRexOp1rcmp_id
        191 => {
            if let InstructionData::BinaryImm {
                opcode,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_dynrexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put4(imm as u32);
                return;
            }
        }
        // Recipe Op1rcmp_sp
        192 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_op1(bits, rex2(in_reg0, RU::rsp.into()), sink);
                modrm_rr(in_reg0, RU::rsp.into(), sink);
                return;
            }
        }
        // Recipe RexOp1rcmp_sp
        193 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_rexop1(bits, rex2(in_reg0, RU::rsp.into()), sink);
                modrm_rr(in_reg0, RU::rsp.into(), sink);
                return;
            }
        }
        // Recipe Op2seti_abcd
        194 => {
            if let InstructionData::IntCond {
                opcode,
                cond,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op2(bits | icc2opc(cond), rex1(out_reg0), sink);
                modrm_r_bits(out_reg0, bits, sink);
                return;
            }
        }
        // Recipe RexOp2seti
        195 => {
            if let InstructionData::IntCond {
                opcode,
                cond,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop2(bits | icc2opc(cond), rex1(out_reg0), sink);
                modrm_r_bits(out_reg0, bits, sink);
                return;
            }
        }
        // Recipe Op2setf_abcd
        196 => {
            if let InstructionData::FloatCond {
                opcode,
                cond,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op2(bits | fcc2opc(cond), rex1(out_reg0), sink);
                modrm_r_bits(out_reg0, bits, sink);
                return;
            }
        }
        // Recipe RexOp2setf
        197 => {
            if let InstructionData::FloatCond {
                opcode,
                cond,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop2(bits | fcc2opc(cond), rex1(out_reg0), sink);
                modrm_r_bits(out_reg0, bits, sink);
                return;
            }
        }
        // Recipe DynRexOp2cmov
        198 => {
            if let InstructionData::IntSelect {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg1 = divert.reg(args[1], &func.locations);
                let in_reg2 = divert.reg(args[2], &func.locations);
                put_dynrexop2(bits | icc2opc(cond), rex2(in_reg1, in_reg2), sink);
                modrm_rr(in_reg1, in_reg2, sink);
                return;
            }
        }
        // Recipe Mp3fa
        199 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_mp3(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe Mp2r_ib_unsigned_fpr
        200 => {
            if let InstructionData::ExtractLane {
                opcode,
                lane,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp2(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(in_reg0, out_reg0, sink);
                let imm:i64 = lane.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe null_fpr
        201 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                return;
            }
        }
        // Recipe Mp3r_ib_unsigned_r
        202 => {
            if let InstructionData::InsertLane {
                opcode,
                lane,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_mp3(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                let imm:i64 = lane.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe Mp2r_ib_unsigned_r
        203 => {
            if let InstructionData::InsertLane {
                opcode,
                lane,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_mp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                let imm:i64 = lane.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe RexMp3r_ib_unsigned_r
        204 => {
            if let InstructionData::InsertLane {
                opcode,
                lane,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_rexmp3(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                let imm:i64 = lane.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe Mp3fa_ib
        205 => {
            if let InstructionData::InsertLane {
                opcode,
                lane,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_mp3(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                let imm:i64 = lane.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe Mp3r_ib_unsigned_gpr
        206 => {
            if let InstructionData::ExtractLane {
                opcode,
                lane,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_mp3(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(out_reg0, in_reg0, sink); // note the flipped register in the ModR/M byte
                let imm:i64 = lane.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe RexMp3r_ib_unsigned_gpr
        207 => {
            if let InstructionData::ExtractLane {
                opcode,
                lane,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexmp3(bits, rex2(in_reg0, out_reg0), sink);
                modrm_rr(out_reg0, in_reg0, sink); // note the flipped register in the ModR/M byte
                let imm:i64 = lane.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe DynRexMp2vconst_optimized
        208 => {
            if let InstructionData::UnaryConst {
                opcode,
                constant_handle,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_dynrexmp2(bits, rex2(out_reg0, out_reg0), sink);
                modrm_rr(out_reg0, out_reg0, sink);
                return;
            }
        }
        // Recipe DynRexOp2vconst
        209 => {
            if let InstructionData::UnaryConst {
                opcode,
                constant_handle,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_dynrexop2(bits, rex2(0, out_reg0), sink);
                modrm_riprel(out_reg0, sink);
                const_disp4(constant_handle, func, sink);
                return;
            }
        }
        // Recipe DynRexOp2fst
        210 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_dynrexop2(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else if needs_offset(in_reg1) {
                    modrm_disp8(in_reg1, in_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg1, in_reg0, sink);
                }
                return;
            }
        }
        // Recipe Op2fstDisp8
        211 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp8(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp8(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op2fstDisp32
        212 => {
            if let InstructionData::Store {
                opcode,
                flags,
                offset,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex2(in_reg1, in_reg0), sink);
                if needs_sib_byte(in_reg1) {
                    modrm_sib_disp32(in_reg0, sink);
                    sib_noindex(in_reg1, sink);
                } else {
                    modrm_disp32(in_reg1, in_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe DynRexOp2fld
        213 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_dynrexop2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else if needs_offset(in_reg0) {
                    modrm_disp8(in_reg0, out_reg0, sink);
                    sink.put1(0);
                } else {
                    modrm_rm(in_reg0, out_reg0, sink);
                }
                return;
            }
        }
        // Recipe Op2fldDisp8
        214 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp8(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put1(offset as u8);
                return;
            }
        }
        // Recipe Op2fldDisp32
        215 => {
            if let InstructionData::Load {
                opcode,
                flags,
                offset,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                if !flags.notrap() {
                    sink.trap(TrapCode::HeapOutOfBounds, func.srclocs[inst]);
                }
                put_op2(bits, rex2(in_reg0, out_reg0), sink);
                if needs_sib_byte(in_reg0) {
                    modrm_sib_disp32(out_reg0, sink);
                    sib_noindex(in_reg0, sink);
                } else {
                    modrm_disp32(in_reg0, out_reg0, sink);
                }
                let offset: i32 = offset.into();
                sink.put4(offset as u32);
                return;
            }
        }
        // Recipe Op2fspillSib32
        216 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_stk0 = StackRef::masked(
                    divert.stack(results[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let base = stk_base(out_stk0.base);
                put_op2(bits, rex2(base, in_reg0), sink);
                modrm_sib_disp32(in_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(out_stk0.offset as u32);
                return;
            }
        }
        // Recipe Op2fregspill32
        217 => {
            if let InstructionData::RegSpill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                let dst = StackRef::sp(dst, &func.stack_slots);
                let base = stk_base(dst.base);
                put_op2(bits, rex2(base, src), sink);
                modrm_sib_disp32(src, sink);
                sib_noindex(base, sink);
                sink.put4(dst.offset as u32);
                return;
            }
        }
        // Recipe Op2ffillSib32
        218 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_stk0 = StackRef::masked(
                    divert.stack(args[0], &func.locations),
                    StackBaseMask(1),
                    &func.stack_slots,
                ).unwrap();
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                let base = stk_base(in_stk0.base);
                put_op2(bits, rex2(base, out_reg0), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib_noindex(base, sink);
                sink.put4(in_stk0.offset as u32);
                return;
            }
        }
        // Recipe Op2fregfill32
        219 => {
            if let InstructionData::RegFill {
                opcode,
                src,
                dst,
                arg,
                ..
            } = *inst_data {
                divert.apply(inst_data);
                let src = StackRef::sp(src, &func.stack_slots);
                let base = stk_base(src.base);
                put_op2(bits, rex2(base, dst), sink);
                modrm_sib_disp32(dst, sink);
                sib_noindex(base, sink);
                sink.put4(src.offset as u32);
                return;
            }
        }
        // Recipe Mp2fax
        220 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_mp2(bits, rex2(in_reg0, in_reg1), sink);
                modrm_rr(in_reg0, in_reg1, sink);
                return;
            }
        }
        // Recipe Mp3fcmp
        221 => {
            if let InstructionData::Binary {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                put_mp3(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe Mp2f_ib
        222 => {
            if let InstructionData::BinaryImm {
                opcode,
                imm,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_mp2(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                let imm: i64 = imm.into();
                sink.put1(imm as u8);
                return;
            }
        }
        // Recipe Mp2icscc_fpr
        223 => {
            if let InstructionData::IntCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                // Comparison instruction.
                put_mp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe Mp3icscc_fpr
        224 => {
            if let InstructionData::IntCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                // Comparison instruction.
                put_mp3(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                return;
            }
        }
        // Recipe Op2pfcmp
        225 => {
            if let InstructionData::FloatCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                // Comparison instruction.
                put_op2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                // Add immediate byte indicating what type of comparison.
                use crate::ir::condcodes::FloatCC::*;
                let imm = match cond {
                    Equal                      => 0x00,
                    LessThan                   => 0x01,
                    LessThanOrEqual            => 0x02,
                    Unordered                  => 0x03,
                    NotEqual                   => 0x04,
                    UnorderedOrGreaterThanOrEqual => 0x05,
                    UnorderedOrGreaterThan => 0x06,
                    Ordered                    => 0x07,
                    _ => panic!("{} not supported by pfcmp", cond),
                };
                sink.put1(imm);
                return;
            }
        }
        // Recipe RexOp2pfcmp
        226 => {
            if let InstructionData::FloatCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                // Comparison instruction.
                put_rexop2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                // Add immediate byte indicating what type of comparison.
                use crate::ir::condcodes::FloatCC::*;
                let imm = match cond {
                    Equal                      => 0x00,
                    LessThan                   => 0x01,
                    LessThanOrEqual            => 0x02,
                    Unordered                  => 0x03,
                    NotEqual                   => 0x04,
                    UnorderedOrGreaterThanOrEqual => 0x05,
                    UnorderedOrGreaterThan => 0x06,
                    Ordered                    => 0x07,
                    _ => panic!("{} not supported by pfcmp", cond),
                };
                sink.put1(imm);
                return;
            }
        }
        // Recipe Mp2pfcmp
        227 => {
            if let InstructionData::FloatCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                // Comparison instruction.
                put_mp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                // Add immediate byte indicating what type of comparison.
                use crate::ir::condcodes::FloatCC::*;
                let imm = match cond {
                    Equal                      => 0x00,
                    LessThan                   => 0x01,
                    LessThanOrEqual            => 0x02,
                    Unordered                  => 0x03,
                    NotEqual                   => 0x04,
                    UnorderedOrGreaterThanOrEqual => 0x05,
                    UnorderedOrGreaterThan => 0x06,
                    Ordered                    => 0x07,
                    _ => panic!("{} not supported by pfcmp", cond),
                };
                sink.put1(imm);
                return;
            }
        }
        // Recipe RexMp2pfcmp
        228 => {
            if let InstructionData::FloatCompare {
                opcode,
                cond,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                // Comparison instruction.
                put_rexmp2(bits, rex2(in_reg1, in_reg0), sink);
                modrm_rr(in_reg1, in_reg0, sink);
                // Add immediate byte indicating what type of comparison.
                use crate::ir::condcodes::FloatCC::*;
                let imm = match cond {
                    Equal                      => 0x00,
                    LessThan                   => 0x01,
                    LessThanOrEqual            => 0x02,
                    Unordered                  => 0x03,
                    NotEqual                   => 0x04,
                    UnorderedOrGreaterThanOrEqual => 0x05,
                    UnorderedOrGreaterThan => 0x06,
                    Ordered                    => 0x07,
                    _ => panic!("{} not supported by pfcmp", cond),
                };
                sink.put1(imm);
                return;
            }
        }
        // Recipe Op1fnaddr4
        229 => {
            if let InstructionData::FuncAddr {
                opcode,
                func_ref,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                sink.reloc_external(Reloc::Abs4,
                                    &func.dfg.ext_funcs[func_ref].name,
                                    0);
                sink.put4(0);
                return;
            }
        }
        // Recipe RexOp1fnaddr8
        230 => {
            if let InstructionData::FuncAddr {
                opcode,
                func_ref,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                sink.reloc_external(Reloc::Abs8,
                                    &func.dfg.ext_funcs[func_ref].name,
                                    0);
                sink.put8(0);
                return;
            }
        }
        // Recipe Op1allones_fnaddr4
        231 => {
            if let InstructionData::FuncAddr {
                opcode,
                func_ref,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                sink.reloc_external(Reloc::Abs4,
                                    &func.dfg.ext_funcs[func_ref].name,
                                    0);
                // Write the immediate as `!0` for the benefit of BaldrMonkey.
                sink.put4(!0);
                return;
            }
        }
        // Recipe RexOp1allones_fnaddr8
        232 => {
            if let InstructionData::FuncAddr {
                opcode,
                func_ref,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                sink.reloc_external(Reloc::Abs8,
                                    &func.dfg.ext_funcs[func_ref].name,
                                    0);
                // Write the immediate as `!0` for the benefit of BaldrMonkey.
                sink.put8(!0);
                return;
            }
        }
        // Recipe RexOp1pcrel_fnaddr8
        233 => {
            if let InstructionData::FuncAddr {
                opcode,
                func_ref,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex2(0, out_reg0), sink);
                modrm_riprel(out_reg0, sink);
                // The addend adjusts for the difference between the end of the
                // instruction and the beginning of the immediate field.
                sink.reloc_external(Reloc::X86PCRel4,
                                    &func.dfg.ext_funcs[func_ref].name,
                                    -4);
                sink.put4(0);
                return;
            }
        }
        // Recipe RexOp1got_fnaddr8
        234 => {
            if let InstructionData::FuncAddr {
                opcode,
                func_ref,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex2(0, out_reg0), sink);
                modrm_riprel(out_reg0, sink);
                // The addend adjusts for the difference between the end of the
                // instruction and the beginning of the immediate field.
                sink.reloc_external(Reloc::X86GOTPCRel4,
                                    &func.dfg.ext_funcs[func_ref].name,
                                    -4);
                sink.put4(0);
                return;
            }
        }
        // Recipe Op1gvaddr4
        235 => {
            if let InstructionData::UnaryGlobalValue {
                opcode,
                global_value,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                sink.reloc_external(Reloc::Abs4,
                                    &func.global_values[global_value].symbol_name(),
                                    0);
                sink.put4(0);
                return;
            }
        }
        // Recipe RexOp1gvaddr8
        236 => {
            if let InstructionData::UnaryGlobalValue {
                opcode,
                global_value,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                sink.reloc_external(Reloc::Abs8,
                                    &func.global_values[global_value].symbol_name(),
                                    0);
                sink.put8(0);
                return;
            }
        }
        // Recipe RexOp1pcrel_gvaddr8
        237 => {
            if let InstructionData::UnaryGlobalValue {
                opcode,
                global_value,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex2(0, out_reg0), sink);
                modrm_rm(5, out_reg0, sink);
                // The addend adjusts for the difference between the end of the
                // instruction and the beginning of the immediate field.
                sink.reloc_external(Reloc::X86PCRel4,
                                    &func.global_values[global_value].symbol_name(),
                                    -4);
                sink.put4(0);
                return;
            }
        }
        // Recipe RexOp1got_gvaddr8
        238 => {
            if let InstructionData::UnaryGlobalValue {
                opcode,
                global_value,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex2(0, out_reg0), sink);
                modrm_rm(5, out_reg0, sink);
                // The addend adjusts for the difference between the end of the
                // instruction and the beginning of the immediate field.
                sink.reloc_external(Reloc::X86GOTPCRel4,
                                    &func.global_values[global_value].symbol_name(),
                                    -4);
                sink.put4(0);
                return;
            }
        }
        // Recipe Op1spaddr4_id
        239 => {
            if let InstructionData::StackLoad {
                opcode,
                stack_slot,
                offset,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                let sp = StackRef::sp(stack_slot, &func.stack_slots);
                let base = stk_base(sp.base);
                put_op1(bits, rex2(out_reg0, base), sink);
                modrm_sib_disp8(out_reg0, sink);
                sib_noindex(base, sink);
                let imm : i32 = offset.into();
                sink.put4(sp.offset.checked_add(imm).unwrap() as u32);
                return;
            }
        }
        // Recipe RexOp1spaddr8_id
        240 => {
            if let InstructionData::StackLoad {
                opcode,
                stack_slot,
                offset,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                let sp = StackRef::sp(stack_slot, &func.stack_slots);
                let base = stk_base(sp.base);
                put_rexop1(bits, rex2(base, out_reg0), sink);
                modrm_sib_disp32(out_reg0, sink);
                sib_noindex(base, sink);
                let imm : i32 = offset.into();
                sink.put4(sp.offset.checked_add(imm).unwrap() as u32);
                return;
            }
        }
        // Recipe Op1call_id
        241 => {
            if let InstructionData::Call {
                opcode,
                func_ref,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                put_op1(bits, BASE_REX, sink);
                // The addend adjusts for the difference between the end of the
                // instruction and the beginning of the immediate field.
                sink.reloc_external(Reloc::X86CallPCRel4,
                                    &func.dfg.ext_funcs[func_ref].name,
                                    -4);
                sink.put4(0);
                return;
            }
        }
        // Recipe Op1call_plt_id
        242 => {
            if let InstructionData::Call {
                opcode,
                func_ref,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                put_op1(bits, BASE_REX, sink);
                sink.reloc_external(Reloc::X86CallPLTRel4,
                                    &func.dfg.ext_funcs[func_ref].name,
                                    -4);
                sink.put4(0);
                return;
            }
        }
        // Recipe Op1call_r
        243 => {
            if let InstructionData::CallIndirect {
                opcode,
                sig_ref,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                put_op1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                return;
            }
        }
        // Recipe RexOp1call_r
        244 => {
            if let InstructionData::CallIndirect {
                opcode,
                sig_ref,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                sink.trap(TrapCode::StackOverflow, func.srclocs[inst]);
                put_rexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                return;
            }
        }
        // Recipe Op1ret
        245 => {
            if let InstructionData::MultiAry {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_op1(bits, BASE_REX, sink);
                return;
            }
        }
        // Recipe Op1jmpb
        246 => {
            if let InstructionData::Jump {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_op1(bits, BASE_REX, sink);
                disp1(destination, func, sink);
                return;
            }
        }
        // Recipe Op1jmpd
        247 => {
            if let InstructionData::Jump {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_op1(bits, BASE_REX, sink);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe Op1brib
        248 => {
            if let InstructionData::BranchInt {
                opcode,
                cond,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_op1(bits | icc2opc(cond), BASE_REX, sink);
                disp1(destination, func, sink);
                return;
            }
        }
        // Recipe RexOp1brib
        249 => {
            if let InstructionData::BranchInt {
                opcode,
                cond,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_rexop1(bits | icc2opc(cond), BASE_REX, sink);
                disp1(destination, func, sink);
                return;
            }
        }
        // Recipe Op2brid
        250 => {
            if let InstructionData::BranchInt {
                opcode,
                cond,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_op2(bits | icc2opc(cond), BASE_REX, sink);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe RexOp2brid
        251 => {
            if let InstructionData::BranchInt {
                opcode,
                cond,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_rexop2(bits | icc2opc(cond), BASE_REX, sink);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe Op1brfb
        252 => {
            if let InstructionData::BranchFloat {
                opcode,
                cond,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_op1(bits | fcc2opc(cond), BASE_REX, sink);
                disp1(destination, func, sink);
                return;
            }
        }
        // Recipe RexOp1brfb
        253 => {
            if let InstructionData::BranchFloat {
                opcode,
                cond,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_rexop1(bits | fcc2opc(cond), BASE_REX, sink);
                disp1(destination, func, sink);
                return;
            }
        }
        // Recipe Op2brfd
        254 => {
            if let InstructionData::BranchFloat {
                opcode,
                cond,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_op2(bits | fcc2opc(cond), BASE_REX, sink);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe RexOp2brfd
        255 => {
            if let InstructionData::BranchFloat {
                opcode,
                cond,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                put_rexop2(bits | fcc2opc(cond), BASE_REX, sink);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe Op1tjccb
        256 => {
            if let InstructionData::Branch {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                // test r, r.
                put_op1((bits & 0xff00) | 0x85, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Jcc instruction.
                sink.put1(bits as u8);
                disp1(destination, func, sink);
                return;
            }
        }
        // Recipe RexOp1tjccb
        257 => {
            if let InstructionData::Branch {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                // test r, r.
                put_rexop1((bits & 0xff00) | 0x85, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Jcc instruction.
                sink.put1(bits as u8);
                disp1(destination, func, sink);
                return;
            }
        }
        // Recipe Op1tjccd
        258 => {
            if let InstructionData::Branch {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                // test r, r.
                put_op1((bits & 0xff00) | 0x85, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Jcc instruction.
                sink.put1(0x0f);
                sink.put1(bits as u8);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe RexOp1tjccd
        259 => {
            if let InstructionData::Branch {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                // test r, r.
                put_rexop1((bits & 0xff00) | 0x85, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Jcc instruction.
                sink.put1(0x0f);
                sink.put1(bits as u8);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe Op1t8jccd_long
        260 => {
            if let InstructionData::Branch {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                // test32 r, 0xff.
                put_op1((bits & 0xff00) | 0xf7, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                sink.put4(0xff);
                // Jcc instruction.
                sink.put1(0x0f);
                sink.put1(bits as u8);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe Op1t8jccb_abcd
        261 => {
            if let InstructionData::Branch {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                // test8 r, r.
                put_op1((bits & 0xff00) | 0x84, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Jcc instruction.
                sink.put1(bits as u8);
                disp1(destination, func, sink);
                return;
            }
        }
        // Recipe RexOp1t8jccb
        262 => {
            if let InstructionData::Branch {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                // test8 r, r.
                put_rexop1((bits & 0xff00) | 0x84, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Jcc instruction.
                sink.put1(bits as u8);
                disp1(destination, func, sink);
                return;
            }
        }
        // Recipe Op1t8jccd_abcd
        263 => {
            if let InstructionData::Branch {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                // test8 r, r.
                put_op1((bits & 0xff00) | 0x84, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Jcc instruction.
                sink.put1(0x0f);
                sink.put1(bits as u8);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe RexOp1t8jccd
        264 => {
            if let InstructionData::Branch {
                opcode,
                destination,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                let in_reg0 = divert.reg(args[0], &func.locations);
                // test8 r, r.
                put_rexop1((bits & 0xff00) | 0x84, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Jcc instruction.
                sink.put1(0x0f);
                sink.put1(bits as u8);
                disp4(destination, func, sink);
                return;
            }
        }
        // Recipe RexOp1jt_entry
        265 => {
            if let InstructionData::BranchTableEntry {
                opcode,
                imm,
                table,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex3(in_reg1, out_reg0, in_reg0), sink);
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib(imm.trailing_zeros() as u8, in_reg0, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(out_reg0, sink);
                    sib(imm.trailing_zeros() as u8, in_reg0, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe Op1jt_entry
        266 => {
            if let InstructionData::BranchTableEntry {
                opcode,
                imm,
                table,
                ref args,
                ..
            } = *inst_data {
                let in_reg0 = divert.reg(args[0], &func.locations);
                let in_reg1 = divert.reg(args[1], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op1(bits, rex3(in_reg1, out_reg0, in_reg0), sink);
                if needs_offset(in_reg1) {
                    modrm_sib_disp8(out_reg0, sink);
                    sib(imm.trailing_zeros() as u8, in_reg0, in_reg1, sink);
                    sink.put1(0);
                } else {
                    modrm_sib(out_reg0, sink);
                    sib(imm.trailing_zeros() as u8, in_reg0, in_reg1, sink);
                }
                return;
            }
        }
        // Recipe RexOp1jt_base
        267 => {
            if let InstructionData::BranchTableBase {
                opcode,
                table,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_rexop1(bits, rex2(0, out_reg0), sink);
                modrm_riprel(out_reg0, sink);
                
                // No reloc is needed here as the jump table is emitted directly after
                // the function body.
                jt_disp4(table, func, sink);
                return;
            }
        }
        // Recipe Op1jt_base
        268 => {
            if let InstructionData::BranchTableBase {
                opcode,
                table,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                put_op1(bits, rex2(0, out_reg0), sink);
                modrm_riprel(out_reg0, sink);
                
                // No reloc is needed here as the jump table is emitted directly after
                // the function body.
                jt_disp4(table, func, sink);
                return;
            }
        }
        // Recipe RexOp1indirect_jmp
        269 => {
            if let InstructionData::IndirectJump {
                opcode,
                table,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_rexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                return;
            }
        }
        // Recipe Op1indirect_jmp
        270 => {
            if let InstructionData::IndirectJump {
                opcode,
                table,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                put_op1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                return;
            }
        }
        // Recipe Op2trap
        271 => {
            if let InstructionData::Trap {
                opcode,
                code,
                ..
            } = *inst_data {
                sink.trap(code, func.srclocs[inst]);
                put_op2(bits, BASE_REX, sink);
                return;
            }
        }
        // Recipe debugtrap
        272 => {
            if let InstructionData::NullAry {
                opcode,
                ..
            } = *inst_data {
                sink.put1(0xcc);
                return;
            }
        }
        // Recipe trapif
        273 => {
            if let InstructionData::IntCondTrap {
                opcode,
                cond,
                code,
                ..
            } = *inst_data {
                // Jump over a 2-byte ud2.
                sink.put1(0x70 | (icc2opc(cond.inverse()) as u8));
                sink.put1(2);
                // ud2.
                sink.trap(code, func.srclocs[inst]);
                sink.put1(0x0f);
                sink.put1(0x0b);
                return;
            }
        }
        // Recipe trapff
        274 => {
            if let InstructionData::FloatCondTrap {
                opcode,
                cond,
                code,
                ..
            } = *inst_data {
                // Jump over a 2-byte ud2.
                sink.put1(0x70 | (fcc2opc(cond.inverse()) as u8));
                sink.put1(2);
                // ud2.
                sink.trap(code, func.srclocs[inst]);
                sink.put1(0x0f);
                sink.put1(0x0b);
                return;
            }
        }
        // Recipe Op1pu_id_ref
        275 => {
            if let InstructionData::NullAry {
                opcode,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // The destination register is encoded in the low bits of the opcode.
                // No ModR/M.
                put_op1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                sink.put4(0);
                return;
            }
        }
        // Recipe RexOp1pu_id_ref
        276 => {
            if let InstructionData::NullAry {
                opcode,
                ..
            } = *inst_data {
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // The destination register is encoded in the low bits of the opcode.
                // No ModR/M.
                put_rexop1(bits | (out_reg0 & 7), rex1(out_reg0), sink);
                sink.put4(0);
                return;
            }
        }
        // Recipe Op1is_zero
        277 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Test instruction.
                put_op1(bits, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Check ZF = 1 flag to see if register holds 0.
                sink.put1(0x0f);
                sink.put1(0x94);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe RexOp1is_zero
        278 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Test instruction.
                put_rexop1(bits, rex2(in_reg0, in_reg0), sink);
                modrm_rr(in_reg0, in_reg0, sink);
                // Check ZF = 1 flag to see if register holds 0.
                sink.put1(0x0f);
                sink.put1(0x94);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe Op1is_invalid
        279 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_op1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                sink.put1(0xff);
                // `setCC` instruction, no REX.
                use crate::ir::condcodes::IntCC::*;
                let setcc = 0x90 | icc2opc(Equal);
                sink.put1(0x0f);
                sink.put1(setcc as u8);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe RexOp1is_invalid
        280 => {
            if let InstructionData::Unary {
                opcode,
                arg,
                ..
            } = *inst_data {
                let args = [arg];
                let in_reg0 = divert.reg(args[0], &func.locations);
                let results = [func.dfg.first_result(inst)];
                let out_reg0 = divert.reg(results[0], &func.locations);
                // Comparison instruction.
                put_rexop1(bits, rex1(in_reg0), sink);
                modrm_r_bits(in_reg0, bits, sink);
                sink.put1(0xff);
                // `setCC` instruction, no REX.
                use crate::ir::condcodes::IntCC::*;
                let setcc = 0x90 | icc2opc(Equal);
                sink.put1(0x0f);
                sink.put1(setcc as u8);
                modrm_rr(out_reg0, 0, sink);
                return;
            }
        }
        // Recipe safepoint
        281 => {
            if let InstructionData::MultiAry {
                opcode,
                ref args,
                ..
            } = *inst_data {
                let args = args.as_slice(&func.dfg.value_lists);
                sink.add_stackmap(args, func, isa);
                return;
            }
        }
        _ => {},
    }
    if encoding.is_legal() {
        bad_encoding(func, inst);
    }
}
