; ==== Source/DynamicReceiverExpiredPanic.ll
; ModuleID = 'DynamicReceiverExpiredPanic'
source_filename = "DynamicReceiverExpiredPanic"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aDynamicReceiverExpiredPanic = global i8 0
@vtable_x3a_x3aDynamicReceiverExpiredPanic_x3a_x3aDynamicReceiverExpiredOwner_x3a_x3acl_x3a_x3aDynamicReceiverExpiredPanic_x3a_x3aDynamicReceiverExpiredReadable = internal constant { i64, i64, ptr, ptr } { i64 4, i64 4, ptr @ultraviolet_x3a_x3aruntime_x3a_x3adrop_x3a_x3aDynamicReceiverExpiredPanic_x3a_x3aDynamicReceiverExpiredOwner, ptr @DynamicReceiverExpiredPanic_x3a_x3aDynamicReceiverExpiredOwner_x3a_x3areadExpiredDynamicValue }
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f15104CBF7A917380 = internal unnamed_addr constant [24 x i8] c"dynamic-receiver-expired", align 1

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5ftag_x5ffrom(ptr, ptr) #0

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16)) #0

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3anew_x5fscoped(ptr noundef nonnull align 8 dereferenceable(40)) #0

define i32 @DynamicReceiverExpiredPanic_x3a_x3aDynamicReceiverExpiredOwner_x3a_x3areadExpiredDynamicValue(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  %2 = getelementptr i8, ptr %self, i64 0
  %3 = load i32, ptr %2, align 1
  ret i32 %3
}

define i32 @DynamicReceiverExpiredPanic_x3a_x3adynamicReceiverExpiredPanicValue(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %freed5 = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %abi_return4 = alloca { i64, i64 }, align 8
  %freed = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %byref_arg3 = alloca i64, align 8
  %byref_arg2 = alloca i64, align 8
  %byref_arg = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %0 = alloca { i32, [0 x i32] }, align 8
  %opened1 = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %opened = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %aggregate.literal = alloca { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] }, align 8
  %options = alloca { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] }, align 8
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aDynamicReceiverExpiredPanic, align 1
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
  call void @llvm.memset.p0.i64(ptr align 8 %aggregate.literal, i8 0, i64 40, i1 false)
  store i64 0, ptr %aggregate.literal, align 8
  %10 = getelementptr i8, ptr %aggregate.literal, i64 8
  store { i8, [7 x i8], [24 x i8], [0 x i64] } zeroinitializer, ptr %10, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %options, ptr align 8 %aggregate.literal, i64 40, i1 false)
  %11 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3anew_x5fscoped(ptr noundef nonnull align 8 dereferenceable(40) %options)
  store { i64, i64 } %11, ptr %abi_return, align 8
  %12 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return, align 1
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %12, ptr %opened1, align 4
  store { i32, [0 x i32] } zeroinitializer, ptr %0, align 4
  %13 = getelementptr i8, ptr %0, i64 0
  store i32 37, ptr %13, align 1
  %14 = load { i32, [0 x i32] }, ptr %0, align 4
  %15 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %opened1, align 4
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %15, ptr %byref_arg, align 4
  store i64 4, ptr %byref_arg2, align 4
  store i64 4, ptr %byref_arg3, align 4
  %16 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aalloc(ptr noundef nonnull align 8 dereferenceable(16) %byref_arg, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg2, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg3)
  store { i32, [0 x i32] } %14, ptr %16, align 4
  %17 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %opened1, align 4
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %17, ptr %byref_arg, align 4
  store i64 4, ptr %byref_arg2, align 4
  store i64 4, ptr %byref_arg3, align 4
  %18 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aalloc(ptr noundef nonnull align 8 dereferenceable(16) %byref_arg, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg2, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg3)
  %19 = load { i32, [0 x i32] }, ptr %16, align 4
  store { i32, [0 x i32] } %19, ptr %18, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5ftag_x5ffrom(ptr %18, ptr %16)
  %20 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %opened1, align 4
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %20, ptr %byref_arg, align 4
  store i64 16, ptr %byref_arg2, align 4
  store i64 8, ptr %byref_arg3, align 4
  %21 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aalloc(ptr noundef nonnull align 8 dereferenceable(16) %byref_arg, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg2, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg3)
  %22 = insertvalue { ptr, ptr } zeroinitializer, ptr %18, 0
  %23 = insertvalue { ptr, ptr } %22, ptr @vtable_x3a_x3aDynamicReceiverExpiredPanic_x3a_x3aDynamicReceiverExpiredOwner_x3a_x3acl_x3a_x3aDynamicReceiverExpiredPanic_x3a_x3aDynamicReceiverExpiredReadable, 1
  store { ptr, ptr } %23, ptr %21, align 8
  %24 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %opened1)
  store { i64, i64 } %24, ptr %abi_return4, align 8
  %25 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return4, align 1
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %25, ptr %freed5, align 4
  %26 = load { ptr, ptr }, ptr %21, align 8
  %27 = extractvalue { ptr, ptr } %26, 0
  %28 = extractvalue { ptr, ptr } %26, 1
  %29 = getelementptr ptr, ptr %28, i64 3
  %30 = load ptr, ptr %29, align 8
  %31 = call i8 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5fis_x5factive(ptr %27)
  %32 = icmp ne i8 %31, 0
  br i1 %32, label %dynrecv.addr.valid, label %dynrecv.addr.expired

