; ==== Source/IRLoweringForms.ll
; ModuleID = 'IRLoweringForms'
source_filename = "IRLoweringForms"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aIRLoweringForms = global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAD2ACA7747985764 = internal constant [4 x i8] c"\01\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2D157A98A06F49A8 = internal constant [4 x i8] c"\0D\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2CC03B1E03CB12B8 = internal constant [4 x i8] c"\1D\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f7979A1B9CC1F91B4 = internal constant [8 x i8] c"\11\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f6D3572669B2CDE42 = internal constant [4 x i8] c"\07\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f1C894C9EAB51B351 = internal constant [8 x i8] c"\14\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fECF582CAA5B1B50E = internal constant [4 x i8] c"\0B\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fCD3AC65E44F721B1 = internal constant [4 x i8] c"\04\00\00\00", align 1

; Function Attrs: noreturn nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4)) #0

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16)) #1

; Function Attrs: nounwind
declare i64 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3amark(ptr noundef nonnull align 8 dereferenceable(16)) #1

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3areset_x5fto(ptr noundef nonnull align 8 dereferenceable(16), ptr noundef nonnull align 8 dereferenceable(8)) #1

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fenter(i64) #1

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64) #1

define i32 @IRLoweringForms_x3a_x3airLoweringLoopProbe(ptr noundef nonnull align 4 dereferenceable(4) %limit, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %total = alloca i32, align 4
  %index = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aIRLoweringForms, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  %4 = load ptr, ptr %__panic, align 8
  %5 = getelementptr i8, ptr %4, i64 4
  %6 = load i32, ptr %5, align 4
  ret i32 %6

poison.cont:                                      ; preds = %entry
  %7 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %7, align 1
  %8 = getelementptr i8, ptr %7, i64 4
  store i32 0, ptr %8, align 4
  store i32 0, ptr %index, align 4
  store i32 0, ptr %total, align 4
  br label %loop.cond

loop.end:                                         ; preds = %loop.cond
  %9 = load i32, ptr %total, align 4
  ret i32 %9

loop.cond:                                        ; preds = %op_ok5, %poison.cont
  %10 = load i32, ptr %index, align 4
  %11 = load i32, ptr %limit, align 4
  %12 = icmp slt i32 %10, %11
  %13 = zext i1 %12 to i8
  %14 = icmp ne i8 %13, 0
  br i1 %14, label %loop.body, label %loop.end

loop.body:                                        ; preds = %loop.cond
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %loop.body
  %15 = load ptr, ptr %__panic, align 8
  %16 = load i8, ptr %15, align 1
  %17 = icmp ne i8 %16, 0
  br i1 %17, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %loop.body
  %18 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %18, align 1
  %19 = getelementptr i8, ptr %18, i64 4
  store i32 4, ptr %19, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %20 = load ptr, ptr %__panic, align 8
  %21 = getelementptr i8, ptr %20, i64 4
  %22 = load i32, ptr %21, align 4
  ret i32 %22

panic.cont:                                       ; preds = %check_ok
  %23 = load i32, ptr %total, align 4
  %24 = load i32, ptr %index, align 4
  %25 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %23, i32 %24)
  %26 = extractvalue { i32, i1 } %25, 0
  %27 = extractvalue { i32, i1 } %25, 1
  %28 = freeze i32 %26
  br i1 %27, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  store i32 %28, ptr %total, align 4
  br i1 true, label %check_ok1, label %check_fail2

op_fail:                                          ; preds = %panic.cont
  %29 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %29, align 1
  %30 = getelementptr i8, ptr %29, i64 4
  store i32 4, ptr %30, align 4
  ret i32 0

check_ok1:                                        ; preds = %check_fail2, %op_ok
  %31 = load ptr, ptr %__panic, align 8
  %32 = load i8, ptr %31, align 1
  %33 = icmp ne i8 %32, 0
  br i1 %33, label %panic.take3, label %panic.cont4

check_fail2:                                      ; preds = %op_ok
  %34 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %34, align 1
  %35 = getelementptr i8, ptr %34, i64 4
  store i32 4, ptr %35, align 4
  br label %check_ok1

panic.take3:                                      ; preds = %check_ok1
  %36 = load ptr, ptr %__panic, align 8
  %37 = getelementptr i8, ptr %36, i64 4
  %38 = load i32, ptr %37, align 4
  ret i32 %38

panic.cont4:                                      ; preds = %check_ok1
  %39 = load i32, ptr %index, align 4
  %40 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %39, i32 1)
  %41 = extractvalue { i32, i1 } %40, 0
  %42 = extractvalue { i32, i1 } %40, 1
  %43 = freeze i32 %41
  br i1 %42, label %op_fail6, label %op_ok5

op_ok5:                                           ; preds = %panic.cont4
  store i32 %43, ptr %index, align 4
  br label %loop.cond

op_fail6:                                         ; preds = %panic.cont4
  %44 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %44, align 1
  %45 = getelementptr i8, ptr %44, i64 4
  store i32 4, ptr %45, align 4
  ret i32 0
}

