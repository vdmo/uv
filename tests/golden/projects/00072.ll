; ==== Source/FineGrainedLoopKeyWarning.ll
; ModuleID = 'FineGrainedLoopKeyWarning'
source_filename = "FineGrainedLoopKeyWarning"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aFineGrainedLoopKeyWarning = hidden global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fA8C7F832281A39C5 = internal constant [8 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2CDCDC0DFC5D1141 = internal constant [8 x i8] c"\04\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fCA649E0BE1B85F41 = internal constant [24 x i8] c"container.f:leaf.f:value", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAF63BD4C8601B7DF = internal constant [1 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f7214997F83F34938 = internal constant [9 x i8] c"container", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f89CD31291D2AEFA4 = internal constant [8 x i8] c"\01\00\00\00\00\00\00\00", align 1
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

define i32 @FineGrainedLoopKeyWarning_x3a_x3afineGrainedLoopKeyWarningReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %coerce_bits6 = alloca { ptr, i64 }, align 8
  %coerce_bits5 = alloca { ptr, i64 }, align 8
  %"FineGrainedLoopKeyWarning_x3a_x3afineGrainedLoopKeyWarningReference$tmp$__uv_implicit_key_scope_124" = alloca ptr, align 8
  %"FineGrainedLoopKeyWarning_x3a_x3afineGrainedLoopKeyWarningReference$tmp$__uv_implicit_key_scope_12" = alloca ptr, align 8
  %coerce_bits3 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %"FineGrainedLoopKeyWarning_x3a_x3afineGrainedLoopKeyWarningReference$tmp$__uv_key_scope_82" = alloca ptr, align 8
  %"FineGrainedLoopKeyWarning_x3a_x3afineGrainedLoopKeyWarningReference$tmp$__uv_key_scope_8" = alloca ptr, align 8
  %total = alloca i32, align 4
  %index = alloca i64, align 8
  %aggregate.literal1 = alloca { i32, [0 x i32] }, align 4
  %aggregate.literal = alloca { { i32, [0 x i32] }, [0 x i32] }, align 4
  %container = alloca { { i32, [0 x i32] }, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aFineGrainedLoopKeyWarning, align 1
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
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 4, i1 false)
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal1, i8 0, i64 4, i1 false)
  store i32 1, ptr %aggregate.literal1, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %aggregate.literal, ptr align 4 %aggregate.literal1, i64 4, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %container, ptr align 4 %aggregate.literal, i64 4, i1 false)
  store i64 0, ptr %index, align 4
  store i32 0, ptr %total, align 4
  br label %loop.cond

loop.end:                                         ; preds = %loop.cond
  %9 = load i32, ptr %total, align 4
  ret i32 %9

loop.cond:                                        ; preds = %op_ok11, %poison.cont
  %10 = load i64, ptr %index, align 4
  %11 = icmp ult i64 %10, 4
  %12 = zext i1 %11 to i8
  %13 = icmp ne i8 %12, 0
  br i1 %13, label %loop.body, label %loop.end

loop.body:                                        ; preds = %loop.cond
  %14 = call ptr @uv_key_scope_enter()
  store ptr %14, ptr %"FineGrainedLoopKeyWarning_x3a_x3afineGrainedLoopKeyWarningReference$tmp$__uv_key_scope_82", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fCA649E0BE1B85F41, i64 24 }, ptr %coerce_bits, align 8
  %15 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %15, i8 0)
  %16 = load ptr, ptr %"FineGrainedLoopKeyWarning_x3a_x3afineGrainedLoopKeyWarningReference$tmp$__uv_key_scope_82", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fCA649E0BE1B85F41, i64 24 }, ptr %coerce_bits3, align 8
  %17 = load { i64, i64 }, ptr %coerce_bits3, align 1
  call void @uv_key_acquire(ptr %16, { i64, i64 } %17, i8 0)
  fence seq_cst
  fence seq_cst
  fence seq_cst
  %18 = call ptr @uv_key_scope_enter()
  store ptr %18, ptr %"FineGrainedLoopKeyWarning_x3a_x3afineGrainedLoopKeyWarningReference$tmp$__uv_implicit_key_scope_124", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f7214997F83F34938, i64 9 }, ptr %coerce_bits5, align 8
  %19 = load { i64, i64 }, ptr %coerce_bits5, align 1
  call void @uv_key_check_conflict({ i64, i64 } %19, i8 0)
  %20 = load ptr, ptr %"FineGrainedLoopKeyWarning_x3a_x3afineGrainedLoopKeyWarningReference$tmp$__uv_implicit_key_scope_124", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f7214997F83F34938, i64 9 }, ptr %coerce_bits6, align 8
  %21 = load { i64, i64 }, ptr %coerce_bits6, align 1
  call void @uv_key_acquire(ptr %20, { i64, i64 } %21, i8 0)
  fence seq_cst
  fence seq_cst
  fence seq_cst
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %loop.body
  %22 = load ptr, ptr %__panic, align 8
  %23 = load i8, ptr %22, align 1
  %24 = icmp ne i8 %23, 0
  br i1 %24, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %loop.body
  %25 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %25, align 1
  %26 = getelementptr i8, ptr %25, i64 4
  store i32 4, ptr %26, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 0
  %29 = load i8, ptr %28, align 1
  %30 = load ptr, ptr %__panic, align 8
  %31 = getelementptr i8, ptr %30, i64 4
  %32 = load i32, ptr %31, align 4
  %33 = load ptr, ptr %"FineGrainedLoopKeyWarning_x3a_x3afineGrainedLoopKeyWarningReference$tmp$__uv_implicit_key_scope_124", align 8
  call void @uv_key_scope_exit(ptr %33)
  %34 = load ptr, ptr %"FineGrainedLoopKeyWarning_x3a_x3afineGrainedLoopKeyWarningReference$tmp$__uv_key_scope_82", align 8
  call void @uv_key_scope_exit(ptr %34)
  %35 = load ptr, ptr %__panic, align 8
  %36 = getelementptr i8, ptr %35, i64 4
  %37 = load i32, ptr %36, align 4
  ret i32 %37

