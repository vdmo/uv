; ==== Source/ComptimeRuntimeErasure.ll
; ModuleID = 'ComptimeRuntimeErasure'
source_filename = "ComptimeRuntimeErasure"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure = global i8 0
@ComptimeRuntimeErasure_x3a_x3aRUNTIME_x5fERASURE_x5fCAPTURED_x5fVALUE = constant i32 17, align 4
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2DEA994B2809D300 = internal constant [4 x i8] c"%\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fA8C7F832281A39C5 = internal constant [8 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f89CD31291D2AEFA4 = internal constant [8 x i8] c"\01\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fC7C2BF3B330983E6 = internal constant [8 x i8] c"\03\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4BD7A317074C5B62 = internal constant [8 x i8] c"\07\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fE6BD86443DF8CE07 = internal constant [8 x i8] c"\02\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fECCAE30D575F9996 = internal constant [4 x i8] c"\13\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f6CE032EBFE88A752 = internal constant [4 x i8] c"\17\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f8D1ACE904A398D17 = internal constant [4 x i8] c"\02\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f1C894C9EAB51B351 = internal constant [8 x i8] c"\14\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fED4AC2454255EBFE = internal constant [4 x i8] c";\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fACD58AFCAAF42074 = internal constant [4 x i8] c"\11\00\00\00", align 1

define i32 @ComptimeRuntimeErasure_x3a_x3aemittedRuntimeErasureDerivedValue(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
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
  ret i32 37
}

define i8 @ComptimeRuntimeErasure_x3a_x3acompileTimeRuntimeErasureCapabilityAndReflectionChecks(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %reflected_field_count = alloca i64, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
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
  store i64 0, ptr %reflected_field_count, align 4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %10 = load ptr, ptr %__panic, align 8
  %11 = load i8, ptr %10, align 1
  %12 = icmp ne i8 %11, 0
  br i1 %12, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %13 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %13, align 1
  %14 = getelementptr i8, ptr %13, i64 4
  store i32 4, ptr %14, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %15 = load ptr, ptr %__panic, align 8
  %16 = getelementptr i8, ptr %15, i64 4
  %17 = load i32, ptr %16, align 4
  %18 = trunc i32 %17 to i8
  ret i8 %18

panic.cont:                                       ; preds = %check_ok
  %19 = load i64, ptr %reflected_field_count, align 4
  %20 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %19, i64 1)
  %21 = extractvalue { i64, i1 } %20, 0
  %22 = extractvalue { i64, i1 } %20, 1
  %23 = freeze i64 %21
  br i1 %22, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  store i64 %23, ptr %reflected_field_count, align 4
  br i1 true, label %check_ok1, label %check_fail2

op_fail:                                          ; preds = %panic.cont
  %24 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %24, align 1
  %25 = getelementptr i8, ptr %24, i64 4
  store i32 4, ptr %25, align 4
  ret i8 0

check_ok1:                                        ; preds = %check_fail2, %op_ok
  %26 = load ptr, ptr %__panic, align 8
  %27 = load i8, ptr %26, align 1
  %28 = icmp ne i8 %27, 0
  br i1 %28, label %panic.take3, label %panic.cont4

check_fail2:                                      ; preds = %op_ok
  %29 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %29, align 1
  %30 = getelementptr i8, ptr %29, i64 4
  store i32 4, ptr %30, align 4
  br label %check_ok1

panic.take3:                                      ; preds = %check_ok1
  %31 = load ptr, ptr %__panic, align 8
  %32 = getelementptr i8, ptr %31, i64 4
  %33 = load i32, ptr %32, align 4
  %34 = trunc i32 %33 to i8
  ret i8 %34

panic.cont4:                                      ; preds = %check_ok1
  %35 = load i64, ptr %reflected_field_count, align 4
  %36 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %35, i64 0)
  %37 = extractvalue { i64, i1 } %36, 0
  %38 = extractvalue { i64, i1 } %36, 1
  %39 = freeze i64 %37
  br i1 %38, label %op_fail6, label %op_ok5

op_ok5:                                           ; preds = %panic.cont4
  store i64 %39, ptr %reflected_field_count, align 4
  br i1 true, label %check_ok7, label %check_fail8

op_fail6:                                         ; preds = %panic.cont4
  %40 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %40, align 1
  %41 = getelementptr i8, ptr %40, i64 4
  store i32 4, ptr %41, align 4
  ret i8 0

check_ok7:                                        ; preds = %check_fail8, %op_ok5
  %42 = load ptr, ptr %__panic, align 8
  %43 = load i8, ptr %42, align 1
  %44 = icmp ne i8 %43, 0
  br i1 %44, label %panic.take9, label %panic.cont10

check_fail8:                                      ; preds = %op_ok5
  %45 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %45, align 1
  %46 = getelementptr i8, ptr %45, i64 4
  store i32 4, ptr %46, align 4
  br label %check_ok7

