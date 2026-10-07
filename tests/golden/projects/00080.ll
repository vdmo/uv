; ==== Source/StaleAfterYieldReleaseWarning.ll
; ModuleID = 'StaleAfterYieldReleaseWarning'
source_filename = "StaleAfterYieldReleaseWarning"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStaleAfterYieldReleaseWarning = hidden global i8 0
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

define void @StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %0, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %resumed_value = alloca i32, align 4
  %coerce_bits3 = alloca { ptr, i64 }, align 8
  %coerce_bits2 = alloca { ptr, i64 }, align 8
  %2 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %3 = alloca i32, align 4
  %coerce_bits1 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %shared_value = alloca i32, align 4
  %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_key_scope_2" = alloca ptr, align 8
  %__bind_3_observed_value = alloca i32, align 4
  %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_implicit_key_scope_10" = alloca ptr, align 8
  %4 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStaleAfterYieldReleaseWarning, align 1
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
  store i32 1, ptr %shared_value, align 4
  %10 = call ptr @uv_key_scope_enter()
  store ptr %10, ptr %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_key_scope_2", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits, align 8
  %11 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %11, i8 0)
  %12 = load ptr, ptr %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_key_scope_2", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits1, align 8
  %13 = load { i64, i64 }, ptr %coerce_bits1, align 1
  call void @uv_key_acquire(ptr %12, { i64, i64 } %13, i8 0)
  fence seq_cst
  fence seq_cst
  %14 = load i32, ptr %shared_value, align 1
  store i32 %14, ptr %__bind_3_observed_value, align 4
  %15 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aalloc_x5fframe(i64 56, i64 8)
  %16 = getelementptr i8, ptr %15, i64 0
  store i64 0, ptr %16, align 4
  %17 = getelementptr i8, ptr %15, i64 8
  store ptr @"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$resume", ptr %17, align 8
  %18 = getelementptr i8, ptr %15, i64 16
  store ptr null, ptr %18, align 8
  %19 = getelementptr i8, ptr %15, i64 24
  store ptr null, ptr %19, align 8
  %20 = call ptr @uv_key_release_all()
  %21 = getelementptr i8, ptr %15, i64 24
  store ptr %20, ptr %21, align 8
  %22 = load ptr, ptr %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_key_scope_2", align 8
  %23 = getelementptr i8, ptr %15, i64 32
  store ptr %22, ptr %23, align 8
  %24 = load i32, ptr %__bind_3_observed_value, align 4
  %25 = getelementptr i8, ptr %15, i64 40
  store i32 %24, ptr %25, align 4
  %26 = load ptr, ptr %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_implicit_key_scope_10", align 8
  %27 = getelementptr i8, ptr %15, i64 48
  store ptr %26, ptr %27, align 8
  %28 = getelementptr i8, ptr %15, i64 0
  store i64 1, ptr %28, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %2, align 4
  store i8 0, ptr %2, align 1
  %29 = getelementptr i8, ptr %2, i64 8
  store i32 1, ptr %3, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %29, ptr align 1 %3, i64 4, i1 false)
  %30 = getelementptr i8, ptr %29, i64 8
  store ptr %15, ptr %30, align 8
  %31 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %2, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %31, ptr %0, align 4
  ret void

yield.cont:                                       ; No predecessors!
  fence seq_cst
  %32 = call ptr @uv_key_scope_enter()
  store ptr %32, ptr %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_implicit_key_scope_10", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f4AE488D8D2409103, i64 14 }, ptr %coerce_bits2, align 8
  %33 = load { i64, i64 }, ptr %coerce_bits2, align 1
  call void @uv_key_check_conflict({ i64, i64 } %33, i8 0)
  %34 = load ptr, ptr %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_implicit_key_scope_10", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f4AE488D8D2409103, i64 14 }, ptr %coerce_bits3, align 8
  %35 = load { i64, i64 }, ptr %coerce_bits3, align 1
  call void @uv_key_acquire(ptr %34, { i64, i64 } %35, i8 0)
  fence seq_cst
  %36 = load i32, ptr %__bind_3_observed_value, align 1
  store i32 %36, ptr %resumed_value, align 4
  %37 = load ptr, ptr %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_implicit_key_scope_10", align 8
  call void @uv_key_scope_exit(ptr %37)
  %38 = load ptr, ptr %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_key_scope_2", align 8
  call void @uv_key_scope_exit(ptr %38)
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %39 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %39, ptr %0, align 4
  ret void
}

