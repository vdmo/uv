; ==== Source/StructuredParallelLoweringEvidence.ll
; ModuleID = 'StructuredParallelLoweringEvidence'
source_filename = "StructuredParallelLoweringEvidence"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence = global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fA8C7F832281A39C5 = internal constant [8 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2CDCDC0DFC5D1141 = internal constant [8 x i8] c"\04\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fA09E307A7F948ACD = internal constant [8 x i8] c"\08\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fADAAA9AF328EA9CC = internal constant [4 x i8] c")\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f6D3572669B2CDE42 = internal constant [4 x i8] c"\07\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2D401A55EEC16520 = internal constant [4 x i8] c"\05\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fEC75A392BABB62A6 = internal constant [4 x i8] c"c\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4CFAD6C24F7BF87D = internal constant [4 x i8] c"\08\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f8D9AADC8352FDF7F = internal constant [4 x i8] c"*\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f994EF6653E295FD1 = internal constant [4 x i8] c"\FF\FF\FF\7F", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAD2ACA7747985764 = internal constant [4 x i8] c"\01\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f89CD31291D2AEFA4 = internal constant [8 x i8] c"\01\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3abytes_x5f5777AA367F8561AB = internal constant [57 x i8] c"queued_unstarted_suppression=0;mid_execution_checkpoint=5", align 1
@0 = private unnamed_addr constant [2 x i8] c"+\00", align 1

; Function Attrs: nounwind
declare void @CancelToken_x3a_x3aActive_x3a_x3acancel(ptr) #0

; Function Attrs: nounwind
declare i64 @CancelToken_x3a_x3aActive_x3a_x3achild(ptr) #0

; Function Attrs: nounwind
declare i8 @CancelToken_x3a_x3aActive_x3a_x3ais_x5fcancelled(ptr) #0

; Function Attrs: nounwind
declare void @CancelToken_x3a_x3aActive_x3a_x3await_x5fcancelled(ptr noundef nonnull align 8 dereferenceable(24), ptr) #0

; Function Attrs: nounwind
declare i64 @CancelToken_x3a_x3anew() #0

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3ainline(ptr noundef nonnull align 8 dereferenceable(96)) #0

; Function Attrs: nounwind
declare i16 @ultraviolet_x3a_x3aruntime_x3a_x3aio_x3a_x3awrite_x5ffile(ptr noundef nonnull align 8 dereferenceable(16), ptr noundef nonnull align 8 dereferenceable(16), ptr noundef nonnull align 8 dereferenceable(16)) #0

; Function Attrs: noreturn nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4)) #1

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16)) #0

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3anew_x5fscoped(ptr noundef nonnull align 8 dereferenceable(40)) #0

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3asystem_x3a_x3aargument(ptr noundef nonnull align 8 dereferenceable(16), ptr noundef nonnull align 8 dereferenceable(8)) #0

; Function Attrs: nounwind
declare i64 @ultraviolet_x3a_x3aruntime_x3a_x3asystem_x3a_x3aargument_x5fcount(ptr noundef nonnull align 8 dereferenceable(16)) #0

; Function Attrs: nounwind
declare void @uv_dispatch_run(ptr noundef nonnull align 8 dereferenceable(24), i64, i64, ptr, ptr, ptr, { i64, i64 }, ptr, ptr, i32, i64, ptr noundef nonnull byval({ i64, i64, i64 }) align 8 dereferenceable(24)) #0

; Function Attrs: nounwind
declare ptr @uv_parallel_begin({ i64, i64 }, i64, ptr) #0

; Function Attrs: nounwind
declare i32 @uv_parallel_join(ptr) #0

; Function Attrs: nounwind
declare ptr @uv_spawn_create(ptr, i64, ptr, ptr, i64, i64, i32) #0

; Function Attrs: nounwind
declare ptr @uv_spawn_wait(ptr) #0

define i32 @StructuredParallelLoweringEvidence_x3a_x3aqueuedCancellationRuntimeEvidence(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %byref_arg10 = alloca i32, align 4
  %reduced = alloca i32, align 4
  %byref_arg6 = alloca { i64, i64, i64 }, align 8
  %coerce_bits5 = alloca { ptr, i64 }, align 8
  %byref_arg = alloca { i8, [7 x i8], i64, i64 }, align 8
  %0 = alloca { i64, i64, i64 }, align 8
  %"StructuredParallelLoweringEvidence_x3a_x3aqueuedCancellationRuntimeEvidence$tmp$dispatch_result_var_29" = alloca i32, align 4
  %"StructuredParallelLoweringEvidence_x3a_x3aqueuedCancellationRuntimeEvidence$tmp$dispatch_env_storage_27" = alloca {}, align 1
  %spawned = alloca ptr, align 8
  %"StructuredParallelLoweringEvidence_x3a_x3aqueuedCancellationRuntimeEvidence$tmp$spawn_env_storage_10" = alloca {}, align 1
  %coerce_bits4 = alloca { ptr, ptr }, align 8
  %coerce_bits = alloca { i64, [0 x i64] }, align 8
  %abi_return3 = alloca { i64, i64 }, align 8
  %child2 = alloca { i64, [0 x i64] }, align 8
  %abi_return1 = alloca i64, align 8
  %child = alloca { i64, [0 x i64] }, align 8
  %token = alloca { i64, [0 x i64] }, align 8
  %abi_return = alloca i64, align 8
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
  %2 = icmp ne i8 %1, 0
  br i1 %2, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %3 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %3, align 1
  %4 = getelementptr i8, ptr %3, i64 4
  store i32 10, ptr %4, align 4
  %5 = load ptr, ptr %__panic, align 8
  %6 = getelementptr i8, ptr %5, i64 4
  %7 = load i32, ptr %6, align 4
  ret i32 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  %10 = call i64 @CancelToken_x3a_x3anew()
  store i64 %10, ptr %abi_return, align 8
  %11 = load { i64, [0 x i64] }, ptr %abi_return, align 1
  store { i64, [0 x i64] } %11, ptr %token, align 4
  %12 = call i64 @CancelToken_x3a_x3aActive_x3a_x3achild(ptr %token)
  store i64 %12, ptr %abi_return1, align 8
  %13 = load { i64, [0 x i64] }, ptr %abi_return1, align 1
  store { i64, [0 x i64] } %13, ptr %child2, align 4
  call void @CancelToken_x3a_x3aActive_x3a_x3acancel(ptr %token)
  %14 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3ainline(ptr noundef nonnull align 8 dereferenceable(96) %context)
  store { i64, i64 } %14, ptr %abi_return3, align 8
  %15 = load { ptr, ptr }, ptr %abi_return3, align 1
  %16 = load { i64, [0 x i64] }, ptr %child2, align 4
  store { i64, [0 x i64] } %16, ptr %coerce_bits, align 8
  %17 = load i64, ptr %coerce_bits, align 1
  store { ptr, ptr } %15, ptr %coerce_bits4, align 8
  %18 = load { i64, i64 }, ptr %coerce_bits4, align 1
  %19 = call ptr @uv_parallel_begin({ i64, i64 } %18, i64 %17, ptr null)
  store {} zeroinitializer, ptr %"StructuredParallelLoweringEvidence_x3a_x3aqueuedCancellationRuntimeEvidence$tmp$spawn_env_storage_10", align 1
  %20 = call ptr @uv_spawn_create(ptr %"StructuredParallelLoweringEvidence_x3a_x3aqueuedCancellationRuntimeEvidence$tmp$spawn_env_storage_10", i64 0, ptr @__cx_spawn_body_StructuredParallelLoweringEvidence_0, ptr null, i64 4, i64 0, i32 -1)
  store ptr %20, ptr %spawned, align 8
  store {} zeroinitializer, ptr %"StructuredParallelLoweringEvidence_x3a_x3aqueuedCancellationRuntimeEvidence$tmp$dispatch_env_storage_27", align 1
  store i32 0, ptr %"StructuredParallelLoweringEvidence_x3a_x3aqueuedCancellationRuntimeEvidence$tmp$dispatch_result_var_29", align 4
  store { i64, i64, i64 } zeroinitializer, ptr %0, align 4
  %21 = getelementptr i8, ptr %0, i64 0
  store i64 64, ptr %21, align 1
  %22 = getelementptr i8, ptr %0, i64 8
  store i64 1, ptr %22, align 1
  %23 = getelementptr i8, ptr %0, i64 16
  store i64 1, ptr %23, align 1
  %24 = load { i64, i64, i64 }, ptr %0, align 4
  store { i8, [7 x i8], i64, i64 } { i8 4, [7 x i8] zeroinitializer, i64 0, i64 3 }, ptr %byref_arg, align 4
  store { ptr, i64 } { ptr @0, i64 1 }, ptr %coerce_bits5, align 8
  %25 = load { i64, i64 }, ptr %coerce_bits5, align 1
  store { i64, i64, i64 } %24, ptr %byref_arg6, align 4
  call void @uv_dispatch_run(ptr noundef nonnull align 8 dereferenceable(24) %byref_arg, i64 8, i64 4, ptr @__cx_dispatch_body_StructuredParallelLoweringEvidence_1, ptr null, ptr %"StructuredParallelLoweringEvidence_x3a_x3aqueuedCancellationRuntimeEvidence$tmp$dispatch_env_storage_27", { i64, i64 } %25, ptr %"StructuredParallelLoweringEvidence_x3a_x3aqueuedCancellationRuntimeEvidence$tmp$dispatch_result_var_29", ptr null, i32 1, i64 0, ptr noundef nonnull byval({ i64, i64, i64 }) align 8 dereferenceable(24) %byref_arg6)
  %26 = load i32, ptr %"StructuredParallelLoweringEvidence_x3a_x3aqueuedCancellationRuntimeEvidence$tmp$dispatch_result_var_29", align 4
  store i32 %26, ptr %reduced, align 4
  %27 = load ptr, ptr %spawned, align 8
  %28 = call ptr @uv_spawn_wait(ptr %27)
  %29 = load i32, ptr %28, align 4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %30 = load ptr, ptr %__panic, align 8
  %31 = load i8, ptr %30, align 1
  %32 = icmp ne i8 %31, 0
  br i1 %32, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %33 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %33, align 1
  %34 = getelementptr i8, ptr %33, i64 4
  store i32 4, ptr %34, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %35 = load ptr, ptr %__panic, align 8
  %36 = getelementptr i8, ptr %35, i64 0
  %37 = load i8, ptr %36, align 1
  %38 = load ptr, ptr %__panic, align 8
  %39 = getelementptr i8, ptr %38, i64 4
  %40 = load i32, ptr %39, align 4
  %41 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %41, align 1
  %42 = getelementptr i8, ptr %41, i64 4
  store i32 0, ptr %42, align 4
  %43 = call i32 @uv_parallel_join(ptr %19)
  %44 = icmp eq i32 %43, 0
  %45 = xor i1 %44, true
  %46 = zext i1 %45 to i8
  %47 = icmp ne i8 %46, 0
  %48 = icmp ne i8 %46, 0
  br i1 %48, label %if.then, label %if.else

panic.cont:                                       ; preds = %check_ok
  %49 = load i32, ptr %reduced, align 4
  %50 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %29, i32 %49)
  %51 = extractvalue { i32, i1 } %50, 0
  %52 = extractvalue { i32, i1 } %50, 1
  %53 = freeze i32 %51
  br i1 %52, label %op_fail, label %op_ok

