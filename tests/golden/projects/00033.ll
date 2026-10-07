; ==== Source/GpuRuntimeEvidence.ll
; ModuleID = 'GpuRuntimeEvidence'
source_filename = "GpuRuntimeEvidence"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aGpuRuntimeEvidence = global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fA09E307A7F948ACD = internal constant [8 x i8] c"\08\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f1E01B4FFF7399B4D = internal constant [8 x i8] c"\88\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f6AD26A20123BA583 = internal constant [8 x i8] c"\06\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fE6BD86443DF8CE07 = internal constant [8 x i8] c"\02\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2CDCDC0DFC5D1141 = internal constant [8 x i8] c"\04\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fADAAA9AF328EA9CC = internal constant [4 x i8] c")\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAF63C54C8601C577 = internal constant [1 x i8] c"\08", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fA8C7F832281A39C5 = internal constant [8 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f89CD31291D2AEFA4 = internal constant [8 x i8] c"\01\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAD6323825FA766DC = internal constant [8 x i8] c"\E8\03\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f6D3572669B2CDE42 = internal constant [4 x i8] c"\07\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fECF582CAA5B1B50E = internal constant [4 x i8] c"\0B\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAD2ACA7747985764 = internal constant [4 x i8] c"\01\00\00\00", align 1
@0 = private unnamed_addr constant [2 x i8] c"+\00", align 1

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3acpu(ptr noundef nonnull align 8 dereferenceable(96)) #0

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3agpu(ptr noundef nonnull align 8 dereferenceable(96)) #0

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3ainline(ptr noundef nonnull align 8 dereferenceable(96)) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3abarrier() #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3aglobal_x5fid(ptr noalias noundef nonnull sret({ i64, i64, i64 }) align 8 dereferenceable(24)) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3aglobal_x5fsize(ptr noalias noundef nonnull sret({ i64, i64, i64 }) align 8 dereferenceable(24)) #0

; Function Attrs: nounwind
declare i64 @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3alinear_x5fid() #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3alocal_x5fid(ptr noalias noundef nonnull sret({ i64, i64, i64 }) align 8 dereferenceable(24)) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3amemory_x5fbarrier() #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3anum_x5fworkgroups(ptr noalias noundef nonnull sret({ i64, i64, i64 }) align 8 dereferenceable(24)) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3aworkgroup_x5fbarrier() #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3aworkgroup_x5fid(ptr noalias noundef nonnull sret({ i64, i64, i64 }) align 8 dereferenceable(24)) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3aworkgroup_x5fsize(ptr noalias noundef nonnull sret({ i64, i64, i64 }) align 8 dereferenceable(24)) #0

; Function Attrs: noreturn nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4)) #1

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5ftag_x5fscope(ptr, i64) #0

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16)) #0

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3anew_x5fscoped(ptr noundef nonnull align 8 dereferenceable(40)) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fenter(i64) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64) #0

; Function Attrs: nounwind
declare void @uv_dispatch_run(ptr noundef nonnull align 8 dereferenceable(24), i64, i64, ptr, ptr, ptr, { i64, i64 }, ptr, ptr, i32, i64, ptr noundef nonnull byval({ i64, i64, i64 }) align 8 dereferenceable(24)) #0

; Function Attrs: nounwind
declare ptr @uv_parallel_begin({ i64, i64 }, i64, ptr) #0

; Function Attrs: nounwind
declare i32 @uv_parallel_join(ptr) #0

define i8 @GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %observed = alloca i64, align 8
  %byref_arg6 = alloca i32, align 4
  %byref_arg2 = alloca { i64, i64, i64 }, align 8
  %coerce_bits1 = alloca { ptr, i64 }, align 8
  %byref_arg = alloca { i8, [7 x i8], i64, i64 }, align 8
  %0 = alloca { i64, i64, i64 }, align 8
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$dispatch_result_var_10" = alloca i64, align 8
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$dispatch_env_storage_8" = alloca {}, align 1
  %coerce_bits = alloca { ptr, ptr }, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aGpuRuntimeEvidence, align 1
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
  %8 = trunc i32 %7 to i8
  ret i8 %8

poison.cont:                                      ; preds = %entry
  %9 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 0, ptr %10, align 4
  %11 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3agpu(ptr noundef nonnull align 8 dereferenceable(96) %context)
  store { i64, i64 } %11, ptr %abi_return, align 8
  %12 = load { ptr, ptr }, ptr %abi_return, align 1
  store { ptr, ptr } %12, ptr %coerce_bits, align 8
  %13 = load { i64, i64 }, ptr %coerce_bits, align 1
  %14 = call ptr @uv_parallel_begin({ i64, i64 } %13, i64 -1, ptr null)
  store {} zeroinitializer, ptr %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$dispatch_env_storage_8", align 1
  store i64 0, ptr %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$dispatch_result_var_10", align 4
  store { i64, i64, i64 } zeroinitializer, ptr %0, align 4
  %15 = getelementptr i8, ptr %0, i64 0
  store i64 2, ptr %15, align 1
  %16 = getelementptr i8, ptr %0, i64 8
  store i64 2, ptr %16, align 1
  %17 = getelementptr i8, ptr %0, i64 16
  store i64 2, ptr %17, align 1
  %18 = load { i64, i64, i64 }, ptr %0, align 4
  store { i8, [7 x i8], i64, i64 } { i8 4, [7 x i8] zeroinitializer, i64 0, i64 16 }, ptr %byref_arg, align 4
  store { ptr, i64 } { ptr @0, i64 1 }, ptr %coerce_bits1, align 8
  %19 = load { i64, i64 }, ptr %coerce_bits1, align 1
  store { i64, i64, i64 } %18, ptr %byref_arg2, align 4
  call void @uv_dispatch_run(ptr noundef nonnull align 8 dereferenceable(24) %byref_arg, i64 8, i64 8, ptr @__cx_dispatch_body_GpuRuntimeEvidence_0, ptr null, ptr %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$dispatch_env_storage_8", { i64, i64 } %19, ptr %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$dispatch_result_var_10", ptr null, i32 0, i64 0, ptr noundef nonnull byval({ i64, i64, i64 }) align 8 dereferenceable(24) %byref_arg2)
  %20 = load i64, ptr %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$dispatch_result_var_10", align 4
  %21 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %21, align 1
  %22 = getelementptr i8, ptr %21, i64 4
  store i32 0, ptr %22, align 4
  %23 = call i32 @uv_parallel_join(ptr %14)
  %24 = icmp eq i32 %23, 0
  %25 = xor i1 %24, true
  %26 = zext i1 %25 to i8
  %27 = icmp ne i8 %26, 0
  %28 = icmp ne i8 %26, 0
  br i1 %28, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  %29 = load ptr, ptr %__panic, align 8
  %30 = getelementptr i8, ptr %29, i64 0
  store i8 1, ptr %30, align 1
  %31 = load ptr, ptr %__panic, align 8
  %32 = getelementptr i8, ptr %31, i64 4
  store i32 %23, ptr %32, align 4
  br label %if.merge

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %if.result = phi i64 [ 0, %if.then ], [ 0, %if.else ]
  %33 = load ptr, ptr %__panic, align 8
  %34 = getelementptr i8, ptr %33, i64 0
  %35 = load i8, ptr %34, align 1
  %36 = load ptr, ptr %__panic, align 8
  %37 = getelementptr i8, ptr %36, i64 4
  %38 = load i32, ptr %37, align 4
  %39 = icmp ne i8 %35, 0
  %40 = zext i1 %39 to i8
  %41 = icmp ne i8 %40, 0
  %42 = and i1 false, %41
  br i1 %42, label %if.then3, label %if.else4