panic.take9:                                      ; preds = %check_ok7
  %47 = load ptr, ptr %__panic, align 8
  %48 = getelementptr i8, ptr %47, i64 4
  %49 = load i32, ptr %48, align 4
  %50 = trunc i32 %49 to i8
  ret i8 %50

panic.cont10:                                     ; preds = %check_ok7
  %51 = load i64, ptr %reflected_field_count, align 4
  %52 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %51, i64 1)
  %53 = extractvalue { i64, i1 } %52, 0
  %54 = extractvalue { i64, i1 } %52, 1
  %55 = freeze i64 %53
  br i1 %54, label %op_fail12, label %op_ok11

op_ok11:                                          ; preds = %panic.cont10
  store i64 %55, ptr %reflected_field_count, align 4
  br i1 true, label %check_ok13, label %check_fail14

op_fail12:                                        ; preds = %panic.cont10
  %56 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %56, align 1
  %57 = getelementptr i8, ptr %56, i64 4
  store i32 4, ptr %57, align 4
  ret i8 0

check_ok13:                                       ; preds = %check_fail14, %op_ok11
  %58 = load ptr, ptr %__panic, align 8
  %59 = load i8, ptr %58, align 1
  %60 = icmp ne i8 %59, 0
  br i1 %60, label %panic.take15, label %panic.cont16

check_fail14:                                     ; preds = %op_ok11
  %61 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %61, align 1
  %62 = getelementptr i8, ptr %61, i64 4
  store i32 4, ptr %62, align 4
  br label %check_ok13

panic.take15:                                     ; preds = %check_ok13
  %63 = load ptr, ptr %__panic, align 8
  %64 = getelementptr i8, ptr %63, i64 4
  %65 = load i32, ptr %64, align 4
  %66 = trunc i32 %65 to i8
  ret i8 %66

panic.cont16:                                     ; preds = %check_ok13
  %67 = load i64, ptr %reflected_field_count, align 4
  %68 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %67, i64 1)
  %69 = extractvalue { i64, i1 } %68, 0
  %70 = extractvalue { i64, i1 } %68, 1
  %71 = freeze i64 %69
  br i1 %70, label %op_fail18, label %op_ok17

op_ok17:                                          ; preds = %panic.cont16
  store i64 %71, ptr %reflected_field_count, align 4
  %72 = load i64, ptr %reflected_field_count, align 4
  %73 = icmp eq i64 %72, 3
  %74 = zext i1 %73 to i8
  %75 = icmp ne i8 %74, 0
  %76 = zext i1 %75 to i8
  ret i8 %76

op_fail18:                                        ; preds = %panic.cont16
  %77 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %77, align 1
  %78 = getelementptr i8, ptr %77, i64 4
  store i32 4, ptr %78, align 4
  ret i8 0
}

define i64 @ComptimeRuntimeErasure_x3a_x3acompileTimeRuntimeErasureForms(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %selected = alloca i64, align 8
  %total = alloca i64, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  store i64 7, ptr %total, align 4
  store i64 7, ptr %selected, align 4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %10 = load ptr, ptr %__panic, align 8
  %11 = load i8, ptr %10, align 1
  %12 = icmp ne i8 %11, 0
  br i1 %12, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %13 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %13, align 1
  %14 = getelementptr i8, ptr %13, i64 4
  store i32 4, ptr %14, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %15 = load ptr, ptr %__panic, align 8
  %16 = getelementptr i8, ptr %15, i64 0
  %17 = load i8, ptr %16, align 1
  %18 = load ptr, ptr %__panic, align 8
  %19 = getelementptr i8, ptr %18, i64 4
  %20 = load i32, ptr %19, align 4
  %21 = load ptr, ptr %__panic, align 8
  %22 = getelementptr i8, ptr %21, i64 4
  %23 = load i32, ptr %22, align 4
  %24 = zext i32 %23 to i64
  ret i64 %24

panic.cont:                                       ; preds = %check_ok
  %25 = load i64, ptr %total, align 4
  %26 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %25, i64 1)
  %27 = extractvalue { i64, i1 } %26, 0
  %28 = extractvalue { i64, i1 } %26, 1
  %29 = freeze i64 %27
  br i1 %28, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  store i64 %29, ptr %total, align 4
  br i1 true, label %check_ok1, label %check_fail2

op_fail:                                          ; preds = %panic.cont
  %30 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %30, align 1
  %31 = getelementptr i8, ptr %30, i64 4
  store i32 4, ptr %31, align 4
  ret i64 0

check_ok1:                                        ; preds = %check_fail2, %op_ok
  %32 = load ptr, ptr %__panic, align 8
  %33 = load i8, ptr %32, align 1
  %34 = icmp ne i8 %33, 0
  br i1 %34, label %panic.take3, label %panic.cont4

check_fail2:                                      ; preds = %op_ok
  %35 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %35, align 1
  %36 = getelementptr i8, ptr %35, i64 4
  store i32 4, ptr %36, align 4
  br label %check_ok1

