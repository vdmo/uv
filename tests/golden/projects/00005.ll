; ==== Source/HostedExportLibrary.ll
; ModuleID = 'HostedExportLibrary'
source_filename = "HostedExportLibrary"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aHostedExportLibrary = hidden global i8 0
@HostedExportLibrary_x3a_x3a_x5fHOSTED_x5fSTATE_x5fREFERENCE = internal global i32 0, align 4
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAD2ACA7747985764 = internal constant [4 x i8] c"\01\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fD8C5C4CC67AFA0D8 = internal constant [16 x i8] c"HelloUltraviolet", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f6D3572669B2CDE42 = internal constant [4 x i8] c"\07\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2D401A55EEC16520 = internal constant [4 x i8] c"\05\00\00\00", align 1
@__uv_library_attached = internal global i1 false
@__uv_image_panic_record = common hidden global { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, align 4
@llvm.global_ctors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_ctor, ptr null }]
@llvm.global_dtors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_dtor, ptr null }]
@__uv_host_session_owner_token = hidden global i8 0

; Function Attrs: nounwind
declare i8 @ultraviolet_x3a_x3aruntime_x3a_x3aio_x3a_x3aexists(ptr noundef nonnull align 8 dereferenceable(16), ptr noundef nonnull align 8 dereferenceable(16)) #0

define hidden i32 @HostedExportLibrary_x5fx3a_x5fx3ahostedReferenceIncrement_x3a_x3a_x5f_x5fhost_x5fbody(ptr noundef nonnull align 8 dereferenceable(16) %context, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = call ptr @uv_host_session_current_env()
  %1 = icmp ne ptr %0, null
  br i1 %1, label %hosted.state.session, label %hosted.state.fallback

hosted.state.session:                             ; preds = %entry
  %2 = getelementptr i8, ptr %0, i64 108
  br label %hosted.state.merge

hosted.state.fallback:                            ; preds = %entry
  br label %hosted.state.merge

hosted.state.merge:                               ; preds = %hosted.state.fallback, %hosted.state.session
  %3 = phi ptr [ %2, %hosted.state.session ], [ @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aHostedExportLibrary, %hosted.state.fallback ]
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

define hidden i32 @HostedExportLibrary_x5fx3a_x5fx3ahostedReferencePairSum_x3a_x3a_x5f_x5fhost_x5fbody(ptr noundef nonnull align 8 dereferenceable(16) %context, ptr noundef nonnull align 4 dereferenceable(8) %pair, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = call ptr @uv_host_session_current_env()
  %1 = icmp ne ptr %0, null
  br i1 %1, label %hosted.state.session, label %hosted.state.fallback

hosted.state.session:                             ; preds = %entry
  %2 = getelementptr i8, ptr %0, i64 108
  br label %hosted.state.merge

hosted.state.fallback:                            ; preds = %entry
  br label %hosted.state.merge

hosted.state.merge:                               ; preds = %hosted.state.fallback, %hosted.state.session
  %3 = phi ptr [ %2, %hosted.state.session ], [ @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aHostedExportLibrary, %hosted.state.fallback ]
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
  %21 = getelementptr i8, ptr %pair, i64 0
  %22 = load i32, ptr %21, align 1
  %23 = getelementptr i8, ptr %pair, i64 4
  %24 = load i32, ptr %23, align 1
  %25 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %22, i32 %24)
  %26 = extractvalue { i32, i1 } %25, 0
  %27 = extractvalue { i32, i1 } %25, 1
  %28 = freeze i32 %26
  br i1 %27, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  ret i32 %28

op_fail:                                          ; preds = %panic.cont
  %29 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %29, align 1
  %30 = getelementptr i8, ptr %29, i64 4
  store i32 4, ptr %30, align 4
  ret i32 0
}

define hidden { i32, i32, [0 x i32] } @HostedExportLibrary_x5fx3a_x5fx3ahostedReferencePairEcho_x3a_x3a_x5f_x5fhost_x5fbody(ptr noundef nonnull align 8 dereferenceable(16) %context, ptr noundef nonnull align 4 dereferenceable(8) %pair, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i32, i32, [0 x i32] }, align 8
  %1 = call ptr @uv_host_session_current_env()
  %2 = icmp ne ptr %1, null
  br i1 %2, label %hosted.state.session, label %hosted.state.fallback

hosted.state.session:                             ; preds = %entry
  %3 = getelementptr i8, ptr %1, i64 108
  br label %hosted.state.merge

hosted.state.fallback:                            ; preds = %entry
  br label %hosted.state.merge

hosted.state.merge:                               ; preds = %hosted.state.fallback, %hosted.state.session
  %4 = phi ptr [ %3, %hosted.state.session ], [ @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aHostedExportLibrary, %hosted.state.fallback ]
  %5 = load i8, ptr %4, align 1
  %6 = icmp ne i8 %5, 0
  br i1 %6, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %hosted.state.merge
  %7 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %7, align 1
  %8 = getelementptr i8, ptr %7, i64 4
  store i32 10, ptr %8, align 4
  ret { i32, i32, [0 x i32] } zeroinitializer

