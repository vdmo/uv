; ==== Source/DynamicKeyRuntimeSyncInfo.ll
; ModuleID = 'DynamicKeyRuntimeSyncInfo'
source_filename = "DynamicKeyRuntimeSyncInfo"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aDynamicKeyRuntimeSyncInfo = hidden global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f75C3F0F4D649389E = internal constant [7 x i8] c"backing", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAF63BD4C8601B7DF = internal constant [1 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f987468C2D70EDBD5 = internal constant [8 x i8] c"\10\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2CDCDC0DFC5D1141 = internal constant [8 x i8] c"\04\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fA37F35E9B1876EEF = internal constant [7 x i8] c"values.", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAF63BC4C8601B62C = internal constant [1 x i8] c"\01", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fECF582CAA5B1B50E = internal constant [4 x i8] c"\0B\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2D157A98A06F49A8 = internal constant [4 x i8] c"\0D\00\00\00", align 1
@__uv_library_attached = internal global i1 false
@__uv_image_panic_record = common hidden global { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, align 4
@llvm.global_ctors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_ctor, ptr null }]
@llvm.global_dtors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_dtor, ptr null }]

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3ainline(ptr noundef nonnull align 8 dereferenceable(96)) #0

; Function Attrs: noreturn nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4)) #1

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16)) #0

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3anew_x5fscoped(ptr noundef nonnull align 8 dereferenceable(40)) #0

; Function Attrs: nounwind
declare void @uv_key_acquire(ptr, { i64, i64 }, i8) #0

; Function Attrs: nounwind
declare void @uv_key_check_conflict({ i64, i64 }, i8) #0

; Function Attrs: nounwind
declare ptr @uv_key_scope_enter() #0

; Function Attrs: nounwind
declare void @uv_key_scope_exit(ptr) #0

; Function Attrs: nounwind
declare ptr @uv_parallel_begin({ i64, i64 }, i64, ptr) #0

; Function Attrs: nounwind
declare i32 @uv_parallel_join(ptr) #0

; Function Attrs: nounwind
declare ptr @uv_spawn_create(ptr, i64, ptr, ptr, i64, i64, i32) #0

; Function Attrs: nounwind
declare ptr @uv_spawn_wait(ptr) #0

define i32 @DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %index, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %byref_arg = alloca i32, align 4
  %second = alloca ptr, align 8
  %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$spawn_env_storage_66" = alloca { ptr, ptr }, align 8
  %first = alloca ptr, align 8
  %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$spawn_env_storage_25" = alloca { ptr, ptr }, align 8
  %coerce_bits3 = alloca { ptr, ptr }, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %values = alloca { ptr, i64 }, align 8
  %coerce_bits2 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_implicit_key_scope_61" = alloca ptr, align 8
  %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_implicit_key_scope_6" = alloca ptr, align 8
  %aggregate.literal = alloca [4 x i32], align 4
  %backing = alloca [4 x i32], align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aDynamicKeyRuntimeSyncInfo, align 1
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
  %9 = getelementptr [4 x i32], ptr %aggregate.literal, i64 0, i64 0
  store i32 1, ptr %9, align 4
  %10 = getelementptr [4 x i32], ptr %aggregate.literal, i64 0, i64 1
  store i32 2, ptr %10, align 4
  %11 = getelementptr [4 x i32], ptr %aggregate.literal, i64 0, i64 2
  store i32 3, ptr %11, align 4
  %12 = getelementptr [4 x i32], ptr %aggregate.literal, i64 0, i64 3
  store i32 4, ptr %12, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %backing, ptr align 4 %aggregate.literal, i64 16, i1 false)
  fence seq_cst
  fence seq_cst
  %13 = call ptr @uv_key_scope_enter()
  store ptr %13, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_implicit_key_scope_61", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f75C3F0F4D649389E, i64 7 }, ptr %coerce_bits, align 8
  %14 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %14, i8 0)
  %15 = load ptr, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_implicit_key_scope_61", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f75C3F0F4D649389E, i64 7 }, ptr %coerce_bits2, align 8
  %16 = load { i64, i64 }, ptr %coerce_bits2, align 1
  call void @uv_key_acquire(ptr %15, { i64, i64 } %16, i8 0)
  fence seq_cst
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %17 = load ptr, ptr %__panic, align 8
  %18 = load i8, ptr %17, align 1
  %19 = icmp ne i8 %18, 0
  br i1 %19, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %20 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %20, align 1
  %21 = getelementptr i8, ptr %20, i64 4
  store i32 6, ptr %21, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %22 = load ptr, ptr %__panic, align 8
  %23 = getelementptr i8, ptr %22, i64 0
  %24 = load i8, ptr %23, align 1
  %25 = load ptr, ptr %__panic, align 8
  %26 = getelementptr i8, ptr %25, i64 4
  %27 = load i32, ptr %26, align 4
  %28 = load ptr, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_implicit_key_scope_61", align 8
  call void @uv_key_scope_exit(ptr %28)
  %29 = load ptr, ptr %__panic, align 8
  %30 = getelementptr i8, ptr %29, i64 4
  %31 = load i32, ptr %30, align 4
  ret i32 %31