define internal void @"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$resume"(ptr %__uv_host_env, ptr %__uv_async_out, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_frame, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_input, ptr %__panic) {
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
  %shared_value = alloca i32, align 4
  %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_key_scope_2" = alloca ptr, align 8
  %__bind_3_observed_value = alloca i32, align 4
  %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_implicit_key_scope_10" = alloca ptr, align 8
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
  %3 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStaleAfterYieldReleaseWarning, align 1
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
  store ptr %12, ptr %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_key_scope_2", align 8
  %13 = getelementptr i8, ptr %9, i64 40
  %14 = load i32, ptr %13, align 4
  store i32 %14, ptr %__bind_3_observed_value, align 4
  %15 = getelementptr i8, ptr %9, i64 48
  %16 = load ptr, ptr %15, align 8
  store ptr %16, ptr %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_implicit_key_scope_10", align 8
  %17 = getelementptr i8, ptr %9, i64 0
  %18 = load i64, ptr %17, align 4
  switch i64 %18, label %async.resume.invalid [
    i64 1, label %yield.resume.1
  ]

async.resume.invalid:                             ; preds = %poison.cont
  ret void

async.resume.start:                               ; No predecessors!
  store i32 1, ptr %shared_value, align 4
  %19 = call ptr @uv_key_scope_enter()
  store ptr %19, ptr %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_key_scope_2", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits, align 8
  %20 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %20, i8 0)
  %21 = load ptr, ptr %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_key_scope_2", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits6, align 8
  %22 = load { i64, i64 }, ptr %coerce_bits6, align 1
  call void @uv_key_acquire(ptr %21, { i64, i64 } %22, i8 0)
  fence seq_cst
  fence seq_cst
  %23 = load i32, ptr %shared_value, align 1
  store i32 %23, ptr %__bind_3_observed_value, align 4
  %24 = call ptr @uv_key_release_all()
  %25 = getelementptr i8, ptr %9, i64 24
  store ptr %24, ptr %25, align 8
  %26 = load ptr, ptr %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_key_scope_2", align 8
  %27 = getelementptr i8, ptr %9, i64 32
  store ptr %26, ptr %27, align 8
  %28 = load i32, ptr %__bind_3_observed_value, align 4
  %29 = getelementptr i8, ptr %9, i64 40
  store i32 %28, ptr %29, align 4
  %30 = load ptr, ptr %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_implicit_key_scope_10", align 8
  %31 = getelementptr i8, ptr %9, i64 48
  store ptr %30, ptr %31, align 8
  %32 = getelementptr i8, ptr %9, i64 0
  store i64 1, ptr %32, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 0, ptr %1, align 1
  %33 = getelementptr i8, ptr %1, i64 8
  store i32 1, ptr %2, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %33, ptr align 1 %2, i64 4, i1 false)
  %34 = getelementptr i8, ptr %33, i64 8
  store ptr %9, ptr %34, align 8
  %35 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  %36 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %35, ptr %36, align 4
  ret void

yield.cont:                                       ; preds = %yield.resume.1
  %37 = getelementptr i8, ptr %9, i64 24
  %38 = load ptr, ptr %37, align 8
  call void @uv_key_reacquire(ptr %38)
  %39 = getelementptr i8, ptr %9, i64 24
  store ptr null, ptr %39, align 8
  store {} zeroinitializer, ptr %yield_input, align 1
  fence seq_cst
  %40 = call ptr @uv_key_scope_enter()
  store ptr %40, ptr %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_implicit_key_scope_10", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f4AE488D8D2409103, i64 14 }, ptr %coerce_bits7, align 8
  %41 = load { i64, i64 }, ptr %coerce_bits7, align 1
  call void @uv_key_check_conflict({ i64, i64 } %41, i8 0)
  %42 = load ptr, ptr %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_implicit_key_scope_10", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f4AE488D8D2409103, i64 14 }, ptr %coerce_bits8, align 8
  %43 = load { i64, i64 }, ptr %coerce_bits8, align 1
  call void @uv_key_acquire(ptr %42, { i64, i64 } %43, i8 0)
  fence seq_cst
  %44 = load i32, ptr %__bind_3_observed_value, align 1
  store i32 %44, ptr %resumed_value, align 4
  %45 = load ptr, ptr %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_implicit_key_scope_10", align 8
  call void @uv_key_scope_exit(ptr %45)
  %46 = load ptr, ptr %"StaleAfterYieldReleaseWarning_x3a_x3astaleAfterYieldReleaseWarningReference$tmp$__uv_key_scope_2", align 8
  call void @uv_key_scope_exit(ptr %46)
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %47 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %0, align 4
  %48 = load ptr, ptr %__uv_async_out2, align 8
  %49 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %47, ptr %49, align 4
  ret void

yield.resume.1:                                   ; preds = %poison.cont
  br label %yield.cont
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aStaleAfterYieldReleaseWarning(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aStaleAfterYieldReleaseWarning(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #1

define hidden void @__cx_lifecycle_init_StaleAfterYieldReleaseWarning(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aStaleAfterYieldReleaseWarning(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_StaleAfterYieldReleaseWarning(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aStaleAfterYieldReleaseWarning(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aStaleAfterYieldReleaseWarning(ptr %dll_attach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aStaleAfterYieldReleaseWarning(ptr %dll_detach_panic_out)
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