panic.take3:                                      ; preds = %check_ok1
  %37 = load ptr, ptr %__panic, align 8
  %38 = getelementptr i8, ptr %37, i64 0
  %39 = load i8, ptr %38, align 1
  %40 = load ptr, ptr %__panic, align 8
  %41 = getelementptr i8, ptr %40, i64 4
  %42 = load i32, ptr %41, align 4
  %43 = load ptr, ptr %__panic, align 8
  %44 = getelementptr i8, ptr %43, i64 4
  %45 = load i32, ptr %44, align 4
  %46 = zext i32 %45 to i64
  ret i64 %46

panic.cont4:                                      ; preds = %check_ok1
  %47 = load i64, ptr %total, align 4
  %48 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %47, i64 2)
  %49 = extractvalue { i64, i1 } %48, 0
  %50 = extractvalue { i64, i1 } %48, 1
  %51 = freeze i64 %49
  br i1 %50, label %op_fail6, label %op_ok5

op_ok5:                                           ; preds = %panic.cont4
  store i64 %51, ptr %total, align 4
  br i1 true, label %check_ok7, label %check_fail8

op_fail6:                                         ; preds = %panic.cont4
  %52 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %52, align 1
  %53 = getelementptr i8, ptr %52, i64 4
  store i32 4, ptr %53, align 4
  ret i64 0

check_ok7:                                        ; preds = %check_fail8, %op_ok5
  %54 = load ptr, ptr %__panic, align 8
  %55 = load i8, ptr %54, align 1
  %56 = icmp ne i8 %55, 0
  br i1 %56, label %panic.take9, label %panic.cont10

check_fail8:                                      ; preds = %op_ok5
  %57 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %57, align 1
  %58 = getelementptr i8, ptr %57, i64 4
  store i32 4, ptr %58, align 4
  br label %check_ok7

panic.take9:                                      ; preds = %check_ok7
  %59 = load ptr, ptr %__panic, align 8
  %60 = getelementptr i8, ptr %59, i64 0
  %61 = load i8, ptr %60, align 1
  %62 = load ptr, ptr %__panic, align 8
  %63 = getelementptr i8, ptr %62, i64 4
  %64 = load i32, ptr %63, align 4
  %65 = load ptr, ptr %__panic, align 8
  %66 = getelementptr i8, ptr %65, i64 4
  %67 = load i32, ptr %66, align 4
  %68 = zext i32 %67 to i64
  ret i64 %68

panic.cont10:                                     ; preds = %check_ok7
  %69 = load i64, ptr %total, align 4
  %70 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %69, i64 3)
  %71 = extractvalue { i64, i1 } %70, 0
  %72 = extractvalue { i64, i1 } %70, 1
  %73 = freeze i64 %71
  br i1 %72, label %op_fail12, label %op_ok11

op_ok11:                                          ; preds = %panic.cont10
  store i64 %73, ptr %total, align 4
  br i1 true, label %check_ok13, label %check_fail14

op_fail12:                                        ; preds = %panic.cont10
  %74 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %74, align 1
  %75 = getelementptr i8, ptr %74, i64 4
  store i32 4, ptr %75, align 4
  ret i64 0

check_ok13:                                       ; preds = %check_fail14, %op_ok11
  %76 = load ptr, ptr %__panic, align 8
  %77 = load i8, ptr %76, align 1
  %78 = icmp ne i8 %77, 0
  br i1 %78, label %panic.take15, label %panic.cont16

check_fail14:                                     ; preds = %op_ok11
  %79 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %79, align 1
  %80 = getelementptr i8, ptr %79, i64 4
  store i32 4, ptr %80, align 4
  br label %check_ok13

panic.take15:                                     ; preds = %check_ok13
  %81 = load ptr, ptr %__panic, align 8
  %82 = getelementptr i8, ptr %81, i64 0
  %83 = load i8, ptr %82, align 1
  %84 = load ptr, ptr %__panic, align 8
  %85 = getelementptr i8, ptr %84, i64 4
  %86 = load i32, ptr %85, align 4
  %87 = load ptr, ptr %__panic, align 8
  %88 = getelementptr i8, ptr %87, i64 4
  %89 = load i32, ptr %88, align 4
  %90 = zext i32 %89 to i64
  ret i64 %90

panic.cont16:                                     ; preds = %check_ok13
  %91 = load i64, ptr %total, align 4
  %92 = load i64, ptr %selected, align 4
  %93 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %91, i64 %92)
  %94 = extractvalue { i64, i1 } %93, 0
  %95 = extractvalue { i64, i1 } %93, 1
  %96 = freeze i64 %94
  br i1 %95, label %op_fail18, label %op_ok17

op_ok17:                                          ; preds = %panic.cont16
  ret i64 %96

op_fail18:                                        ; preds = %panic.cont16
  %97 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %97, align 1
  %98 = getelementptr i8, ptr %97, i64 4
  store i32 4, ptr %98, align 4
  ret i64 0
}