if.then:                                          ; preds = %panic.take
  %54 = load ptr, ptr %__panic, align 8
  %55 = getelementptr i8, ptr %54, i64 0
  store i8 1, ptr %55, align 1
  %56 = load ptr, ptr %__panic, align 8
  %57 = getelementptr i8, ptr %56, i64 4
  store i32 %43, ptr %57, align 4
  br label %if.merge

if.else:                                          ; preds = %panic.take
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %if.result = phi i64 [ 0, %if.then ], [ 0, %if.else ]
  %58 = load ptr, ptr %__panic, align 8
  %59 = getelementptr i8, ptr %58, i64 0
  %60 = load i8, ptr %59, align 1
  %61 = load ptr, ptr %__panic, align 8
  %62 = getelementptr i8, ptr %61, i64 4
  %63 = load i32, ptr %62, align 4
  %64 = icmp ne i8 %60, 0
  %65 = zext i1 %64 to i8
  %66 = icmp ne i8 %65, 0
  %67 = and i1 true, %66
  br i1 %67, label %if.then7, label %if.else8

if.then7:                                         ; preds = %if.merge
  store i32 %63, ptr %byref_arg10, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg10)
  unreachable

if.else8:                                         ; preds = %if.merge
  br label %if.merge9

if.merge9:                                        ; preds = %if.else8
  %68 = icmp ne i8 %60, 0
  %69 = xor i1 %68, true
  %70 = and i1 true, %69
  br i1 %70, label %if.then11, label %if.else12

if.then11:                                        ; preds = %if.merge9
  %71 = load ptr, ptr %__panic, align 8
  %72 = getelementptr i8, ptr %71, i64 0
  store i8 1, ptr %72, align 1
  %73 = load ptr, ptr %__panic, align 8
  %74 = getelementptr i8, ptr %73, i64 4
  store i32 %40, ptr %74, align 4
  br label %if.merge13

if.else12:                                        ; preds = %if.merge9
  br label %if.merge13

if.merge13:                                       ; preds = %if.else12, %if.then11
  %if.result14 = phi i64 [ 0, %if.then11 ], [ 0, %if.else12 ]
  %75 = icmp ne i8 %60, 0
  %76 = zext i1 %75 to i8
  %77 = icmp ne i8 %76, 0
  %78 = or i1 true, %77
  %79 = icmp ne i8 %60, 0
  %80 = icmp ne i8 %60, 0
  br i1 %80, label %if.then15, label %if.else16

if.then15:                                        ; preds = %if.merge13
  br label %if.merge17

if.else16:                                        ; preds = %if.merge13
  br label %if.merge17

if.merge17:                                       ; preds = %if.else16, %if.then15
  %if.result18 = phi i32 [ %63, %if.then15 ], [ %40, %if.else16 ]
  %81 = load ptr, ptr %__panic, align 8
  %82 = getelementptr i8, ptr %81, i64 4
  %83 = load i32, ptr %82, align 4
  ret i32 %83

op_ok:                                            ; preds = %panic.cont
  %84 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %84, align 1
  %85 = getelementptr i8, ptr %84, i64 4
  store i32 0, ptr %85, align 4
  %86 = call i32 @uv_parallel_join(ptr %19)
  %87 = icmp eq i32 %86, 0
  %88 = xor i1 %87, true
  %89 = zext i1 %88 to i8
  %90 = icmp ne i8 %89, 0
  %91 = icmp ne i8 %89, 0
  br i1 %91, label %if.then19, label %if.else20

op_fail:                                          ; preds = %panic.cont
  %92 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %92, align 1
  %93 = getelementptr i8, ptr %92, i64 4
  store i32 4, ptr %93, align 4
  ret i32 0

if.then19:                                        ; preds = %op_ok
  %94 = load ptr, ptr %__panic, align 8
  %95 = getelementptr i8, ptr %94, i64 0
  store i8 1, ptr %95, align 1
  %96 = load ptr, ptr %__panic, align 8
  %97 = getelementptr i8, ptr %96, i64 4
  store i32 %86, ptr %97, align 4
  br label %if.merge21

if.else20:                                        ; preds = %op_ok
  br label %if.merge21

if.merge21:                                       ; preds = %if.else20, %if.then19
  %if.result22 = phi i64 [ 0, %if.then19 ], [ 0, %if.else20 ]
  %98 = load ptr, ptr %__panic, align 8
  %99 = getelementptr i8, ptr %98, i64 0
  %100 = load i8, ptr %99, align 1
  %101 = load ptr, ptr %__panic, align 8
  %102 = getelementptr i8, ptr %101, i64 4
  %103 = load i32, ptr %102, align 4
  %104 = icmp ne i8 %100, 0
  %105 = zext i1 %104 to i8
  %106 = icmp ne i8 %105, 0
  %107 = and i1 false, %106
  br i1 %107, label %if.then23, label %if.else24

if.then23:                                        ; preds = %if.merge21
  store i32 %103, ptr %byref_arg10, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg10)
  unreachable

if.else24:                                        ; preds = %if.merge21
  br label %if.merge25

if.merge25:                                       ; preds = %if.else24
  %108 = icmp ne i8 %100, 0
  %109 = xor i1 %108, true
  %110 = and i1 false, %109
  br i1 %110, label %if.then26, label %if.else27

if.then26:                                        ; preds = %if.merge25
  %111 = load ptr, ptr %__panic, align 8
  %112 = getelementptr i8, ptr %111, i64 0
  store i8 1, ptr %112, align 1
  %113 = load ptr, ptr %__panic, align 8
  %114 = getelementptr i8, ptr %113, i64 4
  store i32 0, ptr %114, align 4
  br label %if.merge28

if.else27:                                        ; preds = %if.merge25
  br label %if.merge28

if.merge28:                                       ; preds = %if.else27, %if.then26
  %if.result29 = phi i64 [ 0, %if.then26 ], [ 0, %if.else27 ]
  %115 = icmp ne i8 %100, 0
  %116 = zext i1 %115 to i8
  %117 = icmp ne i8 %116, 0
  %118 = or i1 false, %117
  %119 = icmp ne i8 %100, 0
  %120 = icmp ne i8 %100, 0
  br i1 %120, label %if.then30, label %if.else31

if.then30:                                        ; preds = %if.merge28
  br label %if.merge32

if.else31:                                        ; preds = %if.merge28
  br label %if.merge32

if.merge32:                                       ; preds = %if.else31, %if.then30
  %if.result33 = phi i32 [ %103, %if.then30 ], [ 0, %if.else31 ]
  %121 = load ptr, ptr %__panic, align 8
  %122 = load i8, ptr %121, align 1
  %123 = icmp ne i8 %122, 0
  br i1 %123, label %panic.take34, label %panic.cont35

panic.take34:                                     ; preds = %if.merge32
  %124 = load ptr, ptr %__panic, align 8
  %125 = getelementptr i8, ptr %124, i64 4
  %126 = load i32, ptr %125, align 4
  ret i32 %126

panic.cont35:                                     ; preds = %if.merge32
  ret i32 %53
}

define internal void @__cx_spawn_body_StructuredParallelLoweringEvidence_0(ptr %__uv_host_env, ptr noundef nonnull align 1 %env, ptr noundef nonnull align 4 dereferenceable(4) %result, ptr %__panic) {
entry:
  %abi_return6 = alloca { i64, i64 }, align 8
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
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
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
  %9 = load ptr, ptr %result3, align 8
  store i32 41, ptr %9, align 4
  %10 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$05")
  store { i64, i64 } %10, ptr %abi_return6, align 8
  %11 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return6, align 1
  ret void
}