if.then3:                                         ; preds = %if.merge
  store i32 %38, ptr %byref_arg6, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg6)
  unreachable

if.else4:                                         ; preds = %if.merge
  br label %if.merge5

if.merge5:                                        ; preds = %if.else4
  %43 = icmp ne i8 %35, 0
  %44 = xor i1 %43, true
  %45 = and i1 false, %44
  br i1 %45, label %if.then7, label %if.else8

if.then7:                                         ; preds = %if.merge5
  %46 = load ptr, ptr %__panic, align 8
  %47 = getelementptr i8, ptr %46, i64 0
  store i8 1, ptr %47, align 1
  %48 = load ptr, ptr %__panic, align 8
  %49 = getelementptr i8, ptr %48, i64 4
  store i32 0, ptr %49, align 4
  br label %if.merge9

if.else8:                                         ; preds = %if.merge5
  br label %if.merge9

if.merge9:                                        ; preds = %if.else8, %if.then7
  %if.result10 = phi i64 [ 0, %if.then7 ], [ 0, %if.else8 ]
  %50 = icmp ne i8 %35, 0
  %51 = zext i1 %50 to i8
  %52 = icmp ne i8 %51, 0
  %53 = or i1 false, %52
  %54 = icmp ne i8 %35, 0
  %55 = icmp ne i8 %35, 0
  br i1 %55, label %if.then11, label %if.else12

if.then11:                                        ; preds = %if.merge9
  br label %if.merge13

if.else12:                                        ; preds = %if.merge9
  br label %if.merge13

if.merge13:                                       ; preds = %if.else12, %if.then11
  %if.result14 = phi i32 [ %38, %if.then11 ], [ 0, %if.else12 ]
  %56 = load ptr, ptr %__panic, align 8
  %57 = load i8, ptr %56, align 1
  %58 = icmp ne i8 %57, 0
  br i1 %58, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %if.merge13
  %59 = load ptr, ptr %__panic, align 8
  %60 = getelementptr i8, ptr %59, i64 4
  %61 = load i32, ptr %60, align 4
  %62 = trunc i32 %61 to i8
  ret i8 %62

panic.cont:                                       ; preds = %if.merge13
  store i64 %20, ptr %observed, align 4
  %63 = load i64, ptr %observed, align 4
  %64 = icmp eq i64 %63, 136
  %65 = zext i1 %64 to i8
  %66 = icmp ne i8 %65, 0
  %67 = zext i1 %66 to i8
  ret i8 %67
}