define i8 @ComptimeRuntimeErasure_x3a_x3aemitRuntimeErasureQuoteSpliceItems(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
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
  ret i8 1
}

define i32 @ComptimeRuntimeErasure_x3a_x3aemittedRuntimeErasureQuoteCaptureValue(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
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
  %16 = load i32, ptr @ComptimeRuntimeErasure_x3a_x3aRUNTIME_x5fERASURE_x5fCAPTURED_x5fVALUE, align 4
  ret i32 %16
}

define i32 @ComptimeRuntimeErasure_x3a_x3aemittedRuntimeErasureSplicedValue(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
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
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %9 = load ptr, ptr %__panic, align 8
  %10 = load i8, ptr %9, align 1
  %11 = icmp ne i8 %10, 0
  br i1 %11, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %12 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %12, align 1
  %13 = getelementptr i8, ptr %12, i64 4
  store i32 4, ptr %13, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %14 = load ptr, ptr %__panic, align 8
  %15 = getelementptr i8, ptr %14, i64 4
  %16 = load i32, ptr %15, align 4
  ret i32 %16

panic.cont:                                       ; preds = %check_ok
  %17 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 19, i32 23)
  %18 = extractvalue { i32, i1 } %17, 0
  %19 = extractvalue { i32, i1 } %17, 1
  %20 = freeze i32 %18
  br i1 %19, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  ret i32 %20

op_fail:                                          ; preds = %panic.cont
  %21 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %21, align 1
  %22 = getelementptr i8, ptr %21, i64 4
  store i32 4, ptr %22, align 4
  ret i32 0
}

define i32 @ComptimeRuntimeErasure_x3a_x3aemittedRuntimeErasureIdentifierSpliceValue(ptr noundef nonnull align 4 dereferenceable(4) %runtime_erasure_input, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
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
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %9 = load ptr, ptr %__panic, align 8
  %10 = load i8, ptr %9, align 1
  %11 = icmp ne i8 %10, 0
  br i1 %11, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %12 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %12, align 1
  %13 = getelementptr i8, ptr %12, i64 4
  store i32 4, ptr %13, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %14 = load ptr, ptr %__panic, align 8
  %15 = getelementptr i8, ptr %14, i64 4
  %16 = load i32, ptr %15, align 4
  ret i32 %16

panic.cont:                                       ; preds = %check_ok
  %17 = load i32, ptr %runtime_erasure_input, align 4
  %18 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %17, i32 2)
  %19 = extractvalue { i32, i1 } %18, 0
  %20 = extractvalue { i32, i1 } %18, 1
  %21 = freeze i32 %19
  br i1 %20, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  ret i32 %21

op_fail:                                          ; preds = %panic.cont
  %22 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %22, align 1
  %23 = getelementptr i8, ptr %22, i64 4
  store i32 4, ptr %23, align 4
  ret i32 0
}

