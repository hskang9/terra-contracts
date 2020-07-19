// x86 recipe predicates.
fn recipe_predicate_rexop1u_id(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryImm { imm, .. } = *inst {
        return predicates::is_signed_int(imm, 32, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1ldwithindex(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::LoadComplex { offset, .. } = *inst {
        return predicates::is_equal(offset, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1ldwithindexdisp8(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::LoadComplex { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1ldwithindexdisp32(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::LoadComplex { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 32, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1stwithindex(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::StoreComplex { offset, .. } = *inst {
        return predicates::is_equal(offset, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1stwithindexdisp8(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::StoreComplex { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1stwithindexdisp32(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::StoreComplex { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 32, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1st(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::Store { offset, .. } = *inst {
        return predicates::is_equal(offset, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1stdisp8(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::Store { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1ld(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::Load { offset, .. } = *inst {
        return predicates::is_equal(offset, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1lddisp8(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::Load { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1lddisp32(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::Load { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 32, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1adjustsp_ib(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryImm { imm, .. } = *inst {
        return predicates::is_signed_int(imm, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op2f32imm_z(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryIeee32 { imm, .. } = *inst {
        return predicates::is_zero_32_bit_float(imm);
    }
    unreachable!();
}
fn recipe_predicate_mp2f64imm_z(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryIeee64 { imm, .. } = *inst {
        return predicates::is_zero_64_bit_float(imm);
    }
    unreachable!();
}
fn recipe_predicate_mp3furmi_rnd(isap: crate::settings::PredicateView, _: &ir::InstructionData) -> bool {
    isap.test(16)
}
fn recipe_predicate_op2fcscc(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::FloatCompare { cond, .. } = *inst {
        return predicates::is_equal(cond, ir::condcodes::FloatCC::Ordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::Unordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::OrderedNotEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::GreaterThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::GreaterThanOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrLessThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrLessThanOrEqual);
    }
    unreachable!();
}
fn recipe_predicate_dynrexop1r_ib(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::BinaryImm { imm, .. } = *inst {
        return predicates::is_signed_int(imm, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_dynrexop1r_id(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::BinaryImm { imm, .. } = *inst {
        return predicates::is_signed_int(imm, 32, 0);
    }
    unreachable!();
}
fn recipe_predicate_dynrexop1icscc_ib(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::IntCompareImm { imm, .. } = *inst {
        return predicates::is_signed_int(imm, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_dynrexop1icscc_id(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::IntCompareImm { imm, .. } = *inst {
        return predicates::is_signed_int(imm, 32, 0);
    }
    unreachable!();
}
fn recipe_predicate_mp2r_ib_unsigned_fpr(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::ExtractLane { lane, .. } = *inst {
        return predicates::is_unsigned_int(lane, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_mp3r_ib_unsigned_r(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::InsertLane { lane, .. } = *inst {
        return predicates::is_unsigned_int(lane, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op2pfcmp(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::FloatCompare { cond, .. } = *inst {
        return predicates::is_equal(cond, ir::condcodes::FloatCC::Equal) || predicates::is_equal(cond, ir::condcodes::FloatCC::LessThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::LessThanOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::Unordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::NotEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrGreaterThanOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrGreaterThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::Ordered);
    }
    unreachable!();
}
fn recipe_predicate_op1brfb(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::BranchFloat { cond, .. } = *inst {
        return predicates::is_equal(cond, ir::condcodes::FloatCC::Ordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::Unordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::OrderedNotEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::GreaterThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::GreaterThanOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrLessThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrLessThanOrEqual);
    }
    unreachable!();
}
fn recipe_predicate_rexop1jt_entry(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::BranchTableEntry { imm, .. } = *inst {
        return predicates::is_equal(imm, 1) || predicates::is_equal(imm, 2) || predicates::is_equal(imm, 4) || predicates::is_equal(imm, 8);
    }
    unreachable!();
}
fn recipe_predicate_trapff(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::FloatCondTrap { cond, .. } = *inst {
        return predicates::is_equal(cond, ir::condcodes::FloatCC::Ordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::Unordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::OrderedNotEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::GreaterThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::GreaterThanOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrLessThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrLessThanOrEqual);
    }
    unreachable!();
}

/// x86 recipe predicate table.
///
/// One entry per recipe, set to Some only when the recipe is guarded by a predicate.
pub static RECIPE_PREDICATES: [RecipePredicate; 282] = [
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_rexop1u_id),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1stdisp8),
    Some(recipe_predicate_op1stdisp8),
    Some(recipe_predicate_op1stdisp8),
    Some(recipe_predicate_op1stdisp8),
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1stdisp8),
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1lddisp32),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1adjustsp_ib),
    Some(recipe_predicate_rexop1u_id),
    Some(recipe_predicate_op1adjustsp_ib),
    Some(recipe_predicate_rexop1u_id),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1stdisp8),
    Some(recipe_predicate_op1stdisp8),
    None,
    None,
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindexdisp32),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op2f32imm_z),
    Some(recipe_predicate_mp2f64imm_z),
    Some(recipe_predicate_op2f32imm_z),
    Some(recipe_predicate_mp2f64imm_z),
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_mp3furmi_rnd),
    Some(recipe_predicate_mp3furmi_rnd),
    None,
    None,
    Some(recipe_predicate_op2fcscc),
    Some(recipe_predicate_op2fcscc),
    Some(recipe_predicate_op2fcscc),
    Some(recipe_predicate_op2fcscc),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_dynrexop1r_ib),
    Some(recipe_predicate_dynrexop1r_id),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_dynrexop1icscc_ib),
    Some(recipe_predicate_dynrexop1icscc_id),
    None,
    Some(recipe_predicate_dynrexop1r_ib),
    Some(recipe_predicate_dynrexop1r_id),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_mp2r_ib_unsigned_fpr),
    None,
    Some(recipe_predicate_mp3r_ib_unsigned_r),
    Some(recipe_predicate_mp3r_ib_unsigned_r),
    Some(recipe_predicate_mp3r_ib_unsigned_r),
    Some(recipe_predicate_mp3r_ib_unsigned_r),
    Some(recipe_predicate_mp2r_ib_unsigned_fpr),
    Some(recipe_predicate_mp2r_ib_unsigned_fpr),
    None,
    None,
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1stdisp8),
    None,
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp32),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_dynrexop1r_ib),
    None,
    None,
    Some(recipe_predicate_op2pfcmp),
    Some(recipe_predicate_op2pfcmp),
    Some(recipe_predicate_op2pfcmp),
    Some(recipe_predicate_op2pfcmp),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1brfb),
    Some(recipe_predicate_op1brfb),
    Some(recipe_predicate_op1brfb),
    Some(recipe_predicate_op1brfb),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_rexop1jt_entry),
    Some(recipe_predicate_rexop1jt_entry),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_trapff),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];

// x86 instruction predicates.
fn inst_predicate_0(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryImm { imm, .. } = *inst {
        let _ = func;
        return predicates::is_unsigned_int(imm, 32, 0);
    }
    unreachable!();
}
fn inst_predicate_1(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryImm { imm, .. } = *inst {
        let _ = func;
        return predicates::is_zero_int(imm);
    }
    unreachable!();
}
fn inst_predicate_2(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I16
}
fn inst_predicate_3(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I32
}
fn inst_predicate_4(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I64
}
fn inst_predicate_5(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I8
}
fn inst_predicate_6(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B1
}
fn inst_predicate_7(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B8
}
fn inst_predicate_8(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::LoadComplex { ref args, .. } = *inst {
        let _ = func;
        return predicates::has_length_of(args, 2, func);
    }
    unreachable!();
}
fn inst_predicate_9(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::StoreComplex { ref args, .. } = *inst {
        let _ = func;
        return predicates::has_length_of(args, 3, func);
    }
    unreachable!();
}
fn inst_predicate_10(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::F32
}
fn inst_predicate_11(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::F64
}
fn inst_predicate_12(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryIeee32 { imm, .. } = *inst {
        let _ = func;
        return predicates::is_zero_32_bit_float(imm);
    }
    unreachable!();
}
fn inst_predicate_13(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryIeee64 { imm, .. } = *inst {
        let _ = func;
        return predicates::is_zero_64_bit_float(imm);
    }
    unreachable!();
}
fn inst_predicate_14(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B8X16
}
fn inst_predicate_15(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B16X8
}
fn inst_predicate_16(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B32X4
}
fn inst_predicate_17(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B64X2
}
fn inst_predicate_18(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I8X16
}
fn inst_predicate_19(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I16X8
}
fn inst_predicate_20(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I32X4
}
fn inst_predicate_21(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I64X2
}
fn inst_predicate_22(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::F32X4
}
fn inst_predicate_23(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::F64X2
}
fn inst_predicate_24(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryConst { constant_handle, .. } = *inst {
        let _ = func;
        return predicates::is_all_zeroes(func.dfg.constants.get(constant_handle));
    }
    unreachable!();
}
fn inst_predicate_25(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryConst { constant_handle, .. } = *inst {
        let _ = func;
        return predicates::is_all_ones(func.dfg.constants.get(constant_handle));
    }
    unreachable!();
}
fn inst_predicate_26(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::IntCompare { cond, .. } = *inst {
        let _ = func;
        return predicates::is_equal(cond, IntCC::Equal);
    }
    unreachable!();
}
fn inst_predicate_27(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::IntCompare { cond, .. } = *inst {
        let _ = func;
        return predicates::is_equal(cond, IntCC::SignedGreaterThan);
    }
    unreachable!();
}
fn inst_predicate_28(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::FuncAddr { func_ref, .. } = *inst {
        let _ = func;
        return predicates::is_colocated_func(func_ref, func);
    }
    unreachable!();
}
fn inst_predicate_29(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryGlobalValue { global_value, .. } = *inst {
        let _ = func;
        return predicates::is_colocated_data(global_value, func);
    }
    unreachable!();
}
fn inst_predicate_30(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::Call { func_ref, .. } = *inst {
        let _ = func;
        return predicates::is_colocated_func(func_ref, func);
    }
    unreachable!();
}

/// x86 instruction predicate table.
///
/// One entry per instruction predicate, so the encoding bytecode can embed indexes into this
/// table.
pub static INST_PREDICATES: [InstPredicate; 31] = [
    inst_predicate_0,
    inst_predicate_1,
    inst_predicate_2,
    inst_predicate_3,
    inst_predicate_4,
    inst_predicate_5,
    inst_predicate_6,
    inst_predicate_7,
    inst_predicate_8,
    inst_predicate_9,
    inst_predicate_10,
    inst_predicate_11,
    inst_predicate_12,
    inst_predicate_13,
    inst_predicate_14,
    inst_predicate_15,
    inst_predicate_16,
    inst_predicate_17,
    inst_predicate_18,
    inst_predicate_19,
    inst_predicate_20,
    inst_predicate_21,
    inst_predicate_22,
    inst_predicate_23,
    inst_predicate_24,
    inst_predicate_25,
    inst_predicate_26,
    inst_predicate_27,
    inst_predicate_28,
    inst_predicate_29,
    inst_predicate_30,
];

/// x86 encoding lists.
///
/// This contains the entire encodings bytecode for every single instruction; the encodings
/// interpreter knows where to start from thanks to the initial lookup in the level 1 and level 2
/// table entries below.
pub static ENCLISTS: [u16; 2068] = [
    // 000000: adjust_sp_down.i64 (I64)
    // --> [RexOp1adjustsp#8029] and stop
    0x00c7, 0x8029,
    // end of adjust_sp_down.i64 (I64)
    // 000002: band.i64 (I64)
    // --> [DynRexOp1rr#8021] and stop
    // 000002: band.b64 (I64)
    // --> [DynRexOp1rr#8021] and stop
    0x014b, 0x8021,
    // end of band.b64 (I64)
    // end of band.i64 (I64)
    // 000004: band_imm.i64 (I64)
    // --> [DynRexOp1r_ib#c083]
    0x0152, 0xc083,
    // --> [DynRexOp1r_id#c081] and stop
    0x0155, 0xc081,
    // end of band_imm.i64 (I64)
    // 000008: bint.i64 (I64)
    // skip 4 unless inst_predicate_6
    // 000008: bint.i32 (I64)
    // skip 4 unless inst_predicate_6
    // 000008: bint.i8 (I64)
    // skip 4 unless inst_predicate_6
    // 000008: bint.i16 (I64)
    // skip 4 unless inst_predicate_6
    0x5006,
    // --> [RexOp2urm_noflags#4b6]
    // --> [RexOp2urm_noflags#4b6]
    // --> [RexOp2urm_noflags#4b6]
    // --> [RexOp2urm_noflags#4b6]
    0x0022, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6]
    // --> [Op2urm_noflags_abcd#4b6]
    // --> [Op2urm_noflags_abcd#4b6]
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // stop unless inst_predicate_7
    // stop unless inst_predicate_7
    // stop unless inst_predicate_7
    // stop unless inst_predicate_7
    0x1007,
    // --> [RexOp2urm_noflags#4b6]
    // --> [RexOp2urm_noflags#4b6]
    // --> [RexOp2urm_noflags#4b6]
    // --> [RexOp2urm_noflags#4b6]
    0x0022, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6] and stop
    // --> [Op2urm_noflags_abcd#4b6] and stop
    // --> [Op2urm_noflags_abcd#4b6] and stop
    // --> [Op2urm_noflags_abcd#4b6] and stop
    0x0021, 0x04b6,
    // end of bint.i16 (I64)
    // end of bint.i8 (I64)
    // end of bint.i32 (I64)
    // end of bint.i64 (I64)
    // 000012: bitcast.i64 (I64)
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexMp2rfumr#857e] and stop
    0x00d7, 0x857e,
    // end of bitcast.i64 (I64)
    // 000015: bnot.i64 (I64)
    // --> [DynRexOp1ur#a0f7] and stop
    // 000015: bnot.b64 (I64)
    // --> [DynRexOp1ur#a0f7] and stop
    0x0157, 0xa0f7,
    // end of bnot.b64 (I64)
    // end of bnot.i64 (I64)
    // 000017: bor.i64 (I64)
    // --> [DynRexOp1rr#8009] and stop
    // 000017: bor.b64 (I64)
    // --> [DynRexOp1rr#8009] and stop
    0x014b, 0x8009,
    // end of bor.b64 (I64)
    // end of bor.i64 (I64)
    // 000019: bor_imm.i64 (I64)
    // --> [DynRexOp1r_ib#9083]
    0x0152, 0x9083,
    // --> [DynRexOp1r_id#9081] and stop
    0x0155, 0x9081,
    // end of bor_imm.i64 (I64)
    // 00001d: brnz.i64 (I64)
    // --> [RexOp1tjccb#8075]
    0x0202, 0x8075,
    // --> [RexOp1tjccd#8085] and stop
    0x0207, 0x8085,
    // end of brnz.i64 (I64)
    // 000021: brz.i64 (I64)
    // --> [RexOp1tjccb#8074]
    0x0202, 0x8074,
    // --> [RexOp1tjccd#8084] and stop
    0x0207, 0x8084,
    // end of brz.i64 (I64)
    // 000025: bxor.i64 (I64)
    // --> [DynRexOp1rr#8031] and stop
    // 000025: bxor.b64 (I64)
    // --> [DynRexOp1rr#8031] and stop
    0x014b, 0x8031,
    // end of bxor.b64 (I64)
    // end of bxor.i64 (I64)
    // 000027: bxor_imm.i64 (I64)
    // --> [DynRexOp1r_ib#e083]
    0x0152, 0xe083,
    // --> [DynRexOp1r_id#e081] and stop
    0x0155, 0xe081,
    // end of bxor_imm.i64 (I64)
    // 00002b: call_indirect.i64 (I64)
    // --> [RexOp1call_r#20ff]
    0x01e8, 0x20ff,
    // --> [Op1call_r#20ff] and stop
    // 00002d: call_indirect.i32 (I32)
    // --> [Op1call_r#20ff] and stop
    0x01e7, 0x20ff,
    // end of call_indirect.i32 (I32)
    // end of call_indirect.i64 (I64)
    // 00002f: clz.i64 (I64)
    // stop unless PredicateView(14)
    0x102d,
    // --> [RexMp2urm#86bd] and stop
    0x0171, 0x86bd,
    // end of clz.i64 (I64)
    // 000032: copy.i64 (I64)
    // --> [DynRexOp1umr#8089] and stop
    0x0005, 0x8089,
    // end of copy.i64 (I64)
    // 000034: copy_nop.i64 (I64)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.i32 (I64)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.i8 (I64)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.i16 (I64)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.f64 (I64)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.f32 (I64)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.b8x16 (I64)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.b16x8 (I64)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.b32x4 (I64)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.b64x2 (I64)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.i8x16 (I64)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.i16x8 (I64)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.i32x4 (I64)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.i64x2 (I64)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.f32x4 (I64)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.f64x2 (I64)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.i32 (I32)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.i8 (I32)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.i16 (I32)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.f64 (I32)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.f32 (I32)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.i64 (I32)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.b8x16 (I32)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.b16x8 (I32)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.b32x4 (I32)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.b64x2 (I32)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.i8x16 (I32)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.i16x8 (I32)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.i32x4 (I32)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.i64x2 (I32)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.f32x4 (I32)
    // --> [stacknull#00] and stop
    // 000034: copy_nop.f64x2 (I32)
    // --> [stacknull#00] and stop
    0x00c3, 0x0000,
    // end of copy_nop.f64x2 (I32)
    // end of copy_nop.f32x4 (I32)
    // end of copy_nop.i64x2 (I32)
    // end of copy_nop.i32x4 (I32)
    // end of copy_nop.i16x8 (I32)
    // end of copy_nop.i8x16 (I32)
    // end of copy_nop.b64x2 (I32)
    // end of copy_nop.b32x4 (I32)
    // end of copy_nop.b16x8 (I32)
    // end of copy_nop.b8x16 (I32)
    // end of copy_nop.i64 (I32)
    // end of copy_nop.f32 (I32)
    // end of copy_nop.f64 (I32)
    // end of copy_nop.i16 (I32)
    // end of copy_nop.i8 (I32)
    // end of copy_nop.i32 (I32)
    // end of copy_nop.f64x2 (I64)
    // end of copy_nop.f32x4 (I64)
    // end of copy_nop.i64x2 (I64)
    // end of copy_nop.i32x4 (I64)
    // end of copy_nop.i16x8 (I64)
    // end of copy_nop.i8x16 (I64)
    // end of copy_nop.b64x2 (I64)
    // end of copy_nop.b32x4 (I64)
    // end of copy_nop.b16x8 (I64)
    // end of copy_nop.b8x16 (I64)
    // end of copy_nop.f32 (I64)
    // end of copy_nop.f64 (I64)
    // end of copy_nop.i16 (I64)
    // end of copy_nop.i8 (I64)
    // end of copy_nop.i32 (I64)
    // end of copy_nop.i64 (I64)
    // 000036: copy_to_ssa.i64 (I64)
    // --> [RexOp1umr_reg_to_ssa#8089] and stop
    // 000036: copy_to_ssa.r64 (I64)
    // --> [RexOp1umr_reg_to_ssa#8089] and stop
    0x002f, 0x8089,
    // end of copy_to_ssa.r64 (I64)
    // end of copy_to_ssa.i64 (I64)
    // 000038: ctz.i64 (I64)
    // stop unless PredicateView(13)
    0x102c,
    // --> [RexMp2urm#86bc] and stop
    0x0171, 0x86bc,
    // end of ctz.i64 (I64)
    // 00003b: fill.i64 (I64)
    // --> [RexOp1fillSib32#808b] and stop
    // 00003b: fill.r64 (I64)
    // --> [RexOp1fillSib32#808b] and stop
    0x00b1, 0x808b,
    // end of fill.r64 (I64)
    // end of fill.i64 (I64)
    // 00003d: fill_nop.i64 (I64)
    // --> [fillnull#00] and stop
    // 00003d: fill_nop.i32 (I64)
    // --> [fillnull#00] and stop
    // 00003d: fill_nop.b1 (I64)
    // --> [fillnull#00] and stop
    // 00003d: fill_nop.i8 (I64)
    // --> [fillnull#00] and stop
    // 00003d: fill_nop.i16 (I64)
    // --> [fillnull#00] and stop
    // 00003d: fill_nop.i32 (I32)
    // --> [fillnull#00] and stop
    // 00003d: fill_nop.b1 (I32)
    // --> [fillnull#00] and stop
    // 00003d: fill_nop.i8 (I32)
    // --> [fillnull#00] and stop
    // 00003d: fill_nop.i16 (I32)
    // --> [fillnull#00] and stop
    // 00003d: fill_nop.i64 (I32)
    // --> [fillnull#00] and stop
    0x00b7, 0x0000,
    // end of fill_nop.i64 (I32)
    // end of fill_nop.i16 (I32)
    // end of fill_nop.i8 (I32)
    // end of fill_nop.b1 (I32)
    // end of fill_nop.i32 (I32)
    // end of fill_nop.i16 (I64)
    // end of fill_nop.i8 (I64)
    // end of fill_nop.b1 (I64)
    // end of fill_nop.i32 (I64)
    // end of fill_nop.i64 (I64)
    // 00003f: func_addr.i64 (I64)
    // skip 2 unless PredicateView(11)
    0x302a,
    // --> [RexOp1fnaddr8#80b8]
    0x01cc, 0x80b8,
    // skip 2 unless PredicateView(9)
    0x3028,
    // --> [RexOp1allones_fnaddr8#80b8]
    0x01d0, 0x80b8,
    // skip 2 unless inst_predicate_28
    0x301c,
    // --> [RexOp1pcrel_fnaddr8#808d]
    0x01d2, 0x808d,
    // stop unless PredicateView(10)
    0x1029,
    // --> [RexOp1got_fnaddr8#808b] and stop
    0x01d5, 0x808b,
    // end of func_addr.i64 (I64)
    // 00004b: get_pinned_reg.i64 (I64)
    // --> [get_pinned_reg#00] and stop
    0x0001, 0x0000,
    // end of get_pinned_reg.i64 (I64)
    // 00004d: iadd.i64 (I64)
    // --> [DynRexOp1rr#8001] and stop
    0x014b, 0x8001,
    // end of iadd.i64 (I64)
    // 00004f: iadd_ifcarry.i64 (I64)
    // --> [DynRexOp1rio#8011] and stop
    0x0151, 0x8011,
    // end of iadd_ifcarry.i64 (I64)
    // 000051: iadd_ifcin.i64 (I64)
    // --> [DynRexOp1rin#8011] and stop
    0x014f, 0x8011,
    // end of iadd_ifcin.i64 (I64)
    // 000053: iadd_ifcout.i64 (I64)
    // --> [DynRexOp1rout#8001] and stop
    0x014d, 0x8001,
    // end of iadd_ifcout.i64 (I64)
    // 000055: iadd_imm.i64 (I64)
    // --> [DynRexOp1r_ib#8083]
    0x0152, 0x8083,
    // --> [DynRexOp1r_id#8081] and stop
    0x0155, 0x8081,
    // end of iadd_imm.i64 (I64)
    // 000059: icmp.i64 (I64)
    // --> [DynRexOp1icscc#8039] and stop
    0x0175, 0x8039,
    // end of icmp.i64 (I64)
    // 00005b: icmp_imm.i64 (I64)
    // --> [DynRexOp1icscc_ib#f083]
    0x0176, 0xf083,
    // --> [DynRexOp1icscc_id#f081] and stop
    0x0179, 0xf081,
    // end of icmp_imm.i64 (I64)
    // 00005f: iconst.i64 (I64)
    // skip 4 unless inst_predicate_0
    0x5000,
    // --> [RexOp1pu_id#b8]
    0x0010, 0x00b8,
    // --> [Op1pu_id#b8]
    0x000e, 0x00b8,
    // --> [RexOp1u_id#80c7]
    0x0012, 0x80c7,
    // --> [RexOp1pu_iq#80b8]
    0x0014, 0x80b8,
    // stop unless inst_predicate_1
    // 000068: iconst.i16 (I64)
    // stop unless inst_predicate_1
    0x1001,
    // --> [RexOp1u_id_z#31]
    // --> [RexOp1u_id_z#31]
    0x001c, 0x0031,
    // --> [Op1u_id_z#31] and stop
    // --> [Op1u_id_z#31] and stop
    0x001b, 0x0031,
    // end of iconst.i16 (I64)
    // end of iconst.i64 (I64)
    // 00006d: ifcmp.i64 (I64)
    // --> [DynRexOp1rcmp#8039] and stop
    0x017b, 0x8039,
    // end of ifcmp.i64 (I64)
    // 00006f: ifcmp_imm.i64 (I64)
    // --> [DynRexOp1rcmp_ib#f083]
    0x017c, 0xf083,
    // --> [DynRexOp1rcmp_id#f081] and stop
    0x017f, 0xf081,
    // end of ifcmp_imm.i64 (I64)
    // 000073: ifcmp_sp.i64 (I64)
    // --> [RexOp1rcmp_sp#8039] and stop
    0x0183, 0x8039,
    // end of ifcmp_sp.i64 (I64)
    // 000075: imul.i64 (I64)
    // --> [DynRexOp2rrx#84af] and stop
    0x015d, 0x84af,
    // end of imul.i64 (I64)
    // 000077: indirect_jump_table_br.i64 (I64)
    // --> [RexOp1indirect_jmp#40ff]
    0x021a, 0x40ff,
    // --> [Op1indirect_jmp#40ff] and stop
    // 000079: indirect_jump_table_br.i32 (I32)
    // --> [Op1indirect_jmp#40ff] and stop
    0x021d, 0x40ff,
    // end of indirect_jump_table_br.i32 (I32)
    // end of indirect_jump_table_br.i64 (I64)
    // 00007b: ishl.i64 (I64)
    // --> [RexOp1rc#c0d3] and stop
    0x016d, 0xc0d3,
    // end of ishl.i64 (I64)
    // 00007d: ishl_imm.i64 (I64)
    // --> [DynRexOp1r_ib#c0c1] and stop
    0x0153, 0xc0c1,
    // end of ishl_imm.i64 (I64)
    // 00007f: istore16.i64 (I64)
    // --> [RexMp1st#189]
    // 00007f: istore16.i32 (I64)
    // --> [RexMp1st#189]
    0x0076, 0x0189,
    // --> [Mp1st#189]
    // --> [Mp1st#189]
    0x0074, 0x0189,
    // --> [RexMp1stDisp8#189]
    // --> [RexMp1stDisp8#189]
    0x007e, 0x0189,
    // --> [Mp1stDisp8#189]
    // --> [Mp1stDisp8#189]
    0x007c, 0x0189,
    // --> [RexMp1stDisp32#189]
    // --> [RexMp1stDisp32#189]
    0x0086, 0x0189,
    // --> [Mp1stDisp32#189] and stop
    // --> [Mp1stDisp32#189] and stop
    0x0085, 0x0189,
    // end of istore16.i32 (I64)
    // end of istore16.i64 (I64)
    // 00008b: istore16_complex.i64 (I64)
    // stop unless inst_predicate_9
    // 00008b: istore16_complex.i32 (I64)
    // stop unless inst_predicate_9
    0x1009,
    // --> [RexMp1stWithIndex#189]
    // --> [RexMp1stWithIndex#189]
    0x0052, 0x0189,
    // --> [Mp1stWithIndex#189]
    // --> [Mp1stWithIndex#189]
    0x0050, 0x0189,
    // --> [RexMp1stWithIndexDisp8#189]
    // --> [RexMp1stWithIndexDisp8#189]
    0x005a, 0x0189,
    // --> [Mp1stWithIndexDisp8#189]
    // --> [Mp1stWithIndexDisp8#189]
    0x0058, 0x0189,
    // --> [RexMp1stWithIndexDisp32#189]
    // --> [RexMp1stWithIndexDisp32#189]
    0x0062, 0x0189,
    // --> [Mp1stWithIndexDisp32#189] and stop
    // --> [Mp1stWithIndexDisp32#189] and stop
    0x0061, 0x0189,
    // end of istore16_complex.i32 (I64)
    // end of istore16_complex.i64 (I64)
    // 000098: istore32.i64 (I64)
    // --> [RexOp1st#89]
    // 000098: store.i32 (I64)
    // --> [RexOp1st#89]
    // 000098: store.r32 (I64)
    // --> [RexOp1st#89]
    0x0072, 0x0089,
    // --> [Op1st#89]
    // --> [Op1st#89]
    // --> [Op1st#89]
    0x0070, 0x0089,
    // --> [RexOp1stDisp8#89]
    // --> [RexOp1stDisp8#89]
    // --> [RexOp1stDisp8#89]
    0x007a, 0x0089,
    // --> [Op1stDisp8#89]
    // --> [Op1stDisp8#89]
    // --> [Op1stDisp8#89]
    0x0078, 0x0089,
    // --> [RexOp1stDisp32#89]
    // --> [RexOp1stDisp32#89]
    // --> [RexOp1stDisp32#89]
    0x0082, 0x0089,
    // --> [Op1stDisp32#89] and stop
    // --> [Op1stDisp32#89] and stop
    // --> [Op1stDisp32#89] and stop
    0x0081, 0x0089,
    // end of store.r32 (I64)
    // end of store.i32 (I64)
    // end of istore32.i64 (I64)
    // 0000a4: istore8.i64 (I64)
    // --> [RexOp1st#88]
    // 0000a4: istore8.i32 (I64)
    // --> [RexOp1st#88]
    0x0072, 0x0088,
    // --> [Op1st_abcd#88]
    // --> [Op1st_abcd#88]
    0x0088, 0x0088,
    // --> [RexOp1stDisp8#88]
    // --> [RexOp1stDisp8#88]
    0x007a, 0x0088,
    // --> [Op1stDisp8_abcd#88]
    // --> [Op1stDisp8_abcd#88]
    0x008a, 0x0088,
    // --> [RexOp1stDisp32#88]
    // --> [RexOp1stDisp32#88]
    0x0082, 0x0088,
    // --> [Op1stDisp32_abcd#88] and stop
    // --> [Op1stDisp32_abcd#88] and stop
    0x008d, 0x0088,
    // end of istore8.i32 (I64)
    // end of istore8.i64 (I64)
    // 0000b0: istore8_complex.i64 (I64)
    // stop unless inst_predicate_9
    // 0000b0: istore8_complex.i32 (I64)
    // stop unless inst_predicate_9
    0x1009,
    // --> [RexOp1stWithIndex_abcd#88]
    // --> [RexOp1stWithIndex_abcd#88]
    0x0066, 0x0088,
    // --> [Op1stWithIndex_abcd#88]
    // --> [Op1stWithIndex_abcd#88]
    0x0064, 0x0088,
    // --> [RexOp1stWithIndexDisp8_abcd#88]
    // --> [RexOp1stWithIndexDisp8_abcd#88]
    0x006a, 0x0088,
    // --> [Op1stWithIndexDisp8_abcd#88]
    // --> [Op1stWithIndexDisp8_abcd#88]
    0x0068, 0x0088,
    // --> [RexOp1stWithIndexDisp32_abcd#88]
    // --> [RexOp1stWithIndexDisp32_abcd#88]
    0x006e, 0x0088,
    // --> [Op1stWithIndexDisp32_abcd#88] and stop
    // --> [Op1stWithIndexDisp32_abcd#88] and stop
    0x006d, 0x0088,
    // end of istore8_complex.i32 (I64)
    // end of istore8_complex.i64 (I64)
    // 0000bd: isub.i64 (I64)
    // --> [DynRexOp1rr#8029] and stop
    0x014b, 0x8029,
    // end of isub.i64 (I64)
    // 0000bf: isub_ifbin.i64 (I64)
    // --> [DynRexOp1rin#8019] and stop
    0x014f, 0x8019,
    // end of isub_ifbin.i64 (I64)
    // 0000c1: isub_ifborrow.i64 (I64)
    // --> [DynRexOp1rio#8019] and stop
    0x0151, 0x8019,
    // end of isub_ifborrow.i64 (I64)
    // 0000c3: isub_ifbout.i64 (I64)
    // --> [DynRexOp1rout#8029] and stop
    0x014d, 0x8029,
    // end of isub_ifbout.i64 (I64)
    // 0000c5: jump_table_base.i64 (I64)
    // --> [RexOp1jt_base#808d] and stop
    0x0217, 0x808d,
    // end of jump_table_base.i64 (I64)
    // 0000c7: jump_table_entry.i64 (I64)
    // --> [RexOp1jt_entry#8063] and stop
    0x0213, 0x8063,
    // end of jump_table_entry.i64 (I64)
    // 0000c9: load.i64 (I64)
    // --> [RexOp1ld#808b]
    // 0000c9: load.r64 (I64)
    // --> [RexOp1ld#808b]
    0x0098, 0x808b,
    // --> [RexOp1ldDisp8#808b]
    // --> [RexOp1ldDisp8#808b]
    0x00a0, 0x808b,
    // --> [RexOp1ldDisp32#808b] and stop
    // --> [RexOp1ldDisp32#808b] and stop
    0x00a9, 0x808b,
    // end of load.r64 (I64)
    // end of load.i64 (I64)
    // 0000cf: load_complex.i64 (I64)
    // stop unless inst_predicate_8
    0x1008,
    // --> [RexOp1ldWithIndex#808b]
    0x0036, 0x808b,
    // --> [RexOp1ldWithIndexDisp8#808b]
    0x003e, 0x808b,
    // --> [RexOp1ldWithIndexDisp32#808b] and stop
    0x0047, 0x808b,
    // end of load_complex.i64 (I64)
    // 0000d6: popcnt.i64 (I64)
    // stop unless PredicateView(15)
    0x102e,
    // --> [RexMp2urm#86b8] and stop
    0x0171, 0x86b8,
    // end of popcnt.i64 (I64)
    // 0000d9: regfill.i64 (I64)
    // --> [RexOp1regfill32#808b] and stop
    // 0000d9: regfill.r64 (I64)
    // --> [RexOp1regfill32#808b] and stop
    0x00b5, 0x808b,
    // end of regfill.r64 (I64)
    // end of regfill.i64 (I64)
    // 0000db: regmove.i64 (I64)
    // --> [RexOp1rmov#8089] and stop
    // 0000db: regmove.r64 (I64)
    // --> [RexOp1rmov#8089] and stop
    0x000d, 0x8089,
    // end of regmove.r64 (I64)
    // end of regmove.i64 (I64)
    // 0000dd: regspill.i64 (I64)
    // --> [RexOp1regspill32#8089] and stop
    // 0000dd: regspill.r64 (I64)
    // --> [RexOp1regspill32#8089] and stop
    0x0095, 0x8089,
    // end of regspill.r64 (I64)
    // end of regspill.i64 (I64)
    // 0000df: rotl.i64 (I64)
    // --> [RexOp1rc#80d3] and stop
    0x016d, 0x80d3,
    // end of rotl.i64 (I64)
    // 0000e1: rotl_imm.i64 (I64)
    // --> [DynRexOp1r_ib#80c1] and stop
    0x0153, 0x80c1,
    // end of rotl_imm.i64 (I64)
    // 0000e3: rotr.i64 (I64)
    // --> [RexOp1rc#90d3] and stop
    0x016d, 0x90d3,
    // end of rotr.i64 (I64)
    // 0000e5: rotr_imm.i64 (I64)
    // --> [DynRexOp1r_ib#90c1] and stop
    0x0153, 0x90c1,
    // end of rotr_imm.i64 (I64)
    // 0000e7: selectif.i64 (I64)
    // --> [DynRexOp2cmov#8440] and stop
    0x018d, 0x8440,
    // end of selectif.i64 (I64)
    // 0000e9: set_pinned_reg.i64 (I64)
    // --> [RexOp1set_pinned_reg#8089]
    0x0002, 0x8089,
    // --> [RexOp1set_pinned_reg#8089] and stop
    0x0003, 0x8089,
    // end of set_pinned_reg.i64 (I64)
    // 0000ed: sextend.i64 (I64)
    // skip 2 unless inst_predicate_5
    0x3005,
    // --> [RexOp2urm_noflags#84be]
    0x0022, 0x84be,
    // skip 2 unless inst_predicate_2
    0x3002,
    // --> [RexOp2urm_noflags#84bf]
    0x0022, 0x84bf,
    // stop unless inst_predicate_3
    0x1003,
    // --> [RexOp1urm_noflags#8063] and stop
    0x0027, 0x8063,
    // end of sextend.i64 (I64)
    // 0000f6: sload16.i64 (I64)
    // --> [RexOp2ld#84bf]
    0x009c, 0x84bf,
    // --> [RexOp2ldDisp8#84bf]
    0x00a4, 0x84bf,
    // --> [RexOp2ldDisp32#84bf] and stop
    0x00ad, 0x84bf,
    // end of sload16.i64 (I64)
    // 0000fc: sload16_complex.i64 (I64)
    // stop unless inst_predicate_8
    0x1008,
    // --> [RexOp2ldWithIndex#84bf]
    0x003a, 0x84bf,
    // --> [RexOp2ldWithIndexDisp8#84bf]
    0x0042, 0x84bf,
    // --> [RexOp2ldWithIndexDisp32#84bf] and stop
    0x004b, 0x84bf,
    // end of sload16_complex.i64 (I64)
    // 000103: sload32.i64 (I64)
    // --> [RexOp1ld#8063]
    0x0098, 0x8063,
    // --> [RexOp1ldDisp8#8063]
    0x00a0, 0x8063,
    // --> [RexOp1ldDisp32#8063] and stop
    0x00a9, 0x8063,
    // end of sload32.i64 (I64)
    // 000109: sload8.i64 (I64)
    // --> [RexOp2ld#84be]
    0x009c, 0x84be,
    // --> [RexOp2ldDisp8#84be]
    0x00a4, 0x84be,
    // --> [RexOp2ldDisp32#84be] and stop
    0x00ad, 0x84be,
    // end of sload8.i64 (I64)
    // 00010f: sload8_complex.i64 (I64)
    // stop unless inst_predicate_8
    0x1008,
    // --> [RexOp2ldWithIndex#84be]
    0x003a, 0x84be,
    // --> [RexOp2ldWithIndexDisp8#84be]
    0x0042, 0x84be,
    // --> [RexOp2ldWithIndexDisp32#84be] and stop
    0x004b, 0x84be,
    // end of sload8_complex.i64 (I64)
    // 000116: spill.i64 (I64)
    // --> [RexOp1spillSib32#8089] and stop
    // 000116: spill.r64 (I64)
    // --> [RexOp1spillSib32#8089] and stop
    0x0091, 0x8089,
    // end of spill.r64 (I64)
    // end of spill.i64 (I64)
    // 000118: sshr.i64 (I64)
    // --> [RexOp1rc#f0d3] and stop
    0x016d, 0xf0d3,
    // end of sshr.i64 (I64)
    // 00011a: sshr_imm.i64 (I64)
    // --> [DynRexOp1r_ib#f0c1] and stop
    0x0153, 0xf0c1,
    // end of sshr_imm.i64 (I64)
    // 00011c: stack_addr.i64 (I64)
    // --> [RexOp1spaddr8_id#808d] and stop
    0x01e1, 0x808d,
    // end of stack_addr.i64 (I64)
    // 00011e: store.i64 (I64)
    // --> [RexOp1st#8089]
    // 00011e: store.r64 (I64)
    // --> [RexOp1st#8089]
    0x0072, 0x8089,
    // --> [RexOp1stDisp8#8089]
    // --> [RexOp1stDisp8#8089]
    0x007a, 0x8089,
    // --> [RexOp1stDisp32#8089] and stop
    // --> [RexOp1stDisp32#8089] and stop
    0x0083, 0x8089,
    // end of store.r64 (I64)
    // end of store.i64 (I64)
    // 000124: store_complex.i64 (I64)
    // stop unless inst_predicate_9
    0x1009,
    // --> [RexOp1stWithIndex#8089]
    0x004e, 0x8089,
    // --> [RexOp1stWithIndexDisp8#8089]
    0x0056, 0x8089,
    // --> [RexOp1stWithIndexDisp32#8089] and stop
    0x005f, 0x8089,
    // end of store_complex.i64 (I64)
    // 00012b: symbol_value.i64 (I64)
    // skip 2 unless PredicateView(12)
    0x302b,
    // --> [RexOp1gvaddr8#80b8]
    0x01d8, 0x80b8,
    // skip 3 unless PredicateView(10)
    0x4029,
    // skip 2 unless inst_predicate_29
    0x301d,
    // --> [RexOp1pcrel_gvaddr8#808d]
    0x01da, 0x808d,
    // stop unless PredicateView(10)
    0x1029,
    // --> [RexOp1got_gvaddr8#808b] and stop
    0x01dd, 0x808b,
    // end of symbol_value.i64 (I64)
    // 000135: uextend.i64 (I64)
    // skip 4 unless inst_predicate_5
    0x5005,
    // --> [RexOp2urm_noflags#4b6]
    0x0022, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // skip 4 unless inst_predicate_2
    0x5002,
    // --> [RexOp2urm_noflags#4b7]
    0x0022, 0x04b7,
    // --> [Op2urm_noflags#4b7]
    0x0024, 0x04b7,
    // stop unless inst_predicate_3
    0x1003,
    // --> [RexOp1umr#89]
    // 000140: copy.b1 (I64)
    // --> [RexOp1umr#89]
    // 000140: copy.i8 (I64)
    // --> [RexOp1umr#89]
    // 000140: copy.i16 (I64)
    // --> [RexOp1umr#89]
    0x0008, 0x0089,
    // --> [Op1umr#89] and stop
    // --> [Op1umr#89] and stop
    // --> [Op1umr#89] and stop
    // --> [Op1umr#89] and stop
    // 000142: copy.r32 (I32)
    // --> [Op1umr#89] and stop
    // 000142: copy.b1 (I32)
    // --> [Op1umr#89] and stop
    // 000142: copy.i8 (I32)
    // --> [Op1umr#89] and stop
    // 000142: copy.i16 (I32)
    // --> [Op1umr#89] and stop
    0x0007, 0x0089,
    // end of copy.i16 (I32)
    // end of copy.i8 (I32)
    // end of copy.b1 (I32)
    // end of copy.r32 (I32)
    // end of copy.i16 (I64)
    // end of copy.i8 (I64)
    // end of copy.b1 (I64)
    // end of uextend.i64 (I64)
    // 000144: uload16.i64 (I64)
    // --> [RexOp2ld#84b7]
    0x009c, 0x84b7,
    // --> [RexOp2ldDisp8#84b7]
    0x00a4, 0x84b7,
    // --> [RexOp2ldDisp32#84b7] and stop
    0x00ad, 0x84b7,
    // end of uload16.i64 (I64)
    // 00014a: uload16_complex.i64 (I64)
    // stop unless inst_predicate_8
    0x1008,
    // --> [RexOp2ldWithIndex#84b7]
    0x003a, 0x84b7,
    // --> [RexOp2ldWithIndexDisp8#84b7]
    0x0042, 0x84b7,
    // --> [RexOp2ldWithIndexDisp32#84b7] and stop
    0x004b, 0x84b7,
    // end of uload16_complex.i64 (I64)
    // 000151: uload32.i64 (I64)
    // --> [RexOp1ld#8b]
    // 000151: load.i32 (I64)
    // --> [RexOp1ld#8b]
    // 000151: load.r32 (I64)
    // --> [RexOp1ld#8b]
    0x0098, 0x008b,
    // --> [Op1ld#8b]
    // --> [Op1ld#8b]
    // --> [Op1ld#8b]
    0x0096, 0x008b,
    // --> [RexOp1ldDisp8#8b]
    // --> [RexOp1ldDisp8#8b]
    // --> [RexOp1ldDisp8#8b]
    0x00a0, 0x008b,
    // --> [Op1ldDisp8#8b]
    // --> [Op1ldDisp8#8b]
    // --> [Op1ldDisp8#8b]
    0x009e, 0x008b,
    // --> [RexOp1ldDisp32#8b]
    // --> [RexOp1ldDisp32#8b]
    // --> [RexOp1ldDisp32#8b]
    0x00a8, 0x008b,
    // --> [Op1ldDisp32#8b] and stop
    // --> [Op1ldDisp32#8b] and stop
    // --> [Op1ldDisp32#8b] and stop
    0x00a7, 0x008b,
    // end of load.r32 (I64)
    // end of load.i32 (I64)
    // end of uload32.i64 (I64)
    // 00015d: uload8.i64 (I64)
    // --> [RexOp2ld#84b6]
    0x009c, 0x84b6,
    // --> [RexOp2ldDisp8#84b6]
    0x00a4, 0x84b6,
    // --> [RexOp2ldDisp32#84b6] and stop
    0x00ad, 0x84b6,
    // end of uload8.i64 (I64)
    // 000163: uload8_complex.i64 (I64)
    // stop unless inst_predicate_8
    0x1008,
    // --> [RexOp2ldWithIndex#84b6]
    0x003a, 0x84b6,
    // --> [RexOp2ldWithIndexDisp8#84b6]
    0x0042, 0x84b6,
    // --> [RexOp2ldWithIndexDisp32#84b6] and stop
    0x004b, 0x84b6,
    // end of uload8_complex.i64 (I64)
    // 00016a: ushr.i64 (I64)
    // --> [RexOp1rc#d0d3] and stop
    0x016d, 0xd0d3,
    // end of ushr.i64 (I64)
    // 00016c: ushr_imm.i64 (I64)
    // --> [DynRexOp1r_ib#d0c1] and stop
    0x0153, 0xd0c1,
    // end of ushr_imm.i64 (I64)
    // 00016e: x86_bsf.i64 (I64)
    // --> [DynRexOp2bsf_and_bsr#84bc] and stop
    0x0173, 0x84bc,
    // end of x86_bsf.i64 (I64)
    // 000170: x86_bsr.i64 (I64)
    // --> [DynRexOp2bsf_and_bsr#84bd] and stop
    0x0173, 0x84bd,
    // end of x86_bsr.i64 (I64)
    // 000172: x86_cvtt2si.i64 (I64)
    // skip 2 unless inst_predicate_10
    0x300a,
    // --> [RexMp2rfurm#862c]
    0x0130, 0x862c,
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexMp2rfurm#872c] and stop
    0x0131, 0x872c,
    // end of x86_cvtt2si.i64 (I64)
    // 000178: x86_pop.i64 (I64)
    // --> [RexOp1popq#58]
    0x00c0, 0x0058,
    // --> [Op1popq#58] and stop
    // 00017a: x86_pop.i32 (I32)
    // --> [Op1popq#58] and stop
    0x00bf, 0x0058,
    // end of x86_pop.i32 (I32)
    // end of x86_pop.i64 (I64)
    // 00017c: x86_push.i64 (I64)
    // --> [RexOp1pushq#50]
    0x00bc, 0x0050,
    // --> [Op1pushq#50] and stop
    // 00017e: x86_push.i32 (I32)
    // --> [Op1pushq#50] and stop
    0x00bb, 0x0050,
    // end of x86_push.i32 (I32)
    // end of x86_push.i64 (I64)
    // 000180: x86_sdivmodx.i64 (I64)
    // --> [DynRexOp1div#f0f7] and stop
    0x015f, 0xf0f7,
    // end of x86_sdivmodx.i64 (I64)
    // 000182: x86_smulx.i64 (I64)
    // --> [DynRexOp1mulx#d0f7] and stop
    0x0161, 0xd0f7,
    // end of x86_smulx.i64 (I64)
    // 000184: x86_udivmodx.i64 (I64)
    // --> [DynRexOp1div#e0f7] and stop
    0x015f, 0xe0f7,
    // end of x86_udivmodx.i64 (I64)
    // 000186: x86_umulx.i64 (I64)
    // --> [DynRexOp1mulx#c0f7] and stop
    0x0161, 0xc0f7,
    // end of x86_umulx.i64 (I64)
    // 000188: band.i32 (I64)
    // --> [DynRexOp1rr#21] and stop
    // 000188: band.b32 (I64)
    // --> [DynRexOp1rr#21] and stop
    // 000188: band.i32 (I32)
    // --> [DynRexOp1rr#21] and stop
    // 000188: band.b32 (I32)
    // --> [DynRexOp1rr#21] and stop
    0x014b, 0x0021,
    // end of band.b32 (I32)
    // end of band.i32 (I32)
    // end of band.b32 (I64)
    // end of band.i32 (I64)
    // 00018a: band_imm.i32 (I64)
    // --> [DynRexOp1r_ib#4083]
    // 00018a: band_imm.i32 (I32)
    // --> [DynRexOp1r_ib#4083]
    0x0152, 0x4083,
    // --> [DynRexOp1r_id#4081] and stop
    // --> [DynRexOp1r_id#4081] and stop
    0x0155, 0x4081,
    // end of band_imm.i32 (I32)
    // end of band_imm.i32 (I64)
    // 00018e: bitcast.i32 (I64)
    // stop unless inst_predicate_10
    0x100a,
    // --> [RexMp2rfumr#57e]
    0x00d6, 0x057e,
    // --> [Mp2rfumr#57e] and stop
    0x00d5, 0x057e,
    // end of bitcast.i32 (I64)
    // 000193: bnot.i32 (I64)
    // --> [DynRexOp1ur#20f7] and stop
    // 000193: bnot.b32 (I64)
    // --> [DynRexOp1ur#20f7] and stop
    // 000193: bnot.i32 (I32)
    // --> [DynRexOp1ur#20f7] and stop
    // 000193: bnot.b32 (I32)
    // --> [DynRexOp1ur#20f7] and stop
    0x0157, 0x20f7,
    // end of bnot.b32 (I32)
    // end of bnot.i32 (I32)
    // end of bnot.b32 (I64)
    // end of bnot.i32 (I64)
    // 000195: bor.i32 (I64)
    // --> [DynRexOp1rr#09] and stop
    // 000195: bor.b32 (I64)
    // --> [DynRexOp1rr#09] and stop
    // 000195: bor.i32 (I32)
    // --> [DynRexOp1rr#09] and stop
    // 000195: bor.b32 (I32)
    // --> [DynRexOp1rr#09] and stop
    0x014b, 0x0009,
    // end of bor.b32 (I32)
    // end of bor.i32 (I32)
    // end of bor.b32 (I64)
    // end of bor.i32 (I64)
    // 000197: bor_imm.i32 (I64)
    // --> [DynRexOp1r_ib#1083]
    // 000197: bor_imm.i32 (I32)
    // --> [DynRexOp1r_ib#1083]
    0x0152, 0x1083,
    // --> [DynRexOp1r_id#1081] and stop
    // --> [DynRexOp1r_id#1081] and stop
    0x0155, 0x1081,
    // end of bor_imm.i32 (I32)
    // end of bor_imm.i32 (I64)
    // 00019b: brnz.i32 (I64)
    // --> [RexOp1tjccb#75]
    0x0202, 0x0075,
    // --> [Op1tjccb#75]
    0x0200, 0x0075,
    // --> [RexOp1tjccd#85]
    0x0206, 0x0085,
    // --> [Op1tjccd#85] and stop
    0x0205, 0x0085,
    // end of brnz.i32 (I64)
    // 0001a3: brz.i32 (I64)
    // --> [RexOp1tjccb#74]
    0x0202, 0x0074,
    // --> [Op1tjccb#74]
    0x0200, 0x0074,
    // --> [RexOp1tjccd#84]
    0x0206, 0x0084,
    // --> [Op1tjccd#84] and stop
    0x0205, 0x0084,
    // end of brz.i32 (I64)
    // 0001ab: bxor.i32 (I64)
    // --> [DynRexOp1rr#31] and stop
    // 0001ab: bxor.b32 (I64)
    // --> [DynRexOp1rr#31] and stop
    // 0001ab: bxor.i32 (I32)
    // --> [DynRexOp1rr#31] and stop
    // 0001ab: bxor.b32 (I32)
    // --> [DynRexOp1rr#31] and stop
    0x014b, 0x0031,
    // end of bxor.b32 (I32)
    // end of bxor.i32 (I32)
    // end of bxor.b32 (I64)
    // end of bxor.i32 (I64)
    // 0001ad: bxor_imm.i32 (I64)
    // --> [DynRexOp1r_ib#6083]
    // 0001ad: bxor_imm.i32 (I32)
    // --> [DynRexOp1r_ib#6083]
    0x0152, 0x6083,
    // --> [DynRexOp1r_id#6081] and stop
    // --> [DynRexOp1r_id#6081] and stop
    0x0155, 0x6081,
    // end of bxor_imm.i32 (I32)
    // end of bxor_imm.i32 (I64)
    // 0001b1: clz.i32 (I64)
    // stop unless PredicateView(14)
    0x102d,
    // --> [RexMp2urm#6bd]
    0x0170, 0x06bd,
    // --> [Mp2urm#6bd] and stop
    0x016f, 0x06bd,
    // end of clz.i32 (I64)
    // 0001b6: copy.i32 (I64)
    // --> [DynRexOp1umr#89] and stop
    // 0001b6: copy.i32 (I32)
    // --> [DynRexOp1umr#89] and stop
    0x0005, 0x0089,
    // end of copy.i32 (I32)
    // end of copy.i32 (I64)
    // 0001b8: copy_to_ssa.i32 (I64)
    // --> [RexOp1umr_reg_to_ssa#89] and stop
    // 0001b8: copy_to_ssa.b1 (I64)
    // --> [RexOp1umr_reg_to_ssa#89] and stop
    // 0001b8: copy_to_ssa.i8 (I64)
    // --> [RexOp1umr_reg_to_ssa#89] and stop
    // 0001b8: copy_to_ssa.i16 (I64)
    // --> [RexOp1umr_reg_to_ssa#89] and stop
    0x002f, 0x0089,
    // end of copy_to_ssa.i16 (I64)
    // end of copy_to_ssa.i8 (I64)
    // end of copy_to_ssa.b1 (I64)
    // end of copy_to_ssa.i32 (I64)
    // 0001ba: ctz.i32 (I64)
    // stop unless PredicateView(13)
    0x102c,
    // --> [RexMp2urm#6bc]
    0x0170, 0x06bc,
    // --> [Mp2urm#6bc] and stop
    0x016f, 0x06bc,
    // end of ctz.i32 (I64)
    // 0001bf: fill.i32 (I64)
    // --> [RexOp1fillSib32#8b]
    // 0001bf: fill.b1 (I64)
    // --> [RexOp1fillSib32#8b]
    // 0001bf: fill.i8 (I64)
    // --> [RexOp1fillSib32#8b]
    // 0001bf: fill.i16 (I64)
    // --> [RexOp1fillSib32#8b]
    0x00b0, 0x008b,
    // --> [Op1fillSib32#8b] and stop
    // --> [Op1fillSib32#8b] and stop
    // --> [Op1fillSib32#8b] and stop
    // --> [Op1fillSib32#8b] and stop
    // 0001c1: fill.i32 (I32)
    // --> [Op1fillSib32#8b] and stop
    // 0001c1: fill.r32 (I32)
    // --> [Op1fillSib32#8b] and stop
    // 0001c1: fill.b1 (I32)
    // --> [Op1fillSib32#8b] and stop
    // 0001c1: fill.i8 (I32)
    // --> [Op1fillSib32#8b] and stop
    // 0001c1: fill.i16 (I32)
    // --> [Op1fillSib32#8b] and stop
    0x00af, 0x008b,
    // end of fill.i16 (I32)
    // end of fill.i8 (I32)
    // end of fill.b1 (I32)
    // end of fill.r32 (I32)
    // end of fill.i32 (I32)
    // end of fill.i16 (I64)
    // end of fill.i8 (I64)
    // end of fill.b1 (I64)
    // end of fill.i32 (I64)
    // 0001c3: iadd.i32 (I64)
    // --> [DynRexOp1rr#01] and stop
    // 0001c3: iadd.i32 (I32)
    // --> [DynRexOp1rr#01] and stop
    0x014b, 0x0001,
    // end of iadd.i32 (I32)
    // end of iadd.i32 (I64)
    // 0001c5: iadd_ifcarry.i32 (I64)
    // --> [DynRexOp1rio#11] and stop
    // 0001c5: iadd_ifcarry.i32 (I32)
    // --> [DynRexOp1rio#11] and stop
    0x0151, 0x0011,
    // end of iadd_ifcarry.i32 (I32)
    // end of iadd_ifcarry.i32 (I64)
    // 0001c7: iadd_ifcin.i32 (I64)
    // --> [DynRexOp1rin#11] and stop
    // 0001c7: iadd_ifcin.i32 (I32)
    // --> [DynRexOp1rin#11] and stop
    0x014f, 0x0011,
    // end of iadd_ifcin.i32 (I32)
    // end of iadd_ifcin.i32 (I64)
    // 0001c9: iadd_ifcout.i32 (I64)
    // --> [DynRexOp1rout#01] and stop
    // 0001c9: iadd_ifcout.i32 (I32)
    // --> [DynRexOp1rout#01] and stop
    0x014d, 0x0001,
    // end of iadd_ifcout.i32 (I32)
    // end of iadd_ifcout.i32 (I64)
    // 0001cb: iadd_imm.i32 (I64)
    // --> [DynRexOp1r_ib#83]
    // 0001cb: iadd_imm.i32 (I32)
    // --> [DynRexOp1r_ib#83]
    0x0152, 0x0083,
    // --> [DynRexOp1r_id#81] and stop
    // --> [DynRexOp1r_id#81] and stop
    0x0155, 0x0081,
    // end of iadd_imm.i32 (I32)
    // end of iadd_imm.i32 (I64)
    // 0001cf: icmp.i32 (I64)
    // --> [DynRexOp1icscc#39] and stop
    // 0001cf: icmp.i32 (I32)
    // --> [DynRexOp1icscc#39] and stop
    0x0175, 0x0039,
    // end of icmp.i32 (I32)
    // end of icmp.i32 (I64)
    // 0001d1: icmp_imm.i32 (I64)
    // --> [DynRexOp1icscc_ib#7083]
    // 0001d1: icmp_imm.i32 (I32)
    // --> [DynRexOp1icscc_ib#7083]
    0x0176, 0x7083,
    // --> [DynRexOp1icscc_id#7081] and stop
    // --> [DynRexOp1icscc_id#7081] and stop
    0x0179, 0x7081,
    // end of icmp_imm.i32 (I32)
    // end of icmp_imm.i32 (I64)
    // 0001d5: iconst.i32 (I64)
    // --> [RexOp1pu_id#b8]
    0x0010, 0x00b8,
    // --> [Op1pu_id#b8]
    0x000e, 0x00b8,
    // stop unless inst_predicate_1
    0x1001,
    // --> [RexOp1u_id_z#31]
    0x001c, 0x0031,
    // --> [Op1u_id_z#31] and stop
    0x001b, 0x0031,
    // end of iconst.i32 (I64)
    // 0001de: ifcmp.i32 (I64)
    // --> [DynRexOp1rcmp#39] and stop
    // 0001de: ifcmp.i32 (I32)
    // --> [DynRexOp1rcmp#39] and stop
    0x017b, 0x0039,
    // end of ifcmp.i32 (I32)
    // end of ifcmp.i32 (I64)
    // 0001e0: ifcmp_imm.i32 (I64)
    // --> [DynRexOp1rcmp_ib#7083]
    // 0001e0: ifcmp_imm.i32 (I32)
    // --> [DynRexOp1rcmp_ib#7083]
    0x017c, 0x7083,
    // --> [DynRexOp1rcmp_id#7081] and stop
    // --> [DynRexOp1rcmp_id#7081] and stop
    0x017f, 0x7081,
    // end of ifcmp_imm.i32 (I32)
    // end of ifcmp_imm.i32 (I64)
    // 0001e4: imul.i32 (I64)
    // --> [DynRexOp2rrx#4af] and stop
    // 0001e4: imul.i32 (I32)
    // --> [DynRexOp2rrx#4af] and stop
    0x015d, 0x04af,
    // end of imul.i32 (I32)
    // end of imul.i32 (I64)
    // 0001e6: ireduce.i32 (I64)
    // stop unless inst_predicate_4
    0x1004,
    // --> [null#00] and stop
    0x001f, 0x0000,
    // end of ireduce.i32 (I64)
    // 0001e9: ishl.i32 (I64)
    // --> [RexOp1rc#40d3]
    0x016c, 0x40d3,
    // --> [Op1rc#40d3] and stop
    // 0001eb: ishl.i32 (I32)
    // --> [Op1rc#40d3] and stop
    0x016b, 0x40d3,
    // end of ishl.i32 (I32)
    // end of ishl.i32 (I64)
    // 0001ed: ishl_imm.i32 (I64)
    // --> [DynRexOp1r_ib#40c1] and stop
    // 0001ed: ishl_imm.i32 (I32)
    // --> [DynRexOp1r_ib#40c1] and stop
    0x0153, 0x40c1,
    // end of ishl_imm.i32 (I32)
    // end of ishl_imm.i32 (I64)
    // 0001ef: isub.i32 (I64)
    // --> [DynRexOp1rr#29] and stop
    // 0001ef: isub.i32 (I32)
    // --> [DynRexOp1rr#29] and stop
    0x014b, 0x0029,
    // end of isub.i32 (I32)
    // end of isub.i32 (I64)
    // 0001f1: isub_ifbin.i32 (I64)
    // --> [DynRexOp1rin#19] and stop
    // 0001f1: isub_ifbin.i32 (I32)
    // --> [DynRexOp1rin#19] and stop
    0x014f, 0x0019,
    // end of isub_ifbin.i32 (I32)
    // end of isub_ifbin.i32 (I64)
    // 0001f3: isub_ifborrow.i32 (I64)
    // --> [DynRexOp1rio#19] and stop
    // 0001f3: isub_ifborrow.i32 (I32)
    // --> [DynRexOp1rio#19] and stop
    0x0151, 0x0019,
    // end of isub_ifborrow.i32 (I32)
    // end of isub_ifborrow.i32 (I64)
    // 0001f5: isub_ifbout.i32 (I64)
    // --> [DynRexOp1rout#29] and stop
    // 0001f5: isub_ifbout.i32 (I32)
    // --> [DynRexOp1rout#29] and stop
    0x014d, 0x0029,
    // end of isub_ifbout.i32 (I32)
    // end of isub_ifbout.i32 (I64)
    // 0001f7: load_complex.i32 (I64)
    // stop unless inst_predicate_8
    // 0001f7: uload32_complex (I64)
    // stop unless inst_predicate_8
    0x1008,
    // --> [RexOp1ldWithIndex#8b]
    // --> [RexOp1ldWithIndex#8b]
    0x0036, 0x008b,
    // --> [Op1ldWithIndex#8b]
    // --> [Op1ldWithIndex#8b]
    0x0034, 0x008b,
    // --> [RexOp1ldWithIndexDisp8#8b]
    // --> [RexOp1ldWithIndexDisp8#8b]
    0x003e, 0x008b,
    // --> [Op1ldWithIndexDisp8#8b]
    // --> [Op1ldWithIndexDisp8#8b]
    0x003c, 0x008b,
    // --> [RexOp1ldWithIndexDisp32#8b]
    // --> [RexOp1ldWithIndexDisp32#8b]
    0x0046, 0x008b,
    // --> [Op1ldWithIndexDisp32#8b] and stop
    // --> [Op1ldWithIndexDisp32#8b] and stop
    0x0045, 0x008b,
    // end of uload32_complex (I64)
    // end of load_complex.i32 (I64)
    // 000204: popcnt.i32 (I64)
    // stop unless PredicateView(15)
    0x102e,
    // --> [RexMp2urm#6b8]
    0x0170, 0x06b8,
    // --> [Mp2urm#6b8] and stop
    0x016f, 0x06b8,
    // end of popcnt.i32 (I64)
    // 000209: regfill.i32 (I64)
    // --> [RexOp1regfill32#8b]
    // 000209: regfill.b1 (I64)
    // --> [RexOp1regfill32#8b]
    // 000209: regfill.i8 (I64)
    // --> [RexOp1regfill32#8b]
    // 000209: regfill.i16 (I64)
    // --> [RexOp1regfill32#8b]
    0x00b4, 0x008b,
    // --> [Op1regfill32#8b] and stop
    // --> [Op1regfill32#8b] and stop
    // --> [Op1regfill32#8b] and stop
    // --> [Op1regfill32#8b] and stop
    // 00020b: regfill.i32 (I32)
    // --> [Op1regfill32#8b] and stop
    // 00020b: regfill.r32 (I32)
    // --> [Op1regfill32#8b] and stop
    // 00020b: regfill.b1 (I32)
    // --> [Op1regfill32#8b] and stop
    // 00020b: regfill.i8 (I32)
    // --> [Op1regfill32#8b] and stop
    // 00020b: regfill.i16 (I32)
    // --> [Op1regfill32#8b] and stop
    0x00b3, 0x008b,
    // end of regfill.i16 (I32)
    // end of regfill.i8 (I32)
    // end of regfill.b1 (I32)
    // end of regfill.r32 (I32)
    // end of regfill.i32 (I32)
    // end of regfill.i16 (I64)
    // end of regfill.i8 (I64)
    // end of regfill.b1 (I64)
    // end of regfill.i32 (I64)
    // 00020d: regmove.i32 (I64)
    // --> [RexOp1rmov#89] and stop
    // 00020d: regmove.i16 (I64)
    // --> [RexOp1rmov#89] and stop
    // 00020d: regmove.b8 (I64)
    // --> [RexOp1rmov#89] and stop
    // 00020d: regmove.b16 (I64)
    // --> [RexOp1rmov#89] and stop
    // 00020d: regmove.b32 (I64)
    // --> [RexOp1rmov#89] and stop
    // 00020d: regmove.r32 (I64)
    // --> [RexOp1rmov#89] and stop
    0x000d, 0x0089,
    // end of regmove.r32 (I64)
    // end of regmove.b32 (I64)
    // end of regmove.b16 (I64)
    // end of regmove.b8 (I64)
    // end of regmove.i16 (I64)
    // end of regmove.i32 (I64)
    // 00020f: regspill.i32 (I64)
    // --> [RexOp1regspill32#89]
    // 00020f: regspill.b1 (I64)
    // --> [RexOp1regspill32#89]
    // 00020f: regspill.i8 (I64)
    // --> [RexOp1regspill32#89]
    // 00020f: regspill.i16 (I64)
    // --> [RexOp1regspill32#89]
    0x0094, 0x0089,
    // --> [Op1regspill32#89] and stop
    // --> [Op1regspill32#89] and stop
    // --> [Op1regspill32#89] and stop
    // --> [Op1regspill32#89] and stop
    // 000211: regspill.i32 (I32)
    // --> [Op1regspill32#89] and stop
    // 000211: regspill.r32 (I32)
    // --> [Op1regspill32#89] and stop
    // 000211: regspill.b1 (I32)
    // --> [Op1regspill32#89] and stop
    // 000211: regspill.i8 (I32)
    // --> [Op1regspill32#89] and stop
    // 000211: regspill.i16 (I32)
    // --> [Op1regspill32#89] and stop
    0x0093, 0x0089,
    // end of regspill.i16 (I32)
    // end of regspill.i8 (I32)
    // end of regspill.b1 (I32)
    // end of regspill.r32 (I32)
    // end of regspill.i32 (I32)
    // end of regspill.i16 (I64)
    // end of regspill.i8 (I64)
    // end of regspill.b1 (I64)
    // end of regspill.i32 (I64)
    // 000213: rotl.i32 (I64)
    // --> [RexOp1rc#d3]
    0x016c, 0x00d3,
    // --> [Op1rc#d3] and stop
    // 000215: rotl.i32 (I32)
    // --> [Op1rc#d3] and stop
    0x016b, 0x00d3,
    // end of rotl.i32 (I32)
    // end of rotl.i32 (I64)
    // 000217: rotl_imm.i32 (I64)
    // --> [DynRexOp1r_ib#c1] and stop
    // 000217: rotl_imm.i32 (I32)
    // --> [DynRexOp1r_ib#c1] and stop
    0x0153, 0x00c1,
    // end of rotl_imm.i32 (I32)
    // end of rotl_imm.i32 (I64)
    // 000219: rotr.i32 (I64)
    // --> [RexOp1rc#10d3]
    0x016c, 0x10d3,
    // --> [Op1rc#10d3] and stop
    // 00021b: rotr.i32 (I32)
    // --> [Op1rc#10d3] and stop
    0x016b, 0x10d3,
    // end of rotr.i32 (I32)
    // end of rotr.i32 (I64)
    // 00021d: rotr_imm.i32 (I64)
    // --> [DynRexOp1r_ib#10c1] and stop
    // 00021d: rotr_imm.i32 (I32)
    // --> [DynRexOp1r_ib#10c1] and stop
    0x0153, 0x10c1,
    // end of rotr_imm.i32 (I32)
    // end of rotr_imm.i32 (I64)
    // 00021f: selectif.i32 (I64)
    // --> [DynRexOp2cmov#440] and stop
    // 00021f: selectif.i32 (I32)
    // --> [DynRexOp2cmov#440] and stop
    0x018d, 0x0440,
    // end of selectif.i32 (I32)
    // end of selectif.i32 (I64)
    // 000221: sextend.i32 (I64)
    // skip 4 unless inst_predicate_5
    0x5005,
    // --> [RexOp2urm_noflags#4be]
    0x0022, 0x04be,
    // --> [Op2urm_noflags_abcd#4be]
    0x0020, 0x04be,
    // stop unless inst_predicate_2
    0x1002,
    // --> [RexOp2urm_noflags#4bf]
    0x0022, 0x04bf,
    // --> [Op2urm_noflags#4bf] and stop
    0x0025, 0x04bf,
    // end of sextend.i32 (I64)
    // 00022b: sload16.i32 (I64)
    // --> [RexOp2ld#4bf]
    0x009c, 0x04bf,
    // --> [Op2ld#4bf]
    0x009a, 0x04bf,
    // --> [RexOp2ldDisp8#4bf]
    0x00a4, 0x04bf,
    // --> [Op2ldDisp8#4bf]
    0x00a2, 0x04bf,
    // --> [RexOp2ldDisp32#4bf]
    0x00ac, 0x04bf,
    // --> [Op2ldDisp32#4bf] and stop
    0x00ab, 0x04bf,
    // end of sload16.i32 (I64)
    // 000237: sload16_complex.i32 (I64)
    // stop unless inst_predicate_8
    0x1008,
    // --> [RexOp2ldWithIndex#4bf]
    0x003a, 0x04bf,
    // --> [Op2ldWithIndex#4bf]
    0x0038, 0x04bf,
    // --> [RexOp2ldWithIndexDisp8#4bf]
    0x0042, 0x04bf,
    // --> [Op2ldWithIndexDisp8#4bf]
    0x0040, 0x04bf,
    // --> [RexOp2ldWithIndexDisp32#4bf]
    0x004a, 0x04bf,
    // --> [Op2ldWithIndexDisp32#4bf] and stop
    0x0049, 0x04bf,
    // end of sload16_complex.i32 (I64)
    // 000244: sload8.i32 (I64)
    // --> [RexOp2ld#4be]
    0x009c, 0x04be,
    // --> [Op2ld#4be]
    0x009a, 0x04be,
    // --> [RexOp2ldDisp8#4be]
    0x00a4, 0x04be,
    // --> [Op2ldDisp8#4be]
    0x00a2, 0x04be,
    // --> [RexOp2ldDisp32#4be]
    0x00ac, 0x04be,
    // --> [Op2ldDisp32#4be] and stop
    0x00ab, 0x04be,
    // end of sload8.i32 (I64)
    // 000250: sload8_complex.i32 (I64)
    // stop unless inst_predicate_8
    0x1008,
    // --> [RexOp2ldWithIndex#4be]
    0x003a, 0x04be,
    // --> [Op2ldWithIndex#4be]
    0x0038, 0x04be,
    // --> [RexOp2ldWithIndexDisp8#4be]
    0x0042, 0x04be,
    // --> [Op2ldWithIndexDisp8#4be]
    0x0040, 0x04be,
    // --> [RexOp2ldWithIndexDisp32#4be]
    0x004a, 0x04be,
    // --> [Op2ldWithIndexDisp32#4be] and stop
    0x0049, 0x04be,
    // end of sload8_complex.i32 (I64)
    // 00025d: spill.i32 (I64)
    // --> [RexOp1spillSib32#89]
    // 00025d: spill.b1 (I64)
    // --> [RexOp1spillSib32#89]
    // 00025d: spill.i8 (I64)
    // --> [RexOp1spillSib32#89]
    // 00025d: spill.i16 (I64)
    // --> [RexOp1spillSib32#89]
    0x0090, 0x0089,
    // --> [Op1spillSib32#89] and stop
    // --> [Op1spillSib32#89] and stop
    // --> [Op1spillSib32#89] and stop
    // --> [Op1spillSib32#89] and stop
    // 00025f: spill.i32 (I32)
    // --> [Op1spillSib32#89] and stop
    // 00025f: spill.r32 (I32)
    // --> [Op1spillSib32#89] and stop
    // 00025f: spill.b1 (I32)
    // --> [Op1spillSib32#89] and stop
    // 00025f: spill.i8 (I32)
    // --> [Op1spillSib32#89] and stop
    // 00025f: spill.i16 (I32)
    // --> [Op1spillSib32#89] and stop
    0x008f, 0x0089,
    // end of spill.i16 (I32)
    // end of spill.i8 (I32)
    // end of spill.b1 (I32)
    // end of spill.r32 (I32)
    // end of spill.i32 (I32)
    // end of spill.i16 (I64)
    // end of spill.i8 (I64)
    // end of spill.b1 (I64)
    // end of spill.i32 (I64)
    // 000261: sshr.i32 (I64)
    // --> [RexOp1rc#70d3]
    0x016c, 0x70d3,
    // --> [Op1rc#70d3] and stop
    // 000263: sshr.i32 (I32)
    // --> [Op1rc#70d3] and stop
    0x016b, 0x70d3,
    // end of sshr.i32 (I32)
    // end of sshr.i32 (I64)
    // 000265: sshr_imm.i32 (I64)
    // --> [DynRexOp1r_ib#70c1] and stop
    // 000265: sshr_imm.i32 (I32)
    // --> [DynRexOp1r_ib#70c1] and stop
    0x0153, 0x70c1,
    // end of sshr_imm.i32 (I32)
    // end of sshr_imm.i32 (I64)
    // 000267: store_complex.i32 (I64)
    // stop unless inst_predicate_9
    // 000267: istore32_complex (I64)
    // stop unless inst_predicate_9
    0x1009,
    // --> [RexOp1stWithIndex#89]
    // --> [RexOp1stWithIndex#89]
    0x004e, 0x0089,
    // --> [Op1stWithIndex#89]
    // --> [Op1stWithIndex#89]
    0x004c, 0x0089,
    // --> [RexOp1stWithIndexDisp8#89]
    // --> [RexOp1stWithIndexDisp8#89]
    0x0056, 0x0089,
    // --> [Op1stWithIndexDisp8#89]
    // --> [Op1stWithIndexDisp8#89]
    0x0054, 0x0089,
    // --> [RexOp1stWithIndexDisp32#89]
    // --> [RexOp1stWithIndexDisp32#89]
    0x005e, 0x0089,
    // --> [Op1stWithIndexDisp32#89] and stop
    // --> [Op1stWithIndexDisp32#89] and stop
    0x005d, 0x0089,
    // end of istore32_complex (I64)
    // end of store_complex.i32 (I64)
    // 000274: uextend.i32 (I64)
    // skip 4 unless inst_predicate_5
    0x5005,
    // --> [RexOp2urm_noflags#4b6]
    0x0022, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // stop unless inst_predicate_2
    0x1002,
    // --> [RexOp2urm_noflags#4b7]
    0x0022, 0x04b7,
    // --> [Op2urm_noflags#4b7] and stop
    0x0025, 0x04b7,
    // end of uextend.i32 (I64)
    // 00027e: uload16.i32 (I64)
    // --> [RexOp2ld#4b7]
    0x009c, 0x04b7,
    // --> [Op2ld#4b7]
    0x009a, 0x04b7,
    // --> [RexOp2ldDisp8#4b7]
    0x00a4, 0x04b7,
    // --> [Op2ldDisp8#4b7]
    0x00a2, 0x04b7,
    // --> [RexOp2ldDisp32#4b7]
    0x00ac, 0x04b7,
    // --> [Op2ldDisp32#4b7] and stop
    0x00ab, 0x04b7,
    // end of uload16.i32 (I64)
    // 00028a: uload16_complex.i32 (I64)
    // stop unless inst_predicate_8
    0x1008,
    // --> [RexOp2ldWithIndex#4b7]
    0x003a, 0x04b7,
    // --> [Op2ldWithIndex#4b7]
    0x0038, 0x04b7,
    // --> [RexOp2ldWithIndexDisp8#4b7]
    0x0042, 0x04b7,
    // --> [Op2ldWithIndexDisp8#4b7]
    0x0040, 0x04b7,
    // --> [RexOp2ldWithIndexDisp32#4b7]
    0x004a, 0x04b7,
    // --> [Op2ldWithIndexDisp32#4b7] and stop
    0x0049, 0x04b7,
    // end of uload16_complex.i32 (I64)
    // 000297: uload8.i32 (I64)
    // --> [RexOp2ld#4b6]
    0x009c, 0x04b6,
    // --> [Op2ld#4b6]
    0x009a, 0x04b6,
    // --> [RexOp2ldDisp8#4b6]
    0x00a4, 0x04b6,
    // --> [Op2ldDisp8#4b6]
    0x00a2, 0x04b6,
    // --> [RexOp2ldDisp32#4b6]
    0x00ac, 0x04b6,
    // --> [Op2ldDisp32#4b6] and stop
    0x00ab, 0x04b6,
    // end of uload8.i32 (I64)
    // 0002a3: uload8_complex.i32 (I64)
    // stop unless inst_predicate_8
    0x1008,
    // --> [RexOp2ldWithIndex#4b6]
    0x003a, 0x04b6,
    // --> [Op2ldWithIndex#4b6]
    0x0038, 0x04b6,
    // --> [RexOp2ldWithIndexDisp8#4b6]
    0x0042, 0x04b6,
    // --> [Op2ldWithIndexDisp8#4b6]
    0x0040, 0x04b6,
    // --> [RexOp2ldWithIndexDisp32#4b6]
    0x004a, 0x04b6,
    // --> [Op2ldWithIndexDisp32#4b6] and stop
    0x0049, 0x04b6,
    // end of uload8_complex.i32 (I64)
    // 0002b0: ushr.i32 (I64)
    // --> [RexOp1rc#50d3]
    0x016c, 0x50d3,
    // --> [Op1rc#50d3] and stop
    // 0002b2: ushr.i32 (I32)
    // --> [Op1rc#50d3] and stop
    0x016b, 0x50d3,
    // end of ushr.i32 (I32)
    // end of ushr.i32 (I64)
    // 0002b4: ushr_imm.i32 (I64)
    // --> [DynRexOp1r_ib#50c1] and stop
    // 0002b4: ushr_imm.i32 (I32)
    // --> [DynRexOp1r_ib#50c1] and stop
    0x0153, 0x50c1,
    // end of ushr_imm.i32 (I32)
    // end of ushr_imm.i32 (I64)
    // 0002b6: x86_bsf.i32 (I64)
    // --> [DynRexOp2bsf_and_bsr#4bc] and stop
    // 0002b6: x86_bsf.i32 (I32)
    // --> [DynRexOp2bsf_and_bsr#4bc] and stop
    0x0173, 0x04bc,
    // end of x86_bsf.i32 (I32)
    // end of x86_bsf.i32 (I64)
    // 0002b8: x86_bsr.i32 (I64)
    // --> [DynRexOp2bsf_and_bsr#4bd] and stop
    // 0002b8: x86_bsr.i32 (I32)
    // --> [DynRexOp2bsf_and_bsr#4bd] and stop
    0x0173, 0x04bd,
    // end of x86_bsr.i32 (I32)
    // end of x86_bsr.i32 (I64)
    // 0002ba: x86_cvtt2si.i32 (I64)
    // skip 4 unless inst_predicate_10
    0x500a,
    // --> [RexMp2rfurm#62c]
    0x0130, 0x062c,
    // --> [Mp2rfurm#62c]
    0x012e, 0x062c,
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexMp2rfurm#72c]
    0x0130, 0x072c,
    // --> [Mp2rfurm#72c] and stop
    0x012f, 0x072c,
    // end of x86_cvtt2si.i32 (I64)
    // 0002c4: x86_sdivmodx.i32 (I64)
    // --> [DynRexOp1div#70f7] and stop
    // 0002c4: x86_sdivmodx.i32 (I32)
    // --> [DynRexOp1div#70f7] and stop
    0x015f, 0x70f7,
    // end of x86_sdivmodx.i32 (I32)
    // end of x86_sdivmodx.i32 (I64)
    // 0002c6: x86_smulx.i32 (I64)
    // --> [DynRexOp1mulx#50f7] and stop
    // 0002c6: x86_smulx.i32 (I32)
    // --> [DynRexOp1mulx#50f7] and stop
    0x0161, 0x50f7,
    // end of x86_smulx.i32 (I32)
    // end of x86_smulx.i32 (I64)
    // 0002c8: x86_udivmodx.i32 (I64)
    // --> [DynRexOp1div#60f7] and stop
    // 0002c8: x86_udivmodx.i32 (I32)
    // --> [DynRexOp1div#60f7] and stop
    0x015f, 0x60f7,
    // end of x86_udivmodx.i32 (I32)
    // end of x86_udivmodx.i32 (I64)
    // 0002ca: x86_umulx.i32 (I64)
    // --> [DynRexOp1mulx#40f7] and stop
    // 0002ca: x86_umulx.i32 (I32)
    // --> [DynRexOp1mulx#40f7] and stop
    0x0161, 0x40f7,
    // end of x86_umulx.i32 (I32)
    // end of x86_umulx.i32 (I64)
    // 0002cc: copy.r64 (I64)
    // --> [RexOp1umr#8089] and stop
    0x0009, 0x8089,
    // end of copy.r64 (I64)
    // 0002ce: is_invalid.r64 (I64)
    // --> [RexOp1is_invalid#f083] and stop
    0x0231, 0xf083,
    // end of is_invalid.r64 (I64)
    // 0002d0: is_null.r64 (I64)
    // --> [RexOp1is_zero#8085] and stop
    0x022d, 0x8085,
    // end of is_null.r64 (I64)
    // 0002d2: null.r64 (I64)
    // --> [RexOp1pu_id_ref#b8]
    0x0228, 0x00b8,
    // --> [Op1pu_id_ref#b8] and stop
    // 0002d4: null.r32 (I32)
    // --> [Op1pu_id_ref#b8] and stop
    0x0227, 0x00b8,
    // end of null.r32 (I32)
    // end of null.r64 (I64)
    // 0002d6: band.b1 (I64)
    // --> [RexOp1rr#21]
    0x015a, 0x0021,
    // --> [Op1rr#21] and stop
    // 0002d8: band.b1 (I32)
    // --> [Op1rr#21] and stop
    0x0159, 0x0021,
    // end of band.b1 (I32)
    // end of band.b1 (I64)
    // 0002da: bconst.b1 (I64)
    // --> [RexOp1pu_id_bool#b8]
    // 0002da: bconst.b8 (I64)
    // --> [RexOp1pu_id_bool#b8]
    // 0002da: bconst.b16 (I64)
    // --> [RexOp1pu_id_bool#b8]
    // 0002da: bconst.b32 (I64)
    // --> [RexOp1pu_id_bool#b8]
    0x0018, 0x00b8,
    // --> [Op1pu_id_bool#b8] and stop
    // --> [Op1pu_id_bool#b8] and stop
    // --> [Op1pu_id_bool#b8] and stop
    // --> [Op1pu_id_bool#b8] and stop
    // 0002dc: bconst.b1 (I32)
    // --> [Op1pu_id_bool#b8] and stop
    // 0002dc: bconst.b8 (I32)
    // --> [Op1pu_id_bool#b8] and stop
    // 0002dc: bconst.b16 (I32)
    // --> [Op1pu_id_bool#b8] and stop
    // 0002dc: bconst.b32 (I32)
    // --> [Op1pu_id_bool#b8] and stop
    0x0017, 0x00b8,
    // end of bconst.b32 (I32)
    // end of bconst.b16 (I32)
    // end of bconst.b8 (I32)
    // end of bconst.b1 (I32)
    // end of bconst.b32 (I64)
    // end of bconst.b16 (I64)
    // end of bconst.b8 (I64)
    // end of bconst.b1 (I64)
    // 0002de: bor.b1 (I64)
    // --> [RexOp1rr#09]
    0x015a, 0x0009,
    // --> [Op1rr#09] and stop
    // 0002e0: bor.b1 (I32)
    // --> [Op1rr#09] and stop
    0x0159, 0x0009,
    // end of bor.b1 (I32)
    // end of bor.b1 (I64)
    // 0002e2: brnz.b1 (I64)
    // --> [RexOp1t8jccb#75]
    0x020c, 0x0075,
    // --> [Op1t8jccb_abcd#75]
    0x020a, 0x0075,
    // --> [RexOp1t8jccd#85]
    0x0210, 0x0085,
    // --> [Op1t8jccd_abcd#85] and stop
    0x020f, 0x0085,
    // end of brnz.b1 (I64)
    // 0002ea: brz.b1 (I64)
    // --> [RexOp1t8jccb#74]
    0x020c, 0x0074,
    // --> [Op1t8jccb_abcd#74]
    0x020a, 0x0074,
    // --> [RexOp1t8jccd#84]
    0x0210, 0x0084,
    // --> [Op1t8jccd_abcd#84] and stop
    0x020f, 0x0084,
    // end of brz.b1 (I64)
    // 0002f2: bxor.b1 (I64)
    // --> [RexOp1rr#31]
    0x015a, 0x0031,
    // --> [Op1rr#31] and stop
    // 0002f4: bxor.b1 (I32)
    // --> [Op1rr#31] and stop
    0x0159, 0x0031,
    // end of bxor.b1 (I32)
    // end of bxor.b1 (I64)
    // 0002f6: regmove.b1 (I64)
    // --> [RexOp1rmov#89]
    0x000c, 0x0089,
    // --> [Op1rmov#89] and stop
    // 0002f8: regmove.i32 (I32)
    // --> [Op1rmov#89] and stop
    // 0002f8: regmove.r32 (I32)
    // --> [Op1rmov#89] and stop
    // 0002f8: regmove.b1 (I32)
    // --> [Op1rmov#89] and stop
    // 0002f8: regmove.i16 (I32)
    // --> [Op1rmov#89] and stop
    // 0002f8: regmove.b8 (I32)
    // --> [Op1rmov#89] and stop
    // 0002f8: regmove.b16 (I32)
    // --> [Op1rmov#89] and stop
    // 0002f8: regmove.b32 (I32)
    // --> [Op1rmov#89] and stop
    0x000b, 0x0089,
    // end of regmove.b32 (I32)
    // end of regmove.b16 (I32)
    // end of regmove.b8 (I32)
    // end of regmove.i16 (I32)
    // end of regmove.b1 (I32)
    // end of regmove.r32 (I32)
    // end of regmove.i32 (I32)
    // end of regmove.b1 (I64)
    // 0002fa: iconst.i8 (I64)
    // stop unless inst_predicate_1
    0x1001,
    // --> [RexOp1u_id_z#30]
    0x001c, 0x0030,
    // --> [Op1u_id_z#30] and stop
    0x001b, 0x0030,
    // end of iconst.i8 (I64)
    // 0002ff: ireduce.i8 (I64)
    // skip 2 unless inst_predicate_2
    0x3002,
    // --> [null#00]
    0x001e, 0x0000,
    // skip 2 unless inst_predicate_3
    // 000302: ireduce.i16 (I64)
    // skip 2 unless inst_predicate_3
    0x3003,
    // --> [null#00]
    // --> [null#00]
    0x001e, 0x0000,
    // stop unless inst_predicate_4
    // stop unless inst_predicate_4
    0x1004,
    // --> [null#00] and stop
    // --> [null#00] and stop
    0x001f, 0x0000,
    // end of ireduce.i16 (I64)
    // end of ireduce.i8 (I64)
    // 000308: regmove.i8 (I64)
    // --> [RexOp1rmov#89]
    0x000c, 0x0089,
    // --> [RexOp1rmov#89]
    0x000c, 0x0089,
    // --> [Op1rmov#89] and stop
    0x000b, 0x0089,
    // end of regmove.i8 (I64)
    // 00030e: bconst.b64 (I64)
    // --> [RexOp1pu_id_bool#b8] and stop
    0x0019, 0x00b8,
    // end of bconst.b64 (I64)
    // 000310: adjust_sp_down_imm (I64)
    // --> [RexOp1adjustsp_ib#d083]
    0x00cc, 0xd083,
    // --> [RexOp1adjustsp_id#d081] and stop
    0x00cf, 0xd081,
    // end of adjust_sp_down_imm (I64)
    // 000314: adjust_sp_up_imm (I64)
    // --> [RexOp1adjustsp_ib#8083]
    0x00cc, 0x8083,
    // --> [RexOp1adjustsp_id#8081] and stop
    0x00cf, 0x8081,
    // end of adjust_sp_up_imm (I64)
    // 000318: brff (I64)
    // --> [RexOp1brfb#70]
    0x01fa, 0x0070,
    // --> [Op1brfb#70]
    0x01f8, 0x0070,
    // --> [RexOp2brfd#480]
    0x01fe, 0x0480,
    // --> [Op2brfd#480] and stop
    0x01fd, 0x0480,
    // end of brff (I64)
    // 000320: brif (I64)
    // --> [RexOp1brib#70]
    0x01f2, 0x0070,
    // --> [Op1brib#70]
    0x01f0, 0x0070,
    // --> [RexOp2brid#480]
    0x01f6, 0x0480,
    // --> [Op2brid#480] and stop
    0x01f5, 0x0480,
    // end of brif (I64)
    // 000328: call (I64)
    // skip 2 unless inst_predicate_30
    0x301e,
    // --> [Op1call_id#e8]
    0x01e2, 0x00e8,
    // stop unless PredicateView(10)
    0x1029,
    // --> [Op1call_plt_id#e8] and stop
    0x01e5, 0x00e8,
    // end of call (I64)
    // 00032e: copy_special (I64)
    // --> [RexOp1copysp#8089] and stop
    0x0029, 0x8089,
    // end of copy_special (I64)
    // 000330: debugtrap (I64)
    // --> [debugtrap#00] and stop
    // 000330: debugtrap (I32)
    // --> [debugtrap#00] and stop
    0x0221, 0x0000,
    // end of debugtrap (I32)
    // end of debugtrap (I64)
    // 000332: f32const (I64)
    // stop unless inst_predicate_12
    0x100c,
    // --> [RexOp2f32imm_z#457]
    0x0124, 0x0457,
    // --> [Op2f32imm_z#457] and stop
    0x0121, 0x0457,
    // end of f32const (I64)
    // 000337: f64const (I64)
    // stop unless inst_predicate_13
    0x100d,
    // --> [RexMp2f64imm_z#557]
    0x0126, 0x0557,
    // --> [Mp2f64imm_z#557] and stop
    0x0123, 0x0557,
    // end of f64const (I64)
    // 00033c: jump (I64)
    // --> [Op1jmpb#eb]
    // 00033c: jump (I32)
    // --> [Op1jmpb#eb]
    0x01ec, 0x00eb,
    // --> [Op1jmpd#e9] and stop
    // --> [Op1jmpd#e9] and stop
    0x01ef, 0x00e9,
    // end of jump (I32)
    // end of jump (I64)
    // 000340: resumable_trap (I64)
    // --> [Op2trap#40b] and stop
    // 000340: trap (I64)
    // --> [Op2trap#40b] and stop
    // 000340: resumable_trap (I32)
    // --> [Op2trap#40b] and stop
    // 000340: trap (I32)
    // --> [Op2trap#40b] and stop
    0x021f, 0x040b,
    // end of trap (I32)
    // end of resumable_trap (I32)
    // end of trap (I64)
    // end of resumable_trap (I64)
    // 000342: return (I64)
    // --> [Op1ret#c3] and stop
    // 000342: return (I32)
    // --> [Op1ret#c3] and stop
    0x01eb, 0x00c3,
    // end of return (I32)
    // end of return (I64)
    // 000344: safepoint (I64)
    // --> [safepoint#00] and stop
    // 000344: safepoint (I32)
    // --> [safepoint#00] and stop
    0x0233, 0x0000,
    // end of safepoint (I32)
    // end of safepoint (I64)
    // 000346: sload32_complex (I64)
    // stop unless inst_predicate_8
    0x1008,
    // --> [RexOp1ldWithIndex#8063]
    0x0036, 0x8063,
    // --> [RexOp1ldWithIndexDisp8#8063]
    0x003e, 0x8063,
    // --> [RexOp1ldWithIndexDisp32#8063] and stop
    0x0047, 0x8063,
    // end of sload32_complex (I64)
    // 00034d: trapff (I64)
    // --> [trapff#00] and stop
    // 00034d: trapff (I32)
    // --> [trapff#00] and stop
    0x0225, 0x0000,
    // end of trapff (I32)
    // end of trapff (I64)
    // 00034f: trapif (I64)
    // --> [trapif#00] and stop
    // 00034f: trapif (I32)
    // --> [trapif#00] and stop
    0x0223, 0x0000,
    // end of trapif (I32)
    // end of trapif (I64)
    // 000351: trueff (I64)
    // --> [RexOp2setf#490]
    0x018a, 0x0490,
    // --> [Op2setf_abcd#490] and stop
    // 000353: trueff (I32)
    // --> [Op2setf_abcd#490] and stop
    0x0189, 0x0490,
    // end of trueff (I32)
    // end of trueff (I64)
    // 000355: trueif (I64)
    // --> [RexOp2seti#490]
    0x0186, 0x0490,
    // --> [Op2seti_abcd#490] and stop
    // 000357: trueif (I32)
    // --> [Op2seti_abcd#490] and stop
    0x0185, 0x0490,
    // end of trueif (I32)
    // end of trueif (I64)
    // 000359: band.f64 (I64)
    // --> [RexOp2fa#454]
    // 000359: band.f32 (I64)
    // --> [RexOp2fa#454]
    0x0164, 0x0454,
    // --> [Op2fa#454] and stop
    // --> [Op2fa#454] and stop
    // 00035b: band.f64 (I32)
    // --> [Op2fa#454] and stop
    // 00035b: band.f32 (I32)
    // --> [Op2fa#454] and stop
    0x0163, 0x0454,
    // end of band.f32 (I32)
    // end of band.f64 (I32)
    // end of band.f32 (I64)
    // end of band.f64 (I64)
    // 00035d: band_not.f64 (I64)
    // --> [RexOp2fax#455]
    // 00035d: band_not.f32 (I64)
    // --> [RexOp2fax#455]
    0x0168, 0x0455,
    // --> [Op2fax#455] and stop
    // --> [Op2fax#455] and stop
    // 00035f: band_not.f64 (I32)
    // --> [Op2fax#455] and stop
    // 00035f: band_not.f32 (I32)
    // --> [Op2fax#455] and stop
    0x0167, 0x0455,
    // end of band_not.f32 (I32)
    // end of band_not.f64 (I32)
    // end of band_not.f32 (I64)
    // end of band_not.f64 (I64)
    // 000361: bitcast.f64 (I64)
    // stop unless inst_predicate_4
    0x1004,
    // --> [RexMp2frurm#856e] and stop
    // 000362: scalar_to_vector.b64x2 (I64)
    // --> [RexMp2frurm#856e] and stop
    // 000362: scalar_to_vector.i64x2 (I64)
    // --> [RexMp2frurm#856e] and stop
    0x00d3, 0x856e,
    // end of scalar_to_vector.i64x2 (I64)
    // end of scalar_to_vector.b64x2 (I64)
    // end of bitcast.f64 (I64)
    // 000364: bor.f64 (I64)
    // --> [RexOp2fa#456]
    // 000364: bor.f32 (I64)
    // --> [RexOp2fa#456]
    0x0164, 0x0456,
    // --> [Op2fa#456] and stop
    // --> [Op2fa#456] and stop
    // 000366: bor.f64 (I32)
    // --> [Op2fa#456] and stop
    // 000366: bor.f32 (I32)
    // --> [Op2fa#456] and stop
    0x0163, 0x0456,
    // end of bor.f32 (I32)
    // end of bor.f64 (I32)
    // end of bor.f32 (I64)
    // end of bor.f64 (I64)
    // 000368: bxor.f64 (I64)
    // --> [RexOp2fa#457]
    // 000368: bxor.f32 (I64)
    // --> [RexOp2fa#457]
    0x0164, 0x0457,
    // --> [Op2fa#457] and stop
    // --> [Op2fa#457] and stop
    // 00036a: bxor.f64 (I32)
    // --> [Op2fa#457] and stop
    // 00036a: bxor.f32 (I32)
    // --> [Op2fa#457] and stop
    0x0163, 0x0457,
    // end of bxor.f32 (I32)
    // end of bxor.f64 (I32)
    // end of bxor.f32 (I64)
    // end of bxor.f64 (I64)
    // 00036c: ceil.f64 (I64)
    // stop unless PredicateView(16)
    // 00036c: floor.f64 (I64)
    // stop unless PredicateView(16)
    // 00036c: nearest.f64 (I64)
    // stop unless PredicateView(16)
    // 00036c: trunc.f64 (I64)
    // stop unless PredicateView(16)
    0x102f,
    // --> [RexMp3furmi_rnd#d0b]
    // --> [RexMp3furmi_rnd#d0b]
    // --> [RexMp3furmi_rnd#d0b]
    // --> [RexMp3furmi_rnd#d0b]
    0x0134, 0x0d0b,
    // --> [Mp3furmi_rnd#d0b] and stop
    // --> [Mp3furmi_rnd#d0b] and stop
    // --> [Mp3furmi_rnd#d0b] and stop
    // --> [Mp3furmi_rnd#d0b] and stop
    0x0133, 0x0d0b,
    // end of trunc.f64 (I64)
    // end of nearest.f64 (I64)
    // end of floor.f64 (I64)
    // end of ceil.f64 (I64)
    // 000371: copy.f64 (I64)
    // --> [RexOp2furm#428]
    // 000371: copy.f32 (I64)
    // --> [RexOp2furm#428]
    0x00da, 0x0428,
    // --> [Op2furm#428] and stop
    // --> [Op2furm#428] and stop
    // 000373: copy.b8x16 (I64)
    // --> [Op2furm#428] and stop
    // 000373: copy.b16x8 (I64)
    // --> [Op2furm#428] and stop
    // 000373: copy.b32x4 (I64)
    // --> [Op2furm#428] and stop
    // 000373: copy.b64x2 (I64)
    // --> [Op2furm#428] and stop
    // 000373: copy.i8x16 (I64)
    // --> [Op2furm#428] and stop
    // 000373: copy.i16x8 (I64)
    // --> [Op2furm#428] and stop
    // 000373: copy.i32x4 (I64)
    // --> [Op2furm#428] and stop
    // 000373: copy.i64x2 (I64)
    // --> [Op2furm#428] and stop
    // 000373: copy.f32x4 (I64)
    // --> [Op2furm#428] and stop
    // 000373: copy.f64x2 (I64)
    // --> [Op2furm#428] and stop
    // 000373: copy.f64 (I32)
    // --> [Op2furm#428] and stop
    // 000373: copy.f32 (I32)
    // --> [Op2furm#428] and stop
    // 000373: copy.b8x16 (I32)
    // --> [Op2furm#428] and stop
    // 000373: copy.b16x8 (I32)
    // --> [Op2furm#428] and stop
    // 000373: copy.b32x4 (I32)
    // --> [Op2furm#428] and stop
    // 000373: copy.b64x2 (I32)
    // --> [Op2furm#428] and stop
    // 000373: copy.i8x16 (I32)
    // --> [Op2furm#428] and stop
    // 000373: copy.i16x8 (I32)
    // --> [Op2furm#428] and stop
    // 000373: copy.i32x4 (I32)
    // --> [Op2furm#428] and stop
    // 000373: copy.i64x2 (I32)
    // --> [Op2furm#428] and stop
    // 000373: copy.f32x4 (I32)
    // --> [Op2furm#428] and stop
    // 000373: copy.f64x2 (I32)
    // --> [Op2furm#428] and stop
    0x00d9, 0x0428,
    // end of copy.f64x2 (I32)
    // end of copy.f32x4 (I32)
    // end of copy.i64x2 (I32)
    // end of copy.i32x4 (I32)
    // end of copy.i16x8 (I32)
    // end of copy.i8x16 (I32)
    // end of copy.b64x2 (I32)
    // end of copy.b32x4 (I32)
    // end of copy.b16x8 (I32)
    // end of copy.b8x16 (I32)
    // end of copy.f32 (I32)
    // end of copy.f64 (I32)
    // end of copy.f64x2 (I64)
    // end of copy.f32x4 (I64)
    // end of copy.i64x2 (I64)
    // end of copy.i32x4 (I64)
    // end of copy.i16x8 (I64)
    // end of copy.i8x16 (I64)
    // end of copy.b64x2 (I64)
    // end of copy.b32x4 (I64)
    // end of copy.b16x8 (I64)
    // end of copy.b8x16 (I64)
    // end of copy.f32 (I64)
    // end of copy.f64 (I64)
    // 000375: copy_to_ssa.f64 (I64)
    // --> [RexMp2furm_reg_to_ssa#710] and stop
    0x0033, 0x0710,
    // end of copy_to_ssa.f64 (I64)
    // 000377: fadd.f64 (I64)
    // --> [RexMp2fa#758]
    0x0138, 0x0758,
    // --> [Mp2fa#758] and stop
    // 000379: fadd.f64 (I32)
    // --> [Mp2fa#758] and stop
    0x0137, 0x0758,
    // end of fadd.f64 (I32)
    // end of fadd.f64 (I64)
    // 00037b: fcmp.f64 (I64)
    // --> [RexMp2fcscc#52e]
    0x0140, 0x052e,
    // --> [Mp2fcscc#52e] and stop
    // 00037d: fcmp.f64 (I32)
    // --> [Mp2fcscc#52e] and stop
    0x013f, 0x052e,
    // end of fcmp.f64 (I32)
    // end of fcmp.f64 (I64)
    // 00037f: fcvt_from_sint.f64 (I64)
    // skip 2 unless inst_predicate_3
    0x3003,
    // --> [DynRexMp2frurm#72a]
    0x0128, 0x072a,
    // stop unless inst_predicate_4
    0x1004,
    // --> [DynRexMp2frurm#872a] and stop
    0x0129, 0x872a,
    // end of fcvt_from_sint.f64 (I64)
    // 000385: fdiv.f64 (I64)
    // --> [RexMp2fa#75e]
    0x0138, 0x075e,
    // --> [Mp2fa#75e] and stop
    // 000387: fdiv.f64 (I32)
    // --> [Mp2fa#75e] and stop
    0x0137, 0x075e,
    // end of fdiv.f64 (I32)
    // end of fdiv.f64 (I64)
    // 000389: ffcmp.f64 (I64)
    // --> [RexMp2fcmp#52e]
    0x0148, 0x052e,
    // --> [Mp2fcmp#52e] and stop
    // 00038b: ffcmp.f64 (I32)
    // --> [Mp2fcmp#52e] and stop
    0x0147, 0x052e,
    // end of ffcmp.f64 (I32)
    // end of ffcmp.f64 (I64)
    // 00038d: fill.f64 (I64)
    // --> [RexMp2ffillSib32#710]
    0x0112, 0x0710,
    // --> [Mp2ffillSib32#710] and stop
    // 00038f: fill.f64 (I32)
    // --> [Mp2ffillSib32#710] and stop
    0x0111, 0x0710,
    // end of fill.f64 (I32)
    // end of fill.f64 (I64)
    // 000391: fill_nop.f64 (I64)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.f32 (I64)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.b8x16 (I64)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.b16x8 (I64)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.b32x4 (I64)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.b64x2 (I64)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.i8x16 (I64)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.i16x8 (I64)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.i32x4 (I64)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.i64x2 (I64)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.f32x4 (I64)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.f64x2 (I64)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.f64 (I32)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.f32 (I32)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.b8x16 (I32)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.b16x8 (I32)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.b32x4 (I32)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.b64x2 (I32)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.i8x16 (I32)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.i16x8 (I32)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.i32x4 (I32)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.i64x2 (I32)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.f32x4 (I32)
    // --> [ffillnull#00] and stop
    // 000391: fill_nop.f64x2 (I32)
    // --> [ffillnull#00] and stop
    0x00b9, 0x0000,
    // end of fill_nop.f64x2 (I32)
    // end of fill_nop.f32x4 (I32)
    // end of fill_nop.i64x2 (I32)
    // end of fill_nop.i32x4 (I32)
    // end of fill_nop.i16x8 (I32)
    // end of fill_nop.i8x16 (I32)
    // end of fill_nop.b64x2 (I32)
    // end of fill_nop.b32x4 (I32)
    // end of fill_nop.b16x8 (I32)
    // end of fill_nop.b8x16 (I32)
    // end of fill_nop.f32 (I32)
    // end of fill_nop.f64 (I32)
    // end of fill_nop.f64x2 (I64)
    // end of fill_nop.f32x4 (I64)
    // end of fill_nop.i64x2 (I64)
    // end of fill_nop.i32x4 (I64)
    // end of fill_nop.i16x8 (I64)
    // end of fill_nop.i8x16 (I64)
    // end of fill_nop.b64x2 (I64)
    // end of fill_nop.b32x4 (I64)
    // end of fill_nop.b16x8 (I64)
    // end of fill_nop.b8x16 (I64)
    // end of fill_nop.f32 (I64)
    // end of fill_nop.f64 (I64)
    // 000393: fmul.f64 (I64)
    // --> [RexMp2fa#759]
    0x0138, 0x0759,
    // --> [Mp2fa#759] and stop
    // 000395: fmul.f64 (I32)
    // --> [Mp2fa#759] and stop
    0x0137, 0x0759,
    // end of fmul.f64 (I32)
    // end of fmul.f64 (I64)
    // 000397: fpromote.f64 (I64)
    // stop unless inst_predicate_10
    0x100a,
    // --> [RexMp2furm#65a]
    0x012c, 0x065a,
    // --> [Mp2furm#65a] and stop
    0x012b, 0x065a,
    // end of fpromote.f64 (I64)
    // 00039c: fsub.f64 (I64)
    // --> [RexMp2fa#75c]
    0x0138, 0x075c,
    // --> [Mp2fa#75c] and stop
    // 00039e: fsub.f64 (I32)
    // --> [Mp2fa#75c] and stop
    0x0137, 0x075c,
    // end of fsub.f64 (I32)
    // end of fsub.f64 (I64)
    // 0003a0: load.f64 (I64)
    // --> [RexMp2fld#710]
    0x00e2, 0x0710,
    // --> [Mp2fld#710]
    0x00e0, 0x0710,
    // --> [RexMp2fldDisp8#710]
    0x00e6, 0x0710,
    // --> [Mp2fldDisp8#710]
    0x00e4, 0x0710,
    // --> [RexMp2fldDisp32#710]
    0x00ea, 0x0710,
    // --> [Mp2fldDisp32#710] and stop
    0x00e9, 0x0710,
    // end of load.f64 (I64)
    // 0003ac: load_complex.f64 (I64)
    // --> [RexMp2fldWithIndex#710]
    0x00ee, 0x0710,
    // --> [Mp2fldWithIndex#710]
    0x00ec, 0x0710,
    // --> [RexMp2fldWithIndexDisp8#710]
    0x00f2, 0x0710,
    // --> [Mp2fldWithIndexDisp8#710]
    0x00f0, 0x0710,
    // --> [RexMp2fldWithIndexDisp32#710]
    0x00f6, 0x0710,
    // --> [Mp2fldWithIndexDisp32#710] and stop
    0x00f5, 0x0710,
    // end of load_complex.f64 (I64)
    // 0003b8: raw_bitcast.f64 (I64)
    // skip 2 unless inst_predicate_14
    // 0003b8: raw_bitcast.f32 (I64)
    // skip 2 unless inst_predicate_14
    // 0003b8: raw_bitcast.f64 (I32)
    // skip 2 unless inst_predicate_14
    // 0003b8: raw_bitcast.f32 (I32)
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_15
    // skip 2 unless inst_predicate_15
    // skip 2 unless inst_predicate_15
    // skip 2 unless inst_predicate_15
    0x300f,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_16
    // skip 2 unless inst_predicate_16
    // skip 2 unless inst_predicate_16
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // stop unless inst_predicate_23
    // stop unless inst_predicate_23
    // stop unless inst_predicate_23
    // stop unless inst_predicate_23
    0x1017,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    // 0003d4: scalar_to_vector.f32x4 (I64)
    // --> [null_fpr#00] and stop
    // 0003d4: scalar_to_vector.f64x2 (I64)
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    // 0003d4: scalar_to_vector.f32x4 (I32)
    // --> [null_fpr#00] and stop
    // 0003d4: scalar_to_vector.f64x2 (I32)
    // --> [null_fpr#00] and stop
    0x0193, 0x0000,
    // end of scalar_to_vector.f64x2 (I32)
    // end of scalar_to_vector.f32x4 (I32)
    // end of raw_bitcast.f32 (I32)
    // end of raw_bitcast.f64 (I32)
    // end of scalar_to_vector.f64x2 (I64)
    // end of scalar_to_vector.f32x4 (I64)
    // end of raw_bitcast.f32 (I64)
    // end of raw_bitcast.f64 (I64)
    // 0003d6: regfill.f64 (I64)
    // --> [RexMp2fregfill32#710]
    0x0116, 0x0710,
    // --> [Mp2fregfill32#710] and stop
    // 0003d8: regfill.f64 (I32)
    // --> [Mp2fregfill32#710] and stop
    0x0115, 0x0710,
    // end of regfill.f64 (I32)
    // end of regfill.f64 (I64)
    // 0003da: regmove.f64 (I64)
    // --> [RexOp2frmov#428] and stop
    // 0003da: regmove.f32 (I64)
    // --> [RexOp2frmov#428] and stop
    0x00df, 0x0428,
    // end of regmove.f32 (I64)
    // end of regmove.f64 (I64)
    // 0003dc: regspill.f64 (I64)
    // --> [RexMp2fregspill32#711]
    0x011e, 0x0711,
    // --> [Mp2fregspill32#711] and stop
    // 0003de: regspill.f64 (I32)
    // --> [Mp2fregspill32#711] and stop
    0x011d, 0x0711,
    // end of regspill.f64 (I32)
    // end of regspill.f64 (I64)
    // 0003e0: spill.f64 (I64)
    // --> [RexMp2fspillSib32#711]
    0x011a, 0x0711,
    // --> [Mp2fspillSib32#711] and stop
    // 0003e2: spill.f64 (I32)
    // --> [Mp2fspillSib32#711] and stop
    0x0119, 0x0711,
    // end of spill.f64 (I32)
    // end of spill.f64 (I64)
    // 0003e4: sqrt.f64 (I64)
    // --> [RexMp2furm#751]
    0x012c, 0x0751,
    // --> [Mp2furm#751] and stop
    // 0003e6: sqrt.f64 (I32)
    // --> [Mp2furm#751] and stop
    0x012b, 0x0751,
    // end of sqrt.f64 (I32)
    // end of sqrt.f64 (I64)
    // 0003e8: store.f64 (I64)
    // --> [RexMp2fst#711]
    0x00fa, 0x0711,
    // --> [Mp2fst#711]
    0x00f8, 0x0711,
    // --> [RexMp2fstDisp8#711]
    0x00fe, 0x0711,
    // --> [Mp2fstDisp8#711]
    0x00fc, 0x0711,
    // --> [RexMp2fstDisp32#711]
    0x0102, 0x0711,
    // --> [Mp2fstDisp32#711] and stop
    0x0101, 0x0711,
    // end of store.f64 (I64)
    // 0003f4: store_complex.f64 (I64)
    // --> [RexMp2fstWithIndex#711]
    0x0106, 0x0711,
    // --> [Mp2fstWithIndex#711]
    0x0104, 0x0711,
    // --> [RexMp2fstWithIndexDisp8#711]
    0x010a, 0x0711,
    // --> [Mp2fstWithIndexDisp8#711]
    0x0108, 0x0711,
    // --> [RexMp2fstWithIndexDisp32#711]
    0x010e, 0x0711,
    // --> [Mp2fstWithIndexDisp32#711] and stop
    0x010d, 0x0711,
    // end of store_complex.f64 (I64)
    // 000400: x86_fmax.f64 (I64)
    // --> [RexMp2fa#75f]
    0x0138, 0x075f,
    // --> [Mp2fa#75f] and stop
    // 000402: x86_fmax.f64 (I32)
    // --> [Mp2fa#75f] and stop
    0x0137, 0x075f,
    // end of x86_fmax.f64 (I32)
    // end of x86_fmax.f64 (I64)
    // 000404: x86_fmin.f64 (I64)
    // --> [RexMp2fa#75d]
    0x0138, 0x075d,
    // --> [Mp2fa#75d] and stop
    // 000406: x86_fmin.f64 (I32)
    // --> [Mp2fa#75d] and stop
    0x0137, 0x075d,
    // end of x86_fmin.f64 (I32)
    // end of x86_fmin.f64 (I64)
    // 000408: bitcast.f32 (I64)
    // stop unless inst_predicate_3
    0x1003,
    // --> [RexMp2frurm#56e]
    // 000409: scalar_to_vector.b8x16 (I64)
    // --> [RexMp2frurm#56e]
    // 000409: scalar_to_vector.b16x8 (I64)
    // --> [RexMp2frurm#56e]
    // 000409: scalar_to_vector.b32x4 (I64)
    // --> [RexMp2frurm#56e]
    // 000409: scalar_to_vector.i8x16 (I64)
    // --> [RexMp2frurm#56e]
    // 000409: scalar_to_vector.i16x8 (I64)
    // --> [RexMp2frurm#56e]
    // 000409: scalar_to_vector.i32x4 (I64)
    // --> [RexMp2frurm#56e]
    0x00d2, 0x056e,
    // --> [Mp2frurm#56e] and stop
    // --> [Mp2frurm#56e] and stop
    // --> [Mp2frurm#56e] and stop
    // --> [Mp2frurm#56e] and stop
    // --> [Mp2frurm#56e] and stop
    // --> [Mp2frurm#56e] and stop
    // --> [Mp2frurm#56e] and stop
    // 00040b: scalar_to_vector.b8x16 (I32)
    // --> [Mp2frurm#56e] and stop
    // 00040b: scalar_to_vector.b16x8 (I32)
    // --> [Mp2frurm#56e] and stop
    // 00040b: scalar_to_vector.b32x4 (I32)
    // --> [Mp2frurm#56e] and stop
    // 00040b: scalar_to_vector.i8x16 (I32)
    // --> [Mp2frurm#56e] and stop
    // 00040b: scalar_to_vector.i16x8 (I32)
    // --> [Mp2frurm#56e] and stop
    // 00040b: scalar_to_vector.i32x4 (I32)
    // --> [Mp2frurm#56e] and stop
    0x00d1, 0x056e,
    // end of scalar_to_vector.i32x4 (I32)
    // end of scalar_to_vector.i16x8 (I32)
    // end of scalar_to_vector.i8x16 (I32)
    // end of scalar_to_vector.b32x4 (I32)
    // end of scalar_to_vector.b16x8 (I32)
    // end of scalar_to_vector.b8x16 (I32)
    // end of scalar_to_vector.i32x4 (I64)
    // end of scalar_to_vector.i16x8 (I64)
    // end of scalar_to_vector.i8x16 (I64)
    // end of scalar_to_vector.b32x4 (I64)
    // end of scalar_to_vector.b16x8 (I64)
    // end of scalar_to_vector.b8x16 (I64)
    // end of bitcast.f32 (I64)
    // 00040d: ceil.f32 (I64)
    // stop unless PredicateView(16)
    // 00040d: floor.f32 (I64)
    // stop unless PredicateView(16)
    // 00040d: nearest.f32 (I64)
    // stop unless PredicateView(16)
    // 00040d: trunc.f32 (I64)
    // stop unless PredicateView(16)
    0x102f,
    // --> [RexMp3furmi_rnd#d0a]
    // --> [RexMp3furmi_rnd#d0a]
    // --> [RexMp3furmi_rnd#d0a]
    // --> [RexMp3furmi_rnd#d0a]
    0x0134, 0x0d0a,
    // --> [Mp3furmi_rnd#d0a] and stop
    // --> [Mp3furmi_rnd#d0a] and stop
    // --> [Mp3furmi_rnd#d0a] and stop
    // --> [Mp3furmi_rnd#d0a] and stop
    0x0133, 0x0d0a,
    // end of trunc.f32 (I64)
    // end of nearest.f32 (I64)
    // end of floor.f32 (I64)
    // end of ceil.f32 (I64)
    // 000412: copy_to_ssa.f32 (I64)
    // --> [RexMp2furm_reg_to_ssa#610] and stop
    0x0033, 0x0610,
    // end of copy_to_ssa.f32 (I64)
    // 000414: fadd.f32 (I64)
    // --> [RexMp2fa#658]
    0x0138, 0x0658,
    // --> [Mp2fa#658] and stop
    // 000416: fadd.f32 (I32)
    // --> [Mp2fa#658] and stop
    0x0137, 0x0658,
    // end of fadd.f32 (I32)
    // end of fadd.f32 (I64)
    // 000418: fcmp.f32 (I64)
    // --> [RexOp2fcscc#42e]
    0x013c, 0x042e,
    // --> [Op2fcscc#42e] and stop
    // 00041a: fcmp.f32 (I32)
    // --> [Op2fcscc#42e] and stop
    0x013b, 0x042e,
    // end of fcmp.f32 (I32)
    // end of fcmp.f32 (I64)
    // 00041c: fcvt_from_sint.f32 (I64)
    // skip 2 unless inst_predicate_3
    0x3003,
    // --> [DynRexMp2frurm#62a]
    0x0128, 0x062a,
    // stop unless inst_predicate_4
    0x1004,
    // --> [DynRexMp2frurm#862a] and stop
    0x0129, 0x862a,
    // end of fcvt_from_sint.f32 (I64)
    // 000422: fdemote.f32 (I64)
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexMp2furm#75a]
    0x012c, 0x075a,
    // --> [Mp2furm#75a] and stop
    0x012b, 0x075a,
    // end of fdemote.f32 (I64)
    // 000427: fdiv.f32 (I64)
    // --> [RexMp2fa#65e]
    0x0138, 0x065e,
    // --> [Mp2fa#65e] and stop
    // 000429: fdiv.f32 (I32)
    // --> [Mp2fa#65e] and stop
    0x0137, 0x065e,
    // end of fdiv.f32 (I32)
    // end of fdiv.f32 (I64)
    // 00042b: ffcmp.f32 (I64)
    // --> [RexOp2fcmp#42e]
    0x0144, 0x042e,
    // --> [Op2fcmp#42e] and stop
    // 00042d: ffcmp.f32 (I32)
    // --> [Op2fcmp#42e] and stop
    0x0143, 0x042e,
    // end of ffcmp.f32 (I32)
    // end of ffcmp.f32 (I64)
    // 00042f: fill.f32 (I64)
    // --> [RexMp2ffillSib32#610]
    0x0112, 0x0610,
    // --> [Mp2ffillSib32#610] and stop
    // 000431: fill.f32 (I32)
    // --> [Mp2ffillSib32#610] and stop
    0x0111, 0x0610,
    // end of fill.f32 (I32)
    // end of fill.f32 (I64)
    // 000433: fmul.f32 (I64)
    // --> [RexMp2fa#659]
    0x0138, 0x0659,
    // --> [Mp2fa#659] and stop
    // 000435: fmul.f32 (I32)
    // --> [Mp2fa#659] and stop
    0x0137, 0x0659,
    // end of fmul.f32 (I32)
    // end of fmul.f32 (I64)
    // 000437: fsub.f32 (I64)
    // --> [RexMp2fa#65c]
    0x0138, 0x065c,
    // --> [Mp2fa#65c] and stop
    // 000439: fsub.f32 (I32)
    // --> [Mp2fa#65c] and stop
    0x0137, 0x065c,
    // end of fsub.f32 (I32)
    // end of fsub.f32 (I64)
    // 00043b: load.f32 (I64)
    // --> [RexMp2fld#610]
    0x00e2, 0x0610,
    // --> [Mp2fld#610]
    0x00e0, 0x0610,
    // --> [RexMp2fldDisp8#610]
    0x00e6, 0x0610,
    // --> [Mp2fldDisp8#610]
    0x00e4, 0x0610,
    // --> [RexMp2fldDisp32#610]
    0x00ea, 0x0610,
    // --> [Mp2fldDisp32#610] and stop
    0x00e9, 0x0610,
    // end of load.f32 (I64)
    // 000447: load_complex.f32 (I64)
    // --> [RexMp2fldWithIndex#610]
    0x00ee, 0x0610,
    // --> [Mp2fldWithIndex#610]
    0x00ec, 0x0610,
    // --> [RexMp2fldWithIndexDisp8#610]
    0x00f2, 0x0610,
    // --> [Mp2fldWithIndexDisp8#610]
    0x00f0, 0x0610,
    // --> [RexMp2fldWithIndexDisp32#610]
    0x00f6, 0x0610,
    // --> [Mp2fldWithIndexDisp32#610] and stop
    0x00f5, 0x0610,
    // end of load_complex.f32 (I64)
    // 000453: regfill.f32 (I64)
    // --> [RexMp2fregfill32#610]
    0x0116, 0x0610,
    // --> [Mp2fregfill32#610] and stop
    // 000455: regfill.f32 (I32)
    // --> [Mp2fregfill32#610] and stop
    0x0115, 0x0610,
    // end of regfill.f32 (I32)
    // end of regfill.f32 (I64)
    // 000457: regspill.f32 (I64)
    // --> [RexMp2fregspill32#611]
    0x011e, 0x0611,
    // --> [Mp2fregspill32#611] and stop
    // 000459: regspill.f32 (I32)
    // --> [Mp2fregspill32#611] and stop
    0x011d, 0x0611,
    // end of regspill.f32 (I32)
    // end of regspill.f32 (I64)
    // 00045b: spill.f32 (I64)
    // --> [RexMp2fspillSib32#611]
    0x011a, 0x0611,
    // --> [Mp2fspillSib32#611] and stop
    // 00045d: spill.f32 (I32)
    // --> [Mp2fspillSib32#611] and stop
    0x0119, 0x0611,
    // end of spill.f32 (I32)
    // end of spill.f32 (I64)
    // 00045f: sqrt.f32 (I64)
    // --> [RexMp2furm#651]
    0x012c, 0x0651,
    // --> [Mp2furm#651] and stop
    // 000461: sqrt.f32 (I32)
    // --> [Mp2furm#651] and stop
    0x012b, 0x0651,
    // end of sqrt.f32 (I32)
    // end of sqrt.f32 (I64)
    // 000463: store.f32 (I64)
    // --> [RexMp2fst#611]
    0x00fa, 0x0611,
    // --> [Mp2fst#611]
    0x00f8, 0x0611,
    // --> [RexMp2fstDisp8#611]
    0x00fe, 0x0611,
    // --> [Mp2fstDisp8#611]
    0x00fc, 0x0611,
    // --> [RexMp2fstDisp32#611]
    0x0102, 0x0611,
    // --> [Mp2fstDisp32#611] and stop
    0x0101, 0x0611,
    // end of store.f32 (I64)
    // 00046f: store_complex.f32 (I64)
    // --> [RexMp2fstWithIndex#611]
    0x0106, 0x0611,
    // --> [Mp2fstWithIndex#611]
    0x0104, 0x0611,
    // --> [RexMp2fstWithIndexDisp8#611]
    0x010a, 0x0611,
    // --> [Mp2fstWithIndexDisp8#611]
    0x0108, 0x0611,
    // --> [RexMp2fstWithIndexDisp32#611]
    0x010e, 0x0611,
    // --> [Mp2fstWithIndexDisp32#611] and stop
    0x010d, 0x0611,
    // end of store_complex.f32 (I64)
    // 00047b: x86_fmax.f32 (I64)
    // --> [RexMp2fa#65f]
    0x0138, 0x065f,
    // --> [Mp2fa#65f] and stop
    // 00047d: x86_fmax.f32 (I32)
    // --> [Mp2fa#65f] and stop
    0x0137, 0x065f,
    // end of x86_fmax.f32 (I32)
    // end of x86_fmax.f32 (I64)
    // 00047f: x86_fmin.f32 (I64)
    // --> [RexMp2fa#65d]
    0x0138, 0x065d,
    // --> [Mp2fa#65d] and stop
    // 000481: x86_fmin.f32 (I32)
    // --> [Mp2fa#65d] and stop
    0x0137, 0x065d,
    // end of x86_fmin.f32 (I32)
    // end of x86_fmin.f32 (I64)
    // 000483: band.b8x16 (I64)
    // --> [Mp2fa#5db] and stop
    // 000483: band.b16x8 (I64)
    // --> [Mp2fa#5db] and stop
    // 000483: band.b32x4 (I64)
    // --> [Mp2fa#5db] and stop
    // 000483: band.b64x2 (I64)
    // --> [Mp2fa#5db] and stop
    // 000483: band.i8x16 (I64)
    // --> [Mp2fa#5db] and stop
    // 000483: band.i16x8 (I64)
    // --> [Mp2fa#5db] and stop
    // 000483: band.i32x4 (I64)
    // --> [Mp2fa#5db] and stop
    // 000483: band.i64x2 (I64)
    // --> [Mp2fa#5db] and stop
    // 000483: band.f32x4 (I64)
    // --> [Mp2fa#5db] and stop
    // 000483: band.f64x2 (I64)
    // --> [Mp2fa#5db] and stop
    // 000483: band.b8x16 (I32)
    // --> [Mp2fa#5db] and stop
    // 000483: band.b16x8 (I32)
    // --> [Mp2fa#5db] and stop
    // 000483: band.b32x4 (I32)
    // --> [Mp2fa#5db] and stop
    // 000483: band.b64x2 (I32)
    // --> [Mp2fa#5db] and stop
    // 000483: band.i8x16 (I32)
    // --> [Mp2fa#5db] and stop
    // 000483: band.i16x8 (I32)
    // --> [Mp2fa#5db] and stop
    // 000483: band.i32x4 (I32)
    // --> [Mp2fa#5db] and stop
    // 000483: band.i64x2 (I32)
    // --> [Mp2fa#5db] and stop
    // 000483: band.f32x4 (I32)
    // --> [Mp2fa#5db] and stop
    // 000483: band.f64x2 (I32)
    // --> [Mp2fa#5db] and stop
    0x0137, 0x05db,
    // end of band.f64x2 (I32)
    // end of band.f32x4 (I32)
    // end of band.i64x2 (I32)
    // end of band.i32x4 (I32)
    // end of band.i16x8 (I32)
    // end of band.i8x16 (I32)
    // end of band.b64x2 (I32)
    // end of band.b32x4 (I32)
    // end of band.b16x8 (I32)
    // end of band.b8x16 (I32)
    // end of band.f64x2 (I64)
    // end of band.f32x4 (I64)
    // end of band.i64x2 (I64)
    // end of band.i32x4 (I64)
    // end of band.i16x8 (I64)
    // end of band.i8x16 (I64)
    // end of band.b64x2 (I64)
    // end of band.b32x4 (I64)
    // end of band.b16x8 (I64)
    // end of band.b8x16 (I64)
    // 000485: band_not.b8x16 (I64)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.b16x8 (I64)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.b32x4 (I64)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.b64x2 (I64)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.i8x16 (I64)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.i16x8 (I64)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.i32x4 (I64)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.i64x2 (I64)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.f32x4 (I64)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.f64x2 (I64)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.b8x16 (I32)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.b16x8 (I32)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.b32x4 (I32)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.b64x2 (I32)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.i8x16 (I32)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.i16x8 (I32)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.i32x4 (I32)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.i64x2 (I32)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.f32x4 (I32)
    // --> [Mp2fax#5df] and stop
    // 000485: band_not.f64x2 (I32)
    // --> [Mp2fax#5df] and stop
    0x01b9, 0x05df,
    // end of band_not.f64x2 (I32)
    // end of band_not.f32x4 (I32)
    // end of band_not.i64x2 (I32)
    // end of band_not.i32x4 (I32)
    // end of band_not.i16x8 (I32)
    // end of band_not.i8x16 (I32)
    // end of band_not.b64x2 (I32)
    // end of band_not.b32x4 (I32)
    // end of band_not.b16x8 (I32)
    // end of band_not.b8x16 (I32)
    // end of band_not.f64x2 (I64)
    // end of band_not.f32x4 (I64)
    // end of band_not.i64x2 (I64)
    // end of band_not.i32x4 (I64)
    // end of band_not.i16x8 (I64)
    // end of band_not.i8x16 (I64)
    // end of band_not.b64x2 (I64)
    // end of band_not.b32x4 (I64)
    // end of band_not.b16x8 (I64)
    // end of band_not.b8x16 (I64)
    // 000487: bor.b8x16 (I64)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.b16x8 (I64)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.b32x4 (I64)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.b64x2 (I64)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.i8x16 (I64)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.i16x8 (I64)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.i32x4 (I64)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.i64x2 (I64)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.f32x4 (I64)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.f64x2 (I64)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.b8x16 (I32)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.b16x8 (I32)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.b32x4 (I32)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.b64x2 (I32)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.i8x16 (I32)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.i16x8 (I32)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.i32x4 (I32)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.i64x2 (I32)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.f32x4 (I32)
    // --> [Mp2fa#5eb] and stop
    // 000487: bor.f64x2 (I32)
    // --> [Mp2fa#5eb] and stop
    0x0137, 0x05eb,
    // end of bor.f64x2 (I32)
    // end of bor.f32x4 (I32)
    // end of bor.i64x2 (I32)
    // end of bor.i32x4 (I32)
    // end of bor.i16x8 (I32)
    // end of bor.i8x16 (I32)
    // end of bor.b64x2 (I32)
    // end of bor.b32x4 (I32)
    // end of bor.b16x8 (I32)
    // end of bor.b8x16 (I32)
    // end of bor.f64x2 (I64)
    // end of bor.f32x4 (I64)
    // end of bor.i64x2 (I64)
    // end of bor.i32x4 (I64)
    // end of bor.i16x8 (I64)
    // end of bor.i8x16 (I64)
    // end of bor.b64x2 (I64)
    // end of bor.b32x4 (I64)
    // end of bor.b16x8 (I64)
    // end of bor.b8x16 (I64)
    // 000489: bxor.b8x16 (I64)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.b16x8 (I64)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.b32x4 (I64)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.b64x2 (I64)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.i8x16 (I64)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.i16x8 (I64)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.i32x4 (I64)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.i64x2 (I64)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.f32x4 (I64)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.f64x2 (I64)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.b8x16 (I32)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.b16x8 (I32)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.b32x4 (I32)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.b64x2 (I32)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.i8x16 (I32)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.i16x8 (I32)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.i32x4 (I32)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.i64x2 (I32)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.f32x4 (I32)
    // --> [Mp2fa#5ef] and stop
    // 000489: bxor.f64x2 (I32)
    // --> [Mp2fa#5ef] and stop
    0x0137, 0x05ef,
    // end of bxor.f64x2 (I32)
    // end of bxor.f32x4 (I32)
    // end of bxor.i64x2 (I32)
    // end of bxor.i32x4 (I32)
    // end of bxor.i16x8 (I32)
    // end of bxor.i8x16 (I32)
    // end of bxor.b64x2 (I32)
    // end of bxor.b32x4 (I32)
    // end of bxor.b16x8 (I32)
    // end of bxor.b8x16 (I32)
    // end of bxor.f64x2 (I64)
    // end of bxor.f32x4 (I64)
    // end of bxor.i64x2 (I64)
    // end of bxor.i32x4 (I64)
    // end of bxor.i16x8 (I64)
    // end of bxor.i8x16 (I64)
    // end of bxor.b64x2 (I64)
    // end of bxor.b32x4 (I64)
    // end of bxor.b16x8 (I64)
    // end of bxor.b8x16 (I64)
    // 00048b: fill.b8x16 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.b16x8 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.b32x4 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.b64x2 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.i8x16 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.i16x8 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.i32x4 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.i64x2 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.f32x4 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.f64x2 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.b8x16 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.b16x8 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.b32x4 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.b64x2 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.i8x16 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.i16x8 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.i32x4 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.i64x2 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.f32x4 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 00048b: fill.f64x2 (I32)
    // --> [Op2ffillSib32#410] and stop
    0x01b5, 0x0410,
    // end of fill.f64x2 (I32)
    // end of fill.f32x4 (I32)
    // end of fill.i64x2 (I32)
    // end of fill.i32x4 (I32)
    // end of fill.i16x8 (I32)
    // end of fill.i8x16 (I32)
    // end of fill.b64x2 (I32)
    // end of fill.b32x4 (I32)
    // end of fill.b16x8 (I32)
    // end of fill.b8x16 (I32)
    // end of fill.f64x2 (I64)
    // end of fill.f32x4 (I64)
    // end of fill.i64x2 (I64)
    // end of fill.i32x4 (I64)
    // end of fill.i16x8 (I64)
    // end of fill.i8x16 (I64)
    // end of fill.b64x2 (I64)
    // end of fill.b32x4 (I64)
    // end of fill.b16x8 (I64)
    // end of fill.b8x16 (I64)
    // 00048d: load.b8x16 (I64)
    // --> [DynRexOp2fld#410]
    // 00048d: load.b16x8 (I64)
    // --> [DynRexOp2fld#410]
    // 00048d: load.b32x4 (I64)
    // --> [DynRexOp2fld#410]
    // 00048d: load.b64x2 (I64)
    // --> [DynRexOp2fld#410]
    // 00048d: load.i8x16 (I64)
    // --> [DynRexOp2fld#410]
    // 00048d: load.i16x8 (I64)
    // --> [DynRexOp2fld#410]
    // 00048d: load.i32x4 (I64)
    // --> [DynRexOp2fld#410]
    // 00048d: load.i64x2 (I64)
    // --> [DynRexOp2fld#410]
    // 00048d: load.f32x4 (I64)
    // --> [DynRexOp2fld#410]
    // 00048d: load.f64x2 (I64)
    // --> [DynRexOp2fld#410]
    // 00048d: load.b8x16 (I32)
    // --> [DynRexOp2fld#410]
    // 00048d: load.b16x8 (I32)
    // --> [DynRexOp2fld#410]
    // 00048d: load.b32x4 (I32)
    // --> [DynRexOp2fld#410]
    // 00048d: load.b64x2 (I32)
    // --> [DynRexOp2fld#410]
    // 00048d: load.i8x16 (I32)
    // --> [DynRexOp2fld#410]
    // 00048d: load.i16x8 (I32)
    // --> [DynRexOp2fld#410]
    // 00048d: load.i32x4 (I32)
    // --> [DynRexOp2fld#410]
    // 00048d: load.i64x2 (I32)
    // --> [DynRexOp2fld#410]
    // 00048d: load.f32x4 (I32)
    // --> [DynRexOp2fld#410]
    // 00048d: load.f64x2 (I32)
    // --> [DynRexOp2fld#410]
    0x01aa, 0x0410,
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    0x01ac, 0x0410,
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    0x01af, 0x0410,
    // end of load.f64x2 (I32)
    // end of load.f32x4 (I32)
    // end of load.i64x2 (I32)
    // end of load.i32x4 (I32)
    // end of load.i16x8 (I32)
    // end of load.i8x16 (I32)
    // end of load.b64x2 (I32)
    // end of load.b32x4 (I32)
    // end of load.b16x8 (I32)
    // end of load.b8x16 (I32)
    // end of load.f64x2 (I64)
    // end of load.f32x4 (I64)
    // end of load.i64x2 (I64)
    // end of load.i32x4 (I64)
    // end of load.i16x8 (I64)
    // end of load.i8x16 (I64)
    // end of load.b64x2 (I64)
    // end of load.b32x4 (I64)
    // end of load.b16x8 (I64)
    // end of load.b8x16 (I64)
    // 000493: raw_bitcast.b8x16 (I64)
    // skip 2 unless inst_predicate_15
    // 000493: raw_bitcast.b8x16 (I32)
    // skip 2 unless inst_predicate_15
    0x300f,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_16
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_10
    // skip 2 unless inst_predicate_10
    0x300a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // stop unless inst_predicate_11
    // stop unless inst_predicate_11
    0x100b,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x0193, 0x0000,
    // end of raw_bitcast.b8x16 (I32)
    // end of raw_bitcast.b8x16 (I64)
    // 0004b4: regfill.b8x16 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.b16x8 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.b32x4 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.b64x2 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.i8x16 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.i16x8 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.i32x4 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.i64x2 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.f32x4 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.f64x2 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.b8x16 (I32)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.b16x8 (I32)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.b32x4 (I32)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.b64x2 (I32)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.i8x16 (I32)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.i16x8 (I32)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.i32x4 (I32)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.i64x2 (I32)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.f32x4 (I32)
    // --> [Op2fregfill32#410] and stop
    // 0004b4: regfill.f64x2 (I32)
    // --> [Op2fregfill32#410] and stop
    0x01b7, 0x0410,
    // end of regfill.f64x2 (I32)
    // end of regfill.f32x4 (I32)
    // end of regfill.i64x2 (I32)
    // end of regfill.i32x4 (I32)
    // end of regfill.i16x8 (I32)
    // end of regfill.i8x16 (I32)
    // end of regfill.b64x2 (I32)
    // end of regfill.b32x4 (I32)
    // end of regfill.b16x8 (I32)
    // end of regfill.b8x16 (I32)
    // end of regfill.f64x2 (I64)
    // end of regfill.f32x4 (I64)
    // end of regfill.i64x2 (I64)
    // end of regfill.i32x4 (I64)
    // end of regfill.i16x8 (I64)
    // end of regfill.i8x16 (I64)
    // end of regfill.b64x2 (I64)
    // end of regfill.b32x4 (I64)
    // end of regfill.b16x8 (I64)
    // end of regfill.b8x16 (I64)
    // 0004b6: regmove.b8x16 (I64)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.b16x8 (I64)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.b32x4 (I64)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.b64x2 (I64)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.i8x16 (I64)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.i16x8 (I64)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.i32x4 (I64)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.i64x2 (I64)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.f32x4 (I64)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.f64x2 (I64)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.f64 (I32)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.f32 (I32)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.b8x16 (I32)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.b16x8 (I32)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.b32x4 (I32)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.b64x2 (I32)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.i8x16 (I32)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.i16x8 (I32)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.i32x4 (I32)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.i64x2 (I32)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.f32x4 (I32)
    // --> [Op2frmov#428] and stop
    // 0004b6: regmove.f64x2 (I32)
    // --> [Op2frmov#428] and stop
    0x00dd, 0x0428,
    // end of regmove.f64x2 (I32)
    // end of regmove.f32x4 (I32)
    // end of regmove.i64x2 (I32)
    // end of regmove.i32x4 (I32)
    // end of regmove.i16x8 (I32)
    // end of regmove.i8x16 (I32)
    // end of regmove.b64x2 (I32)
    // end of regmove.b32x4 (I32)
    // end of regmove.b16x8 (I32)
    // end of regmove.b8x16 (I32)
    // end of regmove.f32 (I32)
    // end of regmove.f64 (I32)
    // end of regmove.f64x2 (I64)
    // end of regmove.f32x4 (I64)
    // end of regmove.i64x2 (I64)
    // end of regmove.i32x4 (I64)
    // end of regmove.i16x8 (I64)
    // end of regmove.i8x16 (I64)
    // end of regmove.b64x2 (I64)
    // end of regmove.b32x4 (I64)
    // end of regmove.b16x8 (I64)
    // end of regmove.b8x16 (I64)
    // 0004b8: regspill.b8x16 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.b16x8 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.b32x4 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.b64x2 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.i8x16 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.i16x8 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.i32x4 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.i64x2 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.f32x4 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.f64x2 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.b8x16 (I32)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.b16x8 (I32)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.b32x4 (I32)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.b64x2 (I32)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.i8x16 (I32)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.i16x8 (I32)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.i32x4 (I32)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.i64x2 (I32)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.f32x4 (I32)
    // --> [Op2fregspill32#411] and stop
    // 0004b8: regspill.f64x2 (I32)
    // --> [Op2fregspill32#411] and stop
    0x01b3, 0x0411,
    // end of regspill.f64x2 (I32)
    // end of regspill.f32x4 (I32)
    // end of regspill.i64x2 (I32)
    // end of regspill.i32x4 (I32)
    // end of regspill.i16x8 (I32)
    // end of regspill.i8x16 (I32)
    // end of regspill.b64x2 (I32)
    // end of regspill.b32x4 (I32)
    // end of regspill.b16x8 (I32)
    // end of regspill.b8x16 (I32)
    // end of regspill.f64x2 (I64)
    // end of regspill.f32x4 (I64)
    // end of regspill.i64x2 (I64)
    // end of regspill.i32x4 (I64)
    // end of regspill.i16x8 (I64)
    // end of regspill.i8x16 (I64)
    // end of regspill.b64x2 (I64)
    // end of regspill.b32x4 (I64)
    // end of regspill.b16x8 (I64)
    // end of regspill.b8x16 (I64)
    // 0004ba: spill.b8x16 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.b16x8 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.b32x4 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.b64x2 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.i8x16 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.i16x8 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.i32x4 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.i64x2 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.f32x4 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.f64x2 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.b8x16 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.b16x8 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.b32x4 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.b64x2 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.i8x16 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.i16x8 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.i32x4 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.i64x2 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.f32x4 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 0004ba: spill.f64x2 (I32)
    // --> [Op2fspillSib32#411] and stop
    0x01b1, 0x0411,
    // end of spill.f64x2 (I32)
    // end of spill.f32x4 (I32)
    // end of spill.i64x2 (I32)
    // end of spill.i32x4 (I32)
    // end of spill.i16x8 (I32)
    // end of spill.i8x16 (I32)
    // end of spill.b64x2 (I32)
    // end of spill.b32x4 (I32)
    // end of spill.b16x8 (I32)
    // end of spill.b8x16 (I32)
    // end of spill.f64x2 (I64)
    // end of spill.f32x4 (I64)
    // end of spill.i64x2 (I64)
    // end of spill.i32x4 (I64)
    // end of spill.i16x8 (I64)
    // end of spill.i8x16 (I64)
    // end of spill.b64x2 (I64)
    // end of spill.b32x4 (I64)
    // end of spill.b16x8 (I64)
    // end of spill.b8x16 (I64)
    // 0004bc: store.b8x16 (I64)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.b16x8 (I64)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.b32x4 (I64)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.b64x2 (I64)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.i8x16 (I64)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.i16x8 (I64)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.i32x4 (I64)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.i64x2 (I64)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.f32x4 (I64)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.f64x2 (I64)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.b8x16 (I32)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.b16x8 (I32)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.b32x4 (I32)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.b64x2 (I32)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.i8x16 (I32)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.i16x8 (I32)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.i32x4 (I32)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.i64x2 (I32)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.f32x4 (I32)
    // --> [DynRexOp2fst#411]
    // 0004bc: store.f64x2 (I32)
    // --> [DynRexOp2fst#411]
    0x01a4, 0x0411,
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    0x01a6, 0x0411,
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    0x01a9, 0x0411,
    // end of store.f64x2 (I32)
    // end of store.f32x4 (I32)
    // end of store.i64x2 (I32)
    // end of store.i32x4 (I32)
    // end of store.i16x8 (I32)
    // end of store.i8x16 (I32)
    // end of store.b64x2 (I32)
    // end of store.b32x4 (I32)
    // end of store.b16x8 (I32)
    // end of store.b8x16 (I32)
    // end of store.f64x2 (I64)
    // end of store.f32x4 (I64)
    // end of store.i64x2 (I64)
    // end of store.i32x4 (I64)
    // end of store.i16x8 (I64)
    // end of store.i8x16 (I64)
    // end of store.b64x2 (I64)
    // end of store.b32x4 (I64)
    // end of store.b16x8 (I64)
    // end of store.b8x16 (I64)
    // 0004c2: vconst.b8x16 (I64)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.b16x8 (I64)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.b32x4 (I64)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.b64x2 (I64)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.i8x16 (I64)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.i16x8 (I64)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.i32x4 (I64)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.i64x2 (I64)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.f32x4 (I64)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.f64x2 (I64)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.b8x16 (I32)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.b16x8 (I32)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.b32x4 (I32)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.b64x2 (I32)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.i8x16 (I32)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.i16x8 (I32)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.i32x4 (I32)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.i64x2 (I32)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.f32x4 (I32)
    // skip 2 unless inst_predicate_24
    // 0004c2: vconst.f64x2 (I32)
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    0x01a0, 0x05ef,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    0x01a0, 0x0574,
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    0x01a3, 0x0410,
    // end of vconst.f64x2 (I32)
    // end of vconst.f32x4 (I32)
    // end of vconst.i64x2 (I32)
    // end of vconst.i32x4 (I32)
    // end of vconst.i16x8 (I32)
    // end of vconst.i8x16 (I32)
    // end of vconst.b64x2 (I32)
    // end of vconst.b32x4 (I32)
    // end of vconst.b16x8 (I32)
    // end of vconst.b8x16 (I32)
    // end of vconst.f64x2 (I64)
    // end of vconst.f32x4 (I64)
    // end of vconst.i64x2 (I64)
    // end of vconst.i32x4 (I64)
    // end of vconst.i16x8 (I64)
    // end of vconst.i8x16 (I64)
    // end of vconst.b64x2 (I64)
    // end of vconst.b32x4 (I64)
    // end of vconst.b16x8 (I64)
    // end of vconst.b8x16 (I64)
    // 0004ca: x86_pextr.b8x16 (I64)
    // stop unless PredicateView(17)
    // 0004ca: x86_pextr.i8x16 (I64)
    // stop unless PredicateView(17)
    // 0004ca: x86_pextr.b8x16 (I32)
    // stop unless PredicateView(17)
    // 0004ca: x86_pextr.i8x16 (I32)
    // stop unless PredicateView(17)
    0x1030,
    // --> [Mp3r_ib_unsigned_gpr#d14] and stop
    // --> [Mp3r_ib_unsigned_gpr#d14] and stop
    // --> [Mp3r_ib_unsigned_gpr#d14] and stop
    // --> [Mp3r_ib_unsigned_gpr#d14] and stop
    0x019d, 0x0d14,
    // end of x86_pextr.i8x16 (I32)
    // end of x86_pextr.b8x16 (I32)
    // end of x86_pextr.i8x16 (I64)
    // end of x86_pextr.b8x16 (I64)
    // 0004cd: x86_pinsr.b8x16 (I64)
    // stop unless PredicateView(17)
    // 0004cd: x86_pinsr.i8x16 (I64)
    // stop unless PredicateView(17)
    // 0004cd: x86_pinsr.b8x16 (I32)
    // stop unless PredicateView(17)
    // 0004cd: x86_pinsr.i8x16 (I32)
    // stop unless PredicateView(17)
    0x1030,
    // --> [Mp3r_ib_unsigned_r#d20] and stop
    // --> [Mp3r_ib_unsigned_r#d20] and stop
    // --> [Mp3r_ib_unsigned_r#d20] and stop
    // --> [Mp3r_ib_unsigned_r#d20] and stop
    0x0195, 0x0d20,
    // end of x86_pinsr.i8x16 (I32)
    // end of x86_pinsr.b8x16 (I32)
    // end of x86_pinsr.i8x16 (I64)
    // end of x86_pinsr.b8x16 (I64)
    // 0004d0: x86_pshufb.b8x16 (I64)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.b16x8 (I64)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.b32x4 (I64)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.b64x2 (I64)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.i8x16 (I64)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.i16x8 (I64)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.i32x4 (I64)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.i64x2 (I64)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.f32x4 (I64)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.f64x2 (I64)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.b8x16 (I32)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.b16x8 (I32)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.b32x4 (I32)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.b64x2 (I32)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.i8x16 (I32)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.i16x8 (I32)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.i32x4 (I32)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.i64x2 (I32)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.f32x4 (I32)
    // stop unless PredicateView(21)
    // 0004d0: x86_pshufb.f64x2 (I32)
    // stop unless PredicateView(21)
    0x1034,
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    0x018f, 0x0900,
    // end of x86_pshufb.f64x2 (I32)
    // end of x86_pshufb.f32x4 (I32)
    // end of x86_pshufb.i64x2 (I32)
    // end of x86_pshufb.i32x4 (I32)
    // end of x86_pshufb.i16x8 (I32)
    // end of x86_pshufb.i8x16 (I32)
    // end of x86_pshufb.b64x2 (I32)
    // end of x86_pshufb.b32x4 (I32)
    // end of x86_pshufb.b16x8 (I32)
    // end of x86_pshufb.b8x16 (I32)
    // end of x86_pshufb.f64x2 (I64)
    // end of x86_pshufb.f32x4 (I64)
    // end of x86_pshufb.i64x2 (I64)
    // end of x86_pshufb.i32x4 (I64)
    // end of x86_pshufb.i16x8 (I64)
    // end of x86_pshufb.i8x16 (I64)
    // end of x86_pshufb.b64x2 (I64)
    // end of x86_pshufb.b32x4 (I64)
    // end of x86_pshufb.b16x8 (I64)
    // end of x86_pshufb.b8x16 (I64)
    // 0004d3: x86_ptest.b8x16 (I64)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.b16x8 (I64)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.b32x4 (I64)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.b64x2 (I64)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.i8x16 (I64)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.i16x8 (I64)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.i32x4 (I64)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.i64x2 (I64)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.f32x4 (I64)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.f64x2 (I64)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.b8x16 (I32)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.b16x8 (I32)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.b32x4 (I32)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.b64x2 (I32)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.i8x16 (I32)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.i16x8 (I32)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.i32x4 (I32)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.i64x2 (I32)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.f32x4 (I32)
    // stop unless PredicateView(17)
    // 0004d3: x86_ptest.f64x2 (I32)
    // stop unless PredicateView(17)
    0x1030,
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    0x01bb, 0x0917,
    // end of x86_ptest.f64x2 (I32)
    // end of x86_ptest.f32x4 (I32)
    // end of x86_ptest.i64x2 (I32)
    // end of x86_ptest.i32x4 (I32)
    // end of x86_ptest.i16x8 (I32)
    // end of x86_ptest.i8x16 (I32)
    // end of x86_ptest.b64x2 (I32)
    // end of x86_ptest.b32x4 (I32)
    // end of x86_ptest.b16x8 (I32)
    // end of x86_ptest.b8x16 (I32)
    // end of x86_ptest.f64x2 (I64)
    // end of x86_ptest.f32x4 (I64)
    // end of x86_ptest.i64x2 (I64)
    // end of x86_ptest.i32x4 (I64)
    // end of x86_ptest.i16x8 (I64)
    // end of x86_ptest.i8x16 (I64)
    // end of x86_ptest.b64x2 (I64)
    // end of x86_ptest.b32x4 (I64)
    // end of x86_ptest.b16x8 (I64)
    // end of x86_ptest.b8x16 (I64)
    // 0004d6: raw_bitcast.b16x8 (I64)
    // skip 2 unless inst_predicate_14
    // 0004d6: raw_bitcast.b16x8 (I32)
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_16
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_10
    // skip 2 unless inst_predicate_10
    0x300a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // stop unless inst_predicate_11
    // stop unless inst_predicate_11
    0x100b,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x0193, 0x0000,
    // end of raw_bitcast.b16x8 (I32)
    // end of raw_bitcast.b16x8 (I64)
    // 0004f7: x86_pextr.b16x8 (I64)
    // stop unless PredicateView(17)
    // 0004f7: x86_pextr.i16x8 (I64)
    // stop unless PredicateView(17)
    // 0004f7: x86_pextr.b16x8 (I32)
    // stop unless PredicateView(17)
    // 0004f7: x86_pextr.i16x8 (I32)
    // stop unless PredicateView(17)
    0x1030,
    // --> [Mp3r_ib_unsigned_gpr#d15] and stop
    // --> [Mp3r_ib_unsigned_gpr#d15] and stop
    // --> [Mp3r_ib_unsigned_gpr#d15] and stop
    // --> [Mp3r_ib_unsigned_gpr#d15] and stop
    0x019d, 0x0d15,
    // end of x86_pextr.i16x8 (I32)
    // end of x86_pextr.b16x8 (I32)
    // end of x86_pextr.i16x8 (I64)
    // end of x86_pextr.b16x8 (I64)
    // 0004fa: x86_pinsr.b16x8 (I64)
    // --> [Mp2r_ib_unsigned_r#5c4] and stop
    // 0004fa: x86_pinsr.i16x8 (I64)
    // --> [Mp2r_ib_unsigned_r#5c4] and stop
    // 0004fa: x86_pinsr.b16x8 (I32)
    // --> [Mp2r_ib_unsigned_r#5c4] and stop
    // 0004fa: x86_pinsr.i16x8 (I32)
    // --> [Mp2r_ib_unsigned_r#5c4] and stop
    0x0197, 0x05c4,
    // end of x86_pinsr.i16x8 (I32)
    // end of x86_pinsr.b16x8 (I32)
    // end of x86_pinsr.i16x8 (I64)
    // end of x86_pinsr.b16x8 (I64)
    // 0004fc: raw_bitcast.b32x4 (I64)
    // skip 2 unless inst_predicate_14
    // 0004fc: raw_bitcast.b32x4 (I32)
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_15
    // skip 2 unless inst_predicate_15
    0x300f,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_10
    // skip 2 unless inst_predicate_10
    0x300a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // stop unless inst_predicate_11
    // stop unless inst_predicate_11
    0x100b,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x0193, 0x0000,
    // end of raw_bitcast.b32x4 (I32)
    // end of raw_bitcast.b32x4 (I64)
    // 00051d: x86_pextr.b32x4 (I64)
    // stop unless PredicateView(17)
    // 00051d: x86_pextr.i32x4 (I64)
    // stop unless PredicateView(17)
    // 00051d: x86_pextr.f32x4 (I64)
    // stop unless PredicateView(17)
    // 00051d: x86_pextr.b32x4 (I32)
    // stop unless PredicateView(17)
    // 00051d: x86_pextr.i32x4 (I32)
    // stop unless PredicateView(17)
    // 00051d: x86_pextr.f32x4 (I32)
    // stop unless PredicateView(17)
    0x1030,
    // --> [Mp3r_ib_unsigned_gpr#d16] and stop
    // --> [Mp3r_ib_unsigned_gpr#d16] and stop
    // --> [Mp3r_ib_unsigned_gpr#d16] and stop
    // --> [Mp3r_ib_unsigned_gpr#d16] and stop
    // --> [Mp3r_ib_unsigned_gpr#d16] and stop
    // --> [Mp3r_ib_unsigned_gpr#d16] and stop
    0x019d, 0x0d16,
    // end of x86_pextr.f32x4 (I32)
    // end of x86_pextr.i32x4 (I32)
    // end of x86_pextr.b32x4 (I32)
    // end of x86_pextr.f32x4 (I64)
    // end of x86_pextr.i32x4 (I64)
    // end of x86_pextr.b32x4 (I64)
    // 000520: x86_pinsr.b32x4 (I64)
    // stop unless PredicateView(17)
    // 000520: x86_pinsr.i32x4 (I64)
    // stop unless PredicateView(17)
    // 000520: x86_pinsr.f32x4 (I64)
    // stop unless PredicateView(17)
    // 000520: x86_pinsr.b32x4 (I32)
    // stop unless PredicateView(17)
    // 000520: x86_pinsr.i32x4 (I32)
    // stop unless PredicateView(17)
    // 000520: x86_pinsr.f32x4 (I32)
    // stop unless PredicateView(17)
    0x1030,
    // --> [Mp3r_ib_unsigned_r#d22] and stop
    // --> [Mp3r_ib_unsigned_r#d22] and stop
    // --> [Mp3r_ib_unsigned_r#d22] and stop
    // --> [Mp3r_ib_unsigned_r#d22] and stop
    // --> [Mp3r_ib_unsigned_r#d22] and stop
    // --> [Mp3r_ib_unsigned_r#d22] and stop
    0x0195, 0x0d22,
    // end of x86_pinsr.f32x4 (I32)
    // end of x86_pinsr.i32x4 (I32)
    // end of x86_pinsr.b32x4 (I32)
    // end of x86_pinsr.f32x4 (I64)
    // end of x86_pinsr.i32x4 (I64)
    // end of x86_pinsr.b32x4 (I64)
    // 000523: x86_pshufd.b32x4 (I64)
    // --> [Mp2r_ib_unsigned_fpr#570] and stop
    // 000523: x86_pshufd.i32x4 (I64)
    // --> [Mp2r_ib_unsigned_fpr#570] and stop
    // 000523: x86_pshufd.f32x4 (I64)
    // --> [Mp2r_ib_unsigned_fpr#570] and stop
    // 000523: x86_pshufd.b32x4 (I32)
    // --> [Mp2r_ib_unsigned_fpr#570] and stop
    // 000523: x86_pshufd.i32x4 (I32)
    // --> [Mp2r_ib_unsigned_fpr#570] and stop
    // 000523: x86_pshufd.f32x4 (I32)
    // --> [Mp2r_ib_unsigned_fpr#570] and stop
    0x0191, 0x0570,
    // end of x86_pshufd.f32x4 (I32)
    // end of x86_pshufd.i32x4 (I32)
    // end of x86_pshufd.b32x4 (I32)
    // end of x86_pshufd.f32x4 (I64)
    // end of x86_pshufd.i32x4 (I64)
    // end of x86_pshufd.b32x4 (I64)
    // 000525: raw_bitcast.b64x2 (I64)
    // skip 2 unless inst_predicate_14
    // 000525: raw_bitcast.b64x2 (I32)
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_15
    // skip 2 unless inst_predicate_15
    0x300f,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_16
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_10
    // skip 2 unless inst_predicate_10
    0x300a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // stop unless inst_predicate_11
    // stop unless inst_predicate_11
    0x100b,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x0193, 0x0000,
    // end of raw_bitcast.b64x2 (I32)
    // end of raw_bitcast.b64x2 (I64)
    // 000546: x86_pextr.b64x2 (I64)
    // stop unless PredicateView(17)
    // 000546: x86_pextr.i64x2 (I64)
    // stop unless PredicateView(17)
    // 000546: x86_pextr.f64x2 (I64)
    // stop unless PredicateView(17)
    0x1030,
    // --> [RexMp3r_ib_unsigned_gpr#8d16] and stop
    // --> [RexMp3r_ib_unsigned_gpr#8d16] and stop
    // --> [RexMp3r_ib_unsigned_gpr#8d16] and stop
    0x019f, 0x8d16,
    // end of x86_pextr.f64x2 (I64)
    // end of x86_pextr.i64x2 (I64)
    // end of x86_pextr.b64x2 (I64)
    // 000549: x86_pinsr.b64x2 (I64)
    // stop unless PredicateView(17)
    // 000549: x86_pinsr.i64x2 (I64)
    // stop unless PredicateView(17)
    // 000549: x86_pinsr.f64x2 (I64)
    // stop unless PredicateView(17)
    0x1030,
    // --> [RexMp3r_ib_unsigned_r#8d22] and stop
    // --> [RexMp3r_ib_unsigned_r#8d22] and stop
    // --> [RexMp3r_ib_unsigned_r#8d22] and stop
    0x0199, 0x8d22,
    // end of x86_pinsr.f64x2 (I64)
    // end of x86_pinsr.i64x2 (I64)
    // end of x86_pinsr.b64x2 (I64)
    // 00054c: avg_round.i8x16 (I64)
    // --> [Mp2fa#5e0] and stop
    // 00054c: avg_round.i8x16 (I32)
    // --> [Mp2fa#5e0] and stop
    0x0137, 0x05e0,
    // end of avg_round.i8x16 (I32)
    // end of avg_round.i8x16 (I64)
    // 00054e: iadd.i8x16 (I64)
    // --> [Mp2fa#5fc] and stop
    // 00054e: iadd.i8x16 (I32)
    // --> [Mp2fa#5fc] and stop
    0x0137, 0x05fc,
    // end of iadd.i8x16 (I32)
    // end of iadd.i8x16 (I64)
    // 000550: icmp.i8x16 (I64)
    // skip 2 unless inst_predicate_26
    // 000550: icmp.i8x16 (I32)
    // skip 2 unless inst_predicate_26
    0x301a,
    // --> [Mp2icscc_fpr#574]
    // --> [Mp2icscc_fpr#574]
    0x01be, 0x0574,
    // stop unless inst_predicate_27
    // stop unless inst_predicate_27
    0x101b,
    // --> [Mp2icscc_fpr#564] and stop
    // --> [Mp2icscc_fpr#564] and stop
    0x01bf, 0x0564,
    // end of icmp.i8x16 (I32)
    // end of icmp.i8x16 (I64)
    // 000556: isub.i8x16 (I64)
    // --> [Mp2fa#5f8] and stop
    // 000556: isub.i8x16 (I32)
    // --> [Mp2fa#5f8] and stop
    0x0137, 0x05f8,
    // end of isub.i8x16 (I32)
    // end of isub.i8x16 (I64)
    // 000558: raw_bitcast.i8x16 (I64)
    // skip 2 unless inst_predicate_14
    // 000558: raw_bitcast.i8x16 (I32)
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_15
    // skip 2 unless inst_predicate_15
    0x300f,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_16
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_10
    // skip 2 unless inst_predicate_10
    0x300a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // stop unless inst_predicate_11
    // stop unless inst_predicate_11
    0x100b,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x0193, 0x0000,
    // end of raw_bitcast.i8x16 (I32)
    // end of raw_bitcast.i8x16 (I64)
    // 000579: sadd_sat.i8x16 (I64)
    // --> [Mp2fa#5ec] and stop
    // 000579: sadd_sat.i8x16 (I32)
    // --> [Mp2fa#5ec] and stop
    0x0137, 0x05ec,
    // end of sadd_sat.i8x16 (I32)
    // end of sadd_sat.i8x16 (I64)
    // 00057b: ssub_sat.i8x16 (I64)
    // --> [Mp2fa#5e8] and stop
    // 00057b: ssub_sat.i8x16 (I32)
    // --> [Mp2fa#5e8] and stop
    0x0137, 0x05e8,
    // end of ssub_sat.i8x16 (I32)
    // end of ssub_sat.i8x16 (I64)
    // 00057d: uadd_sat.i8x16 (I64)
    // --> [Mp2fa#5dc] and stop
    // 00057d: uadd_sat.i8x16 (I32)
    // --> [Mp2fa#5dc] and stop
    0x0137, 0x05dc,
    // end of uadd_sat.i8x16 (I32)
    // end of uadd_sat.i8x16 (I64)
    // 00057f: usub_sat.i8x16 (I64)
    // --> [Mp2fa#5d8] and stop
    // 00057f: usub_sat.i8x16 (I32)
    // --> [Mp2fa#5d8] and stop
    0x0137, 0x05d8,
    // end of usub_sat.i8x16 (I32)
    // end of usub_sat.i8x16 (I64)
    // 000581: x86_pmaxs.i8x16 (I64)
    // stop unless PredicateView(17)
    // 000581: x86_pmaxs.i8x16 (I32)
    // stop unless PredicateView(17)
    0x1030,
    // --> [Mp3fa#93c] and stop
    // --> [Mp3fa#93c] and stop
    0x018f, 0x093c,
    // end of x86_pmaxs.i8x16 (I32)
    // end of x86_pmaxs.i8x16 (I64)
    // 000584: x86_pmaxu.i8x16 (I64)
    // --> [Mp2fa#5de] and stop
    // 000584: x86_pmaxu.i8x16 (I32)
    // --> [Mp2fa#5de] and stop
    0x0137, 0x05de,
    // end of x86_pmaxu.i8x16 (I32)
    // end of x86_pmaxu.i8x16 (I64)
    // 000586: x86_pmins.i8x16 (I64)
    // stop unless PredicateView(17)
    // 000586: x86_pmins.i8x16 (I32)
    // stop unless PredicateView(17)
    0x1030,
    // --> [Mp3fa#938] and stop
    // --> [Mp3fa#938] and stop
    0x018f, 0x0938,
    // end of x86_pmins.i8x16 (I32)
    // end of x86_pmins.i8x16 (I64)
    // 000589: x86_pminu.i8x16 (I64)
    // --> [Mp2fa#5da] and stop
    // 000589: x86_pminu.i8x16 (I32)
    // --> [Mp2fa#5da] and stop
    0x0137, 0x05da,
    // end of x86_pminu.i8x16 (I32)
    // end of x86_pminu.i8x16 (I64)
    // 00058b: avg_round.i16x8 (I64)
    // --> [Mp2fa#5e3] and stop
    // 00058b: avg_round.i16x8 (I32)
    // --> [Mp2fa#5e3] and stop
    0x0137, 0x05e3,
    // end of avg_round.i16x8 (I32)
    // end of avg_round.i16x8 (I64)
    // 00058d: iadd.i16x8 (I64)
    // --> [Mp2fa#5fd] and stop
    // 00058d: iadd.i16x8 (I32)
    // --> [Mp2fa#5fd] and stop
    0x0137, 0x05fd,
    // end of iadd.i16x8 (I32)
    // end of iadd.i16x8 (I64)
    // 00058f: icmp.i16x8 (I64)
    // skip 2 unless inst_predicate_26
    // 00058f: icmp.i16x8 (I32)
    // skip 2 unless inst_predicate_26
    0x301a,
    // --> [Mp2icscc_fpr#575]
    // --> [Mp2icscc_fpr#575]
    0x01be, 0x0575,
    // stop unless inst_predicate_27
    // stop unless inst_predicate_27
    0x101b,
    // --> [Mp2icscc_fpr#565] and stop
    // --> [Mp2icscc_fpr#565] and stop
    0x01bf, 0x0565,
    // end of icmp.i16x8 (I32)
    // end of icmp.i16x8 (I64)
    // 000595: imul.i16x8 (I64)
    // --> [Mp2fa#5d5] and stop
    // 000595: imul.i16x8 (I32)
    // --> [Mp2fa#5d5] and stop
    0x0137, 0x05d5,
    // end of imul.i16x8 (I32)
    // end of imul.i16x8 (I64)
    // 000597: ishl_imm.i16x8 (I64)
    // --> [Mp2f_ib#6571] and stop
    // 000597: ishl_imm.i16x8 (I32)
    // --> [Mp2f_ib#6571] and stop
    0x01bd, 0x6571,
    // end of ishl_imm.i16x8 (I32)
    // end of ishl_imm.i16x8 (I64)
    // 000599: isub.i16x8 (I64)
    // --> [Mp2fa#5f9] and stop
    // 000599: isub.i16x8 (I32)
    // --> [Mp2fa#5f9] and stop
    0x0137, 0x05f9,
    // end of isub.i16x8 (I32)
    // end of isub.i16x8 (I64)
    // 00059b: raw_bitcast.i16x8 (I64)
    // skip 2 unless inst_predicate_14
    // 00059b: raw_bitcast.i16x8 (I32)
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_15
    // skip 2 unless inst_predicate_15
    0x300f,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_16
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_10
    // skip 2 unless inst_predicate_10
    0x300a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // stop unless inst_predicate_11
    // stop unless inst_predicate_11
    0x100b,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x0193, 0x0000,
    // end of raw_bitcast.i16x8 (I32)
    // end of raw_bitcast.i16x8 (I64)
    // 0005bc: sadd_sat.i16x8 (I64)
    // --> [Mp2fa#5ed] and stop
    // 0005bc: sadd_sat.i16x8 (I32)
    // --> [Mp2fa#5ed] and stop
    0x0137, 0x05ed,
    // end of sadd_sat.i16x8 (I32)
    // end of sadd_sat.i16x8 (I64)
    // 0005be: sshr_imm.i16x8 (I64)
    // --> [Mp2f_ib#4571] and stop
    // 0005be: sshr_imm.i16x8 (I32)
    // --> [Mp2f_ib#4571] and stop
    0x01bd, 0x4571,
    // end of sshr_imm.i16x8 (I32)
    // end of sshr_imm.i16x8 (I64)
    // 0005c0: ssub_sat.i16x8 (I64)
    // --> [Mp2fa#5e9] and stop
    // 0005c0: ssub_sat.i16x8 (I32)
    // --> [Mp2fa#5e9] and stop
    0x0137, 0x05e9,
    // end of ssub_sat.i16x8 (I32)
    // end of ssub_sat.i16x8 (I64)
    // 0005c2: uadd_sat.i16x8 (I64)
    // --> [Mp2fa#5dd] and stop
    // 0005c2: uadd_sat.i16x8 (I32)
    // --> [Mp2fa#5dd] and stop
    0x0137, 0x05dd,
    // end of uadd_sat.i16x8 (I32)
    // end of uadd_sat.i16x8 (I64)
    // 0005c4: ushr_imm.i16x8 (I64)
    // --> [Mp2f_ib#2571] and stop
    // 0005c4: ushr_imm.i16x8 (I32)
    // --> [Mp2f_ib#2571] and stop
    0x01bd, 0x2571,
    // end of ushr_imm.i16x8 (I32)
    // end of ushr_imm.i16x8 (I64)
    // 0005c6: usub_sat.i16x8 (I64)
    // --> [Mp2fa#5d9] and stop
    // 0005c6: usub_sat.i16x8 (I32)
    // --> [Mp2fa#5d9] and stop
    0x0137, 0x05d9,
    // end of usub_sat.i16x8 (I32)
    // end of usub_sat.i16x8 (I64)
    // 0005c8: x86_pmaxs.i16x8 (I64)
    // --> [Mp2fa#5ee] and stop
    // 0005c8: x86_pmaxs.i16x8 (I32)
    // --> [Mp2fa#5ee] and stop
    0x0137, 0x05ee,
    // end of x86_pmaxs.i16x8 (I32)
    // end of x86_pmaxs.i16x8 (I64)
    // 0005ca: x86_pmaxu.i16x8 (I64)
    // stop unless PredicateView(17)
    // 0005ca: x86_pmaxu.i16x8 (I32)
    // stop unless PredicateView(17)
    0x1030,
    // --> [Mp3fa#93e] and stop
    // --> [Mp3fa#93e] and stop
    0x018f, 0x093e,
    // end of x86_pmaxu.i16x8 (I32)
    // end of x86_pmaxu.i16x8 (I64)
    // 0005cd: x86_pmins.i16x8 (I64)
    // --> [Mp2fa#5ea] and stop
    // 0005cd: x86_pmins.i16x8 (I32)
    // --> [Mp2fa#5ea] and stop
    0x0137, 0x05ea,
    // end of x86_pmins.i16x8 (I32)
    // end of x86_pmins.i16x8 (I64)
    // 0005cf: x86_pminu.i16x8 (I64)
    // stop unless PredicateView(17)
    // 0005cf: x86_pminu.i16x8 (I32)
    // stop unless PredicateView(17)
    0x1030,
    // --> [Mp3fa#93a] and stop
    // --> [Mp3fa#93a] and stop
    0x018f, 0x093a,
    // end of x86_pminu.i16x8 (I32)
    // end of x86_pminu.i16x8 (I64)
    // 0005d2: x86_psll.i16x8 (I64)
    // --> [Mp2fa#5f1] and stop
    // 0005d2: x86_psll.i16x8 (I32)
    // --> [Mp2fa#5f1] and stop
    0x0137, 0x05f1,
    // end of x86_psll.i16x8 (I32)
    // end of x86_psll.i16x8 (I64)
    // 0005d4: x86_psra.i16x8 (I64)
    // --> [Mp2fa#5e1] and stop
    // 0005d4: x86_psra.i16x8 (I32)
    // --> [Mp2fa#5e1] and stop
    0x0137, 0x05e1,
    // end of x86_psra.i16x8 (I32)
    // end of x86_psra.i16x8 (I64)
    // 0005d6: x86_psrl.i16x8 (I64)
    // --> [Mp2fa#5d1] and stop
    // 0005d6: x86_psrl.i16x8 (I32)
    // --> [Mp2fa#5d1] and stop
    0x0137, 0x05d1,
    // end of x86_psrl.i16x8 (I32)
    // end of x86_psrl.i16x8 (I64)
    // 0005d8: iadd.i32x4 (I64)
    // --> [Mp2fa#5fe] and stop
    // 0005d8: iadd.i32x4 (I32)
    // --> [Mp2fa#5fe] and stop
    0x0137, 0x05fe,
    // end of iadd.i32x4 (I32)
    // end of iadd.i32x4 (I64)
    // 0005da: icmp.i32x4 (I64)
    // skip 2 unless inst_predicate_26
    // 0005da: icmp.i32x4 (I32)
    // skip 2 unless inst_predicate_26
    0x301a,
    // --> [Mp2icscc_fpr#576]
    // --> [Mp2icscc_fpr#576]
    0x01be, 0x0576,
    // stop unless inst_predicate_27
    // stop unless inst_predicate_27
    0x101b,
    // --> [Mp2icscc_fpr#566] and stop
    // --> [Mp2icscc_fpr#566] and stop
    0x01bf, 0x0566,
    // end of icmp.i32x4 (I32)
    // end of icmp.i32x4 (I64)
    // 0005e0: imul.i32x4 (I64)
    // stop unless PredicateView(17)
    // 0005e0: imul.i32x4 (I32)
    // stop unless PredicateView(17)
    0x1030,
    // --> [Mp3fa#940] and stop
    // --> [Mp3fa#940] and stop
    0x018f, 0x0940,
    // end of imul.i32x4 (I32)
    // end of imul.i32x4 (I64)
    // 0005e3: ishl_imm.i32x4 (I64)
    // --> [Mp2f_ib#6572] and stop
    // 0005e3: ishl_imm.i32x4 (I32)
    // --> [Mp2f_ib#6572] and stop
    0x01bd, 0x6572,
    // end of ishl_imm.i32x4 (I32)
    // end of ishl_imm.i32x4 (I64)
    // 0005e5: isub.i32x4 (I64)
    // --> [Mp2fa#5fa] and stop
    // 0005e5: isub.i32x4 (I32)
    // --> [Mp2fa#5fa] and stop
    0x0137, 0x05fa,
    // end of isub.i32x4 (I32)
    // end of isub.i32x4 (I64)
    // 0005e7: raw_bitcast.i32x4 (I64)
    // skip 2 unless inst_predicate_14
    // 0005e7: raw_bitcast.i32x4 (I32)
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_15
    // skip 2 unless inst_predicate_15
    0x300f,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_16
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_10
    // skip 2 unless inst_predicate_10
    0x300a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // stop unless inst_predicate_11
    // stop unless inst_predicate_11
    0x100b,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x0193, 0x0000,
    // end of raw_bitcast.i32x4 (I32)
    // end of raw_bitcast.i32x4 (I64)
    // 000608: sshr_imm.i32x4 (I64)
    // --> [Mp2f_ib#4572] and stop
    // 000608: sshr_imm.i32x4 (I32)
    // --> [Mp2f_ib#4572] and stop
    0x01bd, 0x4572,
    // end of sshr_imm.i32x4 (I32)
    // end of sshr_imm.i32x4 (I64)
    // 00060a: ushr_imm.i32x4 (I64)
    // --> [Mp2f_ib#2572] and stop
    // 00060a: ushr_imm.i32x4 (I32)
    // --> [Mp2f_ib#2572] and stop
    0x01bd, 0x2572,
    // end of ushr_imm.i32x4 (I32)
    // end of ushr_imm.i32x4 (I64)
    // 00060c: x86_pmaxs.i32x4 (I64)
    // stop unless PredicateView(17)
    // 00060c: x86_pmaxs.i32x4 (I32)
    // stop unless PredicateView(17)
    0x1030,
    // --> [Mp3fa#93d] and stop
    // --> [Mp3fa#93d] and stop
    0x018f, 0x093d,
    // end of x86_pmaxs.i32x4 (I32)
    // end of x86_pmaxs.i32x4 (I64)
    // 00060f: x86_pmaxu.i32x4 (I64)
    // stop unless PredicateView(17)
    // 00060f: x86_pmaxu.i32x4 (I32)
    // stop unless PredicateView(17)
    0x1030,
    // --> [Mp3fa#93f] and stop
    // --> [Mp3fa#93f] and stop
    0x018f, 0x093f,
    // end of x86_pmaxu.i32x4 (I32)
    // end of x86_pmaxu.i32x4 (I64)
    // 000612: x86_pmins.i32x4 (I64)
    // stop unless PredicateView(17)
    // 000612: x86_pmins.i32x4 (I32)
    // stop unless PredicateView(17)
    0x1030,
    // --> [Mp3fa#939] and stop
    // --> [Mp3fa#939] and stop
    0x018f, 0x0939,
    // end of x86_pmins.i32x4 (I32)
    // end of x86_pmins.i32x4 (I64)
    // 000615: x86_pminu.i32x4 (I64)
    // stop unless PredicateView(17)
    // 000615: x86_pminu.i32x4 (I32)
    // stop unless PredicateView(17)
    0x1030,
    // --> [Mp3fa#93b] and stop
    // --> [Mp3fa#93b] and stop
    0x018f, 0x093b,
    // end of x86_pminu.i32x4 (I32)
    // end of x86_pminu.i32x4 (I64)
    // 000618: x86_psll.i32x4 (I64)
    // --> [Mp2fa#5f2] and stop
    // 000618: x86_psll.i32x4 (I32)
    // --> [Mp2fa#5f2] and stop
    0x0137, 0x05f2,
    // end of x86_psll.i32x4 (I32)
    // end of x86_psll.i32x4 (I64)
    // 00061a: x86_psra.i32x4 (I64)
    // --> [Mp2fa#5e2] and stop
    // 00061a: x86_psra.i32x4 (I32)
    // --> [Mp2fa#5e2] and stop
    0x0137, 0x05e2,
    // end of x86_psra.i32x4 (I32)
    // end of x86_psra.i32x4 (I64)
    // 00061c: x86_psrl.i32x4 (I64)
    // --> [Mp2fa#5d2] and stop
    // 00061c: x86_psrl.i32x4 (I32)
    // --> [Mp2fa#5d2] and stop
    0x0137, 0x05d2,
    // end of x86_psrl.i32x4 (I32)
    // end of x86_psrl.i32x4 (I64)
    // 00061e: bitcast.i64x2 (I64)
    // skip 4 unless inst_predicate_3
    0x5003,
    // --> [RexMp2frurm#56e]
    0x00d2, 0x056e,
    // --> [Mp2frurm#56e]
    0x00d0, 0x056e,
    // stop unless inst_predicate_4
    0x1004,
    // --> [RexMp2frurm#856e] and stop
    0x00d3, 0x856e,
    // end of bitcast.i64x2 (I64)
    // 000626: iadd.i64x2 (I64)
    // --> [Mp2fa#5d4] and stop
    // 000626: iadd.i64x2 (I32)
    // --> [Mp2fa#5d4] and stop
    0x0137, 0x05d4,
    // end of iadd.i64x2 (I32)
    // end of iadd.i64x2 (I64)
    // 000628: icmp.i64x2 (I64)
    // skip 3 unless PredicateView(17)
    // 000628: icmp.i64x2 (I32)
    // skip 3 unless PredicateView(17)
    0x4030,
    // skip 2 unless inst_predicate_26
    // skip 2 unless inst_predicate_26
    0x301a,
    // --> [Mp3icscc_fpr#929]
    // --> [Mp3icscc_fpr#929]
    0x01c0, 0x0929,
    // stop unless PredicateView(19)
    // stop unless PredicateView(19)
    0x1032,
    // stop unless inst_predicate_27
    // stop unless inst_predicate_27
    0x101b,
    // --> [Mp3icscc_fpr#937] and stop
    // --> [Mp3icscc_fpr#937] and stop
    0x01c1, 0x0937,
    // end of icmp.i64x2 (I32)
    // end of icmp.i64x2 (I64)
    // 000630: ishl_imm.i64x2 (I64)
    // --> [Mp2f_ib#6573] and stop
    // 000630: ishl_imm.i64x2 (I32)
    // --> [Mp2f_ib#6573] and stop
    0x01bd, 0x6573,
    // end of ishl_imm.i64x2 (I32)
    // end of ishl_imm.i64x2 (I64)
    // 000632: isub.i64x2 (I64)
    // --> [Mp2fa#5fb] and stop
    // 000632: isub.i64x2 (I32)
    // --> [Mp2fa#5fb] and stop
    0x0137, 0x05fb,
    // end of isub.i64x2 (I32)
    // end of isub.i64x2 (I64)
    // 000634: raw_bitcast.i64x2 (I64)
    // skip 2 unless inst_predicate_14
    // 000634: raw_bitcast.i64x2 (I32)
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_15
    // skip 2 unless inst_predicate_15
    0x300f,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_16
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_10
    // skip 2 unless inst_predicate_10
    0x300a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // stop unless inst_predicate_11
    // stop unless inst_predicate_11
    0x100b,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x0193, 0x0000,
    // end of raw_bitcast.i64x2 (I32)
    // end of raw_bitcast.i64x2 (I64)
    // 000655: sshr_imm.i64x2 (I64)
    // --> [Mp2f_ib#4573] and stop
    // 000655: sshr_imm.i64x2 (I32)
    // --> [Mp2f_ib#4573] and stop
    0x01bd, 0x4573,
    // end of sshr_imm.i64x2 (I32)
    // end of sshr_imm.i64x2 (I64)
    // 000657: ushr_imm.i64x2 (I64)
    // --> [Mp2f_ib#2573] and stop
    // 000657: ushr_imm.i64x2 (I32)
    // --> [Mp2f_ib#2573] and stop
    0x01bd, 0x2573,
    // end of ushr_imm.i64x2 (I32)
    // end of ushr_imm.i64x2 (I64)
    // 000659: x86_psll.i64x2 (I64)
    // --> [Mp2fa#5f3] and stop
    // 000659: x86_psll.i64x2 (I32)
    // --> [Mp2fa#5f3] and stop
    0x0137, 0x05f3,
    // end of x86_psll.i64x2 (I32)
    // end of x86_psll.i64x2 (I64)
    // 00065b: x86_psrl.i64x2 (I64)
    // --> [Mp2fa#5d3] and stop
    // 00065b: x86_psrl.i64x2 (I32)
    // --> [Mp2fa#5d3] and stop
    0x0137, 0x05d3,
    // end of x86_psrl.i64x2 (I32)
    // end of x86_psrl.i64x2 (I64)
    // 00065d: fadd.f32x4 (I64)
    // --> [RexOp2fa#458]
    0x0164, 0x0458,
    // --> [Op2fa#458] and stop
    // 00065f: fadd.f32x4 (I32)
    // --> [Op2fa#458] and stop
    0x0163, 0x0458,
    // end of fadd.f32x4 (I32)
    // end of fadd.f32x4 (I64)
    // 000661: fcmp.f32x4 (I64)
    // --> [RexOp2pfcmp#4c2]
    0x01c4, 0x04c2,
    // --> [Op2pfcmp#4c2] and stop
    // 000663: fcmp.f32x4 (I32)
    // --> [Op2pfcmp#4c2] and stop
    0x01c3, 0x04c2,
    // end of fcmp.f32x4 (I32)
    // end of fcmp.f32x4 (I64)
    // 000665: fdiv.f32x4 (I64)
    // --> [RexOp2fa#45e]
    0x0164, 0x045e,
    // --> [Op2fa#45e] and stop
    // 000667: fdiv.f32x4 (I32)
    // --> [Op2fa#45e] and stop
    0x0163, 0x045e,
    // end of fdiv.f32x4 (I32)
    // end of fdiv.f32x4 (I64)
    // 000669: fmax.f32x4 (I64)
    // --> [RexOp2fa#45f]
    0x0164, 0x045f,
    // --> [Op2fa#45f] and stop
    // 00066b: fmax.f32x4 (I32)
    // --> [Op2fa#45f] and stop
    0x0163, 0x045f,
    // end of fmax.f32x4 (I32)
    // end of fmax.f32x4 (I64)
    // 00066d: fmin.f32x4 (I64)
    // --> [RexOp2fa#45d]
    0x0164, 0x045d,
    // --> [Op2fa#45d] and stop
    // 00066f: fmin.f32x4 (I32)
    // --> [Op2fa#45d] and stop
    0x0163, 0x045d,
    // end of fmin.f32x4 (I32)
    // end of fmin.f32x4 (I64)
    // 000671: fmul.f32x4 (I64)
    // --> [RexOp2fa#459]
    0x0164, 0x0459,
    // --> [Op2fa#459] and stop
    // 000673: fmul.f32x4 (I32)
    // --> [Op2fa#459] and stop
    0x0163, 0x0459,
    // end of fmul.f32x4 (I32)
    // end of fmul.f32x4 (I64)
    // 000675: fsub.f32x4 (I64)
    // --> [RexOp2fa#45c]
    0x0164, 0x045c,
    // --> [Op2fa#45c] and stop
    // 000677: fsub.f32x4 (I32)
    // --> [Op2fa#45c] and stop
    0x0163, 0x045c,
    // end of fsub.f32x4 (I32)
    // end of fsub.f32x4 (I64)
    // 000679: raw_bitcast.f32x4 (I64)
    // skip 2 unless inst_predicate_14
    // 000679: raw_bitcast.f32x4 (I32)
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_15
    // skip 2 unless inst_predicate_15
    0x300f,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_16
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_10
    // skip 2 unless inst_predicate_10
    0x300a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // stop unless inst_predicate_11
    // stop unless inst_predicate_11
    0x100b,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x0193, 0x0000,
    // end of raw_bitcast.f32x4 (I32)
    // end of raw_bitcast.f32x4 (I64)
    // 00069a: sqrt.f32x4 (I64)
    // --> [RexOp2furm#451]
    0x00da, 0x0451,
    // --> [Op2furm#451] and stop
    // 00069c: sqrt.f32x4 (I32)
    // --> [Op2furm#451] and stop
    0x00d9, 0x0451,
    // end of sqrt.f32x4 (I32)
    // end of sqrt.f32x4 (I64)
    // 00069e: x86_insertps.f32x4 (I64)
    // stop unless PredicateView(17)
    // 00069e: x86_insertps.f32x4 (I32)
    // stop unless PredicateView(17)
    0x1030,
    // --> [Mp3fa_ib#d21] and stop
    // --> [Mp3fa_ib#d21] and stop
    0x019b, 0x0d21,
    // end of x86_insertps.f32x4 (I32)
    // end of x86_insertps.f32x4 (I64)
    // 0006a1: fadd.f64x2 (I64)
    // --> [RexMp2fa#558]
    0x0138, 0x0558,
    // --> [Mp2fa#558] and stop
    // 0006a3: fadd.f64x2 (I32)
    // --> [Mp2fa#558] and stop
    0x0137, 0x0558,
    // end of fadd.f64x2 (I32)
    // end of fadd.f64x2 (I64)
    // 0006a5: fcmp.f64x2 (I64)
    // --> [RexMp2pfcmp#5c2]
    0x01c8, 0x05c2,
    // --> [Mp2pfcmp#5c2] and stop
    // 0006a7: fcmp.f64x2 (I32)
    // --> [Mp2pfcmp#5c2] and stop
    0x01c7, 0x05c2,
    // end of fcmp.f64x2 (I32)
    // end of fcmp.f64x2 (I64)
    // 0006a9: fdiv.f64x2 (I64)
    // --> [RexMp2fa#55e]
    0x0138, 0x055e,
    // --> [Mp2fa#55e] and stop
    // 0006ab: fdiv.f64x2 (I32)
    // --> [Mp2fa#55e] and stop
    0x0137, 0x055e,
    // end of fdiv.f64x2 (I32)
    // end of fdiv.f64x2 (I64)
    // 0006ad: fmax.f64x2 (I64)
    // --> [RexMp2fa#55f]
    0x0138, 0x055f,
    // --> [Mp2fa#55f] and stop
    // 0006af: fmax.f64x2 (I32)
    // --> [Mp2fa#55f] and stop
    0x0137, 0x055f,
    // end of fmax.f64x2 (I32)
    // end of fmax.f64x2 (I64)
    // 0006b1: fmin.f64x2 (I64)
    // --> [RexMp2fa#55d]
    0x0138, 0x055d,
    // --> [Mp2fa#55d] and stop
    // 0006b3: fmin.f64x2 (I32)
    // --> [Mp2fa#55d] and stop
    0x0137, 0x055d,
    // end of fmin.f64x2 (I32)
    // end of fmin.f64x2 (I64)
    // 0006b5: fmul.f64x2 (I64)
    // --> [RexMp2fa#559]
    0x0138, 0x0559,
    // --> [Mp2fa#559] and stop
    // 0006b7: fmul.f64x2 (I32)
    // --> [Mp2fa#559] and stop
    0x0137, 0x0559,
    // end of fmul.f64x2 (I32)
    // end of fmul.f64x2 (I64)
    // 0006b9: fsub.f64x2 (I64)
    // --> [RexMp2fa#55c]
    0x0138, 0x055c,
    // --> [Mp2fa#55c] and stop
    // 0006bb: fsub.f64x2 (I32)
    // --> [Mp2fa#55c] and stop
    0x0137, 0x055c,
    // end of fsub.f64x2 (I32)
    // end of fsub.f64x2 (I64)
    // 0006bd: raw_bitcast.f64x2 (I64)
    // skip 2 unless inst_predicate_14
    // 0006bd: raw_bitcast.f64x2 (I32)
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_15
    // skip 2 unless inst_predicate_15
    0x300f,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_16
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // skip 2 unless inst_predicate_10
    // skip 2 unless inst_predicate_10
    0x300a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x0192, 0x0000,
    // stop unless inst_predicate_11
    // stop unless inst_predicate_11
    0x100b,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x0193, 0x0000,
    // end of raw_bitcast.f64x2 (I32)
    // end of raw_bitcast.f64x2 (I64)
    // 0006de: sqrt.f64x2 (I64)
    // --> [RexMp2furm#551]
    0x012c, 0x0551,
    // --> [Mp2furm#551] and stop
    // 0006e0: sqrt.f64x2 (I32)
    // --> [Mp2furm#551] and stop
    0x012b, 0x0551,
    // end of sqrt.f64x2 (I32)
    // end of sqrt.f64x2 (I64)
    // 0006e2: x86_movlhps.f64x2 (I64)
    // --> [Op2fa#416] and stop
    // 0006e2: x86_movlhps.f64x2 (I32)
    // --> [Op2fa#416] and stop
    0x0163, 0x0416,
    // end of x86_movlhps.f64x2 (I32)
    // end of x86_movlhps.f64x2 (I64)
    // 0006e4: x86_movsd.f64x2 (I64)
    // --> [Mp2fa#710] and stop
    // 0006e4: x86_movsd.f64x2 (I32)
    // --> [Mp2fa#710] and stop
    0x0137, 0x0710,
    // end of x86_movsd.f64x2 (I32)
    // end of x86_movsd.f64x2 (I64)
    // 0006e6: adjust_sp_down.i32 (I32)
    // --> [Op1adjustsp#29] and stop
    0x00c5, 0x0029,
    // end of adjust_sp_down.i32 (I32)
    // 0006e8: bint.i32 (I32)
    // skip 2 unless inst_predicate_6
    // 0006e8: bint.i8 (I32)
    // skip 2 unless inst_predicate_6
    // 0006e8: bint.i16 (I32)
    // skip 2 unless inst_predicate_6
    0x3006,
    // --> [Op2urm_noflags_abcd#4b6]
    // --> [Op2urm_noflags_abcd#4b6]
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // stop unless inst_predicate_7
    // stop unless inst_predicate_7
    // stop unless inst_predicate_7
    0x1007,
    // --> [Op2urm_noflags_abcd#4b6] and stop
    // --> [Op2urm_noflags_abcd#4b6] and stop
    // --> [Op2urm_noflags_abcd#4b6] and stop
    0x0021, 0x04b6,
    // end of bint.i16 (I32)
    // end of bint.i8 (I32)
    // end of bint.i32 (I32)
    // 0006ee: bitcast.i32 (I32)
    // stop unless inst_predicate_10
    0x100a,
    // --> [Mp2rfumr#57e] and stop
    0x00d5, 0x057e,
    // end of bitcast.i32 (I32)
    // 0006f1: brnz.i32 (I32)
    // --> [Op1tjccb#75]
    0x0200, 0x0075,
    // --> [Op1tjccd#85] and stop
    0x0205, 0x0085,
    // end of brnz.i32 (I32)
    // 0006f5: brz.i32 (I32)
    // --> [Op1tjccb#74]
    0x0200, 0x0074,
    // --> [Op1tjccd#84] and stop
    0x0205, 0x0084,
    // end of brz.i32 (I32)
    // 0006f9: clz.i32 (I32)
    // stop unless PredicateView(14)
    0x102d,
    // --> [Mp2urm#6bd] and stop
    0x016f, 0x06bd,
    // end of clz.i32 (I32)
    // 0006fc: copy_to_ssa.i32 (I32)
    // --> [Op1umr_reg_to_ssa#89] and stop
    // 0006fc: copy_to_ssa.r32 (I32)
    // --> [Op1umr_reg_to_ssa#89] and stop
    // 0006fc: copy_to_ssa.b1 (I32)
    // --> [Op1umr_reg_to_ssa#89] and stop
    // 0006fc: copy_to_ssa.i8 (I32)
    // --> [Op1umr_reg_to_ssa#89] and stop
    // 0006fc: copy_to_ssa.i16 (I32)
    // --> [Op1umr_reg_to_ssa#89] and stop
    0x002d, 0x0089,
    // end of copy_to_ssa.i16 (I32)
    // end of copy_to_ssa.i8 (I32)
    // end of copy_to_ssa.b1 (I32)
    // end of copy_to_ssa.r32 (I32)
    // end of copy_to_ssa.i32 (I32)
    // 0006fe: ctz.i32 (I32)
    // stop unless PredicateView(13)
    0x102c,
    // --> [Mp2urm#6bc] and stop
    0x016f, 0x06bc,
    // end of ctz.i32 (I32)
    // 000701: func_addr.i32 (I32)
    // skip 2 unless PredicateView(11)
    0x302a,
    // --> [Op1fnaddr4#b8]
    0x01ca, 0x00b8,
    // stop unless PredicateView(9)
    0x1028,
    // --> [Op1allones_fnaddr4#b8] and stop
    0x01cf, 0x00b8,
    // end of func_addr.i32 (I32)
    // 000707: iconst.i32 (I32)
    // --> [Op1pu_id#b8]
    0x000e, 0x00b8,
    // stop unless inst_predicate_1
    // 000709: iconst.i16 (I32)
    // stop unless inst_predicate_1
    0x1001,
    // --> [Op1u_id_z#31] and stop
    // --> [Op1u_id_z#31] and stop
    0x001b, 0x0031,
    // end of iconst.i16 (I32)
    // end of iconst.i32 (I32)
    // 00070c: ifcmp_sp.i32 (I32)
    // --> [Op1rcmp_sp#39] and stop
    0x0181, 0x0039,
    // end of ifcmp_sp.i32 (I32)
    // 00070e: istore16.i32 (I32)
    // --> [Mp1st#189]
    0x0074, 0x0189,
    // --> [Mp1stDisp8#189]
    0x007c, 0x0189,
    // --> [Mp1stDisp32#189] and stop
    0x0085, 0x0189,
    // end of istore16.i32 (I32)
    // 000714: istore16_complex.i32 (I32)
    // stop unless inst_predicate_9
    0x1009,
    // --> [Mp1stWithIndex#189]
    0x0050, 0x0189,
    // --> [Mp1stWithIndexDisp8#189]
    0x0058, 0x0189,
    // --> [Mp1stWithIndexDisp32#189] and stop
    0x0061, 0x0189,
    // end of istore16_complex.i32 (I32)
    // 00071b: istore8.i32 (I32)
    // --> [Op1st_abcd#88]
    0x0088, 0x0088,
    // --> [Op1stDisp8_abcd#88]
    0x008a, 0x0088,
    // --> [Op1stDisp32_abcd#88] and stop
    0x008d, 0x0088,
    // end of istore8.i32 (I32)
    // 000721: istore8_complex.i32 (I32)
    // stop unless inst_predicate_9
    0x1009,
    // --> [Op1stWithIndex_abcd#88]
    0x0064, 0x0088,
    // --> [Op1stWithIndexDisp8_abcd#88]
    0x0068, 0x0088,
    // --> [Op1stWithIndexDisp32_abcd#88] and stop
    0x006d, 0x0088,
    // end of istore8_complex.i32 (I32)
    // 000728: jump_table_base.i32 (I32)
    // --> [Op1jt_base#8d] and stop
    0x0219, 0x008d,
    // end of jump_table_base.i32 (I32)
    // 00072a: jump_table_entry.i32 (I32)
    // --> [Op1jt_entry#8b] and stop
    0x0215, 0x008b,
    // end of jump_table_entry.i32 (I32)
    // 00072c: load.i32 (I32)
    // --> [Op1ld#8b]
    // 00072c: load.r32 (I32)
    // --> [Op1ld#8b]
    0x0096, 0x008b,
    // --> [Op1ldDisp8#8b]
    // --> [Op1ldDisp8#8b]
    0x009e, 0x008b,
    // --> [Op1ldDisp32#8b] and stop
    // --> [Op1ldDisp32#8b] and stop
    0x00a7, 0x008b,
    // end of load.r32 (I32)
    // end of load.i32 (I32)
    // 000732: load_complex.i32 (I32)
    // stop unless inst_predicate_8
    0x1008,
    // --> [Op1ldWithIndex#8b]
    0x0034, 0x008b,
    // --> [Op1ldWithIndexDisp8#8b]
    0x003c, 0x008b,
    // --> [Op1ldWithIndexDisp32#8b] and stop
    0x0045, 0x008b,
    // end of load_complex.i32 (I32)
    // 000739: popcnt.i32 (I32)
    // stop unless PredicateView(15)
    0x102e,
    // --> [Mp2urm#6b8] and stop
    0x016f, 0x06b8,
    // end of popcnt.i32 (I32)
    // 00073c: sextend.i32 (I32)
    // skip 2 unless inst_predicate_5
    0x3005,
    // --> [Op2urm_noflags_abcd#4be]
    0x0020, 0x04be,
    // stop unless inst_predicate_2
    0x1002,
    // --> [Op2urm_noflags#4bf] and stop
    0x0025, 0x04bf,
    // end of sextend.i32 (I32)
    // 000742: sload16.i32 (I32)
    // --> [Op2ld#4bf]
    0x009a, 0x04bf,
    // --> [Op2ldDisp8#4bf]
    0x00a2, 0x04bf,
    // --> [Op2ldDisp32#4bf] and stop
    0x00ab, 0x04bf,
    // end of sload16.i32 (I32)
    // 000748: sload16_complex.i32 (I32)
    // stop unless inst_predicate_8
    0x1008,
    // --> [Op2ldWithIndex#4bf]
    0x0038, 0x04bf,
    // --> [Op2ldWithIndexDisp8#4bf]
    0x0040, 0x04bf,
    // --> [Op2ldWithIndexDisp32#4bf] and stop
    0x0049, 0x04bf,
    // end of sload16_complex.i32 (I32)
    // 00074f: sload8.i32 (I32)
    // --> [Op2ld#4be]
    0x009a, 0x04be,
    // --> [Op2ldDisp8#4be]
    0x00a2, 0x04be,
    // --> [Op2ldDisp32#4be] and stop
    0x00ab, 0x04be,
    // end of sload8.i32 (I32)
    // 000755: sload8_complex.i32 (I32)
    // stop unless inst_predicate_8
    0x1008,
    // --> [Op2ldWithIndex#4be]
    0x0038, 0x04be,
    // --> [Op2ldWithIndexDisp8#4be]
    0x0040, 0x04be,
    // --> [Op2ldWithIndexDisp32#4be] and stop
    0x0049, 0x04be,
    // end of sload8_complex.i32 (I32)
    // 00075c: stack_addr.i32 (I32)
    // --> [Op1spaddr4_id#8d] and stop
    0x01df, 0x008d,
    // end of stack_addr.i32 (I32)
    // 00075e: store.i32 (I32)
    // --> [Op1st#89]
    // 00075e: store.r32 (I32)
    // --> [Op1st#89]
    0x0070, 0x0089,
    // --> [Op1stDisp8#89]
    // --> [Op1stDisp8#89]
    0x0078, 0x0089,
    // --> [Op1stDisp32#89] and stop
    // --> [Op1stDisp32#89] and stop
    0x0081, 0x0089,
    // end of store.r32 (I32)
    // end of store.i32 (I32)
    // 000764: store_complex.i32 (I32)
    // stop unless inst_predicate_9
    0x1009,
    // --> [Op1stWithIndex#89]
    0x004c, 0x0089,
    // --> [Op1stWithIndexDisp8#89]
    0x0054, 0x0089,
    // --> [Op1stWithIndexDisp32#89] and stop
    0x005d, 0x0089,
    // end of store_complex.i32 (I32)
    // 00076b: symbol_value.i32 (I32)
    // stop unless PredicateView(12)
    0x102b,
    // --> [Op1gvaddr4#b8] and stop
    0x01d7, 0x00b8,
    // end of symbol_value.i32 (I32)
    // 00076e: uextend.i32 (I32)
    // skip 2 unless inst_predicate_5
    0x3005,
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // stop unless inst_predicate_2
    0x1002,
    // --> [Op2urm_noflags#4b7] and stop
    0x0025, 0x04b7,
    // end of uextend.i32 (I32)
    // 000774: uload16.i32 (I32)
    // --> [Op2ld#4b7]
    0x009a, 0x04b7,
    // --> [Op2ldDisp8#4b7]
    0x00a2, 0x04b7,
    // --> [Op2ldDisp32#4b7] and stop
    0x00ab, 0x04b7,
    // end of uload16.i32 (I32)
    // 00077a: uload16_complex.i32 (I32)
    // stop unless inst_predicate_8
    0x1008,
    // --> [Op2ldWithIndex#4b7]
    0x0038, 0x04b7,
    // --> [Op2ldWithIndexDisp8#4b7]
    0x0040, 0x04b7,
    // --> [Op2ldWithIndexDisp32#4b7] and stop
    0x0049, 0x04b7,
    // end of uload16_complex.i32 (I32)
    // 000781: uload8.i32 (I32)
    // --> [Op2ld#4b6]
    0x009a, 0x04b6,
    // --> [Op2ldDisp8#4b6]
    0x00a2, 0x04b6,
    // --> [Op2ldDisp32#4b6] and stop
    0x00ab, 0x04b6,
    // end of uload8.i32 (I32)
    // 000787: uload8_complex.i32 (I32)
    // stop unless inst_predicate_8
    0x1008,
    // --> [Op2ldWithIndex#4b6]
    0x0038, 0x04b6,
    // --> [Op2ldWithIndexDisp8#4b6]
    0x0040, 0x04b6,
    // --> [Op2ldWithIndexDisp32#4b6] and stop
    0x0049, 0x04b6,
    // end of uload8_complex.i32 (I32)
    // 00078e: x86_cvtt2si.i32 (I32)
    // skip 2 unless inst_predicate_10
    0x300a,
    // --> [Mp2rfurm#62c]
    0x012e, 0x062c,
    // stop unless inst_predicate_11
    0x100b,
    // --> [Mp2rfurm#72c] and stop
    0x012f, 0x072c,
    // end of x86_cvtt2si.i32 (I32)
    // 000794: is_invalid.r32 (I32)
    // --> [Op1is_invalid#7083] and stop
    0x022f, 0x7083,
    // end of is_invalid.r32 (I32)
    // 000796: is_null.r32 (I32)
    // --> [Op1is_zero#85] and stop
    0x022b, 0x0085,
    // end of is_null.r32 (I32)
    // 000798: brnz.b1 (I32)
    // --> [Op1t8jccd_long#85]
    0x0208, 0x0085,
    // --> [Op1t8jccb_abcd#75]
    0x020a, 0x0075,
    // --> [Op1t8jccd_abcd#85] and stop
    0x020f, 0x0085,
    // end of brnz.b1 (I32)
    // 00079e: brz.b1 (I32)
    // --> [Op1t8jccd_long#84]
    0x0208, 0x0084,
    // --> [Op1t8jccb_abcd#74]
    0x020a, 0x0074,
    // --> [Op1t8jccd_abcd#84] and stop
    0x020f, 0x0084,
    // end of brz.b1 (I32)
    // 0007a4: iconst.i8 (I32)
    // stop unless inst_predicate_1
    0x1001,
    // --> [Op1u_id_z#30] and stop
    0x001b, 0x0030,
    // end of iconst.i8 (I32)
    // 0007a7: ireduce.i8 (I32)
    // skip 2 unless inst_predicate_2
    0x3002,
    // --> [null#00]
    0x001e, 0x0000,
    // stop unless inst_predicate_3
    // 0007aa: ireduce.i16 (I32)
    // stop unless inst_predicate_3
    0x1003,
    // --> [null#00] and stop
    // --> [null#00] and stop
    0x001f, 0x0000,
    // end of ireduce.i16 (I32)
    // end of ireduce.i8 (I32)
    // 0007ad: regmove.i8 (I32)
    // --> [Op1rmov#89]
    0x000a, 0x0089,
    // --> [Op1rmov#89] and stop
    0x000b, 0x0089,
    // end of regmove.i8 (I32)
    // 0007b1: adjust_sp_down_imm (I32)
    // --> [Op1adjustsp_ib#5083]
    0x00c8, 0x5083,
    // --> [Op1adjustsp_id#5081] and stop
    0x00cb, 0x5081,
    // end of adjust_sp_down_imm (I32)
    // 0007b5: adjust_sp_up_imm (I32)
    // --> [Op1adjustsp_ib#83]
    0x00c8, 0x0083,
    // --> [Op1adjustsp_id#81] and stop
    0x00cb, 0x0081,
    // end of adjust_sp_up_imm (I32)
    // 0007b9: brff (I32)
    // --> [Op1brfb#70]
    0x01f8, 0x0070,
    // --> [Op2brfd#480] and stop
    0x01fd, 0x0480,
    // end of brff (I32)
    // 0007bd: brif (I32)
    // --> [Op1brib#70]
    0x01f0, 0x0070,
    // --> [Op2brid#480] and stop
    0x01f5, 0x0480,
    // end of brif (I32)
    // 0007c1: call (I32)
    // --> [Op1call_id#e8] and stop
    0x01e3, 0x00e8,
    // end of call (I32)
    // 0007c3: copy_special (I32)
    // --> [Op1copysp#89] and stop
    0x002b, 0x0089,
    // end of copy_special (I32)
    // 0007c5: f32const (I32)
    // stop unless inst_predicate_12
    0x100c,
    // --> [Op2f32imm_z#457] and stop
    0x0121, 0x0457,
    // end of f32const (I32)
    // 0007c8: f64const (I32)
    // stop unless inst_predicate_13
    0x100d,
    // --> [Mp2f64imm_z#557] and stop
    0x0123, 0x0557,
    // end of f64const (I32)
    // 0007cb: ceil.f64 (I32)
    // stop unless PredicateView(16)
    // 0007cb: floor.f64 (I32)
    // stop unless PredicateView(16)
    // 0007cb: nearest.f64 (I32)
    // stop unless PredicateView(16)
    // 0007cb: trunc.f64 (I32)
    // stop unless PredicateView(16)
    0x102f,
    // --> [Mp3furmi_rnd#d0b] and stop
    // --> [Mp3furmi_rnd#d0b] and stop
    // --> [Mp3furmi_rnd#d0b] and stop
    // --> [Mp3furmi_rnd#d0b] and stop
    0x0133, 0x0d0b,
    // end of trunc.f64 (I32)
    // end of nearest.f64 (I32)
    // end of floor.f64 (I32)
    // end of ceil.f64 (I32)
    // 0007ce: copy_to_ssa.f64 (I32)
    // --> [Mp2furm_reg_to_ssa#710] and stop
    0x0031, 0x0710,
    // end of copy_to_ssa.f64 (I32)
    // 0007d0: fcvt_from_sint.f64 (I32)
    // stop unless inst_predicate_3
    0x1003,
    // --> [DynRexMp2frurm#72a] and stop
    0x0129, 0x072a,
    // end of fcvt_from_sint.f64 (I32)
    // 0007d3: fpromote.f64 (I32)
    // stop unless inst_predicate_10
    0x100a,
    // --> [Mp2furm#65a] and stop
    0x012b, 0x065a,
    // end of fpromote.f64 (I32)
    // 0007d6: load.f64 (I32)
    // --> [Mp2fld#710]
    0x00e0, 0x0710,
    // --> [Mp2fldDisp8#710]
    0x00e4, 0x0710,
    // --> [Mp2fldDisp32#710] and stop
    0x00e9, 0x0710,
    // end of load.f64 (I32)
    // 0007dc: load_complex.f64 (I32)
    // --> [Mp2fldWithIndex#710]
    0x00ec, 0x0710,
    // --> [Mp2fldWithIndexDisp8#710]
    0x00f0, 0x0710,
    // --> [Mp2fldWithIndexDisp32#710] and stop
    0x00f5, 0x0710,
    // end of load_complex.f64 (I32)
    // 0007e2: store.f64 (I32)
    // --> [Mp2fst#711]
    0x00f8, 0x0711,
    // --> [Mp2fstDisp8#711]
    0x00fc, 0x0711,
    // --> [Mp2fstDisp32#711] and stop
    0x0101, 0x0711,
    // end of store.f64 (I32)
    // 0007e8: store_complex.f64 (I32)
    // --> [Mp2fstWithIndex#711]
    0x0104, 0x0711,
    // --> [Mp2fstWithIndexDisp8#711]
    0x0108, 0x0711,
    // --> [Mp2fstWithIndexDisp32#711] and stop
    0x010d, 0x0711,
    // end of store_complex.f64 (I32)
    // 0007ee: bitcast.f32 (I32)
    // stop unless inst_predicate_3
    // 0007ee: bitcast.i64x2 (I32)
    // stop unless inst_predicate_3
    0x1003,
    // --> [Mp2frurm#56e] and stop
    // --> [Mp2frurm#56e] and stop
    0x00d1, 0x056e,
    // end of bitcast.i64x2 (I32)
    // end of bitcast.f32 (I32)
    // 0007f1: ceil.f32 (I32)
    // stop unless PredicateView(16)
    // 0007f1: floor.f32 (I32)
    // stop unless PredicateView(16)
    // 0007f1: nearest.f32 (I32)
    // stop unless PredicateView(16)
    // 0007f1: trunc.f32 (I32)
    // stop unless PredicateView(16)
    0x102f,
    // --> [Mp3furmi_rnd#d0a] and stop
    // --> [Mp3furmi_rnd#d0a] and stop
    // --> [Mp3furmi_rnd#d0a] and stop
    // --> [Mp3furmi_rnd#d0a] and stop
    0x0133, 0x0d0a,
    // end of trunc.f32 (I32)
    // end of nearest.f32 (I32)
    // end of floor.f32 (I32)
    // end of ceil.f32 (I32)
    // 0007f4: copy_to_ssa.f32 (I32)
    // --> [Mp2furm_reg_to_ssa#610] and stop
    0x0031, 0x0610,
    // end of copy_to_ssa.f32 (I32)
    // 0007f6: fcvt_from_sint.f32 (I32)
    // stop unless inst_predicate_3
    0x1003,
    // --> [DynRexMp2frurm#62a] and stop
    0x0129, 0x062a,
    // end of fcvt_from_sint.f32 (I32)
    // 0007f9: fdemote.f32 (I32)
    // stop unless inst_predicate_11
    0x100b,
    // --> [Mp2furm#75a] and stop
    0x012b, 0x075a,
    // end of fdemote.f32 (I32)
    // 0007fc: load.f32 (I32)
    // --> [Mp2fld#610]
    0x00e0, 0x0610,
    // --> [Mp2fldDisp8#610]
    0x00e4, 0x0610,
    // --> [Mp2fldDisp32#610] and stop
    0x00e9, 0x0610,
    // end of load.f32 (I32)
    // 000802: load_complex.f32 (I32)
    // --> [Mp2fldWithIndex#610]
    0x00ec, 0x0610,
    // --> [Mp2fldWithIndexDisp8#610]
    0x00f0, 0x0610,
    // --> [Mp2fldWithIndexDisp32#610] and stop
    0x00f5, 0x0610,
    // end of load_complex.f32 (I32)
    // 000808: store.f32 (I32)
    // --> [Mp2fst#611]
    0x00f8, 0x0611,
    // --> [Mp2fstDisp8#611]
    0x00fc, 0x0611,
    // --> [Mp2fstDisp32#611] and stop
    0x0101, 0x0611,
    // end of store.f32 (I32)
    // 00080e: store_complex.f32 (I32)
    // --> [Mp2fstWithIndex#611]
    0x0104, 0x0611,
    // --> [Mp2fstWithIndexDisp8#611]
    0x0108, 0x0611,
    // --> [Mp2fstWithIndexDisp32#611] and stop
    0x010d, 0x0611,
];

/// x86 level 2 hash tables.
///
/// This hash table, keyed by instruction opcode, contains all the starting offsets for the
/// encodings interpreter, for all the CPU modes. It is jumped to after a lookup on the
/// instruction's controlling type in the level 1 hash table.
pub static LEVEL2: [Level2Entry<u16>; 1936] = [
    // I64
    // 000000: i64, 128 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfbout), offset: 0x0000c3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfborrow), offset: 0x0000c1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000002 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000017 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brnz), offset: 0x00001d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x000015 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000025 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brz), offset: 0x000021 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandImm), offset: 0x000004 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BorImm), offset: 0x000019 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BxorImm), offset: 0x000027 },
    Level2Entry { opcode: Some(crate::ir::Opcode::JumpTableBase), offset: 0x0000c5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IndirectJumpTableBr), offset: 0x000077 },
    Level2Entry { opcode: Some(crate::ir::Opcode::JumpTableEntry), offset: 0x0000c7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Rotl), offset: 0x0000df },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ishl), offset: 0x00007b },
    Level2Entry { opcode: Some(crate::ir::Opcode::RotlImm), offset: 0x0000e1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RotrImm), offset: 0x0000e5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x00007d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ushr), offset: 0x00016a },
    Level2Entry { opcode: Some(crate::ir::Opcode::CallIndirect), offset: 0x00002b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FuncAddr), offset: 0x00003f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Clz), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sshr), offset: 0x000118 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ctz), offset: 0x000038 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Popcnt), offset: 0x0000d6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x00011a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0000c9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0000cf },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x00011e },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x000124 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8), offset: 0x00015d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8Complex), offset: 0x000163 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Rotr), offset: 0x0000e3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8), offset: 0x000109 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore8), offset: 0x0000a4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore8Complex), offset: 0x0000b0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8Complex), offset: 0x00010f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16), offset: 0x000144 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16), offset: 0x0000f6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16Complex), offset: 0x0000fc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore16), offset: 0x00007f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore16Complex), offset: 0x00008b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload32), offset: 0x000151 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16Complex), offset: 0x00014a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload32), offset: 0x000103 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x000012 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore32), offset: 0x000098 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StackAddr), offset: 0x00011c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bint), offset: 0x000008 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SymbolValue), offset: 0x00012b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::GetPinnedReg), offset: 0x00004b },
    Level2Entry { opcode: Some(crate::ir::Opcode::SetPinnedReg), offset: 0x0000e9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sextend), offset: 0x0000ed },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x00005f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uextend), offset: 0x000135 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x00016c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Selectif), offset: 0x0000e7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Sdivmodx), offset: 0x000180 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000032 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000116 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00003d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0000db },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Cvtt2si), offset: 0x000172 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000036 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: Some(crate::ir::Opcode::AdjustSpDown), offset: 0x000000 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Bsf), offset: 0x00016e },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Bsr), offset: 0x000170 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IfcmpSp), offset: 0x000073 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0000dd },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0000d9 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Push), offset: 0x00017c },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pop), offset: 0x000178 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Udivmodx), offset: 0x000184 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Umulx), offset: 0x000186 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Smulx), offset: 0x000182 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x000059 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IcmpImm), offset: 0x00005b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ifcmp), offset: 0x00006d },
    Level2Entry { opcode: Some(crate::ir::Opcode::IfcmpImm), offset: 0x00006f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x00004d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x0000bd },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x000075 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddImm), offset: 0x000055 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcin), offset: 0x000051 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcout), offset: 0x000053 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcarry), offset: 0x00004f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfbin), offset: 0x0000bf },
    Level2Entry { opcode: None, offset: 0 },
    // 000080: i32, 128 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfbout), offset: 0x0001f5 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfborrow), offset: 0x0001f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000188 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000195 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brnz), offset: 0x00019b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x000193 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0001ab },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brz), offset: 0x0001a3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandImm), offset: 0x00018a },
    Level2Entry { opcode: Some(crate::ir::Opcode::BorImm), offset: 0x000197 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BxorImm), offset: 0x0001ad },
    Level2Entry { opcode: Some(crate::ir::Opcode::Rotl), offset: 0x000213 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Rotr), offset: 0x000219 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RotlImm), offset: 0x000217 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RotrImm), offset: 0x00021d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ishl), offset: 0x0001e9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ushr), offset: 0x0002b0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sshr), offset: 0x000261 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x0001ed },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x0002b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x000265 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Clz), offset: 0x0001b1 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ctz), offset: 0x0001ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Popcnt), offset: 0x000204 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x000151 },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0001f7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000098 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x000267 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8), offset: 0x000297 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8Complex), offset: 0x0002a3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8), offset: 0x000244 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8Complex), offset: 0x000250 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore8), offset: 0x0000a4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore8Complex), offset: 0x0000b0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16), offset: 0x00027e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16Complex), offset: 0x00028a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16), offset: 0x00022b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16Complex), offset: 0x000237 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore16), offset: 0x00007f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore16Complex), offset: 0x00008b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x00018e },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bint), offset: 0x000008 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ireduce), offset: 0x0001e6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uextend), offset: 0x000274 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sextend), offset: 0x000221 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x0001d5 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Selectif), offset: 0x00021f },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Sdivmodx), offset: 0x0002c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0001b6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00025d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001bf },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00003d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00020d },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Cvtt2si), offset: 0x0002ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0001b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Bsf), offset: 0x0002b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Bsr), offset: 0x0002b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Smulx), offset: 0x0002c6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00020f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000209 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Umulx), offset: 0x0002ca },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Udivmodx), offset: 0x0002c8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x0001cf },
    Level2Entry { opcode: Some(crate::ir::Opcode::IcmpImm), offset: 0x0001d1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ifcmp), offset: 0x0001de },
    Level2Entry { opcode: Some(crate::ir::Opcode::IfcmpImm), offset: 0x0001e0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x0001c3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x0001ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x0001e4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddImm), offset: 0x0001cb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcin), offset: 0x0001c7 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcout), offset: 0x0001c9 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcarry), offset: 0x0001c5 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfbin), offset: 0x0001f1 },
    Level2Entry { opcode: None, offset: 0 },
    // 000100: r64, 16 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::IsNull), offset: 0x0002d0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Null), offset: 0x0002d2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0000c9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0000d9 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0000dd },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0002cc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000116 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00003b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0000db },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000036 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsInvalid), offset: 0x0002ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x00011e },
    // 000110: b1, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0002d6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0002de },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brnz), offset: 0x0002e2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brz), offset: 0x0002ea },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000140 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0002f2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001bf },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00003d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0002f6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0001b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00025d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00020f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000209 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x0002da },
    // 000130: i8, 16 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00025d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00020f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000209 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bint), offset: 0x000008 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000140 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ireduce), offset: 0x0002ff },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001bf },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00003d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000308 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x0002fa },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0001b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    // 000140: i16, 16 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00025d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00020f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000209 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bint), offset: 0x000008 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000140 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ireduce), offset: 0x000302 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001bf },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00003d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00020d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x000068 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0001b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    // 000150: b8, 4 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00020d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x0002da },
    // 000154: b16, 4 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00020d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x0002da },
    // 000158: b32, 8 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00020d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000188 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000195 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0001ab },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x000193 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x0002da },
    // 000160: r32, 4 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000098 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x000151 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00020d },
    // 000164: b64, 8 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000002 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000017 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000025 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x000015 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x00030e },
    // 00016c: typeless, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Jump), offset: 0x00033c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload32Complex), offset: 0x0001f7 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brif), offset: 0x000320 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brff), offset: 0x000318 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopySpecial), offset: 0x00032e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Debugtrap), offset: 0x000330 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trap), offset: 0x000340 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ResumableTrap), offset: 0x000340 },
    Level2Entry { opcode: Some(crate::ir::Opcode::AdjustSpUpImm), offset: 0x000314 },
    Level2Entry { opcode: Some(crate::ir::Opcode::AdjustSpDownImm), offset: 0x000310 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore32Complex), offset: 0x000267 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Return), offset: 0x000342 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trapif), offset: 0x00034f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Call), offset: 0x000328 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Safepoint), offset: 0x000344 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trapff), offset: 0x00034d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trueif), offset: 0x000355 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload32Complex), offset: 0x000346 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::F32const), offset: 0x000332 },
    Level2Entry { opcode: Some(crate::ir::Opcode::F64const), offset: 0x000337 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trueff), offset: 0x000351 },
    // 00018c: f64, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FcvtFromSint), offset: 0x00037f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000359 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000364 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000368 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x00035d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000371 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00038d },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0003da },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmax), offset: 0x000400 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000375 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmin), offset: 0x000404 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0003e0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0003dc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0003d6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x00037b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ffcmp), offset: 0x000389 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x000377 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x00039c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x000393 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x000385 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x0003e4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0003a0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0003ac },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0003e8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0003f4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ceil), offset: 0x00036c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Floor), offset: 0x00036c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trunc), offset: 0x00036c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Nearest), offset: 0x00036c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x000361 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0003b8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fpromote), offset: 0x000397 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 0001cc: f32, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FcvtFromSint), offset: 0x00041c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000359 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000364 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000368 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x00035d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000371 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00042f },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0003da },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmax), offset: 0x00047b },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000412 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmin), offset: 0x00047f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00045b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000457 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000453 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x000418 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ffcmp), offset: 0x00042b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x000414 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x000437 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x000433 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x000427 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x00045f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00043b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000447 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000463 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x00046f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ceil), offset: 0x00040d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Floor), offset: 0x00040d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trunc), offset: 0x00040d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Nearest), offset: 0x00040d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x000408 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0003b8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdemote), offset: 0x000422 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 00020c: b8x16, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000493 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x0004ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000409 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x0004cd },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    // 00022c: b16x8, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0004d6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x0004f7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000409 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x0004fa },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    // 00024c: b32x4, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufd), offset: 0x000523 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0004fc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x00051d },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000409 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000520 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    // 00026c: b64x2, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000525 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000546 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000362 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000549 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    // 00028c: i8x16, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x0004ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x0004cd },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxs), offset: 0x000581 },
    Level2Entry { opcode: Some(crate::ir::Opcode::AvgRound), offset: 0x00054c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x000550 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmins), offset: 0x000586 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pminu), offset: 0x000589 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x00054e },
    Level2Entry { opcode: Some(crate::ir::Opcode::UaddSat), offset: 0x00057d },
    Level2Entry { opcode: Some(crate::ir::Opcode::SaddSat), offset: 0x000579 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x000556 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UsubSat), offset: 0x00057f },
    Level2Entry { opcode: Some(crate::ir::Opcode::SsubSat), offset: 0x00057b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxu), offset: 0x000584 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000558 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000409 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 0002cc: i16x8, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x000597 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x0005be },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psll), offset: 0x0005d2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x0005c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x0004fa },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psra), offset: 0x0005d4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxs), offset: 0x0005c8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::AvgRound), offset: 0x00058b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x00058f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmins), offset: 0x0005cd },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x0004f7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x00058d },
    Level2Entry { opcode: Some(crate::ir::Opcode::UaddSat), offset: 0x0005c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SaddSat), offset: 0x0005bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x000599 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UsubSat), offset: 0x0005c6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SsubSat), offset: 0x0005c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pminu), offset: 0x0005cf },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x000595 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxu), offset: 0x0005ca },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psrl), offset: 0x0005d6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x00059b },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000409 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 00030c: i32x4, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufd), offset: 0x000523 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x0005e3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x000608 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psll), offset: 0x000618 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x00060a },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000520 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psra), offset: 0x00061a },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxs), offset: 0x00060c },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x00051d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x0005da },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmins), offset: 0x000612 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pminu), offset: 0x000615 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x0005d8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x0005e5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxu), offset: 0x00060f },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psrl), offset: 0x00061c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x0005e0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0005e7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000409 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 00034c: i64x2, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x000630 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x000655 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psll), offset: 0x000659 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x000657 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000549 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psrl), offset: 0x00065b },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000546 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x000628 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x000626 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x000632 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x00061e },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000634 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000362 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 00038c: f32x4, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufd), offset: 0x000523 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Insertps), offset: 0x00069e },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000520 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x00051d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x000661 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x00065d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x000675 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x000671 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x000665 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x00069a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmin), offset: 0x00066d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmax), offset: 0x000669 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000679 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x0003d4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 0003cc: f64x2, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Movsd), offset: 0x0006e4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Movlhps), offset: 0x0006e2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000546 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000549 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x0006a5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x0006a1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x0006b9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x0006b5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x0006a9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x0006de },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmin), offset: 0x0006b1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmax), offset: 0x0006ad },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0006bd },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x0003d4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // I32
    // 00040c: i32, 128 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfbout), offset: 0x0001f5 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfborrow), offset: 0x0001f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000188 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000195 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brnz), offset: 0x0006f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x000193 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0001ab },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brz), offset: 0x0006f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandImm), offset: 0x00018a },
    Level2Entry { opcode: Some(crate::ir::Opcode::BorImm), offset: 0x000197 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BxorImm), offset: 0x0001ad },
    Level2Entry { opcode: Some(crate::ir::Opcode::JumpTableBase), offset: 0x000728 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IndirectJumpTableBr), offset: 0x000079 },
    Level2Entry { opcode: Some(crate::ir::Opcode::JumpTableEntry), offset: 0x00072a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Rotl), offset: 0x000215 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ishl), offset: 0x0001eb },
    Level2Entry { opcode: Some(crate::ir::Opcode::RotlImm), offset: 0x000217 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RotrImm), offset: 0x00021d },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x0001ed },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ushr), offset: 0x0002b2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CallIndirect), offset: 0x00002d },
    Level2Entry { opcode: Some(crate::ir::Opcode::FuncAddr), offset: 0x000701 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Clz), offset: 0x0006f9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sshr), offset: 0x000263 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ctz), offset: 0x0006fe },
    Level2Entry { opcode: Some(crate::ir::Opcode::Popcnt), offset: 0x000739 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x000265 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00072c },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000732 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x00075e },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x000764 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8), offset: 0x000781 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8Complex), offset: 0x000787 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Rotr), offset: 0x00021b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8), offset: 0x00074f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore8), offset: 0x00071b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore8Complex), offset: 0x000721 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8Complex), offset: 0x000755 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16), offset: 0x000774 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16), offset: 0x000742 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16Complex), offset: 0x000748 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore16), offset: 0x00070e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore16Complex), offset: 0x000714 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16Complex), offset: 0x00077a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x0006ee },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StackAddr), offset: 0x00075c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bint), offset: 0x0006e8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SymbolValue), offset: 0x00076b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uextend), offset: 0x00076e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sextend), offset: 0x00073c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x000707 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x0002b4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Selectif), offset: 0x00021f },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Sdivmodx), offset: 0x0002c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0001b6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00025f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001c1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00003d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0002f8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Cvtt2si), offset: 0x00078e },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0006fc },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: Some(crate::ir::Opcode::AdjustSpDown), offset: 0x0006e6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Bsf), offset: 0x0002b6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Bsr), offset: 0x0002b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IfcmpSp), offset: 0x00070c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000211 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00020b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Push), offset: 0x00017e },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pop), offset: 0x00017a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Udivmodx), offset: 0x0002c8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Umulx), offset: 0x0002ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Smulx), offset: 0x0002c6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x0001cf },
    Level2Entry { opcode: Some(crate::ir::Opcode::IcmpImm), offset: 0x0001d1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ifcmp), offset: 0x0001de },
    Level2Entry { opcode: Some(crate::ir::Opcode::IfcmpImm), offset: 0x0001e0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x0001c3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x0001ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x0001e4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddImm), offset: 0x0001cb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcin), offset: 0x0001c7 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcout), offset: 0x0001c9 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcarry), offset: 0x0001c5 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfbin), offset: 0x0001f1 },
    Level2Entry { opcode: None, offset: 0 },
    // 00048c: r32, 16 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::IsNull), offset: 0x000796 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Null), offset: 0x0002d4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00072c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00020b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000211 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000142 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00025f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001c1 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0002f8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0006fc },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsInvalid), offset: 0x000794 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x00075e },
    // 00049c: b1, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0002d8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0002e0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brnz), offset: 0x000798 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brz), offset: 0x00079e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000142 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0002f4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001c1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00003d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0002f8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0006fc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00025f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000211 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00020b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x0002dc },
    // 0004bc: i8, 16 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00025f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000211 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00020b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bint), offset: 0x0006e8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000142 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ireduce), offset: 0x0007a7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001c1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00003d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0007ad },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x0007a4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0006fc },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    // 0004cc: i16, 16 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00025f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000211 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00020b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bint), offset: 0x0006e8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000142 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ireduce), offset: 0x0007aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001c1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00003d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0002f8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x000709 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0006fc },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    // 0004dc: b8, 4 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0002f8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x0002dc },
    // 0004e0: b16, 4 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0002f8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x0002dc },
    // 0004e4: b32, 8 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0002f8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000188 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000195 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0001ab },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x000193 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x0002dc },
    // 0004ec: typeless, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Jump), offset: 0x00033c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brif), offset: 0x0007bd },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brff), offset: 0x0007b9 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopySpecial), offset: 0x0007c3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Debugtrap), offset: 0x000330 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trap), offset: 0x000340 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ResumableTrap), offset: 0x000340 },
    Level2Entry { opcode: Some(crate::ir::Opcode::AdjustSpUpImm), offset: 0x0007b5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::AdjustSpDownImm), offset: 0x0007b1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trapff), offset: 0x00034d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Return), offset: 0x000342 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trapif), offset: 0x00034f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Call), offset: 0x0007c1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Safepoint), offset: 0x000344 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trueif), offset: 0x000357 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trueff), offset: 0x000353 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::F32const), offset: 0x0007c5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::F64const), offset: 0x0007c8 },
    Level2Entry { opcode: None, offset: 0 },
    // 00050c: f64, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FcvtFromSint), offset: 0x0007d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x00035b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000366 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x00036a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x00035f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00038f },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmax), offset: 0x000402 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0007ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmin), offset: 0x000406 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0003e2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0003de },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0003d8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x00037d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ffcmp), offset: 0x00038b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x000379 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x00039e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x000395 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x000387 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x0003e6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0007d6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0007dc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0007e2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0007e8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ceil), offset: 0x0007cb },
    Level2Entry { opcode: Some(crate::ir::Opcode::Floor), offset: 0x0007cb },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trunc), offset: 0x0007cb },
    Level2Entry { opcode: Some(crate::ir::Opcode::Nearest), offset: 0x0007cb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0003b8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fpromote), offset: 0x0007d3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 00054c: f32, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FcvtFromSint), offset: 0x0007f6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x00035b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000366 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x00036a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x00035f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x000431 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmax), offset: 0x00047d },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0007f4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmin), offset: 0x000481 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00045d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000459 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000455 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x00041a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ffcmp), offset: 0x00042d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x000416 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x000439 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x000435 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x000429 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x000461 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0007fc },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000802 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000808 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x00080e },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ceil), offset: 0x0007f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Floor), offset: 0x0007f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trunc), offset: 0x0007f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Nearest), offset: 0x0007f1 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x0007ee },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0003b8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdemote), offset: 0x0007f9 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 00058c: i64, 4 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00003d },
    // 000590: b8x16, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000493 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x0004ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x00040b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x0004cd },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    // 0005b0: b16x8, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0004d6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x0004f7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x00040b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x0004fa },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    // 0005d0: b32x4, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufd), offset: 0x000523 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0004fc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x00051d },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x00040b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000520 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    // 0005f0: b64x2, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000525 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    // 000610: i8x16, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x0004ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x0004cd },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxs), offset: 0x000581 },
    Level2Entry { opcode: Some(crate::ir::Opcode::AvgRound), offset: 0x00054c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x000550 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmins), offset: 0x000586 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pminu), offset: 0x000589 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x00054e },
    Level2Entry { opcode: Some(crate::ir::Opcode::UaddSat), offset: 0x00057d },
    Level2Entry { opcode: Some(crate::ir::Opcode::SaddSat), offset: 0x000579 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x000556 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UsubSat), offset: 0x00057f },
    Level2Entry { opcode: Some(crate::ir::Opcode::SsubSat), offset: 0x00057b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxu), offset: 0x000584 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000558 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x00040b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 000650: i16x8, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x000597 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x0005be },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psll), offset: 0x0005d2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x0005c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x0004fa },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psra), offset: 0x0005d4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxs), offset: 0x0005c8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::AvgRound), offset: 0x00058b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x00058f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmins), offset: 0x0005cd },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x0004f7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x00058d },
    Level2Entry { opcode: Some(crate::ir::Opcode::UaddSat), offset: 0x0005c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SaddSat), offset: 0x0005bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x000599 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UsubSat), offset: 0x0005c6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SsubSat), offset: 0x0005c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pminu), offset: 0x0005cf },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x000595 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxu), offset: 0x0005ca },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psrl), offset: 0x0005d6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x00059b },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x00040b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 000690: i32x4, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufd), offset: 0x000523 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x0005e3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x000608 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psll), offset: 0x000618 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x00060a },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000520 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psra), offset: 0x00061a },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxs), offset: 0x00060c },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x00051d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x0005da },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmins), offset: 0x000612 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pminu), offset: 0x000615 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x0005d8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x0005e5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxu), offset: 0x00060f },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psrl), offset: 0x00061c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x0005e0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0005e7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x00040b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 0006d0: i64x2, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x000630 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x000655 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psll), offset: 0x000659 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x000657 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psrl), offset: 0x00065b },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x000628 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x000626 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x000632 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x0007ee },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000634 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 000710: f32x4, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufd), offset: 0x000523 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Insertps), offset: 0x00069e },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000520 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x00051d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x000663 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x00065f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x000677 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x000673 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x000667 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x00069c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmin), offset: 0x00066f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmax), offset: 0x00066b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000679 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x0003d4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 000750: f64x2, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004c2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000483 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000487 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000489 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000485 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00048b },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000391 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x000034 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0004d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Movsd), offset: 0x0006e4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Movlhps), offset: 0x0006e2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0004d3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x0006a7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00048d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x0006a3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x0006bb },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x0006b7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x0006ab },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x0006e0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004bc },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmin), offset: 0x0006b3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmax), offset: 0x0006af },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0006bd },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x0003d4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
];

