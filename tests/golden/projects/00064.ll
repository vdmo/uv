; ==== Source/StaleValueMarkIRDiagnostics.ll
; ModuleID = 'StaleValueMarkIRDiagnostics'
source_filename = "StaleValueMarkIRDiagnostics"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStaleValueMarkIRDiagnostics = hidden global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAD2ACA7747985764 = internal constant [4 x i8] c"\01\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE = internal constant [12 x i8] c"shared_value", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAF63BD4C8601B7DF = internal constant [1 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f4AE488D8D2409103 = internal constant [14 x i8] c"observed_value", align 1
@__uv_library_attached = internal global i1 false
@__uv_image_panic_record = common hidden global { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, align 4
@llvm.global_ctors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_ctor, ptr null }]
@llvm.global_dtors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_dtor, ptr null }]

; Function Attrs: nounwind
declare ptr @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aalloc_x5fframe(i64, i64) #0

; Function Attrs: nounwind
declare void @uv_key_acquire(ptr, { i64, i64 }, i8) #0

; Function Attrs: nounwind
declare void @uv_key_check_conflict({ i64, i64 }, i8) #0

; Function Attrs: nounwind
declare void @uv_key_reacquire(ptr) #0

; Function Attrs: nounwind
declare ptr @uv_key_release_all() #0

; Function Attrs: nounwind
declare ptr @uv_key_scope_enter() #0

; Function Attrs: nounwind
declare void @uv_key_scope_exit(ptr) #0

define void @StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %0, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$closure_call_value_50" = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %producer = alloca { ptr, ptr }, align 8
  %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$closure_env_storage_3" = alloca { ptr }, align 8
  %shared_value = alloca i32, align 4
  %2 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStaleValueMarkIRDiagnostics, align 1
  %3 = icmp ne i8 %2, 0
  br i1 %3, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 10, ptr %5, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %6 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %6, align 1
  %7 = getelementptr i8, ptr %6, i64 4
  store i32 0, ptr %7, align 4
  store i32 1, ptr %shared_value, align 4
  store { ptr } zeroinitializer, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$closure_env_storage_3", align 8
  %8 = getelementptr i8, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$closure_env_storage_3", i64 0
  store ptr %shared_value, ptr %8, align 8
  %9 = insertvalue { ptr, ptr } zeroinitializer, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$closure_env_storage_3", 0
  %10 = insertvalue { ptr, ptr } %9, ptr @StaleValueMarkIRDiagnostics_x5fx3a_x5fx3astaleValueMarkIRDiagnosticsReference_x3a_x3a_x5fclosure0, 1
  store { ptr, ptr } %10, ptr %producer, align 8
  %11 = load { ptr, ptr }, ptr %producer, align 8
  %12 = extractvalue { ptr, ptr } %11, 1
  %13 = load { ptr, ptr }, ptr %producer, align 8
  %14 = extractvalue { ptr, ptr } %13, 0
  call void @StaleValueMarkIRDiagnostics_x5fx3a_x5fx3astaleValueMarkIRDiagnosticsReference_x3a_x3a_x5fclosure0(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$closure_call_value_50", ptr %14, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %15 = load ptr, ptr %__panic, align 8
  %16 = load i8, ptr %15, align 1
  %17 = icmp ne i8 %16, 0
  br i1 %17, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont
  %18 = load ptr, ptr %__panic, align 8
  %19 = getelementptr i8, ptr %18, i64 0
  %20 = load i8, ptr %19, align 1
  %21 = load ptr, ptr %__panic, align 8
  %22 = getelementptr i8, ptr %21, i64 4
  %23 = load i32, ptr %22, align 4
  ret void

panic.cont:                                       ; preds = %poison.cont
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %24 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %24, ptr %0, align 4
  ret void
}