define i32 @ComptimeRuntimeErasure_x3a_x3acompileTimeRuntimeErasureEntryPoint(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"ComptimeRuntimeErasure_x3a_x3acompileTimeRuntimeErasureEntryPoint$tmp$call_ref_tmp_102" = alloca i32, align 4
  %forms_value19 = alloca i64, align 8
  %forms_value = alloca i64, align 8
  %capabilities_ok12 = alloca i8, align 1
  %capabilities_ok = alloca i8, align 1
  %emitted5 = alloca i8, align 1
  %emitted = alloca i8, align 1
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
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
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
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
  %23 = call i8 @ComptimeRuntimeErasure_x3a_x3aemitRuntimeErasureQuoteSpliceItems(ptr noundef nonnull align 8 dereferenceable(8) %__panic)
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
  %31 = zext i1 %30 to i8
  store i8 %31, ptr %emitted5, align 1
  %32 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
  %33 = icmp ne i8 %32, 0
  br i1 %33, label %poison.take6, label %poison.cont7

poison.take6:                                     ; preds = %panic.cont
  %34 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %34, align 1
  %35 = getelementptr i8, ptr %34, i64 4
  store i32 10, ptr %35, align 4
  %36 = load ptr, ptr %__panic, align 8
  %37 = getelementptr i8, ptr %36, i64 4
  %38 = load i32, ptr %37, align 4
  ret i32 %38

poison.cont7:                                     ; preds = %panic.cont
  %39 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
  %40 = icmp ne i8 %39, 0
  br i1 %40, label %poison.take8, label %poison.cont9

poison.take8:                                     ; preds = %poison.cont7
  %41 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %41, align 1
  %42 = getelementptr i8, ptr %41, i64 4
  store i32 10, ptr %42, align 4
  %43 = load ptr, ptr %__panic, align 8
  %44 = getelementptr i8, ptr %43, i64 4
  %45 = load i32, ptr %44, align 4
  ret i32 %45

poison.cont9:                                     ; preds = %poison.cont7
  %46 = call i8 @ComptimeRuntimeErasure_x3a_x3acompileTimeRuntimeErasureCapabilityAndReflectionChecks(ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %47 = load ptr, ptr %__panic, align 8
  %48 = load i8, ptr %47, align 1
  %49 = icmp ne i8 %48, 0
  br i1 %49, label %panic.take10, label %panic.cont11

panic.take10:                                     ; preds = %poison.cont9
  %50 = load ptr, ptr %__panic, align 8
  %51 = getelementptr i8, ptr %50, i64 0
  %52 = load i8, ptr %51, align 1
  %53 = load ptr, ptr %__panic, align 8
  %54 = getelementptr i8, ptr %53, i64 4
  %55 = load i32, ptr %54, align 4
  %56 = load ptr, ptr %__panic, align 8
  %57 = getelementptr i8, ptr %56, i64 4
  %58 = load i32, ptr %57, align 4
  ret i32 %58

panic.cont11:                                     ; preds = %poison.cont9
  %59 = icmp ne i8 %46, 0
  %60 = zext i1 %59 to i8
  store i8 %60, ptr %capabilities_ok12, align 1
  %61 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
  %62 = icmp ne i8 %61, 0
  br i1 %62, label %poison.take13, label %poison.cont14

poison.take13:                                    ; preds = %panic.cont11
  %63 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %63, align 1
  %64 = getelementptr i8, ptr %63, i64 4
  store i32 10, ptr %64, align 4
  %65 = load ptr, ptr %__panic, align 8
  %66 = getelementptr i8, ptr %65, i64 4
  %67 = load i32, ptr %66, align 4
  ret i32 %67

poison.cont14:                                    ; preds = %panic.cont11
  %68 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
  %69 = icmp ne i8 %68, 0
  br i1 %69, label %poison.take15, label %poison.cont16

poison.take15:                                    ; preds = %poison.cont14
  %70 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %70, align 1
  %71 = getelementptr i8, ptr %70, i64 4
  store i32 10, ptr %71, align 4
  %72 = load ptr, ptr %__panic, align 8
  %73 = getelementptr i8, ptr %72, i64 4
  %74 = load i32, ptr %73, align 4
  ret i32 %74

poison.cont16:                                    ; preds = %poison.cont14
  %75 = call i64 @ComptimeRuntimeErasure_x3a_x3acompileTimeRuntimeErasureForms(ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %76 = load ptr, ptr %__panic, align 8
  %77 = load i8, ptr %76, align 1
  %78 = icmp ne i8 %77, 0
  br i1 %78, label %panic.take17, label %panic.cont18

panic.take17:                                     ; preds = %poison.cont16
  %79 = load ptr, ptr %__panic, align 8
  %80 = getelementptr i8, ptr %79, i64 0
  %81 = load i8, ptr %80, align 1
  %82 = load ptr, ptr %__panic, align 8
  %83 = getelementptr i8, ptr %82, i64 4
  %84 = load i32, ptr %83, align 4
  %85 = load ptr, ptr %__panic, align 8
  %86 = getelementptr i8, ptr %85, i64 4
  %87 = load i32, ptr %86, align 4
  ret i32 %87

panic.cont18:                                     ; preds = %poison.cont16
  store i64 %75, ptr %forms_value19, align 4
  %88 = load i8, ptr %emitted5, align 1
  %89 = icmp ne i8 %88, 0
  %90 = load i8, ptr %emitted5, align 1
  %91 = icmp ne i8 %90, 0
  br i1 %91, label %if.then, label %if.else

if.then:                                          ; preds = %panic.cont18
  %92 = load i8, ptr %capabilities_ok12, align 1
  %93 = load i8, ptr %capabilities_ok12, align 1
  br label %if.merge

if.else:                                          ; preds = %panic.cont18
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %"ComptimeRuntimeErasure_x3a_x3acompileTimeRuntimeErasureEntryPoint$tmp$and_76" = phi i8 [ %93, %if.then ], [ 0, %if.else ]
  %94 = icmp ne i8 %"ComptimeRuntimeErasure_x3a_x3acompileTimeRuntimeErasureEntryPoint$tmp$and_76", 0
  %95 = icmp ne i8 %"ComptimeRuntimeErasure_x3a_x3acompileTimeRuntimeErasureEntryPoint$tmp$and_76", 0
  br i1 %95, label %if.then20, label %if.else21

if.then20:                                        ; preds = %if.merge
  %96 = load i64, ptr %forms_value19, align 4
  %97 = icmp eq i64 %96, 20
  %98 = zext i1 %97 to i8
  br label %if.merge22

if.else21:                                        ; preds = %if.merge
  br label %if.merge22

if.merge22:                                       ; preds = %if.else21, %if.then20
  %"ComptimeRuntimeErasure_x3a_x3acompileTimeRuntimeErasureEntryPoint$tmp$and_79" = phi i8 [ %98, %if.then20 ], [ 0, %if.else21 ]
  %99 = icmp ne i8 %"ComptimeRuntimeErasure_x3a_x3acompileTimeRuntimeErasureEntryPoint$tmp$and_79", 0
  %100 = icmp ne i8 %"ComptimeRuntimeErasure_x3a_x3acompileTimeRuntimeErasureEntryPoint$tmp$and_79", 0
  br i1 %100, label %if.then23, label %if.else24

if.then23:                                        ; preds = %if.merge22
  %101 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
  %102 = icmp ne i8 %101, 0
  br i1 %102, label %poison.take26, label %poison.cont27

if.else24:                                        ; preds = %if.merge22
  br label %if.merge25

if.merge25:                                       ; preds = %if.else24
  ret i32 0

poison.take26:                                    ; preds = %if.then23
  %103 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %103, align 1
  %104 = getelementptr i8, ptr %103, i64 4
  store i32 10, ptr %104, align 4
  %105 = load ptr, ptr %__panic, align 8
  %106 = getelementptr i8, ptr %105, i64 4
  %107 = load i32, ptr %106, align 4
  ret i32 %107

poison.cont27:                                    ; preds = %if.then23
  %108 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
  %109 = icmp ne i8 %108, 0
  br i1 %109, label %poison.take28, label %poison.cont29

poison.take28:                                    ; preds = %poison.cont27
  %110 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %110, align 1
  %111 = getelementptr i8, ptr %110, i64 4
  store i32 10, ptr %111, align 4
  %112 = load ptr, ptr %__panic, align 8
  %113 = getelementptr i8, ptr %112, i64 4
  %114 = load i32, ptr %113, align 4
  ret i32 %114

poison.cont29:                                    ; preds = %poison.cont27
  %115 = call i32 @ComptimeRuntimeErasure_x3a_x3aemittedRuntimeErasureQuoteCaptureValue(ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %116 = load ptr, ptr %__panic, align 8
  %117 = load i8, ptr %116, align 1
  %118 = icmp ne i8 %117, 0
  br i1 %118, label %panic.take30, label %panic.cont31

panic.take30:                                     ; preds = %poison.cont29
  %119 = load ptr, ptr %__panic, align 8
  %120 = getelementptr i8, ptr %119, i64 0
  %121 = load i8, ptr %120, align 1
  %122 = load ptr, ptr %__panic, align 8
  %123 = getelementptr i8, ptr %122, i64 4
  %124 = load i32, ptr %123, align 4
  %125 = load ptr, ptr %__panic, align 8
  %126 = getelementptr i8, ptr %125, i64 4
  %127 = load i32, ptr %126, align 4
  ret i32 %127

panic.cont31:                                     ; preds = %poison.cont29
  %128 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
  %129 = icmp ne i8 %128, 0
  br i1 %129, label %poison.take32, label %poison.cont33

poison.take32:                                    ; preds = %panic.cont31
  %130 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %130, align 1
  %131 = getelementptr i8, ptr %130, i64 4
  store i32 10, ptr %131, align 4
  %132 = load ptr, ptr %__panic, align 8
  %133 = getelementptr i8, ptr %132, i64 4
  %134 = load i32, ptr %133, align 4
  ret i32 %134

poison.cont33:                                    ; preds = %panic.cont31
  %135 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
  %136 = icmp ne i8 %135, 0
  br i1 %136, label %poison.take34, label %poison.cont35

poison.take34:                                    ; preds = %poison.cont33
  %137 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %137, align 1
  %138 = getelementptr i8, ptr %137, i64 4
  store i32 10, ptr %138, align 4
  %139 = load ptr, ptr %__panic, align 8
  %140 = getelementptr i8, ptr %139, i64 4
  %141 = load i32, ptr %140, align 4
  ret i32 %141

poison.cont35:                                    ; preds = %poison.cont33
  %142 = call i32 @ComptimeRuntimeErasure_x3a_x3aemittedRuntimeErasureSplicedValue(ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %143 = load ptr, ptr %__panic, align 8
  %144 = load i8, ptr %143, align 1
  %145 = icmp ne i8 %144, 0
  br i1 %145, label %panic.take36, label %panic.cont37

panic.take36:                                     ; preds = %poison.cont35
  %146 = load ptr, ptr %__panic, align 8
  %147 = getelementptr i8, ptr %146, i64 0
  %148 = load i8, ptr %147, align 1
  %149 = load ptr, ptr %__panic, align 8
  %150 = getelementptr i8, ptr %149, i64 4
  %151 = load i32, ptr %150, align 4
  %152 = load ptr, ptr %__panic, align 8
  %153 = getelementptr i8, ptr %152, i64 4
  %154 = load i32, ptr %153, align 4
  ret i32 %154

panic.cont37:                                     ; preds = %poison.cont35
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %panic.cont37
  %155 = load ptr, ptr %__panic, align 8
  %156 = load i8, ptr %155, align 1
  %157 = icmp ne i8 %156, 0
  br i1 %157, label %panic.take38, label %panic.cont39

check_fail:                                       ; preds = %panic.cont37
  %158 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %158, align 1
  %159 = getelementptr i8, ptr %158, i64 4
  store i32 4, ptr %159, align 4
  br label %check_ok

panic.take38:                                     ; preds = %check_ok
  %160 = load ptr, ptr %__panic, align 8
  %161 = getelementptr i8, ptr %160, i64 0
  %162 = load i8, ptr %161, align 1
  %163 = load ptr, ptr %__panic, align 8
  %164 = getelementptr i8, ptr %163, i64 4
  %165 = load i32, ptr %164, align 4
  %166 = load ptr, ptr %__panic, align 8
  %167 = getelementptr i8, ptr %166, i64 4
  %168 = load i32, ptr %167, align 4
  ret i32 %168

panic.cont39:                                     ; preds = %check_ok
  %169 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %115, i32 %142)
  %170 = extractvalue { i32, i1 } %169, 0
  %171 = extractvalue { i32, i1 } %169, 1
  %172 = freeze i32 %170
  br i1 %171, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont39
  %173 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
  %174 = icmp ne i8 %173, 0
  br i1 %174, label %poison.take40, label %poison.cont41

op_fail:                                          ; preds = %panic.cont39
  %175 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %175, align 1
  %176 = getelementptr i8, ptr %175, i64 4
  store i32 4, ptr %176, align 4
  ret i32 0

poison.take40:                                    ; preds = %op_ok
  %177 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %177, align 1
  %178 = getelementptr i8, ptr %177, i64 4
  store i32 10, ptr %178, align 4
  %179 = load ptr, ptr %__panic, align 8
  %180 = getelementptr i8, ptr %179, i64 4
  %181 = load i32, ptr %180, align 4
  ret i32 %181

poison.cont41:                                    ; preds = %op_ok
  store i32 59, ptr %"ComptimeRuntimeErasure_x3a_x3acompileTimeRuntimeErasureEntryPoint$tmp$call_ref_tmp_102", align 4
  %182 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
  %183 = icmp ne i8 %182, 0
  br i1 %183, label %poison.take42, label %poison.cont43

poison.take42:                                    ; preds = %poison.cont41
  %184 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %184, align 1
  %185 = getelementptr i8, ptr %184, i64 4
  store i32 10, ptr %185, align 4
  %186 = load ptr, ptr %__panic, align 8
  %187 = getelementptr i8, ptr %186, i64 4
  %188 = load i32, ptr %187, align 4
  ret i32 %188

poison.cont43:                                    ; preds = %poison.cont41
  %189 = call i32 @ComptimeRuntimeErasure_x3a_x3aemittedRuntimeErasureIdentifierSpliceValue(ptr noundef nonnull align 4 dereferenceable(4) %"ComptimeRuntimeErasure_x3a_x3acompileTimeRuntimeErasureEntryPoint$tmp$call_ref_tmp_102", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %190 = load ptr, ptr %__panic, align 8
  %191 = load i8, ptr %190, align 1
  %192 = icmp ne i8 %191, 0
  br i1 %192, label %panic.take44, label %panic.cont45

panic.take44:                                     ; preds = %poison.cont43
  %193 = load ptr, ptr %__panic, align 8
  %194 = getelementptr i8, ptr %193, i64 0
  %195 = load i8, ptr %194, align 1
  %196 = load ptr, ptr %__panic, align 8
  %197 = getelementptr i8, ptr %196, i64 4
  %198 = load i32, ptr %197, align 4
  %199 = load ptr, ptr %__panic, align 8
  %200 = getelementptr i8, ptr %199, i64 4
  %201 = load i32, ptr %200, align 4
  ret i32 %201

panic.cont45:                                     ; preds = %poison.cont43
  br i1 true, label %check_ok46, label %check_fail47

check_ok46:                                       ; preds = %check_fail47, %panic.cont45
  %202 = load ptr, ptr %__panic, align 8
  %203 = load i8, ptr %202, align 1
  %204 = icmp ne i8 %203, 0
  br i1 %204, label %panic.take48, label %panic.cont49

check_fail47:                                     ; preds = %panic.cont45
  %205 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %205, align 1
  %206 = getelementptr i8, ptr %205, i64 4
  store i32 4, ptr %206, align 4
  br label %check_ok46

panic.take48:                                     ; preds = %check_ok46
  %207 = load ptr, ptr %__panic, align 8
  %208 = getelementptr i8, ptr %207, i64 0
  %209 = load i8, ptr %208, align 1
  %210 = load ptr, ptr %__panic, align 8
  %211 = getelementptr i8, ptr %210, i64 4
  %212 = load i32, ptr %211, align 4
  %213 = load ptr, ptr %__panic, align 8
  %214 = getelementptr i8, ptr %213, i64 4
  %215 = load i32, ptr %214, align 4
  ret i32 %215

panic.cont49:                                     ; preds = %check_ok46
  %216 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %172, i32 %189)
  %217 = extractvalue { i32, i1 } %216, 0
  %218 = extractvalue { i32, i1 } %216, 1
  %219 = freeze i32 %217
  br i1 %218, label %op_fail51, label %op_ok50

op_ok50:                                          ; preds = %panic.cont49
  %220 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
  %221 = icmp ne i8 %220, 0
  br i1 %221, label %poison.take52, label %poison.cont53

op_fail51:                                        ; preds = %panic.cont49
  %222 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %222, align 1
  %223 = getelementptr i8, ptr %222, i64 4
  store i32 4, ptr %223, align 4
  ret i32 0

poison.take52:                                    ; preds = %op_ok50
  %224 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %224, align 1
  %225 = getelementptr i8, ptr %224, i64 4
  store i32 10, ptr %225, align 4
  %226 = load ptr, ptr %__panic, align 8
  %227 = getelementptr i8, ptr %226, i64 4
  %228 = load i32, ptr %227, align 4
  ret i32 %228

poison.cont53:                                    ; preds = %op_ok50
  %229 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
  %230 = icmp ne i8 %229, 0
  br i1 %230, label %poison.take54, label %poison.cont55

poison.take54:                                    ; preds = %poison.cont53
  %231 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %231, align 1
  %232 = getelementptr i8, ptr %231, i64 4
  store i32 10, ptr %232, align 4
  %233 = load ptr, ptr %__panic, align 8
  %234 = getelementptr i8, ptr %233, i64 4
  %235 = load i32, ptr %234, align 4
  ret i32 %235

poison.cont55:                                    ; preds = %poison.cont53
  %236 = call i32 @ComptimeRuntimeErasure_x3a_x3aemittedRuntimeErasureDerivedValue(ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %237 = load ptr, ptr %__panic, align 8
  %238 = load i8, ptr %237, align 1
  %239 = icmp ne i8 %238, 0
  br i1 %239, label %panic.take56, label %panic.cont57

panic.take56:                                     ; preds = %poison.cont55
  %240 = load ptr, ptr %__panic, align 8
  %241 = getelementptr i8, ptr %240, i64 0
  %242 = load i8, ptr %241, align 1
  %243 = load ptr, ptr %__panic, align 8
  %244 = getelementptr i8, ptr %243, i64 4
  %245 = load i32, ptr %244, align 4
  %246 = load ptr, ptr %__panic, align 8
  %247 = getelementptr i8, ptr %246, i64 4
  %248 = load i32, ptr %247, align 4
  ret i32 %248

panic.cont57:                                     ; preds = %poison.cont55
  br i1 true, label %check_ok58, label %check_fail59

check_ok58:                                       ; preds = %check_fail59, %panic.cont57
  %249 = load ptr, ptr %__panic, align 8
  %250 = load i8, ptr %249, align 1
  %251 = icmp ne i8 %250, 0
  br i1 %251, label %panic.take60, label %panic.cont61

check_fail59:                                     ; preds = %panic.cont57
  %252 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %252, align 1
  %253 = getelementptr i8, ptr %252, i64 4
  store i32 4, ptr %253, align 4
  br label %check_ok58

panic.take60:                                     ; preds = %check_ok58
  %254 = load ptr, ptr %__panic, align 8
  %255 = getelementptr i8, ptr %254, i64 0
  %256 = load i8, ptr %255, align 1
  %257 = load ptr, ptr %__panic, align 8
  %258 = getelementptr i8, ptr %257, i64 4
  %259 = load i32, ptr %258, align 4
  %260 = load ptr, ptr %__panic, align 8
  %261 = getelementptr i8, ptr %260, i64 4
  %262 = load i32, ptr %261, align 4
  ret i32 %262

panic.cont61:                                     ; preds = %check_ok58
  %263 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %219, i32 %236)
  %264 = extractvalue { i32, i1 } %263, 0
  %265 = extractvalue { i32, i1 } %263, 1
  %266 = freeze i32 %264
  br i1 %265, label %op_fail63, label %op_ok62

op_ok62:                                          ; preds = %panic.cont61
  ret i32 %266

op_fail63:                                        ; preds = %panic.cont61
  %267 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %267, align 1
  %268 = getelementptr i8, ptr %267, i64 4
  store i32 4, ptr %268, align 4
  ret i32 0
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aComptimeRuntimeErasure(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  %2 = load ptr, ptr %__panic, align 8
  %3 = load i8, ptr %2, align 1
  %4 = icmp ne i8 %3, 0
  br i1 %4, label %init.panic.take, label %init.panic.cont

init.panic.take:                                  ; preds = %entry
  store i8 1, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aComptimeRuntimeErasure, align 1
  %5 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 10, ptr %6, align 4
  ret void

init.panic.cont:                                  ; preds = %entry
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aComptimeRuntimeErasure(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64) #0

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #0

define void @__cx_lifecycle_init_ComptimeRuntimeErasure(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aComptimeRuntimeErasure(ptr %0)
  ret void
}

define void @__cx_lifecycle_deinit_ComptimeRuntimeErasure(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aComptimeRuntimeErasure(ptr %0)
  ret void
}

attributes #0 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