/// x86 level 1 hash table for the CPU mode I64.
///
/// This hash table, keyed by instruction controlling type, contains all the level 2
/// hash-tables offsets for the given CPU mode, as well as a legalization identifier indicating
/// which legalization scheme to apply when the instruction doesn't have any valid encoding for
/// this CPU mode.
pub static LEVEL1_I64: [Level1Entry<u16>; 32] = [
    Level1Entry { ty: ir::types::INVALID, log2len: 5, offset: 0x00016c, legalize: 0 }, // expand_flags
    Level1Entry { ty: ir::types::F32X4, log2len: 6, offset: 0x00038c, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::B16X8, log2len: 5, offset: 0x00022c, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::B64X2, log2len: 5, offset: 0x00026c, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::I8X16, log2len: 6, offset: 0x00028c, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::B8X16, log2len: 5, offset: 0x00020c, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::I16X8, log2len: 6, offset: 0x0002cc, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::I64X2, log2len: 6, offset: 0x00034c, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::F64X2, log2len: 6, offset: 0x0003cc, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::I32X4, log2len: 6, offset: 0x00030c, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::B1, log2len: 5, offset: 0x000110, legalize: 0 }, // expand_flags
    Level1Entry { ty: ir::types::B8, log2len: 2, offset: 0x000150, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::B16, log2len: 2, offset: 0x000154, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::B32, log2len: 3, offset: 0x000158, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::B64, log2len: 3, offset: 0x000164, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::I8, log2len: 4, offset: 0x000130, legalize: 3 }, // x86_widen
    Level1Entry { ty: ir::types::I16, log2len: 4, offset: 0x000140, legalize: 3 }, // x86_widen
    Level1Entry { ty: ir::types::I32, log2len: 7, offset: 0x000080, legalize: 1 }, // x86_expand
    Level1Entry { ty: ir::types::I64, log2len: 7, offset: 0x000000, legalize: 1 }, // x86_expand
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::F32, log2len: 6, offset: 0x0001cc, legalize: 1 }, // x86_expand
    Level1Entry { ty: ir::types::F64, log2len: 6, offset: 0x00018c, legalize: 1 }, // x86_expand
    Level1Entry { ty: ir::types::B32X4, log2len: 5, offset: 0x00024c, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::R32, log2len: 2, offset: 0x000160, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::R64, log2len: 4, offset: 0x000100, legalize: 1 }, // x86_expand
];

