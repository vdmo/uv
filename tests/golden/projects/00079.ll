; ==== Source/SpeculativeLargeStructWarning.ll
; ModuleID = 'SpeculativeLargeStructWarning'
source_filename = "SpeculativeLargeStructWarning"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aSpeculativeLargeStructWarning = hidden global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fE6BD86443DF8CE07 = internal constant [8 x i8] c"\02\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fCFB8A9D063B5E9E5 = internal constant [7 x i8] c"payload", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAF63BC4C8601B62C = internal constant [1 x i8] c"\01", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fA8C7F832281A39C5 = internal constant [8 x i8] zeroinitializer, align 1
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

define i64 @SpeculativeLargeStructWarning_x3a_x3aspeculativeLargeStructWarningReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"SpeculativeLargeStructWarning_x3a_x3aspeculativeLargeStructWarningReference$tmp$__uv_spec_snapshot_18" = alloca { [17 x i64], [0 x i64] }, align 8
  %coerce_bits3 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %"SpeculativeLargeStructWarning_x3a_x3aspeculativeLargeStructWarningReference$tmp$__uv_key_scope_152" = alloca ptr, align 8
  %"SpeculativeLargeStructWarning_x3a_x3aspeculativeLargeStructWarningReference$tmp$__uv_key_scope_15" = alloca ptr, align 8
  %aggregate.literal1 = alloca [17 x i64], align 8
  %aggregate.literal = alloca { [17 x i64], [0 x i64] }, align 8
  %payload = alloca { [17 x i64], [0 x i64] }, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aSpeculativeLargeStructWarning, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  call void @llvm.memset.p0.i64(ptr align 8 %aggregate.literal, i8 0, i64 136, i1 false)
  %10 = getelementptr [17 x i64], ptr %aggregate.literal1, i64 0, i64 0
  store i64 1, ptr %10, align 8
  %11 = getelementptr [17 x i64], ptr %aggregate.literal1, i64 0, i64 1
  store i64 1, ptr %11, align 8
  %12 = getelementptr [17 x i64], ptr %aggregate.literal1, i64 0, i64 2
  store i64 1, ptr %12, align 8
  %13 = getelementptr [17 x i64], ptr %aggregate.literal1, i64 0, i64 3
  store i64 1, ptr %13, align 8
  %14 = getelementptr [17 x i64], ptr %aggregate.literal1, i64 0, i64 4
  store i64 1, ptr %14, align 8
  %15 = getelementptr [17 x i64], ptr %aggregate.literal1, i64 0, i64 5
  store i64 1, ptr %15, align 8
  %16 = getelementptr [17 x i64], ptr %aggregate.literal1, i64 0, i64 6
  store i64 1, ptr %16, align 8
  %17 = getelementptr [17 x i64], ptr %aggregate.literal1, i64 0, i64 7
  store i64 1, ptr %17, align 8
  %18 = getelementptr [17 x i64], ptr %aggregate.literal1, i64 0, i64 8
  store i64 1, ptr %18, align 8
  %19 = getelementptr [17 x i64], ptr %aggregate.literal1, i64 0, i64 9
  store i64 1, ptr %19, align 8
  %20 = getelementptr [17 x i64], ptr %aggregate.literal1, i64 0, i64 10
  store i64 1, ptr %20, align 8
  %21 = getelementptr [17 x i64], ptr %aggregate.literal1, i64 0, i64 11
  store i64 1, ptr %21, align 8
  %22 = getelementptr [17 x i64], ptr %aggregate.literal1, i64 0, i64 12
  store i64 1, ptr %22, align 8
  %23 = getelementptr [17 x i64], ptr %aggregate.literal1, i64 0, i64 13
  store i64 1, ptr %23, align 8
  %24 = getelementptr [17 x i64], ptr %aggregate.literal1, i64 0, i64 14
  store i64 1, ptr %24, align 8
  %25 = getelementptr [17 x i64], ptr %aggregate.literal1, i64 0, i64 15
  store i64 1, ptr %25, align 8
  %26 = getelementptr [17 x i64], ptr %aggregate.literal1, i64 0, i64 16
  store i64 1, ptr %26, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %aggregate.literal, ptr align 8 %aggregate.literal1, i64 136, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %payload, ptr align 8 %aggregate.literal, i64 136, i1 false)
  %27 = call ptr @uv_key_scope_enter()
  store ptr %27, ptr %"SpeculativeLargeStructWarning_x3a_x3aspeculativeLargeStructWarningReference$tmp$__uv_key_scope_152", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fCFB8A9D063B5E9E5, i64 7 }, ptr %coerce_bits, align 8
  %28 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %28, i8 1)
  %29 = load ptr, ptr %"SpeculativeLargeStructWarning_x3a_x3aspeculativeLargeStructWarningReference$tmp$__uv_key_scope_152", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fCFB8A9D063B5E9E5, i64 7 }, ptr %coerce_bits3, align 8
  %30 = load { i64, i64 }, ptr %coerce_bits3, align 1
  call void @uv_key_acquire(ptr %29, { i64, i64 } %30, i8 1)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %"SpeculativeLargeStructWarning_x3a_x3aspeculativeLargeStructWarningReference$tmp$__uv_spec_snapshot_18", ptr align 8 %payload, i64 136, i1 false)
  %31 = getelementptr i8, ptr %payload, i64 0
  %32 = getelementptr i8, ptr %payload, i64 0
  %33 = getelementptr i64, ptr %32, i64 0
  %34 = getelementptr i8, ptr %payload, i64 0
  %35 = getelementptr [17 x i64], ptr %34, i64 0, i64 0
  store i64 2, ptr %35, align 4
  %36 = load ptr, ptr %__panic, align 8
  %37 = getelementptr i8, ptr %36, i64 0
  %38 = load i8, ptr %37, align 1
  %39 = load ptr, ptr %__panic, align 8
  %40 = getelementptr i8, ptr %39, i64 4
  %41 = load i32, ptr %40, align 4
  %42 = icmp ne i8 %38, 0
  %43 = icmp ne i8 %38, 0
  br i1 %43, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %payload, ptr align 8 %"SpeculativeLargeStructWarning_x3a_x3aspeculativeLargeStructWarningReference$tmp$__uv_spec_snapshot_18", i64 136, i1 false)
  br label %if.merge

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %44 = load ptr, ptr %"SpeculativeLargeStructWarning_x3a_x3aspeculativeLargeStructWarningReference$tmp$__uv_key_scope_152", align 8
  call void @uv_key_scope_exit(ptr %44)
  fence seq_cst
  %45 = getelementptr i8, ptr %payload, i64 0
  fence seq_cst
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %if.merge
  %46 = load ptr, ptr %__panic, align 8
  %47 = load i8, ptr %46, align 1
  %48 = icmp ne i8 %47, 0
  br i1 %48, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %if.merge
  %49 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %49, align 1
  %50 = getelementptr i8, ptr %49, i64 4
  store i32 4, ptr %50, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %51 = load ptr, ptr %__panic, align 8
  %52 = getelementptr i8, ptr %51, i64 0
  %53 = load i8, ptr %52, align 1
  %54 = load ptr, ptr %__panic, align 8
  %55 = getelementptr i8, ptr %54, i64 4
  %56 = load i32, ptr %55, align 4
  %57 = load ptr, ptr %__panic, align 8
  %58 = getelementptr i8, ptr %57, i64 4
  %59 = load i32, ptr %58, align 4
  %60 = zext i32 %59 to i64
  ret i64 %60

