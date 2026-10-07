; ==== Source/ModalSpecialLowering.ll
; ModuleID = 'ModalSpecialLowering'
source_filename = "ModalSpecialLowering"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aModalSpecialLowering = global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fECF582CAA5B1B50E = internal constant [4 x i8] c"\0B\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fACD58AFCAAF42074 = internal constant [4 x i8] c"\11\00\00\00", align 1

define i32 @ModalSpecialLowering_x3a_x3aModalSpecialLoweringState_x3a_x3aReady_x3a_x3areadValue(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  %2 = getelementptr i8, ptr %self, i64 0
  %3 = load i32, ptr %2, align 1
  ret i32 %3
}

define i32 @ModalSpecialLowering_x3a_x3aModalSpecialLoweringState_x3a_x3aReady_x3a_x3areplaceValue(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  %2 = getelementptr i8, ptr %self, i64 0
  %3 = getelementptr i8, ptr %self, i64 0
  %4 = load i32, ptr %value, align 4
  store i32 %4, ptr %3, align 4
  %5 = getelementptr i8, ptr %self, i64 0
  %6 = load i32, ptr %5, align 1
  ret i32 %6
}

define { i32, [0 x i32] } @ModalSpecialLowering_x3a_x3aModalSpecialLoweringState_x3a_x3aReady_x3a_x3afinish(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i32, [0 x i32] }, align 8
  %1 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %1, align 1
  %2 = getelementptr i8, ptr %1, i64 4
  store i32 0, ptr %2, align 4
  store { i32, [0 x i32] } zeroinitializer, ptr %0, align 4
  %3 = load i32, ptr %value, align 4
  %4 = getelementptr i8, ptr %0, i64 0
  store i32 %3, ptr %4, align 1
  %5 = load { i32, [0 x i32] }, ptr %0, align 4
  ret { i32, [0 x i32] } %5
}

define i32 @ModalSpecialLowering_x3a_x3amodalSpecialStateFieldReadProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aModalSpecialLowering, align 1
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
  %9 = getelementptr i8, ptr %value, i64 0
  %10 = load i32, ptr %9, align 1
  ret i32 %10
}

define i32 @ModalSpecialLowering_x3a_x3amodalSpecialStateMethodProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"ModalSpecialLowering_x3a_x3amodalSpecialStateMethodProbe$tmp$call_ref_tmp_20" = alloca i32, align 4
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %ready = alloca { i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aModalSpecialLowering, align 1
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
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 4, i1 false)
  store i32 7, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %ready, ptr align 4 %aggregate.literal, i64 4, i1 false)
  store i32 11, ptr %"ModalSpecialLowering_x3a_x3amodalSpecialStateMethodProbe$tmp$call_ref_tmp_20", align 4
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aModalSpecialLowering, align 1
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
  %16 = call i32 @ModalSpecialLowering_x3a_x3aModalSpecialLoweringState_x3a_x3aReady_x3a_x3areplaceValue(ptr noundef nonnull align 4 dereferenceable(4) %ready, ptr noundef nonnull align 4 dereferenceable(4) %"ModalSpecialLowering_x3a_x3amodalSpecialStateMethodProbe$tmp$call_ref_tmp_20", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %17 = load ptr, ptr %__panic, align 8
  %18 = load i8, ptr %17, align 1
  %19 = icmp ne i8 %18, 0
  br i1 %19, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont2
  %20 = load ptr, ptr %__panic, align 8
  %21 = getelementptr i8, ptr %20, i64 0
  %22 = load i8, ptr %21, align 1
  %23 = load ptr, ptr %__panic, align 8
  %24 = getelementptr i8, ptr %23, i64 4
  %25 = load i32, ptr %24, align 4
  %26 = load ptr, ptr %__panic, align 8
  %27 = getelementptr i8, ptr %26, i64 4
  %28 = load i32, ptr %27, align 4
  ret i32 %28

panic.cont:                                       ; preds = %poison.cont2
  ret i32 %16
}

define i32 @ModalSpecialLowering_x3a_x3amodalSpecialTransitionProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %done3 = alloca { i32, [0 x i32] }, align 4
  %"ModalSpecialLowering_x3a_x3amodalSpecialTransitionProbe$tmp$call_ref_tmp_38" = alloca i32, align 4
  %done = alloca { i32, [0 x i32] }, align 4
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %ready = alloca { i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aModalSpecialLowering, align 1
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
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 4, i1 false)
  store i32 13, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %ready, ptr align 4 %aggregate.literal, i64 4, i1 false)
  store i32 17, ptr %"ModalSpecialLowering_x3a_x3amodalSpecialTransitionProbe$tmp$call_ref_tmp_38", align 4
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aModalSpecialLowering, align 1
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
  %16 = call { i32, [0 x i32] } @ModalSpecialLowering_x3a_x3aModalSpecialLoweringState_x3a_x3aReady_x3a_x3afinish(ptr noundef nonnull align 4 dereferenceable(4) %ready, ptr noundef nonnull align 4 dereferenceable(4) %"ModalSpecialLowering_x3a_x3amodalSpecialTransitionProbe$tmp$call_ref_tmp_38", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %17 = load ptr, ptr %__panic, align 8
  %18 = load i8, ptr %17, align 1
  %19 = icmp ne i8 %18, 0
  br i1 %19, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont2
  %20 = load ptr, ptr %__panic, align 8
  %21 = getelementptr i8, ptr %20, i64 4
  %22 = load i32, ptr %21, align 4
  ret i32 %22

panic.cont:                                       ; preds = %poison.cont2
  store { i32, [0 x i32] } %16, ptr %done3, align 4
  %23 = getelementptr i8, ptr %done3, i64 0
  %24 = load i32, ptr %23, align 1
  ret i32 %24
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aModalSpecialLowering(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aModalSpecialLowering(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #1

define void @__cx_lifecycle_init_ModalSpecialLowering(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aModalSpecialLowering(ptr %0)
  ret void
}

define void @__cx_lifecycle_deinit_ModalSpecialLowering(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aModalSpecialLowering(ptr %0)
  ret void
}

attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
