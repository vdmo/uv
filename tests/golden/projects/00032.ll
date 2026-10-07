; ==== Source/FlowProofRuntimeErasure.ll
; ModuleID = 'FlowProofRuntimeErasure'
source_filename = "FlowProofRuntimeErasure"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aFlowProofRuntimeErasure = global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAD2ACA7747985764 = internal constant [4 x i8] c"\01\00\00\00", align 1

define i32 @FlowProofRuntimeErasure_x3a_x3arequirePositiveValue(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aFlowProofRuntimeErasure, align 1
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
  %9 = load i32, ptr %value, align 4
  ret i32 %9
}

define i32 @FlowProofRuntimeErasure_x3a_x3abranchValidatedValue(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aFlowProofRuntimeErasure, align 1
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
  %9 = load i32, ptr %value, align 4
  %10 = icmp sgt i32 %9, 0
  %11 = zext i1 %10 to i8
  %12 = icmp ne i8 %11, 0
  %13 = icmp ne i8 %11, 0
  br i1 %13, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  %14 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aFlowProofRuntimeErasure, align 1
  %15 = icmp ne i8 %14, 0
  br i1 %15, label %poison.take1, label %poison.cont2

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else
  ret i32 0

poison.take1:                                     ; preds = %if.then
  %16 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %16, align 1
  %17 = getelementptr i8, ptr %16, i64 4
  store i32 10, ptr %17, align 4
  %18 = load ptr, ptr %__panic, align 8
  %19 = getelementptr i8, ptr %18, i64 4
  %20 = load i32, ptr %19, align 4
  ret i32 %20

poison.cont2:                                     ; preds = %if.then
  %21 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aFlowProofRuntimeErasure, align 1
  %22 = icmp ne i8 %21, 0
  br i1 %22, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %23 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %23, align 1
  %24 = getelementptr i8, ptr %23, i64 4
  store i32 10, ptr %24, align 4
  %25 = load ptr, ptr %__panic, align 8
  %26 = getelementptr i8, ptr %25, i64 4
  %27 = load i32, ptr %26, align 4
  ret i32 %27

poison.cont4:                                     ; preds = %poison.cont2
  %28 = call i32 @FlowProofRuntimeErasure_x3a_x3arequirePositiveValue(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %29 = load ptr, ptr %__panic, align 8
  %30 = load i8, ptr %29, align 1
  %31 = icmp ne i8 %30, 0
  br i1 %31, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  %32 = load ptr, ptr %__panic, align 8
  %33 = getelementptr i8, ptr %32, i64 4
  %34 = load i32, ptr %33, align 4
  ret i32 %34

panic.cont:                                       ; preds = %poison.cont4
  ret i32 %28
}

define i32 @FlowProofRuntimeErasure_x3a_x3aFlowBehavioralReadableOwner_x3a_x3areadFlowBehavioralValue(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %self = alloca {}, align 8
  store {} zeroinitializer, ptr %self, align 1
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %entry
  %2 = load ptr, ptr %__panic, align 8
  %3 = load i8, ptr %2, align 1
  %4 = icmp ne i8 %3, 0
  br i1 %4, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %entry
  %5 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 4, ptr %6, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %7 = load ptr, ptr %__panic, align 8
  %8 = getelementptr i8, ptr %7, i64 4
  %9 = load i32, ptr %8, align 4
  ret i32 %9

panic.cont:                                       ; preds = %check_ok
  %10 = load i32, ptr %value, align 4
  %11 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %10, i32 1)
  %12 = extractvalue { i32, i1 } %11, 0
  %13 = extractvalue { i32, i1 } %11, 1
  %14 = freeze i32 %12
  br i1 %13, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  ret i32 %14

op_fail:                                          ; preds = %panic.cont
  %15 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %15, align 1
  %16 = getelementptr i8, ptr %15, i64 4
  store i32 4, ptr %16, align 4
  ret i32 0
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aFlowProofRuntimeErasure(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aFlowProofRuntimeErasure(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #0

define void @__cx_lifecycle_init_FlowProofRuntimeErasure(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aFlowProofRuntimeErasure(ptr %0)
  ret void
}

define void @__cx_lifecycle_deinit_FlowProofRuntimeErasure(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aFlowProofRuntimeErasure(ptr %0)
  ret void
}

attributes #0 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
