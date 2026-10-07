; ==== Source/AsyncStateMachineDiagnosticsTable.ll
; ModuleID = 'AsyncStateMachineDiagnosticsTable'
source_filename = "AsyncStateMachineDiagnosticsTable"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aAsyncStateMachineDiagnosticsTable = hidden global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAD2ACA7747985764 = internal constant [4 x i8] c"\01\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@__uv_library_attached = internal global i1 false
@__uv_image_panic_record = common hidden global { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, align 4
@llvm.global_ctors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_ctor, ptr null }]
@llvm.global_dtors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_dtor, ptr null }]

define void @AsyncStateMachineDiagnosticsTable_x3a_x3aacceptsLargeAsyncPayload(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %0, ptr noundef nonnull align 8 dereferenceable(320) %payload, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %2 = alloca i32, align 4
  %3 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aAsyncStateMachineDiagnosticsTable, align 1
  %4 = icmp ne i8 %3, 0
  br i1 %4, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %5 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 10, ptr %6, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %7 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %7, align 1
  %8 = getelementptr i8, ptr %7, i64 4
  store i32 0, ptr %8, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %9 = getelementptr i8, ptr %1, i64 8
  store i32 1, ptr %2, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %9, ptr align 1 %2, i64 4, i1 false)
  %10 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %10, ptr %0, align 4
  ret void
}

define internal void @"AsyncStateMachineDiagnosticsTable_x3a_x3aacceptsLargeAsyncPayload$resume"(ptr %__uv_host_env, ptr %__uv_async_out, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_frame, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_input, ptr %__panic) {
entry:
  %0 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %1 = alloca i32, align 4
  %__panic5 = alloca ptr, align 8
  %__uv_async_input4 = alloca ptr, align 8
  %__uv_async_frame3 = alloca ptr, align 8
  %__uv_async_out2 = alloca ptr, align 8
  %__uv_host_env1 = alloca ptr, align 8
  store ptr %__uv_host_env, ptr %__uv_host_env1, align 8
  store ptr %__uv_async_out, ptr %__uv_async_out2, align 8
  store ptr %__uv_async_frame, ptr %__uv_async_frame3, align 8
  store ptr %__uv_async_input, ptr %__uv_async_input4, align 8
  store ptr %__panic, ptr %__panic5, align 8
  %2 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aAsyncStateMachineDiagnosticsTable, align 1
  %3 = icmp ne i8 %2, 0
  br i1 %3, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 10, ptr %5, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %6 = load ptr, ptr %__panic5, align 8
  store i8 0, ptr %6, align 1
  %7 = getelementptr i8, ptr %6, i64 4
  store i32 0, ptr %7, align 4
  %8 = load ptr, ptr %__uv_async_frame3, align 8
  %9 = load ptr, ptr %__uv_async_input4, align 8
  %10 = getelementptr i8, ptr %8, i64 0
  %11 = load i64, ptr %10, align 4
  switch i64 %11, label %async.resume.invalid [
  ]

async.resume.invalid:                             ; preds = %poison.cont
  ret void

async.resume.start:                               ; No predecessors!
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %12 = getelementptr i8, ptr %0, i64 8
  store i32 1, ptr %1, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %12, ptr align 1 %1, i64 4, i1 false)
  %13 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %0, align 4
  %14 = load ptr, ptr %__uv_async_out2, align 8
  %15 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %13, ptr %15, align 4
  ret void
}

define i32 @AsyncStateMachineDiagnosticsTable_x3a_x3aasyncStateMachineDiagnosticsTableReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %operation = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %aggregate.literal1 = alloca [40 x i64], align 8
  %aggregate.literal = alloca { [40 x i64], [0 x i64] }, align 8
  %payload = alloca { [40 x i64], [0 x i64] }, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aAsyncStateMachineDiagnosticsTable, align 1
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
  call void @llvm.memset.p0.i64(ptr align 8 %aggregate.literal, i8 0, i64 320, i1 false)
  br label %array.repeat.fill

array.repeat.fill:                                ; preds = %array.repeat.fill, %poison.cont
  %array.repeat.index = phi i64 [ 0, %poison.cont ], [ %12, %array.repeat.fill ]
  %9 = getelementptr [40 x i64], ptr %aggregate.literal1, i64 0, i64 %array.repeat.index
  store i64 3, ptr %9, align 8
  %10 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %array.repeat.index, i64 1)
  %11 = extractvalue { i64, i1 } %10, 0
  %12 = freeze i64 %11
  %13 = icmp eq i64 %12, 40
  br i1 %13, label %array.repeat.done, label %array.repeat.fill

array.repeat.done:                                ; preds = %array.repeat.fill
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %aggregate.literal, ptr align 8 %aggregate.literal1, i64 320, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %payload, ptr align 8 %aggregate.literal, i64 320, i1 false)
  %14 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aAsyncStateMachineDiagnosticsTable, align 1
  %15 = icmp ne i8 %14, 0
  br i1 %15, label %poison.take2, label %poison.cont3

poison.take2:                                     ; preds = %array.repeat.done
  %16 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %16, align 1
  %17 = getelementptr i8, ptr %16, i64 4
  store i32 10, ptr %17, align 4
  %18 = load ptr, ptr %__panic, align 8
  %19 = getelementptr i8, ptr %18, i64 4
  %20 = load i32, ptr %19, align 4
  ret i32 %20

poison.cont3:                                     ; preds = %array.repeat.done
  %21 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aAsyncStateMachineDiagnosticsTable, align 1
  %22 = icmp ne i8 %21, 0
  br i1 %22, label %poison.take4, label %poison.cont5

poison.take4:                                     ; preds = %poison.cont3
  %23 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %23, align 1
  %24 = getelementptr i8, ptr %23, i64 4
  store i32 10, ptr %24, align 4
  %25 = load ptr, ptr %__panic, align 8
  %26 = getelementptr i8, ptr %25, i64 4
  %27 = load i32, ptr %26, align 4
  ret i32 %27

poison.cont5:                                     ; preds = %poison.cont3
  call void @AsyncStateMachineDiagnosticsTable_x3a_x3aacceptsLargeAsyncPayload(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %operation, ptr noundef nonnull align 8 dereferenceable(320) %payload, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %28 = load ptr, ptr %__panic, align 8
  %29 = load i8, ptr %28, align 1
  %30 = icmp ne i8 %29, 0
  br i1 %30, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont5
  %31 = load ptr, ptr %__panic, align 8
  %32 = getelementptr i8, ptr %31, i64 0
  %33 = load i8, ptr %32, align 1
  %34 = load ptr, ptr %__panic, align 8
  %35 = getelementptr i8, ptr %34, i64 4
  %36 = load i32, ptr %35, align 4
  %37 = load ptr, ptr %__panic, align 8
  %38 = getelementptr i8, ptr %37, i64 4
  %39 = load i32, ptr %38, align 4
  ret i32 %39

panic.cont:                                       ; preds = %poison.cont5
  ret i32 0
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aAsyncStateMachineDiagnosticsTable(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aAsyncStateMachineDiagnosticsTable(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64) #2

define hidden void @__cx_lifecycle_init_AsyncStateMachineDiagnosticsTable(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aAsyncStateMachineDiagnosticsTable(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_AsyncStateMachineDiagnosticsTable(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aAsyncStateMachineDiagnosticsTable(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aAsyncStateMachineDiagnosticsTable(ptr %dll_attach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aAsyncStateMachineDiagnosticsTable(ptr %dll_detach_panic_out)
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

attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
