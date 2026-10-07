; ==== Source/MacOSHostedSharedLibrary.ll
; ModuleID = 'MacOSHostedSharedLibrary'
source_filename = "MacOSHostedSharedLibrary"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aMacOSHostedSharedLibrary = hidden global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAD2ACA7747985764 = internal constant [4 x i8] c"\01\00\00\00", align 1
@__uv_library_attached = internal global i1 false
@__uv_image_panic_record = common hidden global { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, align 4
@llvm.global_ctors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_ctor, ptr null }]
@llvm.global_dtors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_dtor, ptr null }]
@__uv_host_session_owner_token = hidden global i8 0

define hidden i32 @MacOSHostedSharedLibrary_x5fx3a_x5fx3amacOSHostedReferenceIncrement_x3a_x3a_x5f_x5fhost_x5fbody(ptr noundef nonnull align 8 dereferenceable(16) %context, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = call ptr @uv_host_session_current_env()
  %1 = icmp ne ptr %0, null
  br i1 %1, label %hosted.state.session, label %hosted.state.fallback

hosted.state.session:                             ; preds = %entry
  %2 = getelementptr i8, ptr %0, i64 104
  br label %hosted.state.merge

hosted.state.fallback:                            ; preds = %entry
  br label %hosted.state.merge

hosted.state.merge:                               ; preds = %hosted.state.fallback, %hosted.state.session
  %3 = phi ptr [ %2, %hosted.state.session ], [ @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aMacOSHostedSharedLibrary, %hosted.state.fallback ]
  %4 = load i8, ptr %3, align 1
  %5 = icmp ne i8 %4, 0
  br i1 %5, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %hosted.state.merge
  %6 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %6, align 1
  %7 = getelementptr i8, ptr %6, i64 4
  store i32 10, ptr %7, align 4
  %8 = load ptr, ptr %__panic, align 8
  %9 = getelementptr i8, ptr %8, i64 4
  %10 = load i32, ptr %9, align 4
  ret i32 %10

poison.cont:                                      ; preds = %hosted.state.merge
  %11 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %11, align 1
  %12 = getelementptr i8, ptr %11, i64 4
  store i32 0, ptr %12, align 4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %13 = load ptr, ptr %__panic, align 8
  %14 = load i8, ptr %13, align 1
  %15 = icmp ne i8 %14, 0
  br i1 %15, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %16 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %16, align 1
  %17 = getelementptr i8, ptr %16, i64 4
  store i32 4, ptr %17, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %18 = load ptr, ptr %__panic, align 8
  %19 = getelementptr i8, ptr %18, i64 4
  %20 = load i32, ptr %19, align 4
  ret i32 %20

panic.cont:                                       ; preds = %check_ok
  %21 = load i32, ptr %value, align 4
  %22 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %21, i32 1)
  %23 = extractvalue { i32, i1 } %22, 0
  %24 = extractvalue { i32, i1 } %22, 1
  %25 = freeze i32 %23
  br i1 %24, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  ret i32 %25

