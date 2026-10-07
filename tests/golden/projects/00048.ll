; ==== Source/ReducedEmptyDispatchPanic.ll
; ModuleID = 'ReducedEmptyDispatchPanic'
source_filename = "ReducedEmptyDispatchPanic"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aReducedEmptyDispatchPanic = global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fA09E307A7F948ACD = internal constant [8 x i8] c"\08\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2CDCDC0DFC5D1141 = internal constant [8 x i8] c"\04\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@0 = private unnamed_addr constant [2 x i8] c"+\00", align 1

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3acpu(ptr noundef nonnull align 8 dereferenceable(96)) #0

; Function Attrs: noreturn nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4)) #1

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16)) #0

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3anew_x5fscoped(ptr noundef nonnull align 8 dereferenceable(40)) #0

; Function Attrs: nounwind
declare void @uv_dispatch_run(ptr noundef nonnull align 8 dereferenceable(24), i64, i64, ptr, ptr, ptr, { i64, i64 }, ptr, ptr, i32, i64, ptr noundef nonnull byval({ i64, i64, i64 }) align 8 dereferenceable(24)) #0

; Function Attrs: nounwind
declare ptr @uv_parallel_begin({ i64, i64 }, i64, ptr) #0

; Function Attrs: nounwind
declare i32 @uv_parallel_join(ptr) #0

define i32 @ReducedEmptyDispatchPanic_x3a_x3amain(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %value = alloca i32, align 4
  %byref_arg6 = alloca i32, align 4
  %byref_arg2 = alloca { i64, i64, i64 }, align 8
  %coerce_bits1 = alloca { ptr, i64 }, align 8
  %byref_arg = alloca { i8, [7 x i8], i64, i64 }, align 8
  %0 = alloca { i64, i64, i64 }, align 8
  %"ReducedEmptyDispatchPanic_x3a_x3amain$tmp$dispatch_result_var_10" = alloca i32, align 4
  %"ReducedEmptyDispatchPanic_x3a_x3amain$tmp$dispatch_env_storage_8" = alloca {}, align 1
  %coerce_bits = alloca { ptr, ptr }, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aReducedEmptyDispatchPanic, align 1
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
  %10 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3acpu(ptr noundef nonnull align 8 dereferenceable(96) %context)
  store { i64, i64 } %10, ptr %abi_return, align 8
  %11 = load { ptr, ptr }, ptr %abi_return, align 1
  store { ptr, ptr } %11, ptr %coerce_bits, align 8
  %12 = load { i64, i64 }, ptr %coerce_bits, align 1
  %13 = call ptr @uv_parallel_begin({ i64, i64 } %12, i64 -1, ptr null)
  store {} zeroinitializer, ptr %"ReducedEmptyDispatchPanic_x3a_x3amain$tmp$dispatch_env_storage_8", align 1
  store i32 0, ptr %"ReducedEmptyDispatchPanic_x3a_x3amain$tmp$dispatch_result_var_10", align 4
  store { i64, i64, i64 } zeroinitializer, ptr %0, align 4
  %14 = getelementptr i8, ptr %0, i64 0
  store i64 64, ptr %14, align 1
  %15 = getelementptr i8, ptr %0, i64 8
  store i64 1, ptr %15, align 1
  %16 = getelementptr i8, ptr %0, i64 16
  store i64 1, ptr %16, align 1
  %17 = load { i64, i64, i64 }, ptr %0, align 4
  store { i8, [7 x i8], i64, i64 } { i8 4, [7 x i8] zeroinitializer, i64 0, i64 0 }, ptr %byref_arg, align 4
  store { ptr, i64 } { ptr @0, i64 1 }, ptr %coerce_bits1, align 8
  %18 = load { i64, i64 }, ptr %coerce_bits1, align 1
  store { i64, i64, i64 } %17, ptr %byref_arg2, align 4
  call void @uv_dispatch_run(ptr noundef nonnull align 8 dereferenceable(24) %byref_arg, i64 8, i64 4, ptr @__cx_dispatch_body_ReducedEmptyDispatchPanic_0, ptr null, ptr %"ReducedEmptyDispatchPanic_x3a_x3amain$tmp$dispatch_env_storage_8", { i64, i64 } %18, ptr %"ReducedEmptyDispatchPanic_x3a_x3amain$tmp$dispatch_result_var_10", ptr null, i32 0, i64 0, ptr noundef nonnull byval({ i64, i64, i64 }) align 8 dereferenceable(24) %byref_arg2)
  %19 = load i32, ptr %"ReducedEmptyDispatchPanic_x3a_x3amain$tmp$dispatch_result_var_10", align 4
  %20 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %20, align 1
  %21 = getelementptr i8, ptr %20, i64 4
  store i32 0, ptr %21, align 4
  %22 = call i32 @uv_parallel_join(ptr %13)
  %23 = icmp eq i32 %22, 0
  %24 = xor i1 %23, true
  %25 = zext i1 %24 to i8
  %26 = icmp ne i8 %25, 0
  %27 = icmp ne i8 %25, 0
  br i1 %27, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  %28 = load ptr, ptr %__panic, align 8
  %29 = getelementptr i8, ptr %28, i64 0
  store i8 1, ptr %29, align 1
  %30 = load ptr, ptr %__panic, align 8
  %31 = getelementptr i8, ptr %30, i64 4
  store i32 %22, ptr %31, align 4
  br label %if.merge

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %if.result = phi i64 [ 0, %if.then ], [ 0, %if.else ]
  %32 = load ptr, ptr %__panic, align 8
  %33 = getelementptr i8, ptr %32, i64 0
  %34 = load i8, ptr %33, align 1
  %35 = load ptr, ptr %__panic, align 8
  %36 = getelementptr i8, ptr %35, i64 4
  %37 = load i32, ptr %36, align 4
  %38 = icmp ne i8 %34, 0
  %39 = zext i1 %38 to i8
  %40 = icmp ne i8 %39, 0
  %41 = and i1 false, %40
  br i1 %41, label %if.then3, label %if.else4

if.then3:                                         ; preds = %if.merge
  store i32 %37, ptr %byref_arg6, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg6)
  unreachable