define internal void @__cx_dispatch_body_GpuRuntimeEvidence_0(ptr %__uv_host_env, ptr noundef nonnull align 8 dereferenceable(8) %elem, ptr noundef nonnull align 1 %env, ptr noundef nonnull align 8 dereferenceable(8) %result, ptr %__panic) {
entry:
  %abi_return137 = alloca { i64, i64 }, align 8
  %abi_return134 = alloca { i64, i64 }, align 8
  %abi_return127 = alloca { i64, i64 }, align 8
  %matches_topology = alloca i8, align 1
  %0 = alloca { i64, i64, i64 }, align 8
  %1 = alloca { i64, i64, i64 }, align 8
  %2 = alloca { i64, i64, i64 }, align 8
  %3 = alloca { i64, i64, i64 }, align 8
  %4 = alloca { i64, i64, i64 }, align 8
  %5 = alloca { i64, i64, i64 }, align 8
  %6 = alloca { i64, i64, i64 }, align 8
  %7 = alloca { i64, i64, i64 }, align 8
  %8 = alloca { i64, i64, i64 }, align 8
  %9 = alloca { i64, i64, i64 }, align 8
  %10 = alloca { i64, i64, i64 }, align 8
  %11 = alloca { i64, i64, i64 }, align 8
  %12 = alloca { i64, i64, i64 }, align 8
  %13 = alloca { i64, i64, i64 }, align 8
  %14 = alloca { i64, i64, i64 }, align 8
  %15 = alloca { i64, i64, i64 }, align 8
  %16 = alloca { i64, i64, i64 }, align 8
  %17 = alloca { i64, i64, i64 }, align 8
  %private_read = alloca i32, align 4
  %private_pointer = alloca ptr, align 8
  %forms_have_pointer_layout = alloca i8, align 1
  %private_value = alloca i32, align 4
  %expected_global_x = alloca i64, align 8
  %abi_return54 = alloca { i64, i64 }, align 8
  %abi_return49 = alloca { i64, i64 }, align 8
  %expected_workgroup_x = alloca i64, align 8
  %abi_return42 = alloca { i64, i64 }, align 8
  %expected_local_z = alloca i64, align 8
  %abi_return35 = alloca { i64, i64 }, align 8
  %expected_local_y = alloca i64, align 8
  %abi_return28 = alloca { i64, i64 }, align 8
  %abi_return21 = alloca { i64, i64 }, align 8
  %expected_local_x = alloca i64, align 8
  %abi_return14 = alloca { i64, i64 }, align 8
  %expected_local_linear = alloca i64, align 8
  %abi_return7 = alloca { i64, i64 }, align 8
  %relative_index = alloca i64, align 8
  %num_workgroups = alloca { i64, i64, i64 }, align 8
  %global_size = alloca { i64, i64, i64 }, align 8
  %workgroup_size = alloca { i64, i64, i64 }, align 8
  %workgroup_id = alloca { i64, i64, i64 }, align 8
  %local_id = alloca { i64, i64, i64 }, align 8
  %global_id = alloca { i64, i64, i64 }, align 8
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
  %18 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aGpuRuntimeEvidence, align 1
  %19 = icmp ne i8 %18, 0
  br i1 %19, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %20 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %20, align 1
  %21 = getelementptr i8, ptr %20, i64 4
  store i32 10, ptr %21, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %22 = load ptr, ptr %__panic5, align 8
  store i8 0, ptr %22, align 1
  %23 = getelementptr i8, ptr %22, i64 4
  store i32 0, ptr %23, align 4
  call void @llvm.memset.p0.i64(ptr align 8 %aggregate.literal, i8 0, i64 40, i1 false)
  store i64 0, ptr %aggregate.literal, align 8
  %24 = getelementptr i8, ptr %aggregate.literal, i64 8
  store { i8, [7 x i8], [24 x i8], [0 x i64] } zeroinitializer, ptr %24, align 8
  %25 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3anew_x5fscoped(ptr noundef nonnull align 8 dereferenceable(40) %aggregate.literal)
  store { i64, i64 } %25, ptr %abi_return, align 8
  %26 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return, align 1
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %26, ptr %"region$06", align 4
  %27 = load ptr, ptr %elem2, align 8
  %28 = load i64, ptr %27, align 4
  store i64 %28, ptr %index, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fenter(i64 6)
  call void @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3aglobal_x5fid(ptr noalias noundef nonnull sret({ i64, i64, i64 }) align 8 dereferenceable(24) %global_id)
  call void @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3alocal_x5fid(ptr noalias noundef nonnull sret({ i64, i64, i64 }) align 8 dereferenceable(24) %local_id)
  call void @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3aworkgroup_x5fid(ptr noalias noundef nonnull sret({ i64, i64, i64 }) align 8 dereferenceable(24) %workgroup_id)
  call void @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3aworkgroup_x5fsize(ptr noalias noundef nonnull sret({ i64, i64, i64 }) align 8 dereferenceable(24) %workgroup_size)
  call void @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3aglobal_x5fsize(ptr noalias noundef nonnull sret({ i64, i64, i64 }) align 8 dereferenceable(24) %global_size)
  call void @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3anum_x5fworkgroups(ptr noalias noundef nonnull sret({ i64, i64, i64 }) align 8 dereferenceable(24) %num_workgroups)
  %29 = load i64, ptr %index, align 1
  store i64 %29, ptr %relative_index, align 4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %30 = load ptr, ptr %__panic5, align 8
  %31 = load i8, ptr %30, align 1
  %32 = icmp ne i8 %31, 0
  br i1 %32, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %33 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %33, align 1
  %34 = getelementptr i8, ptr %33, i64 4
  store i32 3, ptr %34, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %35 = load ptr, ptr %__panic5, align 8
  %36 = getelementptr i8, ptr %35, i64 0
  %37 = load i8, ptr %36, align 1
  %38 = load ptr, ptr %__panic5, align 8
  %39 = getelementptr i8, ptr %38, i64 4
  %40 = load i32, ptr %39, align 4
  %41 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$06")
  store { i64, i64 } %41, ptr %abi_return7, align 8
  %42 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return7, align 1
  ret void

panic.cont:                                       ; preds = %check_ok
  %43 = load i64, ptr %relative_index, align 4
  br i1 true, label %check_ok8, label %check_fail9

check_ok8:                                        ; preds = %panic.cont
  %44 = urem i64 %43, 8
  %45 = freeze i64 %44
  store i64 %45, ptr %expected_local_linear, align 4
  br i1 true, label %check_ok10, label %check_fail11

check_fail9:                                      ; preds = %panic.cont
  %46 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %46, align 1
  %47 = getelementptr i8, ptr %46, i64 4
  store i32 3, ptr %47, align 4
  ret void

check_ok10:                                       ; preds = %check_fail11, %check_ok8
  %48 = load ptr, ptr %__panic5, align 8
  %49 = load i8, ptr %48, align 1
  %50 = icmp ne i8 %49, 0
  br i1 %50, label %panic.take12, label %panic.cont13

check_fail11:                                     ; preds = %check_ok8
  %51 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %51, align 1
  %52 = getelementptr i8, ptr %51, i64 4
  store i32 3, ptr %52, align 4
  br label %check_ok10

panic.take12:                                     ; preds = %check_ok10
  %53 = load ptr, ptr %__panic5, align 8
  %54 = getelementptr i8, ptr %53, i64 0
  %55 = load i8, ptr %54, align 1
  %56 = load ptr, ptr %__panic5, align 8
  %57 = getelementptr i8, ptr %56, i64 4
  %58 = load i32, ptr %57, align 4
  %59 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$06")
  store { i64, i64 } %59, ptr %abi_return14, align 8
  %60 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return14, align 1
  ret void

panic.cont13:                                     ; preds = %check_ok10
  %61 = load i64, ptr %expected_local_linear, align 4
  br i1 true, label %check_ok15, label %check_fail16

check_ok15:                                       ; preds = %panic.cont13
  %62 = urem i64 %61, 2
  %63 = freeze i64 %62
  store i64 %63, ptr %expected_local_x, align 4
  br i1 true, label %check_ok17, label %check_fail18

check_fail16:                                     ; preds = %panic.cont13
  %64 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %64, align 1
  %65 = getelementptr i8, ptr %64, i64 4
  store i32 3, ptr %65, align 4
  ret void

check_ok17:                                       ; preds = %check_fail18, %check_ok15
  %66 = load ptr, ptr %__panic5, align 8
  %67 = load i8, ptr %66, align 1
  %68 = icmp ne i8 %67, 0
  br i1 %68, label %panic.take19, label %panic.cont20

check_fail18:                                     ; preds = %check_ok15
  %69 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %69, align 1
  %70 = getelementptr i8, ptr %69, i64 4
  store i32 3, ptr %70, align 4
  br label %check_ok17

panic.take19:                                     ; preds = %check_ok17
  %71 = load ptr, ptr %__panic5, align 8
  %72 = getelementptr i8, ptr %71, i64 0
  %73 = load i8, ptr %72, align 1
  %74 = load ptr, ptr %__panic5, align 8
  %75 = getelementptr i8, ptr %74, i64 4
  %76 = load i32, ptr %75, align 4
  %77 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$06")
  store { i64, i64 } %77, ptr %abi_return21, align 8
  %78 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return21, align 1
  ret void

panic.cont20:                                     ; preds = %check_ok17
  %79 = load i64, ptr %expected_local_linear, align 4
  br i1 true, label %check_ok22, label %check_fail23

check_ok22:                                       ; preds = %panic.cont20
  %80 = udiv i64 %79, 2
  %81 = freeze i64 %80
  br i1 true, label %check_ok24, label %check_fail25

check_fail23:                                     ; preds = %panic.cont20
  %82 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %82, align 1
  %83 = getelementptr i8, ptr %82, i64 4
  store i32 3, ptr %83, align 4
  ret void

check_ok24:                                       ; preds = %check_fail25, %check_ok22
  %84 = load ptr, ptr %__panic5, align 8
  %85 = load i8, ptr %84, align 1
  %86 = icmp ne i8 %85, 0
  br i1 %86, label %panic.take26, label %panic.cont27

check_fail25:                                     ; preds = %check_ok22
  %87 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %87, align 1
  %88 = getelementptr i8, ptr %87, i64 4
  store i32 3, ptr %88, align 4
  br label %check_ok24

panic.take26:                                     ; preds = %check_ok24
  %89 = load ptr, ptr %__panic5, align 8
  %90 = getelementptr i8, ptr %89, i64 0
  %91 = load i8, ptr %90, align 1
  %92 = load ptr, ptr %__panic5, align 8
  %93 = getelementptr i8, ptr %92, i64 4
  %94 = load i32, ptr %93, align 4
  %95 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$06")
  store { i64, i64 } %95, ptr %abi_return28, align 8
  %96 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return28, align 1
  ret void

panic.cont27:                                     ; preds = %check_ok24
  br i1 true, label %check_ok29, label %check_fail30

check_ok29:                                       ; preds = %panic.cont27
  %97 = urem i64 %81, 2
  %98 = freeze i64 %97
  store i64 %98, ptr %expected_local_y, align 4
  br i1 true, label %check_ok31, label %check_fail32

check_fail30:                                     ; preds = %panic.cont27
  %99 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %99, align 1
  %100 = getelementptr i8, ptr %99, i64 4
  store i32 3, ptr %100, align 4
  ret void

check_ok31:                                       ; preds = %check_fail32, %check_ok29
  %101 = load ptr, ptr %__panic5, align 8
  %102 = load i8, ptr %101, align 1
  %103 = icmp ne i8 %102, 0
  br i1 %103, label %panic.take33, label %panic.cont34

check_fail32:                                     ; preds = %check_ok29
  %104 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %104, align 1
  %105 = getelementptr i8, ptr %104, i64 4
  store i32 3, ptr %105, align 4
  br label %check_ok31

panic.take33:                                     ; preds = %check_ok31
  %106 = load ptr, ptr %__panic5, align 8
  %107 = getelementptr i8, ptr %106, i64 0
  %108 = load i8, ptr %107, align 1
  %109 = load ptr, ptr %__panic5, align 8
  %110 = getelementptr i8, ptr %109, i64 4
  %111 = load i32, ptr %110, align 4
  %112 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$06")
  store { i64, i64 } %112, ptr %abi_return35, align 8
  %113 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return35, align 1
  ret void

panic.cont34:                                     ; preds = %check_ok31
  %114 = load i64, ptr %expected_local_linear, align 4
  br i1 true, label %check_ok36, label %check_fail37

check_ok36:                                       ; preds = %panic.cont34
  %115 = udiv i64 %114, 4
  %116 = freeze i64 %115
  store i64 %116, ptr %expected_local_z, align 4
  br i1 true, label %check_ok38, label %check_fail39

check_fail37:                                     ; preds = %panic.cont34
  %117 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %117, align 1
  %118 = getelementptr i8, ptr %117, i64 4
  store i32 3, ptr %118, align 4
  ret void

check_ok38:                                       ; preds = %check_fail39, %check_ok36
  %119 = load ptr, ptr %__panic5, align 8
  %120 = load i8, ptr %119, align 1
  %121 = icmp ne i8 %120, 0
  br i1 %121, label %panic.take40, label %panic.cont41

check_fail39:                                     ; preds = %check_ok36
  %122 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %122, align 1
  %123 = getelementptr i8, ptr %122, i64 4
  store i32 3, ptr %123, align 4
  br label %check_ok38

panic.take40:                                     ; preds = %check_ok38
  %124 = load ptr, ptr %__panic5, align 8
  %125 = getelementptr i8, ptr %124, i64 0
  %126 = load i8, ptr %125, align 1
  %127 = load ptr, ptr %__panic5, align 8
  %128 = getelementptr i8, ptr %127, i64 4
  %129 = load i32, ptr %128, align 4
  %130 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$06")
  store { i64, i64 } %130, ptr %abi_return42, align 8
  %131 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return42, align 1
  ret void

panic.cont41:                                     ; preds = %check_ok38
  %132 = load i64, ptr %relative_index, align 4
  br i1 true, label %check_ok43, label %check_fail44

check_ok43:                                       ; preds = %panic.cont41
  %133 = udiv i64 %132, 8
  %134 = freeze i64 %133
  store i64 %134, ptr %expected_workgroup_x, align 4
  br i1 true, label %check_ok45, label %check_fail46

check_fail44:                                     ; preds = %panic.cont41
  %135 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %135, align 1
  %136 = getelementptr i8, ptr %135, i64 4
  store i32 3, ptr %136, align 4
  ret void

check_ok45:                                       ; preds = %check_fail46, %check_ok43
  %137 = load ptr, ptr %__panic5, align 8
  %138 = load i8, ptr %137, align 1
  %139 = icmp ne i8 %138, 0
  br i1 %139, label %panic.take47, label %panic.cont48

check_fail46:                                     ; preds = %check_ok43
  %140 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %140, align 1
  %141 = getelementptr i8, ptr %140, i64 4
  store i32 4, ptr %141, align 4
  br label %check_ok45

panic.take47:                                     ; preds = %check_ok45
  %142 = load ptr, ptr %__panic5, align 8
  %143 = getelementptr i8, ptr %142, i64 0
  %144 = load i8, ptr %143, align 1
  %145 = load ptr, ptr %__panic5, align 8
  %146 = getelementptr i8, ptr %145, i64 4
  %147 = load i32, ptr %146, align 4
  %148 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$06")
  store { i64, i64 } %148, ptr %abi_return49, align 8
  %149 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return49, align 1
  ret void

panic.cont48:                                     ; preds = %check_ok45
  %150 = load i64, ptr %expected_workgroup_x, align 4
  %151 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %150, i64 2)
  %152 = extractvalue { i64, i1 } %151, 0
  %153 = extractvalue { i64, i1 } %151, 1
  %154 = freeze i64 %152
  br i1 %153, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont48
  br i1 true, label %check_ok50, label %check_fail51