panic.cont:                                       ; preds = %check_ok
  %38 = load i32, ptr %total, align 4
  %39 = getelementptr i8, ptr %container, i64 0
  %40 = getelementptr i8, ptr %39, i64 0
  %41 = load i32, ptr %40, align 1
  %42 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %38, i32 %41)
  %43 = extractvalue { i32, i1 } %42, 0
  %44 = extractvalue { i32, i1 } %42, 1
  %45 = freeze i32 %43
  br i1 %44, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  store i32 %45, ptr %total, align 4
  %46 = load ptr, ptr %"FineGrainedLoopKeyWarning_x3a_x3afineGrainedLoopKeyWarningReference$tmp$__uv_implicit_key_scope_124", align 8
  call void @uv_key_scope_exit(ptr %46)
  %47 = load ptr, ptr %"FineGrainedLoopKeyWarning_x3a_x3afineGrainedLoopKeyWarningReference$tmp$__uv_key_scope_82", align 8
  call void @uv_key_scope_exit(ptr %47)
  br i1 true, label %check_ok7, label %check_fail8

op_fail:                                          ; preds = %panic.cont
  %48 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %48, align 1
  %49 = getelementptr i8, ptr %48, i64 4
  store i32 4, ptr %49, align 4
  ret i32 0

check_ok7:                                        ; preds = %check_fail8, %op_ok
  %50 = load ptr, ptr %__panic, align 8
  %51 = load i8, ptr %50, align 1
  %52 = icmp ne i8 %51, 0
  br i1 %52, label %panic.take9, label %panic.cont10

check_fail8:                                      ; preds = %op_ok
  %53 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %53, align 1
  %54 = getelementptr i8, ptr %53, i64 4
  store i32 4, ptr %54, align 4
  br label %check_ok7

panic.take9:                                      ; preds = %check_ok7
  %55 = load ptr, ptr %__panic, align 8
  %56 = getelementptr i8, ptr %55, i64 0
  %57 = load i8, ptr %56, align 1
  %58 = load ptr, ptr %__panic, align 8
  %59 = getelementptr i8, ptr %58, i64 4
  %60 = load i32, ptr %59, align 4
  %61 = load ptr, ptr %__panic, align 8
  %62 = getelementptr i8, ptr %61, i64 4
  %63 = load i32, ptr %62, align 4
  ret i32 %63

panic.cont10:                                     ; preds = %check_ok7
  %64 = load i64, ptr %index, align 4
  %65 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %64, i64 1)
  %66 = extractvalue { i64, i1 } %65, 0
  %67 = extractvalue { i64, i1 } %65, 1
  %68 = freeze i64 %66
  br i1 %67, label %op_fail12, label %op_ok11

op_ok11:                                          ; preds = %panic.cont10
  store i64 %68, ptr %index, align 4
  br label %loop.cond

op_fail12:                                        ; preds = %panic.cont10
  %69 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %69, align 1
  %70 = getelementptr i8, ptr %69, i64 4
  store i32 4, ptr %70, align 4
  ret i32 0
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aFineGrainedLoopKeyWarning(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aFineGrainedLoopKeyWarning(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
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

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64) #3

define hidden void @__cx_lifecycle_init_FineGrainedLoopKeyWarning(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aFineGrainedLoopKeyWarning(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_FineGrainedLoopKeyWarning(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aFineGrainedLoopKeyWarning(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aFineGrainedLoopKeyWarning(ptr %dll_attach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aFineGrainedLoopKeyWarning(ptr %dll_detach_panic_out)
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
attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #3 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