if.else4:                                         ; preds = %if.merge
  br label %if.merge5

if.merge5:                                        ; preds = %if.else4
  %42 = icmp ne i8 %34, 0
  %43 = xor i1 %42, true
  %44 = and i1 false, %43
  br i1 %44, label %if.then7, label %if.else8

if.then7:                                         ; preds = %if.merge5
  %45 = load ptr, ptr %__panic, align 8
  %46 = getelementptr i8, ptr %45, i64 0
  store i8 1, ptr %46, align 1
  %47 = load ptr, ptr %__panic, align 8
  %48 = getelementptr i8, ptr %47, i64 4
  store i32 0, ptr %48, align 4
  br label %if.merge9

if.else8:                                         ; preds = %if.merge5
  br label %if.merge9

if.merge9:                                        ; preds = %if.else8, %if.then7
  %if.result10 = phi i64 [ 0, %if.then7 ], [ 0, %if.else8 ]
  %49 = icmp ne i8 %34, 0
  %50 = zext i1 %49 to i8
  %51 = icmp ne i8 %50, 0
  %52 = or i1 false, %51
  %53 = icmp ne i8 %34, 0
  %54 = icmp ne i8 %34, 0
  br i1 %54, label %if.then11, label %if.else12

if.then11:                                        ; preds = %if.merge9
  br label %if.merge13

if.else12:                                        ; preds = %if.merge9
  br label %if.merge13

if.merge13:                                       ; preds = %if.else12, %if.then11
  %if.result14 = phi i32 [ %37, %if.then11 ], [ 0, %if.else12 ]
  %55 = load ptr, ptr %__panic, align 8
  %56 = load i8, ptr %55, align 1
  %57 = icmp ne i8 %56, 0
  br i1 %57, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %if.merge13
  %58 = load ptr, ptr %__panic, align 8
  %59 = getelementptr i8, ptr %58, i64 4
  %60 = load i32, ptr %59, align 4
  ret i32 %60

panic.cont:                                       ; preds = %if.merge13
  store i32 %19, ptr %value, align 4
  %61 = load i32, ptr %value, align 4
  ret i32 %61
}

