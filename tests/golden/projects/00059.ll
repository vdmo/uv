; ==== Source/FreeProcedureOverloadResolution.ll
; ModuleID = 'FreeProcedureOverloadResolution'
source_filename = "FreeProcedureOverloadResolution"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aFreeProcedureOverloadResolution = hidden global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAD2ACA7747985764 = internal constant [4 x i8] c"\01\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f8D1ACE904A398D17 = internal constant [4 x i8] c"\02\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fCD3AC65E44F721B1 = internal constant [4 x i8] c"\04\00\00\00", align 1
@__uv_library_attached = internal global i1 false
@__uv_image_panic_record = common hidden global { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, align 4
@llvm.global_ctors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_ctor, ptr null }]
@llvm.global_dtors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_dtor, ptr null }]

define i32 @FreeProcedureOverloadResolution_x3a_x3aselectOverload_x3a_x3a_x24overload_x3a_x3a0(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aFreeProcedureOverloadResolution, align 1
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
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %9 = load ptr, ptr %__panic, align 8
  %10 = load i8, ptr %9, align 1
  %11 = icmp ne i8 %10, 0
  br i1 %11, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %12 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %12, align 1
  %13 = getelementptr i8, ptr %12, i64 4
  store i32 4, ptr %13, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %14 = load ptr, ptr %__panic, align 8
  %15 = getelementptr i8, ptr %14, i64 4
  %16 = load i32, ptr %15, align 4
  ret i32 %16

panic.cont:                                       ; preds = %check_ok
  %17 = load i32, ptr %value, align 4
  %18 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %17, i32 1)
  %19 = extractvalue { i32, i1 } %18, 0
  %20 = extractvalue { i32, i1 } %18, 1
  %21 = freeze i32 %19
  br i1 %20, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  ret i32 %21

op_fail:                                          ; preds = %panic.cont
  %22 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %22, align 1
  %23 = getelementptr i8, ptr %22, i64 4
  store i32 4, ptr %23, align 4
  ret i32 0
}

define i32 @FreeProcedureOverloadResolution_x3a_x3aselectOverload_x3a_x3a_x24overload_x3a_x3a1(ptr noundef nonnull align 1 dereferenceable(1) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aFreeProcedureOverloadResolution, align 1
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
  %9 = load i8, ptr %value, align 1
  %10 = icmp ne i8 %9, 0
  %11 = load i8, ptr %value, align 1
  %12 = icmp ne i8 %11, 0
  br i1 %12, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  ret i32 2

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else
  ret i32 0
}

define i32 @FreeProcedureOverloadResolution_x3a_x3afreeProcedureOverloadResolutionReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"FreeProcedureOverloadResolution_x3a_x3afreeProcedureOverloadResolutionReference$tmp$call_ref_tmp_14" = alloca i8, align 1
  %"FreeProcedureOverloadResolution_x3a_x3afreeProcedureOverloadResolutionReference$tmp$call_ref_tmp_10" = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aFreeProcedureOverloadResolution, align 1
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
  store i32 4, ptr %"FreeProcedureOverloadResolution_x3a_x3afreeProcedureOverloadResolutionReference$tmp$call_ref_tmp_10", align 4
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aFreeProcedureOverloadResolution, align 1
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
  %16 = call i32 @FreeProcedureOverloadResolution_x3a_x3aselectOverload_x3a_x3a_x24overload_x3a_x3a0(ptr noundef nonnull align 4 dereferenceable(4) %"FreeProcedureOverloadResolution_x3a_x3afreeProcedureOverloadResolutionReference$tmp$call_ref_tmp_10", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %17 = load ptr, ptr %__panic, align 8
  %18 = load i8, ptr %17, align 1
  %19 = icmp ne i8 %18, 0
  br i1 %19, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont2
  %20 = load ptr, ptr %__panic, align 8
  %21 = getelementptr i8, ptr %20, i64 4
  %22 = load i32, ptr %21, align 4
  ret i32 %22

panic.cont:                                       ; preds = %poison.cont2
  store i8 1, ptr %"FreeProcedureOverloadResolution_x3a_x3afreeProcedureOverloadResolutionReference$tmp$call_ref_tmp_14", align 1
  %23 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aFreeProcedureOverloadResolution, align 1
  %24 = icmp ne i8 %23, 0
  br i1 %24, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %panic.cont
  %25 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %25, align 1
  %26 = getelementptr i8, ptr %25, i64 4
  store i32 10, ptr %26, align 4
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 4
  %29 = load i32, ptr %28, align 4
  ret i32 %29

poison.cont4:                                     ; preds = %panic.cont
  %30 = call i32 @FreeProcedureOverloadResolution_x3a_x3aselectOverload_x3a_x3a_x24overload_x3a_x3a1(ptr noundef nonnull align 1 dereferenceable(1) %"FreeProcedureOverloadResolution_x3a_x3afreeProcedureOverloadResolutionReference$tmp$call_ref_tmp_14", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %31 = load ptr, ptr %__panic, align 8
  %32 = load i8, ptr %31, align 1
  %33 = icmp ne i8 %32, 0
  br i1 %33, label %panic.take5, label %panic.cont6

panic.take5:                                      ; preds = %poison.cont4
  %34 = load ptr, ptr %__panic, align 8
  %35 = getelementptr i8, ptr %34, i64 4
  %36 = load i32, ptr %35, align 4
  ret i32 %36

panic.cont6:                                      ; preds = %poison.cont4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %panic.cont6
  %37 = load ptr, ptr %__panic, align 8
  %38 = load i8, ptr %37, align 1
  %39 = icmp ne i8 %38, 0
  br i1 %39, label %panic.take7, label %panic.cont8

check_fail:                                       ; preds = %panic.cont6
  %40 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %40, align 1
  %41 = getelementptr i8, ptr %40, i64 4
  store i32 4, ptr %41, align 4
  br label %check_ok

panic.take7:                                      ; preds = %check_ok
  %42 = load ptr, ptr %__panic, align 8
  %43 = getelementptr i8, ptr %42, i64 4
  %44 = load i32, ptr %43, align 4
  ret i32 %44

panic.cont8:                                      ; preds = %check_ok
  %45 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %16, i32 %30)
  %46 = extractvalue { i32, i1 } %45, 0
  %47 = extractvalue { i32, i1 } %45, 1
  %48 = freeze i32 %46
  br i1 %47, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont8
  ret i32 %48

op_fail:                                          ; preds = %panic.cont8
  %49 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %49, align 1
  %50 = getelementptr i8, ptr %49, i64 4
  store i32 4, ptr %50, align 4
  ret i32 0
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aFreeProcedureOverloadResolution(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aFreeProcedureOverloadResolution(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #0

define hidden void @__cx_lifecycle_init_FreeProcedureOverloadResolution(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aFreeProcedureOverloadResolution(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_FreeProcedureOverloadResolution(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aFreeProcedureOverloadResolution(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aFreeProcedureOverloadResolution(ptr %dll_attach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aFreeProcedureOverloadResolution(ptr %dll_detach_panic_out)
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

attributes #0 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
