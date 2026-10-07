; ==== Source/DynamicKeyRuntimeInfo.ll
; ModuleID = 'DynamicKeyRuntimeInfo'
source_filename = "DynamicKeyRuntimeInfo"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aDynamicKeyRuntimeInfo = hidden global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fA8C7F832281A39C5 = internal constant [8 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fA37F35E9B1876EEF = internal constant [7 x i8] c"values.", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAF63BD4C8601B7DF = internal constant [1 x i8] zeroinitializer, align 1
@__uv_library_attached = internal global i1 false
@__uv_image_panic_record = common hidden global { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, align 4
@llvm.global_ctors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_ctor, ptr null }]
@llvm.global_dtors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_dtor, ptr null }]

; Function Attrs: nounwind
declare void @uv_key_acquire(ptr, { i64, i64 }, i8) #0

; Function Attrs: nounwind
declare void @uv_key_check_conflict({ i64, i64 }, i8) #0

; Function Attrs: nounwind
declare ptr @uv_key_scope_enter() #0

; Function Attrs: nounwind
declare void @uv_key_scope_exit(ptr) #0

define i32 @DynamicKeyRuntimeInfo_x3a_x3adynamicKeyRuntimeInfoReference(ptr noundef nonnull align 8 dereferenceable(8) %index, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %coerce_bits2 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %"DynamicKeyRuntimeInfo_x3a_x3adynamicKeyRuntimeInfoReference$tmp$__uv_key_scope_101" = alloca ptr, align 8
  %"DynamicKeyRuntimeInfo_x3a_x3adynamicKeyRuntimeInfoReference$tmp$__uv_key_scope_10" = alloca ptr, align 8
  %observed = alloca i32, align 4
  %aggregate.literal = alloca [4 x i32], align 4
  %values = alloca [4 x i32], align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aDynamicKeyRuntimeInfo, align 1
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
  %9 = load i64, ptr %index, align 4
  %10 = icmp uge i64 %9, 0
  %11 = zext i1 %10 to i8
  %12 = icmp ne i8 %11, 0
  %13 = icmp ne i8 %11, 0
  br i1 %13, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  br label %if.merge

if.else:                                          ; preds = %poison.cont
  %14 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %14, align 1
  %15 = getelementptr i8, ptr %14, i64 4
  store i32 11, ptr %15, align 4
  %16 = load ptr, ptr %__panic, align 8
  %17 = getelementptr i8, ptr %16, i64 4
  %18 = load i32, ptr %17, align 4
  ret i32 %18

if.merge:                                         ; preds = %if.then
  %19 = getelementptr [4 x i32], ptr %aggregate.literal, i64 0, i64 0
  store i32 1, ptr %19, align 4
  %20 = getelementptr [4 x i32], ptr %aggregate.literal, i64 0, i64 1
  store i32 2, ptr %20, align 4
  %21 = getelementptr [4 x i32], ptr %aggregate.literal, i64 0, i64 2
  store i32 3, ptr %21, align 4
  %22 = getelementptr [4 x i32], ptr %aggregate.literal, i64 0, i64 3
  store i32 4, ptr %22, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %values, ptr align 4 %aggregate.literal, i64 16, i1 false)
  store i32 0, ptr %observed, align 4
  %23 = call ptr @uv_key_scope_enter()
  store ptr %23, ptr %"DynamicKeyRuntimeInfo_x3a_x3adynamicKeyRuntimeInfoReference$tmp$__uv_key_scope_101", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fA37F35E9B1876EEF, i64 7 }, ptr %coerce_bits, align 8
  %24 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %24, i8 0)
  %25 = load ptr, ptr %"DynamicKeyRuntimeInfo_x3a_x3adynamicKeyRuntimeInfoReference$tmp$__uv_key_scope_101", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fA37F35E9B1876EEF, i64 7 }, ptr %coerce_bits2, align 8
  %26 = load { i64, i64 }, ptr %coerce_bits2, align 1
  call void @uv_key_acquire(ptr %25, { i64, i64 } %26, i8 0)
  fence seq_cst
  %27 = load i64, ptr %index, align 4
  %28 = icmp ult i64 %27, 4
  br i1 %28, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %if.merge
  %29 = load ptr, ptr %__panic, align 8
  %30 = load i8, ptr %29, align 1
  %31 = icmp ne i8 %30, 0
  br i1 %31, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %if.merge
  %32 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %32, align 1
  %33 = getelementptr i8, ptr %32, i64 4
  store i32 6, ptr %33, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %34 = load ptr, ptr %__panic, align 8
  %35 = getelementptr i8, ptr %34, i64 0
  %36 = load i8, ptr %35, align 1
  %37 = load ptr, ptr %__panic, align 8
  %38 = getelementptr i8, ptr %37, i64 4
  %39 = load i32, ptr %38, align 4
  %40 = load ptr, ptr %"DynamicKeyRuntimeInfo_x3a_x3adynamicKeyRuntimeInfoReference$tmp$__uv_key_scope_101", align 8
  call void @uv_key_scope_exit(ptr %40)
  %41 = load ptr, ptr %__panic, align 8
  %42 = getelementptr i8, ptr %41, i64 4
  %43 = load i32, ptr %42, align 4
  ret i32 %43