define i32 @IRLoweringForms_x3a_x3airLoweringIfCaseProbe(ptr noundef nonnull align 4 dereferenceable(8) %choice, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"IRLoweringForms_x3a_x3airLoweringIfCaseProbe$tmp$if_case_clause_result_19" = alloca i32, align 4
  %payload2 = alloca i32, align 4
  %"IRLoweringForms_x3a_x3airLoweringIfCaseProbe$tmp$if_case_clause_result_12" = alloca i32, align 4
  %payload = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aIRLoweringForms, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  %4 = load ptr, ptr %__panic, align 8
  %5 = getelementptr i8, ptr %4, i64 4
  %6 = load i32, ptr %5, align 4
  ret i32 %6

poison.cont:                                      ; preds = %entry
  %7 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %7, align 1
  %8 = getelementptr i8, ptr %7, i64 4
  store i32 0, ptr %8, align 4
  %9 = load i8, ptr %choice, align 1
  %10 = icmp eq i8 %9, 0
  %11 = getelementptr i8, ptr %choice, i64 4
  %12 = getelementptr i8, ptr %11, i64 0
  %13 = load i32, ptr %12, align 1
  %14 = and i1 %10, true
  br i1 %14, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %op_ok7, %op_ok
  %ifcase.result = phi i32 [ %37, %op_ok ], [ %61, %op_ok7 ]
  ret i32 %ifcase.result

ifcase.case:                                      ; preds = %poison.cont
  %15 = getelementptr i8, ptr %choice, i64 4
  %16 = getelementptr i8, ptr %15, i64 0
  %17 = load i32, ptr %16, align 1
  store i32 %17, ptr %payload, align 4
  br i1 true, label %check_ok, label %check_fail

ifcase.next:                                      ; preds = %poison.cont
  %18 = load i8, ptr %choice, align 1
  %19 = icmp eq i8 %18, 1
  %20 = getelementptr i8, ptr %choice, i64 4
  %21 = getelementptr i8, ptr %20, i64 0
  %22 = load i32, ptr %21, align 1
  %23 = and i1 %19, true
  br i1 %23, label %ifcase.case1, label %ifcase.unmatched

check_ok:                                         ; preds = %check_fail, %ifcase.case
  %24 = load ptr, ptr %__panic, align 8
  %25 = load i8, ptr %24, align 1
  %26 = icmp ne i8 %25, 0
  br i1 %26, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %ifcase.case
  %27 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %27, align 1
  %28 = getelementptr i8, ptr %27, i64 4
  store i32 4, ptr %28, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %29 = load ptr, ptr %__panic, align 8
  %30 = getelementptr i8, ptr %29, i64 4
  %31 = load i32, ptr %30, align 4
  ret i32 %31

panic.cont:                                       ; preds = %check_ok
  %32 = load i32, ptr %payload, align 4
  %33 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %32, i32 1)
  %34 = extractvalue { i32, i1 } %33, 0
  %35 = extractvalue { i32, i1 } %33, 1
  %36 = freeze i32 %34
  br i1 %35, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  store i32 %36, ptr %"IRLoweringForms_x3a_x3airLoweringIfCaseProbe$tmp$if_case_clause_result_12", align 4
  %37 = load i32, ptr %"IRLoweringForms_x3a_x3airLoweringIfCaseProbe$tmp$if_case_clause_result_12", align 4
  br label %ifcase.merge

op_fail:                                          ; preds = %panic.cont
  %38 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %38, align 1
  %39 = getelementptr i8, ptr %38, i64 4
  store i32 4, ptr %39, align 4
  ret i32 0

ifcase.case1:                                     ; preds = %ifcase.next
  %40 = getelementptr i8, ptr %choice, i64 4
  %41 = getelementptr i8, ptr %40, i64 0
  %42 = load i32, ptr %41, align 1
  store i32 %42, ptr %payload2, align 4
  br i1 true, label %check_ok3, label %check_fail4

ifcase.unmatched:                                 ; preds = %ifcase.next
  %43 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %43, align 1
  %44 = getelementptr i8, ptr %43, i64 4
  store i32 18, ptr %44, align 4
  %45 = load ptr, ptr %__panic, align 8
  %46 = getelementptr i8, ptr %45, i64 4
  %47 = load i32, ptr %46, align 4
  ret i32 %47

check_ok3:                                        ; preds = %check_fail4, %ifcase.case1
  %48 = load ptr, ptr %__panic, align 8
  %49 = load i8, ptr %48, align 1
  %50 = icmp ne i8 %49, 0
  br i1 %50, label %panic.take5, label %panic.cont6

check_fail4:                                      ; preds = %ifcase.case1
  %51 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %51, align 1
  %52 = getelementptr i8, ptr %51, i64 4
  store i32 4, ptr %52, align 4
  br label %check_ok3

