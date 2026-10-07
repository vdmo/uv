; ==== Source/CancellationNoAdditionalDiagnostics.ll
; ModuleID = 'CancellationNoAdditionalDiagnostics'
source_filename = "CancellationNoAdditionalDiagnostics"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aCancellationNoAdditionalDiagnostics = hidden global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAD2ACA7747985764 = internal constant [4 x i8] c"\01\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@__uv_library_attached = internal global i1 false
@__uv_image_panic_record = common hidden global { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, align 4
@llvm.global_ctors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_ctor, ptr null }]
@llvm.global_dtors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_dtor, ptr null }]

; Function Attrs: nounwind
declare i8 @CancelToken_x3a_x3aActive_x3a_x3ais_x5fcancelled(ptr) #0

; Function Attrs: nounwind
declare i64 @CancelToken_x3a_x3anew() #0

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3acpu(ptr noundef nonnull align 8 dereferenceable(96)) #0

; Function Attrs: noreturn nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4)) #1

; Function Attrs: nounwind
declare ptr @uv_parallel_begin({ i64, i64 }, i64, ptr) #0

; Function Attrs: nounwind
declare i32 @uv_parallel_join(ptr) #0

define i32 @CancellationNoAdditionalDiagnostics_x3a_x3acancellationNoAdditionalDiagnosticsReference(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %byref_arg = alloca i32, align 4
  %coerce_bits2 = alloca { ptr, ptr }, align 8
  %coerce_bits = alloca { i64, [0 x i64] }, align 8
  %abi_return1 = alloca { i64, i64 }, align 8
  %token = alloca { i64, [0 x i64] }, align 8
  %abi_return = alloca i64, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aCancellationNoAdditionalDiagnostics, align 1
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
  %9 = call i64 @CancelToken_x3a_x3anew()
  store i64 %9, ptr %abi_return, align 8
  %10 = load { i64, [0 x i64] }, ptr %abi_return, align 1
  store { i64, [0 x i64] } %10, ptr %token, align 4
  %11 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3acpu(ptr noundef nonnull align 8 dereferenceable(96) %context)
  store { i64, i64 } %11, ptr %abi_return1, align 8
  %12 = load { ptr, ptr }, ptr %abi_return1, align 1
  %13 = load { i64, [0 x i64] }, ptr %token, align 4
  store { i64, [0 x i64] } %13, ptr %coerce_bits, align 8
  %14 = load i64, ptr %coerce_bits, align 1
  store { ptr, ptr } %12, ptr %coerce_bits2, align 8
  %15 = load { i64, i64 }, ptr %coerce_bits2, align 1
  %16 = call ptr @uv_parallel_begin({ i64, i64 } %15, i64 %14, ptr null)
  %17 = call i8 @CancelToken_x3a_x3aActive_x3a_x3ais_x5fcancelled(ptr %token)
  %18 = trunc i8 %17 to i1
  br i1 %18, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  br label %if.merge

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %"CancellationNoAdditionalDiagnostics_x3a_x3acancellationNoAdditionalDiagnosticsReference$tmp$if_8" = phi i32 [ 1, %if.then ], [ 0, %if.else ]
  %19 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %19, align 1
  %20 = getelementptr i8, ptr %19, i64 4
  store i32 0, ptr %20, align 4
  %21 = call i32 @uv_parallel_join(ptr %16)
  %22 = icmp eq i32 %21, 0
  %23 = xor i1 %22, true
  %24 = zext i1 %23 to i8
  %25 = icmp ne i8 %24, 0
  %26 = icmp ne i8 %24, 0
  br i1 %26, label %if.then3, label %if.else4

if.then3:                                         ; preds = %if.merge
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 0
  store i8 1, ptr %28, align 1
  %29 = load ptr, ptr %__panic, align 8
  %30 = getelementptr i8, ptr %29, i64 4
  store i32 %21, ptr %30, align 4
  br label %if.merge5

if.else4:                                         ; preds = %if.merge
  br label %if.merge5

if.merge5:                                        ; preds = %if.else4, %if.then3
  %if.result = phi i64 [ 0, %if.then3 ], [ 0, %if.else4 ]
  %31 = load ptr, ptr %__panic, align 8
  %32 = getelementptr i8, ptr %31, i64 0
  %33 = load i8, ptr %32, align 1
  %34 = load ptr, ptr %__panic, align 8
  %35 = getelementptr i8, ptr %34, i64 4
  %36 = load i32, ptr %35, align 4
  %37 = icmp ne i8 %33, 0
  %38 = zext i1 %37 to i8
  %39 = icmp ne i8 %38, 0
  %40 = and i1 false, %39
  br i1 %40, label %if.then6, label %if.else7

if.then6:                                         ; preds = %if.merge5
  store i32 %36, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else7:                                         ; preds = %if.merge5
  br label %if.merge8

if.merge8:                                        ; preds = %if.else7
  %41 = icmp ne i8 %33, 0
  %42 = xor i1 %41, true
  %43 = and i1 false, %42
  br i1 %43, label %if.then9, label %if.else10

if.then9:                                         ; preds = %if.merge8
  %44 = load ptr, ptr %__panic, align 8
  %45 = getelementptr i8, ptr %44, i64 0
  store i8 1, ptr %45, align 1
  %46 = load ptr, ptr %__panic, align 8
  %47 = getelementptr i8, ptr %46, i64 4
  store i32 0, ptr %47, align 4
  br label %if.merge11

if.else10:                                        ; preds = %if.merge8
  br label %if.merge11

if.merge11:                                       ; preds = %if.else10, %if.then9
  %if.result12 = phi i64 [ 0, %if.then9 ], [ 0, %if.else10 ]
  %48 = icmp ne i8 %33, 0
  %49 = zext i1 %48 to i8
  %50 = icmp ne i8 %49, 0
  %51 = or i1 false, %50
  %52 = icmp ne i8 %33, 0
  %53 = icmp ne i8 %33, 0
  br i1 %53, label %if.then13, label %if.else14

if.then13:                                        ; preds = %if.merge11
  br label %if.merge15

if.else14:                                        ; preds = %if.merge11
  br label %if.merge15

if.merge15:                                       ; preds = %if.else14, %if.then13
  %if.result16 = phi i32 [ %36, %if.then13 ], [ 0, %if.else14 ]
  %54 = load ptr, ptr %__panic, align 8
  %55 = load i8, ptr %54, align 1
  %56 = icmp ne i8 %55, 0
  br i1 %56, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %if.merge15
  %57 = load ptr, ptr %__panic, align 8
  %58 = getelementptr i8, ptr %57, i64 4
  %59 = load i32, ptr %58, align 4
  ret i32 %59

panic.cont:                                       ; preds = %if.merge15
  ret i32 %"CancellationNoAdditionalDiagnostics_x3a_x3acancellationNoAdditionalDiagnosticsReference$tmp$if_8"
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aCancellationNoAdditionalDiagnostics(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aCancellationNoAdditionalDiagnostics(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define hidden void @__cx_lifecycle_init_CancellationNoAdditionalDiagnostics(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aCancellationNoAdditionalDiagnostics(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_CancellationNoAdditionalDiagnostics(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aCancellationNoAdditionalDiagnostics(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aCancellationNoAdditionalDiagnostics(ptr %dll_attach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aCancellationNoAdditionalDiagnostics(ptr %dll_detach_panic_out)
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
