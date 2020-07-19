; ModuleID = 'probe1.3a1fbbbh-cgu.0'
source_filename = "probe1.3a1fbbbh-cgu.0"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>" = type { [0 x i64], %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>", [0 x i64] }
%"core::iter::adapters::StepBy<core::ops::range::Range<i32>>" = type { [0 x i64], i64, [0 x i32], { i32, i32 }, [0 x i8], i8, [7 x i8] }
%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@0 = private unnamed_addr constant <{ [27 x i8] }> <{ [27 x i8] c"assertion failed: step != 0" }>, align 1
@1 = private unnamed_addr constant <{ [32 x i8] }> <{ [32 x i8] c"src/libcore/iter/adapters/mod.rs" }>, align 1
@2 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [27 x i8] }>, <{ [27 x i8] }>* @0, i32 0, i32 0, i32 0), [8 x i8] c"\1B\00\00\00\00\00\00\00", i8* getelementptr inbounds (<{ [32 x i8] }>, <{ [32 x i8] }>* @1, i32 0, i32 0, i32 0), [16 x i8] c" \00\00\00\00\00\00\00\C5\01\00\00\09\00\00\00" }>, align 8

; core::iter::traits::iterator::Iterator::rev
; Function Attrs: inlinehint nonlazybind uwtable
define void @_ZN4core4iter6traits8iterator8Iterator3rev17hd291750d96535063E(%"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>"* noalias nocapture sret dereferenceable(24), %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture dereferenceable(24) %self) unnamed_addr #0 {
start:
  %_2 = alloca %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>", align 8
  %1 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %_2 to i8*
  %2 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %self to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %1, i8* align 8 %2, i64 24, i1 false)
; call core::iter::adapters::Rev<T>::new
  call void @"_ZN4core4iter8adapters12Rev$LT$T$GT$3new17h7f06bd5fcddc2cd2E"(%"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>"* noalias nocapture sret dereferenceable(24) %0, %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture dereferenceable(24) %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::iter::traits::iterator::Iterator::step_by
; Function Attrs: inlinehint nonlazybind uwtable
define void @_ZN4core4iter6traits8iterator8Iterator7step_by17h522e1cf38c707f77E(%"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture sret dereferenceable(24), i32 %self.0, i32 %self.1, i64 %step) unnamed_addr #0 {
start:
; call core::iter::adapters::StepBy<I>::new
  call void @"_ZN4core4iter8adapters15StepBy$LT$I$GT$3new17hbdcf0f4ead3a56eeE"(%"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture sret dereferenceable(24) %0, i32 %self.0, i32 %self.1, i64 %step)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::iter::adapters::Rev<T>::new
; Function Attrs: nonlazybind uwtable
define void @"_ZN4core4iter8adapters12Rev$LT$T$GT$3new17h7f06bd5fcddc2cd2E"(%"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>"* noalias nocapture sret dereferenceable(24), %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture dereferenceable(24) %iter) unnamed_addr #1 {
start:
  %_2 = alloca %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>", align 8
  %1 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %_2 to i8*
  %2 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %iter to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %1, i8* align 8 %2, i64 24, i1 false)
  %3 = bitcast %"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>"* %0 to %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"*
  %4 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %3 to i8*
  %5 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %_2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %4, i8* align 8 %5, i64 24, i1 false)
  ret void
}

; core::iter::adapters::StepBy<I>::new
; Function Attrs: nonlazybind uwtable
define void @"_ZN4core4iter8adapters15StepBy$LT$I$GT$3new17hbdcf0f4ead3a56eeE"(%"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture sret dereferenceable(24), i32 %iter.0, i32 %iter.1, i64 %step) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %1 = alloca { i8*, i32 }, align 8
  %2 = icmp ne i64 %step, 0
  %3 = xor i1 %2, true
  br i1 %3, label %bb3, label %bb2

bb1:                                              ; preds = %bb4
  %4 = bitcast { i8*, i32 }* %1 to i8**
  %5 = load i8*, i8** %4, align 8
  %6 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  %7 = load i32, i32* %6, align 8
  %8 = insertvalue { i8*, i32 } undef, i8* %5, 0
  %9 = insertvalue { i8*, i32 } %8, i32 %7, 1
  resume { i8*, i32 } %9

bb2:                                              ; preds = %start
  %10 = sub i64 %step, 1
  %11 = getelementptr inbounds %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>", %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %0, i32 0, i32 3
  %12 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %11, i32 0, i32 0
  store i32 %iter.0, i32* %12, align 8
  %13 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %11, i32 0, i32 1
  store i32 %iter.1, i32* %13, align 4
  %14 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %0 to i64*
  store i64 %10, i64* %14, align 8
  %15 = getelementptr inbounds %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>", %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %0, i32 0, i32 5
  store i8 1, i8* %15, align 8
  ret void

bb3:                                              ; preds = %start
; invoke core::panicking::panic
  invoke void @_ZN4core9panicking5panic17h2f49f09cf859b728E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast (<{ i8*, [8 x i8], i8*, [16 x i8] }>* @2 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
          to label %unreachable unwind label %cleanup

bb4:                                              ; preds = %cleanup
  br label %bb1

unreachable:                                      ; preds = %bb3
  unreachable

cleanup:                                          ; preds = %bb3
  %16 = landingpad { i8*, i32 }
          cleanup
  %17 = extractvalue { i8*, i32 } %16, 0
  %18 = extractvalue { i8*, i32 } %16, 1
  %19 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 0
  store i8* %17, i8** %19, align 8
  %20 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  store i32 %18, i32* %20, align 8
  br label %bb4
}

; probe1::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe15probe17h801bef6735066992E() unnamed_addr #1 {
start:
  %_3 = alloca { i32, i32 }, align 4
  %_2 = alloca %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>", align 8
  %_1 = alloca %"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>", align 8
  %0 = bitcast { i32, i32 }* %_3 to i32*
  store i32 0, i32* %0, align 4
  %1 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_3, i32 0, i32 1
  store i32 10, i32* %1, align 4
  %2 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_3, i32 0, i32 0
  %3 = load i32, i32* %2, align 4
  %4 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_3, i32 0, i32 1
  %5 = load i32, i32* %4, align 4
; call core::iter::traits::iterator::Iterator::step_by
  call void @_ZN4core4iter6traits8iterator8Iterator7step_by17h522e1cf38c707f77E(%"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture sret dereferenceable(24) %_2, i32 %3, i32 %5, i64 2)
  br label %bb1

bb1:                                              ; preds = %start
; call core::iter::traits::iterator::Iterator::rev
  call void @_ZN4core4iter6traits8iterator8Iterator3rev17hd291750d96535063E(%"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>"* noalias nocapture sret dereferenceable(24) %_1, %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture dereferenceable(24) %_2)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; Function Attrs: argmemonly nounwind
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* nocapture writeonly, i8* nocapture readonly, i64, i1 immarg) #2

; Function Attrs: nounwind nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #3

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17h2f49f09cf859b728E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40)) unnamed_addr #4

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #2 = { argmemonly nounwind }
attributes #3 = { nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #4 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"RtLibUseGOT", i32 1}
