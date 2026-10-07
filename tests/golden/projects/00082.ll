; ==== Source/UnknownCalleeAccessWarning.ll
; ModuleID = 'UnknownCalleeAccessWarning'
source_filename = "UnknownCalleeAccessWarning"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aUnknownCalleeAccessWarning = hidden global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f16F3E46051EEE3E8 = internal constant [6 x i8] c"target", align 1
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

define i32 @UnknownCalleeAccessWarning_x3a_x3aUnknownAccessTarget_x3a_x3areadValue(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret i32 0
}

define i32 @UnknownCalleeAccessWarning_x3a_x3asummarizeAsUnknown(ptr noundef nonnull align 4 dereferenceable(4) %target, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %coerce_bits2 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %"UnknownCalleeAccessWarning_x3a_x3asummarizeAsUnknown$tmp$__uv_implicit_key_scope_31" = alloca ptr, align 8
  %"UnknownCalleeAccessWarning_x3a_x3asummarizeAsUnknown$tmp$__uv_implicit_key_scope_3" = alloca ptr, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aUnknownCalleeAccessWarning, align 1
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
  fence seq_cst
  %9 = call ptr @uv_key_scope_enter()
  store ptr %9, ptr %"UnknownCalleeAccessWarning_x3a_x3asummarizeAsUnknown$tmp$__uv_implicit_key_scope_31", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f16F3E46051EEE3E8, i64 6 }, ptr %coerce_bits, align 8
  %10 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %10, i8 0)
  %11 = load ptr, ptr %"UnknownCalleeAccessWarning_x3a_x3asummarizeAsUnknown$tmp$__uv_implicit_key_scope_31", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f16F3E46051EEE3E8, i64 6 }, ptr %coerce_bits2, align 8
  %12 = load { i64, i64 }, ptr %coerce_bits2, align 1
  call void @uv_key_acquire(ptr %11, { i64, i64 } %12, i8 0)
  %13 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aUnknownCalleeAccessWarning, align 1
  %14 = icmp ne i8 %13, 0
  br i1 %14, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont
  %15 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %15, align 1
  %16 = getelementptr i8, ptr %15, i64 4
  store i32 10, ptr %16, align 4
  %17 = load ptr, ptr %__panic, align 8
  %18 = getelementptr i8, ptr %17, i64 4
  %19 = load i32, ptr %18, align 4
  ret i32 %19

poison.cont4:                                     ; preds = %poison.cont
  %20 = call i32 @UnknownCalleeAccessWarning_x3a_x3aUnknownAccessTarget_x3a_x3areadValue(ptr noundef nonnull align 4 dereferenceable(4) %target, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %21 = load ptr, ptr %__panic, align 8
  %22 = load i8, ptr %21, align 1
  %23 = icmp ne i8 %22, 0
  br i1 %23, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  %24 = load ptr, ptr %__panic, align 8
  %25 = getelementptr i8, ptr %24, i64 0
  %26 = load i8, ptr %25, align 1
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 4
  %29 = load i32, ptr %28, align 4
  %30 = load ptr, ptr %"UnknownCalleeAccessWarning_x3a_x3asummarizeAsUnknown$tmp$__uv_implicit_key_scope_31", align 8
  call void @uv_key_scope_exit(ptr %30)
  %31 = load ptr, ptr %__panic, align 8
  %32 = getelementptr i8, ptr %31, i64 4
  %33 = load i32, ptr %32, align 4
  ret i32 %33

panic.cont:                                       ; preds = %poison.cont4
  fence seq_cst
  %34 = load ptr, ptr %"UnknownCalleeAccessWarning_x3a_x3asummarizeAsUnknown$tmp$__uv_implicit_key_scope_31", align 8
  call void @uv_key_scope_exit(ptr %34)
  ret i32 %20
}