op_fail:                                          ; preds = %panic.cont48
  %155 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %155, align 1
  %156 = getelementptr i8, ptr %155, i64 4
  store i32 4, ptr %156, align 4
  ret void

check_ok50:                                       ; preds = %check_fail51, %op_ok
  %157 = load ptr, ptr %__panic5, align 8
  %158 = load i8, ptr %157, align 1
  %159 = icmp ne i8 %158, 0
  br i1 %159, label %panic.take52, label %panic.cont53

check_fail51:                                     ; preds = %op_ok
  %160 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %160, align 1
  %161 = getelementptr i8, ptr %160, i64 4
  store i32 4, ptr %161, align 4
  br label %check_ok50

panic.take52:                                     ; preds = %check_ok50
  %162 = load ptr, ptr %__panic5, align 8
  %163 = getelementptr i8, ptr %162, i64 0
  %164 = load i8, ptr %163, align 1
  %165 = load ptr, ptr %__panic5, align 8
  %166 = getelementptr i8, ptr %165, i64 4
  %167 = load i32, ptr %166, align 4
  %168 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$06")
  store { i64, i64 } %168, ptr %abi_return54, align 8
  %169 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return54, align 1
  ret void

panic.cont53:                                     ; preds = %check_ok50
  %170 = load i64, ptr %expected_local_x, align 4
  %171 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %170, i64 %154)
  %172 = extractvalue { i64, i1 } %171, 0
  %173 = extractvalue { i64, i1 } %171, 1
  %174 = freeze i64 %172
  br i1 %173, label %op_fail56, label %op_ok55

op_ok55:                                          ; preds = %panic.cont53
  store i64 %174, ptr %expected_global_x, align 4
  store i32 41, ptr %private_value, align 4
  br i1 true, label %if.then, label %if.else

op_fail56:                                        ; preds = %panic.cont53
  %175 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %175, align 1
  %176 = getelementptr i8, ptr %175, i64 4
  store i32 4, ptr %176, align 4
  ret void

if.then:                                          ; preds = %op_ok55
  br label %if.merge

if.else:                                          ; preds = %op_ok55
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_81" = phi i8 [ 1, %if.then ], [ 0, %if.else ]
  %177 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_81", 0
  %178 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_81", 0
  br i1 %178, label %if.then57, label %if.else58

if.then57:                                        ; preds = %if.merge
  br label %if.merge59

if.else58:                                        ; preds = %if.merge
  br label %if.merge59

if.merge59:                                       ; preds = %if.else58, %if.then57
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_83" = phi i8 [ 1, %if.then57 ], [ 0, %if.else58 ]
  %179 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_83", 0
  %180 = zext i1 %179 to i8
  store i8 %180, ptr %forms_have_pointer_layout, align 1
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5ftag_x5fscope(ptr %private_value, i64 6)
  store ptr %private_value, ptr %private_pointer, align 8
  %181 = load ptr, ptr %private_pointer, align 8
  %182 = load i32, ptr %181, align 4
  store i32 %182, ptr %private_read, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3amemory_x5fbarrier()
  call void @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3aworkgroup_x5fbarrier()
  call void @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3abarrier()
  %183 = load { i64, i64, i64 }, ptr %global_id, align 4
  store { i64, i64, i64 } %183, ptr %17, align 4
  %184 = getelementptr i8, ptr %17, i64 0
  %185 = load i64, ptr %184, align 1
  %186 = load i64, ptr %expected_global_x, align 4
  %187 = icmp eq i64 %185, %186
  %188 = zext i1 %187 to i8
  %189 = icmp ne i8 %188, 0
  %190 = icmp ne i8 %188, 0
  br i1 %190, label %if.then60, label %if.else61