define internal void @__cx_dispatch_body_StructuredParallelLoweringEvidence_1(ptr %__uv_host_env, ptr noundef nonnull align 8 dereferenceable(8) %elem, ptr noundef nonnull align 1 %env, ptr noundef nonnull align 4 dereferenceable(4) %result, ptr %__panic) {
entry:
  %abi_return7 = alloca { i64, i64 }, align 8
  %index = alloca i64, align 8
  %"region$06" = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %aggregate.literal = alloca { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] }, align 8
  %"region$0" = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %__panic5 = alloca ptr, align 8
  %result4 = alloca ptr, align 8
  %env3 = alloca ptr, align 8
  %elem2 = alloca ptr, align 8
  %__uv_host_env1 = alloca ptr, align 8
  store ptr %__uv_host_env, ptr %__uv_host_env1, align 8
  store ptr %elem, ptr %elem2, align 8
  store ptr %env, ptr %env3, align 8
  store ptr %result, ptr %result4, align 8
  store ptr %__panic, ptr %__panic5, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic5, align 8
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
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %8, ptr %"region$06", align 4
  %9 = load ptr, ptr %elem2, align 8
  %10 = load i64, ptr %9, align 4
  store i64 %10, ptr %index, align 4
  %11 = load ptr, ptr %result4, align 8
  store i32 7, ptr %11, align 4
  %12 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$06")
  store { i64, i64 } %12, ptr %abi_return7, align 8
  %13 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return7, align 1
  ret void
}

define i32 @StructuredParallelLoweringEvidence_x3a_x3amidExecutionCancellationRuntimeEvidence(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %byref_arg = alloca i32, align 4
  %spawned = alloca ptr, align 8
  %"StructuredParallelLoweringEvidence_x3a_x3amidExecutionCancellationRuntimeEvidence$tmp$spawn_env_storage_104" = alloca { ptr }, align 8
  %coerce_bits4 = alloca { ptr, ptr }, align 8
  %coerce_bits = alloca { i64, [0 x i64] }, align 8
  %abi_return3 = alloca { i64, i64 }, align 8
  %child2 = alloca { i64, [0 x i64] }, align 8
  %abi_return1 = alloca i64, align 8
  %child = alloca { i64, [0 x i64] }, align 8
  %token = alloca { i64, [0 x i64] }, align 8
  %abi_return = alloca i64, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
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
  %11 = call i64 @CancelToken_x3a_x3aActive_x3a_x3achild(ptr %token)
  store i64 %11, ptr %abi_return1, align 8
  %12 = load { i64, [0 x i64] }, ptr %abi_return1, align 1
  store { i64, [0 x i64] } %12, ptr %child2, align 4
  %13 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3ainline(ptr noundef nonnull align 8 dereferenceable(96) %context)
  store { i64, i64 } %13, ptr %abi_return3, align 8
  %14 = load { ptr, ptr }, ptr %abi_return3, align 1
  %15 = load { i64, [0 x i64] }, ptr %child2, align 4
  store { i64, [0 x i64] } %15, ptr %coerce_bits, align 8
  %16 = load i64, ptr %coerce_bits, align 1
  store { ptr, ptr } %14, ptr %coerce_bits4, align 8
  %17 = load { i64, i64 }, ptr %coerce_bits4, align 1
  %18 = call ptr @uv_parallel_begin({ i64, i64 } %17, i64 %16, ptr null)
  store { ptr } zeroinitializer, ptr %"StructuredParallelLoweringEvidence_x3a_x3amidExecutionCancellationRuntimeEvidence$tmp$spawn_env_storage_104", align 8
  %19 = getelementptr i8, ptr %"StructuredParallelLoweringEvidence_x3a_x3amidExecutionCancellationRuntimeEvidence$tmp$spawn_env_storage_104", i64 0
  store ptr %child2, ptr %19, align 8
  %20 = call ptr @uv_spawn_create(ptr %"StructuredParallelLoweringEvidence_x3a_x3amidExecutionCancellationRuntimeEvidence$tmp$spawn_env_storage_104", i64 8, ptr @__cx_spawn_body_StructuredParallelLoweringEvidence_2, ptr null, i64 4, i64 0, i32 -1)
  store ptr %20, ptr %spawned, align 8
  %21 = load ptr, ptr %spawned, align 8
  %22 = call ptr @uv_spawn_wait(ptr %21)
  %23 = load i32, ptr %22, align 4
  %24 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %24, align 1
  %25 = getelementptr i8, ptr %24, i64 4
  store i32 0, ptr %25, align 4
  %26 = call i32 @uv_parallel_join(ptr %18)
  %27 = icmp eq i32 %26, 0
  %28 = xor i1 %27, true
  %29 = zext i1 %28 to i8
  %30 = icmp ne i8 %29, 0
  %31 = icmp ne i8 %29, 0
  br i1 %31, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  %32 = load ptr, ptr %__panic, align 8
  %33 = getelementptr i8, ptr %32, i64 0
  store i8 1, ptr %33, align 1
  %34 = load ptr, ptr %__panic, align 8
  %35 = getelementptr i8, ptr %34, i64 4
  store i32 %26, ptr %35, align 4
  br label %if.merge

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %if.result = phi i64 [ 0, %if.then ], [ 0, %if.else ]
  %36 = load ptr, ptr %__panic, align 8
  %37 = getelementptr i8, ptr %36, i64 0
  %38 = load i8, ptr %37, align 1
  %39 = load ptr, ptr %__panic, align 8
  %40 = getelementptr i8, ptr %39, i64 4
  %41 = load i32, ptr %40, align 4
  %42 = icmp ne i8 %38, 0
  %43 = zext i1 %42 to i8
  %44 = icmp ne i8 %43, 0
  %45 = and i1 false, %44
  br i1 %45, label %if.then5, label %if.else6

if.then5:                                         ; preds = %if.merge
  store i32 %41, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else6:                                         ; preds = %if.merge
  br label %if.merge7

if.merge7:                                        ; preds = %if.else6
  %46 = icmp ne i8 %38, 0
  %47 = xor i1 %46, true
  %48 = and i1 false, %47
  br i1 %48, label %if.then8, label %if.else9

if.then8:                                         ; preds = %if.merge7
  %49 = load ptr, ptr %__panic, align 8
  %50 = getelementptr i8, ptr %49, i64 0
  store i8 1, ptr %50, align 1
  %51 = load ptr, ptr %__panic, align 8
  %52 = getelementptr i8, ptr %51, i64 4
  store i32 0, ptr %52, align 4
  br label %if.merge10

if.else9:                                         ; preds = %if.merge7
  br label %if.merge10

if.merge10:                                       ; preds = %if.else9, %if.then8
  %if.result11 = phi i64 [ 0, %if.then8 ], [ 0, %if.else9 ]
  %53 = icmp ne i8 %38, 0
  %54 = zext i1 %53 to i8
  %55 = icmp ne i8 %54, 0
  %56 = or i1 false, %55
  %57 = icmp ne i8 %38, 0
  %58 = icmp ne i8 %38, 0
  br i1 %58, label %if.then12, label %if.else13

if.then12:                                        ; preds = %if.merge10
  br label %if.merge14

if.else13:                                        ; preds = %if.merge10
  br label %if.merge14

if.merge14:                                       ; preds = %if.else13, %if.then12
  %if.result15 = phi i32 [ %41, %if.then12 ], [ 0, %if.else13 ]
  %59 = load ptr, ptr %__panic, align 8
  %60 = load i8, ptr %59, align 1
  %61 = icmp ne i8 %60, 0
  br i1 %61, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %if.merge14
  %62 = load ptr, ptr %__panic, align 8
  %63 = getelementptr i8, ptr %62, i64 4
  %64 = load i32, ptr %63, align 4
  ret i32 %64

panic.cont:                                       ; preds = %if.merge14
  ret i32 %23
}

define internal void @__cx_spawn_body_StructuredParallelLoweringEvidence_2(ptr %__uv_host_env, ptr noundef nonnull align 8 dereferenceable(8) %env, ptr noundef nonnull align 4 dereferenceable(4) %result, ptr %__panic) {
entry:
  %abi_return6 = alloca { i64, i64 }, align 8
  %sret = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
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
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
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
  %9 = load ptr, ptr %env2, align 8
  %10 = getelementptr i8, ptr %9, i64 0
  %11 = load ptr, ptr %10, align 8
  call void @CancelToken_x3a_x3aActive_x3a_x3await_x5fcancelled(ptr noundef nonnull align 8 dereferenceable(24) %sret, ptr %11)
  %12 = load ptr, ptr %env2, align 8
  %13 = getelementptr i8, ptr %12, i64 0
  %14 = load ptr, ptr %13, align 8
  call void @CancelToken_x3a_x3aActive_x3a_x3acancel(ptr %14)
  %15 = load ptr, ptr %env2, align 8
  %16 = getelementptr i8, ptr %15, i64 0
  %17 = load ptr, ptr %16, align 8
  %18 = call i8 @CancelToken_x3a_x3aActive_x3a_x3ais_x5fcancelled(ptr %17)
  %19 = trunc i8 %18 to i1
  br i1 %19, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  br label %if.merge

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %"StructuredParallelLoweringEvidence_x3a_x3amidExecutionCancellationRuntimeEvidence$tmp$if_122" = phi i32 [ 5, %if.then ], [ 99, %if.else ]
  %20 = load ptr, ptr %result3, align 8
  store i32 %"StructuredParallelLoweringEvidence_x3a_x3amidExecutionCancellationRuntimeEvidence$tmp$if_122", ptr %20, align 4
  %21 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$05")
  store { i64, i64 } %21, ptr %abi_return6, align 8
  %22 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return6, align 1
  ret void
}