panic.cont:                                       ; preds = %check_ok
  fence seq_cst
  br i1 true, label %check_ok3, label %check_fail4

check_ok3:                                        ; preds = %check_fail4, %panic.cont
  %44 = load ptr, ptr %__panic, align 8
  %45 = load i8, ptr %44, align 1
  %46 = icmp ne i8 %45, 0
  br i1 %46, label %panic.take5, label %panic.cont6

check_fail4:                                      ; preds = %panic.cont
  %47 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %47, align 1
  %48 = getelementptr i8, ptr %47, i64 4
  store i32 4, ptr %48, align 4
  br label %check_ok3

panic.take5:                                      ; preds = %check_ok3
  %49 = load ptr, ptr %__panic, align 8
  %50 = getelementptr i8, ptr %49, i64 0
  %51 = load i8, ptr %50, align 1
  %52 = load ptr, ptr %__panic, align 8
  %53 = getelementptr i8, ptr %52, i64 4
  %54 = load i32, ptr %53, align 4
  %55 = load ptr, ptr %"DynamicKeyRuntimeInfo_x3a_x3adynamicKeyRuntimeInfoReference$tmp$__uv_key_scope_101", align 8
  call void @uv_key_scope_exit(ptr %55)
  %56 = load ptr, ptr %__panic, align 8
  %57 = getelementptr i8, ptr %56, i64 4
  %58 = load i32, ptr %57, align 4
  ret i32 %58

panic.cont6:                                      ; preds = %check_ok3
  %59 = load i64, ptr %index, align 4
  %60 = getelementptr i32, ptr %values, i64 %59
  %61 = load i32, ptr %60, align 4
  %62 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %61, i32 0)
  %63 = extractvalue { i32, i1 } %62, 0
  %64 = extractvalue { i32, i1 } %62, 1
  %65 = freeze i32 %63
  br i1 %64, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont6
  store i32 %65, ptr %observed, align 4
  %66 = load ptr, ptr %"DynamicKeyRuntimeInfo_x3a_x3adynamicKeyRuntimeInfoReference$tmp$__uv_key_scope_101", align 8
  call void @uv_key_scope_exit(ptr %66)
  %67 = load i32, ptr %observed, align 4
  ret i32 %67

op_fail:                                          ; preds = %panic.cont6
  %68 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %68, align 1
  %69 = getelementptr i8, ptr %68, i64 4
  store i32 4, ptr %69, align 4
  ret i32 0
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aDynamicKeyRuntimeInfo(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aDynamicKeyRuntimeInfo(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #2

define hidden void @__cx_lifecycle_init_DynamicKeyRuntimeInfo(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aDynamicKeyRuntimeInfo(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_DynamicKeyRuntimeInfo(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aDynamicKeyRuntimeInfo(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aDynamicKeyRuntimeInfo(ptr %dll_attach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aDynamicKeyRuntimeInfo(ptr %dll_detach_panic_out)
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
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