define internal void @StaleValueMarkIRDiagnostics_x5fx3a_x5fx3astaleValueMarkIRDiagnosticsReference_x3a_x3a_x5fclosure0(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %0, ptr %__env, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %resumed_value = alloca i32, align 4
  %coerce_bits3 = alloca { ptr, i64 }, align 8
  %coerce_bits2 = alloca { ptr, i64 }, align 8
  %2 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %3 = alloca i32, align 4
  %coerce_bits1 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_key_scope_7" = alloca ptr, align 8
  %__bind_5_observed_value = alloca i32, align 4
  %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_implicit_key_scope_18" = alloca ptr, align 8
  %4 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStaleValueMarkIRDiagnostics, align 1
  %5 = icmp ne i8 %4, 0
  br i1 %5, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %6 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %6, align 1
  %7 = getelementptr i8, ptr %6, i64 4
  store i32 10, ptr %7, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  %10 = call ptr @uv_key_scope_enter()
  store ptr %10, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_key_scope_7", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits, align 8
  %11 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %11, i8 0)
  %12 = load ptr, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_key_scope_7", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits1, align 8
  %13 = load { i64, i64 }, ptr %coerce_bits1, align 1
  call void @uv_key_acquire(ptr %12, { i64, i64 } %13, i8 0)
  fence seq_cst
  %14 = getelementptr i8, ptr %__env, i64 0
  %15 = load ptr, ptr %14, align 8
  %16 = load i32, ptr %15, align 4
  fence seq_cst
  store i32 %16, ptr %__bind_5_observed_value, align 4
  %17 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aalloc_x5fframe(i64 56, i64 8)
  %18 = getelementptr i8, ptr %17, i64 0
  store i64 0, ptr %18, align 4
  %19 = getelementptr i8, ptr %17, i64 8
  store ptr @"StaleValueMarkIRDiagnostics_x5fx3a_x5fx3astaleValueMarkIRDiagnosticsReference_x3a_x3a_x5fclosure0$resume", ptr %19, align 8
  %20 = getelementptr i8, ptr %17, i64 16
  store ptr null, ptr %20, align 8
  %21 = getelementptr i8, ptr %17, i64 24
  store ptr null, ptr %21, align 8
  %22 = call ptr @uv_key_release_all()
  %23 = getelementptr i8, ptr %17, i64 24
  store ptr %22, ptr %23, align 8
  %24 = load ptr, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_key_scope_7", align 8
  %25 = getelementptr i8, ptr %17, i64 32
  store ptr %24, ptr %25, align 8
  %26 = load i32, ptr %__bind_5_observed_value, align 4
  %27 = getelementptr i8, ptr %17, i64 40
  store i32 %26, ptr %27, align 4
  %28 = load ptr, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_implicit_key_scope_18", align 8
  %29 = getelementptr i8, ptr %17, i64 48
  store ptr %28, ptr %29, align 8
  %30 = getelementptr i8, ptr %17, i64 0
  store i64 1, ptr %30, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %2, align 4
  store i8 0, ptr %2, align 1
  %31 = getelementptr i8, ptr %2, i64 8
  store i32 1, ptr %3, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %31, ptr align 1 %3, i64 4, i1 false)
  %32 = getelementptr i8, ptr %31, i64 8
  store ptr %17, ptr %32, align 8
  %33 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %2, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %33, ptr %0, align 4
  ret void

yield.cont:                                       ; No predecessors!
  fence seq_cst
  %34 = call ptr @uv_key_scope_enter()
  store ptr %34, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_implicit_key_scope_18", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f4AE488D8D2409103, i64 14 }, ptr %coerce_bits2, align 8
  %35 = load { i64, i64 }, ptr %coerce_bits2, align 1
  call void @uv_key_check_conflict({ i64, i64 } %35, i8 0)
  %36 = load ptr, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_implicit_key_scope_18", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f4AE488D8D2409103, i64 14 }, ptr %coerce_bits3, align 8
  %37 = load { i64, i64 }, ptr %coerce_bits3, align 1
  call void @uv_key_acquire(ptr %36, { i64, i64 } %37, i8 0)
  fence seq_cst
  %38 = load i32, ptr %__bind_5_observed_value, align 1
  store i32 %38, ptr %resumed_value, align 4
  %39 = load ptr, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_implicit_key_scope_18", align 8
  call void @uv_key_scope_exit(ptr %39)
  %40 = load ptr, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_key_scope_7", align 8
  call void @uv_key_scope_exit(ptr %40)
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %41 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %41, ptr %0, align 4
  ret void
}