/// x86 level 1 hash table for the CPU mode I32.
///
/// This hash table, keyed by instruction controlling type, contains all the level 2
/// hash-tables offsets for the given CPU mode, as well as a legalization identifier indicating
/// which legalization scheme to apply when the instruction doesn't have any valid encoding for
/// this CPU mode.
pub static LEVEL1_I32: [Level1Entry<u16>; 32] = [
    Level1Entry { ty: ir::types::INVALID, log2len: 5, offset: 0x0004ec, legalize: 0 }, // expand_flags
    Level1Entry { ty: ir::types::F32X4, log2len: 6, offset: 0x000710, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::B16X8, log2len: 5, offset: 0x0005b0, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::B64X2, log2len: 5, offset: 0x0005f0, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::I8X16, log2len: 6, offset: 0x000610, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::I16X8, log2len: 6, offset: 0x000650, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::I64X2, log2len: 6, offset: 0x0006d0, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::F64X2, log2len: 6, offset: 0x000750, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::I32X4, log2len: 6, offset: 0x000690, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::B1, log2len: 5, offset: 0x00049c, legalize: 0 }, // expand_flags
    Level1Entry { ty: ir::types::B8, log2len: 2, offset: 0x0004dc, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::B16, log2len: 2, offset: 0x0004e0, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::B32, log2len: 3, offset: 0x0004e4, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::B8X16, log2len: 5, offset: 0x000590, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::I8, log2len: 4, offset: 0x0004bc, legalize: 3 }, // x86_widen
    Level1Entry { ty: ir::types::I16, log2len: 4, offset: 0x0004cc, legalize: 3 }, // x86_widen
    Level1Entry { ty: ir::types::I32, log2len: 7, offset: 0x00040c, legalize: 1 }, // x86_expand
    Level1Entry { ty: ir::types::I64, log2len: 2, offset: 0x00058c, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::F32, log2len: 6, offset: 0x00054c, legalize: 1 }, // x86_expand
    Level1Entry { ty: ir::types::F64, log2len: 6, offset: 0x00050c, legalize: 1 }, // x86_expand
    Level1Entry { ty: ir::types::B32X4, log2len: 5, offset: 0x0005d0, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::R32, log2len: 4, offset: 0x00048c, legalize: 1 }, // x86_expand
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
];