define i32 @StructuredParallelLoweringEvidence_x3a_x3acancellationLoweringEvidence(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
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
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
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
  %23 = call i32 @StructuredParallelLoweringEvidence_x3a_x3aqueuedCancellationRuntimeEvidence(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %24 = load ptr, ptr %__panic, align 8
  %25 = load i8, ptr %24, align 1
  %26 = icmp ne i8 %25, 0
  br i1 %26, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 4
  %29 = load i32, ptr %28, align 4
  ret i32 %29

panic.cont:                                       ; preds = %poison.cont4
  %30 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
  %31 = icmp ne i8 %30, 0
  br i1 %31, label %poison.take5, label %poison.cont6

poison.take5:                                     ; preds = %panic.cont
  %32 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %32, align 1
  %33 = getelementptr i8, ptr %32, i64 4
  store i32 10, ptr %33, align 4
  %34 = load ptr, ptr %__panic, align 8
  %35 = getelementptr i8, ptr %34, i64 4
  %36 = load i32, ptr %35, align 4
  ret i32 %36

poison.cont6:                                     ; preds = %panic.cont
  %37 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
  %38 = icmp ne i8 %37, 0
  br i1 %38, label %poison.take7, label %poison.cont8

poison.take7:                                     ; preds = %poison.cont6
  %39 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %39, align 1
  %40 = getelementptr i8, ptr %39, i64 4
  store i32 10, ptr %40, align 4
  %41 = load ptr, ptr %__panic, align 8
  %42 = getelementptr i8, ptr %41, i64 4
  %43 = load i32, ptr %42, align 4
  ret i32 %43

poison.cont8:                                     ; preds = %poison.cont6
  %44 = call i32 @StructuredParallelLoweringEvidence_x3a_x3amidExecutionCancellationRuntimeEvidence(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %45 = load ptr, ptr %__panic, align 8
  %46 = load i8, ptr %45, align 1
  %47 = icmp ne i8 %46, 0
  br i1 %47, label %panic.take9, label %panic.cont10

panic.take9:                                      ; preds = %poison.cont8
  %48 = load ptr, ptr %__panic, align 8
  %49 = getelementptr i8, ptr %48, i64 4
  %50 = load i32, ptr %49, align 4
  ret i32 %50

panic.cont10:                                     ; preds = %poison.cont8
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %panic.cont10
  %51 = load ptr, ptr %__panic, align 8
  %52 = load i8, ptr %51, align 1
  %53 = icmp ne i8 %52, 0
  br i1 %53, label %panic.take11, label %panic.cont12

check_fail:                                       ; preds = %panic.cont10
  %54 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %54, align 1
  %55 = getelementptr i8, ptr %54, i64 4
  store i32 4, ptr %55, align 4
  br label %check_ok

panic.take11:                                     ; preds = %check_ok
  %56 = load ptr, ptr %__panic, align 8
  %57 = getelementptr i8, ptr %56, i64 4
  %58 = load i32, ptr %57, align 4
  ret i32 %58

panic.cont12:                                     ; preds = %check_ok
  %59 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %23, i32 %44)
  %60 = extractvalue { i32, i1 } %59, 0
  %61 = extractvalue { i32, i1 } %59, 1
  %62 = freeze i32 %60
  br i1 %61, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont12
  ret i32 %62

op_fail:                                          ; preds = %panic.cont12
  %63 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %63, align 1
  %64 = getelementptr i8, ptr %63, i64 4
  store i32 4, ptr %64, align 4
  ret i32 0
}

define i32 @StructuredParallelLoweringEvidence_x3a_x3anestedParallelLoweringEvidence(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %byref_arg = alloca i32, align 4
  %coerce_bits2 = alloca { ptr, ptr }, align 8
  %abi_return1 = alloca { i64, i64 }, align 8
  %coerce_bits = alloca { ptr, ptr }, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
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
  %9 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3ainline(ptr noundef nonnull align 8 dereferenceable(96) %context)
  store { i64, i64 } %9, ptr %abi_return, align 8
  %10 = load { ptr, ptr }, ptr %abi_return, align 1
  store { ptr, ptr } %10, ptr %coerce_bits, align 8
  %11 = load { i64, i64 }, ptr %coerce_bits, align 1
  %12 = call ptr @uv_parallel_begin({ i64, i64 } %11, i64 -1, ptr null)
  %13 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3ainline(ptr noundef nonnull align 8 dereferenceable(96) %context)
  store { i64, i64 } %13, ptr %abi_return1, align 8
  %14 = load { ptr, ptr }, ptr %abi_return1, align 1
  store { ptr, ptr } %14, ptr %coerce_bits2, align 8
  %15 = load { i64, i64 }, ptr %coerce_bits2, align 1
  %16 = call ptr @uv_parallel_begin({ i64, i64 } %15, i64 -1, ptr null)
  %17 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %17, align 1
  %18 = getelementptr i8, ptr %17, i64 4
  store i32 0, ptr %18, align 4
  %19 = call i32 @uv_parallel_join(ptr %16)
  %20 = icmp eq i32 %19, 0
  %21 = xor i1 %20, true
  %22 = zext i1 %21 to i8
  %23 = icmp ne i8 %22, 0
  %24 = icmp ne i8 %22, 0
  br i1 %24, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  %25 = load ptr, ptr %__panic, align 8
  %26 = getelementptr i8, ptr %25, i64 0
  store i8 1, ptr %26, align 1
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 4
  store i32 %19, ptr %28, align 4
  br label %if.merge

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %if.result = phi i64 [ 0, %if.then ], [ 0, %if.else ]
  %29 = load ptr, ptr %__panic, align 8
  %30 = getelementptr i8, ptr %29, i64 0
  %31 = load i8, ptr %30, align 1
  %32 = load ptr, ptr %__panic, align 8
  %33 = getelementptr i8, ptr %32, i64 4
  %34 = load i32, ptr %33, align 4
  %35 = icmp ne i8 %31, 0
  %36 = zext i1 %35 to i8
  %37 = icmp ne i8 %36, 0
  %38 = and i1 false, %37
  br i1 %38, label %if.then3, label %if.else4

if.then3:                                         ; preds = %if.merge
  store i32 %34, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else4:                                         ; preds = %if.merge
  br label %if.merge5

if.merge5:                                        ; preds = %if.else4
  %39 = icmp ne i8 %31, 0
  %40 = xor i1 %39, true
  %41 = and i1 false, %40
  br i1 %41, label %if.then6, label %if.else7

if.then6:                                         ; preds = %if.merge5
  %42 = load ptr, ptr %__panic, align 8
  %43 = getelementptr i8, ptr %42, i64 0
  store i8 1, ptr %43, align 1
  %44 = load ptr, ptr %__panic, align 8
  %45 = getelementptr i8, ptr %44, i64 4
  store i32 0, ptr %45, align 4
  br label %if.merge8

if.else7:                                         ; preds = %if.merge5
  br label %if.merge8

if.merge8:                                        ; preds = %if.else7, %if.then6
  %if.result9 = phi i64 [ 0, %if.then6 ], [ 0, %if.else7 ]
  %46 = icmp ne i8 %31, 0
  %47 = zext i1 %46 to i8
  %48 = icmp ne i8 %47, 0
  %49 = or i1 false, %48
  %50 = icmp ne i8 %31, 0
  %51 = icmp ne i8 %31, 0
  br i1 %51, label %if.then10, label %if.else11

if.then10:                                        ; preds = %if.merge8
  br label %if.merge12

if.else11:                                        ; preds = %if.merge8
  br label %if.merge12

if.merge12:                                       ; preds = %if.else11, %if.then10
  %if.result13 = phi i32 [ %34, %if.then10 ], [ 0, %if.else11 ]
  %52 = load ptr, ptr %__panic, align 8
  %53 = load i8, ptr %52, align 1
  %54 = icmp ne i8 %53, 0
  br i1 %54, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %if.merge12
  %55 = load ptr, ptr %__panic, align 8
  %56 = getelementptr i8, ptr %55, i64 4
  %57 = load i32, ptr %56, align 4
  ret i32 %57

panic.cont:                                       ; preds = %if.merge12
  %58 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %58, align 1
  %59 = getelementptr i8, ptr %58, i64 4
  store i32 0, ptr %59, align 4
  %60 = call i32 @uv_parallel_join(ptr %12)
  %61 = icmp eq i32 %60, 0
  %62 = xor i1 %61, true
  %63 = zext i1 %62 to i8
  %64 = icmp ne i8 %63, 0
  %65 = icmp ne i8 %63, 0
  br i1 %65, label %if.then14, label %if.else15

if.then14:                                        ; preds = %panic.cont
  %66 = load ptr, ptr %__panic, align 8
  %67 = getelementptr i8, ptr %66, i64 0
  store i8 1, ptr %67, align 1
  %68 = load ptr, ptr %__panic, align 8
  %69 = getelementptr i8, ptr %68, i64 4
  store i32 %60, ptr %69, align 4
  br label %if.merge16

if.else15:                                        ; preds = %panic.cont
  br label %if.merge16

if.merge16:                                       ; preds = %if.else15, %if.then14
  %if.result17 = phi i64 [ 0, %if.then14 ], [ 0, %if.else15 ]
  %70 = load ptr, ptr %__panic, align 8
  %71 = getelementptr i8, ptr %70, i64 0
  %72 = load i8, ptr %71, align 1
  %73 = load ptr, ptr %__panic, align 8
  %74 = getelementptr i8, ptr %73, i64 4
  %75 = load i32, ptr %74, align 4
  %76 = icmp ne i8 %72, 0
  %77 = zext i1 %76 to i8
  %78 = icmp ne i8 %77, 0
  %79 = and i1 false, %78
  br i1 %79, label %if.then18, label %if.else19

if.then18:                                        ; preds = %if.merge16
  store i32 %75, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else19:                                        ; preds = %if.merge16
  br label %if.merge20

if.merge20:                                       ; preds = %if.else19
  %80 = icmp ne i8 %72, 0
  %81 = xor i1 %80, true
  %82 = and i1 false, %81
  br i1 %82, label %if.then21, label %if.else22

if.then21:                                        ; preds = %if.merge20
  %83 = load ptr, ptr %__panic, align 8
  %84 = getelementptr i8, ptr %83, i64 0
  store i8 1, ptr %84, align 1
  %85 = load ptr, ptr %__panic, align 8
  %86 = getelementptr i8, ptr %85, i64 4
  store i32 0, ptr %86, align 4
  br label %if.merge23

if.else22:                                        ; preds = %if.merge20
  br label %if.merge23

if.merge23:                                       ; preds = %if.else22, %if.then21
  %if.result24 = phi i64 [ 0, %if.then21 ], [ 0, %if.else22 ]
  %87 = icmp ne i8 %72, 0
  %88 = zext i1 %87 to i8
  %89 = icmp ne i8 %88, 0
  %90 = or i1 false, %89
  %91 = icmp ne i8 %72, 0
  %92 = icmp ne i8 %72, 0
  br i1 %92, label %if.then25, label %if.else26

if.then25:                                        ; preds = %if.merge23
  br label %if.merge27

if.else26:                                        ; preds = %if.merge23
  br label %if.merge27

if.merge27:                                       ; preds = %if.else26, %if.then25
  %if.result28 = phi i32 [ %75, %if.then25 ], [ 0, %if.else26 ]
  %93 = load ptr, ptr %__panic, align 8
  %94 = load i8, ptr %93, align 1
  %95 = icmp ne i8 %94, 0
  br i1 %95, label %panic.take29, label %panic.cont30

panic.take29:                                     ; preds = %if.merge27
  %96 = load ptr, ptr %__panic, align 8
  %97 = getelementptr i8, ptr %96, i64 4
  %98 = load i32, ptr %97, align 4
  ret i32 %98

panic.cont30:                                     ; preds = %if.merge27
  ret i32 8
}

define i32 @StructuredParallelLoweringEvidence_x3a_x3aoperatorUndefinedPanicValue(ptr noundef nonnull align 4 dereferenceable(4) %divisor, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %numerator = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
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
  store i32 42, ptr %numerator, align 4
  %9 = load i32, ptr %divisor, align 4
  %10 = icmp ne i32 %9, 0
  br i1 %10, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %11 = load ptr, ptr %__panic, align 8
  %12 = load i8, ptr %11, align 1
  %13 = icmp ne i8 %12, 0
  br i1 %13, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %14 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %14, align 1
  %15 = getelementptr i8, ptr %14, i64 4
  store i32 3, ptr %15, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %16 = load ptr, ptr %__panic, align 8
  %17 = getelementptr i8, ptr %16, i64 4
  %18 = load i32, ptr %17, align 4
  ret i32 %18

panic.cont:                                       ; preds = %check_ok
  %19 = load i32, ptr %numerator, align 4
  %20 = load i32, ptr %divisor, align 4
  %21 = icmp ne i32 %20, 0
  br i1 %21, label %check_ok1, label %check_fail2

check_ok1:                                        ; preds = %panic.cont
  %22 = icmp eq i32 %19, -2147483648
  %23 = icmp eq i32 %20, -1
  %24 = and i1 %22, %23
  %25 = xor i1 %24, true
  br i1 %25, label %check_ok3, label %check_fail4

check_fail2:                                      ; preds = %panic.cont
  %26 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %26, align 1
  %27 = getelementptr i8, ptr %26, i64 4
  store i32 3, ptr %27, align 4
  %28 = load ptr, ptr %__panic, align 8
  %29 = getelementptr i8, ptr %28, i64 4
  %30 = load i32, ptr %29, align 4
  ret i32 %30

check_ok3:                                        ; preds = %check_ok1
  %31 = sdiv i32 %19, %20
  %32 = freeze i32 %31
  ret i32 %32

check_fail4:                                      ; preds = %check_ok1
  %33 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %33, align 1
  %34 = getelementptr i8, ptr %33, i64 4
  store i32 4, ptr %34, align 4
  %35 = load ptr, ptr %__panic, align 8
  %36 = getelementptr i8, ptr %35, i64 4
  %37 = load i32, ptr %36, align 4
  ret i32 %37
}

define i32 @StructuredParallelLoweringEvidence_x3a_x3aintegerOverflowPanicValue(ptr noundef nonnull align 4 dereferenceable(4) %delta, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %maximum = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
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
  store i32 2147483647, ptr %maximum, align 4
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
  %17 = load i32, ptr %maximum, align 4
  %18 = load i32, ptr %delta, align 4
  %19 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %17, i32 %18)
  %20 = extractvalue { i32, i1 } %19, 0
  %21 = extractvalue { i32, i1 } %19, 1
  %22 = freeze i32 %20
  br i1 %21, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  ret i32 %22

op_fail:                                          ; preds = %panic.cont
  %23 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %23, align 1
  %24 = getelementptr i8, ptr %23, i64 4
  store i32 4, ptr %24, align 4
  ret i32 0
}