panic.cont:                                       ; preds = %check_ok
  fence seq_cst
  %32 = getelementptr i32, ptr %backing, i64 0
  %33 = insertvalue { ptr, i64 } zeroinitializer, ptr %32, 0
  %34 = insertvalue { ptr, i64 } %33, i64 4, 1
  store { ptr, i64 } %34, ptr %values, align 8
  %35 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3ainline(ptr noundef nonnull align 8 dereferenceable(96) %context)
  store { i64, i64 } %35, ptr %abi_return, align 8
  %36 = load { ptr, ptr }, ptr %abi_return, align 1
  store { ptr, ptr } %36, ptr %coerce_bits3, align 8
  %37 = load { i64, i64 }, ptr %coerce_bits3, align 1
  %38 = call ptr @uv_parallel_begin({ i64, i64 } %37, i64 -1, ptr null)
  store { ptr, ptr } zeroinitializer, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$spawn_env_storage_25", align 8
  %39 = getelementptr i8, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$spawn_env_storage_25", i64 0
  store ptr %values, ptr %39, align 8
  %40 = getelementptr i8, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$spawn_env_storage_25", i64 8
  store ptr %index, ptr %40, align 8
  %41 = call ptr @uv_spawn_create(ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$spawn_env_storage_25", i64 16, ptr @__cx_spawn_body_DynamicKeyRuntimeSyncInfo_0, ptr null, i64 4, i64 0, i32 -1)
  store ptr %41, ptr %first, align 8
  store { ptr, ptr } zeroinitializer, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$spawn_env_storage_66", align 8
  %42 = getelementptr i8, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$spawn_env_storage_66", i64 0
  store ptr %values, ptr %42, align 8
  %43 = getelementptr i8, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$spawn_env_storage_66", i64 8
  store ptr %index, ptr %43, align 8
  %44 = call ptr @uv_spawn_create(ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$spawn_env_storage_66", i64 16, ptr @__cx_spawn_body_DynamicKeyRuntimeSyncInfo_1, ptr null, i64 4, i64 0, i32 -1)
  store ptr %44, ptr %second, align 8
  %45 = load ptr, ptr %first, align 8
  %46 = call ptr @uv_spawn_wait(ptr %45)
  %47 = load i32, ptr %46, align 4
  %48 = load ptr, ptr %second, align 8
  %49 = call ptr @uv_spawn_wait(ptr %48)
  %50 = load i32, ptr %49, align 4
  br i1 true, label %check_ok4, label %check_fail5

check_ok4:                                        ; preds = %check_fail5, %panic.cont
  %51 = load ptr, ptr %__panic, align 8
  %52 = load i8, ptr %51, align 1
  %53 = icmp ne i8 %52, 0
  br i1 %53, label %panic.take6, label %panic.cont7

check_fail5:                                      ; preds = %panic.cont
  %54 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %54, align 1
  %55 = getelementptr i8, ptr %54, i64 4
  store i32 4, ptr %55, align 4
  br label %check_ok4

panic.take6:                                      ; preds = %check_ok4
  %56 = load ptr, ptr %__panic, align 8
  %57 = getelementptr i8, ptr %56, i64 0
  %58 = load i8, ptr %57, align 1
  %59 = load ptr, ptr %__panic, align 8
  %60 = getelementptr i8, ptr %59, i64 4
  %61 = load i32, ptr %60, align 4
  %62 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %62, align 1
  %63 = getelementptr i8, ptr %62, i64 4
  store i32 0, ptr %63, align 4
  %64 = call i32 @uv_parallel_join(ptr %38)
  %65 = icmp eq i32 %64, 0
  %66 = xor i1 %65, true
  %67 = zext i1 %66 to i8
  %68 = icmp ne i8 %67, 0
  %69 = icmp ne i8 %67, 0
  br i1 %69, label %if.then, label %if.else

panic.cont7:                                      ; preds = %check_ok4
  %70 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %47, i32 %50)
  %71 = extractvalue { i32, i1 } %70, 0
  %72 = extractvalue { i32, i1 } %70, 1
  %73 = freeze i32 %71
  br i1 %72, label %op_fail, label %op_ok

if.then:                                          ; preds = %panic.take6
  %74 = load ptr, ptr %__panic, align 8
  %75 = getelementptr i8, ptr %74, i64 0
  store i8 1, ptr %75, align 1
  %76 = load ptr, ptr %__panic, align 8
  %77 = getelementptr i8, ptr %76, i64 4
  store i32 %64, ptr %77, align 4
  br label %if.merge

if.else:                                          ; preds = %panic.take6
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %if.result = phi i64 [ 0, %if.then ], [ 0, %if.else ]
  %78 = load ptr, ptr %__panic, align 8
  %79 = getelementptr i8, ptr %78, i64 0
  %80 = load i8, ptr %79, align 1
  %81 = load ptr, ptr %__panic, align 8
  %82 = getelementptr i8, ptr %81, i64 4
  %83 = load i32, ptr %82, align 4
  %84 = icmp ne i8 %80, 0
  %85 = zext i1 %84 to i8
  %86 = icmp ne i8 %85, 0
  %87 = and i1 true, %86
  br i1 %87, label %if.then8, label %if.else9

if.then8:                                         ; preds = %if.merge
  store i32 %83, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else9:                                         ; preds = %if.merge
  br label %if.merge10

if.merge10:                                       ; preds = %if.else9
  %88 = icmp ne i8 %80, 0
  %89 = xor i1 %88, true
  %90 = and i1 true, %89
  br i1 %90, label %if.then11, label %if.else12

if.then11:                                        ; preds = %if.merge10
  %91 = load ptr, ptr %__panic, align 8
  %92 = getelementptr i8, ptr %91, i64 0
  store i8 1, ptr %92, align 1
  %93 = load ptr, ptr %__panic, align 8
  %94 = getelementptr i8, ptr %93, i64 4
  store i32 %61, ptr %94, align 4
  br label %if.merge13

if.else12:                                        ; preds = %if.merge10
  br label %if.merge13

if.merge13:                                       ; preds = %if.else12, %if.then11
  %if.result14 = phi i64 [ 0, %if.then11 ], [ 0, %if.else12 ]
  %95 = icmp ne i8 %80, 0
  %96 = zext i1 %95 to i8
  %97 = icmp ne i8 %96, 0
  %98 = or i1 true, %97
  %99 = icmp ne i8 %80, 0
  %100 = icmp ne i8 %80, 0
  br i1 %100, label %if.then15, label %if.else16

if.then15:                                        ; preds = %if.merge13
  br label %if.merge17

if.else16:                                        ; preds = %if.merge13
  br label %if.merge17

if.merge17:                                       ; preds = %if.else16, %if.then15
  %if.result18 = phi i32 [ %83, %if.then15 ], [ %61, %if.else16 ]
  %101 = load ptr, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_implicit_key_scope_61", align 8
  call void @uv_key_scope_exit(ptr %101)
  %102 = load ptr, ptr %__panic, align 8
  %103 = getelementptr i8, ptr %102, i64 4
  %104 = load i32, ptr %103, align 4
  ret i32 %104

op_ok:                                            ; preds = %panic.cont7
  %105 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %105, align 1
  %106 = getelementptr i8, ptr %105, i64 4
  store i32 0, ptr %106, align 4
  %107 = call i32 @uv_parallel_join(ptr %38)
  %108 = icmp eq i32 %107, 0
  %109 = xor i1 %108, true
  %110 = zext i1 %109 to i8
  %111 = icmp ne i8 %110, 0
  %112 = icmp ne i8 %110, 0
  br i1 %112, label %if.then19, label %if.else20

op_fail:                                          ; preds = %panic.cont7
  %113 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %113, align 1
  %114 = getelementptr i8, ptr %113, i64 4
  store i32 4, ptr %114, align 4
  ret i32 0

if.then19:                                        ; preds = %op_ok
  %115 = load ptr, ptr %__panic, align 8
  %116 = getelementptr i8, ptr %115, i64 0
  store i8 1, ptr %116, align 1
  %117 = load ptr, ptr %__panic, align 8
  %118 = getelementptr i8, ptr %117, i64 4
  store i32 %107, ptr %118, align 4
  br label %if.merge21

if.else20:                                        ; preds = %op_ok
  br label %if.merge21

if.merge21:                                       ; preds = %if.else20, %if.then19
  %if.result22 = phi i64 [ 0, %if.then19 ], [ 0, %if.else20 ]
  %119 = load ptr, ptr %__panic, align 8
  %120 = getelementptr i8, ptr %119, i64 0
  %121 = load i8, ptr %120, align 1
  %122 = load ptr, ptr %__panic, align 8
  %123 = getelementptr i8, ptr %122, i64 4
  %124 = load i32, ptr %123, align 4
  %125 = icmp ne i8 %121, 0
  %126 = zext i1 %125 to i8
  %127 = icmp ne i8 %126, 0
  %128 = and i1 false, %127
  br i1 %128, label %if.then23, label %if.else24

if.then23:                                        ; preds = %if.merge21
  store i32 %124, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else24:                                        ; preds = %if.merge21
  br label %if.merge25

if.merge25:                                       ; preds = %if.else24
  %129 = icmp ne i8 %121, 0
  %130 = xor i1 %129, true
  %131 = and i1 false, %130
  br i1 %131, label %if.then26, label %if.else27

if.then26:                                        ; preds = %if.merge25
  %132 = load ptr, ptr %__panic, align 8
  %133 = getelementptr i8, ptr %132, i64 0
  store i8 1, ptr %133, align 1
  %134 = load ptr, ptr %__panic, align 8
  %135 = getelementptr i8, ptr %134, i64 4
  store i32 0, ptr %135, align 4
  br label %if.merge28

if.else27:                                        ; preds = %if.merge25
  br label %if.merge28

if.merge28:                                       ; preds = %if.else27, %if.then26
  %if.result29 = phi i64 [ 0, %if.then26 ], [ 0, %if.else27 ]
  %136 = icmp ne i8 %121, 0
  %137 = zext i1 %136 to i8
  %138 = icmp ne i8 %137, 0
  %139 = or i1 false, %138
  %140 = icmp ne i8 %121, 0
  %141 = icmp ne i8 %121, 0
  br i1 %141, label %if.then30, label %if.else31

if.then30:                                        ; preds = %if.merge28
  br label %if.merge32

if.else31:                                        ; preds = %if.merge28
  br label %if.merge32

if.merge32:                                       ; preds = %if.else31, %if.then30
  %if.result33 = phi i32 [ %124, %if.then30 ], [ 0, %if.else31 ]
  %142 = load ptr, ptr %__panic, align 8
  %143 = load i8, ptr %142, align 1
  %144 = icmp ne i8 %143, 0
  br i1 %144, label %panic.take34, label %panic.cont35

panic.take34:                                     ; preds = %if.merge32
  %145 = load ptr, ptr %__panic, align 8
  %146 = getelementptr i8, ptr %145, i64 4
  %147 = load i32, ptr %146, align 4
  ret i32 %147

panic.cont35:                                     ; preds = %if.merge32
  %148 = load ptr, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_implicit_key_scope_61", align 8
  call void @uv_key_scope_exit(ptr %148)
  ret i32 %73
}