/// x86 recipe names, using the same recipe index spaces as the one specified by the
/// corresponding binemit file.
static RECIPE_NAMES: [&str; 282] = [
    "get_pinned_reg",
    "RexOp1set_pinned_reg",
    "DynRexOp1umr",
    "Op1umr",
    "RexOp1umr",
    "Op1rmov",
    "RexOp1rmov",
    "Op1pu_id",
    "RexOp1pu_id",
    "RexOp1u_id",
    "RexOp1pu_iq",
    "Op1pu_id_bool",
    "RexOp1pu_id_bool",
    "Op1u_id_z",
    "RexOp1u_id_z",
    "null",
    "Op2urm_noflags_abcd",
    "RexOp2urm_noflags",
    "Op2urm_noflags",
    "RexOp1urm_noflags",
    "RexOp1copysp",
    "Op1copysp",
    "Op1umr_reg_to_ssa",
    "RexOp1umr_reg_to_ssa",
    "Mp2furm_reg_to_ssa",
    "RexMp2furm_reg_to_ssa",
    "Op1ldWithIndex",
    "RexOp1ldWithIndex",
    "Op2ldWithIndex",
    "RexOp2ldWithIndex",
    "Op1ldWithIndexDisp8",
    "RexOp1ldWithIndexDisp8",
    "Op2ldWithIndexDisp8",
    "RexOp2ldWithIndexDisp8",
    "Op1ldWithIndexDisp32",
    "RexOp1ldWithIndexDisp32",
    "Op2ldWithIndexDisp32",
    "RexOp2ldWithIndexDisp32",
    "Op1stWithIndex",
    "RexOp1stWithIndex",
    "Mp1stWithIndex",
    "RexMp1stWithIndex",
    "Op1stWithIndexDisp8",
    "RexOp1stWithIndexDisp8",
    "Mp1stWithIndexDisp8",
    "RexMp1stWithIndexDisp8",
    "Op1stWithIndexDisp32",
    "RexOp1stWithIndexDisp32",
    "Mp1stWithIndexDisp32",
    "RexMp1stWithIndexDisp32",
    "Op1stWithIndex_abcd",
    "RexOp1stWithIndex_abcd",
    "Op1stWithIndexDisp8_abcd",
    "RexOp1stWithIndexDisp8_abcd",
    "Op1stWithIndexDisp32_abcd",
    "RexOp1stWithIndexDisp32_abcd",
    "Op1st",
    "RexOp1st",
    "Mp1st",
    "RexMp1st",
    "Op1stDisp8",
    "RexOp1stDisp8",
    "Mp1stDisp8",
    "RexMp1stDisp8",
    "Op1stDisp32",
    "RexOp1stDisp32",
    "Mp1stDisp32",
    "RexMp1stDisp32",
    "Op1st_abcd",
    "Op1stDisp8_abcd",
    "Op1stDisp32_abcd",
    "Op1spillSib32",
    "RexOp1spillSib32",
    "Op1regspill32",
    "RexOp1regspill32",
    "Op1ld",
    "RexOp1ld",
    "Op2ld",
    "RexOp2ld",
    "Op1ldDisp8",
    "RexOp1ldDisp8",
    "Op2ldDisp8",
    "RexOp2ldDisp8",
    "Op1ldDisp32",
    "RexOp1ldDisp32",
    "Op2ldDisp32",
    "RexOp2ldDisp32",
    "Op1fillSib32",
    "RexOp1fillSib32",
    "Op1regfill32",
    "RexOp1regfill32",
    "fillnull",
    "ffillnull",
    "Op1pushq",
    "RexOp1pushq",
    "Op1popq",
    "RexOp1popq",
    "stacknull",
    "Op1adjustsp",
    "RexOp1adjustsp",
    "Op1adjustsp_ib",
    "Op1adjustsp_id",
    "RexOp1adjustsp_ib",
    "RexOp1adjustsp_id",
    "Mp2frurm",
    "RexMp2frurm",
    "Mp2rfumr",
    "RexMp2rfumr",
    "Op2furm",
    "RexOp2furm",
    "Op2frmov",
    "RexOp2frmov",
    "Mp2fld",
    "RexMp2fld",
    "Mp2fldDisp8",
    "RexMp2fldDisp8",
    "Mp2fldDisp32",
    "RexMp2fldDisp32",
    "Mp2fldWithIndex",
    "RexMp2fldWithIndex",
    "Mp2fldWithIndexDisp8",
    "RexMp2fldWithIndexDisp8",
    "Mp2fldWithIndexDisp32",
    "RexMp2fldWithIndexDisp32",
    "Mp2fst",
    "RexMp2fst",
    "Mp2fstDisp8",
    "RexMp2fstDisp8",
    "Mp2fstDisp32",
    "RexMp2fstDisp32",
    "Mp2fstWithIndex",
    "RexMp2fstWithIndex",
    "Mp2fstWithIndexDisp8",
    "RexMp2fstWithIndexDisp8",
    "Mp2fstWithIndexDisp32",
    "RexMp2fstWithIndexDisp32",
    "Mp2ffillSib32",
    "RexMp2ffillSib32",
    "Mp2fregfill32",
    "RexMp2fregfill32",
    "Mp2fspillSib32",
    "RexMp2fspillSib32",
    "Mp2fregspill32",
    "RexMp2fregspill32",
    "Op2f32imm_z",
    "Mp2f64imm_z",
    "RexOp2f32imm_z",
    "RexMp2f64imm_z",
    "DynRexMp2frurm",
    "Mp2furm",
    "RexMp2furm",
    "Mp2rfurm",
    "RexMp2rfurm",
    "Mp3furmi_rnd",
    "RexMp3furmi_rnd",
    "Mp2fa",
    "RexMp2fa",
    "Op2fcscc",
    "RexOp2fcscc",
    "Mp2fcscc",
    "RexMp2fcscc",
    "Op2fcmp",
    "RexOp2fcmp",
    "Mp2fcmp",
    "RexMp2fcmp",
    "DynRexOp1rr",
    "DynRexOp1rout",
    "DynRexOp1rin",
    "DynRexOp1rio",
    "DynRexOp1r_ib",
    "DynRexOp1r_id",
    "DynRexOp1ur",
    "Op1rr",
    "RexOp1rr",
    "DynRexOp2rrx",
    "DynRexOp1div",
    "DynRexOp1mulx",
    "Op2fa",
    "RexOp2fa",
    "Op2fax",
    "RexOp2fax",
    "Op1rc",
    "RexOp1rc",
    "Mp2urm",
    "RexMp2urm",
    "DynRexOp2bsf_and_bsr",
    "DynRexOp1icscc",
    "DynRexOp1icscc_ib",
    "DynRexOp1icscc_id",
    "DynRexOp1rcmp",
    "DynRexOp1rcmp_ib",
    "DynRexOp1rcmp_id",
    "Op1rcmp_sp",
    "RexOp1rcmp_sp",
    "Op2seti_abcd",
    "RexOp2seti",
    "Op2setf_abcd",
    "RexOp2setf",
    "DynRexOp2cmov",
    "Mp3fa",
    "Mp2r_ib_unsigned_fpr",
    "null_fpr",
    "Mp3r_ib_unsigned_r",
    "Mp2r_ib_unsigned_r",
    "RexMp3r_ib_unsigned_r",
    "Mp3fa_ib",
    "Mp3r_ib_unsigned_gpr",
    "RexMp3r_ib_unsigned_gpr",
    "DynRexMp2vconst_optimized",
    "DynRexOp2vconst",
    "DynRexOp2fst",
    "Op2fstDisp8",
    "Op2fstDisp32",
    "DynRexOp2fld",
    "Op2fldDisp8",
    "Op2fldDisp32",
    "Op2fspillSib32",
    "Op2fregspill32",
    "Op2ffillSib32",
    "Op2fregfill32",
    "Mp2fax",
    "Mp3fcmp",
    "Mp2f_ib",
    "Mp2icscc_fpr",
    "Mp3icscc_fpr",
    "Op2pfcmp",
    "RexOp2pfcmp",
    "Mp2pfcmp",
    "RexMp2pfcmp",
    "Op1fnaddr4",
    "RexOp1fnaddr8",
    "Op1allones_fnaddr4",
    "RexOp1allones_fnaddr8",
    "RexOp1pcrel_fnaddr8",
    "RexOp1got_fnaddr8",
    "Op1gvaddr4",
    "RexOp1gvaddr8",
    "RexOp1pcrel_gvaddr8",
    "RexOp1got_gvaddr8",
    "Op1spaddr4_id",
    "RexOp1spaddr8_id",
    "Op1call_id",
    "Op1call_plt_id",
    "Op1call_r",
    "RexOp1call_r",
    "Op1ret",
    "Op1jmpb",
    "Op1jmpd",
    "Op1brib",
    "RexOp1brib",
    "Op2brid",
    "RexOp2brid",
    "Op1brfb",
    "RexOp1brfb",
    "Op2brfd",
    "RexOp2brfd",
    "Op1tjccb",
    "RexOp1tjccb",
    "Op1tjccd",
    "RexOp1tjccd",
    "Op1t8jccd_long",
    "Op1t8jccb_abcd",
    "RexOp1t8jccb",
    "Op1t8jccd_abcd",
    "RexOp1t8jccd",
    "RexOp1jt_entry",
    "Op1jt_entry",
    "RexOp1jt_base",
    "Op1jt_base",
    "RexOp1indirect_jmp",
    "Op1indirect_jmp",
    "Op2trap",
    "debugtrap",
    "trapif",
    "trapff",
    "Op1pu_id_ref",
    "RexOp1pu_id_ref",
    "Op1is_zero",
    "RexOp1is_zero",
    "Op1is_invalid",
    "RexOp1is_invalid",
    "safepoint",
];