poison.cont:                                      ; preds = %hosted.state.merge
  %9 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 0, ptr %10, align 4
  store { i32, i32, [0 x i32] } zeroinitializer, ptr %0, align 4
  %11 = getelementptr i8, ptr %pair, i64 0
  %12 = load i32, ptr %11, align 1
  %13 = getelementptr i8, ptr %0, i64 0
  store i32 %12, ptr %13, align 1
  %14 = getelementptr i8, ptr %pair, i64 4
  %15 = load i32, ptr %14, align 1
  %16 = getelementptr i8, ptr %0, i64 4
  store i32 %15, ptr %16, align 1
  %17 = load { i32, i32, [0 x i32] }, ptr %0, align 4
  ret { i32, i32, [0 x i32] } %17
}

define hidden i32 @HostedExportLibrary_x5fx3a_x5fx3ahostedReferenceTripleSum_x3a_x3a_x5f_x5fhost_x5fbody(ptr noundef nonnull align 8 dereferenceable(16) %context, ptr noundef nonnull align 4 dereferenceable(12) %triple, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = call ptr @uv_host_session_current_env()
  %1 = icmp ne ptr %0, null
  br i1 %1, label %hosted.state.session, label %hosted.state.fallback

hosted.state.session:                             ; preds = %entry
  %2 = getelementptr i8, ptr %0, i64 108
  br label %hosted.state.merge

hosted.state.fallback:                            ; preds = %entry
  br label %hosted.state.merge

hosted.state.merge:                               ; preds = %hosted.state.fallback, %hosted.state.session
  %3 = phi ptr [ %2, %hosted.state.session ], [ @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aHostedExportLibrary, %hosted.state.fallback ]
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
  %21 = getelementptr i8, ptr %triple, i64 0
  %22 = load i32, ptr %21, align 1
  %23 = getelementptr i8, ptr %triple, i64 4
  %24 = load i32, ptr %23, align 1
  %25 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %22, i32 %24)
  %26 = extractvalue { i32, i1 } %25, 0
  %27 = extractvalue { i32, i1 } %25, 1
  %28 = freeze i32 %26
  br i1 %27, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  br i1 true, label %check_ok1, label %check_fail2

op_fail:                                          ; preds = %panic.cont
  %29 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %29, align 1
  %30 = getelementptr i8, ptr %29, i64 4
  store i32 4, ptr %30, align 4
  ret i32 0

check_ok1:                                        ; preds = %check_fail2, %op_ok
  %31 = load ptr, ptr %__panic, align 8
  %32 = load i8, ptr %31, align 1
  %33 = icmp ne i8 %32, 0
  br i1 %33, label %panic.take3, label %panic.cont4

check_fail2:                                      ; preds = %op_ok
  %34 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %34, align 1
  %35 = getelementptr i8, ptr %34, i64 4
  store i32 4, ptr %35, align 4
  br label %check_ok1

panic.take3:                                      ; preds = %check_ok1
  %36 = load ptr, ptr %__panic, align 8
  %37 = getelementptr i8, ptr %36, i64 4
  %38 = load i32, ptr %37, align 4
  ret i32 %38

panic.cont4:                                      ; preds = %check_ok1
  %39 = getelementptr i8, ptr %triple, i64 8
  %40 = load i32, ptr %39, align 1
  %41 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %28, i32 %40)
  %42 = extractvalue { i32, i1 } %41, 0
  %43 = extractvalue { i32, i1 } %41, 1
  %44 = freeze i32 %42
  br i1 %43, label %op_fail6, label %op_ok5

op_ok5:                                           ; preds = %panic.cont4
  ret i32 %44

op_fail6:                                         ; preds = %panic.cont4
  %45 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %45, align 1
  %46 = getelementptr i8, ptr %45, i64 4
  store i32 4, ptr %46, align 4
  ret i32 0
}

define hidden { i32, i32, i32, [0 x i32] } @HostedExportLibrary_x5fx3a_x5fx3ahostedReferenceTripleEcho_x3a_x3a_x5f_x5fhost_x5fbody(ptr noundef nonnull align 8 dereferenceable(16) %context, ptr noundef nonnull align 4 dereferenceable(12) %triple, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i32, i32, i32, [0 x i32] }, align 8
  %1 = call ptr @uv_host_session_current_env()
  %2 = icmp ne ptr %1, null
  br i1 %2, label %hosted.state.session, label %hosted.state.fallback

hosted.state.session:                             ; preds = %entry
  %3 = getelementptr i8, ptr %1, i64 108
  br label %hosted.state.merge

hosted.state.fallback:                            ; preds = %entry
  br label %hosted.state.merge

hosted.state.merge:                               ; preds = %hosted.state.fallback, %hosted.state.session
  %4 = phi ptr [ %3, %hosted.state.session ], [ @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aHostedExportLibrary, %hosted.state.fallback ]
  %5 = load i8, ptr %4, align 1
  %6 = icmp ne i8 %5, 0
  br i1 %6, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %hosted.state.merge
  %7 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %7, align 1
  %8 = getelementptr i8, ptr %7, i64 4
  store i32 10, ptr %8, align 4
  ret { i32, i32, i32, [0 x i32] } zeroinitializer