panic.take5:                                      ; preds = %check_ok3
  %53 = load ptr, ptr %__panic, align 8
  %54 = getelementptr i8, ptr %53, i64 4
  %55 = load i32, ptr %54, align 4
  ret i32 %55

panic.cont6:                                      ; preds = %check_ok3
  %56 = load i32, ptr %payload2, align 4
  %57 = call { i32, i1 } @llvm.ssub.with.overflow.i32(i32 %56, i32 1)
  %58 = extractvalue { i32, i1 } %57, 0
  %59 = extractvalue { i32, i1 } %57, 1
  %60 = freeze i32 %58
  br i1 %59, label %op_fail8, label %op_ok7

op_ok7:                                           ; preds = %panic.cont6
  store i32 %60, ptr %"IRLoweringForms_x3a_x3airLoweringIfCaseProbe$tmp$if_case_clause_result_19", align 4
  %61 = load i32, ptr %"IRLoweringForms_x3a_x3airLoweringIfCaseProbe$tmp$if_case_clause_result_19", align 4
  br label %ifcase.merge

op_fail8:                                         ; preds = %panic.cont6
  %62 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %62, align 1
  %63 = getelementptr i8, ptr %62, i64 4
  store i32 4, ptr %63, align 4
  ret i32 0
}

define i32 @IRLoweringForms_x3a_x3airLoweringBranchPhiProbe(ptr noundef nonnull align 1 dereferenceable(1) %flag, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aIRLoweringForms, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  %4 = load ptr, ptr %__panic, align 8
  %5 = getelementptr i8, ptr %4, i64 4
  %6 = load i32, ptr %5, align 4
  ret i32 %6

poison.cont:                                      ; preds = %entry
  %7 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %7, align 1
  %8 = getelementptr i8, ptr %7, i64 4
  store i32 0, ptr %8, align 4
  %9 = load i8, ptr %flag, align 1
  %10 = icmp ne i8 %9, 0
  %11 = load i8, ptr %flag, align 1
  %12 = icmp ne i8 %11, 0
  br i1 %12, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  br label %if.merge

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %"IRLoweringForms_x3a_x3airLoweringBranchPhiProbe$tmp$if_27" = phi i32 [ 13, %if.then ], [ 29, %if.else ]
  ret i32 %"IRLoweringForms_x3a_x3airLoweringBranchPhiProbe$tmp$if_27"
}

define i32 @IRLoweringForms_x3a_x3airLoweringRegionFrameProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %abi_return39 = alloca { i64, i64 }, align 8
  %abi_return23 = alloca { i64, i64 }, align 8
  %byref_arg15 = alloca i32, align 4
  %frame_allocated = alloca i32, align 4
  %abi_return4 = alloca { i64, i64 }, align 8
  %byref_arg3 = alloca i64, align 8
  %byref_arg2 = alloca i64, align 8
  %byref_arg1 = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %byref_arg = alloca { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] }, align 8
  %__bind_7_lowering_region = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %0 = alloca { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] }, align 8
  %score = alloca i32, align 4
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aIRLoweringForms, align 1
  %2 = icmp ne i8 %1, 0
  br i1 %2, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %3 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %3, align 1
  %4 = getelementptr i8, ptr %3, i64 4
  store i32 10, ptr %4, align 4
  %5 = load ptr, ptr %__panic, align 8
  %6 = getelementptr i8, ptr %5, i64 4
  %7 = load i32, ptr %6, align 4
  ret i32 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  store i32 0, ptr %score, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fenter(i64 17)
  store { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] } zeroinitializer, ptr %0, align 4
  %10 = getelementptr i8, ptr %0, i64 0
  store i64 0, ptr %10, align 1
  %11 = getelementptr i8, ptr %0, i64 8
  store { i8, [7 x i8], [24 x i8], [0 x i64] } zeroinitializer, ptr %11, align 1
  %12 = load { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] }, ptr %0, align 4
  store { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] } %12, ptr %byref_arg, align 4
  %13 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3anew_x5fscoped(ptr noundef nonnull align 8 dereferenceable(40) %byref_arg)
  store { i64, i64 } %13, ptr %abi_return, align 8
  %14 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return, align 1
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %14, ptr %__bind_7_lowering_region, align 4
  %15 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %__bind_7_lowering_region, align 4
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %15, ptr %byref_arg1, align 4
  store i64 4, ptr %byref_arg2, align 4
  store i64 4, ptr %byref_arg3, align 4
  %16 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aalloc(ptr noundef nonnull align 8 dereferenceable(16) %byref_arg1, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg2, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg3)
  store i32 7, ptr %16, align 4
  %17 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %__bind_7_lowering_region, align 4
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %17, ptr %byref_arg1, align 4
  store i64 4, ptr %byref_arg2, align 4
  store i64 4, ptr %byref_arg3, align 4
  %18 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aalloc(ptr noundef nonnull align 8 dereferenceable(16) %byref_arg1, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg2, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg3)
  %19 = load i32, ptr %16, align 1
  store i32 %19, ptr %18, align 4
  %20 = load i32, ptr %18, align 4
  %21 = icmp eq i32 %20, 7
  %22 = zext i1 %21 to i8
  %23 = icmp ne i8 %22, 0
  %24 = icmp ne i8 %22, 0
  br i1 %24, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  br i1 true, label %check_ok, label %check_fail

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else, %op_ok
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fenter(i64 20)
  %25 = call i64 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3amark(ptr noundef nonnull align 8 dereferenceable(16) %__bind_7_lowering_region)
  %26 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %__bind_7_lowering_region, align 4
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %26, ptr %byref_arg1, align 4
  store i64 4, ptr %byref_arg2, align 4
  store i64 4, ptr %byref_arg3, align 4
  %27 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aalloc(ptr noundef nonnull align 8 dereferenceable(16) %byref_arg1, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg2, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg3)
  store i32 11, ptr %27, align 4
  %28 = load i32, ptr %27, align 1
  store i32 %28, ptr %frame_allocated, align 4
  %29 = load i32, ptr %frame_allocated, align 4
  %30 = icmp eq i32 %29, 11
  %31 = zext i1 %30 to i8
  %32 = icmp ne i8 %31, 0
  %33 = icmp ne i8 %31, 0
  br i1 %33, label %if.then5, label %if.else6