define internal void @__cx_spawn_body_DynamicKeyRuntimeSyncInfo_0(ptr %__uv_host_env, ptr noundef nonnull align 8 dereferenceable(16) %env, ptr noundef nonnull align 4 dereferenceable(4) %result, ptr %__panic) {
entry:
  %abi_return9 = alloca { i64, i64 }, align 8
  %abi_return8 = alloca { i64, i64 }, align 8
  %coerce_bits7 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_key_scope_366" = alloca ptr, align 8
  %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_key_scope_36" = alloca ptr, align 8
  %observed = alloca i32, align 4
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
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aDynamicKeyRuntimeSyncInfo, align 1
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
  store i32 0, ptr %observed, align 4
  %9 = call ptr @uv_key_scope_enter()
  store ptr %9, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_key_scope_366", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fA37F35E9B1876EEF, i64 7 }, ptr %coerce_bits, align 8
  %10 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %10, i8 1)
  %11 = load ptr, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_key_scope_366", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fA37F35E9B1876EEF, i64 7 }, ptr %coerce_bits7, align 8
  %12 = load { i64, i64 }, ptr %coerce_bits7, align 1
  call void @uv_key_acquire(ptr %11, { i64, i64 } %12, i8 1)
  %13 = load ptr, ptr %env2, align 8
  %14 = getelementptr i8, ptr %13, i64 0
  %15 = load ptr, ptr %14, align 8
  %16 = load ptr, ptr %env2, align 8
  %17 = getelementptr i8, ptr %16, i64 8
  %18 = load ptr, ptr %17, align 8
  %19 = load i64, ptr %18, align 4
  %20 = load { ptr, i64 }, ptr %15, align 8
  %21 = extractvalue { ptr, i64 } %20, 1
  %22 = icmp ult i64 %19, %21
  br i1 %22, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %23 = load ptr, ptr %__panic4, align 8
  %24 = load i8, ptr %23, align 1
  %25 = icmp ne i8 %24, 0
  br i1 %25, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %26 = load ptr, ptr %__panic4, align 8
  store i8 1, ptr %26, align 1
  %27 = getelementptr i8, ptr %26, i64 4
  store i32 6, ptr %27, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %28 = load ptr, ptr %__panic4, align 8
  %29 = getelementptr i8, ptr %28, i64 0
  %30 = load i8, ptr %29, align 1
  %31 = load ptr, ptr %__panic4, align 8
  %32 = getelementptr i8, ptr %31, i64 4
  %33 = load i32, ptr %32, align 4
  %34 = load ptr, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_key_scope_366", align 8
  call void @uv_key_scope_exit(ptr %34)
  %35 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$05")
  store { i64, i64 } %35, ptr %abi_return8, align 8
  %36 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return8, align 1
  ret void