define internal void @"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$resume"(ptr %__uv_host_env, ptr %__uv_async_out, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_frame, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_input, ptr %__panic) {
entry:
  %0 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$closure_call_value_50" = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %producer = alloca { ptr, ptr }, align 8
  %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$closure_env_storage_3" = alloca { ptr }, align 8
  %shared_value = alloca i32, align 4
  %__panic5 = alloca ptr, align 8
  %__uv_async_input4 = alloca ptr, align 8
  %__uv_async_frame3 = alloca ptr, align 8
  %__uv_async_out2 = alloca ptr, align 8
  %__uv_host_env1 = alloca ptr, align 8
  store ptr %__uv_host_env, ptr %__uv_host_env1, align 8
  store ptr %__uv_async_out, ptr %__uv_async_out2, align 8
  store ptr %__uv_async_frame, ptr %__uv_async_frame3, align 8
  store ptr %__uv_async_input, ptr %__uv_async_input4, align 8
  store ptr %__panic, ptr %__panic5, align 8
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStaleValueMarkIRDiagnostics, align 1
  %2 = icmp ne i8 %1, 0
  br i1 %2, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %3 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %3, align 1
  %4 = getelementptr i8, ptr %3, i64 4
  store i32 10, ptr %4, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %5 = load ptr, ptr %__panic5, align 8
  store i8 0, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 0, ptr %6, align 4
  %7 = load ptr, ptr %__uv_async_frame3, align 8
  %8 = load ptr, ptr %__uv_async_input4, align 8
  %9 = getelementptr i8, ptr %7, i64 0
  %10 = load i64, ptr %9, align 4
  switch i64 %10, label %async.resume.invalid [
  ]

async.resume.invalid:                             ; preds = %poison.cont
  ret void

async.resume.start:                               ; No predecessors!
  store i32 1, ptr %shared_value, align 4
  store { ptr } zeroinitializer, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$closure_env_storage_3", align 8
  %11 = getelementptr i8, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$closure_env_storage_3", i64 0
  store ptr %shared_value, ptr %11, align 8
  %12 = insertvalue { ptr, ptr } zeroinitializer, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$closure_env_storage_3", 0
  %13 = insertvalue { ptr, ptr } %12, ptr @StaleValueMarkIRDiagnostics_x5fx3a_x5fx3astaleValueMarkIRDiagnosticsReference_x3a_x3a_x5fclosure0, 1
  store { ptr, ptr } %13, ptr %producer, align 8
  %14 = load { ptr, ptr }, ptr %producer, align 8
  %15 = extractvalue { ptr, ptr } %14, 1
  %16 = load { ptr, ptr }, ptr %producer, align 8
  %17 = extractvalue { ptr, ptr } %16, 0
  call void @StaleValueMarkIRDiagnostics_x5fx3a_x5fx3astaleValueMarkIRDiagnosticsReference_x3a_x3a_x5fclosure0(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$closure_call_value_50", ptr %17, ptr noundef nonnull align 8 dereferenceable(8) %__panic5)
  %18 = load ptr, ptr %__panic5, align 8
  %19 = load i8, ptr %18, align 1
  %20 = icmp ne i8 %19, 0
  br i1 %20, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %async.resume.start
  %21 = load ptr, ptr %__panic5, align 8
  %22 = getelementptr i8, ptr %21, i64 0
  %23 = load i8, ptr %22, align 1
  %24 = load ptr, ptr %__panic5, align 8
  %25 = getelementptr i8, ptr %24, i64 4
  %26 = load i32, ptr %25, align 4
  ret void

panic.cont:                                       ; preds = %async.resume.start
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %27 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %0, align 4
  %28 = load ptr, ptr %__uv_async_out2, align 8
  %29 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %27, ptr %29, align 4
  ret void
}

define internal void @"StaleValueMarkIRDiagnostics_x5fx3a_x5fx3astaleValueMarkIRDiagnosticsReference_x3a_x3a_x5fclosure0$resume"(ptr %__uv_host_env, ptr %__uv_async_out, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_frame, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_input, ptr %__panic) {
entry:
  %0 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %resumed_value = alloca i32, align 4
  %coerce_bits8 = alloca { ptr, i64 }, align 8
  %coerce_bits7 = alloca { ptr, i64 }, align 8
  %yield_input = alloca {}, align 8
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %2 = alloca i32, align 4
  %coerce_bits6 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_key_scope_7" = alloca ptr, align 8
  %__bind_5_observed_value = alloca i32, align 4
  %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_implicit_key_scope_18" = alloca ptr, align 8
  %__panic5 = alloca ptr, align 8
  %__uv_async_input4 = alloca ptr, align 8
  %__uv_async_frame3 = alloca ptr, align 8
  %__uv_async_out2 = alloca ptr, align 8
  %__uv_host_env1 = alloca ptr, align 8
  store ptr %__uv_host_env, ptr %__uv_host_env1, align 8
  store ptr %__uv_async_out, ptr %__uv_async_out2, align 8
  store ptr %__uv_async_frame, ptr %__uv_async_frame3, align 8
  store ptr %__uv_async_input, ptr %__uv_async_input4, align 8
  store ptr %__panic, ptr %__panic5, align 8
  %3 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStaleValueMarkIRDiagnostics, align 1
  %4 = icmp ne i8 %3, 0
  br i1 %4, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %5 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 10, ptr %6, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %7 = load ptr, ptr %__panic5, align 8
  store i8 0, ptr %7, align 1
  %8 = getelementptr i8, ptr %7, i64 4
  store i32 0, ptr %8, align 4
  %9 = load ptr, ptr %__uv_async_frame3, align 8
  %10 = load ptr, ptr %__uv_async_input4, align 8
  %11 = getelementptr i8, ptr %9, i64 32
  %12 = load ptr, ptr %11, align 8
  store ptr %12, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_key_scope_7", align 8
  %13 = getelementptr i8, ptr %9, i64 40
  %14 = load i32, ptr %13, align 4
  store i32 %14, ptr %__bind_5_observed_value, align 4
  %15 = getelementptr i8, ptr %9, i64 48
  %16 = load ptr, ptr %15, align 8
  store ptr %16, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_implicit_key_scope_18", align 8
  %17 = getelementptr i8, ptr %9, i64 0
  %18 = load i64, ptr %17, align 4
  switch i64 %18, label %async.resume.invalid [
    i64 1, label %yield.resume.1
  ]

async.resume.invalid:                             ; preds = %poison.cont
  ret void

async.resume.start:                               ; No predecessors!
  %19 = call ptr @uv_key_scope_enter()
  store ptr %19, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_key_scope_7", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits, align 8
  %20 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %20, i8 0)
  %21 = load ptr, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_key_scope_7", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits6, align 8
  %22 = load { i64, i64 }, ptr %coerce_bits6, align 1
  call void @uv_key_acquire(ptr %21, { i64, i64 } %22, i8 0)
  fence seq_cst
  %23 = load ptr, ptr null, align 8
  %24 = load i32, ptr %23, align 4
  fence seq_cst
  store i32 %24, ptr %__bind_5_observed_value, align 4
  %25 = call ptr @uv_key_release_all()
  %26 = getelementptr i8, ptr %9, i64 24
  store ptr %25, ptr %26, align 8
  %27 = load ptr, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_key_scope_7", align 8
  %28 = getelementptr i8, ptr %9, i64 32
  store ptr %27, ptr %28, align 8
  %29 = load i32, ptr %__bind_5_observed_value, align 4
  %30 = getelementptr i8, ptr %9, i64 40
  store i32 %29, ptr %30, align 4
  %31 = load ptr, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_implicit_key_scope_18", align 8
  %32 = getelementptr i8, ptr %9, i64 48
  store ptr %31, ptr %32, align 8
  %33 = getelementptr i8, ptr %9, i64 0
  store i64 1, ptr %33, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 0, ptr %1, align 1
  %34 = getelementptr i8, ptr %1, i64 8
  store i32 1, ptr %2, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %34, ptr align 1 %2, i64 4, i1 false)
  %35 = getelementptr i8, ptr %34, i64 8
  store ptr %9, ptr %35, align 8
  %36 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  %37 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %36, ptr %37, align 4
  ret void