if.then60:                                        ; preds = %if.merge59
  %191 = load { i64, i64, i64 }, ptr %global_id, align 4
  store { i64, i64, i64 } %191, ptr %16, align 4
  %192 = getelementptr i8, ptr %16, i64 8
  %193 = load i64, ptr %192, align 1
  %194 = load i64, ptr %expected_local_y, align 4
  %195 = icmp eq i64 %193, %194
  %196 = zext i1 %195 to i8
  br label %if.merge62

if.else61:                                        ; preds = %if.merge59
  br label %if.merge62

if.merge62:                                       ; preds = %if.else61, %if.then60
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_98" = phi i8 [ %196, %if.then60 ], [ 0, %if.else61 ]
  %197 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_98", 0
  %198 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_98", 0
  br i1 %198, label %if.then63, label %if.else64

if.then63:                                        ; preds = %if.merge62
  %199 = load { i64, i64, i64 }, ptr %global_id, align 4
  store { i64, i64, i64 } %199, ptr %15, align 4
  %200 = getelementptr i8, ptr %15, i64 16
  %201 = load i64, ptr %200, align 1
  %202 = load i64, ptr %expected_local_z, align 4
  %203 = icmp eq i64 %201, %202
  %204 = zext i1 %203 to i8
  br label %if.merge65

if.else64:                                        ; preds = %if.merge62
  br label %if.merge65

if.merge65:                                       ; preds = %if.else64, %if.then63
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_101" = phi i8 [ %204, %if.then63 ], [ 0, %if.else64 ]
  %205 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_101", 0
  %206 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_101", 0
  br i1 %206, label %if.then66, label %if.else67

if.then66:                                        ; preds = %if.merge65
  %207 = load { i64, i64, i64 }, ptr %local_id, align 4
  store { i64, i64, i64 } %207, ptr %14, align 4
  %208 = getelementptr i8, ptr %14, i64 0
  %209 = load i64, ptr %208, align 1
  %210 = load i64, ptr %expected_local_x, align 4
  %211 = icmp eq i64 %209, %210
  %212 = zext i1 %211 to i8
  br label %if.merge68

if.else67:                                        ; preds = %if.merge65
  br label %if.merge68

if.merge68:                                       ; preds = %if.else67, %if.then66
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_104" = phi i8 [ %212, %if.then66 ], [ 0, %if.else67 ]
  %213 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_104", 0
  %214 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_104", 0
  br i1 %214, label %if.then69, label %if.else70

if.then69:                                        ; preds = %if.merge68
  %215 = load { i64, i64, i64 }, ptr %local_id, align 4
  store { i64, i64, i64 } %215, ptr %13, align 4
  %216 = getelementptr i8, ptr %13, i64 8
  %217 = load i64, ptr %216, align 1
  %218 = load i64, ptr %expected_local_y, align 4
  %219 = icmp eq i64 %217, %218
  %220 = zext i1 %219 to i8
  br label %if.merge71

if.else70:                                        ; preds = %if.merge68
  br label %if.merge71

if.merge71:                                       ; preds = %if.else70, %if.then69
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_107" = phi i8 [ %220, %if.then69 ], [ 0, %if.else70 ]
  %221 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_107", 0
  %222 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_107", 0
  br i1 %222, label %if.then72, label %if.else73

if.then72:                                        ; preds = %if.merge71
  %223 = load { i64, i64, i64 }, ptr %local_id, align 4
  store { i64, i64, i64 } %223, ptr %12, align 4
  %224 = getelementptr i8, ptr %12, i64 16
  %225 = load i64, ptr %224, align 1
  %226 = load i64, ptr %expected_local_z, align 4
  %227 = icmp eq i64 %225, %226
  %228 = zext i1 %227 to i8
  br label %if.merge74

if.else73:                                        ; preds = %if.merge71
  br label %if.merge74

if.merge74:                                       ; preds = %if.else73, %if.then72
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_110" = phi i8 [ %228, %if.then72 ], [ 0, %if.else73 ]
  %229 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_110", 0
  %230 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_110", 0
  br i1 %230, label %if.then75, label %if.else76

if.then75:                                        ; preds = %if.merge74
  %231 = load { i64, i64, i64 }, ptr %workgroup_id, align 4
  store { i64, i64, i64 } %231, ptr %11, align 4
  %232 = getelementptr i8, ptr %11, i64 0
  %233 = load i64, ptr %232, align 1
  %234 = load i64, ptr %expected_workgroup_x, align 4
  %235 = icmp eq i64 %233, %234
  %236 = zext i1 %235 to i8
  br label %if.merge77

if.else76:                                        ; preds = %if.merge74
  br label %if.merge77

if.merge77:                                       ; preds = %if.else76, %if.then75
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_113" = phi i8 [ %236, %if.then75 ], [ 0, %if.else76 ]
  %237 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_113", 0
  %238 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_113", 0
  br i1 %238, label %if.then78, label %if.else79

if.then78:                                        ; preds = %if.merge77
  %239 = load { i64, i64, i64 }, ptr %workgroup_id, align 4
  store { i64, i64, i64 } %239, ptr %10, align 4
  %240 = getelementptr i8, ptr %10, i64 8
  %241 = load i64, ptr %240, align 1
  %242 = icmp eq i64 %241, 0
  %243 = zext i1 %242 to i8
  br label %if.merge80

if.else79:                                        ; preds = %if.merge77
  br label %if.merge80

if.merge80:                                       ; preds = %if.else79, %if.then78
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_117" = phi i8 [ %243, %if.then78 ], [ 0, %if.else79 ]
  %244 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_117", 0
  %245 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_117", 0
  br i1 %245, label %if.then81, label %if.else82

if.then81:                                        ; preds = %if.merge80
  %246 = load { i64, i64, i64 }, ptr %workgroup_id, align 4
  store { i64, i64, i64 } %246, ptr %9, align 4
  %247 = getelementptr i8, ptr %9, i64 16
  %248 = load i64, ptr %247, align 1
  %249 = icmp eq i64 %248, 0
  %250 = zext i1 %249 to i8
  br label %if.merge83

if.else82:                                        ; preds = %if.merge80
  br label %if.merge83

if.merge83:                                       ; preds = %if.else82, %if.then81
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_121" = phi i8 [ %250, %if.then81 ], [ 0, %if.else82 ]
  %251 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_121", 0
  %252 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_121", 0
  br i1 %252, label %if.then84, label %if.else85

if.then84:                                        ; preds = %if.merge83
  %253 = load { i64, i64, i64 }, ptr %workgroup_size, align 4
  store { i64, i64, i64 } %253, ptr %8, align 4
  %254 = getelementptr i8, ptr %8, i64 0
  %255 = load i64, ptr %254, align 1
  %256 = icmp eq i64 %255, 2
  %257 = zext i1 %256 to i8
  br label %if.merge86

if.else85:                                        ; preds = %if.merge83
  br label %if.merge86

if.merge86:                                       ; preds = %if.else85, %if.then84
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_125" = phi i8 [ %257, %if.then84 ], [ 0, %if.else85 ]
  %258 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_125", 0
  %259 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_125", 0
  br i1 %259, label %if.then87, label %if.else88

if.then87:                                        ; preds = %if.merge86
  %260 = load { i64, i64, i64 }, ptr %workgroup_size, align 4
  store { i64, i64, i64 } %260, ptr %7, align 4
  %261 = getelementptr i8, ptr %7, i64 8
  %262 = load i64, ptr %261, align 1
  %263 = icmp eq i64 %262, 2
  %264 = zext i1 %263 to i8
  br label %if.merge89