/// x86 recipe constraints list, using the same recipe index spaces as the one
/// specified by the corresponding binemit file. These constraints are used by register
/// allocation to select the right location to use for input and output values.
static RECIPE_CONSTRAINTS: [RecipeConstraints; 282] = [
    // Constraints for recipe get_pinned_reg:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(15),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1set_pinned_reg:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexOp1umr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1umr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1umr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1rmov:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1rmov:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1pu_id:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pu_id:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1u_id:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pu_iq:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1pu_id_bool:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pu_id_bool:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1u_id_z:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1u_id_z:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe null:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2urm_noflags_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2urm_noflags:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2urm_noflags:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1urm_noflags:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1copysp:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1copysp:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1umr_reg_to_ssa:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1umr_reg_to_ssa:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2furm_reg_to_ssa:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2furm_reg_to_ssa:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1ldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1ldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1ldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1ldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1ldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1ldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp1stWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp1stWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp1stWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp1stWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp1stWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp1stWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stWithIndex_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stWithIndex_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stWithIndexDisp8_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stWithIndexDisp8_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stWithIndexDisp32_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stWithIndexDisp32_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1st:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1st:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp1st:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp1st:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp1stDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp1stDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp1stDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp1stDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1st_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stDisp8_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stDisp32_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1spillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1spillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1regspill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1regspill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1ld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1ld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1ldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1ldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1ldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1ldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1fillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1fillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1regfill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1regfill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe fillnull:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe ffillnull:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1pushq:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pushq:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1popq:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1popq:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe stacknull:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1adjustsp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1adjustsp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1adjustsp_ib:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1adjustsp_id:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1adjustsp_ib:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1adjustsp_id:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2frurm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2frurm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2rfumr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2rfumr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2furm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2furm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2frmov:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2frmov:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fst:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fst:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fstDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fstDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fstDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fstDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fstWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fstWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fstWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fstWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fstWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fstWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2ffillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2ffillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fregfill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fregfill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fspillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fspillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fregspill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fregspill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2f32imm_z:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2f64imm_z:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2f32imm_z:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp2f64imm_z:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexMp2frurm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2furm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2furm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2rfurm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2rfurm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp3furmi_rnd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp3furmi_rnd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2fa:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp2fa:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2fcscc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2fcscc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2fcscc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp2fcscc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2fcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2fcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2fcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp2fcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1rr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1rout:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1rin:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1rio:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: true,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1r_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1r_id:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1ur:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1rr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp2rrx:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1div:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(2),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(2),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1mulx:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(2),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2fa:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2fa:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2fax:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(1),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2fax:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(1),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1rc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(1),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(1),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2urm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp2urm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp2bsf_and_bsr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1icscc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1icscc_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1icscc_id:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1rcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1rcmp_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1rcmp_id:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1rcmp_sp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rcmp_sp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2seti_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2seti:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2setf_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2setf:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexOp2cmov:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(2),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp3fa:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2r_ib_unsigned_fpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe null_fpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp3r_ib_unsigned_r:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2r_ib_unsigned_r:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp3r_ib_unsigned_r:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp3fa_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp3r_ib_unsigned_gpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp3r_ib_unsigned_gpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexMp2vconst_optimized:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexOp2vconst:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexOp2fst:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fstDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fstDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexOp2fld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fspillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fregspill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ffillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fregfill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fax:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(1),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp3fcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2f_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2icscc_fpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp3icscc_fpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2pfcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2pfcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2pfcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp2pfcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1fnaddr4:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1fnaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1allones_fnaddr4:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1allones_fnaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pcrel_fnaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1got_fnaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1gvaddr4:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1gvaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pcrel_gvaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1got_gvaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1spaddr4_id:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1spaddr8_id:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1call_id:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1call_plt_id:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1call_r:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1call_r:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1ret:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1jmpb:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1jmpd:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1brib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1brib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2brid:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2brid:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1brfb:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1brfb:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2brfd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2brfd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1tjccb:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1tjccb:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1tjccd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1tjccd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1t8jccd_long:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1t8jccb_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1t8jccb:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1t8jccd_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1t8jccd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1jt_entry:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1jt_entry:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1jt_base:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1jt_base:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1indirect_jmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1indirect_jmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2trap:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe debugtrap:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe trapif:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe trapff:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1pu_id_ref:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pu_id_ref:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1is_zero:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1is_zero:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1is_invalid:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1is_invalid:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe safepoint:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
];