define i32 @StructuredParallelLoweringEvidence_x3a_x3aparallelSpawnPanicEvidence(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %byref_arg = alloca i32, align 4
  %second = alloca ptr, align 8
  %"StructuredParallelLoweringEvidence_x3a_x3aparallelSpawnPanicEvidence$tmp$spawn_env_storage_250" = alloca {}, align 1
  %first = alloca ptr, align 8
  %"StructuredParallelLoweringEvidence_x3a_x3aparallelSpawnPanicEvidence$tmp$spawn_env_storage_228" = alloca {}, align 1
  %coerce_bits4 = alloca { ptr, ptr }, align 8
  %coerce_bits = alloca { i64, [0 x i64] }, align 8
  %abi_return3 = alloca { i64, i64 }, align 8
  %child2 = alloca { i64, [0 x i64] }, align 8
  %abi_return1 = alloca i64, align 8
  %child = alloca { i64, [0 x i64] }, align 8
  %token = alloca { i64, [0 x i64] }, align 8
  %abi_return = alloca i64, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
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
  %11 = call i64 @CancelToken_x3a_x3aActive_x3a_x3achild(ptr %token)
  store i64 %11, ptr %abi_return1, align 8
  %12 = load { i64, [0 x i64] }, ptr %abi_return1, align 1
  store { i64, [0 x i64] } %12, ptr %child2, align 4
  %13 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3ainline(ptr noundef nonnull align 8 dereferenceable(96) %context)
  store { i64, i64 } %13, ptr %abi_return3, align 8
  %14 = load { ptr, ptr }, ptr %abi_return3, align 1
  %15 = load { i64, [0 x i64] }, ptr %child2, align 4
  store { i64, [0 x i64] } %15, ptr %coerce_bits, align 8
  %16 = load i64, ptr %coerce_bits, align 1
  store { ptr, ptr } %14, ptr %coerce_bits4, align 8
  %17 = load { i64, i64 }, ptr %coerce_bits4, align 1
  %18 = call ptr @uv_parallel_begin({ i64, i64 } %17, i64 %16, ptr null)
  store {} zeroinitializer, ptr %"StructuredParallelLoweringEvidence_x3a_x3aparallelSpawnPanicEvidence$tmp$spawn_env_storage_228", align 1
  %19 = call ptr @uv_spawn_create(ptr %"StructuredParallelLoweringEvidence_x3a_x3aparallelSpawnPanicEvidence$tmp$spawn_env_storage_228", i64 0, ptr @__cx_spawn_body_StructuredParallelLoweringEvidence_3, ptr null, i64 4, i64 0, i32 -1)
  store ptr %19, ptr %first, align 8
  store {} zeroinitializer, ptr %"StructuredParallelLoweringEvidence_x3a_x3aparallelSpawnPanicEvidence$tmp$spawn_env_storage_250", align 1
  %20 = call ptr @uv_spawn_create(ptr %"StructuredParallelLoweringEvidence_x3a_x3aparallelSpawnPanicEvidence$tmp$spawn_env_storage_250", i64 0, ptr @__cx_spawn_body_StructuredParallelLoweringEvidence_4, ptr null, i64 4, i64 0, i32 -1)
  store ptr %20, ptr %second, align 8
  %21 = load ptr, ptr %first, align 8
  %22 = call ptr @uv_spawn_wait(ptr %21)
  %23 = load i32, ptr %22, align 4
  %24 = load ptr, ptr %second, align 8
  %25 = call ptr @uv_spawn_wait(ptr %24)
  %26 = load i32, ptr %25, align 4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %27 = load ptr, ptr %__panic, align 8
  %28 = load i8, ptr %27, align 1
  %29 = icmp ne i8 %28, 0
  br i1 %29, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %30 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %30, align 1
  %31 = getelementptr i8, ptr %30, i64 4
  store i32 4, ptr %31, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %32 = load ptr, ptr %__panic, align 8
  %33 = getelementptr i8, ptr %32, i64 0
  %34 = load i8, ptr %33, align 1
  %35 = load ptr, ptr %__panic, align 8
  %36 = getelementptr i8, ptr %35, i64 4
  %37 = load i32, ptr %36, align 4
  %38 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %38, align 1
  %39 = getelementptr i8, ptr %38, i64 4
  store i32 0, ptr %39, align 4
  %40 = call i32 @uv_parallel_join(ptr %18)
  %41 = icmp eq i32 %40, 0
  %42 = xor i1 %41, true
  %43 = zext i1 %42 to i8
  %44 = icmp ne i8 %43, 0
  %45 = icmp ne i8 %43, 0
  br i1 %45, label %if.then, label %if.else

panic.cont:                                       ; preds = %check_ok
  %46 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %23, i32 %26)
  %47 = extractvalue { i32, i1 } %46, 0
  %48 = extractvalue { i32, i1 } %46, 1
  %49 = freeze i32 %47
  br i1 %48, label %op_fail, label %op_ok

if.then:                                          ; preds = %panic.take
  %50 = load ptr, ptr %__panic, align 8
  %51 = getelementptr i8, ptr %50, i64 0
  store i8 1, ptr %51, align 1
  %52 = load ptr, ptr %__panic, align 8
  %53 = getelementptr i8, ptr %52, i64 4
  store i32 %40, ptr %53, align 4
  br label %if.merge