check_ok:                                         ; preds = %check_fail, %if.then
  %34 = load ptr, ptr %__panic, align 8
  %35 = load i8, ptr %34, align 1
  %36 = icmp ne i8 %35, 0
  br i1 %36, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %if.then
  %37 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %37, align 1
  %38 = getelementptr i8, ptr %37, i64 4
  store i32 4, ptr %38, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %39 = load ptr, ptr %__panic, align 8
  %40 = getelementptr i8, ptr %39, i64 0
  %41 = load i8, ptr %40, align 1
  %42 = load ptr, ptr %__panic, align 8
  %43 = getelementptr i8, ptr %42, i64 4
  %44 = load i32, ptr %43, align 4
  %45 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %__bind_7_lowering_region)
  store { i64, i64 } %45, ptr %abi_return4, align 8
  %46 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return4, align 1
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64 17)
  %47 = load ptr, ptr %__panic, align 8
  %48 = getelementptr i8, ptr %47, i64 4
  %49 = load i32, ptr %48, align 4
  ret i32 %49

panic.cont:                                       ; preds = %check_ok
  %50 = load i32, ptr %score, align 4
  %51 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %50, i32 1)
  %52 = extractvalue { i32, i1 } %51, 0
  %53 = extractvalue { i32, i1 } %51, 1
  %54 = freeze i32 %52
  br i1 %53, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  store i32 %54, ptr %score, align 4
  br label %if.merge

op_fail:                                          ; preds = %panic.cont
  %55 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %55, align 1
  %56 = getelementptr i8, ptr %55, i64 4
  store i32 4, ptr %56, align 4
  ret i32 0

if.then5:                                         ; preds = %if.merge
  br i1 true, label %check_ok8, label %check_fail9

if.else6:                                         ; preds = %if.merge
  br label %if.merge7

if.merge7:                                        ; preds = %if.else6, %op_ok24
  %57 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %57, align 1
  %58 = getelementptr i8, ptr %57, i64 4
  store i32 0, ptr %58, align 4
  store i64 %25, ptr %byref_arg2, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3areset_x5fto(ptr noundef nonnull align 8 dereferenceable(16) %__bind_7_lowering_region, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg2)
  %59 = load ptr, ptr %__panic, align 8
  %60 = getelementptr i8, ptr %59, i64 0
  %61 = load i8, ptr %60, align 1
  %62 = load ptr, ptr %__panic, align 8
  %63 = getelementptr i8, ptr %62, i64 4
  %64 = load i32, ptr %63, align 4
  %65 = icmp ne i8 %61, 0
  %66 = zext i1 %65 to i8
  %67 = icmp ne i8 %66, 0
  %68 = and i1 false, %67
  br i1 %68, label %if.then26, label %if.else27

check_ok8:                                        ; preds = %check_fail9, %if.then5
  %69 = load ptr, ptr %__panic, align 8
  %70 = load i8, ptr %69, align 1
  %71 = icmp ne i8 %70, 0
  br i1 %71, label %panic.take10, label %panic.cont11

check_fail9:                                      ; preds = %if.then5
  %72 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %72, align 1
  %73 = getelementptr i8, ptr %72, i64 4
  store i32 4, ptr %73, align 4
  br label %check_ok8