poison.cont:                                      ; preds = %hosted.state.merge
  %9 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 0, ptr %10, align 4
  store { i32, i32, i32, [0 x i32] } zeroinitializer, ptr %0, align 4
  %11 = getelementptr i8, ptr %triple, i64 0
  %12 = load i32, ptr %11, align 1
  %13 = getelementptr i8, ptr %0, i64 0
  store i32 %12, ptr %13, align 1
  %14 = getelementptr i8, ptr %triple, i64 4
  %15 = load i32, ptr %14, align 1
  %16 = getelementptr i8, ptr %0, i64 4
  store i32 %15, ptr %16, align 1
  %17 = getelementptr i8, ptr %triple, i64 8
  %18 = load i32, ptr %17, align 1
  %19 = getelementptr i8, ptr %0, i64 8
  store i32 %18, ptr %19, align 1
  %20 = load { i32, i32, i32, [0 x i32] }, ptr %0, align 4
  ret { i32, i32, i32, [0 x i32] } %20
}

define hidden void @HostedExportLibrary_x5fx3a_x5fx3ahostedReferenceLargeEcho_x3a_x3a_x5f_x5fhost_x5fbody(ptr noalias noundef nonnull sret({ i64, i64, i64, [0 x i64] }) align 8 dereferenceable(24) %0, ptr noundef nonnull align 8 dereferenceable(16) %context, ptr noundef nonnull align 8 dereferenceable(24) %large, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %aggregate.literal = alloca { i64, i64, i64, [0 x i64] }, align 8
  %1 = call ptr @uv_host_session_current_env()
  %2 = icmp ne ptr %1, null
  br i1 %2, label %hosted.state.session, label %hosted.state.fallback

hosted.state.session:                             ; preds = %entry
  %3 = getelementptr i8, ptr %1, i64 108
  br label %hosted.state.merge

hosted.state.fallback:                            ; preds = %entry
  br label %hosted.state.merge

hosted.state.merge:                               ; preds = %hosted.state.fallback, %hosted.state.session
  %4 = phi ptr [ %3, %hosted.state.session ], [ @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aHostedExportLibrary, %hosted.state.fallback ]
  %5 = load i8, ptr %4, align 1
  %6 = icmp ne i8 %5, 0
  br i1 %6, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %hosted.state.merge
  %7 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %7, align 1
  %8 = getelementptr i8, ptr %7, i64 4
  store i32 10, ptr %8, align 4
  ret void

poison.cont:                                      ; preds = %hosted.state.merge
  %9 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 0, ptr %10, align 4
  call void @llvm.memset.p0.i64(ptr align 8 %aggregate.literal, i8 0, i64 24, i1 false)
  %11 = getelementptr i8, ptr %large, i64 0
  %12 = load i64, ptr %11, align 8
  store i64 %12, ptr %aggregate.literal, align 8
  %13 = getelementptr i8, ptr %aggregate.literal, i64 8
  %14 = getelementptr i8, ptr %large, i64 8
  %15 = load i64, ptr %14, align 8
  store i64 %15, ptr %13, align 8
  %16 = getelementptr i8, ptr %aggregate.literal, i64 16
  %17 = getelementptr i8, ptr %large, i64 16
  %18 = load i64, ptr %17, align 8
  store i64 %18, ptr %16, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %0, ptr align 8 %aggregate.literal, i64 24, i1 false)
  ret void
}

define hidden i32 @HostedExportLibrary_x5fx3a_x5fx3ahostedReferenceIOProbe_x3a_x3a_x5f_x5fhost_x5fbody(ptr noundef nonnull align 8 dereferenceable(16) %context, ptr noundef nonnull align 4 dereferenceable(4) %fallback, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"HostedExportLibrary_x5fx3a_x5fx3ahostedReferenceIOProbe_x3a_x3a_x5f_x5fhost_x5fbody$tmp$call_ref_tmp_30" = alloca { ptr, i64 }, align 8
  %0 = call ptr @uv_host_session_current_env()
  %1 = icmp ne ptr %0, null
  br i1 %1, label %hosted.state.session, label %hosted.state.fallback

hosted.state.session:                             ; preds = %entry
  %2 = getelementptr i8, ptr %0, i64 108
  br label %hosted.state.merge

hosted.state.fallback:                            ; preds = %entry
  br label %hosted.state.merge

hosted.state.merge:                               ; preds = %hosted.state.fallback, %hosted.state.session
  %3 = phi ptr [ %2, %hosted.state.session ], [ @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aHostedExportLibrary, %hosted.state.fallback ]
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
  %13 = getelementptr i8, ptr %context, i64 0
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fD8C5C4CC67AFA0D8, i64 16 }, ptr %"HostedExportLibrary_x5fx3a_x5fx3ahostedReferenceIOProbe_x3a_x3a_x5f_x5fhost_x5fbody$tmp$call_ref_tmp_30", align 8
  %14 = getelementptr i8, ptr %context, i64 0
  %15 = call i8 @ultraviolet_x3a_x3aruntime_x3a_x3aio_x3a_x3aexists(ptr noundef nonnull align 8 dereferenceable(16) %14, ptr noundef nonnull align 8 dereferenceable(16) %"HostedExportLibrary_x5fx3a_x5fx3ahostedReferenceIOProbe_x3a_x3a_x5f_x5fhost_x5fbody$tmp$call_ref_tmp_30")
  %16 = icmp ne i8 %15, 0
  %17 = icmp ne i8 %15, 0
  br i1 %17, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  br i1 true, label %check_ok, label %check_fail

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else
  %18 = load i32, ptr %fallback, align 4
  ret i32 %18