yield.cont:                                       ; preds = %yield.resume.1
  %38 = getelementptr i8, ptr %9, i64 24
  %39 = load ptr, ptr %38, align 8
  call void @uv_key_reacquire(ptr %39)
  %40 = getelementptr i8, ptr %9, i64 24
  store ptr null, ptr %40, align 8
  store {} zeroinitializer, ptr %yield_input, align 1
  fence seq_cst
  %41 = call ptr @uv_key_scope_enter()
  store ptr %41, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_implicit_key_scope_18", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f4AE488D8D2409103, i64 14 }, ptr %coerce_bits7, align 8
  %42 = load { i64, i64 }, ptr %coerce_bits7, align 1
  call void @uv_key_check_conflict({ i64, i64 } %42, i8 0)
  %43 = load ptr, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_implicit_key_scope_18", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f4AE488D8D2409103, i64 14 }, ptr %coerce_bits8, align 8
  %44 = load { i64, i64 }, ptr %coerce_bits8, align 1
  call void @uv_key_acquire(ptr %43, { i64, i64 } %44, i8 0)
  fence seq_cst
  %45 = load i32, ptr %__bind_5_observed_value, align 1
  store i32 %45, ptr %resumed_value, align 4
  %46 = load ptr, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_implicit_key_scope_18", align 8
  call void @uv_key_scope_exit(ptr %46)
  %47 = load ptr, ptr %"StaleValueMarkIRDiagnostics_x3a_x3astaleValueMarkIRDiagnosticsReference$tmp$__uv_key_scope_7", align 8
  call void @uv_key_scope_exit(ptr %47)
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %48 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %0, align 4
  %49 = load ptr, ptr %__uv_async_out2, align 8
  %50 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %48, ptr %50, align 4
  ret void

