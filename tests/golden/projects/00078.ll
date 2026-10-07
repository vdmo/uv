; ==== Source/SpeculativeExpensiveBodyWarning.ll
; ModuleID = 'SpeculativeExpensiveBodyWarning'
source_filename = "SpeculativeExpensiveBodyWarning"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aSpeculativeExpensiveBodyWarning = hidden global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAD2ACA7747985764 = internal constant [4 x i8] c"\01\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f77976C7416517C63 = internal constant [7 x i8] c"counter", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAF63BC4C8601B62C = internal constant [1 x i8] c"\01", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAF63BD4C8601B7DF = internal constant [1 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
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

define hidden i32 @SpeculativeExpensiveBodyWarning_x3a_x3aSpeculativeDiagnosticCounter_x3a_x3anextValue(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %entry
  %2 = load ptr, ptr %__panic, align 8
  %3 = load i8, ptr %2, align 1
  %4 = icmp ne i8 %3, 0
  br i1 %4, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %entry
  %5 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 4, ptr %6, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %7 = load ptr, ptr %__panic, align 8
  %8 = getelementptr i8, ptr %7, i64 4
  %9 = load i32, ptr %8, align 4
  ret i32 %9

panic.cont:                                       ; preds = %check_ok
  %10 = getelementptr i8, ptr %self, i64 0
  %11 = load i32, ptr %10, align 1
  %12 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %11, i32 1)
  %13 = extractvalue { i32, i1 } %12, 0
  %14 = extractvalue { i32, i1 } %12, 1
  %15 = freeze i32 %13
  br i1 %14, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  ret i32 %15

op_fail:                                          ; preds = %panic.cont
  %16 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %16, align 1
  %17 = getelementptr i8, ptr %16, i64 4
  store i32 4, ptr %17, align 4
  ret i32 0
}

define i32 @SpeculativeExpensiveBodyWarning_x3a_x3aspeculativeExpensiveBodyWarningReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %coerce_bits11 = alloca { ptr, i64 }, align 8
  %coerce_bits10 = alloca { ptr, i64 }, align 8
  %"SpeculativeExpensiveBodyWarning_x3a_x3aspeculativeExpensiveBodyWarningReference$tmp$__uv_implicit_key_scope_639" = alloca ptr, align 8
  %"SpeculativeExpensiveBodyWarning_x3a_x3aspeculativeExpensiveBodyWarningReference$tmp$__uv_implicit_key_scope_63" = alloca ptr, align 8
  %next_value5 = alloca i32, align 4
  %next_value = alloca i32, align 4
  %"SpeculativeExpensiveBodyWarning_x3a_x3aspeculativeExpensiveBodyWarningReference$tmp$__uv_spec_snapshot_29" = alloca { i32, [0 x i32] }, align 4
  %coerce_bits2 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %"SpeculativeExpensiveBodyWarning_x3a_x3aspeculativeExpensiveBodyWarningReference$tmp$__uv_key_scope_261" = alloca ptr, align 8
  %"SpeculativeExpensiveBodyWarning_x3a_x3aspeculativeExpensiveBodyWarningReference$tmp$__uv_key_scope_26" = alloca ptr, align 8
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %counter = alloca { i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aSpeculativeExpensiveBodyWarning, align 1
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
  store i32 5, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %counter, ptr align 4 %aggregate.literal, i64 4, i1 false)
  %9 = call ptr @uv_key_scope_enter()
  store ptr %9, ptr %"SpeculativeExpensiveBodyWarning_x3a_x3aspeculativeExpensiveBodyWarningReference$tmp$__uv_key_scope_261", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f77976C7416517C63, i64 7 }, ptr %coerce_bits, align 8
  %10 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %10, i8 1)
  %11 = load ptr, ptr %"SpeculativeExpensiveBodyWarning_x3a_x3aspeculativeExpensiveBodyWarningReference$tmp$__uv_key_scope_261", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f77976C7416517C63, i64 7 }, ptr %coerce_bits2, align 8
  %12 = load { i64, i64 }, ptr %coerce_bits2, align 1
  call void @uv_key_acquire(ptr %11, { i64, i64 } %12, i8 1)
  %13 = load { i32, [0 x i32] }, ptr %counter, align 4
  store { i32, [0 x i32] } %13, ptr %"SpeculativeExpensiveBodyWarning_x3a_x3aspeculativeExpensiveBodyWarningReference$tmp$__uv_spec_snapshot_29", align 4
  fence seq_cst
  %14 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aSpeculativeExpensiveBodyWarning, align 1
  %15 = icmp ne i8 %14, 0
  br i1 %15, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont
  %16 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %16, align 1
  %17 = getelementptr i8, ptr %16, i64 4
  store i32 10, ptr %17, align 4
  %18 = load ptr, ptr %__panic, align 8
  %19 = getelementptr i8, ptr %18, i64 4
  %20 = load i32, ptr %19, align 4
  ret i32 %20