op_fail:                                          ; preds = %panic.cont
  %26 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %26, align 1
  %27 = getelementptr i8, ptr %26, i64 4
  store i32 4, ptr %27, align 4
  ret i32 0
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aMacOSHostedSharedLibrary(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aMacOSHostedSharedLibrary(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

declare ptr @uv_host_session_current_env()

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #0

define hidden void @__cx_lifecycle_init_MacOSHostedSharedLibrary(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aMacOSHostedSharedLibrary(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_MacOSHostedSharedLibrary(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aMacOSHostedSharedLibrary(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aMacOSHostedSharedLibrary(ptr %dll_attach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aMacOSHostedSharedLibrary(ptr %dll_detach_panic_out)
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

define i32 @__ultraviolet_host_abi_version() {
entry:
  ret i32 1
}

define i64 @__ultraviolet_host_session_create() {
entry:
  %0 = call ptr @uv_host_alloc(i64 112)
  %1 = icmp ne ptr %0, null
  br i1 %1, label %host.create.alloc.ok, label %host.create.alloc.fail

host.create.alloc.ok:                             ; preds = %entry
  call void @llvm.memset.p0.i64(ptr align 1 %0, i8 0, i64 112, i1 false)
  call void @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x5finit(ptr %0)
  %host_env = alloca ptr, align 8
  store ptr null, ptr %host_env, align 8
  %2 = call i64 @uv_host_session_register(ptr @__uv_host_session_owner_token, ptr %0)
  %3 = icmp ne i64 %2, 0
  br i1 %3, label %host.create.register.ok, label %host.create.register.fail

host.create.alloc.fail:                           ; preds = %entry
  ret i64 0

host.create.register.ok:                          ; preds = %host.create.alloc.ok
  %4 = call i32 @uv_host_session_try_enter(i64 %2, ptr @__uv_host_session_owner_token, ptr %host_env)
  %5 = icmp ne i32 %4, 0
  br i1 %5, label %host.create.enter.ok, label %host.create.enter.fail

host.create.register.fail:                        ; preds = %host.create.alloc.ok
  call void @uv_host_free(ptr %0)
  ret i64 0

host.create.enter.ok:                             ; preds = %host.create.register.ok
  %6 = getelementptr i8, ptr %0, i64 96
  %host_create_panic_out = alloca ptr, align 8
  store ptr %6, ptr %host_create_panic_out, align 8
  store i8 0, ptr %6, align 1
  %7 = getelementptr i8, ptr %6, i64 4
  store i32 0, ptr %7, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aMacOSHostedSharedLibrary(ptr %host_create_panic_out)
  %8 = load i8, ptr %6, align 1
  %9 = icmp ne i8 %8, 0
  br i1 %9, label %host.create.init.fail, label %host.create.init.cont

host.create.enter.fail:                           ; preds = %host.create.register.ok
  store ptr null, ptr %host_env, align 8
  %10 = call i32 @uv_host_session_try_retire(i64 %2, ptr @__uv_host_session_owner_token, ptr %host_env)
  %11 = icmp ne i32 %10, 0
  br i1 %11, label %host.create.retire.ok, label %host.create.retire.fail

host.create.retire.ok:                            ; preds = %host.create.enter.fail
  %12 = load ptr, ptr %host_env, align 8
  %13 = icmp ne ptr %12, null
  %14 = select i1 %13, ptr %12, ptr %0
  call void @uv_host_free(ptr %14)
  ret i64 0

host.create.retire.fail:                          ; preds = %host.create.enter.fail
  store ptr null, ptr %host_env, align 8
  %15 = call i32 @uv_host_session_abort_live(i64 %2, ptr @__uv_host_session_owner_token, ptr %host_env)
  %16 = icmp ne i32 %15, 0
  br i1 %16, label %host.create.abort.ok, label %host.create.abort.fail

host.create.abort.ok:                             ; preds = %host.create.retire.fail
  %17 = load ptr, ptr %host_env, align 8
  %18 = icmp ne ptr %17, null
  %19 = select i1 %18, ptr %17, ptr %0
  call void @uv_host_free(ptr %19)
  ret i64 0

host.create.abort.fail:                           ; preds = %host.create.retire.fail
  ret i64 0

host.create.init.cont:                            ; preds = %host.create.enter.ok
  %20 = call i32 @uv_host_session_leave(i64 %2, ptr @__uv_host_session_owner_token)
  %21 = icmp ne i32 %20, 0
  br i1 %21, label %host.create.leave.ok7, label %host.create.leave.fail8

host.create.init.fail:                            ; preds = %host.create.enter.ok
  store i8 0, ptr %6, align 1
  %22 = getelementptr i8, ptr %6, i64 4
  store i32 0, ptr %22, align 4
  %23 = call i32 @uv_host_session_leave(i64 %2, ptr @__uv_host_session_owner_token)
  %24 = icmp ne i32 %23, 0
  br i1 %24, label %host.create.leave.ok, label %host.create.leave.fail

host.create.leave.ok:                             ; preds = %host.create.init.fail
  store ptr null, ptr %host_env, align 8
  %25 = call i32 @uv_host_session_try_retire(i64 %2, ptr @__uv_host_session_owner_token, ptr %host_env)
  %26 = icmp ne i32 %25, 0
  br i1 %26, label %host.create.retire.ok3, label %host.create.retire.fail4

host.create.leave.fail:                           ; preds = %host.create.init.fail
  store ptr null, ptr %host_env, align 8
  %27 = call i32 @uv_host_session_abort_live(i64 %2, ptr @__uv_host_session_owner_token, ptr %host_env)
  %28 = icmp ne i32 %27, 0
  br i1 %28, label %host.create.abort.ok1, label %host.create.abort.fail2

host.create.abort.ok1:                            ; preds = %host.create.leave.fail
  %29 = load ptr, ptr %host_env, align 8
  %30 = icmp ne ptr %29, null
  %31 = select i1 %30, ptr %29, ptr %0
  call void @uv_host_free(ptr %31)
  ret i64 0

host.create.abort.fail2:                          ; preds = %host.create.leave.fail
  ret i64 0

host.create.retire.ok3:                           ; preds = %host.create.leave.ok
  %32 = load ptr, ptr %host_env, align 8
  %33 = icmp ne ptr %32, null
  %34 = select i1 %33, ptr %32, ptr %0
  call void @uv_host_free(ptr %34)
  ret i64 0

host.create.retire.fail4:                         ; preds = %host.create.leave.ok
  store ptr null, ptr %host_env, align 8
  %35 = call i32 @uv_host_session_abort_live(i64 %2, ptr @__uv_host_session_owner_token, ptr %host_env)
  %36 = icmp ne i32 %35, 0
  br i1 %36, label %host.create.abort.ok5, label %host.create.abort.fail6

host.create.abort.ok5:                            ; preds = %host.create.retire.fail4
  %37 = load ptr, ptr %host_env, align 8
  %38 = icmp ne ptr %37, null
  %39 = select i1 %38, ptr %37, ptr %0
  call void @uv_host_free(ptr %39)
  ret i64 0

host.create.abort.fail6:                          ; preds = %host.create.retire.fail4
  ret i64 0

host.create.leave.ok7:                            ; preds = %host.create.init.cont
  ret i64 %2

host.create.leave.fail8:                          ; preds = %host.create.init.cont
  store ptr null, ptr %host_env, align 8
  %40 = call i32 @uv_host_session_abort_live(i64 %2, ptr @__uv_host_session_owner_token, ptr %host_env)
  %41 = icmp ne i32 %40, 0
  br i1 %41, label %host.create.abort.ok9, label %host.create.abort.fail10

host.create.abort.ok9:                            ; preds = %host.create.leave.fail8
  %42 = load ptr, ptr %host_env, align 8
  %43 = icmp ne ptr %42, null
  %44 = select i1 %43, ptr %42, ptr %0
  call void @uv_host_free(ptr %44)
  ret i64 0

host.create.abort.fail10:                         ; preds = %host.create.leave.fail8
  ret i64 0
}

declare ptr @uv_host_alloc(i64)

declare void @uv_host_free(ptr)

declare i64 @uv_host_session_register(ptr, ptr)

declare i32 @uv_host_session_try_enter(i64, ptr, ptr)

declare i32 @uv_host_session_leave(i64, ptr)

declare i32 @uv_host_session_try_retire(i64, ptr, ptr)

declare i32 @uv_host_session_abort_live(i64, ptr, ptr)

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #1

declare void @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x5finit(ptr)

define i32 @__ultraviolet_host_session_destroy(i64 %0) {
entry:
  %host_env = alloca ptr, align 8
  store ptr null, ptr %host_env, align 8
  %1 = call i32 @uv_host_session_try_retire(i64 %0, ptr @__uv_host_session_owner_token, ptr %host_env)
  %2 = icmp ne i32 %1, 0
  br i1 %2, label %host.destroy.retire.ok, label %host.destroy.retire.fail

host.destroy.retire.ok:                           ; preds = %entry
  %3 = load ptr, ptr %host_env, align 8
  %4 = call i32 @uv_host_session_enter_retired(i64 %0, ptr @__uv_host_session_owner_token, ptr %3)
  %5 = icmp ne i32 %4, 0
  br i1 %5, label %host.destroy.enter.ok, label %host.destroy.enter.fail

host.destroy.retire.fail:                         ; preds = %entry
  ret i32 0

host.destroy.enter.ok:                            ; preds = %host.destroy.retire.ok
  %6 = getelementptr i8, ptr %3, i64 96
  %host_destroy_panic_out = alloca ptr, align 8
  store ptr %6, ptr %host_destroy_panic_out, align 8
  store i8 0, ptr %6, align 1
  %7 = getelementptr i8, ptr %6, i64 4
  store i32 0, ptr %7, align 4
  %8 = alloca i1, align 1
  store i1 false, ptr %8, align 1
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aMacOSHostedSharedLibrary(ptr %host_destroy_panic_out)
  %9 = load i8, ptr %6, align 1
  %10 = icmp ne i8 %9, 0
  br i1 %10, label %host.destroy.deinit.fail, label %host.destroy.deinit.cont

host.destroy.enter.fail:                          ; preds = %host.destroy.retire.ok
  %11 = call i32 @uv_host_session_abort_retired(i64 %0, ptr @__uv_host_session_owner_token, ptr %3)
  %12 = icmp ne i32 %11, 0
  br i1 %12, label %host.destroy.enter.abort.ok, label %host.destroy.enter.abort.fail

host.destroy.enter.abort.ok:                      ; preds = %host.destroy.enter.fail
  call void @uv_host_free(ptr %3)
  ret i32 0

host.destroy.enter.abort.fail:                    ; preds = %host.destroy.enter.fail
  call void @uv_host_free(ptr %3)
  ret i32 0

host.destroy.deinit.fail:                         ; preds = %host.destroy.enter.ok
  store i1 true, ptr %8, align 1
  br label %host.destroy.deinit.done

host.destroy.deinit.done:                         ; preds = %host.destroy.deinit.fail, %host.destroy.deinit.cont
  %13 = load i1, ptr %8, align 1
  %14 = call i32 @uv_host_session_leave_retired(i64 %0, ptr @__uv_host_session_owner_token)
  %15 = icmp ne i32 %14, 0
  br i1 %15, label %host.destroy.leave.ok, label %host.destroy.leave.fail

host.destroy.deinit.cont:                         ; preds = %host.destroy.enter.ok
  br label %host.destroy.deinit.done

host.destroy.leave.ok:                            ; preds = %host.destroy.deinit.done
  call void @uv_host_free(ptr %3)
  %16 = select i1 %13, i32 0, i32 1
  ret i32 %16

host.destroy.leave.fail:                          ; preds = %host.destroy.deinit.done
  %17 = call i32 @uv_host_session_abort_retired(i64 %0, ptr @__uv_host_session_owner_token, ptr %3)
  %18 = icmp ne i32 %17, 0
  br i1 %18, label %host.destroy.abort.ok, label %host.destroy.abort.fail

host.destroy.abort.ok:                            ; preds = %host.destroy.leave.fail
  call void @uv_host_free(ptr %3)
  ret i32 0

host.destroy.abort.fail:                          ; preds = %host.destroy.leave.fail
  ret i32 0
}

declare i32 @uv_host_session_enter_retired(i64, ptr, ptr)

declare i32 @uv_host_session_leave_retired(i64, ptr)

declare i32 @uv_host_session_abort_retired(i64, ptr, ptr)

define i32 @uv_macos_hosted_reference_increment(i64 %__ultraviolet_session, i32 %value) {
entry:
  %host_panic_code4 = alloca i32, align 4
  %host_panic_code3 = alloca i32, align 4
  %host_arg = alloca ptr, align 8
  %host_ctx = alloca { { ptr, ptr }, [0 x i64] }, align 8
  %host_ctx_bundle = alloca { { ptr, ptr }, [0 x i64] }, align 8
  %host_panic_code = alloca i32, align 4
  %host_env = alloca ptr, align 8
  %value2 = alloca i32, align 4
  %__ultraviolet_session1 = alloca i64, align 8
  store i64 %__ultraviolet_session, ptr %__ultraviolet_session1, align 4
  store i32 %value, ptr %value2, align 4
  store ptr null, ptr %host_env, align 8
  %0 = call i32 @uv_host_session_try_enter(i64 %__ultraviolet_session, ptr @__uv_host_session_owner_token, ptr %host_env)
  %1 = icmp ne i32 %0, 0
  br i1 %1, label %host.enter.ok, label %host.enter.reject

host.enter.ok:                                    ; preds = %entry
  %2 = load ptr, ptr %host_env, align 8
  %3 = load { { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, [0 x i64] }, ptr %2, align 8
  %4 = getelementptr i8, ptr %2, i64 96
  store i8 0, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 0, ptr %5, align 4
  store { { ptr, ptr }, [0 x i64] } zeroinitializer, ptr %host_ctx_bundle, align 8
  %6 = getelementptr i8, ptr %2, i64 48
  %7 = load { ptr, ptr }, ptr %6, align 8
  %8 = getelementptr i8, ptr %host_ctx_bundle, i64 0
  store { ptr, ptr } %7, ptr %8, align 8
  %9 = load { { ptr, ptr }, [0 x i64] }, ptr %host_ctx_bundle, align 8
  store { { ptr, ptr }, [0 x i64] } %9, ptr %host_ctx, align 8
  store ptr %4, ptr %host_arg, align 8
  %10 = call i32 @MacOSHostedSharedLibrary_x5fx3a_x5fx3amacOSHostedReferenceIncrement_x3a_x3a_x5f_x5fhost_x5fbody(ptr %host_ctx, ptr %value2, ptr %host_arg)
  %11 = load i8, ptr %4, align 1
  %12 = icmp ne i8 %11, 0
  %13 = getelementptr i8, ptr %4, i64 4
  %14 = load i32, ptr %13, align 4
  %15 = call i32 @uv_host_session_leave(i64 %__ultraviolet_session, ptr @__uv_host_session_owner_token)
  %16 = icmp ne i32 %15, 0
  br i1 %16, label %host.call.leave.ok, label %host.call.leave.fail

host.enter.reject:                                ; preds = %entry
  store i32 14, ptr %host_panic_code, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %host_panic_code)
  unreachable

host.call.leave.ok:                               ; preds = %host.enter.ok
  br i1 %12, label %host.call.panic, label %host.call.ok

host.call.leave.fail:                             ; preds = %host.enter.ok
  store i32 255, ptr %host_panic_code3, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %host_panic_code3)
  unreachable

host.call.panic:                                  ; preds = %host.call.leave.ok
  store i32 %14, ptr %host_panic_code4, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %host_panic_code4)
  unreachable

host.call.ok:                                     ; preds = %host.call.leave.ok
  ret i32 %10
}

attributes #0 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: write) }