panic.take10:                                     ; preds = %check_ok8
  %74 = load ptr, ptr %__panic, align 8
  %75 = getelementptr i8, ptr %74, i64 0
  %76 = load i8, ptr %75, align 1
  %77 = load ptr, ptr %__panic, align 8
  %78 = getelementptr i8, ptr %77, i64 4
  %79 = load i32, ptr %78, align 4
  %80 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %80, align 1
  %81 = getelementptr i8, ptr %80, i64 4
  store i32 0, ptr %81, align 4
  store i64 %25, ptr %byref_arg2, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3areset_x5fto(ptr noundef nonnull align 8 dereferenceable(16) %__bind_7_lowering_region, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg2)
  %82 = load ptr, ptr %__panic, align 8
  %83 = getelementptr i8, ptr %82, i64 0
  %84 = load i8, ptr %83, align 1
  %85 = load ptr, ptr %__panic, align 8
  %86 = getelementptr i8, ptr %85, i64 4
  %87 = load i32, ptr %86, align 4
  %88 = icmp ne i8 %84, 0
  %89 = zext i1 %88 to i8
  %90 = icmp ne i8 %89, 0
  %91 = and i1 true, %90
  br i1 %91, label %if.then12, label %if.else13

panic.cont11:                                     ; preds = %check_ok8
  %92 = load i32, ptr %score, align 4
  %93 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %92, i32 1)
  %94 = extractvalue { i32, i1 } %93, 0
  %95 = extractvalue { i32, i1 } %93, 1
  %96 = freeze i32 %94
  br i1 %95, label %op_fail25, label %op_ok24

if.then12:                                        ; preds = %panic.take10
  store i32 %87, ptr %byref_arg15, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg15)
  unreachable

if.else13:                                        ; preds = %panic.take10
  br label %if.merge14

if.merge14:                                       ; preds = %if.else13
  %97 = icmp ne i8 %84, 0
  %98 = xor i1 %97, true
  %99 = and i1 true, %98
  br i1 %99, label %if.then16, label %if.else17

if.then16:                                        ; preds = %if.merge14
  %100 = load ptr, ptr %__panic, align 8
  %101 = getelementptr i8, ptr %100, i64 0
  store i8 1, ptr %101, align 1
  %102 = load ptr, ptr %__panic, align 8
  %103 = getelementptr i8, ptr %102, i64 4
  store i32 %79, ptr %103, align 4
  br label %if.merge18

if.else17:                                        ; preds = %if.merge14
  br label %if.merge18

if.merge18:                                       ; preds = %if.else17, %if.then16
  %if.result = phi i64 [ 0, %if.then16 ], [ 0, %if.else17 ]
  %104 = icmp ne i8 %84, 0
  %105 = zext i1 %104 to i8
  %106 = icmp ne i8 %105, 0
  %107 = or i1 true, %106
  %108 = icmp ne i8 %84, 0
  %109 = icmp ne i8 %84, 0
  br i1 %109, label %if.then19, label %if.else20

if.then19:                                        ; preds = %if.merge18
  br label %if.merge21

if.else20:                                        ; preds = %if.merge18
  br label %if.merge21

if.merge21:                                       ; preds = %if.else20, %if.then19
  %if.result22 = phi i32 [ %87, %if.then19 ], [ %79, %if.else20 ]
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64 20)
  %110 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %__bind_7_lowering_region)
  store { i64, i64 } %110, ptr %abi_return23, align 8
  %111 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return23, align 1
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64 17)
  %112 = load ptr, ptr %__panic, align 8
  %113 = getelementptr i8, ptr %112, i64 4
  %114 = load i32, ptr %113, align 4
  ret i32 %114

op_ok24:                                          ; preds = %panic.cont11
  store i32 %96, ptr %score, align 4
  br label %if.merge7

op_fail25:                                        ; preds = %panic.cont11
  %115 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %115, align 1
  %116 = getelementptr i8, ptr %115, i64 4
  store i32 4, ptr %116, align 4
  ret i32 0

if.then26:                                        ; preds = %if.merge7
  store i32 %64, ptr %byref_arg15, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg15)
  unreachable

if.else27:                                        ; preds = %if.merge7
  br label %if.merge28

if.merge28:                                       ; preds = %if.else27
  %117 = icmp ne i8 %61, 0
  %118 = xor i1 %117, true
  %119 = and i1 false, %118
  br i1 %119, label %if.then29, label %if.else30

if.then29:                                        ; preds = %if.merge28
  %120 = load ptr, ptr %__panic, align 8
  %121 = getelementptr i8, ptr %120, i64 0
  store i8 1, ptr %121, align 1
  %122 = load ptr, ptr %__panic, align 8
  %123 = getelementptr i8, ptr %122, i64 4
  store i32 0, ptr %123, align 4
  br label %if.merge31

if.else30:                                        ; preds = %if.merge28
  br label %if.merge31

if.merge31:                                       ; preds = %if.else30, %if.then29
  %if.result32 = phi i64 [ 0, %if.then29 ], [ 0, %if.else30 ]
  %124 = icmp ne i8 %61, 0
  %125 = zext i1 %124 to i8
  %126 = icmp ne i8 %125, 0
  %127 = or i1 false, %126
  %128 = icmp ne i8 %61, 0
  %129 = icmp ne i8 %61, 0
  br i1 %129, label %if.then33, label %if.else34

