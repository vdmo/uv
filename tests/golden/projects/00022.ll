; ==== Source/Tests/AttributeTestHarness_x3a_x3aTests.ll
; ModuleID = 'AttributeTestHarness::Tests'
source_filename = "AttributeTestHarness::Tests"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aAttributeTestHarness_x3a_x3aTests = global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f88201FB960FF6465 = internal constant [16 x i8] zeroinitializer, align 1
@__uv_library_attached = internal global i1 false
@__uv_image_panic_record = common global { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, align 4
@llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_ctor, ptr null }]
@llvm.global_dtors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_dtor, ptr null }]

; Function Attrs: nounwind
declare i8 @ultraviolet_x3a_x3aruntime_x3a_x3astring_x3a_x3ais_x5fempty(ptr noundef nonnull align 8 dereferenceable(16)) #0

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3atime_x3a_x3amonotonic(ptr noundef nonnull align 8 dereferenceable(16)) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3atime_x3a_x3amonotonic_x5fresolution(ptr noundef nonnull align 16 dereferenceable(16), ptr noundef nonnull align 8 dereferenceable(16)) #0

define i8 @AttributeTestHarness_x3a_x3aTests_x3a_x3aattributeHarnessRecordsAuthority(ptr noundef nonnull align 8 dereferenceable(128) %authority, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"AttributeTestHarness_x3a_x3aTests_x3a_x3aattributeHarnessRecordsAuthority$tmp$return_snapshot_23" = alloca i8, align 1
  %resolution = alloca { i128, [0 x i128] }, align 16
  %monotonic1 = alloca { ptr, ptr }, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %monotonic = alloca { ptr, ptr }, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aAttributeTestHarness_x3a_x3aTests, align 1
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
  %7 = trunc i32 %6 to i8
  ret i8 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  %10 = getelementptr i8, ptr %authority, i64 48
  %11 = getelementptr i8, ptr %authority, i64 48
  %12 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3atime_x3a_x3amonotonic(ptr noundef nonnull align 8 dereferenceable(16) %11)
  store { i64, i64 } %12, ptr %abi_return, align 8
  %13 = load { ptr, ptr }, ptr %abi_return, align 1
  store { ptr, ptr } %13, ptr %monotonic1, align 8
  call void @ultraviolet_x3a_x3aruntime_x3a_x3atime_x3a_x3amonotonic_x5fresolution(ptr noundef nonnull align 16 dereferenceable(16) %resolution, ptr noundef nonnull align 8 dereferenceable(16) %monotonic1)
  %14 = getelementptr i8, ptr %authority, i64 96
  %15 = getelementptr i8, ptr %authority, i64 96
  %view.prefix = load { ptr, i64 }, ptr %15, align 8
  %view.len = extractvalue { ptr, i64 } %view.prefix, 1
  %16 = icmp eq i64 %view.len, 0
  %17 = zext i1 %16 to i8
  %18 = icmp ne i8 %17, 0
  %19 = xor i1 %18, true
  %20 = zext i1 %19 to i8
  %21 = icmp ne i8 %20, 0
  %22 = icmp ne i8 %20, 0
  br i1 %22, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  %23 = getelementptr i8, ptr %authority, i64 64
  %24 = getelementptr i8, ptr %authority, i64 64
  %view.prefix2 = load { ptr, i64 }, ptr %24, align 8
  %view.len3 = extractvalue { ptr, i64 } %view.prefix2, 1
  %25 = icmp eq i64 %view.len3, 0
  %26 = zext i1 %25 to i8
  %27 = icmp ne i8 %26, 0
  %28 = xor i1 %27, true
  %29 = zext i1 %28 to i8
  br label %if.merge

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %"AttributeTestHarness_x3a_x3aTests_x3a_x3aattributeHarnessRecordsAuthority$tmp$and_13" = phi i8 [ %29, %if.then ], [ 0, %if.else ]
  %30 = icmp ne i8 %"AttributeTestHarness_x3a_x3aTests_x3a_x3aattributeHarnessRecordsAuthority$tmp$and_13", 0
  %31 = icmp ne i8 %"AttributeTestHarness_x3a_x3aTests_x3a_x3aattributeHarnessRecordsAuthority$tmp$and_13", 0
  br i1 %31, label %if.then4, label %if.else5

if.then4:                                         ; preds = %if.merge
  %32 = getelementptr i8, ptr %authority, i64 112
  %33 = getelementptr i8, ptr %authority, i64 112
  %view.prefix7 = load { ptr, i64 }, ptr %33, align 8
  %view.len8 = extractvalue { ptr, i64 } %view.prefix7, 1
  %34 = icmp eq i64 %view.len8, 0
  %35 = zext i1 %34 to i8
  %36 = icmp ne i8 %35, 0
  %37 = xor i1 %36, true
  %38 = zext i1 %37 to i8
  br label %if.merge6

if.else5:                                         ; preds = %if.merge
  br label %if.merge6

if.merge6:                                        ; preds = %if.else5, %if.then4
  %"AttributeTestHarness_x3a_x3aTests_x3a_x3aattributeHarnessRecordsAuthority$tmp$and_18" = phi i8 [ %38, %if.then4 ], [ 0, %if.else5 ]
  %39 = icmp ne i8 %"AttributeTestHarness_x3a_x3aTests_x3a_x3aattributeHarnessRecordsAuthority$tmp$and_18", 0
  %40 = icmp ne i8 %"AttributeTestHarness_x3a_x3aTests_x3a_x3aattributeHarnessRecordsAuthority$tmp$and_18", 0
  br i1 %40, label %if.then9, label %if.else10

if.then9:                                         ; preds = %if.merge6
  %41 = getelementptr i8, ptr %resolution, i64 0
  %42 = load i128, ptr %41, align 1
  %43 = icmp ugt i128 %42, 0
  %44 = zext i1 %43 to i8
  br label %if.merge11

if.else10:                                        ; preds = %if.merge6
  br label %if.merge11

if.merge11:                                       ; preds = %if.else10, %if.then9
  %"AttributeTestHarness_x3a_x3aTests_x3a_x3aattributeHarnessRecordsAuthority$tmp$and_22" = phi i8 [ %44, %if.then9 ], [ 0, %if.else10 ]
  %45 = icmp ne i8 %"AttributeTestHarness_x3a_x3aTests_x3a_x3aattributeHarnessRecordsAuthority$tmp$and_22", 0
  %46 = zext i1 %45 to i8
  store i8 %46, ptr %"AttributeTestHarness_x3a_x3aTests_x3a_x3aattributeHarnessRecordsAuthority$tmp$return_snapshot_23", align 1
  %47 = load i8, ptr %"AttributeTestHarness_x3a_x3aTests_x3a_x3aattributeHarnessRecordsAuthority$tmp$return_snapshot_23", align 1
  %48 = icmp ne i8 %47, 0
  %49 = load i8, ptr %"AttributeTestHarness_x3a_x3aTests_x3a_x3aattributeHarnessRecordsAuthority$tmp$return_snapshot_23", align 1
  %50 = icmp ne i8 %49, 0
  br i1 %50, label %if.then12, label %if.else13

if.then12:                                        ; preds = %if.merge11
  br label %if.merge14

if.else13:                                        ; preds = %if.merge11
  %51 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %51, align 1
  %52 = getelementptr i8, ptr %51, i64 4
  store i32 12, ptr %52, align 4
  %53 = load ptr, ptr %__panic, align 8
  %54 = getelementptr i8, ptr %53, i64 0
  %55 = load i8, ptr %54, align 1
  %56 = load ptr, ptr %__panic, align 8
  %57 = getelementptr i8, ptr %56, i64 4
  %58 = load i32, ptr %57, align 4
  %59 = load ptr, ptr %__panic, align 8
  %60 = getelementptr i8, ptr %59, i64 4
  %61 = load i32, ptr %60, align 4
  %62 = trunc i32 %61 to i8
  ret i8 %62

if.merge14:                                       ; preds = %if.then12
  %63 = load i8, ptr %"AttributeTestHarness_x3a_x3aTests_x3a_x3aattributeHarnessRecordsAuthority$tmp$return_snapshot_23", align 1
  %64 = icmp ne i8 %63, 0
  %65 = zext i1 %64 to i8
  ret i8 %65
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aAttributeTestHarness_x3a_x3aTests(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aAttributeTestHarness_x3a_x3aTests(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define void @__cx_lifecycle_init_AttributeTestHarness_x3a_x3aTests(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aAttributeTestHarness_x3a_x3aTests(ptr %0)
  ret void
}

define void @__cx_lifecycle_deinit_AttributeTestHarness_x3a_x3aTests(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aAttributeTestHarness_x3a_x3aTests(ptr %0)
  ret void
}

define i32 @__ultraviolet_library_entry(ptr %0, i32 %fdwReason, ptr %1) {
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aAttributeTestHarness_x3a_x3aTests(ptr %dll_attach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aAttributeTestHarness_x3a_x3aTests(ptr %dll_detach_panic_out)
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