dynrecv.addr.valid:                               ; preds = %poison.cont
  %33 = call i32 %30(ptr %27, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  br label %dynrecv.addr.join

dynrecv.addr.expired:                             ; preds = %poison.cont
  %34 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %34, align 1
  %35 = getelementptr i8, ptr %34, i64 4
  store i32 9, ptr %35, align 4
  br label %dynrecv.addr.join

dynrecv.addr.join:                                ; preds = %dynrecv.addr.valid, %dynrecv.addr.expired
  %dynrecv.result = phi i32 [ %33, %dynrecv.addr.valid ], [ 0, %dynrecv.addr.expired ]
  %36 = load ptr, ptr %__panic, align 8
  %37 = load i8, ptr %36, align 1
  %38 = icmp ne i8 %37, 0
  br i1 %38, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %dynrecv.addr.join
  %39 = load ptr, ptr %__panic, align 8
  %40 = getelementptr i8, ptr %39, i64 0
  %41 = load i8, ptr %40, align 1
  %42 = load ptr, ptr %__panic, align 8
  %43 = getelementptr i8, ptr %42, i64 4
  %44 = load i32, ptr %43, align 4
  %45 = load ptr, ptr %__panic, align 8
  %46 = getelementptr i8, ptr %45, i64 4
  %47 = load i32, ptr %46, align 4
  ret i32 %47

panic.cont:                                       ; preds = %dynrecv.addr.join
  ret i32 %dynrecv.result
}

define i32 @DynamicReceiverExpiredPanic_x3a_x3amain(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aDynamicReceiverExpiredPanic, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aDynamicReceiverExpiredPanic, align 1
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
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aDynamicReceiverExpiredPanic, align 1
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
  %23 = call i32 @DynamicReceiverExpiredPanic_x3a_x3adynamicReceiverExpiredPanicValue(ptr noundef nonnull align 8 dereferenceable(8) %__panic)
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
  ret i32 %23
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aDynamicReceiverExpiredPanic(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aDynamicReceiverExpiredPanic(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adrop_x3a_x3aDynamicReceiverExpiredPanic_x3a_x3aDynamicReceiverExpiredOwner(ptr noundef nonnull align 8 dereferenceable(8) %data, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #1

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #2

declare ptr @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aalloc(ptr, ptr, ptr)

declare i8 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5fis_x5factive(ptr)

define void @__cx_lifecycle_init_DynamicReceiverExpiredPanic(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aDynamicReceiverExpiredPanic(ptr %0)
  ret void
}

define void @__cx_lifecycle_deinit_DynamicReceiverExpiredPanic(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aDynamicReceiverExpiredPanic(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aDynamicReceiverExpiredPanic(ptr %entry_panic_out)
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
  %24 = call i32 @DynamicReceiverExpiredPanic_x3a_x3amain(ptr %entry_ctx, ptr %entry_panic_out)
  %entry_deinit_panic_seen = alloca i8, align 1
  %entry_deinit_panic_code = alloca i32, align 4
  store i8 0, ptr %entry_deinit_panic_seen, align 1
  store i32 0, ptr %entry_deinit_panic_code, align 4
  %25 = load i8, ptr %entry_panic, align 1
  %26 = icmp ne i8 %25, 0
  br i1 %26, label %entry.panic, label %entry.deinit

entry.deinit:                                     ; preds = %entry.init.cont
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aDynamicReceiverExpiredPanic(ptr %entry_panic_out)
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

attributes #0 = { nounwind }
attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