poison.cont4:                                     ; preds = %poison.cont
  %21 = call i32 @SpeculativeExpensiveBodyWarning_x3a_x3aSpeculativeDiagnosticCounter_x3a_x3anextValue(ptr noundef nonnull align 4 dereferenceable(4) %counter, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %22 = load ptr, ptr %__panic, align 8
  %23 = load i8, ptr %22, align 1
  %24 = icmp ne i8 %23, 0
  br i1 %24, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  %25 = load ptr, ptr %__panic, align 8
  %26 = getelementptr i8, ptr %25, i64 0
  %27 = load i8, ptr %26, align 1
  %28 = load ptr, ptr %__panic, align 8
  %29 = getelementptr i8, ptr %28, i64 4
  %30 = load i32, ptr %29, align 4
  %31 = load ptr, ptr %__panic, align 8
  %32 = getelementptr i8, ptr %31, i64 0
  %33 = load i8, ptr %32, align 1
  %34 = load ptr, ptr %__panic, align 8
  %35 = getelementptr i8, ptr %34, i64 4
  %36 = load i32, ptr %35, align 4
  %37 = icmp ne i8 %33, 0
  %38 = icmp ne i8 %33, 0
  br i1 %38, label %if.then, label %if.else

panic.cont:                                       ; preds = %poison.cont4
  fence seq_cst
  store i32 %21, ptr %next_value5, align 4
  %39 = getelementptr i8, ptr %counter, i64 0
  %40 = getelementptr i8, ptr %counter, i64 0
  %41 = load i32, ptr %next_value5, align 4
  store i32 %41, ptr %40, align 4
  %42 = load ptr, ptr %__panic, align 8
  %43 = getelementptr i8, ptr %42, i64 0
  %44 = load i8, ptr %43, align 1
  %45 = load ptr, ptr %__panic, align 8
  %46 = getelementptr i8, ptr %45, i64 4
  %47 = load i32, ptr %46, align 4
  %48 = icmp ne i8 %44, 0
  %49 = icmp ne i8 %44, 0
  br i1 %49, label %if.then6, label %if.else7

if.then:                                          ; preds = %panic.take
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %counter, ptr align 4 %"SpeculativeExpensiveBodyWarning_x3a_x3aspeculativeExpensiveBodyWarningReference$tmp$__uv_spec_snapshot_29", i64 4, i1 false)
  br label %if.merge

if.else:                                          ; preds = %panic.take
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %50 = load ptr, ptr %"SpeculativeExpensiveBodyWarning_x3a_x3aspeculativeExpensiveBodyWarningReference$tmp$__uv_key_scope_261", align 8
  call void @uv_key_scope_exit(ptr %50)
  %51 = load ptr, ptr %__panic, align 8
  %52 = getelementptr i8, ptr %51, i64 4
  %53 = load i32, ptr %52, align 4
  ret i32 %53

if.then6:                                         ; preds = %panic.cont
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %counter, ptr align 4 %"SpeculativeExpensiveBodyWarning_x3a_x3aspeculativeExpensiveBodyWarningReference$tmp$__uv_spec_snapshot_29", i64 4, i1 false)
  br label %if.merge8

if.else7:                                         ; preds = %panic.cont
  br label %if.merge8