yield.resume.1:                                   ; preds = %poison.cont
  br label %yield.cont
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aStaleValueMarkIRDiagnostics(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aStaleValueMarkIRDiagnostics(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #1

define hidden void @__cx_lifecycle_init_StaleValueMarkIRDiagnostics(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aStaleValueMarkIRDiagnostics(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_StaleValueMarkIRDiagnostics(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aStaleValueMarkIRDiagnostics(ptr %0)
  ret void
}

define hidden i32 @__ultraviolet_library_entry(ptr %0, i32 %fdwReason, ptr %1) {
entry:
  switch i32 %fdwReason, label %dll.other [
    i32 1, label %dll.attach
    i32 0, label %dll.detach
  ]

dll.attach:                                       ; preds = %entry
  %2 = load i1, ptr @__uv_library_attached, align 1
  br i1 %2, label %dll.attach.done, label %dll.attach.work

dll.detach:                                       ; preds = %entry
  %3 = load i1, ptr @__uv_library_attached, align 1
  br i1 %3, label %dll.detach.work, label %dll.detach.done

dll.other:                                        ; preds = %entry
  ret i32 1

dll.attach.work:                                  ; preds = %dll.attach
  store i8 0, ptr @__uv_image_panic_record, align 1
  store i32 0, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  %dll_attach_panic_out = alloca ptr, align 8
  store ptr @__uv_image_panic_record, ptr %dll_attach_panic_out, align 8
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aStaleValueMarkIRDiagnostics(ptr %dll_attach_panic_out)
  %4 = load i8, ptr @__uv_image_panic_record, align 1
  %5 = icmp ne i8 %4, 0
  br i1 %5, label %dll.attach.fail, label %dll.attach.cont

dll.attach.done:                                  ; preds = %dll.attach
  ret i32 1

dll.attach.cont:                                  ; preds = %dll.attach.work
  store i1 true, ptr @__uv_library_attached, align 1
  ret i32 1

dll.attach.fail:                                  ; preds = %dll.attach.work
  store i8 0, ptr @__uv_image_panic_record, align 1
  store i32 0, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  store i1 false, ptr @__uv_library_attached, align 1
  ret i32 0

dll.detach.work:                                  ; preds = %dll.detach
  store i8 0, ptr @__uv_image_panic_record, align 1
  store i32 0, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  %dll_detach_panic_out = alloca ptr, align 8
  store ptr @__uv_image_panic_record, ptr %dll_detach_panic_out, align 8
  %dll_detach_panic_seen = alloca i1, align 1
  %dll_detach_panic_code = alloca i32, align 4
  store i1 false, ptr %dll_detach_panic_seen, align 1
  store i32 0, ptr %dll_detach_panic_code, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aStaleValueMarkIRDiagnostics(ptr %dll_detach_panic_out)
  %6 = load i8, ptr @__uv_image_panic_record, align 1
  %7 = icmp ne i8 %6, 0
  br i1 %7, label %dll.detach.panic.capture, label %dll.detach.cont

dll.detach.done:                                  ; preds = %dll.detach
  ret i32 1

dll.detach.panic.capture:                         ; preds = %dll.detach.work
  %8 = load i1, ptr %dll_detach_panic_seen, align 1
  %9 = load i32, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  br i1 %8, label %dll.detach.panic.clear, label %dll.detach.panic.store

dll.detach.cont:                                  ; preds = %dll.detach.panic.clear, %dll.detach.work
  store i1 false, ptr @__uv_library_attached, align 1
  %10 = load i1, ptr %dll_detach_panic_seen, align 1
  br i1 %10, label %dll.detach.fail, label %dll.detach.success

dll.detach.panic.store:                           ; preds = %dll.detach.panic.capture
  store i1 true, ptr %dll_detach_panic_seen, align 1
  store i32 %9, ptr %dll_detach_panic_code, align 4
  br label %dll.detach.panic.clear

dll.detach.panic.clear:                           ; preds = %dll.detach.panic.store, %dll.detach.panic.capture
  store i8 0, ptr @__uv_image_panic_record, align 1
  store i32 0, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  br label %dll.detach.cont

dll.detach.fail:                                  ; preds = %dll.detach.cont
  %11 = load i32, ptr %dll_detach_panic_code, align 4
  store i8 1, ptr @__uv_image_panic_record, align 1
  store i32 %11, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  ret i32 0

dll.detach.success:                               ; preds = %dll.detach.cont
  ret i32 1
}

define internal void @__uv_library_ctor() {
entry:
  %library_ctor_panic_code = alloca i32, align 4
  %0 = call i32 @__ultraviolet_library_entry(ptr null, i32 1, ptr null)
  %1 = icmp ne i32 %0, 0
  br i1 %1, label %ctor.ok, label %ctor.fail

ctor.ok:                                          ; preds = %entry
  ret void

ctor.fail:                                        ; preds = %entry
  store i32 14, ptr %library_ctor_panic_code, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %library_ctor_panic_code)
  unreachable
}

declare void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr)

define internal void @__uv_library_dtor() {
entry:
  %library_dtor_panic_code = alloca i32, align 4
  %0 = call i32 @__ultraviolet_library_entry(ptr null, i32 0, ptr null)
  %1 = icmp ne i32 %0, 0
  br i1 %1, label %dtor.ok, label %dtor.fail

dtor.ok:                                          ; preds = %entry
  ret void

dtor.fail:                                        ; preds = %entry
  store i32 15, ptr %library_dtor_panic_code, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %library_dtor_panic_code)
  unreachable
}

attributes #0 = { nounwind }
attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