define i32 @UnknownCalleeAccessWarning_x3a_x3aunknownCalleeAccessWarningReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %observed7 = alloca i32, align 4
  %observed = alloca i32, align 4
  %coerce_bits2 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %"UnknownCalleeAccessWarning_x3a_x3aunknownCalleeAccessWarningReference$tmp$__uv_key_scope_281" = alloca ptr, align 8
  %"UnknownCalleeAccessWarning_x3a_x3aunknownCalleeAccessWarningReference$tmp$__uv_key_scope_28" = alloca ptr, align 8
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %target = alloca { i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aUnknownCalleeAccessWarning, align 1
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
  store i32 1, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %target, ptr align 4 %aggregate.literal, i64 4, i1 false)
  %9 = call ptr @uv_key_scope_enter()
  store ptr %9, ptr %"UnknownCalleeAccessWarning_x3a_x3aunknownCalleeAccessWarningReference$tmp$__uv_key_scope_281", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f16F3E46051EEE3E8, i64 6 }, ptr %coerce_bits, align 8
  %10 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %10, i8 0)
  %11 = load ptr, ptr %"UnknownCalleeAccessWarning_x3a_x3aunknownCalleeAccessWarningReference$tmp$__uv_key_scope_281", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f16F3E46051EEE3E8, i64 6 }, ptr %coerce_bits2, align 8
  %12 = load { i64, i64 }, ptr %coerce_bits2, align 1
  call void @uv_key_acquire(ptr %11, { i64, i64 } %12, i8 0)
  %13 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aUnknownCalleeAccessWarning, align 1
  %14 = icmp ne i8 %13, 0
  br i1 %14, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont
  %15 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %15, align 1
  %16 = getelementptr i8, ptr %15, i64 4
  store i32 10, ptr %16, align 4
  %17 = load ptr, ptr %__panic, align 8
  %18 = getelementptr i8, ptr %17, i64 4
  %19 = load i32, ptr %18, align 4
  ret i32 %19

poison.cont4:                                     ; preds = %poison.cont
  %20 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aUnknownCalleeAccessWarning, align 1
  %21 = icmp ne i8 %20, 0
  br i1 %21, label %poison.take5, label %poison.cont6

poison.take5:                                     ; preds = %poison.cont4
  %22 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %22, align 1
  %23 = getelementptr i8, ptr %22, i64 4
  store i32 10, ptr %23, align 4
  %24 = load ptr, ptr %__panic, align 8
  %25 = getelementptr i8, ptr %24, i64 4
  %26 = load i32, ptr %25, align 4
  ret i32 %26

poison.cont6:                                     ; preds = %poison.cont4
  %27 = call i32 @UnknownCalleeAccessWarning_x3a_x3asummarizeAsUnknown(ptr noundef nonnull align 4 dereferenceable(4) %target, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %28 = load ptr, ptr %__panic, align 8
  %29 = load i8, ptr %28, align 1
  %30 = icmp ne i8 %29, 0
  br i1 %30, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont6
  %31 = load ptr, ptr %__panic, align 8
  %32 = getelementptr i8, ptr %31, i64 0
  %33 = load i8, ptr %32, align 1
  %34 = load ptr, ptr %__panic, align 8
  %35 = getelementptr i8, ptr %34, i64 4
  %36 = load i32, ptr %35, align 4
  %37 = load ptr, ptr %"UnknownCalleeAccessWarning_x3a_x3aunknownCalleeAccessWarningReference$tmp$__uv_key_scope_281", align 8
  call void @uv_key_scope_exit(ptr %37)
  %38 = load ptr, ptr %__panic, align 8
  %39 = getelementptr i8, ptr %38, i64 4
  %40 = load i32, ptr %39, align 4
  ret i32 %40

panic.cont:                                       ; preds = %poison.cont6
  store i32 %27, ptr %observed7, align 4
  %41 = load ptr, ptr %"UnknownCalleeAccessWarning_x3a_x3aunknownCalleeAccessWarningReference$tmp$__uv_key_scope_281", align 8
  call void @uv_key_scope_exit(ptr %41)
  ret i32 0
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aUnknownCalleeAccessWarning(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aUnknownCalleeAccessWarning(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
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

define hidden void @__cx_lifecycle_init_UnknownCalleeAccessWarning(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aUnknownCalleeAccessWarning(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_UnknownCalleeAccessWarning(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aUnknownCalleeAccessWarning(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aUnknownCalleeAccessWarning(ptr %dll_attach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aUnknownCalleeAccessWarning(ptr %dll_detach_panic_out)
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
