; ==== Source/SystemExitExecutable.ll
; ModuleID = 'SystemExitExecutable'
source_filename = "SystemExitExecutable"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aSystemExitExecutable = global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f6CE032EBFE88A752 = internal constant [4 x i8] c"\17\00\00\00", align 1

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3asystem_x3a_x3aexit(ptr noundef nonnull align 8 dereferenceable(16), ptr noundef nonnull align 4 dereferenceable(4)) #0

define i32 @SystemExitExecutable_x3a_x3amain(ptr noundef nonnull align 8 dereferenceable(16) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"SystemExitExecutable_x3a_x3amain$tmp$call_ref_tmp_3" = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aSystemExitExecutable, align 1
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
  %9 = getelementptr i8, ptr %context, i64 0
  store i32 23, ptr %"SystemExitExecutable_x3a_x3amain$tmp$call_ref_tmp_3", align 4
  %10 = getelementptr i8, ptr %context, i64 0
  call void @ultraviolet_x3a_x3aruntime_x3a_x3asystem_x3a_x3aexit(ptr noundef nonnull align 8 dereferenceable(16) %10, ptr noundef nonnull align 4 dereferenceable(4) %"SystemExitExecutable_x3a_x3amain$tmp$call_ref_tmp_3")
  unreachable
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aSystemExitExecutable(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aSystemExitExecutable(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define void @__cx_lifecycle_init_SystemExitExecutable(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aSystemExitExecutable(ptr %0)
  ret void
}

define void @__cx_lifecycle_deinit_SystemExitExecutable(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aSystemExitExecutable(ptr %0)
  ret void
}

define i32 @main() {
entry:
  %entry_panic_code_arg = alloca i32, align 4
  %entry_ctx_bundle = alloca { { ptr, ptr }, [0 x i64] }, align 8
  %entry_panic = alloca { i8, [3 x i8], i32, [0 x i32] }, align 8
  store { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, ptr %entry_panic, align 4
  %entry_panic_out = alloca ptr, align 8
  store ptr %entry_panic, ptr %entry_panic_out, align 8
  %entry_ctx = alloca { { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, [0 x i64] }, align 8
  store { { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, [0 x i64] } zeroinitializer, ptr %entry_ctx, align 8
  call void @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x5finit(ptr %entry_ctx)
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aSystemExitExecutable(ptr %entry_panic_out)
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
  store { { ptr, ptr }, [0 x i64] } zeroinitializer, ptr %entry_ctx_bundle, align 8
  %5 = getelementptr i8, ptr %entry_ctx, i64 48
  %6 = load { ptr, ptr }, ptr %5, align 8
  %7 = getelementptr i8, ptr %entry_ctx_bundle, i64 0
  store { ptr, ptr } %6, ptr %7, align 8
  %8 = load { { ptr, ptr }, [0 x i64] }, ptr %entry_ctx_bundle, align 8
  %entry_ctx_bundle1 = alloca { { ptr, ptr }, [0 x i64] }, align 8
  store { { ptr, ptr }, [0 x i64] } %8, ptr %entry_ctx_bundle1, align 8
  %9 = call i32 @SystemExitExecutable_x3a_x3amain(ptr %entry_ctx_bundle1, ptr %entry_panic_out)
  %entry_deinit_panic_seen = alloca i8, align 1
  %entry_deinit_panic_code = alloca i32, align 4
  store i8 0, ptr %entry_deinit_panic_seen, align 1
  store i32 0, ptr %entry_deinit_panic_code, align 4
  %10 = load i8, ptr %entry_panic, align 1
  %11 = icmp ne i8 %10, 0
  br i1 %11, label %entry.panic, label %entry.deinit

entry.deinit:                                     ; preds = %entry.init.cont
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aSystemExitExecutable(ptr %entry_panic_out)
  %12 = load i8, ptr %entry_panic, align 1
  %13 = icmp ne i8 %12, 0
  br i1 %13, label %entry.deinit.panic.capture, label %entry.deinit.panic.cont

entry.deinit.panic.capture:                       ; preds = %entry.deinit
  %14 = load i8, ptr %entry_deinit_panic_seen, align 1
  %15 = icmp ne i8 %14, 0
  %16 = getelementptr i8, ptr %entry_panic, i64 4
  %17 = load i32, ptr %16, align 4
  br i1 %15, label %entry.deinit.panic.clear, label %entry.deinit.panic.store

entry.deinit.panic.cont:                          ; preds = %entry.deinit.panic.clear, %entry.deinit
  %18 = load i8, ptr %entry_deinit_panic_seen, align 1
  %19 = icmp ne i8 %18, 0
  br i1 %19, label %entry.deinit.panic.restore, label %entry.ret

entry.deinit.panic.store:                         ; preds = %entry.deinit.panic.capture
  store i8 1, ptr %entry_deinit_panic_seen, align 1
  store i32 %17, ptr %entry_deinit_panic_code, align 4
  br label %entry.deinit.panic.clear

entry.deinit.panic.clear:                         ; preds = %entry.deinit.panic.store, %entry.deinit.panic.capture
  store i8 0, ptr %entry_panic, align 1
  %20 = getelementptr i8, ptr %entry_panic, i64 4
  store i32 0, ptr %20, align 4
  br label %entry.deinit.panic.cont

entry.deinit.panic.restore:                       ; preds = %entry.deinit.panic.cont
  %21 = load i32, ptr %entry_deinit_panic_code, align 4
  store i8 1, ptr %entry_panic, align 1
  %22 = getelementptr i8, ptr %entry_panic, i64 4
  store i32 %21, ptr %22, align 4
  br label %entry.panic

entry.ret:                                        ; preds = %entry.deinit.panic.cont
  ret i32 %9
}

declare void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr)

declare void @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x5finit(ptr)

attributes #0 = { nounwind }
