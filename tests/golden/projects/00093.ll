; ==== Source/RuntimeLibMissing.ll
; ModuleID = 'RuntimeLibMissing'
source_filename = "RuntimeLibMissing"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aRuntimeLibMissing = global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1

define i32 @RuntimeLibMissing_x3a_x3amain(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aRuntimeLibMissing, align 1
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
  ret i32 0
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aRuntimeLibMissing(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aRuntimeLibMissing(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define void @__cx_lifecycle_init_RuntimeLibMissing(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aRuntimeLibMissing(ptr %0)
  ret void
}

define void @__cx_lifecycle_deinit_RuntimeLibMissing(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aRuntimeLibMissing(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aRuntimeLibMissing(ptr %entry_panic_out)
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
  %24 = call i32 @RuntimeLibMissing_x3a_x3amain(ptr %entry_ctx, ptr %entry_panic_out)
  %entry_deinit_panic_seen = alloca i8, align 1
  %entry_deinit_panic_code = alloca i32, align 4
  store i8 0, ptr %entry_deinit_panic_seen, align 1
  store i32 0, ptr %entry_deinit_panic_code, align 4
  %25 = load i8, ptr %entry_panic, align 1
  %26 = icmp ne i8 %25, 0
  br i1 %26, label %entry.panic, label %entry.deinit

entry.deinit:                                     ; preds = %entry.init.cont
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aRuntimeLibMissing(ptr %entry_panic_out)
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
