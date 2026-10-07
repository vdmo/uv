; ==== Source/AsyncFailedSpawnedWaitPanic.ll
; ModuleID = 'AsyncFailedSpawnedWaitPanic'
source_filename = "AsyncFailedSpawnedWaitPanic"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aAsyncFailedSpawnedWaitPanic = global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fA8C7F832281A39C5 = internal constant [8 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2CDCDC0DFC5D1141 = internal constant [8 x i8] c"\04\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fA09E307A7F948ACD = internal constant [8 x i8] c"\08\00\00\00\00\00\00\00", align 1
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

; Function Attrs: nounwind
declare ptr @uv_spawn_create(ptr, i64, ptr, ptr, i64, i64, i32) #0

; Function Attrs: nounwind
declare ptr @uv_spawn_wait(ptr) #0

define i32 @AsyncFailedSpawnedWaitPanic_x3a_x3amain(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %byref_arg = alloca i32, align 4
  %handle = alloca ptr, align 8
  %"AsyncFailedSpawnedWaitPanic_x3a_x3amain$tmp$spawn_env_storage_5" = alloca {}, align 1
  %coerce_bits = alloca { ptr, ptr }, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aAsyncFailedSpawnedWaitPanic, align 1
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
  %9 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3acpu(ptr noundef nonnull align 8 dereferenceable(96) %context)
  store { i64, i64 } %9, ptr %abi_return, align 8
  %10 = load { ptr, ptr }, ptr %abi_return, align 1
  store { ptr, ptr } %10, ptr %coerce_bits, align 8
  %11 = load { i64, i64 }, ptr %coerce_bits, align 1
  %12 = call ptr @uv_parallel_begin({ i64, i64 } %11, i64 -1, ptr null)
  store {} zeroinitializer, ptr %"AsyncFailedSpawnedWaitPanic_x3a_x3amain$tmp$spawn_env_storage_5", align 1
  %13 = call ptr @uv_spawn_create(ptr %"AsyncFailedSpawnedWaitPanic_x3a_x3amain$tmp$spawn_env_storage_5", i64 0, ptr @__cx_spawn_body_AsyncFailedSpawnedWaitPanic_0, ptr null, i64 4, i64 0, i32 -1)
  store ptr %13, ptr %handle, align 8
  %14 = load ptr, ptr %handle, align 8
  %15 = call ptr @uv_spawn_wait(ptr %14)
  %16 = load i32, ptr %15, align 4
  %17 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %17, align 1
  %18 = getelementptr i8, ptr %17, i64 4
  store i32 0, ptr %18, align 4
  %19 = call i32 @uv_parallel_join(ptr %12)
  %20 = icmp eq i32 %19, 0
  %21 = xor i1 %20, true
  %22 = zext i1 %21 to i8
  %23 = icmp ne i8 %22, 0
  %24 = icmp ne i8 %22, 0
  br i1 %24, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  %25 = load ptr, ptr %__panic, align 8
  %26 = getelementptr i8, ptr %25, i64 0
  store i8 1, ptr %26, align 1
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 4
  store i32 %19, ptr %28, align 4
  br label %if.merge

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %if.result = phi i64 [ 0, %if.then ], [ 0, %if.else ]
  %29 = load ptr, ptr %__panic, align 8
  %30 = getelementptr i8, ptr %29, i64 0
  %31 = load i8, ptr %30, align 1
  %32 = load ptr, ptr %__panic, align 8
  %33 = getelementptr i8, ptr %32, i64 4
  %34 = load i32, ptr %33, align 4
  %35 = icmp ne i8 %31, 0
  %36 = zext i1 %35 to i8
  %37 = icmp ne i8 %36, 0
  %38 = and i1 false, %37
  br i1 %38, label %if.then1, label %if.else2

if.then1:                                         ; preds = %if.merge
  store i32 %34, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else2:                                         ; preds = %if.merge
  br label %if.merge3

if.merge3:                                        ; preds = %if.else2
  %39 = icmp ne i8 %31, 0
  %40 = xor i1 %39, true
  %41 = and i1 false, %40
  br i1 %41, label %if.then4, label %if.else5

if.then4:                                         ; preds = %if.merge3
  %42 = load ptr, ptr %__panic, align 8
  %43 = getelementptr i8, ptr %42, i64 0
  store i8 1, ptr %43, align 1
  %44 = load ptr, ptr %__panic, align 8
  %45 = getelementptr i8, ptr %44, i64 4
  store i32 0, ptr %45, align 4
  br label %if.merge6

if.else5:                                         ; preds = %if.merge3
  br label %if.merge6

if.merge6:                                        ; preds = %if.else5, %if.then4
  %if.result7 = phi i64 [ 0, %if.then4 ], [ 0, %if.else5 ]
  %46 = icmp ne i8 %31, 0
  %47 = zext i1 %46 to i8
  %48 = icmp ne i8 %47, 0
  %49 = or i1 false, %48
  %50 = icmp ne i8 %31, 0
  %51 = icmp ne i8 %31, 0
  br i1 %51, label %if.then8, label %if.else9

if.then8:                                         ; preds = %if.merge6
  br label %if.merge10

if.else9:                                         ; preds = %if.merge6
  br label %if.merge10

if.merge10:                                       ; preds = %if.else9, %if.then8
  %if.result11 = phi i32 [ %34, %if.then8 ], [ 0, %if.else9 ]
  %52 = load ptr, ptr %__panic, align 8
  %53 = load i8, ptr %52, align 1
  %54 = icmp ne i8 %53, 0
  br i1 %54, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %if.merge10
  %55 = load ptr, ptr %__panic, align 8
  %56 = getelementptr i8, ptr %55, i64 4
  %57 = load i32, ptr %56, align 4
  ret i32 %57

panic.cont:                                       ; preds = %if.merge10
  ret i32 %16
}

define internal void @__cx_dispatch_body_AsyncFailedSpawnedWaitPanic_1(ptr %__uv_host_env, ptr noundef nonnull align 8 dereferenceable(8) %elem, ptr noundef nonnull align 1 %env, ptr noundef nonnull align 4 dereferenceable(4) %result, ptr %__panic) {
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
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aAsyncFailedSpawnedWaitPanic, align 1
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

define internal void @__cx_spawn_body_AsyncFailedSpawnedWaitPanic_0(ptr %__uv_host_env, ptr noundef nonnull align 1 %env, ptr noundef nonnull align 4 dereferenceable(4) %result, ptr %__panic) {
entry:
  %abi_return10 = alloca { i64, i64 }, align 8
  %value = alloca i32, align 4
  %byref_arg9 = alloca { i64, i64, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %byref_arg8 = alloca { i8, [7 x i8], i64, i64 }, align 8
  %0 = alloca { i64, i64, i64 }, align 8
  %"AsyncFailedSpawnedWaitPanic_x3a_x3amain$tmp$dispatch_result_var_16" = alloca i32, align 4
  %byref_arg7 = alloca i64, align 8
  %byref_arg6 = alloca i64, align 8
  %byref_arg = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %"region$05" = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %aggregate.literal = alloca { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] }, align 8
  %"region$0" = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %__panic4 = alloca ptr, align 8
  %result3 = alloca ptr, align 8
  %env2 = alloca ptr, align 8
  %__uv_host_env1 = alloca ptr, align 8
  store ptr %__uv_host_env, ptr %__uv_host_env1, align 8
  store ptr %env, ptr %env2, align 8
  store ptr %result, ptr %result3, align 8
  store ptr %__panic, ptr %__panic4, align 8
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aAsyncFailedSpawnedWaitPanic, align 1
  %2 = icmp ne i8 %1, 0
  br i1 %2, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %3 = load ptr, ptr %__panic4, align 8
  store i8 1, ptr %3, align 1
  %4 = getelementptr i8, ptr %3, i64 4
  store i32 10, ptr %4, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %5 = load ptr, ptr %__panic4, align 8
  store i8 0, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 0, ptr %6, align 4
  call void @llvm.memset.p0.i64(ptr align 8 %aggregate.literal, i8 0, i64 40, i1 false)
  store i64 0, ptr %aggregate.literal, align 8
  %7 = getelementptr i8, ptr %aggregate.literal, i64 8
  store { i8, [7 x i8], [24 x i8], [0 x i64] } zeroinitializer, ptr %7, align 8
  %8 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3anew_x5fscoped(ptr noundef nonnull align 8 dereferenceable(40) %aggregate.literal)
  store { i64, i64 } %8, ptr %abi_return, align 8
  %9 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return, align 1
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %9, ptr %"region$05", align 4
  %10 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %"region$05", align 4
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %10, ptr %byref_arg, align 4
  store i64 0, ptr %byref_arg6, align 4
  store i64 1, ptr %byref_arg7, align 4
  %11 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aalloc(ptr noundef nonnull align 8 dereferenceable(16) %byref_arg, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg6, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg7)
  store {} zeroinitializer, ptr %11, align 1
  store i32 0, ptr %"AsyncFailedSpawnedWaitPanic_x3a_x3amain$tmp$dispatch_result_var_16", align 4
  store { i64, i64, i64 } zeroinitializer, ptr %0, align 4
  %12 = getelementptr i8, ptr %0, i64 0
  store i64 64, ptr %12, align 1
  %13 = getelementptr i8, ptr %0, i64 8
  store i64 1, ptr %13, align 1
  %14 = getelementptr i8, ptr %0, i64 16
  store i64 1, ptr %14, align 1
  %15 = load { i64, i64, i64 }, ptr %0, align 4
  store { i8, [7 x i8], i64, i64 } { i8 4, [7 x i8] zeroinitializer, i64 0, i64 0 }, ptr %byref_arg8, align 4
  store { ptr, i64 } { ptr @0, i64 1 }, ptr %coerce_bits, align 8
  %16 = load { i64, i64 }, ptr %coerce_bits, align 1
  store { i64, i64, i64 } %15, ptr %byref_arg9, align 4
  call void @uv_dispatch_run(ptr noundef nonnull align 8 dereferenceable(24) %byref_arg8, i64 8, i64 4, ptr @__cx_dispatch_body_AsyncFailedSpawnedWaitPanic_1, ptr null, ptr %11, { i64, i64 } %16, ptr %"AsyncFailedSpawnedWaitPanic_x3a_x3amain$tmp$dispatch_result_var_16", ptr null, i32 0, i64 0, ptr noundef nonnull byval({ i64, i64, i64 }) align 8 dereferenceable(24) %byref_arg9)
  %17 = load i32, ptr %"AsyncFailedSpawnedWaitPanic_x3a_x3amain$tmp$dispatch_result_var_16", align 4
  store i32 %17, ptr %value, align 4
  %18 = load i32, ptr %value, align 4
  %19 = load ptr, ptr %result3, align 8
  %20 = load i32, ptr %value, align 4
  store i32 %20, ptr %19, align 4
  %21 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$05")
  store { i64, i64 } %21, ptr %abi_return10, align 8
  %22 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return10, align 1
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aAsyncFailedSpawnedWaitPanic(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aAsyncFailedSpawnedWaitPanic(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #2

declare ptr @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aalloc(ptr, ptr, ptr)

define void @__cx_lifecycle_init_AsyncFailedSpawnedWaitPanic(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aAsyncFailedSpawnedWaitPanic(ptr %0)
  ret void
}

define void @__cx_lifecycle_deinit_AsyncFailedSpawnedWaitPanic(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aAsyncFailedSpawnedWaitPanic(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aAsyncFailedSpawnedWaitPanic(ptr %entry_panic_out)
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
  %24 = call i32 @AsyncFailedSpawnedWaitPanic_x3a_x3amain(ptr %entry_ctx, ptr %entry_panic_out)
  %entry_deinit_panic_seen = alloca i8, align 1
  %entry_deinit_panic_code = alloca i32, align 4
  store i8 0, ptr %entry_deinit_panic_seen, align 1
  store i32 0, ptr %entry_deinit_panic_code, align 4
  %25 = load i8, ptr %entry_panic, align 1
  %26 = icmp ne i8 %25, 0
  br i1 %26, label %entry.panic, label %entry.deinit

entry.deinit:                                     ; preds = %entry.init.cont
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aAsyncFailedSpawnedWaitPanic(ptr %entry_panic_out)
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