if.else88:                                        ; preds = %if.merge86
  br label %if.merge89

if.merge89:                                       ; preds = %if.else88, %if.then87
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_129" = phi i8 [ %264, %if.then87 ], [ 0, %if.else88 ]
  %265 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_129", 0
  %266 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_129", 0
  br i1 %266, label %if.then90, label %if.else91

if.then90:                                        ; preds = %if.merge89
  %267 = load { i64, i64, i64 }, ptr %workgroup_size, align 4
  store { i64, i64, i64 } %267, ptr %6, align 4
  %268 = getelementptr i8, ptr %6, i64 16
  %269 = load i64, ptr %268, align 1
  %270 = icmp eq i64 %269, 2
  %271 = zext i1 %270 to i8
  br label %if.merge92

if.else91:                                        ; preds = %if.merge89
  br label %if.merge92

if.merge92:                                       ; preds = %if.else91, %if.then90
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_133" = phi i8 [ %271, %if.then90 ], [ 0, %if.else91 ]
  %272 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_133", 0
  %273 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_133", 0
  br i1 %273, label %if.then93, label %if.else94

if.then93:                                        ; preds = %if.merge92
  %274 = load { i64, i64, i64 }, ptr %global_size, align 4
  store { i64, i64, i64 } %274, ptr %5, align 4
  %275 = getelementptr i8, ptr %5, i64 0
  %276 = load i64, ptr %275, align 1
  %277 = icmp eq i64 %276, 4
  %278 = zext i1 %277 to i8
  br label %if.merge95

if.else94:                                        ; preds = %if.merge92
  br label %if.merge95

if.merge95:                                       ; preds = %if.else94, %if.then93
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_137" = phi i8 [ %278, %if.then93 ], [ 0, %if.else94 ]
  %279 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_137", 0
  %280 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_137", 0
  br i1 %280, label %if.then96, label %if.else97

if.then96:                                        ; preds = %if.merge95
  %281 = load { i64, i64, i64 }, ptr %global_size, align 4
  store { i64, i64, i64 } %281, ptr %4, align 4
  %282 = getelementptr i8, ptr %4, i64 8
  %283 = load i64, ptr %282, align 1
  %284 = icmp eq i64 %283, 2
  %285 = zext i1 %284 to i8
  br label %if.merge98

if.else97:                                        ; preds = %if.merge95
  br label %if.merge98

if.merge98:                                       ; preds = %if.else97, %if.then96
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_141" = phi i8 [ %285, %if.then96 ], [ 0, %if.else97 ]
  %286 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_141", 0
  %287 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_141", 0
  br i1 %287, label %if.then99, label %if.else100

if.then99:                                        ; preds = %if.merge98
  %288 = load { i64, i64, i64 }, ptr %global_size, align 4
  store { i64, i64, i64 } %288, ptr %3, align 4
  %289 = getelementptr i8, ptr %3, i64 16
  %290 = load i64, ptr %289, align 1
  %291 = icmp eq i64 %290, 2
  %292 = zext i1 %291 to i8
  br label %if.merge101

if.else100:                                       ; preds = %if.merge98
  br label %if.merge101

if.merge101:                                      ; preds = %if.else100, %if.then99
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_145" = phi i8 [ %292, %if.then99 ], [ 0, %if.else100 ]
  %293 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_145", 0
  %294 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_145", 0
  br i1 %294, label %if.then102, label %if.else103

if.then102:                                       ; preds = %if.merge101
  %295 = load { i64, i64, i64 }, ptr %num_workgroups, align 4
  store { i64, i64, i64 } %295, ptr %2, align 4
  %296 = getelementptr i8, ptr %2, i64 0
  %297 = load i64, ptr %296, align 1
  %298 = icmp eq i64 %297, 2
  %299 = zext i1 %298 to i8
  br label %if.merge104

if.else103:                                       ; preds = %if.merge101
  br label %if.merge104

if.merge104:                                      ; preds = %if.else103, %if.then102
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_149" = phi i8 [ %299, %if.then102 ], [ 0, %if.else103 ]
  %300 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_149", 0
  %301 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_149", 0
  br i1 %301, label %if.then105, label %if.else106

if.then105:                                       ; preds = %if.merge104
  %302 = load { i64, i64, i64 }, ptr %num_workgroups, align 4
  store { i64, i64, i64 } %302, ptr %1, align 4
  %303 = getelementptr i8, ptr %1, i64 8
  %304 = load i64, ptr %303, align 1
  %305 = icmp eq i64 %304, 1
  %306 = zext i1 %305 to i8
  br label %if.merge107

if.else106:                                       ; preds = %if.merge104
  br label %if.merge107

if.merge107:                                      ; preds = %if.else106, %if.then105
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_153" = phi i8 [ %306, %if.then105 ], [ 0, %if.else106 ]
  %307 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_153", 0
  %308 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_153", 0
  br i1 %308, label %if.then108, label %if.else109

if.then108:                                       ; preds = %if.merge107
  %309 = load { i64, i64, i64 }, ptr %num_workgroups, align 4
  store { i64, i64, i64 } %309, ptr %0, align 4
  %310 = getelementptr i8, ptr %0, i64 16
  %311 = load i64, ptr %310, align 1
  %312 = icmp eq i64 %311, 1
  %313 = zext i1 %312 to i8
  br label %if.merge110

if.else109:                                       ; preds = %if.merge107
  br label %if.merge110

if.merge110:                                      ; preds = %if.else109, %if.then108
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_157" = phi i8 [ %313, %if.then108 ], [ 0, %if.else109 ]
  %314 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_157", 0
  %315 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_157", 0
  br i1 %315, label %if.then111, label %if.else112

if.then111:                                       ; preds = %if.merge110
  %316 = call i64 @ultraviolet_x3a_x3aruntime_x3a_x3agpu_x3a_x3alinear_x5fid()
  %317 = load i64, ptr %relative_index, align 4
  %318 = icmp eq i64 %316, %317
  %319 = zext i1 %318 to i8
  br label %if.merge113

if.else112:                                       ; preds = %if.merge110
  br label %if.merge113

if.merge113:                                      ; preds = %if.else112, %if.then111
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_160" = phi i8 [ %319, %if.then111 ], [ 0, %if.else112 ]
  %320 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_160", 0
  %321 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_160", 0
  br i1 %321, label %if.then114, label %if.else115

if.then114:                                       ; preds = %if.merge113
  %322 = load i8, ptr %forms_have_pointer_layout, align 1
  %323 = load i8, ptr %forms_have_pointer_layout, align 1
  br label %if.merge116

if.else115:                                       ; preds = %if.merge113
  br label %if.merge116

if.merge116:                                      ; preds = %if.else115, %if.then114
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_161" = phi i8 [ %323, %if.then114 ], [ 0, %if.else115 ]
  %324 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_161", 0
  %325 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_161", 0
  br i1 %325, label %if.then117, label %if.else118

if.then117:                                       ; preds = %if.merge116
  %326 = load i32, ptr %private_read, align 4
  %327 = icmp eq i32 %326, 41
  %328 = zext i1 %327 to i8
  br label %if.merge119

if.else118:                                       ; preds = %if.merge116
  br label %if.merge119

if.merge119:                                      ; preds = %if.else118, %if.then117
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_164" = phi i8 [ %328, %if.then117 ], [ 0, %if.else118 ]
  %329 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$and_164", 0
  %330 = zext i1 %329 to i8
  store i8 %330, ptr %matches_topology, align 1
  %331 = load i8, ptr %matches_topology, align 1
  %332 = icmp ne i8 %331, 0
  %333 = load i8, ptr %matches_topology, align 1
  %334 = icmp ne i8 %333, 0
  br i1 %334, label %if.then120, label %if.else121

