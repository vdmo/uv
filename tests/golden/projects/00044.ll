; ==== Source/OperatorUndefinedPanic.ll
; ModuleID = 'OperatorUndefinedPanic'
source_filename = "OperatorUndefinedPanic"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aOperatorUndefinedPanic = global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3afloat_x5fA8C7F832281A39C5 = internal constant [8 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3afloat_x5fAAB1693229BA1DB8 = internal constant [8 x i8] c"\00\00\00\00\00\00\F0?", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f8D9AADC8352FDF7F = internal constant [4 x i8] c"*\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1

define i8 @OperatorUndefinedPanic_x3a_x3aoperatorNaNComparisonsHold(ptr noundef nonnull align 8 dereferenceable(8) %divisor, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %not_a_number = alloca double, align 8
  %zero = alloca double, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aOperatorUndefinedPanic, align 1
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
  %7 = trunc i32 %6 to i8
  ret i8 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  store double 0.000000e+00, ptr %zero, align 8
  %10 = load double, ptr %zero, align 8
  %11 = load double, ptr %divisor, align 8
  %12 = fdiv double %10, %11
  store double %12, ptr %not_a_number, align 8
  %13 = load double, ptr %not_a_number, align 8
  %14 = load double, ptr %not_a_number, align 8
  %15 = fcmp oeq double %13, %14
  %16 = xor i1 %15, true
  %17 = zext i1 %16 to i8
  %18 = icmp ne i8 %17, 0
  %19 = icmp ne i8 %17, 0
  br i1 %19, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  %20 = load double, ptr %not_a_number, align 8
  %21 = load double, ptr %not_a_number, align 8
  %22 = fcmp oeq double %20, %21
  %23 = zext i1 %22 to i8
  %24 = icmp ne i8 %23, 0
  %25 = xor i1 %24, true
  %26 = zext i1 %25 to i8
  br label %if.merge

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %"OperatorUndefinedPanic_x3a_x3aoperatorNaNComparisonsHold$tmp$and_5" = phi i8 [ %26, %if.then ], [ 0, %if.else ]
  %27 = icmp ne i8 %"OperatorUndefinedPanic_x3a_x3aoperatorNaNComparisonsHold$tmp$and_5", 0
  %28 = icmp ne i8 %"OperatorUndefinedPanic_x3a_x3aoperatorNaNComparisonsHold$tmp$and_5", 0
  br i1 %28, label %if.then1, label %if.else2

if.then1:                                         ; preds = %if.merge
  %29 = load double, ptr %not_a_number, align 8
  %30 = fcmp olt double %29, 1.000000e+00
  %31 = zext i1 %30 to i8
  %32 = icmp ne i8 %31, 0
  %33 = xor i1 %32, true
  %34 = zext i1 %33 to i8
  br label %if.merge3

if.else2:                                         ; preds = %if.merge
  br label %if.merge3

if.merge3:                                        ; preds = %if.else2, %if.then1
  %"OperatorUndefinedPanic_x3a_x3aoperatorNaNComparisonsHold$tmp$and_9" = phi i8 [ %34, %if.then1 ], [ 0, %if.else2 ]
  %35 = icmp ne i8 %"OperatorUndefinedPanic_x3a_x3aoperatorNaNComparisonsHold$tmp$and_9", 0
  %36 = icmp ne i8 %"OperatorUndefinedPanic_x3a_x3aoperatorNaNComparisonsHold$tmp$and_9", 0
  br i1 %36, label %if.then4, label %if.else5

if.then4:                                         ; preds = %if.merge3
  %37 = load double, ptr %not_a_number, align 8
  %38 = fcmp ole double %37, 1.000000e+00
  %39 = zext i1 %38 to i8
  %40 = icmp ne i8 %39, 0
  %41 = xor i1 %40, true
  %42 = zext i1 %41 to i8
  br label %if.merge6

if.else5:                                         ; preds = %if.merge3
  br label %if.merge6

if.merge6:                                        ; preds = %if.else5, %if.then4
  %"OperatorUndefinedPanic_x3a_x3aoperatorNaNComparisonsHold$tmp$and_13" = phi i8 [ %42, %if.then4 ], [ 0, %if.else5 ]
  %43 = icmp ne i8 %"OperatorUndefinedPanic_x3a_x3aoperatorNaNComparisonsHold$tmp$and_13", 0
  %44 = icmp ne i8 %"OperatorUndefinedPanic_x3a_x3aoperatorNaNComparisonsHold$tmp$and_13", 0
  br i1 %44, label %if.then7, label %if.else8

if.then7:                                         ; preds = %if.merge6
  %45 = load double, ptr %not_a_number, align 8
  %46 = fcmp ogt double %45, 1.000000e+00
  %47 = zext i1 %46 to i8
  %48 = icmp ne i8 %47, 0
  %49 = xor i1 %48, true
  %50 = zext i1 %49 to i8
  br label %if.merge9

if.else8:                                         ; preds = %if.merge6
  br label %if.merge9

if.merge9:                                        ; preds = %if.else8, %if.then7
  %"OperatorUndefinedPanic_x3a_x3aoperatorNaNComparisonsHold$tmp$and_17" = phi i8 [ %50, %if.then7 ], [ 0, %if.else8 ]
  %51 = icmp ne i8 %"OperatorUndefinedPanic_x3a_x3aoperatorNaNComparisonsHold$tmp$and_17", 0
  %52 = icmp ne i8 %"OperatorUndefinedPanic_x3a_x3aoperatorNaNComparisonsHold$tmp$and_17", 0
  br i1 %52, label %if.then10, label %if.else11

if.then10:                                        ; preds = %if.merge9
  %53 = load double, ptr %not_a_number, align 8
  %54 = fcmp oge double %53, 1.000000e+00
  %55 = zext i1 %54 to i8
  %56 = icmp ne i8 %55, 0
  %57 = xor i1 %56, true
  %58 = zext i1 %57 to i8
  br label %if.merge12

if.else11:                                        ; preds = %if.merge9
  br label %if.merge12

if.merge12:                                       ; preds = %if.else11, %if.then10
  %"OperatorUndefinedPanic_x3a_x3aoperatorNaNComparisonsHold$tmp$and_21" = phi i8 [ %58, %if.then10 ], [ 0, %if.else11 ]
  %59 = icmp ne i8 %"OperatorUndefinedPanic_x3a_x3aoperatorNaNComparisonsHold$tmp$and_21", 0
  %60 = zext i1 %59 to i8
  ret i8 %60
}

define i32 @OperatorUndefinedPanic_x3a_x3aoperatorUndefinedPanicValue(ptr noundef nonnull align 4 dereferenceable(4) %divisor, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %numerator = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aOperatorUndefinedPanic, align 1
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
  store i32 42, ptr %numerator, align 4
  %9 = load i32, ptr %divisor, align 4
  %10 = icmp ne i32 %9, 0
  br i1 %10, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %11 = load ptr, ptr %__panic, align 8
  %12 = load i8, ptr %11, align 1
  %13 = icmp ne i8 %12, 0
  br i1 %13, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %14 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %14, align 1
  %15 = getelementptr i8, ptr %14, i64 4
  store i32 3, ptr %15, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %16 = load ptr, ptr %__panic, align 8
  %17 = getelementptr i8, ptr %16, i64 4
  %18 = load i32, ptr %17, align 4
  ret i32 %18

panic.cont:                                       ; preds = %check_ok
  %19 = load i32, ptr %numerator, align 4
  %20 = load i32, ptr %divisor, align 4
  %21 = icmp ne i32 %20, 0
  br i1 %21, label %check_ok1, label %check_fail2

check_ok1:                                        ; preds = %panic.cont
  %22 = icmp eq i32 %19, -2147483648
  %23 = icmp eq i32 %20, -1
  %24 = and i1 %22, %23
  %25 = xor i1 %24, true
  br i1 %25, label %check_ok3, label %check_fail4

check_fail2:                                      ; preds = %panic.cont
  %26 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %26, align 1
  %27 = getelementptr i8, ptr %26, i64 4
  store i32 3, ptr %27, align 4
  %28 = load ptr, ptr %__panic, align 8
  %29 = getelementptr i8, ptr %28, i64 4
  %30 = load i32, ptr %29, align 4
  ret i32 %30

check_ok3:                                        ; preds = %check_ok1
  %31 = sdiv i32 %19, %20
  %32 = freeze i32 %31
  ret i32 %32

check_fail4:                                      ; preds = %check_ok1
  %33 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %33, align 1
  %34 = getelementptr i8, ptr %33, i64 4
  store i32 4, ptr %34, align 4
  %35 = load ptr, ptr %__panic, align 8
  %36 = getelementptr i8, ptr %35, i64 4
  %37 = load i32, ptr %36, align 4
  ret i32 %37
}

define i32 @OperatorUndefinedPanic_x3a_x3amain(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"OperatorUndefinedPanic_x3a_x3amain$tmp$call_ref_tmp_42" = alloca i32, align 4
  %"OperatorUndefinedPanic_x3a_x3amain$tmp$call_ref_tmp_33" = alloca double, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aOperatorUndefinedPanic, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aOperatorUndefinedPanic, align 1
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
  store double 0.000000e+00, ptr %"OperatorUndefinedPanic_x3a_x3amain$tmp$call_ref_tmp_33", align 8
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aOperatorUndefinedPanic, align 1
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
  %23 = call i8 @OperatorUndefinedPanic_x3a_x3aoperatorNaNComparisonsHold(ptr noundef nonnull align 8 dereferenceable(8) %"OperatorUndefinedPanic_x3a_x3amain$tmp$call_ref_tmp_33", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
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
  %30 = icmp ne i8 %23, 0
  %31 = xor i1 %30, true
  %32 = zext i1 %31 to i8
  %33 = icmp ne i8 %32, 0
  %34 = icmp ne i8 %32, 0
  br i1 %34, label %if.then, label %if.else

if.then:                                          ; preds = %panic.cont
  ret i32 0

if.else:                                          ; preds = %panic.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else
  %35 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aOperatorUndefinedPanic, align 1
  %36 = icmp ne i8 %35, 0
  br i1 %36, label %poison.take5, label %poison.cont6

poison.take5:                                     ; preds = %if.merge
  %37 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %37, align 1
  %38 = getelementptr i8, ptr %37, i64 4
  store i32 10, ptr %38, align 4
  %39 = load ptr, ptr %__panic, align 8
  %40 = getelementptr i8, ptr %39, i64 4
  %41 = load i32, ptr %40, align 4
  ret i32 %41

poison.cont6:                                     ; preds = %if.merge
  store i32 0, ptr %"OperatorUndefinedPanic_x3a_x3amain$tmp$call_ref_tmp_42", align 4
  %42 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aOperatorUndefinedPanic, align 1
  %43 = icmp ne i8 %42, 0
  br i1 %43, label %poison.take7, label %poison.cont8

poison.take7:                                     ; preds = %poison.cont6
  %44 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %44, align 1
  %45 = getelementptr i8, ptr %44, i64 4
  store i32 10, ptr %45, align 4
  %46 = load ptr, ptr %__panic, align 8
  %47 = getelementptr i8, ptr %46, i64 4
  %48 = load i32, ptr %47, align 4
  ret i32 %48

poison.cont8:                                     ; preds = %poison.cont6
  %49 = call i32 @OperatorUndefinedPanic_x3a_x3aoperatorUndefinedPanicValue(ptr noundef nonnull align 4 dereferenceable(4) %"OperatorUndefinedPanic_x3a_x3amain$tmp$call_ref_tmp_42", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %50 = load ptr, ptr %__panic, align 8
  %51 = load i8, ptr %50, align 1
  %52 = icmp ne i8 %51, 0
  br i1 %52, label %panic.take9, label %panic.cont10

panic.take9:                                      ; preds = %poison.cont8
  %53 = load ptr, ptr %__panic, align 8
  %54 = getelementptr i8, ptr %53, i64 4
  %55 = load i32, ptr %54, align 4
  ret i32 %55

panic.cont10:                                     ; preds = %poison.cont8
  ret i32 %49
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aOperatorUndefinedPanic(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aOperatorUndefinedPanic(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define void @__cx_lifecycle_init_OperatorUndefinedPanic(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aOperatorUndefinedPanic(ptr %0)
  ret void
}

define void @__cx_lifecycle_deinit_OperatorUndefinedPanic(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aOperatorUndefinedPanic(ptr %0)
  ret void
}

define i32 @main() {
entry:
  %entry_panic_code_arg = alloca i32, align 4
  %entry_ctx_bundle = alloca { { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, [0 x i64] }, align 8
  %entry_panic = alloca { i8, [3 x i8], i32, [0 x i32] }, align 8
  store { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, ptr %entry_panic, align 4
  %entry_panic_out = alloca ptr, align 8
  store ptr %entry_panic, ptr %entry_panic_out, align 8
  %entry_ctx = alloca { { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, [0 x i64] }, align 8
  store { { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, [0 x i64] } zeroinitializer, ptr %entry_ctx, align 8
  call void @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x5finit(ptr %entry_ctx)
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aOperatorUndefinedPanic(ptr %entry_panic_out)
  %0 = load i8, ptr %entry_panic, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %entry.init.fail, label %entry.init.cont

entry.panic:                                      ; preds = %entry.deinit.panic.restore, %entry.init.cont, %entry.init.fail
  %2 = getelementptr i8, ptr %entry_panic, i64 4
  %3 = load i32, ptr %2, align 4
  store i32 %3, ptr %entry_panic_code_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %entry_panic_code_arg)
  unreachable

entry.init.fail:                                  ; preds = %entry
  br label %entry.panic

entry.init.cont:                                  ; preds = %entry
  %4 = load { { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, [0 x i64] }, ptr %entry_ctx, align 8
  store { { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, [0 x i64] } zeroinitializer, ptr %entry_ctx_bundle, align 8
  %5 = getelementptr i8, ptr %entry_ctx, i64 0
  %6 = load { ptr, ptr }, ptr %5, align 8
  %7 = getelementptr i8, ptr %entry_ctx_bundle, i64 0
  store { ptr, ptr } %6, ptr %7, align 8
  %8 = getelementptr i8, ptr %entry_ctx, i64 16
  %9 = load { ptr, ptr }, ptr %8, align 8
  %10 = getelementptr i8, ptr %entry_ctx_bundle, i64 16
  store { ptr, ptr } %9, ptr %10, align 8
  %11 = getelementptr i8, ptr %entry_ctx, i64 32
  %12 = load { ptr, ptr }, ptr %11, align 8
  %13 = getelementptr i8, ptr %entry_ctx_bundle, i64 32
  store { ptr, ptr } %12, ptr %13, align 8
  %14 = getelementptr i8, ptr %entry_ctx, i64 48
  %15 = load { ptr, ptr }, ptr %14, align 8
  %16 = getelementptr i8, ptr %entry_ctx_bundle, i64 48
  store { ptr, ptr } %15, ptr %16, align 8
  %17 = getelementptr i8, ptr %entry_ctx, i64 64
  %18 = load { ptr, ptr }, ptr %17, align 8
  %19 = getelementptr i8, ptr %entry_ctx_bundle, i64 64
  store { ptr, ptr } %18, ptr %19, align 8
  %20 = getelementptr i8, ptr %entry_ctx, i64 80
  %21 = load { ptr, ptr }, ptr %20, align 8
  %22 = getelementptr i8, ptr %entry_ctx_bundle, i64 80
  store { ptr, ptr } %21, ptr %22, align 8
  %23 = load { { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, [0 x i64] }, ptr %entry_ctx_bundle, align 8
  %24 = call i32 @OperatorUndefinedPanic_x3a_x3amain(ptr %entry_ctx, ptr %entry_panic_out)
  %entry_deinit_panic_seen = alloca i8, align 1
  %entry_deinit_panic_code = alloca i32, align 4
  store i8 0, ptr %entry_deinit_panic_seen, align 1
  store i32 0, ptr %entry_deinit_panic_code, align 4
  %25 = load i8, ptr %entry_panic, align 1
  %26 = icmp ne i8 %25, 0
  br i1 %26, label %entry.panic, label %entry.deinit

entry.deinit:                                     ; preds = %entry.init.cont
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aOperatorUndefinedPanic(ptr %entry_panic_out)
  %27 = load i8, ptr %entry_panic, align 1
  %28 = icmp ne i8 %27, 0
  br i1 %28, label %entry.deinit.panic.capture, label %entry.deinit.panic.cont

entry.deinit.panic.capture:                       ; preds = %entry.deinit
  %29 = load i8, ptr %entry_deinit_panic_seen, align 1
  %30 = icmp ne i8 %29, 0
  %31 = getelementptr i8, ptr %entry_panic, i64 4
  %32 = load i32, ptr %31, align 4
  br i1 %30, label %entry.deinit.panic.clear, label %entry.deinit.panic.store

entry.deinit.panic.cont:                          ; preds = %entry.deinit.panic.clear, %entry.deinit
  %33 = load i8, ptr %entry_deinit_panic_seen, align 1
  %34 = icmp ne i8 %33, 0
  br i1 %34, label %entry.deinit.panic.restore, label %entry.ret

entry.deinit.panic.store:                         ; preds = %entry.deinit.panic.capture
  store i8 1, ptr %entry_deinit_panic_seen, align 1
  store i32 %32, ptr %entry_deinit_panic_code, align 4
  br label %entry.deinit.panic.clear

entry.deinit.panic.clear:                         ; preds = %entry.deinit.panic.store, %entry.deinit.panic.capture
  store i8 0, ptr %entry_panic, align 1
  %35 = getelementptr i8, ptr %entry_panic, i64 4
  store i32 0, ptr %35, align 4
  br label %entry.deinit.panic.cont

entry.deinit.panic.restore:                       ; preds = %entry.deinit.panic.cont
  %36 = load i32, ptr %entry_deinit_panic_code, align 4
  store i8 1, ptr %entry_panic, align 1
  %37 = getelementptr i8, ptr %entry_panic, i64 4
  store i32 %36, ptr %37, align 4
  br label %entry.panic

entry.ret:                                        ; preds = %entry.deinit.panic.cont
  ret i32 %24
}

declare void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr)

declare void @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x5finit(ptr)