panic.cont:                                       ; preds = %check_ok
  %61 = getelementptr i8, ptr %payload, i64 0
  %62 = getelementptr i64, ptr %61, i64 0
  %63 = load i64, ptr %62, align 4
  %64 = call { i64, i1 } @llvm.sadd.with.overflow.i64(i64 %63, i64 0)
  %65 = extractvalue { i64, i1 } %64, 0
  %66 = extractvalue { i64, i1 } %64, 1
  %67 = freeze i64 %65
  br i1 %66, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  ret i64 %67

op_fail:                                          ; preds = %panic.cont
  %68 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %68, align 1
  %69 = getelementptr i8, ptr %68, i64 4
  store i32 4, ptr %69, align 4
  ret i64 0
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aSpeculativeLargeStructWarning(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aSpeculativeLargeStructWarning(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
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
declare { i64, i1 } @llvm.sadd.with.overflow.i64(i64, i64) #3

define hidden void @__cx_lifecycle_init_SpeculativeLargeStructWarning(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aSpeculativeLargeStructWarning(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_SpeculativeLargeStructWarning(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aSpeculativeLargeStructWarning(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aSpeculativeLargeStructWarning(ptr %dll_attach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aSpeculativeLargeStructWarning(ptr %dll_detach_panic_out)
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