if.then120:                                       ; preds = %if.merge119
  br i1 true, label %check_ok123, label %check_fail124

if.else121:                                       ; preds = %if.merge119
  br i1 true, label %check_ok130, label %check_fail131

if.merge122:                                      ; preds = %op_ok135, %op_ok128
  %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$if_185" = phi i64 [ %355, %op_ok128 ], [ %375, %op_ok135 ]
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64 6)
  %335 = load ptr, ptr %result4, align 8
  store i64 %"GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence$tmp$if_185", ptr %335, align 4
  %336 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$06")
  store { i64, i64 } %336, ptr %abi_return137, align 8
  %337 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return137, align 1
  ret void

check_ok123:                                      ; preds = %check_fail124, %if.then120
  %338 = load ptr, ptr %__panic5, align 8
  %339 = load i8, ptr %338, align 1
  %340 = icmp ne i8 %339, 0
  br i1 %340, label %panic.take125, label %panic.cont126

check_fail124:                                    ; preds = %if.then120
  %341 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %341, align 1
  %342 = getelementptr i8, ptr %341, i64 4
  store i32 4, ptr %342, align 4
  br label %check_ok123

panic.take125:                                    ; preds = %check_ok123
  %343 = load ptr, ptr %__panic5, align 8
  %344 = getelementptr i8, ptr %343, i64 0
  %345 = load i8, ptr %344, align 1
  %346 = load ptr, ptr %__panic5, align 8
  %347 = getelementptr i8, ptr %346, i64 4
  %348 = load i32, ptr %347, align 4
  %349 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$06")
  store { i64, i64 } %349, ptr %abi_return127, align 8
  %350 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return127, align 1
  ret void

panic.cont126:                                    ; preds = %check_ok123
  %351 = load i64, ptr %relative_index, align 4
  %352 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %351, i64 1)
  %353 = extractvalue { i64, i1 } %352, 0
  %354 = extractvalue { i64, i1 } %352, 1
  %355 = freeze i64 %353
  br i1 %354, label %op_fail129, label %op_ok128

op_ok128:                                         ; preds = %panic.cont126
  br label %if.merge122

op_fail129:                                       ; preds = %panic.cont126
  %356 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %356, align 1
  %357 = getelementptr i8, ptr %356, i64 4
  store i32 4, ptr %357, align 4
  ret void

check_ok130:                                      ; preds = %check_fail131, %if.else121
  %358 = load ptr, ptr %__panic5, align 8
  %359 = load i8, ptr %358, align 1
  %360 = icmp ne i8 %359, 0
  br i1 %360, label %panic.take132, label %panic.cont133

check_fail131:                                    ; preds = %if.else121
  %361 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %361, align 1
  %362 = getelementptr i8, ptr %361, i64 4
  store i32 4, ptr %362, align 4
  br label %check_ok130

panic.take132:                                    ; preds = %check_ok130
  %363 = load ptr, ptr %__panic5, align 8
  %364 = getelementptr i8, ptr %363, i64 0
  %365 = load i8, ptr %364, align 1
  %366 = load ptr, ptr %__panic5, align 8
  %367 = getelementptr i8, ptr %366, i64 4
  %368 = load i32, ptr %367, align 4
  %369 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %"region$06")
  store { i64, i64 } %369, ptr %abi_return134, align 8
  %370 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return134, align 1
  ret void

panic.cont133:                                    ; preds = %check_ok130
  %371 = load i64, ptr %relative_index, align 4
  %372 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 1000, i64 %371)
  %373 = extractvalue { i64, i1 } %372, 0
  %374 = extractvalue { i64, i1 } %372, 1
  %375 = freeze i64 %373
  br i1 %374, label %op_fail136, label %op_ok135

op_ok135:                                         ; preds = %panic.cont133
  br label %if.merge122

op_fail136:                                       ; preds = %panic.cont133
  %376 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %376, align 1
  %377 = getelementptr i8, ptr %376, i64 4
  store i32 4, ptr %377, align 4
  ret void
}

