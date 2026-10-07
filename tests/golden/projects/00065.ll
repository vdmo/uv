; ==== Source/InlineAlwaysRecursive.ll
; ModuleID = 'InlineAlwaysRecursive'
source_filename = "InlineAlwaysRecursive"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aInlineAlwaysRecursive = hidden global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAD2ACA7747985764 = internal constant [4 x i8] c"\01\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f8D1ACE904A398D17 = internal constant [4 x i8] c"\02\00\00\00", align 1
@__uv_library_attached = internal global i1 false
@__uv_image_panic_record = common hidden global { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, align 4
@llvm.global_ctors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_ctor, ptr null }]
@llvm.global_dtors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_dtor, ptr null }]

; Function Attrs: alwaysinline
define i32 @InlineAlwaysRecursive_x3a_x3ainlineAlwaysRecursive(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) #0 {
entry:
  %"InlineAlwaysRecursive_x3a_x3ainlineAlwaysRecursive$tmp$call_ref_tmp_11" = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aInlineAlwaysRecursive, align 1
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
  %9 = load i32, ptr %value, align 4
  %10 = icmp sle i32 %9, 0
  %11 = zext i1 %10 to i8
  %12 = icmp ne i8 %11, 0
  %13 = icmp ne i8 %11, 0
  br i1 %13, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  ret i32 0

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else
  %14 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aInlineAlwaysRecursive, align 1
  %15 = icmp ne i8 %14, 0
  br i1 %15, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %if.merge
  %16 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %16, align 1
  %17 = getelementptr i8, ptr %16, i64 4
  store i32 10, ptr %17, align 4
  %18 = load ptr, ptr %__panic, align 8
  %19 = getelementptr i8, ptr %18, i64 4
  %20 = load i32, ptr %19, align 4
  ret i32 %20

poison.cont2:                                     ; preds = %if.merge
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont2
  %21 = load ptr, ptr %__panic, align 8
  %22 = load i8, ptr %21, align 1
  %23 = icmp ne i8 %22, 0
  br i1 %23, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont2
  %24 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %24, align 1
  %25 = getelementptr i8, ptr %24, i64 4
  store i32 4, ptr %25, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %26 = load ptr, ptr %__panic, align 8
  %27 = getelementptr i8, ptr %26, i64 4
  %28 = load i32, ptr %27, align 4
  ret i32 %28

panic.cont:                                       ; preds = %check_ok
  %29 = load i32, ptr %value, align 4
  %30 = call { i32, i1 } @llvm.ssub.with.overflow.i32(i32 %29, i32 1)
  %31 = extractvalue { i32, i1 } %30, 0
  %32 = extractvalue { i32, i1 } %30, 1
  %33 = freeze i32 %31
  br i1 %32, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  store i32 %33, ptr %"InlineAlwaysRecursive_x3a_x3ainlineAlwaysRecursive$tmp$call_ref_tmp_11", align 4
  %34 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aInlineAlwaysRecursive, align 1
  %35 = icmp ne i8 %34, 0
  br i1 %35, label %poison.take3, label %poison.cont4

op_fail:                                          ; preds = %panic.cont
  %36 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %36, align 1
  %37 = getelementptr i8, ptr %36, i64 4
  store i32 4, ptr %37, align 4
  ret i32 0

poison.take3:                                     ; preds = %op_ok
  %38 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %38, align 1
  %39 = getelementptr i8, ptr %38, i64 4
  store i32 10, ptr %39, align 4
  %40 = load ptr, ptr %__panic, align 8
  %41 = getelementptr i8, ptr %40, i64 4
  %42 = load i32, ptr %41, align 4
  ret i32 %42

poison.cont4:                                     ; preds = %op_ok
  %43 = call i32 @InlineAlwaysRecursive_x3a_x3ainlineAlwaysRecursive(ptr noundef nonnull align 4 dereferenceable(4) %"InlineAlwaysRecursive_x3a_x3ainlineAlwaysRecursive$tmp$call_ref_tmp_11", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %44 = load ptr, ptr %__panic, align 8
  %45 = load i8, ptr %44, align 1
  %46 = icmp ne i8 %45, 0
  br i1 %46, label %panic.take5, label %panic.cont6

panic.take5:                                      ; preds = %poison.cont4
  %47 = load ptr, ptr %__panic, align 8
  %48 = getelementptr i8, ptr %47, i64 4
  %49 = load i32, ptr %48, align 4
  ret i32 %49

panic.cont6:                                      ; preds = %poison.cont4
  ret i32 %43
}

define i32 @InlineAlwaysRecursive_x3a_x3acallInlineAlwaysRecursive(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"InlineAlwaysRecursive_x3a_x3acallInlineAlwaysRecursive$tmp$call_ref_tmp_16" = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aInlineAlwaysRecursive, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aInlineAlwaysRecursive, align 1
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
  store i32 2, ptr %"InlineAlwaysRecursive_x3a_x3acallInlineAlwaysRecursive$tmp$call_ref_tmp_16", align 4
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aInlineAlwaysRecursive, align 1
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
  %23 = call i32 @InlineAlwaysRecursive_x3a_x3ainlineAlwaysRecursive(ptr noundef nonnull align 4 dereferenceable(4) %"InlineAlwaysRecursive_x3a_x3acallInlineAlwaysRecursive$tmp$call_ref_tmp_16", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
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

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aInlineAlwaysRecursive(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aInlineAlwaysRecursive(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.ssub.with.overflow.i32(i32, i32) #1

define hidden void @__cx_lifecycle_init_InlineAlwaysRecursive(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aInlineAlwaysRecursive(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_InlineAlwaysRecursive(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aInlineAlwaysRecursive(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aInlineAlwaysRecursive(ptr %dll_attach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aInlineAlwaysRecursive(ptr %dll_detach_panic_out)
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

attributes #0 = { alwaysinline }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