panic.cont:                                       ; preds = %check_ok
  %37 = load { ptr, i64 }, ptr %15, align 8
  %38 = extractvalue { ptr, i64 } %37, 0
  %39 = getelementptr i32, ptr %38, i64 %19
  %40 = load { ptr, i64 }, ptr %15, align 8
  %41 = extractvalue { ptr, i64 } %40, 0
  %42 = getelementptr i32, ptr %41, i64 %19
  store i32 11, ptr %42, align 4
  store i32 11, ptr %observed, align 4
  %43 = load ptr, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_key_scope_366", align 8
  call void @uv_key_scope_exit(ptr %43)
  %44 = load i32, ptr %observed, align 4
  %45 = load ptr, ptr %result3, align 8
  %46 = load i32, ptr %observed, align 4
  store i32 %46, ptr %45, align 4
  %47 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$05")
  store { i64, i64 } %47, ptr %abi_return9, align 8
  %48 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return9, align 1
  ret void
}

define internal void @__cx_spawn_body_DynamicKeyRuntimeSyncInfo_1(ptr %__uv_host_env, ptr noundef nonnull align 8 dereferenceable(16) %env, ptr noundef nonnull align 4 dereferenceable(4) %result, ptr %__panic) {
entry:
  %abi_return9 = alloca { i64, i64 }, align 8
  %abi_return8 = alloca { i64, i64 }, align 8
  %coerce_bits7 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_key_scope_776" = alloca ptr, align 8
  %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_key_scope_77" = alloca ptr, align 8
  %observed = alloca i32, align 4
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
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aDynamicKeyRuntimeSyncInfo, align 1
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
  store i32 0, ptr %observed, align 4
  %9 = call ptr @uv_key_scope_enter()
  store ptr %9, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_key_scope_776", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fA37F35E9B1876EEF, i64 7 }, ptr %coerce_bits, align 8
  %10 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %10, i8 1)
  %11 = load ptr, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_key_scope_776", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fA37F35E9B1876EEF, i64 7 }, ptr %coerce_bits7, align 8
  %12 = load { i64, i64 }, ptr %coerce_bits7, align 1
  call void @uv_key_acquire(ptr %11, { i64, i64 } %12, i8 1)
  %13 = load ptr, ptr %env2, align 8
  %14 = getelementptr i8, ptr %13, i64 0
  %15 = load ptr, ptr %14, align 8
  %16 = load ptr, ptr %env2, align 8
  %17 = getelementptr i8, ptr %16, i64 8
  %18 = load ptr, ptr %17, align 8
  %19 = load i64, ptr %18, align 4
  %20 = load { ptr, i64 }, ptr %15, align 8
  %21 = extractvalue { ptr, i64 } %20, 1
  %22 = icmp ult i64 %19, %21
  br i1 %22, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %23 = load ptr, ptr %__panic4, align 8
  %24 = load i8, ptr %23, align 1
  %25 = icmp ne i8 %24, 0
  br i1 %25, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %26 = load ptr, ptr %__panic4, align 8
  store i8 1, ptr %26, align 1
  %27 = getelementptr i8, ptr %26, i64 4
  store i32 6, ptr %27, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %28 = load ptr, ptr %__panic4, align 8
  %29 = getelementptr i8, ptr %28, i64 0
  %30 = load i8, ptr %29, align 1
  %31 = load ptr, ptr %__panic4, align 8
  %32 = getelementptr i8, ptr %31, i64 4
  %33 = load i32, ptr %32, align 4
  %34 = load ptr, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_key_scope_776", align 8
  call void @uv_key_scope_exit(ptr %34)
  %35 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$05")
  store { i64, i64 } %35, ptr %abi_return8, align 8
  %36 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return8, align 1
  ret void

