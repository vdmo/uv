; ==== Source/DispatchDynamicKeyWarning.ll
; ModuleID = 'DispatchDynamicKeyWarning'
source_filename = "DispatchDynamicKeyWarning"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aDispatchDynamicKeyWarning = hidden global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fA8C7F832281A39C5 = internal constant [8 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fA09E307A7F948ACD = internal constant [8 x i8] c"\08\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@__uv_library_attached = internal global i1 false
@__uv_image_panic_record = common hidden global { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, align 4
@llvm.global_ctors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_ctor, ptr null }]
@llvm.global_dtors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_dtor, ptr null }]

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

define void @DispatchDynamicKeyWarning_x3a_x3adispatchDynamicKeyWarningReference(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(16) %values, ptr noundef nonnull align 8 dereferenceable(8) %offset, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %byref_arg6 = alloca i32, align 4
  %byref_arg2 = alloca { i64, i64, i64 }, align 8
  %coerce_bits1 = alloca { ptr, i64 }, align 8
  %byref_arg = alloca { i8, [7 x i8], i64, i64 }, align 8
  %0 = alloca { i64, i64, i64 }, align 8
  %"DispatchDynamicKeyWarning_x3a_x3adispatchDynamicKeyWarningReference$tmp$dispatch_env_storage_8" = alloca { ptr, ptr }, align 8
  %coerce_bits = alloca { ptr, ptr }, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aDispatchDynamicKeyWarning, align 1
  %2 = icmp ne i8 %1, 0
  br i1 %2, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %3 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %3, align 1
  %4 = getelementptr i8, ptr %3, i64 4
  store i32 10, ptr %4, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %5 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 0, ptr %6, align 4
  %7 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3acpu(ptr noundef nonnull align 8 dereferenceable(96) %context)
  store { i64, i64 } %7, ptr %abi_return, align 8
  %8 = load { ptr, ptr }, ptr %abi_return, align 1
  store { ptr, ptr } %8, ptr %coerce_bits, align 8
  %9 = load { i64, i64 }, ptr %coerce_bits, align 1
  %10 = call ptr @uv_parallel_begin({ i64, i64 } %9, i64 -1, ptr null)
  store { ptr, ptr } zeroinitializer, ptr %"DispatchDynamicKeyWarning_x3a_x3adispatchDynamicKeyWarningReference$tmp$dispatch_env_storage_8", align 8
  %11 = getelementptr i8, ptr %"DispatchDynamicKeyWarning_x3a_x3adispatchDynamicKeyWarningReference$tmp$dispatch_env_storage_8", i64 0
  store ptr %values, ptr %11, align 8
  %12 = getelementptr i8, ptr %"DispatchDynamicKeyWarning_x3a_x3adispatchDynamicKeyWarningReference$tmp$dispatch_env_storage_8", i64 8
  store ptr %offset, ptr %12, align 8
  store { i64, i64, i64 } zeroinitializer, ptr %0, align 4
  %13 = getelementptr i8, ptr %0, i64 0
  store i64 64, ptr %13, align 1
  %14 = getelementptr i8, ptr %0, i64 8
  store i64 1, ptr %14, align 1
  %15 = getelementptr i8, ptr %0, i64 16
  store i64 1, ptr %15, align 1
  %16 = load { i64, i64, i64 }, ptr %0, align 4
  store { i8, [7 x i8], i64, i64 } { i8 4, [7 x i8] zeroinitializer, i64 0, i64 4 }, ptr %byref_arg, align 4
  store { ptr, i64 } zeroinitializer, ptr %coerce_bits1, align 8
  %17 = load { i64, i64 }, ptr %coerce_bits1, align 1
  store { i64, i64, i64 } %16, ptr %byref_arg2, align 4
  call void @uv_dispatch_run(ptr noundef nonnull align 8 dereferenceable(24) %byref_arg, i64 8, i64 0, ptr @__cx_dispatch_body_DispatchDynamicKeyWarning_0, ptr null, ptr %"DispatchDynamicKeyWarning_x3a_x3adispatchDynamicKeyWarningReference$tmp$dispatch_env_storage_8", { i64, i64 } %17, ptr null, ptr null, i32 0, i64 0, ptr noundef nonnull byval({ i64, i64, i64 }) align 8 dereferenceable(24) %byref_arg2)
  %18 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %18, align 1
  %19 = getelementptr i8, ptr %18, i64 4
  store i32 0, ptr %19, align 4
  %20 = call i32 @uv_parallel_join(ptr %10)
  %21 = icmp eq i32 %20, 0
  %22 = xor i1 %21, true
  %23 = zext i1 %22 to i8
  %24 = icmp ne i8 %23, 0
  %25 = icmp ne i8 %23, 0
  br i1 %25, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  %26 = load ptr, ptr %__panic, align 8
  %27 = getelementptr i8, ptr %26, i64 0
  store i8 1, ptr %27, align 1
  %28 = load ptr, ptr %__panic, align 8
  %29 = getelementptr i8, ptr %28, i64 4
  store i32 %20, ptr %29, align 4
  br label %if.merge

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %if.result = phi i64 [ 0, %if.then ], [ 0, %if.else ]
  %30 = load ptr, ptr %__panic, align 8
  %31 = getelementptr i8, ptr %30, i64 0
  %32 = load i8, ptr %31, align 1
  %33 = load ptr, ptr %__panic, align 8
  %34 = getelementptr i8, ptr %33, i64 4
  %35 = load i32, ptr %34, align 4
  %36 = icmp ne i8 %32, 0
  %37 = zext i1 %36 to i8
  %38 = icmp ne i8 %37, 0
  %39 = and i1 false, %38
  br i1 %39, label %if.then3, label %if.else4

if.then3:                                         ; preds = %if.merge
  store i32 %35, ptr %byref_arg6, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg6)
  unreachable

