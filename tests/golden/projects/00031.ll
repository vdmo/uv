; ==== Source/ExternAbiForms.ll
; ModuleID = 'ExternAbiForms'
source_filename = "ExternAbiForms"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExternAbiForms = hidden global i8 0
@ExternAbiForms_x3a_x3aEXTERN_x5fABI_x5fFORMS_x5fSTATIC = hidden global i32 0, align 4
@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExternAbiForms_x3a_x3aSupport = external global i8
@__uv_library_attached = internal global i1 false
@__uv_image_panic_record = common hidden global { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, align 4
@llvm.global_ctors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_ctor, ptr null }]
@llvm.global_dtors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_dtor, ptr null }]

declare i32 @externAbiDefaultValue()

declare i32 @externAbiStringValue()

declare i32 @externAbiIdentValue()

define i32 @ExternAbiForms_x3a_x3aexternAbiFormsMarker(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExternAbiForms, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExternAbiForms, align 1
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
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExternAbiForms_x3a_x3aSupport, align 1
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
  %23 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExternAbiForms_x3a_x3aSupport, align 1
  %24 = icmp ne i8 %23, 0
  br i1 %24, label %poison.take5, label %poison.cont6

poison.take5:                                     ; preds = %poison.cont4
  %25 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %25, align 1
  %26 = getelementptr i8, ptr %25, i64 4
  store i32 10, ptr %26, align 4
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 4
  %29 = load i32, ptr %28, align 4
  ret i32 %29

poison.cont6:                                     ; preds = %poison.cont4
  %30 = call i32 @ExternAbiForms_x3a_x3aSupport_x3a_x3asupportUsingValue(ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %31 = load ptr, ptr %__panic, align 8
  %32 = load i8, ptr %31, align 1
  %33 = icmp ne i8 %32, 0
  br i1 %33, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont6
  %34 = load ptr, ptr %__panic, align 8
  %35 = getelementptr i8, ptr %34, i64 4
  %36 = load i32, ptr %35, align 4
  ret i32 %36

panic.cont:                                       ; preds = %poison.cont6
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %panic.cont
  %37 = load ptr, ptr %__panic, align 8
  %38 = load i8, ptr %37, align 1
  %39 = icmp ne i8 %38, 0
  br i1 %39, label %panic.take7, label %panic.cont8

check_fail:                                       ; preds = %panic.cont
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
  %45 = load i32, ptr @ExternAbiForms_x3a_x3aEXTERN_x5fABI_x5fFORMS_x5fSTATIC, align 4
  %46 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %45, i32 %30)
  %47 = extractvalue { i32, i1 } %46, 0
  %48 = extractvalue { i32, i1 } %46, 1
  %49 = freeze i32 %47
  br i1 %48, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont8
  ret i32 %49

op_fail:                                          ; preds = %panic.cont8
  %50 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %50, align 1
  %51 = getelementptr i8, ptr %50, i64 4
  store i32 4, ptr %51, align 4
  ret i32 0
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aExternAbiForms(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  %2 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExternAbiForms_x3a_x3aSupport, align 1
  %3 = icmp ne i8 %2, 0
  br i1 %3, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 10, ptr %5, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %6 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExternAbiForms_x3a_x3aSupport, align 1
  %7 = icmp ne i8 %6, 0
  br i1 %7, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %8 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 10, ptr %9, align 4
  ret void

poison.cont2:                                     ; preds = %poison.cont
  %10 = call i32 @ExternAbiForms_x3a_x3aSupport_x3a_x3asupportValue(ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %11 = load ptr, ptr %__panic, align 8
  %12 = load i8, ptr %11, align 1
  %13 = icmp ne i8 %12, 0
  br i1 %13, label %init.panic.take, label %init.panic.cont

init.panic.take:                                  ; preds = %poison.cont2
  store i8 1, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExternAbiForms, align 1
  store i8 1, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExternAbiForms_x3a_x3aSupport, align 1
  %14 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %14, align 1
  %15 = getelementptr i8, ptr %14, i64 4
  store i32 10, ptr %15, align 4
  ret void

init.panic.cont:                                  ; preds = %poison.cont2
  store i32 %10, ptr @ExternAbiForms_x3a_x3aEXTERN_x5fABI_x5fFORMS_x5fSTATIC, align 4
  %16 = load ptr, ptr %__panic, align 8
  %17 = load i8, ptr %16, align 1
  %18 = icmp ne i8 %17, 0
  br i1 %18, label %init.panic.take3, label %init.panic.cont4

init.panic.take3:                                 ; preds = %init.panic.cont
  store i8 1, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExternAbiForms, align 1
  store i8 1, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExternAbiForms_x3a_x3aSupport, align 1
  %19 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %19, align 1
  %20 = getelementptr i8, ptr %19, i64 4
  store i32 10, ptr %20, align 4
  ret void

init.panic.cont4:                                 ; preds = %init.panic.cont
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aExternAbiForms(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

declare i32 @ExternAbiForms_x3a_x3aSupport_x3a_x3asupportUsingValue(ptr)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #0

declare i32 @ExternAbiForms_x3a_x3aSupport_x3a_x3asupportValue(ptr)

define hidden void @__cx_lifecycle_init_ExternAbiForms(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aExternAbiForms(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_ExternAbiForms(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aExternAbiForms(ptr %0)
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
  call void @__cx_lifecycle_init_ExternAbiForms_x3a_x3aSupport(ptr %dll_attach_panic_out)
  %4 = load i8, ptr @__uv_image_panic_record, align 1
  %5 = icmp ne i8 %4, 0
  br i1 %5, label %dll.attach.fail, label %dll.attach.cont

dll.attach.done:                                  ; preds = %dll.attach
  ret i32 1

dll.attach.cont:                                  ; preds = %dll.attach.work
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aExternAbiForms(ptr %dll_attach_panic_out)
  %6 = load i8, ptr @__uv_image_panic_record, align 1
  %7 = icmp ne i8 %6, 0
  br i1 %7, label %dll.attach.fail2, label %dll.attach.cont1

dll.attach.fail:                                  ; preds = %dll.attach.work
  store i8 0, ptr @__uv_image_panic_record, align 1
  store i32 0, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  store i1 false, ptr @__uv_library_attached, align 1
  ret i32 0

dll.attach.cont1:                                 ; preds = %dll.attach.cont
  store i1 true, ptr @__uv_library_attached, align 1
  ret i32 1

dll.attach.fail2:                                 ; preds = %dll.attach.cont
  store i8 0, ptr @__uv_image_panic_record, align 1
  store i32 0, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  call void @__cx_lifecycle_deinit_ExternAbiForms_x3a_x3aSupport(ptr %dll_attach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aExternAbiForms(ptr %dll_detach_panic_out)
  %8 = load i8, ptr @__uv_image_panic_record, align 1
  %9 = icmp ne i8 %8, 0
  br i1 %9, label %dll.detach.panic.capture, label %dll.detach.cont

dll.detach.done:                                  ; preds = %dll.detach
  ret i32 1

dll.detach.panic.capture:                         ; preds = %dll.detach.work
  %10 = load i1, ptr %dll_detach_panic_seen, align 1
  %11 = load i32, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  br i1 %10, label %dll.detach.panic.clear, label %dll.detach.panic.store

dll.detach.cont:                                  ; preds = %dll.detach.panic.clear, %dll.detach.work
  call void @__cx_lifecycle_deinit_ExternAbiForms_x3a_x3aSupport(ptr %dll_detach_panic_out)
  %12 = load i8, ptr @__uv_image_panic_record, align 1
  %13 = icmp ne i8 %12, 0
  br i1 %13, label %dll.detach.panic.capture3, label %dll.detach.cont4

dll.detach.panic.store:                           ; preds = %dll.detach.panic.capture
  store i1 true, ptr %dll_detach_panic_seen, align 1
  store i32 %11, ptr %dll_detach_panic_code, align 4
  br label %dll.detach.panic.clear

dll.detach.panic.clear:                           ; preds = %dll.detach.panic.store, %dll.detach.panic.capture
  store i8 0, ptr @__uv_image_panic_record, align 1
  store i32 0, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  br label %dll.detach.cont

dll.detach.panic.capture3:                        ; preds = %dll.detach.cont
  %14 = load i1, ptr %dll_detach_panic_seen, align 1
  %15 = load i32, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  br i1 %14, label %dll.detach.panic.clear6, label %dll.detach.panic.store5

dll.detach.cont4:                                 ; preds = %dll.detach.panic.clear6, %dll.detach.cont
  store i1 false, ptr @__uv_library_attached, align 1
  %16 = load i1, ptr %dll_detach_panic_seen, align 1
  br i1 %16, label %dll.detach.fail, label %dll.detach.success

dll.detach.panic.store5:                          ; preds = %dll.detach.panic.capture3
  store i1 true, ptr %dll_detach_panic_seen, align 1
  store i32 %15, ptr %dll_detach_panic_code, align 4
  br label %dll.detach.panic.clear6

dll.detach.panic.clear6:                          ; preds = %dll.detach.panic.store5, %dll.detach.panic.capture3
  store i8 0, ptr @__uv_image_panic_record, align 1
  store i32 0, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  br label %dll.detach.cont4

dll.detach.fail:                                  ; preds = %dll.detach.cont4
  %17 = load i32, ptr %dll_detach_panic_code, align 4
  store i8 1, ptr @__uv_image_panic_record, align 1
  store i32 %17, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  ret i32 0

dll.detach.success:                               ; preds = %dll.detach.cont4
  ret i32 1
}

declare void @__cx_lifecycle_init_ExternAbiForms_x3a_x3aSupport(ptr)

declare void @__cx_lifecycle_deinit_ExternAbiForms_x3a_x3aSupport(ptr)

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
; ==== Source/Support/ExternAbiForms_x3a_x3aSupport.ll
; ModuleID = 'ExternAbiForms::Support'
source_filename = "ExternAbiForms::Support"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExternAbiForms_x3a_x3aSupport = hidden global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fACD58AFCAAF42074 = internal constant [4 x i8] c"\11\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f6CE032EBFE88A752 = internal constant [4 x i8] c"\17\00\00\00", align 1

define i32 @ExternAbiForms_x3a_x3aSupport_x3a_x3asupportValue(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExternAbiForms_x3a_x3aSupport, align 1
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
  ret i32 17
}

define i32 @ExternAbiForms_x3a_x3aSupport_x3a_x3asupportUsingValue(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExternAbiForms_x3a_x3aSupport, align 1
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
  ret i32 23
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aExternAbiForms_x3a_x3aSupport(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aExternAbiForms_x3a_x3aSupport(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define hidden void @__cx_lifecycle_init_ExternAbiForms_x3a_x3aSupport(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aExternAbiForms_x3a_x3aSupport(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_ExternAbiForms_x3a_x3aSupport(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aExternAbiForms_x3a_x3aSupport(ptr %0)
  ret void
}