if.then33:                                        ; preds = %if.merge31
  br label %if.merge35

if.else34:                                        ; preds = %if.merge31
  br label %if.merge35

if.merge35:                                       ; preds = %if.else34, %if.then33
  %if.result36 = phi i32 [ %64, %if.then33 ], [ 0, %if.else34 ]
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64 20)
  %130 = load ptr, ptr %__panic, align 8
  %131 = load i8, ptr %130, align 1
  %132 = icmp ne i8 %131, 0
  br i1 %132, label %panic.take37, label %panic.cont38

panic.take37:                                     ; preds = %if.merge35
  %133 = load ptr, ptr %__panic, align 8
  %134 = getelementptr i8, ptr %133, i64 0
  %135 = load i8, ptr %134, align 1
  %136 = load ptr, ptr %__panic, align 8
  %137 = getelementptr i8, ptr %136, i64 4
  %138 = load i32, ptr %137, align 4
  %139 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %__bind_7_lowering_region)
  store { i64, i64 } %139, ptr %abi_return39, align 8
  %140 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return39, align 1
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64 17)
  %141 = load ptr, ptr %__panic, align 8
  %142 = getelementptr i8, ptr %141, i64 4
  %143 = load i32, ptr %142, align 4
  ret i32 %143

panic.cont38:                                     ; preds = %if.merge35
  %144 = load i32, ptr %score, align 4
  ret i32 %144
}

define i32 @IRLoweringForms_x3a_x3airLoweringFormsProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %aggregate.literal = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 4
  %"IRLoweringForms_x3a_x3airLoweringFormsProbe$tmp$call_ref_tmp_122" = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 4
  %"IRLoweringForms_x3a_x3airLoweringFormsProbe$tmp$call_ref_tmp_116" = alloca i8, align 1
  %"IRLoweringForms_x3a_x3airLoweringFormsProbe$tmp$call_ref_tmp_112" = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aIRLoweringForms, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  %4 = load ptr, ptr %__panic, align 8
  %5 = getelementptr i8, ptr %4, i64 4
  %6 = load i32, ptr %5, align 4
  ret i32 %6

poison.cont:                                      ; preds = %entry
  %7 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %7, align 1
  %8 = getelementptr i8, ptr %7, i64 4
  store i32 0, ptr %8, align 4
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aIRLoweringForms, align 1
  %10 = icmp ne i8 %9, 0
  br i1 %10, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %11 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %11, align 1
  %12 = getelementptr i8, ptr %11, i64 4
  store i32 10, ptr %12, align 4
  %13 = load ptr, ptr %__panic, align 8
  %14 = getelementptr i8, ptr %13, i64 4
  %15 = load i32, ptr %14, align 4
  ret i32 %15

poison.cont2:                                     ; preds = %poison.cont
  store i32 4, ptr %"IRLoweringForms_x3a_x3airLoweringFormsProbe$tmp$call_ref_tmp_112", align 4
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aIRLoweringForms, align 1
  %17 = icmp ne i8 %16, 0
  br i1 %17, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %18 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %18, align 1
  %19 = getelementptr i8, ptr %18, i64 4
  store i32 10, ptr %19, align 4
  %20 = load ptr, ptr %__panic, align 8
  %21 = getelementptr i8, ptr %20, i64 4
  %22 = load i32, ptr %21, align 4
  ret i32 %22