define i8 @GpuRuntimeEvidence_x3a_x3adomainLoweringEvidence(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %cpu_value = alloca i32, align 4
  %coerce_bits13 = alloca { ptr, ptr }, align 8
  %abi_return12 = alloca { i64, i64 }, align 8
  %inline_value = alloca i32, align 4
  %byref_arg = alloca i32, align 4
  %coerce_bits = alloca { ptr, ptr }, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aGpuRuntimeEvidence, align 1
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
  %10 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3ainline(ptr noundef nonnull align 8 dereferenceable(96) %context)
  store { i64, i64 } %10, ptr %abi_return, align 8
  %11 = load { ptr, ptr }, ptr %abi_return, align 1
  store { ptr, ptr } %11, ptr %coerce_bits, align 8
  %12 = load { i64, i64 }, ptr %coerce_bits, align 1
  %13 = call ptr @uv_parallel_begin({ i64, i64 } %12, i64 -1, ptr null)
  %14 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %14, align 1
  %15 = getelementptr i8, ptr %14, i64 4
  store i32 0, ptr %15, align 4
  %16 = call i32 @uv_parallel_join(ptr %13)
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
  %55 = trunc i32 %54 to i8
  ret i8 %55

panic.cont:                                       ; preds = %if.merge10
  store i32 7, ptr %inline_value, align 4
  %56 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3acontext_x3a_x3acpu(ptr noundef nonnull align 8 dereferenceable(96) %context)
  store { i64, i64 } %56, ptr %abi_return12, align 8
  %57 = load { ptr, ptr }, ptr %abi_return12, align 1
  store { ptr, ptr } %57, ptr %coerce_bits13, align 8
  %58 = load { i64, i64 }, ptr %coerce_bits13, align 1
  %59 = call ptr @uv_parallel_begin({ i64, i64 } %58, i64 -1, ptr null)
  %60 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %60, align 1
  %61 = getelementptr i8, ptr %60, i64 4
  store i32 0, ptr %61, align 4
  %62 = call i32 @uv_parallel_join(ptr %59)
  %63 = icmp eq i32 %62, 0
  %64 = xor i1 %63, true
  %65 = zext i1 %64 to i8
  %66 = icmp ne i8 %65, 0
  %67 = icmp ne i8 %65, 0
  br i1 %67, label %if.then14, label %if.else15

if.then14:                                        ; preds = %panic.cont
  %68 = load ptr, ptr %__panic, align 8
  %69 = getelementptr i8, ptr %68, i64 0
  store i8 1, ptr %69, align 1
  %70 = load ptr, ptr %__panic, align 8
  %71 = getelementptr i8, ptr %70, i64 4
  store i32 %62, ptr %71, align 4
  br label %if.merge16

if.else15:                                        ; preds = %panic.cont
  br label %if.merge16

if.merge16:                                       ; preds = %if.else15, %if.then14
  %if.result17 = phi i64 [ 0, %if.then14 ], [ 0, %if.else15 ]
  %72 = load ptr, ptr %__panic, align 8
  %73 = getelementptr i8, ptr %72, i64 0
  %74 = load i8, ptr %73, align 1
  %75 = load ptr, ptr %__panic, align 8
  %76 = getelementptr i8, ptr %75, i64 4
  %77 = load i32, ptr %76, align 4
  %78 = icmp ne i8 %74, 0
  %79 = zext i1 %78 to i8
  %80 = icmp ne i8 %79, 0
  %81 = and i1 false, %80
  br i1 %81, label %if.then18, label %if.else19

if.then18:                                        ; preds = %if.merge16
  store i32 %77, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else19:                                        ; preds = %if.merge16
  br label %if.merge20

if.merge20:                                       ; preds = %if.else19
  %82 = icmp ne i8 %74, 0
  %83 = xor i1 %82, true
  %84 = and i1 false, %83
  br i1 %84, label %if.then21, label %if.else22

if.then21:                                        ; preds = %if.merge20
  %85 = load ptr, ptr %__panic, align 8
  %86 = getelementptr i8, ptr %85, i64 0
  store i8 1, ptr %86, align 1
  %87 = load ptr, ptr %__panic, align 8
  %88 = getelementptr i8, ptr %87, i64 4
  store i32 0, ptr %88, align 4
  br label %if.merge23

if.else22:                                        ; preds = %if.merge20
  br label %if.merge23

if.merge23:                                       ; preds = %if.else22, %if.then21
  %if.result24 = phi i64 [ 0, %if.then21 ], [ 0, %if.else22 ]
  %89 = icmp ne i8 %74, 0
  %90 = zext i1 %89 to i8
  %91 = icmp ne i8 %90, 0
  %92 = or i1 false, %91
  %93 = icmp ne i8 %74, 0
  %94 = icmp ne i8 %74, 0
  br i1 %94, label %if.then25, label %if.else26

if.then25:                                        ; preds = %if.merge23
  br label %if.merge27

if.else26:                                        ; preds = %if.merge23
  br label %if.merge27

if.merge27:                                       ; preds = %if.else26, %if.then25
  %if.result28 = phi i32 [ %77, %if.then25 ], [ 0, %if.else26 ]
  %95 = load ptr, ptr %__panic, align 8
  %96 = load i8, ptr %95, align 1
  %97 = icmp ne i8 %96, 0
  br i1 %97, label %panic.take29, label %panic.cont30

panic.take29:                                     ; preds = %if.merge27
  %98 = load ptr, ptr %__panic, align 8
  %99 = getelementptr i8, ptr %98, i64 4
  %100 = load i32, ptr %99, align 4
  %101 = trunc i32 %100 to i8
  ret i8 %101

panic.cont30:                                     ; preds = %if.merge27
  store i32 11, ptr %cpu_value, align 4
  %102 = load i32, ptr %inline_value, align 4
  %103 = icmp eq i32 %102, 7
  %104 = zext i1 %103 to i8
  %105 = icmp ne i8 %104, 0
  %106 = icmp ne i8 %104, 0
  br i1 %106, label %if.then31, label %if.else32

if.then31:                                        ; preds = %panic.cont30
  %107 = load i32, ptr %cpu_value, align 4
  %108 = icmp eq i32 %107, 11
  %109 = zext i1 %108 to i8
  br label %if.merge33

if.else32:                                        ; preds = %panic.cont30
  br label %if.merge33

if.merge33:                                       ; preds = %if.else32, %if.then31
  %"GpuRuntimeEvidence_x3a_x3adomainLoweringEvidence$tmp$and_280" = phi i8 [ %109, %if.then31 ], [ 0, %if.else32 ]
  %110 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3adomainLoweringEvidence$tmp$and_280", 0
  %111 = zext i1 %110 to i8
  ret i8 %111
}

define i32 @GpuRuntimeEvidence_x3a_x3amain(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aGpuRuntimeEvidence, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aGpuRuntimeEvidence, align 1
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
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aGpuRuntimeEvidence, align 1
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
  %23 = call i8 @GpuRuntimeEvidence_x3a_x3agpuRuntimeEvidence(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
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
  %31 = icmp ne i8 %23, 0
  br i1 %31, label %if.then, label %if.else

if.then:                                          ; preds = %panic.cont
  %32 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aGpuRuntimeEvidence, align 1
  %33 = icmp ne i8 %32, 0
  br i1 %33, label %poison.take5, label %poison.cont6

if.else:                                          ; preds = %panic.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else, %panic.cont10
  %"GpuRuntimeEvidence_x3a_x3amain$tmp$and_292" = phi i8 [ %48, %panic.cont10 ], [ 0, %if.else ]
  %34 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3amain$tmp$and_292", 0
  %35 = icmp ne i8 %"GpuRuntimeEvidence_x3a_x3amain$tmp$and_292", 0
  br i1 %35, label %if.then11, label %if.else12

poison.take5:                                     ; preds = %if.then
  %36 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %36, align 1
  %37 = getelementptr i8, ptr %36, i64 4
  store i32 10, ptr %37, align 4
  %38 = load ptr, ptr %__panic, align 8
  %39 = getelementptr i8, ptr %38, i64 4
  %40 = load i32, ptr %39, align 4
  ret i32 %40

poison.cont6:                                     ; preds = %if.then
  %41 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aGpuRuntimeEvidence, align 1
  %42 = icmp ne i8 %41, 0
  br i1 %42, label %poison.take7, label %poison.cont8

poison.take7:                                     ; preds = %poison.cont6
  %43 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %43, align 1
  %44 = getelementptr i8, ptr %43, i64 4
  store i32 10, ptr %44, align 4
  %45 = load ptr, ptr %__panic, align 8
  %46 = getelementptr i8, ptr %45, i64 4
  %47 = load i32, ptr %46, align 4
  ret i32 %47

poison.cont8:                                     ; preds = %poison.cont6
  %48 = call i8 @GpuRuntimeEvidence_x3a_x3adomainLoweringEvidence(ptr noundef nonnull align 8 dereferenceable(96) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %49 = load ptr, ptr %__panic, align 8
  %50 = load i8, ptr %49, align 1
  %51 = icmp ne i8 %50, 0
  br i1 %51, label %panic.take9, label %panic.cont10

panic.take9:                                      ; preds = %poison.cont8
  %52 = load ptr, ptr %__panic, align 8
  %53 = getelementptr i8, ptr %52, i64 4
  %54 = load i32, ptr %53, align 4
  ret i32 %54

panic.cont10:                                     ; preds = %poison.cont8
  br label %if.merge

if.then11:                                        ; preds = %if.merge
  ret i32 0

if.else12:                                        ; preds = %if.merge
  br label %if.merge13

if.merge13:                                       ; preds = %if.else12
  ret i32 1
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aGpuRuntimeEvidence(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aGpuRuntimeEvidence(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #2

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64) #3

define void @__cx_lifecycle_init_GpuRuntimeEvidence(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aGpuRuntimeEvidence(ptr %0)
  ret void
}

define void @__cx_lifecycle_deinit_GpuRuntimeEvidence(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aGpuRuntimeEvidence(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aGpuRuntimeEvidence(ptr %entry_panic_out)
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
  %24 = call i32 @GpuRuntimeEvidence_x3a_x3amain(ptr %entry_ctx, ptr %entry_panic_out)
  %entry_deinit_panic_seen = alloca i8, align 1
  %entry_deinit_panic_code = alloca i32, align 4
  store i8 0, ptr %entry_deinit_panic_seen, align 1
  store i32 0, ptr %entry_deinit_panic_code, align 4
  %25 = load i8, ptr %entry_panic, align 1
  %26 = icmp ne i8 %25, 0
  br i1 %26, label %entry.panic, label %entry.deinit

entry.deinit:                                     ; preds = %entry.init.cont
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aGpuRuntimeEvidence(ptr %entry_panic_out)
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
attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #3 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