panic.cont:                                       ; preds = %check_ok
  %37 = load { ptr, i64 }, ptr %15, align 8
  %38 = extractvalue { ptr, i64 } %37, 0
  %39 = getelementptr i32, ptr %38, i64 %19
  %40 = load { ptr, i64 }, ptr %15, align 8
  %41 = extractvalue { ptr, i64 } %40, 0
  %42 = getelementptr i32, ptr %41, i64 %19
  store i32 13, ptr %42, align 4
  store i32 13, ptr %observed, align 4
  %43 = load ptr, ptr %"DynamicKeyRuntimeSyncInfo_x3a_x3adynamicKeyRuntimeSyncInfoReference$tmp$__uv_key_scope_776", align 8
  call void @uv_key_scope_exit(ptr %43)
  %44 = load i32, ptr %observed, align 4
  %45 = load ptr, ptr %result3, align 8
  %46 = load i32, ptr %observed, align 4
  store i32 %46, ptr %45, align 4
  %47 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$05")
  store { i64, i64 } %47, ptr %abi_return9, align 8
  %48 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return9, align 1
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aDynamicKeyRuntimeSyncInfo(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aDynamicKeyRuntimeSyncInfo(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #2

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #3

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #4

define hidden void @__cx_lifecycle_init_DynamicKeyRuntimeSyncInfo(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aDynamicKeyRuntimeSyncInfo(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_DynamicKeyRuntimeSyncInfo(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aDynamicKeyRuntimeSyncInfo(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aDynamicKeyRuntimeSyncInfo(ptr %dll_attach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aDynamicKeyRuntimeSyncInfo(ptr %dll_detach_panic_out)
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
attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #3 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #4 = { nocallback nofree nounwind willreturn memory(argmem: write) }