poison.cont4:                                     ; preds = %poison.cont2
  %23 = call i32 @IRLoweringForms_x3a_x3airLoweringLoopProbe(ptr noundef nonnull align 4 dereferenceable(4) %"IRLoweringForms_x3a_x3airLoweringFormsProbe$tmp$call_ref_tmp_112", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %24 = load ptr, ptr %__panic, align 8
  %25 = load i8, ptr %24, align 1
  %26 = icmp ne i8 %25, 0
  br i1 %26, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 4
  %29 = load i32, ptr %28, align 4
  ret i32 %29

panic.cont:                                       ; preds = %poison.cont4
  %30 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aIRLoweringForms, align 1
  %31 = icmp ne i8 %30, 0
  br i1 %31, label %poison.take5, label %poison.cont6

poison.take5:                                     ; preds = %panic.cont
  %32 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %32, align 1
  %33 = getelementptr i8, ptr %32, i64 4
  store i32 10, ptr %33, align 4
  %34 = load ptr, ptr %__panic, align 8
  %35 = getelementptr i8, ptr %34, i64 4
  %36 = load i32, ptr %35, align 4
  ret i32 %36

poison.cont6:                                     ; preds = %panic.cont
  store i8 1, ptr %"IRLoweringForms_x3a_x3airLoweringFormsProbe$tmp$call_ref_tmp_116", align 1
  %37 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aIRLoweringForms, align 1
  %38 = icmp ne i8 %37, 0
  br i1 %38, label %poison.take7, label %poison.cont8

poison.take7:                                     ; preds = %poison.cont6
  %39 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %39, align 1
  %40 = getelementptr i8, ptr %39, i64 4
  store i32 10, ptr %40, align 4
  %41 = load ptr, ptr %__panic, align 8
  %42 = getelementptr i8, ptr %41, i64 4
  %43 = load i32, ptr %42, align 4
  ret i32 %43

poison.cont8:                                     ; preds = %poison.cont6
  %44 = call i32 @IRLoweringForms_x3a_x3airLoweringBranchPhiProbe(ptr noundef nonnull align 1 dereferenceable(1) %"IRLoweringForms_x3a_x3airLoweringFormsProbe$tmp$call_ref_tmp_116", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %45 = load ptr, ptr %__panic, align 8
  %46 = load i8, ptr %45, align 1
  %47 = icmp ne i8 %46, 0
  br i1 %47, label %panic.take9, label %panic.cont10

panic.take9:                                      ; preds = %poison.cont8
  %48 = load ptr, ptr %__panic, align 8
  %49 = getelementptr i8, ptr %48, i64 4
  %50 = load i32, ptr %49, align 4
  ret i32 %50

panic.cont10:                                     ; preds = %poison.cont8
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %panic.cont10
  %51 = load ptr, ptr %__panic, align 8
  %52 = load i8, ptr %51, align 1
  %53 = icmp ne i8 %52, 0
  br i1 %53, label %panic.take11, label %panic.cont12

check_fail:                                       ; preds = %panic.cont10
  %54 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %54, align 1
  %55 = getelementptr i8, ptr %54, i64 4
  store i32 4, ptr %55, align 4
  br label %check_ok

panic.take11:                                     ; preds = %check_ok
  %56 = load ptr, ptr %__panic, align 8
  %57 = getelementptr i8, ptr %56, i64 4
  %58 = load i32, ptr %57, align 4
  ret i32 %58

panic.cont12:                                     ; preds = %check_ok
  %59 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %23, i32 %44)
  %60 = extractvalue { i32, i1 } %59, 0
  %61 = extractvalue { i32, i1 } %59, 1
  %62 = freeze i32 %60
  br i1 %61, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont12
  %63 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aIRLoweringForms, align 1
  %64 = icmp ne i8 %63, 0
  br i1 %64, label %poison.take13, label %poison.cont14

op_fail:                                          ; preds = %panic.cont12
  %65 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %65, align 1
  %66 = getelementptr i8, ptr %65, i64 4
  store i32 4, ptr %66, align 4
  ret i32 0

poison.take13:                                    ; preds = %op_ok
  %67 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %67, align 1
  %68 = getelementptr i8, ptr %67, i64 4
  store i32 10, ptr %68, align 4
  %69 = load ptr, ptr %__panic, align 8
  %70 = getelementptr i8, ptr %69, i64 4
  %71 = load i32, ptr %70, align 4
  ret i32 %71

poison.cont14:                                    ; preds = %op_ok
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 8, i1 false)
  store i8 0, ptr %aggregate.literal, align 1
  %72 = getelementptr i8, ptr %aggregate.literal, i64 4
  %73 = getelementptr i8, ptr %72, i64 0
  store i32 5, ptr %73, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %"IRLoweringForms_x3a_x3airLoweringFormsProbe$tmp$call_ref_tmp_122", ptr align 4 %aggregate.literal, i64 8, i1 false)
  %74 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aIRLoweringForms, align 1
  %75 = icmp ne i8 %74, 0
  br i1 %75, label %poison.take15, label %poison.cont16

poison.take15:                                    ; preds = %poison.cont14
  %76 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %76, align 1
  %77 = getelementptr i8, ptr %76, i64 4
  store i32 10, ptr %77, align 4
  %78 = load ptr, ptr %__panic, align 8
  %79 = getelementptr i8, ptr %78, i64 4
  %80 = load i32, ptr %79, align 4
  ret i32 %80

poison.cont16:                                    ; preds = %poison.cont14
  %81 = call i32 @IRLoweringForms_x3a_x3airLoweringIfCaseProbe(ptr noundef nonnull align 4 dereferenceable(8) %"IRLoweringForms_x3a_x3airLoweringFormsProbe$tmp$call_ref_tmp_122", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %82 = load ptr, ptr %__panic, align 8
  %83 = load i8, ptr %82, align 1
  %84 = icmp ne i8 %83, 0
  br i1 %84, label %panic.take17, label %panic.cont18

panic.take17:                                     ; preds = %poison.cont16
  %85 = load ptr, ptr %__panic, align 8
  %86 = getelementptr i8, ptr %85, i64 4
  %87 = load i32, ptr %86, align 4
  ret i32 %87

panic.cont18:                                     ; preds = %poison.cont16
  br i1 true, label %check_ok19, label %check_fail20

check_ok19:                                       ; preds = %check_fail20, %panic.cont18
  %88 = load ptr, ptr %__panic, align 8
  %89 = load i8, ptr %88, align 1
  %90 = icmp ne i8 %89, 0
  br i1 %90, label %panic.take21, label %panic.cont22

check_fail20:                                     ; preds = %panic.cont18
  %91 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %91, align 1
  %92 = getelementptr i8, ptr %91, i64 4
  store i32 4, ptr %92, align 4
  br label %check_ok19

panic.take21:                                     ; preds = %check_ok19
  %93 = load ptr, ptr %__panic, align 8
  %94 = getelementptr i8, ptr %93, i64 4
  %95 = load i32, ptr %94, align 4
  ret i32 %95

panic.cont22:                                     ; preds = %check_ok19
  %96 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %62, i32 %81)
  %97 = extractvalue { i32, i1 } %96, 0
  %98 = extractvalue { i32, i1 } %96, 1
  %99 = freeze i32 %97
  br i1 %98, label %op_fail24, label %op_ok23