check_ok:                                         ; preds = %check_fail, %if.then
  %19 = load ptr, ptr %__panic, align 8
  %20 = load i8, ptr %19, align 1
  %21 = icmp ne i8 %20, 0
  br i1 %21, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %if.then
  %22 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %22, align 1
  %23 = getelementptr i8, ptr %22, i64 4
  store i32 4, ptr %23, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %24 = load ptr, ptr %__panic, align 8
  %25 = getelementptr i8, ptr %24, i64 4
  %26 = load i32, ptr %25, align 4
  ret i32 %26

panic.cont:                                       ; preds = %check_ok
  %27 = load i32, ptr %fallback, align 4
  %28 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %27, i32 1)
  %29 = extractvalue { i32, i1 } %28, 0
  %30 = extractvalue { i32, i1 } %28, 1
  %31 = freeze i32 %29
  br i1 %30, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  ret i32 %31

op_fail:                                          ; preds = %panic.cont
  %32 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %32, align 1
  %33 = getelementptr i8, ptr %32, i64 4
  store i32 4, ptr %33, align 4
  ret i32 0
}

define hidden i32 @HostedExportLibrary_x5fx3a_x5fx3ahostedReferenceStateIncrement_x3a_x3a_x5f_x5fhost_x5fbody(ptr noundef nonnull align 8 dereferenceable(16) %context, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = call ptr @uv_host_session_current_env()
  %1 = icmp ne ptr %0, null
  br i1 %1, label %hosted.state.session, label %hosted.state.fallback

hosted.state.session:                             ; preds = %entry
  %2 = getelementptr i8, ptr %0, i64 108
  br label %hosted.state.merge

hosted.state.fallback:                            ; preds = %entry
  br label %hosted.state.merge

hosted.state.merge:                               ; preds = %hosted.state.fallback, %hosted.state.session
  %3 = phi ptr [ %2, %hosted.state.session ], [ @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aHostedExportLibrary, %hosted.state.fallback ]
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
  %13 = call ptr @uv_host_session_current_env()
  %14 = icmp ne ptr %13, null
  br i1 %14, label %hosted.state.session1, label %hosted.state.fallback2

hosted.state.session1:                            ; preds = %poison.cont
  %15 = getelementptr i8, ptr %13, i64 108
  br label %hosted.state.merge3

hosted.state.fallback2:                           ; preds = %poison.cont
  br label %hosted.state.merge3

hosted.state.merge3:                              ; preds = %hosted.state.fallback2, %hosted.state.session1
  %16 = phi ptr [ %15, %hosted.state.session1 ], [ @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aHostedExportLibrary, %hosted.state.fallback2 ]
  %17 = load i8, ptr %16, align 1
  %18 = icmp ne i8 %17, 0
  br i1 %18, label %poison.take4, label %poison.cont5

poison.take4:                                     ; preds = %hosted.state.merge3
  %19 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %19, align 1
  %20 = getelementptr i8, ptr %19, i64 4
  store i32 10, ptr %20, align 4
  %21 = load ptr, ptr %__panic, align 8
  %22 = getelementptr i8, ptr %21, i64 4
  %23 = load i32, ptr %22, align 4
  ret i32 %23

poison.cont5:                                     ; preds = %hosted.state.merge3
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont5
  %24 = load ptr, ptr %__panic, align 8
  %25 = load i8, ptr %24, align 1
  %26 = icmp ne i8 %25, 0
  br i1 %26, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont5
  %27 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %27, align 1
  %28 = getelementptr i8, ptr %27, i64 4
  store i32 4, ptr %28, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %29 = load ptr, ptr %__panic, align 8
  %30 = getelementptr i8, ptr %29, i64 4
  %31 = load i32, ptr %30, align 4
  ret i32 %31

panic.cont:                                       ; preds = %check_ok
  %32 = call ptr @uv_host_session_current_env()
  %33 = icmp ne ptr %32, null
  br i1 %33, label %hosted.state.session6, label %hosted.state.fallback7

hosted.state.session6:                            ; preds = %panic.cont
  %34 = getelementptr i8, ptr %32, i64 104
  br label %hosted.state.merge8

hosted.state.fallback7:                           ; preds = %panic.cont
  br label %hosted.state.merge8

hosted.state.merge8:                              ; preds = %hosted.state.fallback7, %hosted.state.session6
  %35 = phi ptr [ %34, %hosted.state.session6 ], [ @HostedExportLibrary_x3a_x3a_x5fHOSTED_x5fSTATE_x5fREFERENCE, %hosted.state.fallback7 ]
  %36 = load i32, ptr %35, align 4
  %37 = load i32, ptr %value, align 4
  %38 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %36, i32 %37)
  %39 = extractvalue { i32, i1 } %38, 0
  %40 = extractvalue { i32, i1 } %38, 1
  %41 = freeze i32 %39
  br i1 %40, label %op_fail, label %op_ok

op_ok:                                            ; preds = %hosted.state.merge8
  %42 = call ptr @uv_host_session_current_env()
  %43 = icmp ne ptr %42, null
  br i1 %43, label %hosted.state.session9, label %hosted.state.fallback10

op_fail:                                          ; preds = %hosted.state.merge8
  %44 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %44, align 1
  %45 = getelementptr i8, ptr %44, i64 4
  store i32 4, ptr %45, align 4
  ret i32 0

hosted.state.session9:                            ; preds = %op_ok
  %46 = getelementptr i8, ptr %42, i64 108
  br label %hosted.state.merge11

hosted.state.fallback10:                          ; preds = %op_ok
  br label %hosted.state.merge11