if.else:                                          ; preds = %panic.take
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %if.result = phi i64 [ 0, %if.then ], [ 0, %if.else ]
  %54 = load ptr, ptr %__panic, align 8
  %55 = getelementptr i8, ptr %54, i64 0
  %56 = load i8, ptr %55, align 1
  %57 = load ptr, ptr %__panic, align 8
  %58 = getelementptr i8, ptr %57, i64 4
  %59 = load i32, ptr %58, align 4
  %60 = icmp ne i8 %56, 0
  %61 = zext i1 %60 to i8
  %62 = icmp ne i8 %61, 0
  %63 = and i1 true, %62
  br i1 %63, label %if.then5, label %if.else6

if.then5:                                         ; preds = %if.merge
  store i32 %59, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else6:                                         ; preds = %if.merge
  br label %if.merge7

if.merge7:                                        ; preds = %if.else6
  %64 = icmp ne i8 %56, 0
  %65 = xor i1 %64, true
  %66 = and i1 true, %65
  br i1 %66, label %if.then8, label %if.else9

if.then8:                                         ; preds = %if.merge7
  %67 = load ptr, ptr %__panic, align 8
  %68 = getelementptr i8, ptr %67, i64 0
  store i8 1, ptr %68, align 1
  %69 = load ptr, ptr %__panic, align 8
  %70 = getelementptr i8, ptr %69, i64 4
  store i32 %37, ptr %70, align 4
  br label %if.merge10

if.else9:                                         ; preds = %if.merge7
  br label %if.merge10

if.merge10:                                       ; preds = %if.else9, %if.then8
  %if.result11 = phi i64 [ 0, %if.then8 ], [ 0, %if.else9 ]
  %71 = icmp ne i8 %56, 0
  %72 = zext i1 %71 to i8
  %73 = icmp ne i8 %72, 0
  %74 = or i1 true, %73
  %75 = icmp ne i8 %56, 0
  %76 = icmp ne i8 %56, 0
  br i1 %76, label %if.then12, label %if.else13

if.then12:                                        ; preds = %if.merge10
  br label %if.merge14

if.else13:                                        ; preds = %if.merge10
  br label %if.merge14

if.merge14:                                       ; preds = %if.else13, %if.then12
  %if.result15 = phi i32 [ %59, %if.then12 ], [ %37, %if.else13 ]
  %77 = load ptr, ptr %__panic, align 8
  %78 = getelementptr i8, ptr %77, i64 4
  %79 = load i32, ptr %78, align 4
  ret i32 %79

op_ok:                                            ; preds = %panic.cont
  %80 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %80, align 1
  %81 = getelementptr i8, ptr %80, i64 4
  store i32 0, ptr %81, align 4
  %82 = call i32 @uv_parallel_join(ptr %18)
  %83 = icmp eq i32 %82, 0
  %84 = xor i1 %83, true
  %85 = zext i1 %84 to i8
  %86 = icmp ne i8 %85, 0
  %87 = icmp ne i8 %85, 0
  br i1 %87, label %if.then16, label %if.else17

op_fail:                                          ; preds = %panic.cont
  %88 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %88, align 1
  %89 = getelementptr i8, ptr %88, i64 4
  store i32 4, ptr %89, align 4
  ret i32 0

if.then16:                                        ; preds = %op_ok
  %90 = load ptr, ptr %__panic, align 8
  %91 = getelementptr i8, ptr %90, i64 0
  store i8 1, ptr %91, align 1
  %92 = load ptr, ptr %__panic, align 8
  %93 = getelementptr i8, ptr %92, i64 4
  store i32 %82, ptr %93, align 4
  br label %if.merge18

if.else17:                                        ; preds = %op_ok
  br label %if.merge18

if.merge18:                                       ; preds = %if.else17, %if.then16
  %if.result19 = phi i64 [ 0, %if.then16 ], [ 0, %if.else17 ]
  %94 = load ptr, ptr %__panic, align 8
  %95 = getelementptr i8, ptr %94, i64 0
  %96 = load i8, ptr %95, align 1
  %97 = load ptr, ptr %__panic, align 8
  %98 = getelementptr i8, ptr %97, i64 4
  %99 = load i32, ptr %98, align 4
  %100 = icmp ne i8 %96, 0
  %101 = zext i1 %100 to i8
  %102 = icmp ne i8 %101, 0
  %103 = and i1 false, %102
  br i1 %103, label %if.then20, label %if.else21

if.then20:                                        ; preds = %if.merge18
  store i32 %99, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else21:                                        ; preds = %if.merge18
  br label %if.merge22

if.merge22:                                       ; preds = %if.else21
  %104 = icmp ne i8 %96, 0
  %105 = xor i1 %104, true
  %106 = and i1 false, %105
  br i1 %106, label %if.then23, label %if.else24

if.then23:                                        ; preds = %if.merge22
  %107 = load ptr, ptr %__panic, align 8
  %108 = getelementptr i8, ptr %107, i64 0
  store i8 1, ptr %108, align 1
  %109 = load ptr, ptr %__panic, align 8
  %110 = getelementptr i8, ptr %109, i64 4
  store i32 0, ptr %110, align 4
  br label %if.merge25

if.else24:                                        ; preds = %if.merge22
  br label %if.merge25

if.merge25:                                       ; preds = %if.else24, %if.then23
  %if.result26 = phi i64 [ 0, %if.then23 ], [ 0, %if.else24 ]
  %111 = icmp ne i8 %96, 0
  %112 = zext i1 %111 to i8
  %113 = icmp ne i8 %112, 0
  %114 = or i1 false, %113
  %115 = icmp ne i8 %96, 0
  %116 = icmp ne i8 %96, 0
  br i1 %116, label %if.then27, label %if.else28

if.then27:                                        ; preds = %if.merge25
  br label %if.merge29

if.else28:                                        ; preds = %if.merge25
  br label %if.merge29

if.merge29:                                       ; preds = %if.else28, %if.then27
  %if.result30 = phi i32 [ %99, %if.then27 ], [ 0, %if.else28 ]
  %117 = load ptr, ptr %__panic, align 8
  %118 = load i8, ptr %117, align 1
  %119 = icmp ne i8 %118, 0
  br i1 %119, label %panic.take31, label %panic.cont32

panic.take31:                                     ; preds = %if.merge29
  %120 = load ptr, ptr %__panic, align 8
  %121 = getelementptr i8, ptr %120, i64 4
  %122 = load i32, ptr %121, align 4
  ret i32 %122

panic.cont32:                                     ; preds = %if.merge29
  ret i32 %49
}

define internal void @__cx_spawn_body_StructuredParallelLoweringEvidence_3(ptr %__uv_host_env, ptr noundef nonnull align 1 %env, ptr noundef nonnull align 4 dereferenceable(4) %result, ptr %__panic) {
entry:
  %abi_return11 = alloca { i64, i64 }, align 8
  %abi_return10 = alloca { i64, i64 }, align 8
  %"StructuredParallelLoweringEvidence_x3a_x3aparallelSpawnPanicEvidence$tmp$call_ref_tmp_234" = alloca i32, align 4
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
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
  %10 = icmp ne i8 %9, 0
  br i1 %10, label %poison.take6, label %poison.cont7

poison.take6:                                     ; preds = %poison.cont
  %11 = load ptr, ptr %__panic4, align 8
  store i8 1, ptr %11, align 1
  %12 = getelementptr i8, ptr %11, i64 4
  store i32 10, ptr %12, align 4
  ret void

poison.cont7:                                     ; preds = %poison.cont
  store i32 0, ptr %"StructuredParallelLoweringEvidence_x3a_x3aparallelSpawnPanicEvidence$tmp$call_ref_tmp_234", align 4
  %13 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
  %14 = icmp ne i8 %13, 0
  br i1 %14, label %poison.take8, label %poison.cont9

poison.take8:                                     ; preds = %poison.cont7
  %15 = load ptr, ptr %__panic4, align 8
  store i8 1, ptr %15, align 1
  %16 = getelementptr i8, ptr %15, i64 4
  store i32 10, ptr %16, align 4
  ret void

poison.cont9:                                     ; preds = %poison.cont7
  %17 = call i32 @StructuredParallelLoweringEvidence_x3a_x3aoperatorUndefinedPanicValue(ptr noundef nonnull align 4 dereferenceable(4) %"StructuredParallelLoweringEvidence_x3a_x3aparallelSpawnPanicEvidence$tmp$call_ref_tmp_234", ptr noundef nonnull align 8 dereferenceable(8) %__panic4)
  %18 = load ptr, ptr %__panic4, align 8
  %19 = load i8, ptr %18, align 1
  %20 = icmp ne i8 %19, 0
  br i1 %20, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont9
  %21 = load ptr, ptr %__panic4, align 8
  %22 = getelementptr i8, ptr %21, i64 0
  %23 = load i8, ptr %22, align 1
  %24 = load ptr, ptr %__panic4, align 8
  %25 = getelementptr i8, ptr %24, i64 4
  %26 = load i32, ptr %25, align 4
  %27 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$05")
  store { i64, i64 } %27, ptr %abi_return10, align 8
  %28 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return10, align 1
  ret void

panic.cont:                                       ; preds = %poison.cont9
  %29 = load ptr, ptr %result3, align 8
  store i32 %17, ptr %29, align 4
  %30 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$05")
  store { i64, i64 } %30, ptr %abi_return11, align 8
  %31 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return11, align 1
  ret void
}