if.merge8:                                        ; preds = %if.else7, %if.then6
  %54 = load ptr, ptr %"SpeculativeExpensiveBodyWarning_x3a_x3aspeculativeExpensiveBodyWarningReference$tmp$__uv_key_scope_261", align 8
  call void @uv_key_scope_exit(ptr %54)
  fence seq_cst
  fence seq_cst
  %55 = call ptr @uv_key_scope_enter()
  store ptr %55, ptr %"SpeculativeExpensiveBodyWarning_x3a_x3aspeculativeExpensiveBodyWarningReference$tmp$__uv_implicit_key_scope_639", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f77976C7416517C63, i64 7 }, ptr %coerce_bits10, align 8
  %56 = load { i64, i64 }, ptr %coerce_bits10, align 1
  call void @uv_key_check_conflict({ i64, i64 } %56, i8 0)
  %57 = load ptr, ptr %"SpeculativeExpensiveBodyWarning_x3a_x3aspeculativeExpensiveBodyWarningReference$tmp$__uv_implicit_key_scope_639", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f77976C7416517C63, i64 7 }, ptr %coerce_bits11, align 8
  %58 = load { i64, i64 }, ptr %coerce_bits11, align 1
  call void @uv_key_acquire(ptr %57, { i64, i64 } %58, i8 0)
  fence seq_cst
  fence seq_cst
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %if.merge8
  %59 = load ptr, ptr %__panic, align 8
  %60 = load i8, ptr %59, align 1
  %61 = icmp ne i8 %60, 0
  br i1 %61, label %panic.take12, label %panic.cont13

check_fail:                                       ; preds = %if.merge8
  %62 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %62, align 1
  %63 = getelementptr i8, ptr %62, i64 4
  store i32 4, ptr %63, align 4
  br label %check_ok

panic.take12:                                     ; preds = %check_ok
  %64 = load ptr, ptr %__panic, align 8
  %65 = getelementptr i8, ptr %64, i64 0
  %66 = load i8, ptr %65, align 1
  %67 = load ptr, ptr %__panic, align 8
  %68 = getelementptr i8, ptr %67, i64 4
  %69 = load i32, ptr %68, align 4
  %70 = load ptr, ptr %"SpeculativeExpensiveBodyWarning_x3a_x3aspeculativeExpensiveBodyWarningReference$tmp$__uv_implicit_key_scope_639", align 8
  call void @uv_key_scope_exit(ptr %70)
  %71 = load ptr, ptr %__panic, align 8
  %72 = getelementptr i8, ptr %71, i64 4
  %73 = load i32, ptr %72, align 4
  ret i32 %73

panic.cont13:                                     ; preds = %check_ok
  %74 = getelementptr i8, ptr %counter, i64 0
  %75 = load i32, ptr %74, align 1
  %76 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %75, i32 0)
  %77 = extractvalue { i32, i1 } %76, 0
  %78 = extractvalue { i32, i1 } %76, 1
  %79 = freeze i32 %77
  br i1 %78, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont13
  %80 = load ptr, ptr %"SpeculativeExpensiveBodyWarning_x3a_x3aspeculativeExpensiveBodyWarningReference$tmp$__uv_implicit_key_scope_639", align 8
  call void @uv_key_scope_exit(ptr %80)
  ret i32 %79

op_fail:                                          ; preds = %panic.cont13
  %81 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %81, align 1
  %82 = getelementptr i8, ptr %81, i64 4
  store i32 4, ptr %82, align 4
  ret i32 0
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aSpeculativeExpensiveBodyWarning(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aSpeculativeExpensiveBodyWarning(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #1

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #2

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #3

define hidden void @__cx_lifecycle_init_SpeculativeExpensiveBodyWarning(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aSpeculativeExpensiveBodyWarning(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_SpeculativeExpensiveBodyWarning(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aSpeculativeExpensiveBodyWarning(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aSpeculativeExpensiveBodyWarning(ptr %dll_attach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aSpeculativeExpensiveBodyWarning(ptr %dll_detach_panic_out)
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
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #3 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