hosted.state.merge11:                             ; preds = %hosted.state.fallback10, %hosted.state.session9
  %47 = phi ptr [ %46, %hosted.state.session9 ], [ @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aHostedExportLibrary, %hosted.state.fallback10 ]
  %48 = load i8, ptr %47, align 1
  %49 = icmp ne i8 %48, 0
  br i1 %49, label %poison.take12, label %poison.cont13

poison.take12:                                    ; preds = %hosted.state.merge11
  %50 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %50, align 1
  %51 = getelementptr i8, ptr %50, i64 4
  store i32 10, ptr %51, align 4
  %52 = load ptr, ptr %__panic, align 8
  %53 = getelementptr i8, ptr %52, i64 4
  %54 = load i32, ptr %53, align 4
  ret i32 %54

poison.cont13:                                    ; preds = %hosted.state.merge11
  %55 = load ptr, ptr %__panic, align 8
  %56 = load i8, ptr %55, align 1
  %57 = icmp ne i8 %56, 0
  br i1 %57, label %panic.take14, label %panic.cont15

panic.take14:                                     ; preds = %poison.cont13
  %58 = load ptr, ptr %__panic, align 8
  %59 = getelementptr i8, ptr %58, i64 4
  %60 = load i32, ptr %59, align 4
  ret i32 %60

panic.cont15:                                     ; preds = %poison.cont13
  %61 = call ptr @uv_host_session_current_env()
  %62 = icmp ne ptr %61, null
  br i1 %62, label %hosted.state.session16, label %hosted.state.fallback17

hosted.state.session16:                           ; preds = %panic.cont15
  %63 = getelementptr i8, ptr %61, i64 108
  br label %hosted.state.merge18

hosted.state.fallback17:                          ; preds = %panic.cont15
  br label %hosted.state.merge18

hosted.state.merge18:                             ; preds = %hosted.state.fallback17, %hosted.state.session16
  %64 = phi ptr [ %63, %hosted.state.session16 ], [ @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aHostedExportLibrary, %hosted.state.fallback17 ]
  %65 = load i8, ptr %64, align 1
  %66 = icmp ne i8 %65, 0
  br i1 %66, label %poison.take19, label %poison.cont20

poison.take19:                                    ; preds = %hosted.state.merge18
  %67 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %67, align 1
  %68 = getelementptr i8, ptr %67, i64 4
  store i32 10, ptr %68, align 4
  %69 = load ptr, ptr %__panic, align 8
  %70 = getelementptr i8, ptr %69, i64 4
  %71 = load i32, ptr %70, align 4
  ret i32 %71

poison.cont20:                                    ; preds = %hosted.state.merge18
  %72 = call ptr @uv_host_session_current_env()
  %73 = icmp ne ptr %72, null
  br i1 %73, label %hosted.state.session21, label %hosted.state.fallback22

hosted.state.session21:                           ; preds = %poison.cont20
  %74 = getelementptr i8, ptr %72, i64 104
  br label %hosted.state.merge23

hosted.state.fallback22:                          ; preds = %poison.cont20
  br label %hosted.state.merge23

hosted.state.merge23:                             ; preds = %hosted.state.fallback22, %hosted.state.session21
  %75 = phi ptr [ %74, %hosted.state.session21 ], [ @HostedExportLibrary_x3a_x3a_x5fHOSTED_x5fSTATE_x5fREFERENCE, %hosted.state.fallback22 ]
  store i32 %41, ptr %75, align 4
  %76 = call ptr @uv_host_session_current_env()
  %77 = icmp ne ptr %76, null
  br i1 %77, label %hosted.state.session24, label %hosted.state.fallback25

hosted.state.session24:                           ; preds = %hosted.state.merge23
  %78 = getelementptr i8, ptr %76, i64 108
  br label %hosted.state.merge26

hosted.state.fallback25:                          ; preds = %hosted.state.merge23
  br label %hosted.state.merge26

hosted.state.merge26:                             ; preds = %hosted.state.fallback25, %hosted.state.session24
  %79 = phi ptr [ %78, %hosted.state.session24 ], [ @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aHostedExportLibrary, %hosted.state.fallback25 ]
  %80 = load i8, ptr %79, align 1
  %81 = icmp ne i8 %80, 0
  br i1 %81, label %poison.take27, label %poison.cont28

poison.take27:                                    ; preds = %hosted.state.merge26
  %82 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %82, align 1
  %83 = getelementptr i8, ptr %82, i64 4
  store i32 10, ptr %83, align 4
  %84 = load ptr, ptr %__panic, align 8
  %85 = getelementptr i8, ptr %84, i64 4
  %86 = load i32, ptr %85, align 4
  ret i32 %86

poison.cont28:                                    ; preds = %hosted.state.merge26
  %87 = call ptr @uv_host_session_current_env()
  %88 = icmp ne ptr %87, null
  br i1 %88, label %hosted.state.session29, label %hosted.state.fallback30

hosted.state.session29:                           ; preds = %poison.cont28
  %89 = getelementptr i8, ptr %87, i64 104
  br label %hosted.state.merge31

hosted.state.fallback30:                          ; preds = %poison.cont28
  br label %hosted.state.merge31

hosted.state.merge31:                             ; preds = %hosted.state.fallback30, %hosted.state.session29
  %90 = phi ptr [ %89, %hosted.state.session29 ], [ @HostedExportLibrary_x3a_x3a_x5fHOSTED_x5fSTATE_x5fREFERENCE, %hosted.state.fallback30 ]
  %91 = load i32, ptr %90, align 4
  ret i32 %91
}