/// x86 recipe sizing descriptors, using the same recipe index spaces as the one
/// specified by the corresponding binemit file. These are used to compute the final size of an
/// instruction, as well as to compute the range of branches.
static RECIPE_SIZING: [RecipeSizing; 282] = [
    // Code size information for recipe get_pinned_reg:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1set_pinned_reg:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1umr:
    RecipeSizing {
        base_size: 2,
        compute_size: size_with_inferred_rex_for_inreg0_outreg0,
        branch_range: None,
    },
    // Code size information for recipe Op1umr:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1umr:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1rmov:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rmov:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1pu_id:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pu_id:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1u_id:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pu_iq:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1pu_id_bool:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pu_id_bool:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1u_id_z:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1u_id_z:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe null:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2urm_noflags_abcd:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2urm_noflags:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2urm_noflags:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1urm_noflags:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1copysp:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1copysp:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1umr_reg_to_ssa:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1umr_reg_to_ssa:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2furm_reg_to_ssa:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2furm_reg_to_ssa:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1ldWithIndex:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ldWithIndex:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op2ldWithIndex:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ldWithIndex:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op1ldWithIndexDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ldWithIndexDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2ldWithIndexDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ldWithIndexDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1ldWithIndexDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ldWithIndexDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2ldWithIndexDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ldWithIndexDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1stWithIndex:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stWithIndex:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp1stWithIndex:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp1stWithIndex:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1stWithIndexDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stWithIndexDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp1stWithIndexDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp1stWithIndexDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1stWithIndexDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stWithIndexDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp1stWithIndexDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp1stWithIndexDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1stWithIndex_abcd:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stWithIndex_abcd:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1stWithIndexDisp8_abcd:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stWithIndexDisp8_abcd:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1stWithIndexDisp32_abcd:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stWithIndexDisp32_abcd:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1st:
    RecipeSizing {
        base_size: 2,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1st:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp1st:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp1st:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1stDisp8:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp1stDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp1stDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1stDisp32:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp1stDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp1stDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1st_abcd:
    RecipeSizing {
        base_size: 2,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1stDisp8_abcd:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1stDisp32_abcd:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1spillSib32:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1spillSib32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1regspill32:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1regspill32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1ld:
    RecipeSizing {
        base_size: 2,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ld:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op2ld:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ld:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op1ldDisp8:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ldDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op2ldDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ldDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op1ldDisp32:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ldDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op2ldDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ldDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op1fillSib32:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1fillSib32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1regfill32:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1regfill32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe fillnull:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe ffillnull:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1pushq:
    RecipeSizing {
        base_size: 1,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pushq:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1popq:
    RecipeSizing {
        base_size: 1,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1popq:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe stacknull:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1adjustsp:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1adjustsp:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1adjustsp_ib:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1adjustsp_id:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1adjustsp_ib:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1adjustsp_id:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2frurm:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2frurm:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2rfumr:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2rfumr:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2furm:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2furm:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2frmov:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2frmov:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fld:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fld:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Mp2fldDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fldDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Mp2fldDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fldDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Mp2fldWithIndex:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fldWithIndex:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Mp2fldWithIndexDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fldWithIndexDisp8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fldWithIndexDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fldWithIndexDisp32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fst:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fst:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp2fstDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fstDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp2fstDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fstDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp2fstWithIndex:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fstWithIndex:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp2fstWithIndexDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fstWithIndexDisp8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fstWithIndexDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fstWithIndexDisp32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2ffillSib32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2ffillSib32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fregfill32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fregfill32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fspillSib32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fspillSib32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fregspill32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fregspill32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2f32imm_z:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2f64imm_z:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2f32imm_z:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2f64imm_z:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp2frurm:
    RecipeSizing {
        base_size: 4,
        compute_size: size_with_inferred_rex_for_inreg0_outreg0,
        branch_range: None,
    },
    // Code size information for recipe Mp2furm:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2furm:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2rfurm:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2rfurm:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp3furmi_rnd:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp3furmi_rnd:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fa:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fa:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fcscc:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fcscc:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fcscc:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fcscc:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fcmp:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fcmp:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fcmp:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fcmp:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1rr:
    RecipeSizing {
        base_size: 2,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1rout:
    RecipeSizing {
        base_size: 2,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1rin:
    RecipeSizing {
        base_size: 2,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1rio:
    RecipeSizing {
        base_size: 2,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1r_ib:
    RecipeSizing {
        base_size: 3,
        compute_size: size_with_inferred_rex_for_inreg0,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1r_id:
    RecipeSizing {
        base_size: 6,
        compute_size: size_with_inferred_rex_for_inreg0,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1ur:
    RecipeSizing {
        base_size: 2,
        compute_size: size_with_inferred_rex_for_inreg0,
        branch_range: None,
    },
    // Code size information for recipe Op1rr:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rr:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2rrx:
    RecipeSizing {
        base_size: 3,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1div:
    RecipeSizing {
        base_size: 2,
        compute_size: size_with_inferred_rex_for_inreg2,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1mulx:
    RecipeSizing {
        base_size: 2,
        compute_size: size_with_inferred_rex_for_inreg1,
        branch_range: None,
    },
    // Code size information for recipe Op2fa:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fa:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fax:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fax:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1rc:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rc:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2urm:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2urm:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2bsf_and_bsr:
    RecipeSizing {
        base_size: 3,
        compute_size: size_with_inferred_rex_for_inreg0_outreg0,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1icscc:
    RecipeSizing {
        base_size: 5,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1icscc_ib:
    RecipeSizing {
        base_size: 6,
        compute_size: size_with_inferred_rex_for_inreg0,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1icscc_id:
    RecipeSizing {
        base_size: 9,
        compute_size: size_with_inferred_rex_for_inreg0,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1rcmp:
    RecipeSizing {
        base_size: 2,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1rcmp_ib:
    RecipeSizing {
        base_size: 3,
        compute_size: size_with_inferred_rex_for_inreg0,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1rcmp_id:
    RecipeSizing {
        base_size: 6,
        compute_size: size_with_inferred_rex_for_inreg0,
        branch_range: None,
    },
    // Code size information for recipe Op1rcmp_sp:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rcmp_sp:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2seti_abcd:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2seti:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2setf_abcd:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2setf:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2cmov:
    RecipeSizing {
        base_size: 3,
        compute_size: size_with_inferred_rex_for_cmov,
        branch_range: None,
    },
    // Code size information for recipe Mp3fa:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2r_ib_unsigned_fpr:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe null_fpr:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp3r_ib_unsigned_r:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2r_ib_unsigned_r:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp3r_ib_unsigned_r:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp3fa_ib:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp3r_ib_unsigned_gpr:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp3r_ib_unsigned_gpr:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp2vconst_optimized:
    RecipeSizing {
        base_size: 4,
        compute_size: size_with_inferred_rex_for_outreg0,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2vconst:
    RecipeSizing {
        base_size: 7,
        compute_size: size_with_inferred_rex_for_outreg0,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2fst:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_inreg1_plus_rex_prefix_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe Op2fstDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op2fstDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2fld:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_0_plus_rex_prefix_for_inreg0_outreg0,
        branch_range: None,
    },
    // Code size information for recipe Op2fldDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op2fldDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op2fspillSib32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fregspill32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2ffillSib32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fregfill32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fax:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp3fcmp:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2f_ib:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2icscc_fpr:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp3icscc_fpr:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2pfcmp:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2pfcmp:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2pfcmp:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2pfcmp:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1fnaddr4:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1fnaddr8:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1allones_fnaddr4:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1allones_fnaddr8:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pcrel_fnaddr8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1got_fnaddr8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1gvaddr4:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1gvaddr8:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pcrel_gvaddr8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1got_gvaddr8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1spaddr4_id:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1spaddr8_id:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1call_id:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1call_plt_id:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1call_r:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1call_r:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1ret:
    RecipeSizing {
        base_size: 1,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1jmpb:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 2, bits: 8 }),
    },
    // Code size information for recipe Op1jmpd:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 5, bits: 32 }),
    },
    // Code size information for recipe Op1brib:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 2, bits: 8 }),
    },
    // Code size information for recipe RexOp1brib:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 3, bits: 8 }),
    },
    // Code size information for recipe Op2brid:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 6, bits: 32 }),
    },
    // Code size information for recipe RexOp2brid:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 7, bits: 32 }),
    },
    // Code size information for recipe Op1brfb:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 2, bits: 8 }),
    },
    // Code size information for recipe RexOp1brfb:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 3, bits: 8 }),
    },
    // Code size information for recipe Op2brfd:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 6, bits: 32 }),
    },
    // Code size information for recipe RexOp2brfd:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 7, bits: 32 }),
    },
    // Code size information for recipe Op1tjccb:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 4, bits: 8 }),
    },
    // Code size information for recipe RexOp1tjccb:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 5, bits: 8 }),
    },
    // Code size information for recipe Op1tjccd:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 8, bits: 32 }),
    },
    // Code size information for recipe RexOp1tjccd:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 9, bits: 32 }),
    },
    // Code size information for recipe Op1t8jccd_long:
    RecipeSizing {
        base_size: 12,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 12, bits: 32 }),
    },
    // Code size information for recipe Op1t8jccb_abcd:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 4, bits: 8 }),
    },
    // Code size information for recipe RexOp1t8jccb:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 5, bits: 8 }),
    },
    // Code size information for recipe Op1t8jccd_abcd:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 8, bits: 32 }),
    },
    // Code size information for recipe RexOp1t8jccd:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 9, bits: 32 }),
    },
    // Code size information for recipe RexOp1jt_entry:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1jt_entry:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1jt_base:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1jt_base:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1indirect_jmp:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1indirect_jmp:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2trap:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe debugtrap:
    RecipeSizing {
        base_size: 1,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe trapif:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe trapff:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1pu_id_ref:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pu_id_ref:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1is_zero:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1is_zero:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1is_invalid:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1is_invalid:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe safepoint:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
];

pub static INFO: isa::EncInfo = isa::EncInfo {
    constraints: &RECIPE_CONSTRAINTS,
    sizing: &RECIPE_SIZING,
    names: &RECIPE_NAMES,
};