if.else4:                                         ; preds = %if.merge
  br label %if.merge5

if.merge5:                                        ; preds = %if.else4
  %40 = icmp ne i8 %32, 0
  %41 = xor i1 %40, true
  %42 = and i1 false, %41
  br i1 %42, label %if.then7, label %if.else8

if.then7:                                         ; preds = %if.merge5
  %43 = load ptr, ptr %__panic, align 8
  %44 = getelementptr i8, ptr %43, i64 0
  store i8 1, ptr %44, align 1
  %45 = load ptr, ptr %__panic, align 8
  %46 = getelementptr i8, ptr %45, i64 4
  store i32 0, ptr %46, align 4
  br label %if.merge9

if.else8:                                         ; preds = %if.merge5
  br label %if.merge9

if.merge9:                                        ; preds = %if.else8, %if.then7
  %if.result10 = phi i64 [ 0, %if.then7 ], [ 0, %if.else8 ]
  %47 = icmp ne i8 %32, 0
  %48 = zext i1 %47 to i8
  %49 = icmp ne i8 %48, 0
  %50 = or i1 false, %49
  %51 = icmp ne i8 %32, 0
  %52 = icmp ne i8 %32, 0
  br i1 %52, label %if.then11, label %if.else12

if.then11:                                        ; preds = %if.merge9
  br label %if.merge13

if.else12:                                        ; preds = %if.merge9
  br label %if.merge13

if.merge13:                                       ; preds = %if.else12, %if.then11
  %if.result14 = phi i32 [ %35, %if.then11 ], [ 0, %if.else12 ]
  %53 = load ptr, ptr %__panic, align 8
  %54 = load i8, ptr %53, align 1
  %55 = icmp ne i8 %54, 0
  br i1 %55, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %if.merge13
  ret void

panic.cont:                                       ; preds = %if.merge13
  ret void
}