define internal void @__cx_dispatch_body_ReducedEmptyDispatchPanic_0(ptr %__uv_host_env, ptr noundef nonnull align 8 dereferenceable(8) %elem, ptr noundef nonnull align 1 %env, ptr noundef nonnull align 4 dereferenceable(4) %result, ptr %__panic) {
entry:
  %abi_return8 = alloca { i64, i64 }, align 8
  %abi_return7 = alloca { i64, i64 }, align 8
  %index = alloca i64, align 8
  %"region$06" = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %aggregate.literal = alloca { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] }, align 8
  %"region$0" = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %__panic5 = alloca ptr, align 8
  %result4 = alloca ptr, align 8
  %env3 = alloca ptr, align 8
  %elem2 = alloca ptr, align 8
  %__uv_host_env1 = alloca ptr, align 8
  store ptr %__uv_host_env, ptr %__uv_host_env1, align 8
  store ptr %elem, ptr %elem2, align 8
  store ptr %env, ptr %env3, align 8
  store ptr %result, ptr %result4, align 8
  store ptr %__panic, ptr %__panic5, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aReducedEmptyDispatchPanic, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic5, align 8
  store i8 0, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 0, ptr %5, align 4
  call void @llvm.memset.p0.i64(ptr align 8 %aggregate.literal, i8 0, i64 40, i1 false)
  store i64 0, ptr %aggregate.literal, align 8
  %6 = getelementptr i8, ptr %aggregate.literal, i64 8
  store { i8, [7 x i8], [24 x i8], [0 x i64] } zeroinitializer, ptr %6, align 8
  %7 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3anew_x5fscoped(ptr noundef nonnull align 8 dereferenceable(40) %aggregate.literal)
  store { i64, i64 } %7, ptr %abi_return, align 8
  %8 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return, align 1
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %8, ptr %"region$06", align 4
  %9 = load ptr, ptr %elem2, align 8
  %10 = load i64, ptr %9, align 4
  store i64 %10, ptr %index, align 4
  %11 = load i64, ptr %index, align 4
  %12 = trunc i64 %11 to i32
  %13 = zext i32 %12 to i64
  %14 = icmp eq i64 %11, %13
  br i1 %14, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %15 = load ptr, ptr %__panic5, align 8
  %16 = load i8, ptr %15, align 1
  %17 = icmp ne i8 %16, 0
  br i1 %17, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %18 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %18, align 1
  %19 = getelementptr i8, ptr %18, i64 4
  store i32 7, ptr %19, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %20 = load ptr, ptr %__panic5, align 8
  %21 = getelementptr i8, ptr %20, i64 0
  %22 = load i8, ptr %21, align 1
  %23 = load ptr, ptr %__panic5, align 8
  %24 = getelementptr i8, ptr %23, i64 4
  %25 = load i32, ptr %24, align 4
  %26 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$06")
  store { i64, i64 } %26, ptr %abi_return7, align 8
  %27 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return7, align 1
  ret void

panic.cont:                                       ; preds = %check_ok
  %28 = load i64, ptr %index, align 4
  %29 = trunc i64 %28 to i32
  %30 = load ptr, ptr %result4, align 8
  store i32 %29, ptr %30, align 4
  %31 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$06")
  store { i64, i64 } %31, ptr %abi_return8, align 8
  %32 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return8, align 1
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aReducedEmptyDispatchPanic(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aReducedEmptyDispatchPanic(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #2

define void @__cx_lifecycle_init_ReducedEmptyDispatchPanic(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aReducedEmptyDispatchPanic(ptr %0)
  ret void
}

define void @__cx_lifecycle_deinit_ReducedEmptyDispatchPanic(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aReducedEmptyDispatchPanic(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aReducedEmptyDispatchPanic(ptr %entry_panic_out)
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
  %24 = call i32 @ReducedEmptyDispatchPanic_x3a_x3amain(ptr %entry_ctx, ptr %entry_panic_out)
  %entry_deinit_panic_seen = alloca i8, align 1
  %entry_deinit_panic_code = alloca i32, align 4
  store i8 0, ptr %entry_deinit_panic_seen, align 1
  store i32 0, ptr %entry_deinit_panic_code, align 4
  %25 = load i8, ptr %entry_panic, align 1
  %26 = icmp ne i8 %25, 0
  br i1 %26, label %entry.panic, label %entry.deinit

entry.deinit:                                     ; preds = %entry.init.cont
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aReducedEmptyDispatchPanic(ptr %entry_panic_out)
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

declare void @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x5finit(ptr)

attributes #0 = { nounwind }
attributes #1 = { noreturn nounwind }
attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: write) }