define internal void @__cx_spawn_body_StructuredParallelLoweringEvidence_4(ptr %__uv_host_env, ptr noundef nonnull align 1 %env, ptr noundef nonnull align 4 dereferenceable(4) %result, ptr %__panic) {
entry:
  %abi_return11 = alloca { i64, i64 }, align 8
  %abi_return10 = alloca { i64, i64 }, align 8
  %"StructuredParallelLoweringEvidence_x3a_x3aparallelSpawnPanicEvidence$tmp$call_ref_tmp_256" = alloca i32, align 4
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
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
  %10 = icmp ne i8 %9, 0
  br i1 %10, label %poison.take6, label %poison.cont7

poison.take6:                                     ; preds = %poison.cont
  %11 = load ptr, ptr %__panic4, align 8
  store i8 1, ptr %11, align 1
  %12 = getelementptr i8, ptr %11, i64 4
  store i32 10, ptr %12, align 4
  ret void

poison.cont7:                                     ; preds = %poison.cont
  store i32 1, ptr %"StructuredParallelLoweringEvidence_x3a_x3aparallelSpawnPanicEvidence$tmp$call_ref_tmp_256", align 4
  %13 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
  %14 = icmp ne i8 %13, 0
  br i1 %14, label %poison.take8, label %poison.cont9

poison.take8:                                     ; preds = %poison.cont7
  %15 = load ptr, ptr %__panic4, align 8
  store i8 1, ptr %15, align 1
  %16 = getelementptr i8, ptr %15, i64 4
  store i32 10, ptr %16, align 4
  ret void

poison.cont9:                                     ; preds = %poison.cont7
  %17 = call i32 @StructuredParallelLoweringEvidence_x3a_x3aintegerOverflowPanicValue(ptr noundef nonnull align 4 dereferenceable(4) %"StructuredParallelLoweringEvidence_x3a_x3aparallelSpawnPanicEvidence$tmp$call_ref_tmp_256", ptr noundef nonnull align 8 dereferenceable(8) %__panic4)
  %18 = load ptr, ptr %__panic4, align 8
  %19 = load i8, ptr %18, align 1
  %20 = icmp ne i8 %19, 0
  br i1 %20, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont9
  %21 = load ptr, ptr %__panic4, align 8
  %22 = getelementptr i8, ptr %21, i64 0
  %23 = load i8, ptr %22, align 1
  %24 = load ptr, ptr %__panic4, align 8
  %25 = getelementptr i8, ptr %24, i64 4
  %26 = load i32, ptr %25, align 4
  %27 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$05")
  store { i64, i64 } %27, ptr %abi_return10, align 8
  %28 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return10, align 1
  ret void

panic.cont:                                       ; preds = %poison.cont9
  %29 = load ptr, ptr %result3, align 8
  store i32 %17, ptr %29, align 4
  %30 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$05")
  store { i64, i64 } %30, ptr %abi_return11, align 8
  %31 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return11, align 1
  ret void
}