define internal void @__cx_dispatch_body_DispatchDynamicKeyWarning_0(ptr %__uv_host_env, ptr noundef nonnull align 8 dereferenceable(8) %elem, ptr noundef nonnull align 8 dereferenceable(16) %env, ptr %result, ptr %__panic) {
entry:
  %abi_return13 = alloca { i64, i64 }, align 8
  %observed = alloca i32, align 4
  %abi_return12 = alloca { i64, i64 }, align 8
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
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aDispatchDynamicKeyWarning, align 1
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
  fence seq_cst
  %11 = load ptr, ptr %env3, align 8
  %12 = getelementptr i8, ptr %11, i64 0
  %13 = load ptr, ptr %12, align 8
  %14 = load ptr, ptr %env3, align 8
  %15 = getelementptr i8, ptr %14, i64 8
  %16 = load ptr, ptr %15, align 8
  %17 = load i64, ptr %16, align 4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %18 = load ptr, ptr %__panic5, align 8
  %19 = load i8, ptr %18, align 1
  %20 = icmp ne i8 %19, 0
  br i1 %20, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %21 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %21, align 1
  %22 = getelementptr i8, ptr %21, i64 4
  store i32 4, ptr %22, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %23 = load ptr, ptr %__panic5, align 8
  %24 = getelementptr i8, ptr %23, i64 0
  %25 = load i8, ptr %24, align 1
  %26 = load ptr, ptr %__panic5, align 8
  %27 = getelementptr i8, ptr %26, i64 4
  %28 = load i32, ptr %27, align 4
  %29 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$06")
  store { i64, i64 } %29, ptr %abi_return7, align 8
  %30 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return7, align 1
  ret void

panic.cont:                                       ; preds = %check_ok
  %31 = load i64, ptr %index, align 4
  %32 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %31, i64 %17)
  %33 = extractvalue { i64, i1 } %32, 0
  %34 = extractvalue { i64, i1 } %32, 1
  %35 = freeze i64 %33
  br i1 %34, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  %36 = load { ptr, i64 }, ptr %13, align 8
  %37 = extractvalue { ptr, i64 } %36, 1
  %38 = icmp ult i64 %35, %37
  br i1 %38, label %check_ok8, label %check_fail9

op_fail:                                          ; preds = %panic.cont
  %39 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %39, align 1
  %40 = getelementptr i8, ptr %39, i64 4
  store i32 4, ptr %40, align 4
  ret void

check_ok8:                                        ; preds = %check_fail9, %op_ok
  %41 = load ptr, ptr %__panic5, align 8
  %42 = load i8, ptr %41, align 1
  %43 = icmp ne i8 %42, 0
  br i1 %43, label %panic.take10, label %panic.cont11

check_fail9:                                      ; preds = %op_ok
  %44 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %44, align 1
  %45 = getelementptr i8, ptr %44, i64 4
  store i32 6, ptr %45, align 4
  br label %check_ok8

panic.take10:                                     ; preds = %check_ok8
  %46 = load ptr, ptr %__panic5, align 8
  %47 = getelementptr i8, ptr %46, i64 0
  %48 = load i8, ptr %47, align 1
  %49 = load ptr, ptr %__panic5, align 8
  %50 = getelementptr i8, ptr %49, i64 4
  %51 = load i32, ptr %50, align 4
  %52 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$06")
  store { i64, i64 } %52, ptr %abi_return12, align 8
  %53 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return12, align 1
  ret void

panic.cont11:                                     ; preds = %check_ok8
  fence seq_cst
  %54 = load { ptr, i64 }, ptr %13, align 8
  %55 = extractvalue { ptr, i64 } %54, 0
  %56 = getelementptr i32, ptr %55, i64 %35
  %57 = load i32, ptr %56, align 1
  store i32 %57, ptr %observed, align 4
  %58 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$06")
  store { i64, i64 } %58, ptr %abi_return13, align 8
  %59 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return13, align 1
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aDispatchDynamicKeyWarning(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aDispatchDynamicKeyWarning(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #2

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64) #3

define hidden void @__cx_lifecycle_init_DispatchDynamicKeyWarning(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aDispatchDynamicKeyWarning(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_DispatchDynamicKeyWarning(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aDispatchDynamicKeyWarning(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aDispatchDynamicKeyWarning(ptr %dll_attach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aDispatchDynamicKeyWarning(ptr %dll_detach_panic_out)
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
attributes #1 = { noreturn nounwind }
attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #3 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