op_ok23:                                          ; preds = %panic.cont22
  %100 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aIRLoweringForms, align 1
  %101 = icmp ne i8 %100, 0
  br i1 %101, label %poison.take25, label %poison.cont26

op_fail24:                                        ; preds = %panic.cont22
  %102 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %102, align 1
  %103 = getelementptr i8, ptr %102, i64 4
  store i32 4, ptr %103, align 4
  ret i32 0

poison.take25:                                    ; preds = %op_ok23
  %104 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %104, align 1
  %105 = getelementptr i8, ptr %104, i64 4
  store i32 10, ptr %105, align 4
  %106 = load ptr, ptr %__panic, align 8
  %107 = getelementptr i8, ptr %106, i64 4
  %108 = load i32, ptr %107, align 4
  ret i32 %108

poison.cont26:                                    ; preds = %op_ok23
  %109 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aIRLoweringForms, align 1
  %110 = icmp ne i8 %109, 0
  br i1 %110, label %poison.take27, label %poison.cont28

poison.take27:                                    ; preds = %poison.cont26
  %111 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %111, align 1
  %112 = getelementptr i8, ptr %111, i64 4
  store i32 10, ptr %112, align 4
  %113 = load ptr, ptr %__panic, align 8
  %114 = getelementptr i8, ptr %113, i64 4
  %115 = load i32, ptr %114, align 4
  ret i32 %115

poison.cont28:                                    ; preds = %poison.cont26
  %116 = call i32 @IRLoweringForms_x3a_x3airLoweringRegionFrameProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %117 = load ptr, ptr %__panic, align 8
  %118 = load i8, ptr %117, align 1
  %119 = icmp ne i8 %118, 0
  br i1 %119, label %panic.take29, label %panic.cont30

panic.take29:                                     ; preds = %poison.cont28
  %120 = load ptr, ptr %__panic, align 8
  %121 = getelementptr i8, ptr %120, i64 4
  %122 = load i32, ptr %121, align 4
  ret i32 %122

panic.cont30:                                     ; preds = %poison.cont28
  br i1 true, label %check_ok31, label %check_fail32

check_ok31:                                       ; preds = %check_fail32, %panic.cont30
  %123 = load ptr, ptr %__panic, align 8
  %124 = load i8, ptr %123, align 1
  %125 = icmp ne i8 %124, 0
  br i1 %125, label %panic.take33, label %panic.cont34

check_fail32:                                     ; preds = %panic.cont30
  %126 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %126, align 1
  %127 = getelementptr i8, ptr %126, i64 4
  store i32 4, ptr %127, align 4
  br label %check_ok31

panic.take33:                                     ; preds = %check_ok31
  %128 = load ptr, ptr %__panic, align 8
  %129 = getelementptr i8, ptr %128, i64 4
  %130 = load i32, ptr %129, align 4
  ret i32 %130

panic.cont34:                                     ; preds = %check_ok31
  %131 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %99, i32 %116)
  %132 = extractvalue { i32, i1 } %131, 0
  %133 = extractvalue { i32, i1 } %131, 1
  %134 = freeze i32 %132
  br i1 %133, label %op_fail36, label %op_ok35

op_ok35:                                          ; preds = %panic.cont34
  ret i32 %134

op_fail36:                                        ; preds = %panic.cont34
  %135 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %135, align 1
  %136 = getelementptr i8, ptr %135, i64 4
  store i32 4, ptr %136, align 4
  ret i32 0
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aIRLoweringForms(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aIRLoweringForms(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #2

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.ssub.with.overflow.i32(i32, i32) #2

declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3anew_x5fscoped(ptr)

declare ptr @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aalloc(ptr, ptr, ptr)

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #3

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #4

define void @__cx_lifecycle_init_IRLoweringForms(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aIRLoweringForms(ptr %0)
  ret void
}

define void @__cx_lifecycle_deinit_IRLoweringForms(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aIRLoweringForms(ptr %0)
  ret void
}

attributes #0 = { noreturn nounwind }
attributes #1 = { nounwind }
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #3 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #4 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