define hidden i32 @HostedExportLibrary_x5fx3a_x5fx3ahostedReferenceCatchValue_x3a_x3a_x5f_x5fhost_x5fbody(ptr noundef nonnull align 8 dereferenceable(16) %context, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = call ptr @uv_host_session_current_env()
  %1 = icmp ne ptr %0, null
  br i1 %1, label %hosted.state.session, label %hosted.state.fallback

hosted.state.session:                             ; preds = %entry
  %2 = getelementptr i8, ptr %0, i64 108
  br label %hosted.state.merge

hosted.state.fallback:                            ; preds = %entry
  br label %hosted.state.merge

hosted.state.merge:                               ; preds = %hosted.state.fallback, %hosted.state.session
  %3 = phi ptr [ %2, %hosted.state.session ], [ @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aHostedExportLibrary, %hosted.state.fallback ]
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
  ret i32 7
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aHostedExportLibrary(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  %2 = call ptr @uv_host_session_current_env()
  %3 = icmp ne ptr %2, null
  br i1 %3, label %hosted.state.session, label %hosted.state.fallback

hosted.state.session:                             ; preds = %entry
  %4 = getelementptr i8, ptr %2, i64 104
  br label %hosted.state.merge

hosted.state.fallback:                            ; preds = %entry
  br label %hosted.state.merge

hosted.state.merge:                               ; preds = %hosted.state.fallback, %hosted.state.session
  %5 = phi ptr [ %4, %hosted.state.session ], [ @HostedExportLibrary_x3a_x3a_x5fHOSTED_x5fSTATE_x5fREFERENCE, %hosted.state.fallback ]
  store i32 5, ptr %5, align 4
  %6 = load ptr, ptr %__panic, align 8
  %7 = load i8, ptr %6, align 1
  %8 = icmp ne i8 %7, 0
  br i1 %8, label %init.panic.take, label %init.panic.cont

init.panic.take:                                  ; preds = %hosted.state.merge
  %9 = call ptr @uv_host_session_current_env()
  %10 = icmp ne ptr %9, null
  br i1 %10, label %hosted.state.session1, label %hosted.state.fallback2

init.panic.cont:                                  ; preds = %hosted.state.merge
  ret void

hosted.state.session1:                            ; preds = %init.panic.take
  %11 = getelementptr i8, ptr %9, i64 108
  br label %hosted.state.merge3

hosted.state.fallback2:                           ; preds = %init.panic.take
  br label %hosted.state.merge3

hosted.state.merge3:                              ; preds = %hosted.state.fallback2, %hosted.state.session1
  %12 = phi ptr [ %11, %hosted.state.session1 ], [ @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aHostedExportLibrary, %hosted.state.fallback2 ]
  store i8 1, ptr %12, align 1
  %13 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %13, align 1
  %14 = getelementptr i8, ptr %13, i64 4
  store i32 10, ptr %14, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aHostedExportLibrary(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

declare ptr @uv_host_session_current_env()

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #1

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #2

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #3

define hidden void @__cx_lifecycle_init_HostedExportLibrary(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aHostedExportLibrary(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_HostedExportLibrary(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aHostedExportLibrary(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aHostedExportLibrary(ptr %dll_attach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aHostedExportLibrary(ptr %dll_detach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aHostedExportLibrary(ptr %host_create_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aHostedExportLibrary(ptr %host_destroy_panic_out)
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

define i32 @uv_hosted_reference_increment(i64 %__ultraviolet_session, i32 %value) {
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
  %10 = call i32 @HostedExportLibrary_x5fx3a_x5fx3ahostedReferenceIncrement_x3a_x3a_x5f_x5fhost_x5fbody(ptr %host_ctx, ptr %value2, ptr %host_arg)
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

define i32 @uv_hosted_reference_pair_sum(i64 %__ultraviolet_session, { i32, i32, [0 x i32] } %pair) {
entry:
  %host_panic_code4 = alloca i32, align 4
  %host_panic_code3 = alloca i32, align 4
  %host_arg = alloca ptr, align 8
  %host_ctx = alloca { { ptr, ptr }, [0 x i64] }, align 8
  %host_ctx_bundle = alloca { { ptr, ptr }, [0 x i64] }, align 8
  %host_panic_code = alloca i32, align 4
  %host_env = alloca ptr, align 8
  %pair2 = alloca { i32, i32, [0 x i32] }, align 8
  %__ultraviolet_session1 = alloca i64, align 8
  store i64 %__ultraviolet_session, ptr %__ultraviolet_session1, align 4
  store { i32, i32, [0 x i32] } %pair, ptr %pair2, align 4
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
  %10 = call i32 @HostedExportLibrary_x5fx3a_x5fx3ahostedReferencePairSum_x3a_x3a_x5f_x5fhost_x5fbody(ptr %host_ctx, ptr %pair2, ptr %host_arg)
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

define { i32, i32, [0 x i32] } @uv_hosted_reference_pair_echo(i64 %__ultraviolet_session, { i32, i32, [0 x i32] } %pair) {
entry:
  %host_panic_code4 = alloca i32, align 4
  %host_panic_code3 = alloca i32, align 4
  %host_arg = alloca ptr, align 8
  %host_ctx = alloca { { ptr, ptr }, [0 x i64] }, align 8
  %host_ctx_bundle = alloca { { ptr, ptr }, [0 x i64] }, align 8
  %host_panic_code = alloca i32, align 4
  %host_env = alloca ptr, align 8
  %pair2 = alloca { i32, i32, [0 x i32] }, align 8
  %__ultraviolet_session1 = alloca i64, align 8
  store i64 %__ultraviolet_session, ptr %__ultraviolet_session1, align 4
  store { i32, i32, [0 x i32] } %pair, ptr %pair2, align 4
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
  %10 = call { i32, i32, [0 x i32] } @HostedExportLibrary_x5fx3a_x5fx3ahostedReferencePairEcho_x3a_x3a_x5f_x5fhost_x5fbody(ptr %host_ctx, ptr %pair2, ptr %host_arg)
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
  ret { i32, i32, [0 x i32] } %10
}

define i32 @uv_hosted_reference_triple_sum(i64 %__ultraviolet_session, { i32, i32, i32, [0 x i32] } %triple) {
entry:
  %host_panic_code4 = alloca i32, align 4
  %host_panic_code3 = alloca i32, align 4
  %host_arg = alloca ptr, align 8
  %host_ctx = alloca { { ptr, ptr }, [0 x i64] }, align 8
  %host_ctx_bundle = alloca { { ptr, ptr }, [0 x i64] }, align 8
  %host_panic_code = alloca i32, align 4
  %host_env = alloca ptr, align 8
  %triple2 = alloca { i32, i32, i32, [0 x i32] }, align 8
  %__ultraviolet_session1 = alloca i64, align 8
  store i64 %__ultraviolet_session, ptr %__ultraviolet_session1, align 4
  store { i32, i32, i32, [0 x i32] } %triple, ptr %triple2, align 4
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
  %10 = call i32 @HostedExportLibrary_x5fx3a_x5fx3ahostedReferenceTripleSum_x3a_x3a_x5f_x5fhost_x5fbody(ptr %host_ctx, ptr %triple2, ptr %host_arg)
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

define { i32, i32, i32, [0 x i32] } @uv_hosted_reference_triple_echo(i64 %__ultraviolet_session, { i32, i32, i32, [0 x i32] } %triple) {
entry:
  %host_panic_code4 = alloca i32, align 4
  %host_panic_code3 = alloca i32, align 4
  %host_arg = alloca ptr, align 8
  %host_ctx = alloca { { ptr, ptr }, [0 x i64] }, align 8
  %host_ctx_bundle = alloca { { ptr, ptr }, [0 x i64] }, align 8
  %host_panic_code = alloca i32, align 4
  %host_env = alloca ptr, align 8
  %triple2 = alloca { i32, i32, i32, [0 x i32] }, align 8
  %__ultraviolet_session1 = alloca i64, align 8
  store i64 %__ultraviolet_session, ptr %__ultraviolet_session1, align 4
  store { i32, i32, i32, [0 x i32] } %triple, ptr %triple2, align 4
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
  %10 = call { i32, i32, i32, [0 x i32] } @HostedExportLibrary_x5fx3a_x5fx3ahostedReferenceTripleEcho_x3a_x3a_x5f_x5fhost_x5fbody(ptr %host_ctx, ptr %triple2, ptr %host_arg)
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
  ret { i32, i32, i32, [0 x i32] } %10
}

define void @uv_hosted_reference_large_echo(ptr %0, i64 %__ultraviolet_session, ptr %large) {
entry:
  %host_panic_code3 = alloca i32, align 4
  %host_panic_code2 = alloca i32, align 4
  %host_arg = alloca ptr, align 8
  %host_ret = alloca { i64, i64, i64, [0 x i64] }, align 8
  %host_ctx = alloca { { ptr, ptr }, [0 x i64] }, align 8
  %host_ctx_bundle = alloca { { ptr, ptr }, [0 x i64] }, align 8
  %host_panic_code = alloca i32, align 4
  %host_env = alloca ptr, align 8
  %__ultraviolet_session1 = alloca i64, align 8
  store i64 %__ultraviolet_session, ptr %__ultraviolet_session1, align 4
  %1 = load { i64, i64, i64, [0 x i64] }, ptr %large, align 4
  store ptr null, ptr %host_env, align 8
  %2 = call i32 @uv_host_session_try_enter(i64 %__ultraviolet_session, ptr @__uv_host_session_owner_token, ptr %host_env)
  %3 = icmp ne i32 %2, 0
  br i1 %3, label %host.enter.ok, label %host.enter.reject

host.enter.ok:                                    ; preds = %entry
  %4 = load ptr, ptr %host_env, align 8
  %5 = load { { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, { ptr, ptr }, [0 x i64] }, ptr %4, align 8
  %6 = getelementptr i8, ptr %4, i64 96
  store i8 0, ptr %6, align 1
  %7 = getelementptr i8, ptr %6, i64 4
  store i32 0, ptr %7, align 4
  store { { ptr, ptr }, [0 x i64] } zeroinitializer, ptr %host_ctx_bundle, align 8
  %8 = getelementptr i8, ptr %4, i64 48
  %9 = load { ptr, ptr }, ptr %8, align 8
  %10 = getelementptr i8, ptr %host_ctx_bundle, i64 0
  store { ptr, ptr } %9, ptr %10, align 8
  %11 = load { { ptr, ptr }, [0 x i64] }, ptr %host_ctx_bundle, align 8
  store { { ptr, ptr }, [0 x i64] } %11, ptr %host_ctx, align 8
  store ptr %6, ptr %host_arg, align 8
  call void @HostedExportLibrary_x5fx3a_x5fx3ahostedReferenceLargeEcho_x3a_x3a_x5f_x5fhost_x5fbody(ptr %host_ret, ptr %host_ctx, ptr %large, ptr %host_arg)
  %12 = load { i64, i64, i64, [0 x i64] }, ptr %host_ret, align 4
  %13 = load i8, ptr %6, align 1
  %14 = icmp ne i8 %13, 0
  %15 = getelementptr i8, ptr %6, i64 4
  %16 = load i32, ptr %15, align 4
  %17 = call i32 @uv_host_session_leave(i64 %__ultraviolet_session, ptr @__uv_host_session_owner_token)
  %18 = icmp ne i32 %17, 0
  br i1 %18, label %host.call.leave.ok, label %host.call.leave.fail

host.enter.reject:                                ; preds = %entry
  store i32 14, ptr %host_panic_code, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %host_panic_code)
  unreachable

host.call.leave.ok:                               ; preds = %host.enter.ok
  br i1 %14, label %host.call.panic, label %host.call.ok

host.call.leave.fail:                             ; preds = %host.enter.ok
  store i32 255, ptr %host_panic_code2, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %host_panic_code2)
  unreachable

host.call.panic:                                  ; preds = %host.call.leave.ok
  store i32 %16, ptr %host_panic_code3, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %host_panic_code3)
  unreachable

host.call.ok:                                     ; preds = %host.call.leave.ok
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %0, ptr align 1 %host_ret, i64 24, i1 false)
  ret void
}

define i32 @uv_hosted_reference_io_probe(i64 %__ultraviolet_session, i32 %fallback) {
entry:
  %host_panic_code4 = alloca i32, align 4
  %host_panic_code3 = alloca i32, align 4
  %host_arg = alloca ptr, align 8
  %host_ctx = alloca { { ptr, ptr }, [0 x i64] }, align 8
  %host_ctx_bundle = alloca { { ptr, ptr }, [0 x i64] }, align 8
  %host_panic_code = alloca i32, align 4
  %host_env = alloca ptr, align 8
  %fallback2 = alloca i32, align 4
  %__ultraviolet_session1 = alloca i64, align 8
  store i64 %__ultraviolet_session, ptr %__ultraviolet_session1, align 4
  store i32 %fallback, ptr %fallback2, align 4
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
  %6 = getelementptr i8, ptr %2, i64 0
  %7 = load { ptr, ptr }, ptr %6, align 8
  %8 = getelementptr i8, ptr %host_ctx_bundle, i64 0
  store { ptr, ptr } %7, ptr %8, align 8
  %9 = load { { ptr, ptr }, [0 x i64] }, ptr %host_ctx_bundle, align 8
  store { { ptr, ptr }, [0 x i64] } %9, ptr %host_ctx, align 8
  store ptr %4, ptr %host_arg, align 8
  %10 = call i32 @HostedExportLibrary_x5fx3a_x5fx3ahostedReferenceIOProbe_x3a_x3a_x5f_x5fhost_x5fbody(ptr %host_ctx, ptr %fallback2, ptr %host_arg)
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

define i32 @uv_hosted_reference_state_increment(i64 %__ultraviolet_session, i32 %value) {
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
  %10 = call i32 @HostedExportLibrary_x5fx3a_x5fx3ahostedReferenceStateIncrement_x3a_x3a_x5f_x5fhost_x5fbody(ptr %host_ctx, ptr %value2, ptr %host_arg)
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

define i32 @uv_hosted_reference_catch_value(i64 %__ultraviolet_session) {
entry:
  %host_arg = alloca ptr, align 8
  %host_ctx = alloca { { ptr, ptr }, [0 x i64] }, align 8
  %host_ctx_bundle = alloca { { ptr, ptr }, [0 x i64] }, align 8
  %host_env = alloca ptr, align 8
  %__ultraviolet_session1 = alloca i64, align 8
  store i64 %__ultraviolet_session, ptr %__ultraviolet_session1, align 4
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
  %10 = call i32 @HostedExportLibrary_x5fx3a_x5fx3ahostedReferenceCatchValue_x3a_x3a_x5f_x5fhost_x5fbody(ptr %host_ctx, ptr %host_arg)
  %11 = load i8, ptr %4, align 1
  %12 = icmp ne i8 %11, 0
  %13 = getelementptr i8, ptr %4, i64 4
  %14 = load i32, ptr %13, align 4
  %15 = call i32 @uv_host_session_leave(i64 %__ultraviolet_session, ptr @__uv_host_session_owner_token)
  %16 = icmp ne i32 %15, 0
  br i1 %16, label %host.call.leave.ok, label %host.call.leave.fail

host.enter.reject:                                ; preds = %entry
  ret i32 0

host.call.leave.ok:                               ; preds = %host.enter.ok
  br i1 %12, label %host.call.panic, label %host.call.ok

host.call.leave.fail:                             ; preds = %host.enter.ok
  ret i32 0

host.call.panic:                                  ; preds = %host.call.leave.ok
  ret i32 0

host.call.ok:                                     ; preds = %host.call.leave.ok
  ret i32 %10
}

attributes #0 = { nounwind }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #3 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
