; ==== Source/PanicHandlingNoAdditionalDiagnostics.ll
; ModuleID = 'PanicHandlingNoAdditionalDiagnostics'
source_filename = "PanicHandlingNoAdditionalDiagnostics"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aPanicHandlingNoAdditionalDiagnostics = hidden global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fA8C7F832281A39C5 = internal constant [8 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2CDCDC0DFC5D1141 = internal constant [8 x i8] c"\04\00\00\00\00\00\00\00", align 1
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
declare ptr @uv_parallel_begin({ i64, i64 }, i64, ptr) #0

; Function Attrs: nounwind
declare i32 @uv_parallel_join(ptr) #0

; Function Attrs: nounwind
declare ptr @uv_spawn_create(ptr, i64, ptr, ptr, i64, i64, i32) #0

define i32 @PanicHandlingNoAdditionalDiagnostics_x3a_x3apanicHandlingNoAdditionalDiagnosticsReference(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %byref_arg = alloca i32, align 4
  %"PanicHandlingNoAdditionalDiagnostics_x3a_x3apanicHandlingNoAdditionalDiagnosticsReference$tmp$spawn_env_storage_5" = alloca {}, align 1
  %coerce_bits = alloca { ptr, ptr }, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aPanicHandlingNoAdditionalDiagnostics, align 1
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
  %9 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3acpu(ptr noundef nonnull align 8 dereferenceable(96) %context)
  store { i64, i64 } %9, ptr %abi_return, align 8
  %10 = load { ptr, ptr }, ptr %abi_return, align 1
  store { ptr, ptr } %10, ptr %coerce_bits, align 8
  %11 = load { i64, i64 }, ptr %coerce_bits, align 1
  %12 = call ptr @uv_parallel_begin({ i64, i64 } %11, i64 -1, ptr null)
  store {} zeroinitializer, ptr %"PanicHandlingNoAdditionalDiagnostics_x3a_x3apanicHandlingNoAdditionalDiagnosticsReference$tmp$spawn_env_storage_5", align 1
  %13 = call ptr @uv_spawn_create(ptr %"PanicHandlingNoAdditionalDiagnostics_x3a_x3apanicHandlingNoAdditionalDiagnosticsReference$tmp$spawn_env_storage_5", i64 0, ptr @__cx_spawn_body_PanicHandlingNoAdditionalDiagnostics_0, ptr null, i64 4, i64 0, i32 -1)
  %14 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %14, align 1
  %15 = getelementptr i8, ptr %14, i64 4
  store i32 0, ptr %15, align 4
  %16 = call i32 @uv_parallel_join(ptr %12)
  %17 = icmp eq i32 %16, 0
  %18 = xor i1 %17, true
  %19 = zext i1 %18 to i8
  %20 = icmp ne i8 %19, 0
  %21 = icmp ne i8 %19, 0
  br i1 %21, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  %22 = load ptr, ptr %__panic, align 8
  %23 = getelementptr i8, ptr %22, i64 0
  store i8 1, ptr %23, align 1
  %24 = load ptr, ptr %__panic, align 8
  %25 = getelementptr i8, ptr %24, i64 4
  store i32 %16, ptr %25, align 4
  br label %if.merge

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %if.result = phi i64 [ 0, %if.then ], [ 0, %if.else ]
  %26 = load ptr, ptr %__panic, align 8
  %27 = getelementptr i8, ptr %26, i64 0
  %28 = load i8, ptr %27, align 1
  %29 = load ptr, ptr %__panic, align 8
  %30 = getelementptr i8, ptr %29, i64 4
  %31 = load i32, ptr %30, align 4
  %32 = icmp ne i8 %28, 0
  %33 = zext i1 %32 to i8
  %34 = icmp ne i8 %33, 0
  %35 = and i1 false, %34
  br i1 %35, label %if.then1, label %if.else2

if.then1:                                         ; preds = %if.merge
  store i32 %31, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else2:                                         ; preds = %if.merge
  br label %if.merge3

if.merge3:                                        ; preds = %if.else2
  %36 = icmp ne i8 %28, 0
  %37 = xor i1 %36, true
  %38 = and i1 false, %37
  br i1 %38, label %if.then4, label %if.else5

if.then4:                                         ; preds = %if.merge3
  %39 = load ptr, ptr %__panic, align 8
  %40 = getelementptr i8, ptr %39, i64 0
  store i8 1, ptr %40, align 1
  %41 = load ptr, ptr %__panic, align 8
  %42 = getelementptr i8, ptr %41, i64 4
  store i32 0, ptr %42, align 4
  br label %if.merge6

if.else5:                                         ; preds = %if.merge3
  br label %if.merge6

if.merge6:                                        ; preds = %if.else5, %if.then4
  %if.result7 = phi i64 [ 0, %if.then4 ], [ 0, %if.else5 ]
  %43 = icmp ne i8 %28, 0
  %44 = zext i1 %43 to i8
  %45 = icmp ne i8 %44, 0
  %46 = or i1 false, %45
  %47 = icmp ne i8 %28, 0
  %48 = icmp ne i8 %28, 0
  br i1 %48, label %if.then8, label %if.else9

if.then8:                                         ; preds = %if.merge6
  br label %if.merge10

if.else9:                                         ; preds = %if.merge6
  br label %if.merge10

if.merge10:                                       ; preds = %if.else9, %if.then8
  %if.result11 = phi i32 [ %31, %if.then8 ], [ 0, %if.else9 ]
  %49 = load ptr, ptr %__panic, align 8
  %50 = load i8, ptr %49, align 1
  %51 = icmp ne i8 %50, 0
  br i1 %51, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %if.merge10
  %52 = load ptr, ptr %__panic, align 8
  %53 = getelementptr i8, ptr %52, i64 4
  %54 = load i32, ptr %53, align 4
  ret i32 %54

panic.cont:                                       ; preds = %if.merge10
  ret i32 0
}

define internal void @__cx_spawn_body_PanicHandlingNoAdditionalDiagnostics_0(ptr %__uv_host_env, ptr noundef nonnull align 1 %env, ptr noundef nonnull align 4 dereferenceable(4) %result, ptr %__panic) {
entry:
  %abi_return7 = alloca { i64, i64 }, align 8
  %abi_return6 = alloca { i64, i64 }, align 8
  %coerce_bits = alloca i64, align 8
  %pointer = alloca ptr, align 8
  %"region$05" = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %aggregate.literal = alloca { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] }, align 8
  %"region$0" = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %__panic4 = alloca ptr, align 8
  %result3 = alloca ptr, align 8
  %env2 = alloca ptr, align 8
  %__uv_host_env1 = alloca ptr, align 8
  store ptr %__uv_host_env, ptr %__uv_host_env1, align 8
  store ptr %env, ptr %env2, align 8
  store ptr %result, ptr %result3, align 8
  store ptr %__panic, ptr %__panic4, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aPanicHandlingNoAdditionalDiagnostics, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic4, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic4, align 8
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
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %8, ptr %"region$05", align 4
  store i64 0, ptr %coerce_bits, align 8
  %9 = load ptr, ptr %coerce_bits, align 1
  store ptr %9, ptr %pointer, align 8
  %10 = load ptr, ptr %pointer, align 8
  %11 = icmp ne ptr %10, null
  br i1 %11, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %12 = load ptr, ptr %__panic4, align 8
  %13 = load i8, ptr %12, align 1
  %14 = icmp ne i8 %13, 0
  br i1 %14, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %15 = load ptr, ptr %__panic4, align 8
  store i8 1, ptr %15, align 1
  %16 = getelementptr i8, ptr %15, i64 4
  store i32 8, ptr %16, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %17 = load ptr, ptr %__panic4, align 8
  %18 = getelementptr i8, ptr %17, i64 0
  %19 = load i8, ptr %18, align 1
  %20 = load ptr, ptr %__panic4, align 8
  %21 = getelementptr i8, ptr %20, i64 4
  %22 = load i32, ptr %21, align 4
  %23 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$05")
  store { i64, i64 } %23, ptr %abi_return6, align 8
  %24 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return6, align 1
  ret void

panic.cont:                                       ; preds = %check_ok
  %25 = load ptr, ptr %pointer, align 8
  %26 = load i32, ptr %25, align 4
  %27 = load ptr, ptr %result3, align 8
  store i32 %26, ptr %27, align 4
  %28 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$05")
  store { i64, i64 } %28, ptr %abi_return7, align 8
  %29 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return7, align 1
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aPanicHandlingNoAdditionalDiagnostics(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aPanicHandlingNoAdditionalDiagnostics(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #2

define hidden void @__cx_lifecycle_init_PanicHandlingNoAdditionalDiagnostics(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aPanicHandlingNoAdditionalDiagnostics(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_PanicHandlingNoAdditionalDiagnostics(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aPanicHandlingNoAdditionalDiagnostics(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aPanicHandlingNoAdditionalDiagnostics(ptr %dll_attach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aPanicHandlingNoAdditionalDiagnostics(ptr %dll_detach_panic_out)
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