define i8 @StructuredParallelLoweringEvidence_x3a_x3aruntimeEvidenceSucceeded(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %nested_result12 = alloca i32, align 4
  %nested_result = alloca i32, align 4
  %cancellation_result5 = alloca i32, align 4
  %cancellation_result = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
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
  %10 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
  %11 = icmp ne i8 %10, 0
  br i1 %11, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %12 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %12, align 1
  %13 = getelementptr i8, ptr %12, i64 4
  store i32 10, ptr %13, align 4
  %14 = load ptr, ptr %__panic, align 8
  %15 = getelementptr i8, ptr %14, i64 4
  %16 = load i32, ptr %15, align 4
  %17 = trunc i32 %16 to i8
  ret i8 %17

poison.cont2:                                     ; preds = %poison.cont
  %18 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
  %19 = icmp ne i8 %18, 0
  br i1 %19, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %20 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %20, align 1
  %21 = getelementptr i8, ptr %20, i64 4
  store i32 10, ptr %21, align 4
  %22 = load ptr, ptr %__panic, align 8
  %23 = getelementptr i8, ptr %22, i64 4
  %24 = load i32, ptr %23, align 4
  %25 = trunc i32 %24 to i8
  ret i8 %25

poison.cont4:                                     ; preds = %poison.cont2
  %26 = call i32 @StructuredParallelLoweringEvidence_x3a_x3acancellationLoweringEvidence(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %27 = load ptr, ptr %__panic, align 8
  %28 = load i8, ptr %27, align 1
  %29 = icmp ne i8 %28, 0
  br i1 %29, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  %30 = load ptr, ptr %__panic, align 8
  %31 = getelementptr i8, ptr %30, i64 4
  %32 = load i32, ptr %31, align 4
  %33 = trunc i32 %32 to i8
  ret i8 %33

panic.cont:                                       ; preds = %poison.cont4
  store i32 %26, ptr %cancellation_result5, align 4
  %34 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
  %35 = icmp ne i8 %34, 0
  br i1 %35, label %poison.take6, label %poison.cont7

poison.take6:                                     ; preds = %panic.cont
  %36 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %36, align 1
  %37 = getelementptr i8, ptr %36, i64 4
  store i32 10, ptr %37, align 4
  %38 = load ptr, ptr %__panic, align 8
  %39 = getelementptr i8, ptr %38, i64 4
  %40 = load i32, ptr %39, align 4
  %41 = trunc i32 %40 to i8
  ret i8 %41

poison.cont7:                                     ; preds = %panic.cont
  %42 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
  %43 = icmp ne i8 %42, 0
  br i1 %43, label %poison.take8, label %poison.cont9

poison.take8:                                     ; preds = %poison.cont7
  %44 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %44, align 1
  %45 = getelementptr i8, ptr %44, i64 4
  store i32 10, ptr %45, align 4
  %46 = load ptr, ptr %__panic, align 8
  %47 = getelementptr i8, ptr %46, i64 4
  %48 = load i32, ptr %47, align 4
  %49 = trunc i32 %48 to i8
  ret i8 %49

poison.cont9:                                     ; preds = %poison.cont7
  %50 = call i32 @StructuredParallelLoweringEvidence_x3a_x3anestedParallelLoweringEvidence(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %51 = load ptr, ptr %__panic, align 8
  %52 = load i8, ptr %51, align 1
  %53 = icmp ne i8 %52, 0
  br i1 %53, label %panic.take10, label %panic.cont11

panic.take10:                                     ; preds = %poison.cont9
  %54 = load ptr, ptr %__panic, align 8
  %55 = getelementptr i8, ptr %54, i64 0
  %56 = load i8, ptr %55, align 1
  %57 = load ptr, ptr %__panic, align 8
  %58 = getelementptr i8, ptr %57, i64 4
  %59 = load i32, ptr %58, align 4
  %60 = load ptr, ptr %__panic, align 8
  %61 = getelementptr i8, ptr %60, i64 4
  %62 = load i32, ptr %61, align 4
  %63 = trunc i32 %62 to i8
  ret i8 %63

panic.cont11:                                     ; preds = %poison.cont9
  store i32 %50, ptr %nested_result12, align 4
  %64 = load i32, ptr %cancellation_result5, align 4
  %65 = icmp eq i32 %64, 5
  %66 = zext i1 %65 to i8
  %67 = icmp ne i8 %66, 0
  %68 = icmp ne i8 %66, 0
  br i1 %68, label %if.then, label %if.else

if.then:                                          ; preds = %panic.cont11
  %69 = load i32, ptr %nested_result12, align 4
  %70 = icmp eq i32 %69, 8
  %71 = zext i1 %70 to i8
  br label %if.merge

if.else:                                          ; preds = %panic.cont11
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %"StructuredParallelLoweringEvidence_x3a_x3aruntimeEvidenceSucceeded$tmp$and_336" = phi i8 [ %71, %if.then ], [ 0, %if.else ]
  %72 = icmp ne i8 %"StructuredParallelLoweringEvidence_x3a_x3aruntimeEvidenceSucceeded$tmp$and_336", 0
  %73 = zext i1 %72 to i8
  ret i8 %73
}

define i32 @StructuredParallelLoweringEvidence_x3a_x3amain(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %abi_return9 = alloca i16, align 2
  %"StructuredParallelLoweringEvidence_x3a_x3amain$tmp$call_ref_tmp_370" = alloca { ptr, i64 }, align 8
  %evidence_path8 = alloca { ptr, i64 }, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %"StructuredParallelLoweringEvidence_x3a_x3amain$tmp$call_ref_tmp_362" = alloca i64, align 8
  %evidence_path = alloca { ptr, i64 }, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
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
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
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
  %23 = call i8 @StructuredParallelLoweringEvidence_x3a_x3aruntimeEvidenceSucceeded(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %24 = load ptr, ptr %__panic, align 8
  %25 = load i8, ptr %24, align 1
  %26 = icmp ne i8 %25, 0
  br i1 %26, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 4
  %29 = load i32, ptr %28, align 4
  ret i32 %29

panic.cont:                                       ; preds = %poison.cont4
  %30 = icmp ne i8 %23, 0
  %31 = xor i1 %30, true
  %32 = zext i1 %31 to i8
  %33 = icmp ne i8 %32, 0
  %34 = icmp ne i8 %32, 0
  br i1 %34, label %if.then, label %if.else

if.then:                                          ; preds = %panic.cont
  ret i32 1

if.else:                                          ; preds = %panic.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else
  %35 = getelementptr i8, ptr %context, i64 48
  %36 = getelementptr i8, ptr %context, i64 48
  %37 = call i64 @ultraviolet_x3a_x3aruntime_x3a_x3asystem_x3a_x3aargument_x5fcount(ptr noundef nonnull align 8 dereferenceable(16) %36)
  %38 = icmp ugt i64 %37, 1
  %39 = zext i1 %38 to i8
  %40 = icmp ne i8 %39, 0
  %41 = icmp ne i8 %39, 0
  br i1 %41, label %if.then5, label %if.else6

if.then5:                                         ; preds = %if.merge
  %42 = getelementptr i8, ptr %context, i64 48
  store i64 0, ptr %"StructuredParallelLoweringEvidence_x3a_x3amain$tmp$call_ref_tmp_362", align 4
  %43 = getelementptr i8, ptr %context, i64 48
  %44 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3asystem_x3a_x3aargument(ptr noundef nonnull align 8 dereferenceable(16) %43, ptr noundef nonnull align 8 dereferenceable(8) %"StructuredParallelLoweringEvidence_x3a_x3amain$tmp$call_ref_tmp_362")
  store { i64, i64 } %44, ptr %abi_return, align 8
  %45 = load { ptr, i64 }, ptr %abi_return, align 1
  store { ptr, i64 } %45, ptr %evidence_path8, align 8
  %46 = getelementptr i8, ptr %context, i64 0
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3abytes_x5f5777AA367F8561AB, i64 57 }, ptr %"StructuredParallelLoweringEvidence_x3a_x3amain$tmp$call_ref_tmp_370", align 8
  %47 = getelementptr i8, ptr %context, i64 0
  %48 = call i16 @ultraviolet_x3a_x3aruntime_x3a_x3aio_x3a_x3awrite_x5ffile(ptr noundef nonnull align 8 dereferenceable(16) %47, ptr noundef nonnull align 8 dereferenceable(16) %evidence_path8, ptr noundef nonnull align 8 dereferenceable(16) %"StructuredParallelLoweringEvidence_x3a_x3amain$tmp$call_ref_tmp_370")
  store i16 %48, ptr %abi_return9, align 2
  %49 = load { i8, [1 x i8] }, ptr %abi_return9, align 1
  ret i32 0

if.else6:                                         ; preds = %if.merge
  br label %if.merge7

if.merge7:                                        ; preds = %if.else6
  %50 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
  %51 = icmp ne i8 %50, 0
  br i1 %51, label %poison.take10, label %poison.cont11

poison.take10:                                    ; preds = %if.merge7
  %52 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %52, align 1
  %53 = getelementptr i8, ptr %52, i64 4
  store i32 10, ptr %53, align 4
  %54 = load ptr, ptr %__panic, align 8
  %55 = getelementptr i8, ptr %54, i64 4
  %56 = load i32, ptr %55, align 4
  ret i32 %56

poison.cont11:                                    ; preds = %if.merge7
  %57 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aStructuredParallelLoweringEvidence, align 1
  %58 = icmp ne i8 %57, 0
  br i1 %58, label %poison.take12, label %poison.cont13

poison.take12:                                    ; preds = %poison.cont11
  %59 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %59, align 1
  %60 = getelementptr i8, ptr %59, i64 4
  store i32 10, ptr %60, align 4
  %61 = load ptr, ptr %__panic, align 8
  %62 = getelementptr i8, ptr %61, i64 4
  %63 = load i32, ptr %62, align 4
  ret i32 %63

poison.cont13:                                    ; preds = %poison.cont11
  %64 = call i32 @StructuredParallelLoweringEvidence_x3a_x3aparallelSpawnPanicEvidence(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %65 = load ptr, ptr %__panic, align 8
  %66 = load i8, ptr %65, align 1
  %67 = icmp ne i8 %66, 0
  br i1 %67, label %panic.take14, label %panic.cont15

panic.take14:                                     ; preds = %poison.cont13
  %68 = load ptr, ptr %__panic, align 8
  %69 = getelementptr i8, ptr %68, i64 4
  %70 = load i32, ptr %69, align 4
  ret i32 %70

panic.cont15:                                     ; preds = %poison.cont13
  ret i32 %64
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aStructuredParallelLoweringEvidence(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aStructuredParallelLoweringEvidence(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #2

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #3

define void @__cx_lifecycle_init_StructuredParallelLoweringEvidence(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aStructuredParallelLoweringEvidence(ptr %0)
  ret void
}

define void @__cx_lifecycle_deinit_StructuredParallelLoweringEvidence(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aStructuredParallelLoweringEvidence(ptr %0)
  ret void
}

define i32 @main() {
entry:
  %entry_panic_code_arg = alloca i32, align 4
  %entry_ctx_bundle = alloca { { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, [0 x i64] }, align 8
  %entry_panic = alloca { i8, [3 x i8], i32, [0 x i32] }, align 8
  store { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, ptr %entry_panic, align 4
  %entry_panic_out = alloca ptr, align 8
  store ptr %entry_panic, ptr %entry_panic_out, align 8
  %entry_ctx = alloca { { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, [0 x i64] }, align 8
  store { { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, [0 x i64] } zeroinitializer, ptr %entry_ctx, align 8
  call void @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x5finit(ptr %entry_ctx)
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aStructuredParallelLoweringEvidence(ptr %entry_panic_out)
  %0 = load i8, ptr %entry_panic, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %entry.init.fail, label %entry.init.cont

entry.panic:                                      ; preds = %entry.deinit.panic.restore, %entry.init.cont, %entry.init.fail
  %2 = getelementptr i8, ptr %entry_panic, i64 4
  %3 = load i32, ptr %2, align 4
  store i32 %3, ptr %entry_panic_code_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %entry_panic_code_arg)
  unreachable

entry.init.fail:                                  ; preds = %entry
  br label %entry.panic

entry.init.cont:                                  ; preds = %entry
  %4 = load { { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, [0 x i64] }, ptr %entry_ctx, align 8
  store { { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, [0 x i64] } zeroinitializer, ptr %entry_ctx_bundle, align 8
  %5 = getelementptr i8, ptr %entry_ctx, i64 0
  %6 = load { ptr, ptr }, ptr %5, align 8
  %7 = getelementptr i8, ptr %entry_ctx_bundle, i64 0
  store { ptr, ptr } %6, ptr %7, align 8
  %8 = getelementptr i8, ptr %entry_ctx, i64 16
  %9 = load { ptr, ptr }, ptr %8, align 8
  %10 = getelementptr i8, ptr %entry_ctx_bundle, i64 16
  store { ptr, ptr } %9, ptr %10, align 8
  %11 = getelementptr i8, ptr %entry_ctx, i64 32
  %12 = load { ptr, ptr }, ptr %11, align 8
  %13 = getelementptr i8, ptr %entry_ctx_bundle, i64 32
  store { ptr, ptr } %12, ptr %13, align 8
  %14 = getelementptr i8, ptr %entry_ctx, i64 48
  %15 = load { ptr, ptr }, ptr %14, align 8
  %16 = getelementptr i8, ptr %entry_ctx_bundle, i64 48
  store { ptr, ptr } %15, ptr %16, align 8
  %17 = getelementptr i8, ptr %entry_ctx, i64 64
  %18 = load { ptr, ptr }, ptr %17, align 8
  %19 = getelementptr i8, ptr %entry_ctx_bundle, i64 64
  store { ptr, ptr } %18, ptr %19, align 8
  %20 = getelementptr i8, ptr %entry_ctx, i64 80
  %21 = load { ptr, ptr }, ptr %20, align 8
  %22 = getelementptr i8, ptr %entry_ctx_bundle, i64 80
  store { ptr, ptr } %21, ptr %22, align 8
  %23 = load { { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, [0 x i64] }, ptr %entry_ctx_bundle, align 8
  %24 = call i32 @StructuredParallelLoweringEvidence_x3a_x3amain(ptr %entry_ctx, ptr %entry_panic_out)
  %entry_deinit_panic_seen = alloca i8, align 1
  %entry_deinit_panic_code = alloca i32, align 4
  store i8 0, ptr %entry_deinit_panic_seen, align 1
  store i32 0, ptr %entry_deinit_panic_code, align 4
  %25 = load i8, ptr %entry_panic, align 1
  %26 = icmp ne i8 %25, 0
  br i1 %26, label %entry.panic, label %entry.deinit

entry.deinit:                                     ; preds = %entry.init.cont
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aStructuredParallelLoweringEvidence(ptr %entry_panic_out)
  %27 = load i8, ptr %entry_panic, align 1
  %28 = icmp ne i8 %27, 0
  br i1 %28, label %entry.deinit.panic.capture, label %entry.deinit.panic.cont

entry.deinit.panic.capture:                       ; preds = %entry.deinit
  %29 = load i8, ptr %entry_deinit_panic_seen, align 1
  %30 = icmp ne i8 %29, 0
  %31 = getelementptr i8, ptr %entry_panic, i64 4
  %32 = load i32, ptr %31, align 4
  br i1 %30, label %entry.deinit.panic.clear, label %entry.deinit.panic.store

entry.deinit.panic.cont:                          ; preds = %entry.deinit.panic.clear, %entry.deinit
  %33 = load i8, ptr %entry_deinit_panic_seen, align 1
  %34 = icmp ne i8 %33, 0
  br i1 %34, label %entry.deinit.panic.restore, label %entry.ret

entry.deinit.panic.store:                         ; preds = %entry.deinit.panic.capture
  store i8 1, ptr %entry_deinit_panic_seen, align 1
  store i32 %32, ptr %entry_deinit_panic_code, align 4
  br label %entry.deinit.panic.clear

entry.deinit.panic.clear:                         ; preds = %entry.deinit.panic.store, %entry.deinit.panic.capture
  store i8 0, ptr %entry_panic, align 1
  %35 = getelementptr i8, ptr %entry_panic, i64 4
  store i32 0, ptr %35, align 4
  br label %entry.deinit.panic.cont

entry.deinit.panic.restore:                       ; preds = %entry.deinit.panic.cont
  %36 = load i32, ptr %entry_deinit_panic_code, align 4
  store i8 1, ptr %entry_panic, align 1
  %37 = getelementptr i8, ptr %entry_panic, i64 4
  store i32 %36, ptr %37, align 4
  br label %entry.panic

entry.ret:                                        ; preds = %entry.deinit.panic.cont
  ret i32 %24
}

declare void @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x5finit(ptr)

attributes #0 = { nounwind }
attributes #1 = { noreturn nounwind }
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #3 = { nocallback nofree nounwind willreturn memory(argmem: write) }
