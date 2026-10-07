; ==== Source/ExpressionSemantics.ll
; ModuleID = 'ExpressionSemantics'
source_filename = "ExpressionSemantics"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics = hidden global i8 0
@ExpressionSemantics_x3a_x3aLOWER_x5fEXPR_x5fPATH_x5fSTATIC = hidden constant i32 43, align 4
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAD2ACA7747985764 = internal constant [4 x i8] c"\01\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fCD1026A0F6A50639 = internal constant [4 x i8] c"\0C\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3afloat_x5fAAB1693229BA1DB8 = internal constant [8 x i8] c"\00\00\00\00\00\00\F0?", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2CEADADB521D2E30 = internal constant [4 x i8] c"\15\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fEBCB249D8172F4C6 = internal constant [4 x i8] c"C\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fED202287F403D086 = internal constant [4 x i8] c"\03\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4BD078952B3D3835 = internal constant [4 x i8] c"@\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f6D3572669B2CDE42 = internal constant [4 x i8] c"\07\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f213FBE7DD29995B1 = internal constant [4 x i8] c"\00^\D0\B2", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fC1454F5921AB46AD = internal constant [8 x i8] c"(\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAD002AB9F9463BEC = internal constant [4 x i8] c"\09\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2DEA994B2809D300 = internal constant [4 x i8] c"%\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fADAAA9AF328EA9CC = internal constant [4 x i8] c")\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f8D1ACE904A398D17 = internal constant [4 x i8] c"\02\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fCD3AC65E44F721B1 = internal constant [4 x i8] c"\04\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2D401A55EEC16520 = internal constant [4 x i8] c"\05\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f7B25F98F63470373 = internal constant [8 x i8] c"6\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fB0F1BFE9D09FE8BD = internal constant [8 x i8] c"8\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f91F6F8E0C5B09E9C = internal constant [8 x i8] c"9\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fACD58AFCAAF42074 = internal constant [4 x i8] c"\11\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f0A896559C31EE3A2 = internal constant [8 x i8] c"G\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fEDA001BFDEFA22EE = internal constant [4 x i8] c"+\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f6DB5519E862330AA = internal constant [4 x i8] c"/\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2C959B60B578F740 = internal constant [4 x i8] c"e\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f0C859F79B81A2CF3 = internal constant [4 x i8] c"f\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f6C8AF37161E47062 = internal constant [4 x i8] c"g\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4C5057CD16338A9D = internal constant [4 x i8] c"h\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAC55ABC4BFFDCE0C = internal constant [4 x i8] c"i\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f8C45AFDDC29F03BF = internal constant [4 x i8] c"j\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fEC4B03D56C69472E = internal constant [4 x i8] c"k\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fCC65A7ABBD5C9859 = internal constant [4 x i8] c"l\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2C6AFBA36726DBC8 = internal constant [4 x i8] c"m\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f0C5AFFBC69C8117B = internal constant [4 x i8] c"n\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f89CD31291D2AEFA4 = internal constant [8 x i8] c"\01\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f6C6053B4139254EA = internal constant [4 x i8] c"o\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f8CF02ED2FBE7719F = internal constant [4 x i8] c"\0A\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4C25B80FC7E16F25 = internal constant [4 x i8] c"p\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fD98B1E90FD653435 = internal constant [4 x i8] c"cell", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAF63BD4C8601B7DF = internal constant [1 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAC2B0C0771ABB294 = internal constant [4 x i8] c"q\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE = internal constant [12 x i8] c"shared_value", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fABAB2CCF86B5602C = internal constant [4 x i8] c"I\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fC7C2BF3B330983E6 = internal constant [8 x i8] c"\03\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fE6BD86443DF8CE07 = internal constant [8 x i8] c"\02\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAF63BC4C8601B62C = internal constant [1 x i8] c"\01", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAF63BF4C8601BB45 = internal constant [1 x i8] c"\02", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f0D301E6EF1629AD3 = internal constant [4 x i8] c"\06\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3afloat_x5fA8C7F832281A39C5 = internal constant [8 x i8] zeroinitializer, align 1
@__uv_library_attached = internal global i1 false
@__uv_image_panic_record = common hidden global { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, align 4
@llvm.global_ctors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_ctor, ptr null }]
@llvm.global_dtors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_dtor, ptr null }]

; Function Attrs: nounwind
declare ptr @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aalloc_x5fframe(i64, i64) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aresume(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8] }) align 1 dereferenceable(24), ptr, ptr, ptr) #0

; Function Attrs: noreturn nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4)) #1

; Function Attrs: nounwind
declare i8 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5fis_x5factive(ptr) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5ftag_x5fscope(ptr, i64) #0

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16)) #0

; Function Attrs: nounwind
declare i64 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3amark(ptr noundef nonnull align 8 dereferenceable(16)) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3areset_x5fto(ptr noundef nonnull align 8 dereferenceable(16), ptr noundef nonnull align 8 dereferenceable(8)) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fenter(i64) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64) #0

; Function Attrs: nounwind
declare void @uv_key_acquire(ptr, { i64, i64 }, i8) #0

; Function Attrs: nounwind
declare void @uv_key_check_conflict({ i64, i64 }, i8) #0

; Function Attrs: nounwind
declare void @uv_key_reacquire(ptr) #0

; Function Attrs: nounwind
declare ptr @uv_key_release_all() #0

; Function Attrs: nounwind
declare ptr @uv_key_scope_enter() #0

; Function Attrs: nounwind
declare void @uv_key_scope_exit(ptr) #0

define hidden { i8, [3 x i8], [4 x i8], [0 x i32] } @ExpressionSemantics_x3a_x3aeffectfulLoweringOutcome(ptr noundef nonnull align 1 dereferenceable(1) %should_fail, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %1 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %2 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %3 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %4 = icmp ne i8 %3, 0
  br i1 %4, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %5 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 10, ptr %6, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

poison.cont:                                      ; preds = %entry
  %7 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %7, align 1
  %8 = getelementptr i8, ptr %7, i64 4
  store i32 0, ptr %8, align 4
  %9 = load i8, ptr %should_fail, align 1
  %10 = icmp ne i8 %9, 0
  %11 = load i8, ptr %should_fail, align 1
  %12 = icmp ne i8 %11, 0
  br i1 %12, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %2, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %1, align 4
  %13 = getelementptr i8, ptr %1, i64 4
  store i8 1, ptr %13, align 1
  %14 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %1, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } %14

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else
  %15 = load i32, ptr %value, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } { i8 1, [3 x i8] zeroinitializer, [4 x i8] zeroinitializer, [0 x i32] zeroinitializer }, ptr %0, align 4
  %16 = getelementptr i8, ptr %0, i64 4
  store i32 %15, ptr %16, align 1
  %17 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %0, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } %17
}

define hidden i32 @ExpressionSemantics_x3a_x3aenumLoweringPayloadValue(ptr noundef nonnull align 4 dereferenceable(8) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"ExpressionSemantics_x3a_x3aenumLoweringPayloadValue$tmp$if_case_clause_result_10" = alloca i32, align 4
  %payload_value4 = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3aenumLoweringPayloadValue$tmp$if_case_clause_result_8" = alloca i32, align 4
  %payload_value = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3aenumLoweringPayloadValue$tmp$if_case_clause_result_6" = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %9 = load i8, ptr %value, align 1
  %10 = icmp eq i8 %9, 0
  br i1 %10, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %ifcase.case3, %ifcase.case1, %ifcase.case
  %ifcase.result = phi i32 [ %11, %ifcase.case ], [ %23, %ifcase.case1 ], [ %35, %ifcase.case3 ]
  ret i32 %ifcase.result

ifcase.case:                                      ; preds = %poison.cont
  store i32 1, ptr %"ExpressionSemantics_x3a_x3aenumLoweringPayloadValue$tmp$if_case_clause_result_6", align 4
  %11 = load i32, ptr %"ExpressionSemantics_x3a_x3aenumLoweringPayloadValue$tmp$if_case_clause_result_6", align 4
  br label %ifcase.merge

ifcase.next:                                      ; preds = %poison.cont
  %12 = load i8, ptr %value, align 1
  %13 = icmp eq i8 %12, 1
  %14 = getelementptr i8, ptr %value, i64 4
  %15 = getelementptr i8, ptr %14, i64 0
  %16 = load i32, ptr %15, align 1
  %17 = and i1 %13, true
  br i1 %17, label %ifcase.case1, label %ifcase.next2

ifcase.case1:                                     ; preds = %ifcase.next
  %18 = getelementptr i8, ptr %value, i64 4
  %19 = getelementptr i8, ptr %18, i64 0
  %20 = load i32, ptr %19, align 1
  store i32 %20, ptr %payload_value, align 4
  %21 = load i32, ptr %payload_value, align 4
  %22 = load i32, ptr %payload_value, align 1
  store i32 %22, ptr %"ExpressionSemantics_x3a_x3aenumLoweringPayloadValue$tmp$if_case_clause_result_8", align 4
  %23 = load i32, ptr %"ExpressionSemantics_x3a_x3aenumLoweringPayloadValue$tmp$if_case_clause_result_8", align 4
  br label %ifcase.merge

ifcase.next2:                                     ; preds = %ifcase.next
  %24 = load i8, ptr %value, align 1
  %25 = icmp eq i8 %24, 2
  %26 = getelementptr i8, ptr %value, i64 4
  %27 = getelementptr i8, ptr %26, i64 0
  %28 = load i32, ptr %27, align 1
  %29 = and i1 %25, true
  br i1 %29, label %ifcase.case3, label %ifcase.unmatched

ifcase.case3:                                     ; preds = %ifcase.next2
  %30 = getelementptr i8, ptr %value, i64 4
  %31 = getelementptr i8, ptr %30, i64 0
  %32 = load i32, ptr %31, align 1
  store i32 %32, ptr %payload_value4, align 4
  %33 = load i32, ptr %payload_value4, align 4
  %34 = load i32, ptr %payload_value4, align 1
  store i32 %34, ptr %"ExpressionSemantics_x3a_x3aenumLoweringPayloadValue$tmp$if_case_clause_result_10", align 4
  %35 = load i32, ptr %"ExpressionSemantics_x3a_x3aenumLoweringPayloadValue$tmp$if_case_clause_result_10", align 4
  br label %ifcase.merge

ifcase.unmatched:                                 ; preds = %ifcase.next2
  %36 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %36, align 1
  %37 = getelementptr i8, ptr %36, i64 4
  store i32 18, ptr %37, align 4
  %38 = load ptr, ptr %__panic, align 8
  %39 = getelementptr i8, ptr %38, i64 4
  %40 = load i32, ptr %39, align 4
  ret i32 %40
}

define hidden i32 @ExpressionSemantics_x3a_x3astatementLoweringEvidenceSink(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %9 = load i32, ptr %value, align 4
  ret i32 %9
}

define i8 @ExpressionSemantics_x3a_x3acheckedIntLiteralReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %value = alloca i8, align 1
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  store i8 12, ptr %value, align 1
  %10 = load i8, ptr %value, align 1
  ret i8 %10
}

define double @ExpressionSemantics_x3a_x3acheckedFloatLiteralExplicitReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %value = alloca double, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret double 0.000000e+00

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 0, ptr %5, align 4
  store double 1.000000e+00, ptr %value, align 8
  %6 = load double, ptr %value, align 8
  ret double %6
}

define i32 @ExpressionSemantics_x3a_x3areferenceArgumentValue(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %9 = load i32, ptr %value, align 4
  ret i32 %9
}

define i32 @ExpressionSemantics_x3a_x3aargsTConsRefReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %value = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  store i32 21, ptr %value, align 4
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %23 = call i32 @ExpressionSemantics_x3a_x3areferenceArgumentValue(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
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
  ret i32 %23
}

define i32 @ExpressionSemantics_x3a_x3alowerCallNoArgsReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  ret i32 67
}

define i32 @ExpressionSemantics_x3a_x3alowerArgsEmptyReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %23 = call i32 @ExpressionSemantics_x3a_x3alowerCallNoArgsReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic)
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
  ret i32 %23
}

define i32 @ExpressionSemantics_x3a_x3alowerExprCallClosureReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"ExpressionSemantics_x3a_x3alowerExprCallClosureReference$tmp$closure_call_value_571" = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3alowerExprCallClosureReference$tmp$closure_call_value_57" = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3alowerExprCallClosureReference$tmp$call_ref_tmp_49" = alloca i32, align 4
  %closure = alloca { ptr, ptr }, align 8
  %"ExpressionSemantics_x3a_x3alowerExprCallClosureReference$tmp$closure_env_storage_30" = alloca { ptr }, align 8
  %bias = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  store i32 3, ptr %bias, align 4
  store { ptr } zeroinitializer, ptr %"ExpressionSemantics_x3a_x3alowerExprCallClosureReference$tmp$closure_env_storage_30", align 8
  %9 = getelementptr i8, ptr %"ExpressionSemantics_x3a_x3alowerExprCallClosureReference$tmp$closure_env_storage_30", i64 0
  store ptr %bias, ptr %9, align 8
  %10 = insertvalue { ptr, ptr } zeroinitializer, ptr %"ExpressionSemantics_x3a_x3alowerExprCallClosureReference$tmp$closure_env_storage_30", 0
  %11 = insertvalue { ptr, ptr } %10, ptr @ExpressionSemantics_x5fx3a_x5fx3alowerExprCallClosureReference_x3a_x3a_x5fclosure0, 1
  store { ptr, ptr } %11, ptr %closure, align 8
  store i32 64, ptr %"ExpressionSemantics_x3a_x3alowerExprCallClosureReference$tmp$call_ref_tmp_49", align 4
  %12 = load { ptr, ptr }, ptr %closure, align 8
  %13 = extractvalue { ptr, ptr } %12, 1
  %14 = load { ptr, ptr }, ptr %closure, align 8
  %15 = extractvalue { ptr, ptr } %14, 0
  %16 = call i32 @ExpressionSemantics_x5fx3a_x5fx3alowerExprCallClosureReference_x3a_x3a_x5fclosure0(ptr %15, ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3alowerExprCallClosureReference$tmp$call_ref_tmp_49", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %17 = load ptr, ptr %__panic, align 8
  %18 = load i8, ptr %17, align 1
  %19 = icmp ne i8 %18, 0
  br i1 %19, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont
  %20 = load ptr, ptr %__panic, align 8
  %21 = getelementptr i8, ptr %20, i64 0
  %22 = load i8, ptr %21, align 1
  %23 = load ptr, ptr %__panic, align 8
  %24 = getelementptr i8, ptr %23, i64 4
  %25 = load i32, ptr %24, align 4
  %26 = load ptr, ptr %__panic, align 8
  %27 = getelementptr i8, ptr %26, i64 4
  %28 = load i32, ptr %27, align 4
  ret i32 %28

panic.cont:                                       ; preds = %poison.cont
  store i32 %16, ptr %"ExpressionSemantics_x3a_x3alowerExprCallClosureReference$tmp$closure_call_value_571", align 4
  %29 = load i32, ptr %"ExpressionSemantics_x3a_x3alowerExprCallClosureReference$tmp$closure_call_value_571", align 4
  ret i32 %29
}

define internal i32 @ExpressionSemantics_x5fx3a_x5fx3alowerExprCallClosureReference_x3a_x3a_x5fclosure0(ptr %__env, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %9 = getelementptr i8, ptr %__env, i64 0
  %10 = load ptr, ptr %9, align 8
  %11 = load i32, ptr %10, align 4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %12 = load ptr, ptr %__panic, align 8
  %13 = load i8, ptr %12, align 1
  %14 = icmp ne i8 %13, 0
  br i1 %14, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %15 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %15, align 1
  %16 = getelementptr i8, ptr %15, i64 4
  store i32 4, ptr %16, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %17 = load ptr, ptr %__panic, align 8
  %18 = getelementptr i8, ptr %17, i64 0
  %19 = load i8, ptr %18, align 1
  %20 = load ptr, ptr %__panic, align 8
  %21 = getelementptr i8, ptr %20, i64 4
  %22 = load i32, ptr %21, align 4
  %23 = load ptr, ptr %__panic, align 8
  %24 = getelementptr i8, ptr %23, i64 4
  %25 = load i32, ptr %24, align 4
  ret i32 %25

panic.cont:                                       ; preds = %check_ok
  %26 = load i32, ptr %value, align 4
  %27 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %26, i32 %11)
  %28 = extractvalue { i32, i1 } %27, 0
  %29 = extractvalue { i32, i1 } %27, 1
  %30 = freeze i32 %28
  br i1 %29, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  ret i32 %30

op_fail:                                          ; preds = %panic.cont
  %31 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %31, align 1
  %32 = getelementptr i8, ptr %31, i64 4
  store i32 4, ptr %32, align 4
  ret i32 0
}

define double @ExpressionSemantics_x3a_x3alowerExprCastReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %value = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret double 0.000000e+00

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 0, ptr %5, align 4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %6 = load ptr, ptr %__panic, align 8
  %7 = load i8, ptr %6, align 1
  %8 = icmp ne i8 %7, 0
  br i1 %8, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %9 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 4, ptr %10, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  ret double 0.000000e+00

panic.cont:                                       ; preds = %check_ok
  store i32 -7, ptr %value, align 4
  %11 = load i32, ptr %value, align 4
  %12 = load ptr, ptr %__panic, align 8
  %13 = load i8, ptr %12, align 1
  %14 = icmp ne i8 %13, 0
  br i1 %14, label %panic.take1, label %panic.cont2

panic.take1:                                      ; preds = %panic.cont
  %15 = load ptr, ptr %__panic, align 8
  %16 = getelementptr i8, ptr %15, i64 0
  %17 = load i8, ptr %16, align 1
  %18 = load ptr, ptr %__panic, align 8
  %19 = getelementptr i8, ptr %18, i64 4
  %20 = load i32, ptr %19, align 4
  ret double 0.000000e+00

panic.cont2:                                      ; preds = %panic.cont
  %21 = load i32, ptr %value, align 4
  %22 = sitofp i32 %21 to double
  ret double %22
}

define double @ExpressionSemantics_x3a_x3aintToFloatUnsignedLoweringReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %value = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret double 0.000000e+00

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 0, ptr %5, align 4
  store i32 -1294967296, ptr %value, align 4
  %6 = load i32, ptr %value, align 4
  %7 = load ptr, ptr %__panic, align 8
  %8 = load i8, ptr %7, align 1
  %9 = icmp ne i8 %8, 0
  br i1 %9, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont
  ret double 0.000000e+00

panic.cont:                                       ; preds = %poison.cont
  %10 = load i32, ptr %value, align 4
  %11 = uitofp i32 %10 to double
  ret double %11
}

define i32 @ExpressionSemantics_x3a_x3alowerExprTransmuteReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  ret i32 -1
}

define i32 @ExpressionSemantics_x3a_x3alowerExprPathReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %16 = load i32, ptr @ExpressionSemantics_x3a_x3aLOWER_x5fEXPR_x5fPATH_x5fSTATIC, align 4
  ret i32 %16
}

define i32 @ExpressionSemantics_x3a_x3aaddressOfReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %dereferenced = alloca i32, align 4
  %pointer = alloca ptr, align 8
  %value = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fenter(i64 40)
  store i32 9, ptr %value, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5ftag_x5fscope(ptr %value, i64 40)
  store ptr %value, ptr %pointer, align 8
  %9 = load ptr, ptr %pointer, align 8
  %10 = icmp ne ptr %9, null
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
  store i32 8, ptr %15, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %16 = load ptr, ptr %__panic, align 8
  %17 = getelementptr i8, ptr %16, i64 0
  %18 = load i8, ptr %17, align 1
  %19 = load ptr, ptr %__panic, align 8
  %20 = getelementptr i8, ptr %19, i64 4
  %21 = load i32, ptr %20, align 4
  %22 = load ptr, ptr %__panic, align 8
  %23 = getelementptr i8, ptr %22, i64 4
  %24 = load i32, ptr %23, align 4
  ret i32 %24

panic.cont:                                       ; preds = %check_ok
  %25 = load ptr, ptr %pointer, align 8
  %26 = call i8 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5fis_x5factive(ptr %25)
  %27 = icmp ne i8 %26, 0
  br i1 %27, label %check_ok1, label %check_fail2

check_ok1:                                        ; preds = %check_fail2, %panic.cont
  %28 = load ptr, ptr %__panic, align 8
  %29 = load i8, ptr %28, align 1
  %30 = icmp ne i8 %29, 0
  br i1 %30, label %panic.take3, label %panic.cont4

check_fail2:                                      ; preds = %panic.cont
  %31 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %31, align 1
  %32 = getelementptr i8, ptr %31, i64 4
  store i32 9, ptr %32, align 4
  br label %check_ok1

panic.take3:                                      ; preds = %check_ok1
  %33 = load ptr, ptr %__panic, align 8
  %34 = getelementptr i8, ptr %33, i64 0
  %35 = load i8, ptr %34, align 1
  %36 = load ptr, ptr %__panic, align 8
  %37 = getelementptr i8, ptr %36, i64 4
  %38 = load i32, ptr %37, align 4
  %39 = load ptr, ptr %__panic, align 8
  %40 = getelementptr i8, ptr %39, i64 4
  %41 = load i32, ptr %40, align 4
  ret i32 %41

panic.cont4:                                      ; preds = %check_ok1
  %42 = load ptr, ptr %pointer, align 8
  %43 = load i32, ptr %42, align 4
  store i32 %43, ptr %dereferenced, align 4
  %44 = load i32, ptr %dereferenced, align 4
  ret i32 %44
}

define i32 @ExpressionSemantics_x3a_x3alocalUsingAliasReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %source_value = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  store i32 37, ptr %source_value, align 4
  store i32 41, ptr %source_value, align 4
  %9 = load i32, ptr %source_value, align 4
  ret i32 %9
}

define i32 @ExpressionSemantics_x3a_x3astatementLoweringEvidenceReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %abi_return151 = alloca { i64, i64 }, align 8
  %abi_return135 = alloca { i64, i64 }, align 8
  %abi_return119 = alloca { i64, i64 }, align 8
  %abi_return103 = alloca { i64, i64 }, align 8
  %byref_arg91 = alloca i64, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %byref_arg86 = alloca { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] }, align 8
  %__bind_26_scratch = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %0 = alloca { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] }, align 8
  %byref_arg = alloca i32, align 4
  %loop_index = alloca i32, align 4
  %valued_break = alloca i32, align 4
  %loop.result.slot = alloca i32, align 4
  %var_value = alloca i32, align 4
  %let_value = alloca i32, align 4
  %total = alloca i32, align 4
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  store i32 0, ptr %total, align 4
  store i32 1, ptr %let_value, align 4
  store i32 2, ptr %var_value, align 4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %10 = load ptr, ptr %__panic, align 8
  %11 = load i8, ptr %10, align 1
  %12 = icmp ne i8 %11, 0
  br i1 %12, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %13 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %13, align 1
  %14 = getelementptr i8, ptr %13, i64 4
  store i32 4, ptr %14, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %15 = load ptr, ptr %__panic, align 8
  %16 = getelementptr i8, ptr %15, i64 4
  %17 = load i32, ptr %16, align 4
  ret i32 %17

panic.cont:                                       ; preds = %check_ok
  %18 = load i32, ptr %var_value, align 4
  %19 = load i32, ptr %let_value, align 4
  %20 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %18, i32 %19)
  %21 = extractvalue { i32, i1 } %20, 0
  %22 = extractvalue { i32, i1 } %20, 1
  %23 = freeze i32 %21
  br i1 %22, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  store i32 %23, ptr %var_value, align 4
  br i1 true, label %check_ok1, label %check_fail2

op_fail:                                          ; preds = %panic.cont
  %24 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %24, align 1
  %25 = getelementptr i8, ptr %24, i64 4
  store i32 4, ptr %25, align 4
  ret i32 0

check_ok1:                                        ; preds = %check_fail2, %op_ok
  %26 = load ptr, ptr %__panic, align 8
  %27 = load i8, ptr %26, align 1
  %28 = icmp ne i8 %27, 0
  br i1 %28, label %panic.take3, label %panic.cont4

check_fail2:                                      ; preds = %op_ok
  %29 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %29, align 1
  %30 = getelementptr i8, ptr %29, i64 4
  store i32 4, ptr %30, align 4
  br label %check_ok1

panic.take3:                                      ; preds = %check_ok1
  %31 = load ptr, ptr %__panic, align 8
  %32 = getelementptr i8, ptr %31, i64 4
  %33 = load i32, ptr %32, align 4
  ret i32 %33

panic.cont4:                                      ; preds = %check_ok1
  %34 = load i32, ptr %total, align 4
  %35 = load i32, ptr %var_value, align 4
  %36 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %34, i32 %35)
  %37 = extractvalue { i32, i1 } %36, 0
  %38 = extractvalue { i32, i1 } %36, 1
  %39 = freeze i32 %37
  br i1 %38, label %op_fail6, label %op_ok5

op_ok5:                                           ; preds = %panic.cont4
  store i32 %39, ptr %total, align 4
  br i1 true, label %check_ok7, label %check_fail8

op_fail6:                                         ; preds = %panic.cont4
  %40 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %40, align 1
  %41 = getelementptr i8, ptr %40, i64 4
  store i32 4, ptr %41, align 4
  ret i32 0

check_ok7:                                        ; preds = %check_fail8, %op_ok5
  %42 = load ptr, ptr %__panic, align 8
  %43 = load i8, ptr %42, align 1
  %44 = icmp ne i8 %43, 0
  br i1 %44, label %panic.take9, label %panic.cont10

check_fail8:                                      ; preds = %op_ok5
  %45 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %45, align 1
  %46 = getelementptr i8, ptr %45, i64 4
  store i32 4, ptr %46, align 4
  br label %check_ok7

panic.take9:                                      ; preds = %check_ok7
  %47 = load ptr, ptr %__panic, align 8
  %48 = getelementptr i8, ptr %47, i64 4
  %49 = load i32, ptr %48, align 4
  ret i32 %49

panic.cont10:                                     ; preds = %check_ok7
  %50 = load i32, ptr %total, align 4
  %51 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %50, i32 1)
  %52 = extractvalue { i32, i1 } %51, 0
  %53 = extractvalue { i32, i1 } %51, 1
  %54 = freeze i32 %52
  br i1 %53, label %op_fail12, label %op_ok11

op_ok11:                                          ; preds = %panic.cont10
  store i32 %54, ptr %total, align 4
  %55 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %56 = icmp ne i8 %55, 0
  br i1 %56, label %poison.take13, label %poison.cont14

op_fail12:                                        ; preds = %panic.cont10
  %57 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %57, align 1
  %58 = getelementptr i8, ptr %57, i64 4
  store i32 4, ptr %58, align 4
  ret i32 0

poison.take13:                                    ; preds = %op_ok11
  %59 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %59, align 1
  %60 = getelementptr i8, ptr %59, i64 4
  store i32 10, ptr %60, align 4
  %61 = load ptr, ptr %__panic, align 8
  %62 = getelementptr i8, ptr %61, i64 4
  %63 = load i32, ptr %62, align 4
  ret i32 %63

poison.cont14:                                    ; preds = %op_ok11
  %64 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %65 = icmp ne i8 %64, 0
  br i1 %65, label %poison.take15, label %poison.cont16

poison.take15:                                    ; preds = %poison.cont14
  %66 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %66, align 1
  %67 = getelementptr i8, ptr %66, i64 4
  store i32 10, ptr %67, align 4
  %68 = load ptr, ptr %__panic, align 8
  %69 = getelementptr i8, ptr %68, i64 4
  %70 = load i32, ptr %69, align 4
  ret i32 %70

poison.cont16:                                    ; preds = %poison.cont14
  %71 = call i32 @ExpressionSemantics_x3a_x3astatementLoweringEvidenceSink(ptr noundef nonnull align 4 dereferenceable(4) %total, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %72 = load ptr, ptr %__panic, align 8
  %73 = load i8, ptr %72, align 1
  %74 = icmp ne i8 %73, 0
  br i1 %74, label %panic.take17, label %panic.cont18

panic.take17:                                     ; preds = %poison.cont16
  %75 = load ptr, ptr %__panic, align 8
  %76 = getelementptr i8, ptr %75, i64 4
  %77 = load i32, ptr %76, align 4
  ret i32 %77

panic.cont18:                                     ; preds = %poison.cont16
  %78 = load i32, ptr %total, align 4
  %79 = icmp sge i32 %78, 0
  %80 = zext i1 %79 to i8
  %81 = icmp ne i8 %80, 0
  %82 = icmp ne i8 %80, 0
  br i1 %82, label %if.then, label %if.else

if.then:                                          ; preds = %panic.cont18
  br label %if.merge

if.else:                                          ; preds = %panic.cont18
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  store i32 0, ptr %loop.result.slot, align 4
  br label %loop.cond

loop.end:                                         ; preds = %loop.body, %loop.cond
  %83 = load i32, ptr %loop.result.slot, align 4
  store i32 %83, ptr %valued_break, align 4
  br i1 true, label %check_ok19, label %check_fail20

loop.cond:                                        ; preds = %if.merge
  br i1 true, label %loop.body, label %loop.end

loop.body:                                        ; preds = %loop.cond
  store i32 4, ptr %loop.result.slot, align 4
  br label %loop.end

check_ok19:                                       ; preds = %check_fail20, %loop.end
  %84 = load ptr, ptr %__panic, align 8
  %85 = load i8, ptr %84, align 1
  %86 = icmp ne i8 %85, 0
  br i1 %86, label %panic.take21, label %panic.cont22

check_fail20:                                     ; preds = %loop.end
  %87 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %87, align 1
  %88 = getelementptr i8, ptr %87, i64 4
  store i32 4, ptr %88, align 4
  br label %check_ok19

panic.take21:                                     ; preds = %check_ok19
  %89 = load ptr, ptr %__panic, align 8
  %90 = getelementptr i8, ptr %89, i64 0
  %91 = load i8, ptr %90, align 1
  %92 = load ptr, ptr %__panic, align 8
  %93 = getelementptr i8, ptr %92, i64 4
  %94 = load i32, ptr %93, align 4
  %95 = load ptr, ptr %__panic, align 8
  %96 = getelementptr i8, ptr %95, i64 4
  %97 = load i32, ptr %96, align 4
  ret i32 %97

panic.cont22:                                     ; preds = %check_ok19
  %98 = load i32, ptr %total, align 4
  %99 = load i32, ptr %valued_break, align 4
  %100 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %98, i32 %99)
  %101 = extractvalue { i32, i1 } %100, 0
  %102 = extractvalue { i32, i1 } %100, 1
  %103 = freeze i32 %101
  br i1 %102, label %op_fail24, label %op_ok23

op_ok23:                                          ; preds = %panic.cont22
  store i32 %103, ptr %total, align 4
  store i32 0, ptr %loop_index, align 4
  br label %loop.head

op_fail24:                                        ; preds = %panic.cont22
  %104 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %104, align 1
  %105 = getelementptr i8, ptr %104, i64 4
  store i32 4, ptr %105, align 4
  ret i32 0

loop.end25:                                       ; preds = %if.then42
  br i1 true, label %check_ok45, label %check_fail46

loop.head:                                        ; preds = %if.merge44, %if.then33, %op_ok23
  br label %loop.body26

loop.body26:                                      ; preds = %loop.head
  br i1 true, label %check_ok27, label %check_fail28

check_ok27:                                       ; preds = %check_fail28, %loop.body26
  %106 = load ptr, ptr %__panic, align 8
  %107 = load i8, ptr %106, align 1
  %108 = icmp ne i8 %107, 0
  br i1 %108, label %panic.take29, label %panic.cont30

check_fail28:                                     ; preds = %loop.body26
  %109 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %109, align 1
  %110 = getelementptr i8, ptr %109, i64 4
  store i32 4, ptr %110, align 4
  br label %check_ok27

panic.take29:                                     ; preds = %check_ok27
  %111 = load ptr, ptr %__panic, align 8
  %112 = getelementptr i8, ptr %111, i64 0
  %113 = load i8, ptr %112, align 1
  %114 = load ptr, ptr %__panic, align 8
  %115 = getelementptr i8, ptr %114, i64 4
  %116 = load i32, ptr %115, align 4
  %117 = load ptr, ptr %__panic, align 8
  %118 = getelementptr i8, ptr %117, i64 4
  %119 = load i32, ptr %118, align 4
  ret i32 %119

panic.cont30:                                     ; preds = %check_ok27
  %120 = load i32, ptr %loop_index, align 4
  %121 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %120, i32 1)
  %122 = extractvalue { i32, i1 } %121, 0
  %123 = extractvalue { i32, i1 } %121, 1
  %124 = freeze i32 %122
  br i1 %123, label %op_fail32, label %op_ok31

op_ok31:                                          ; preds = %panic.cont30
  store i32 %124, ptr %loop_index, align 4
  %125 = load i32, ptr %loop_index, align 4
  %126 = icmp eq i32 %125, 2
  %127 = zext i1 %126 to i8
  %128 = icmp ne i8 %127, 0
  %129 = icmp ne i8 %127, 0
  br i1 %129, label %if.then33, label %if.else34

op_fail32:                                        ; preds = %panic.cont30
  %130 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %130, align 1
  %131 = getelementptr i8, ptr %130, i64 4
  store i32 4, ptr %131, align 4
  ret i32 0

if.then33:                                        ; preds = %op_ok31
  br label %loop.head

if.else34:                                        ; preds = %op_ok31
  br label %if.merge35

if.merge35:                                       ; preds = %if.else34
  br i1 true, label %check_ok36, label %check_fail37

check_ok36:                                       ; preds = %check_fail37, %if.merge35
  %132 = load ptr, ptr %__panic, align 8
  %133 = load i8, ptr %132, align 1
  %134 = icmp ne i8 %133, 0
  br i1 %134, label %panic.take38, label %panic.cont39

check_fail37:                                     ; preds = %if.merge35
  %135 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %135, align 1
  %136 = getelementptr i8, ptr %135, i64 4
  store i32 4, ptr %136, align 4
  br label %check_ok36

panic.take38:                                     ; preds = %check_ok36
  %137 = load ptr, ptr %__panic, align 8
  %138 = getelementptr i8, ptr %137, i64 0
  %139 = load i8, ptr %138, align 1
  %140 = load ptr, ptr %__panic, align 8
  %141 = getelementptr i8, ptr %140, i64 4
  %142 = load i32, ptr %141, align 4
  %143 = load ptr, ptr %__panic, align 8
  %144 = getelementptr i8, ptr %143, i64 4
  %145 = load i32, ptr %144, align 4
  ret i32 %145

panic.cont39:                                     ; preds = %check_ok36
  %146 = load i32, ptr %total, align 4
  %147 = load i32, ptr %loop_index, align 4
  %148 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %146, i32 %147)
  %149 = extractvalue { i32, i1 } %148, 0
  %150 = extractvalue { i32, i1 } %148, 1
  %151 = freeze i32 %149
  br i1 %150, label %op_fail41, label %op_ok40

op_ok40:                                          ; preds = %panic.cont39
  store i32 %151, ptr %total, align 4
  %152 = load i32, ptr %loop_index, align 4
  %153 = icmp sge i32 %152, 3
  %154 = zext i1 %153 to i8
  %155 = icmp ne i8 %154, 0
  %156 = icmp ne i8 %154, 0
  br i1 %156, label %if.then42, label %if.else43

op_fail41:                                        ; preds = %panic.cont39
  %157 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %157, align 1
  %158 = getelementptr i8, ptr %157, i64 4
  store i32 4, ptr %158, align 4
  ret i32 0

if.then42:                                        ; preds = %op_ok40
  br label %loop.end25

if.else43:                                        ; preds = %op_ok40
  br label %if.merge44

if.merge44:                                       ; preds = %if.else43
  br label %loop.head

check_ok45:                                       ; preds = %check_fail46, %loop.end25
  %159 = load ptr, ptr %__panic, align 8
  %160 = load i8, ptr %159, align 1
  %161 = icmp ne i8 %160, 0
  br i1 %161, label %panic.take47, label %panic.cont48

check_fail46:                                     ; preds = %loop.end25
  %162 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %162, align 1
  %163 = getelementptr i8, ptr %162, i64 4
  store i32 4, ptr %163, align 4
  br label %check_ok45

panic.take47:                                     ; preds = %check_ok45
  %164 = load ptr, ptr %__panic, align 8
  %165 = getelementptr i8, ptr %164, i64 0
  %166 = load i8, ptr %165, align 1
  %167 = load ptr, ptr %__panic, align 8
  %168 = getelementptr i8, ptr %167, i64 4
  %169 = load i32, ptr %168, align 4
  %170 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %170, align 1
  %171 = getelementptr i8, ptr %170, i64 4
  store i32 0, ptr %171, align 4
  br i1 true, label %check_ok49, label %check_fail50

panic.cont48:                                     ; preds = %check_ok45
  %172 = load i32, ptr %total, align 4
  %173 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %172, i32 1)
  %174 = extractvalue { i32, i1 } %173, 0
  %175 = extractvalue { i32, i1 } %173, 1
  %176 = freeze i32 %174
  br i1 %175, label %op_fail66, label %op_ok65

check_ok49:                                       ; preds = %check_fail50, %panic.take47
  %177 = load ptr, ptr %__panic, align 8
  %178 = load i8, ptr %177, align 1
  %179 = icmp ne i8 %178, 0
  br i1 %179, label %panic.take51, label %panic.cont52

check_fail50:                                     ; preds = %panic.take47
  %180 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %180, align 1
  %181 = getelementptr i8, ptr %180, i64 4
  store i32 4, ptr %181, align 4
  br label %check_ok49

panic.take51:                                     ; preds = %check_ok49
  %182 = load ptr, ptr %__panic, align 8
  %183 = getelementptr i8, ptr %182, i64 0
  %184 = load i8, ptr %183, align 1
  %185 = load ptr, ptr %__panic, align 8
  %186 = getelementptr i8, ptr %185, i64 4
  %187 = load i32, ptr %186, align 4
  %188 = load ptr, ptr %__panic, align 8
  %189 = getelementptr i8, ptr %188, i64 4
  %190 = load i32, ptr %189, align 4
  ret i32 %190

panic.cont52:                                     ; preds = %check_ok49
  %191 = load i32, ptr %total, align 4
  %192 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %191, i32 5)
  %193 = extractvalue { i32, i1 } %192, 0
  %194 = extractvalue { i32, i1 } %192, 1
  %195 = freeze i32 %193
  br i1 %194, label %op_fail54, label %op_ok53

op_ok53:                                          ; preds = %panic.cont52
  store i32 %195, ptr %total, align 4
  %196 = load ptr, ptr %__panic, align 8
  %197 = getelementptr i8, ptr %196, i64 0
  %198 = load i8, ptr %197, align 1
  %199 = load ptr, ptr %__panic, align 8
  %200 = getelementptr i8, ptr %199, i64 4
  %201 = load i32, ptr %200, align 4
  %202 = icmp ne i8 %198, 0
  %203 = zext i1 %202 to i8
  %204 = icmp ne i8 %203, 0
  %205 = and i1 true, %204
  br i1 %205, label %if.then55, label %if.else56

op_fail54:                                        ; preds = %panic.cont52
  %206 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %206, align 1
  %207 = getelementptr i8, ptr %206, i64 4
  store i32 4, ptr %207, align 4
  ret i32 0

if.then55:                                        ; preds = %op_ok53
  store i32 %201, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else56:                                        ; preds = %op_ok53
  br label %if.merge57

if.merge57:                                       ; preds = %if.else56
  %208 = icmp ne i8 %198, 0
  %209 = xor i1 %208, true
  %210 = and i1 true, %209
  br i1 %210, label %if.then58, label %if.else59

if.then58:                                        ; preds = %if.merge57
  %211 = load ptr, ptr %__panic, align 8
  %212 = getelementptr i8, ptr %211, i64 0
  store i8 1, ptr %212, align 1
  %213 = load ptr, ptr %__panic, align 8
  %214 = getelementptr i8, ptr %213, i64 4
  store i32 %169, ptr %214, align 4
  br label %if.merge60

if.else59:                                        ; preds = %if.merge57
  br label %if.merge60

if.merge60:                                       ; preds = %if.else59, %if.then58
  %if.result = phi i64 [ 0, %if.then58 ], [ 0, %if.else59 ]
  %215 = icmp ne i8 %198, 0
  %216 = zext i1 %215 to i8
  %217 = icmp ne i8 %216, 0
  %218 = or i1 true, %217
  %219 = icmp ne i8 %198, 0
  %220 = icmp ne i8 %198, 0
  br i1 %220, label %if.then61, label %if.else62

if.then61:                                        ; preds = %if.merge60
  br label %if.merge63

if.else62:                                        ; preds = %if.merge60
  br label %if.merge63

if.merge63:                                       ; preds = %if.else62, %if.then61
  %if.result64 = phi i32 [ %201, %if.then61 ], [ %169, %if.else62 ]
  %221 = load ptr, ptr %__panic, align 8
  %222 = getelementptr i8, ptr %221, i64 4
  %223 = load i32, ptr %222, align 4
  ret i32 %223

op_ok65:                                          ; preds = %panic.cont48
  store i32 %176, ptr %total, align 4
  %224 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %224, align 1
  %225 = getelementptr i8, ptr %224, i64 4
  store i32 0, ptr %225, align 4
  br i1 true, label %check_ok67, label %check_fail68

op_fail66:                                        ; preds = %panic.cont48
  %226 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %226, align 1
  %227 = getelementptr i8, ptr %226, i64 4
  store i32 4, ptr %227, align 4
  ret i32 0

check_ok67:                                       ; preds = %check_fail68, %op_ok65
  %228 = load ptr, ptr %__panic, align 8
  %229 = load i8, ptr %228, align 1
  %230 = icmp ne i8 %229, 0
  br i1 %230, label %panic.take69, label %panic.cont70

check_fail68:                                     ; preds = %op_ok65
  %231 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %231, align 1
  %232 = getelementptr i8, ptr %231, i64 4
  store i32 4, ptr %232, align 4
  br label %check_ok67

panic.take69:                                     ; preds = %check_ok67
  %233 = load ptr, ptr %__panic, align 8
  %234 = getelementptr i8, ptr %233, i64 0
  %235 = load i8, ptr %234, align 1
  %236 = load ptr, ptr %__panic, align 8
  %237 = getelementptr i8, ptr %236, i64 4
  %238 = load i32, ptr %237, align 4
  %239 = load ptr, ptr %__panic, align 8
  %240 = getelementptr i8, ptr %239, i64 4
  %241 = load i32, ptr %240, align 4
  ret i32 %241

panic.cont70:                                     ; preds = %check_ok67
  %242 = load i32, ptr %total, align 4
  %243 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %242, i32 5)
  %244 = extractvalue { i32, i1 } %243, 0
  %245 = extractvalue { i32, i1 } %243, 1
  %246 = freeze i32 %244
  br i1 %245, label %op_fail72, label %op_ok71

op_ok71:                                          ; preds = %panic.cont70
  store i32 %246, ptr %total, align 4
  %247 = load ptr, ptr %__panic, align 8
  %248 = getelementptr i8, ptr %247, i64 0
  %249 = load i8, ptr %248, align 1
  %250 = load ptr, ptr %__panic, align 8
  %251 = getelementptr i8, ptr %250, i64 4
  %252 = load i32, ptr %251, align 4
  %253 = icmp ne i8 %249, 0
  %254 = zext i1 %253 to i8
  %255 = icmp ne i8 %254, 0
  %256 = and i1 false, %255
  br i1 %256, label %if.then73, label %if.else74

op_fail72:                                        ; preds = %panic.cont70
  %257 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %257, align 1
  %258 = getelementptr i8, ptr %257, i64 4
  store i32 4, ptr %258, align 4
  ret i32 0

if.then73:                                        ; preds = %op_ok71
  store i32 %252, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else74:                                        ; preds = %op_ok71
  br label %if.merge75

if.merge75:                                       ; preds = %if.else74
  %259 = icmp ne i8 %249, 0
  %260 = xor i1 %259, true
  %261 = and i1 false, %260
  br i1 %261, label %if.then76, label %if.else77

if.then76:                                        ; preds = %if.merge75
  %262 = load ptr, ptr %__panic, align 8
  %263 = getelementptr i8, ptr %262, i64 0
  store i8 1, ptr %263, align 1
  %264 = load ptr, ptr %__panic, align 8
  %265 = getelementptr i8, ptr %264, i64 4
  store i32 0, ptr %265, align 4
  br label %if.merge78

if.else77:                                        ; preds = %if.merge75
  br label %if.merge78

if.merge78:                                       ; preds = %if.else77, %if.then76
  %if.result79 = phi i64 [ 0, %if.then76 ], [ 0, %if.else77 ]
  %266 = icmp ne i8 %249, 0
  %267 = zext i1 %266 to i8
  %268 = icmp ne i8 %267, 0
  %269 = or i1 false, %268
  %270 = icmp ne i8 %249, 0
  %271 = icmp ne i8 %249, 0
  br i1 %271, label %if.then80, label %if.else81

if.then80:                                        ; preds = %if.merge78
  br label %if.merge82

if.else81:                                        ; preds = %if.merge78
  br label %if.merge82

if.merge82:                                       ; preds = %if.else81, %if.then80
  %if.result83 = phi i32 [ %252, %if.then80 ], [ 0, %if.else81 ]
  %272 = load ptr, ptr %__panic, align 8
  %273 = load i8, ptr %272, align 1
  %274 = icmp ne i8 %273, 0
  br i1 %274, label %panic.take84, label %panic.cont85

panic.take84:                                     ; preds = %if.merge82
  %275 = load ptr, ptr %__panic, align 8
  %276 = getelementptr i8, ptr %275, i64 0
  %277 = load i8, ptr %276, align 1
  %278 = load ptr, ptr %__panic, align 8
  %279 = getelementptr i8, ptr %278, i64 4
  %280 = load i32, ptr %279, align 4
  %281 = load ptr, ptr %__panic, align 8
  %282 = getelementptr i8, ptr %281, i64 4
  %283 = load i32, ptr %282, align 4
  ret i32 %283

panic.cont85:                                     ; preds = %if.merge82
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fenter(i64 54)
  store { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] } zeroinitializer, ptr %0, align 4
  %284 = getelementptr i8, ptr %0, i64 0
  store i64 0, ptr %284, align 1
  %285 = getelementptr i8, ptr %0, i64 8
  store { i8, [7 x i8], [24 x i8], [0 x i64] } zeroinitializer, ptr %285, align 1
  %286 = load { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] }, ptr %0, align 4
  store { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] } %286, ptr %byref_arg86, align 4
  %287 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3anew_x5fscoped(ptr noundef nonnull align 8 dereferenceable(40) %byref_arg86)
  store { i64, i64 } %287, ptr %abi_return, align 8
  %288 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return, align 1
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %288, ptr %__bind_26_scratch, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fenter(i64 56)
  %289 = call i64 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3amark(ptr noundef nonnull align 8 dereferenceable(16) %__bind_26_scratch)
  br i1 true, label %check_ok87, label %check_fail88

check_ok87:                                       ; preds = %check_fail88, %panic.cont85
  %290 = load ptr, ptr %__panic, align 8
  %291 = load i8, ptr %290, align 1
  %292 = icmp ne i8 %291, 0
  br i1 %292, label %panic.take89, label %panic.cont90

check_fail88:                                     ; preds = %panic.cont85
  %293 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %293, align 1
  %294 = getelementptr i8, ptr %293, i64 4
  store i32 4, ptr %294, align 4
  br label %check_ok87

panic.take89:                                     ; preds = %check_ok87
  %295 = load ptr, ptr %__panic, align 8
  %296 = getelementptr i8, ptr %295, i64 0
  %297 = load i8, ptr %296, align 1
  %298 = load ptr, ptr %__panic, align 8
  %299 = getelementptr i8, ptr %298, i64 4
  %300 = load i32, ptr %299, align 4
  %301 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %301, align 1
  %302 = getelementptr i8, ptr %301, i64 4
  store i32 0, ptr %302, align 4
  store i64 %289, ptr %byref_arg91, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3areset_x5fto(ptr noundef nonnull align 8 dereferenceable(16) %__bind_26_scratch, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg91)
  %303 = load ptr, ptr %__panic, align 8
  %304 = getelementptr i8, ptr %303, i64 0
  %305 = load i8, ptr %304, align 1
  %306 = load ptr, ptr %__panic, align 8
  %307 = getelementptr i8, ptr %306, i64 4
  %308 = load i32, ptr %307, align 4
  %309 = icmp ne i8 %305, 0
  %310 = zext i1 %309 to i8
  %311 = icmp ne i8 %310, 0
  %312 = and i1 true, %311
  br i1 %312, label %if.then92, label %if.else93

panic.cont90:                                     ; preds = %check_ok87
  %313 = load i32, ptr %total, align 4
  %314 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %313, i32 1)
  %315 = extractvalue { i32, i1 } %314, 0
  %316 = extractvalue { i32, i1 } %314, 1
  %317 = freeze i32 %315
  br i1 %316, label %op_fail105, label %op_ok104

if.then92:                                        ; preds = %panic.take89
  store i32 %308, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else93:                                        ; preds = %panic.take89
  br label %if.merge94

if.merge94:                                       ; preds = %if.else93
  %318 = icmp ne i8 %305, 0
  %319 = xor i1 %318, true
  %320 = and i1 true, %319
  br i1 %320, label %if.then95, label %if.else96

if.then95:                                        ; preds = %if.merge94
  %321 = load ptr, ptr %__panic, align 8
  %322 = getelementptr i8, ptr %321, i64 0
  store i8 1, ptr %322, align 1
  %323 = load ptr, ptr %__panic, align 8
  %324 = getelementptr i8, ptr %323, i64 4
  store i32 %300, ptr %324, align 4
  br label %if.merge97

if.else96:                                        ; preds = %if.merge94
  br label %if.merge97

if.merge97:                                       ; preds = %if.else96, %if.then95
  %if.result98 = phi i64 [ 0, %if.then95 ], [ 0, %if.else96 ]
  %325 = icmp ne i8 %305, 0
  %326 = zext i1 %325 to i8
  %327 = icmp ne i8 %326, 0
  %328 = or i1 true, %327
  %329 = icmp ne i8 %305, 0
  %330 = icmp ne i8 %305, 0
  br i1 %330, label %if.then99, label %if.else100

if.then99:                                        ; preds = %if.merge97
  br label %if.merge101

if.else100:                                       ; preds = %if.merge97
  br label %if.merge101

if.merge101:                                      ; preds = %if.else100, %if.then99
  %if.result102 = phi i32 [ %308, %if.then99 ], [ %300, %if.else100 ]
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64 56)
  %331 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %__bind_26_scratch)
  store { i64, i64 } %331, ptr %abi_return103, align 8
  %332 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return103, align 1
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64 54)
  %333 = load ptr, ptr %__panic, align 8
  %334 = getelementptr i8, ptr %333, i64 4
  %335 = load i32, ptr %334, align 4
  ret i32 %335

op_ok104:                                         ; preds = %panic.cont90
  store i32 %317, ptr %total, align 4
  %336 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %336, align 1
  %337 = getelementptr i8, ptr %336, i64 4
  store i32 0, ptr %337, align 4
  store i64 %289, ptr %byref_arg91, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3areset_x5fto(ptr noundef nonnull align 8 dereferenceable(16) %__bind_26_scratch, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg91)
  %338 = load ptr, ptr %__panic, align 8
  %339 = getelementptr i8, ptr %338, i64 0
  %340 = load i8, ptr %339, align 1
  %341 = load ptr, ptr %__panic, align 8
  %342 = getelementptr i8, ptr %341, i64 4
  %343 = load i32, ptr %342, align 4
  %344 = icmp ne i8 %340, 0
  %345 = zext i1 %344 to i8
  %346 = icmp ne i8 %345, 0
  %347 = and i1 false, %346
  br i1 %347, label %if.then106, label %if.else107

op_fail105:                                       ; preds = %panic.cont90
  %348 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %348, align 1
  %349 = getelementptr i8, ptr %348, i64 4
  store i32 4, ptr %349, align 4
  ret i32 0

if.then106:                                       ; preds = %op_ok104
  store i32 %343, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else107:                                       ; preds = %op_ok104
  br label %if.merge108

if.merge108:                                      ; preds = %if.else107
  %350 = icmp ne i8 %340, 0
  %351 = xor i1 %350, true
  %352 = and i1 false, %351
  br i1 %352, label %if.then109, label %if.else110

if.then109:                                       ; preds = %if.merge108
  %353 = load ptr, ptr %__panic, align 8
  %354 = getelementptr i8, ptr %353, i64 0
  store i8 1, ptr %354, align 1
  %355 = load ptr, ptr %__panic, align 8
  %356 = getelementptr i8, ptr %355, i64 4
  store i32 0, ptr %356, align 4
  br label %if.merge111

if.else110:                                       ; preds = %if.merge108
  br label %if.merge111

if.merge111:                                      ; preds = %if.else110, %if.then109
  %if.result112 = phi i64 [ 0, %if.then109 ], [ 0, %if.else110 ]
  %357 = icmp ne i8 %340, 0
  %358 = zext i1 %357 to i8
  %359 = icmp ne i8 %358, 0
  %360 = or i1 false, %359
  %361 = icmp ne i8 %340, 0
  %362 = icmp ne i8 %340, 0
  br i1 %362, label %if.then113, label %if.else114

if.then113:                                       ; preds = %if.merge111
  br label %if.merge115

if.else114:                                       ; preds = %if.merge111
  br label %if.merge115

if.merge115:                                      ; preds = %if.else114, %if.then113
  %if.result116 = phi i32 [ %343, %if.then113 ], [ 0, %if.else114 ]
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64 56)
  %363 = load ptr, ptr %__panic, align 8
  %364 = load i8, ptr %363, align 1
  %365 = icmp ne i8 %364, 0
  br i1 %365, label %panic.take117, label %panic.cont118

panic.take117:                                    ; preds = %if.merge115
  %366 = load ptr, ptr %__panic, align 8
  %367 = getelementptr i8, ptr %366, i64 0
  %368 = load i8, ptr %367, align 1
  %369 = load ptr, ptr %__panic, align 8
  %370 = getelementptr i8, ptr %369, i64 4
  %371 = load i32, ptr %370, align 4
  %372 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %__bind_26_scratch)
  store { i64, i64 } %372, ptr %abi_return119, align 8
  %373 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return119, align 1
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64 54)
  %374 = load ptr, ptr %__panic, align 8
  %375 = getelementptr i8, ptr %374, i64 4
  %376 = load i32, ptr %375, align 4
  ret i32 %376

panic.cont118:                                    ; preds = %if.merge115
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fenter(i64 57)
  %377 = call i64 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3amark(ptr noundef nonnull align 8 dereferenceable(16) %__bind_26_scratch)
  br i1 true, label %check_ok120, label %check_fail121

check_ok120:                                      ; preds = %check_fail121, %panic.cont118
  %378 = load ptr, ptr %__panic, align 8
  %379 = load i8, ptr %378, align 1
  %380 = icmp ne i8 %379, 0
  br i1 %380, label %panic.take122, label %panic.cont123

check_fail121:                                    ; preds = %panic.cont118
  %381 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %381, align 1
  %382 = getelementptr i8, ptr %381, i64 4
  store i32 4, ptr %382, align 4
  br label %check_ok120

panic.take122:                                    ; preds = %check_ok120
  %383 = load ptr, ptr %__panic, align 8
  %384 = getelementptr i8, ptr %383, i64 0
  %385 = load i8, ptr %384, align 1
  %386 = load ptr, ptr %__panic, align 8
  %387 = getelementptr i8, ptr %386, i64 4
  %388 = load i32, ptr %387, align 4
  %389 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %389, align 1
  %390 = getelementptr i8, ptr %389, i64 4
  store i32 0, ptr %390, align 4
  store i64 %377, ptr %byref_arg91, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3areset_x5fto(ptr noundef nonnull align 8 dereferenceable(16) %__bind_26_scratch, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg91)
  %391 = load ptr, ptr %__panic, align 8
  %392 = getelementptr i8, ptr %391, i64 0
  %393 = load i8, ptr %392, align 1
  %394 = load ptr, ptr %__panic, align 8
  %395 = getelementptr i8, ptr %394, i64 4
  %396 = load i32, ptr %395, align 4
  %397 = icmp ne i8 %393, 0
  %398 = zext i1 %397 to i8
  %399 = icmp ne i8 %398, 0
  %400 = and i1 true, %399
  br i1 %400, label %if.then124, label %if.else125

panic.cont123:                                    ; preds = %check_ok120
  %401 = load i32, ptr %total, align 4
  %402 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %401, i32 1)
  %403 = extractvalue { i32, i1 } %402, 0
  %404 = extractvalue { i32, i1 } %402, 1
  %405 = freeze i32 %403
  br i1 %404, label %op_fail137, label %op_ok136

if.then124:                                       ; preds = %panic.take122
  store i32 %396, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else125:                                       ; preds = %panic.take122
  br label %if.merge126

if.merge126:                                      ; preds = %if.else125
  %406 = icmp ne i8 %393, 0
  %407 = xor i1 %406, true
  %408 = and i1 true, %407
  br i1 %408, label %if.then127, label %if.else128

if.then127:                                       ; preds = %if.merge126
  %409 = load ptr, ptr %__panic, align 8
  %410 = getelementptr i8, ptr %409, i64 0
  store i8 1, ptr %410, align 1
  %411 = load ptr, ptr %__panic, align 8
  %412 = getelementptr i8, ptr %411, i64 4
  store i32 %388, ptr %412, align 4
  br label %if.merge129

if.else128:                                       ; preds = %if.merge126
  br label %if.merge129

if.merge129:                                      ; preds = %if.else128, %if.then127
  %if.result130 = phi i64 [ 0, %if.then127 ], [ 0, %if.else128 ]
  %413 = icmp ne i8 %393, 0
  %414 = zext i1 %413 to i8
  %415 = icmp ne i8 %414, 0
  %416 = or i1 true, %415
  %417 = icmp ne i8 %393, 0
  %418 = icmp ne i8 %393, 0
  br i1 %418, label %if.then131, label %if.else132

if.then131:                                       ; preds = %if.merge129
  br label %if.merge133

if.else132:                                       ; preds = %if.merge129
  br label %if.merge133

if.merge133:                                      ; preds = %if.else132, %if.then131
  %if.result134 = phi i32 [ %396, %if.then131 ], [ %388, %if.else132 ]
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64 57)
  %419 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %__bind_26_scratch)
  store { i64, i64 } %419, ptr %abi_return135, align 8
  %420 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return135, align 1
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64 54)
  %421 = load ptr, ptr %__panic, align 8
  %422 = getelementptr i8, ptr %421, i64 4
  %423 = load i32, ptr %422, align 4
  ret i32 %423

op_ok136:                                         ; preds = %panic.cont123
  store i32 %405, ptr %total, align 4
  %424 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %424, align 1
  %425 = getelementptr i8, ptr %424, i64 4
  store i32 0, ptr %425, align 4
  store i64 %377, ptr %byref_arg91, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3areset_x5fto(ptr noundef nonnull align 8 dereferenceable(16) %__bind_26_scratch, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg91)
  %426 = load ptr, ptr %__panic, align 8
  %427 = getelementptr i8, ptr %426, i64 0
  %428 = load i8, ptr %427, align 1
  %429 = load ptr, ptr %__panic, align 8
  %430 = getelementptr i8, ptr %429, i64 4
  %431 = load i32, ptr %430, align 4
  %432 = icmp ne i8 %428, 0
  %433 = zext i1 %432 to i8
  %434 = icmp ne i8 %433, 0
  %435 = and i1 false, %434
  br i1 %435, label %if.then138, label %if.else139

op_fail137:                                       ; preds = %panic.cont123
  %436 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %436, align 1
  %437 = getelementptr i8, ptr %436, i64 4
  store i32 4, ptr %437, align 4
  ret i32 0

if.then138:                                       ; preds = %op_ok136
  store i32 %431, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else139:                                       ; preds = %op_ok136
  br label %if.merge140

if.merge140:                                      ; preds = %if.else139
  %438 = icmp ne i8 %428, 0
  %439 = xor i1 %438, true
  %440 = and i1 false, %439
  br i1 %440, label %if.then141, label %if.else142

if.then141:                                       ; preds = %if.merge140
  %441 = load ptr, ptr %__panic, align 8
  %442 = getelementptr i8, ptr %441, i64 0
  store i8 1, ptr %442, align 1
  %443 = load ptr, ptr %__panic, align 8
  %444 = getelementptr i8, ptr %443, i64 4
  store i32 0, ptr %444, align 4
  br label %if.merge143

if.else142:                                       ; preds = %if.merge140
  br label %if.merge143

if.merge143:                                      ; preds = %if.else142, %if.then141
  %if.result144 = phi i64 [ 0, %if.then141 ], [ 0, %if.else142 ]
  %445 = icmp ne i8 %428, 0
  %446 = zext i1 %445 to i8
  %447 = icmp ne i8 %446, 0
  %448 = or i1 false, %447
  %449 = icmp ne i8 %428, 0
  %450 = icmp ne i8 %428, 0
  br i1 %450, label %if.then145, label %if.else146

if.then145:                                       ; preds = %if.merge143
  br label %if.merge147

if.else146:                                       ; preds = %if.merge143
  br label %if.merge147

if.merge147:                                      ; preds = %if.else146, %if.then145
  %if.result148 = phi i32 [ %431, %if.then145 ], [ 0, %if.else146 ]
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64 57)
  %451 = load ptr, ptr %__panic, align 8
  %452 = load i8, ptr %451, align 1
  %453 = icmp ne i8 %452, 0
  br i1 %453, label %panic.take149, label %panic.cont150

panic.take149:                                    ; preds = %if.merge147
  %454 = load ptr, ptr %__panic, align 8
  %455 = getelementptr i8, ptr %454, i64 0
  %456 = load i8, ptr %455, align 1
  %457 = load ptr, ptr %__panic, align 8
  %458 = getelementptr i8, ptr %457, i64 4
  %459 = load i32, ptr %458, align 4
  %460 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %__bind_26_scratch)
  store { i64, i64 } %460, ptr %abi_return151, align 8
  %461 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return151, align 1
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64 54)
  %462 = load ptr, ptr %__panic, align 8
  %463 = getelementptr i8, ptr %462, i64 4
  %464 = load i32, ptr %463, align 4
  ret i32 %464

panic.cont150:                                    ; preds = %if.merge147
  br i1 true, label %check_ok152, label %check_fail153

check_ok152:                                      ; preds = %check_fail153, %panic.cont150
  %465 = load ptr, ptr %__panic, align 8
  %466 = load i8, ptr %465, align 1
  %467 = icmp ne i8 %466, 0
  br i1 %467, label %panic.take154, label %panic.cont155

check_fail153:                                    ; preds = %panic.cont150
  %468 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %468, align 1
  %469 = getelementptr i8, ptr %468, i64 4
  store i32 4, ptr %469, align 4
  br label %check_ok152

panic.take154:                                    ; preds = %check_ok152
  %470 = load ptr, ptr %__panic, align 8
  %471 = getelementptr i8, ptr %470, i64 0
  %472 = load i8, ptr %471, align 1
  %473 = load ptr, ptr %__panic, align 8
  %474 = getelementptr i8, ptr %473, i64 4
  %475 = load i32, ptr %474, align 4
  %476 = load ptr, ptr %__panic, align 8
  %477 = getelementptr i8, ptr %476, i64 4
  %478 = load i32, ptr %477, align 4
  ret i32 %478

panic.cont155:                                    ; preds = %check_ok152
  %479 = load i32, ptr %total, align 4
  %480 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %479, i32 1)
  %481 = extractvalue { i32, i1 } %480, 0
  %482 = extractvalue { i32, i1 } %480, 1
  %483 = freeze i32 %481
  br i1 %482, label %op_fail157, label %op_ok156

op_ok156:                                         ; preds = %panic.cont155
  store i32 %483, ptr %total, align 4
  %484 = load i32, ptr %total, align 4
  ret i32 %484

op_fail157:                                       ; preds = %panic.cont155
  %485 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %485, align 1
  %486 = getelementptr i8, ptr %485, i64 4
  store i32 4, ptr %486, align 4
  ret i32 0
}

define i32 @ExpressionSemantics_x3a_x3arawDerefReference(ptr noundef nonnull align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %9 = load ptr, ptr %pointer, align 8
  %10 = icmp ne ptr %9, null
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
  store i32 8, ptr %15, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %16 = load ptr, ptr %__panic, align 8
  %17 = getelementptr i8, ptr %16, i64 4
  %18 = load i32, ptr %17, align 4
  ret i32 %18

panic.cont:                                       ; preds = %check_ok
  %19 = load ptr, ptr %pointer, align 8
  %20 = load i32, ptr %19, align 4
  ret i32 %20
}

define ptr @ExpressionSemantics_x3a_x3arawImmutableDerefPlaceReference(ptr noundef nonnull align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret ptr null

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 0, ptr %5, align 4
  %6 = load ptr, ptr %pointer, align 8
  %7 = load ptr, ptr %pointer, align 8
  ret ptr %7
}

define i32 @ExpressionSemantics_x3a_x3aderefPlaceReference(ptr noundef nonnull align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %9 = load ptr, ptr %pointer, align 8
  %10 = icmp ne ptr %9, null
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
  store i32 8, ptr %15, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %16 = load ptr, ptr %__panic, align 8
  %17 = getelementptr i8, ptr %16, i64 4
  %18 = load i32, ptr %17, align 4
  ret i32 %18

panic.cont:                                       ; preds = %check_ok
  %19 = load ptr, ptr %pointer, align 8
  store i32 17, ptr %19, align 4
  %20 = load ptr, ptr %pointer, align 8
  %21 = icmp ne ptr %20, null
  br i1 %21, label %check_ok1, label %check_fail2

check_ok1:                                        ; preds = %check_fail2, %panic.cont
  %22 = load ptr, ptr %__panic, align 8
  %23 = load i8, ptr %22, align 1
  %24 = icmp ne i8 %23, 0
  br i1 %24, label %panic.take3, label %panic.cont4

check_fail2:                                      ; preds = %panic.cont
  %25 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %25, align 1
  %26 = getelementptr i8, ptr %25, i64 4
  store i32 8, ptr %26, align 4
  br label %check_ok1

panic.take3:                                      ; preds = %check_ok1
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 4
  %29 = load i32, ptr %28, align 4
  ret i32 %29

panic.cont4:                                      ; preds = %check_ok1
  %30 = load ptr, ptr %pointer, align 8
  %31 = load i32, ptr %30, align 4
  ret i32 %31
}

define i32 @ExpressionSemantics_x3a_x3alowerExprAllocReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %abi_return12 = alloca { i64, i64 }, align 8
  %explicit_value = alloca i32, align 4
  %abi_return4 = alloca { i64, i64 }, align 8
  %byref_arg3 = alloca i64, align 8
  %byref_arg2 = alloca i64, align 8
  %byref_arg1 = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %byref_arg = alloca { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] }, align 8
  %__bind_31_scratch = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %0 = alloca { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] }, align 8
  %score = alloca i32, align 4
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  store i32 0, ptr %score, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fenter(i64 71)
  store { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] } zeroinitializer, ptr %0, align 4
  %10 = getelementptr i8, ptr %0, i64 0
  store i64 0, ptr %10, align 1
  %11 = getelementptr i8, ptr %0, i64 8
  store { i8, [7 x i8], [24 x i8], [0 x i64] } zeroinitializer, ptr %11, align 1
  %12 = load { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] }, ptr %0, align 4
  store { i64, { i8, [7 x i8], [24 x i8], [0 x i64] }, [0 x i64] } %12, ptr %byref_arg, align 4
  %13 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3anew_x5fscoped(ptr noundef nonnull align 8 dereferenceable(40) %byref_arg)
  store { i64, i64 } %13, ptr %abi_return, align 8
  %14 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return, align 1
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %14, ptr %__bind_31_scratch, align 4
  %15 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %__bind_31_scratch, align 4
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %15, ptr %byref_arg1, align 4
  store i64 4, ptr %byref_arg2, align 4
  store i64 4, ptr %byref_arg3, align 4
  %16 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aalloc(ptr noundef nonnull align 8 dereferenceable(16) %byref_arg1, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg2, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg3)
  store i32 41, ptr %16, align 4
  %17 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %__bind_31_scratch, align 4
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %17, ptr %byref_arg1, align 4
  store i64 4, ptr %byref_arg2, align 4
  store i64 4, ptr %byref_arg3, align 4
  %18 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aalloc(ptr noundef nonnull align 8 dereferenceable(16) %byref_arg1, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg2, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg3)
  %19 = load i32, ptr %16, align 1
  store i32 %19, ptr %18, align 4
  %20 = load i32, ptr %18, align 4
  %21 = icmp eq i32 %20, 41
  %22 = zext i1 %21 to i8
  %23 = icmp ne i8 %22, 0
  %24 = icmp ne i8 %22, 0
  br i1 %24, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  br i1 true, label %check_ok, label %check_fail

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else, %op_ok
  %25 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %__bind_31_scratch, align 4
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %25, ptr %byref_arg1, align 4
  store i64 4, ptr %byref_arg2, align 4
  store i64 4, ptr %byref_arg3, align 4
  %26 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aalloc(ptr noundef nonnull align 8 dereferenceable(16) %byref_arg1, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg2, ptr noundef nonnull align 8 dereferenceable(8) %byref_arg3)
  store i32 43, ptr %26, align 4
  %27 = load i32, ptr %26, align 1
  store i32 %27, ptr %explicit_value, align 4
  %28 = load i32, ptr %explicit_value, align 4
  %29 = icmp eq i32 %28, 43
  %30 = zext i1 %29 to i8
  %31 = icmp ne i8 %30, 0
  %32 = icmp ne i8 %30, 0
  br i1 %32, label %if.then5, label %if.else6

check_ok:                                         ; preds = %check_fail, %if.then
  %33 = load ptr, ptr %__panic, align 8
  %34 = load i8, ptr %33, align 1
  %35 = icmp ne i8 %34, 0
  br i1 %35, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %if.then
  %36 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %36, align 1
  %37 = getelementptr i8, ptr %36, i64 4
  store i32 4, ptr %37, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %38 = load ptr, ptr %__panic, align 8
  %39 = getelementptr i8, ptr %38, i64 0
  %40 = load i8, ptr %39, align 1
  %41 = load ptr, ptr %__panic, align 8
  %42 = getelementptr i8, ptr %41, i64 4
  %43 = load i32, ptr %42, align 4
  %44 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %__bind_31_scratch)
  store { i64, i64 } %44, ptr %abi_return4, align 8
  %45 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return4, align 1
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64 71)
  %46 = load ptr, ptr %__panic, align 8
  %47 = getelementptr i8, ptr %46, i64 4
  %48 = load i32, ptr %47, align 4
  ret i32 %48

panic.cont:                                       ; preds = %check_ok
  %49 = load i32, ptr %score, align 4
  %50 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %49, i32 1)
  %51 = extractvalue { i32, i1 } %50, 0
  %52 = extractvalue { i32, i1 } %50, 1
  %53 = freeze i32 %51
  br i1 %52, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  store i32 %53, ptr %score, align 4
  br label %if.merge

op_fail:                                          ; preds = %panic.cont
  %54 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %54, align 1
  %55 = getelementptr i8, ptr %54, i64 4
  store i32 4, ptr %55, align 4
  ret i32 0

if.then5:                                         ; preds = %if.merge
  br i1 true, label %check_ok8, label %check_fail9

if.else6:                                         ; preds = %if.merge
  br label %if.merge7

if.merge7:                                        ; preds = %if.else6, %op_ok13
  %56 = load i32, ptr %score, align 4
  ret i32 %56

check_ok8:                                        ; preds = %check_fail9, %if.then5
  %57 = load ptr, ptr %__panic, align 8
  %58 = load i8, ptr %57, align 1
  %59 = icmp ne i8 %58, 0
  br i1 %59, label %panic.take10, label %panic.cont11

check_fail9:                                      ; preds = %if.then5
  %60 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %60, align 1
  %61 = getelementptr i8, ptr %60, i64 4
  store i32 4, ptr %61, align 4
  br label %check_ok8

panic.take10:                                     ; preds = %check_ok8
  %62 = load ptr, ptr %__panic, align 8
  %63 = getelementptr i8, ptr %62, i64 0
  %64 = load i8, ptr %63, align 1
  %65 = load ptr, ptr %__panic, align 8
  %66 = getelementptr i8, ptr %65, i64 4
  %67 = load i32, ptr %66, align 4
  %68 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3afree_x5funchecked(ptr noundef nonnull align 8 dereferenceable(16) %__bind_31_scratch)
  store { i64, i64 } %68, ptr %abi_return12, align 8
  %69 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return12, align 1
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64 71)
  %70 = load ptr, ptr %__panic, align 8
  %71 = getelementptr i8, ptr %70, i64 4
  %72 = load i32, ptr %71, align 4
  ret i32 %72

panic.cont11:                                     ; preds = %check_ok8
  %73 = load i32, ptr %score, align 4
  %74 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %73, i32 1)
  %75 = extractvalue { i32, i1 } %74, 0
  %76 = extractvalue { i32, i1 } %74, 1
  %77 = freeze i32 %75
  br i1 %76, label %op_fail14, label %op_ok13

op_ok13:                                          ; preds = %panic.cont11
  store i32 %77, ptr %score, align 4
  br label %if.merge7

op_fail14:                                        ; preds = %panic.cont11
  %78 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %78, align 1
  %79 = getelementptr i8, ptr %78, i64 4
  store i32 4, ptr %79, align 4
  ret i32 0
}

define i8 @ExpressionSemantics_x3a_x3alowerExprPropagateSuccessReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %value = alloca i32, align 4
  %0 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %1 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %"ExpressionSemantics_x3a_x3alowerExprPropagateSuccessReference$tmp$call_ref_tmp_381" = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3alowerExprPropagateSuccessReference$tmp$call_ref_tmp_378" = alloca i8, align 1
  %2 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %3 = icmp ne i8 %2, 0
  br i1 %3, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 10, ptr %5, align 4
  %6 = load ptr, ptr %__panic, align 8
  %7 = getelementptr i8, ptr %6, i64 4
  %8 = load i32, ptr %7, align 4
  %9 = trunc i32 %8 to i8
  ret i8 %9

poison.cont:                                      ; preds = %entry
  %10 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %10, align 1
  %11 = getelementptr i8, ptr %10, i64 4
  store i32 0, ptr %11, align 4
  %12 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %13 = icmp ne i8 %12, 0
  br i1 %13, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %14 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %14, align 1
  %15 = getelementptr i8, ptr %14, i64 4
  store i32 10, ptr %15, align 4
  %16 = load ptr, ptr %__panic, align 8
  %17 = getelementptr i8, ptr %16, i64 4
  %18 = load i32, ptr %17, align 4
  %19 = trunc i32 %18 to i8
  ret i8 %19

poison.cont2:                                     ; preds = %poison.cont
  store i8 0, ptr %"ExpressionSemantics_x3a_x3alowerExprPropagateSuccessReference$tmp$call_ref_tmp_378", align 1
  store i32 47, ptr %"ExpressionSemantics_x3a_x3alowerExprPropagateSuccessReference$tmp$call_ref_tmp_381", align 4
  %20 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %21 = icmp ne i8 %20, 0
  br i1 %21, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %22 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %22, align 1
  %23 = getelementptr i8, ptr %22, i64 4
  store i32 10, ptr %23, align 4
  %24 = load ptr, ptr %__panic, align 8
  %25 = getelementptr i8, ptr %24, i64 4
  %26 = load i32, ptr %25, align 4
  %27 = trunc i32 %26 to i8
  ret i8 %27

poison.cont4:                                     ; preds = %poison.cont2
  %28 = call { i8, [3 x i8], [4 x i8], [0 x i32] } @ExpressionSemantics_x3a_x3aeffectfulLoweringOutcome(ptr noundef nonnull align 1 dereferenceable(1) %"ExpressionSemantics_x3a_x3alowerExprPropagateSuccessReference$tmp$call_ref_tmp_378", ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3alowerExprPropagateSuccessReference$tmp$call_ref_tmp_381", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %29 = load ptr, ptr %__panic, align 8
  %30 = load i8, ptr %29, align 1
  %31 = icmp ne i8 %30, 0
  br i1 %31, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  %32 = load ptr, ptr %__panic, align 8
  %33 = getelementptr i8, ptr %32, i64 4
  %34 = load i32, ptr %33, align 4
  %35 = trunc i32 %34 to i8
  ret i8 %35

panic.cont:                                       ; preds = %poison.cont4
  %36 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %28, 0
  %37 = icmp eq i8 %36, 0
  br i1 %37, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %ifcase.case5
  store i32 %50, ptr %value, align 4
  %38 = load i32, ptr %value, align 4
  %39 = icmp eq i32 %38, 47
  %40 = zext i1 %39 to i8
  %41 = icmp ne i8 %40, 0
  %42 = zext i1 %41 to i8
  ret i8 %42

ifcase.case:                                      ; preds = %panic.cont
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %28, ptr %1, align 4
  %43 = getelementptr i8, ptr %1, i64 4
  %44 = load i8, ptr %43, align 1
  %45 = icmp ne i8 %44, 0
  %46 = zext i1 %45 to i8
  ret i8 %46

ifcase.next:                                      ; preds = %panic.cont
  %47 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %28, 0
  %48 = icmp eq i8 %47, 1
  br i1 %48, label %ifcase.case5, label %ifcase.unmatched

ifcase.case5:                                     ; preds = %ifcase.next
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %28, ptr %0, align 4
  %49 = getelementptr i8, ptr %0, i64 4
  %50 = load i32, ptr %49, align 1
  br label %ifcase.merge

ifcase.unmatched:                                 ; preds = %ifcase.next
  %51 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %51, align 1
  %52 = getelementptr i8, ptr %51, i64 4
  store i32 18, ptr %52, align 4
  %53 = load ptr, ptr %__panic, align 8
  %54 = getelementptr i8, ptr %53, i64 4
  %55 = load i32, ptr %54, align 4
  %56 = trunc i32 %55 to i8
  ret i8 %56
}

define i8 @ExpressionSemantics_x3a_x3alowerExprPropagateReturnReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %value = alloca i32, align 4
  %0 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %1 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %"ExpressionSemantics_x3a_x3alowerExprPropagateReturnReference$tmp$call_ref_tmp_401" = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3alowerExprPropagateReturnReference$tmp$call_ref_tmp_398" = alloca i8, align 1
  %2 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %3 = icmp ne i8 %2, 0
  br i1 %3, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 10, ptr %5, align 4
  %6 = load ptr, ptr %__panic, align 8
  %7 = getelementptr i8, ptr %6, i64 4
  %8 = load i32, ptr %7, align 4
  %9 = trunc i32 %8 to i8
  ret i8 %9

poison.cont:                                      ; preds = %entry
  %10 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %10, align 1
  %11 = getelementptr i8, ptr %10, i64 4
  store i32 0, ptr %11, align 4
  %12 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %13 = icmp ne i8 %12, 0
  br i1 %13, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %14 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %14, align 1
  %15 = getelementptr i8, ptr %14, i64 4
  store i32 10, ptr %15, align 4
  %16 = load ptr, ptr %__panic, align 8
  %17 = getelementptr i8, ptr %16, i64 4
  %18 = load i32, ptr %17, align 4
  %19 = trunc i32 %18 to i8
  ret i8 %19

poison.cont2:                                     ; preds = %poison.cont
  store i8 1, ptr %"ExpressionSemantics_x3a_x3alowerExprPropagateReturnReference$tmp$call_ref_tmp_398", align 1
  store i32 0, ptr %"ExpressionSemantics_x3a_x3alowerExprPropagateReturnReference$tmp$call_ref_tmp_401", align 4
  %20 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %21 = icmp ne i8 %20, 0
  br i1 %21, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %22 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %22, align 1
  %23 = getelementptr i8, ptr %22, i64 4
  store i32 10, ptr %23, align 4
  %24 = load ptr, ptr %__panic, align 8
  %25 = getelementptr i8, ptr %24, i64 4
  %26 = load i32, ptr %25, align 4
  %27 = trunc i32 %26 to i8
  ret i8 %27

poison.cont4:                                     ; preds = %poison.cont2
  %28 = call { i8, [3 x i8], [4 x i8], [0 x i32] } @ExpressionSemantics_x3a_x3aeffectfulLoweringOutcome(ptr noundef nonnull align 1 dereferenceable(1) %"ExpressionSemantics_x3a_x3alowerExprPropagateReturnReference$tmp$call_ref_tmp_398", ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3alowerExprPropagateReturnReference$tmp$call_ref_tmp_401", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %29 = load ptr, ptr %__panic, align 8
  %30 = load i8, ptr %29, align 1
  %31 = icmp ne i8 %30, 0
  br i1 %31, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  %32 = load ptr, ptr %__panic, align 8
  %33 = getelementptr i8, ptr %32, i64 4
  %34 = load i32, ptr %33, align 4
  %35 = trunc i32 %34 to i8
  ret i8 %35

panic.cont:                                       ; preds = %poison.cont4
  %36 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %28, 0
  %37 = icmp eq i8 %36, 0
  br i1 %37, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %ifcase.case5
  store i32 %45, ptr %value, align 4
  ret i8 0

ifcase.case:                                      ; preds = %panic.cont
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %28, ptr %1, align 4
  %38 = getelementptr i8, ptr %1, i64 4
  %39 = load i8, ptr %38, align 1
  %40 = icmp ne i8 %39, 0
  %41 = zext i1 %40 to i8
  ret i8 %41

ifcase.next:                                      ; preds = %panic.cont
  %42 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %28, 0
  %43 = icmp eq i8 %42, 1
  br i1 %43, label %ifcase.case5, label %ifcase.unmatched

ifcase.case5:                                     ; preds = %ifcase.next
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %28, ptr %0, align 4
  %44 = getelementptr i8, ptr %0, i64 4
  %45 = load i32, ptr %44, align 1
  br label %ifcase.merge

ifcase.unmatched:                                 ; preds = %ifcase.next
  %46 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %46, align 1
  %47 = getelementptr i8, ptr %46, i64 4
  store i32 18, ptr %47, align 4
  %48 = load ptr, ptr %__panic, align 8
  %49 = getelementptr i8, ptr %48, i64 4
  %50 = load i32, ptr %49, align 4
  %51 = trunc i32 %50 to i8
  ret i8 %51
}

define void @ExpressionSemantics_x3a_x3aasyncTryReference(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %0, ptr noundef nonnull align 1 dereferenceable(1) %should_fail, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %2 = alloca i32, align 4
  %propagated = alloca i32, align 4
  %3 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %4 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %5 = alloca i8, align 1
  %6 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %__uv_async_error_0 = alloca i8, align 1
  %7 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %8 = icmp ne i8 %7, 0
  br i1 %8, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %9 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 10, ptr %10, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %11 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %11, align 1
  %12 = getelementptr i8, ptr %11, i64 4
  store i32 0, ptr %12, align 4
  %13 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %14 = icmp ne i8 %13, 0
  br i1 %14, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %15 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %15, align 1
  %16 = getelementptr i8, ptr %15, i64 4
  store i32 10, ptr %16, align 4
  ret void

poison.cont2:                                     ; preds = %poison.cont
  %17 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %18 = icmp ne i8 %17, 0
  br i1 %18, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %19 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %19, align 1
  %20 = getelementptr i8, ptr %19, i64 4
  store i32 10, ptr %20, align 4
  ret void

poison.cont4:                                     ; preds = %poison.cont2
  %21 = call { i8, [3 x i8], [4 x i8], [0 x i32] } @ExpressionSemantics_x3a_x3aeffectfulLoweringOutcome(ptr noundef nonnull align 1 dereferenceable(1) %should_fail, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %22 = load ptr, ptr %__panic, align 8
  %23 = load i8, ptr %22, align 1
  %24 = icmp ne i8 %23, 0
  br i1 %24, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  ret void

panic.cont:                                       ; preds = %poison.cont4
  %25 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %21, 0
  %26 = icmp eq i8 %25, 0
  br i1 %26, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %ifcase.case5
  store i32 %37, ptr %propagated, align 4
  br i1 true, label %check_ok, label %check_fail

ifcase.case:                                      ; preds = %panic.cont
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %21, ptr %6, align 4
  %27 = getelementptr i8, ptr %6, i64 4
  %28 = load i8, ptr %27, align 1
  %29 = icmp ne i8 %28, 0
  %30 = zext i1 %29 to i8
  store i8 %30, ptr %__uv_async_error_0, align 1
  %31 = load i8, ptr %__uv_async_error_0, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %4, align 4
  store i8 2, ptr %4, align 1
  %32 = getelementptr i8, ptr %4, i64 8
  store i8 %31, ptr %5, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %32, ptr align 1 %5, i64 1, i1 false)
  %33 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %4, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %33, ptr %0, align 4
  ret void

ifcase.next:                                      ; preds = %panic.cont
  %34 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %21, 0
  %35 = icmp eq i8 %34, 1
  br i1 %35, label %ifcase.case5, label %ifcase.unmatched

ifcase.case5:                                     ; preds = %ifcase.next
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %21, ptr %3, align 4
  %36 = getelementptr i8, ptr %3, i64 4
  %37 = load i32, ptr %36, align 1
  br label %ifcase.merge

ifcase.unmatched:                                 ; preds = %ifcase.next
  %38 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %38, align 1
  %39 = getelementptr i8, ptr %38, i64 4
  store i32 18, ptr %39, align 4
  ret void

check_ok:                                         ; preds = %check_fail, %ifcase.merge
  %40 = load ptr, ptr %__panic, align 8
  %41 = load i8, ptr %40, align 1
  %42 = icmp ne i8 %41, 0
  br i1 %42, label %panic.take6, label %panic.cont7

check_fail:                                       ; preds = %ifcase.merge
  %43 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %43, align 1
  %44 = getelementptr i8, ptr %43, i64 4
  store i32 4, ptr %44, align 4
  br label %check_ok

panic.take6:                                      ; preds = %check_ok
  %45 = load ptr, ptr %__panic, align 8
  %46 = getelementptr i8, ptr %45, i64 0
  %47 = load i8, ptr %46, align 1
  %48 = load ptr, ptr %__panic, align 8
  %49 = getelementptr i8, ptr %48, i64 4
  %50 = load i32, ptr %49, align 4
  ret void

panic.cont7:                                      ; preds = %check_ok
  %51 = load i32, ptr %propagated, align 4
  %52 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %51, i32 1)
  %53 = extractvalue { i32, i1 } %52, 0
  %54 = extractvalue { i32, i1 } %52, 1
  %55 = freeze i32 %53
  br i1 %54, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont7
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %56 = getelementptr i8, ptr %1, i64 8
  store i32 %55, ptr %2, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %56, ptr align 1 %2, i64 4, i1 false)
  %57 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %57, ptr %0, align 4
  ret void

op_fail:                                          ; preds = %panic.cont7
  %58 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %58, align 1
  %59 = getelementptr i8, ptr %58, i64 4
  store i32 4, ptr %59, align 4
  ret void
}

define internal void @"ExpressionSemantics_x3a_x3aasyncTryReference$resume"(ptr %__uv_host_env, ptr %__uv_async_out, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_frame, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_input, ptr %__panic) {
entry:
  %0 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %1 = alloca i32, align 4
  %propagated = alloca i32, align 4
  %2 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %3 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %4 = alloca i8, align 1
  %5 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %__uv_async_error_0 = alloca i8, align 1
  %__bind_41_value.resume_prelude = alloca i32, align 4
  %__bind_40_should_fail.resume_prelude = alloca i8, align 1
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
  %6 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %7 = icmp ne i8 %6, 0
  br i1 %7, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 10, ptr %9, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %10 = load ptr, ptr %__panic5, align 8
  store i8 0, ptr %10, align 1
  %11 = getelementptr i8, ptr %10, i64 4
  store i32 0, ptr %11, align 4
  %12 = load ptr, ptr %__uv_async_frame3, align 8
  %13 = load ptr, ptr %__uv_async_input4, align 8
  %14 = getelementptr i8, ptr %12, i64 0
  %15 = load i64, ptr %14, align 4
  switch i64 %15, label %async.resume.invalid [
  ]

async.resume.invalid:                             ; preds = %poison.cont
  ret void

async.resume.start:                               ; No predecessors!
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %17 = icmp ne i8 %16, 0
  br i1 %17, label %poison.take6, label %poison.cont7

poison.take6:                                     ; preds = %async.resume.start
  %18 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %18, align 1
  %19 = getelementptr i8, ptr %18, i64 4
  store i32 10, ptr %19, align 4
  ret void

poison.cont7:                                     ; preds = %async.resume.start
  store i8 0, ptr %__bind_40_should_fail.resume_prelude, align 1
  store i32 0, ptr %__bind_41_value.resume_prelude, align 4
  %20 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %21 = icmp ne i8 %20, 0
  br i1 %21, label %poison.take8, label %poison.cont9

poison.take8:                                     ; preds = %poison.cont7
  %22 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %22, align 1
  %23 = getelementptr i8, ptr %22, i64 4
  store i32 10, ptr %23, align 4
  ret void

poison.cont9:                                     ; preds = %poison.cont7
  %24 = call { i8, [3 x i8], [4 x i8], [0 x i32] } @ExpressionSemantics_x3a_x3aeffectfulLoweringOutcome(ptr noundef nonnull align 1 dereferenceable(1) %__bind_40_should_fail.resume_prelude, ptr noundef nonnull align 4 dereferenceable(4) %__bind_41_value.resume_prelude, ptr noundef nonnull align 8 dereferenceable(8) %__panic5)
  %25 = load ptr, ptr %__panic5, align 8
  %26 = load i8, ptr %25, align 1
  %27 = icmp ne i8 %26, 0
  br i1 %27, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont9
  ret void

panic.cont:                                       ; preds = %poison.cont9
  %28 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %24, 0
  %29 = icmp eq i8 %28, 0
  br i1 %29, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %ifcase.case10
  store i32 %42, ptr %propagated, align 4
  br i1 true, label %check_ok, label %check_fail

ifcase.case:                                      ; preds = %panic.cont
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %24, ptr %5, align 4
  %30 = getelementptr i8, ptr %5, i64 4
  %31 = load i8, ptr %30, align 1
  %32 = icmp ne i8 %31, 0
  %33 = zext i1 %32 to i8
  store i8 %33, ptr %__uv_async_error_0, align 1
  %34 = load i8, ptr %__uv_async_error_0, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %3, align 4
  store i8 2, ptr %3, align 1
  %35 = getelementptr i8, ptr %3, i64 8
  store i8 %34, ptr %4, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %35, ptr align 1 %4, i64 1, i1 false)
  %36 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %3, align 4
  %37 = load ptr, ptr %__uv_async_out2, align 8
  %38 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %36, ptr %38, align 4
  ret void

ifcase.next:                                      ; preds = %panic.cont
  %39 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %24, 0
  %40 = icmp eq i8 %39, 1
  br i1 %40, label %ifcase.case10, label %ifcase.unmatched

ifcase.case10:                                    ; preds = %ifcase.next
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %24, ptr %2, align 4
  %41 = getelementptr i8, ptr %2, i64 4
  %42 = load i32, ptr %41, align 1
  br label %ifcase.merge

ifcase.unmatched:                                 ; preds = %ifcase.next
  %43 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %43, align 1
  %44 = getelementptr i8, ptr %43, i64 4
  store i32 18, ptr %44, align 4
  ret void

check_ok:                                         ; preds = %check_fail, %ifcase.merge
  %45 = load ptr, ptr %__panic5, align 8
  %46 = load i8, ptr %45, align 1
  %47 = icmp ne i8 %46, 0
  br i1 %47, label %panic.take11, label %panic.cont12

check_fail:                                       ; preds = %ifcase.merge
  %48 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %48, align 1
  %49 = getelementptr i8, ptr %48, i64 4
  store i32 4, ptr %49, align 4
  br label %check_ok

panic.take11:                                     ; preds = %check_ok
  %50 = load ptr, ptr %__panic5, align 8
  %51 = getelementptr i8, ptr %50, i64 0
  %52 = load i8, ptr %51, align 1
  %53 = load ptr, ptr %__panic5, align 8
  %54 = getelementptr i8, ptr %53, i64 4
  %55 = load i32, ptr %54, align 4
  ret void

panic.cont12:                                     ; preds = %check_ok
  %56 = load i32, ptr %propagated, align 4
  %57 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %56, i32 1)
  %58 = extractvalue { i32, i1 } %57, 0
  %59 = extractvalue { i32, i1 } %57, 1
  %60 = freeze i32 %58
  br i1 %59, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont12
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %61 = getelementptr i8, ptr %0, i64 8
  store i32 %60, ptr %1, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %61, ptr align 1 %1, i64 4, i1 false)
  %62 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %0, align 4
  %63 = load ptr, ptr %__uv_async_out2, align 8
  %64 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %62, ptr %64, align 4
  ret void

op_fail:                                          ; preds = %panic.cont12
  %65 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %65, align 1
  %66 = getelementptr i8, ptr %65, i64 4
  store i32 4, ptr %66, align 4
  ret void
}

define void @ExpressionSemantics_x3a_x3aasyncIteratorLoopReference(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %0, ptr noundef nonnull align 8 dereferenceable(24) %stream, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %coerce_bits = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %sret = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %copied_value = alloca i32, align 4
  %__bind_45_value.iter = alloca i32, align 4
  %iter.async.slot = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %2 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %3 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %9 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %stream, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %9, ptr %iter.async.slot, align 4
  %10 = load ptr, ptr %__panic, align 8
  br label %loop.cond

loop.end:                                         ; preds = %loop.resume, %loop.cond, %loop.cond
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %11 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %11, ptr %0, align 4
  ret void

loop.cond:                                        ; preds = %loop.resume, %poison.cont
  %12 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %iter.async.slot, align 4
  %13 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %12, 0
  switch i8 %13, label %loop.end [
    i8 0, label %loop.body
    i8 1, label %loop.end
  ]

loop.body:                                        ; preds = %loop.cond
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %12, ptr %2, align 4
  %14 = getelementptr i8, ptr %2, i64 8
  %15 = load i32, ptr %14, align 1
  store i32 %15, ptr %__bind_45_value.iter, align 4
  %16 = load i32, ptr %__bind_45_value.iter, align 1
  store i32 %16, ptr %copied_value, align 4
  br label %loop.resume

loop.resume:                                      ; preds = %loop.body
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aresume(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8] }) align 1 dereferenceable(24) %sret, ptr %iter.async.slot, ptr null, ptr %10)
  %17 = load { i8, [7 x i8], [16 x i8] }, ptr %sret, align 1
  store { i8, [7 x i8], [16 x i8] } %17, ptr %coerce_bits, align 1
  %18 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %coerce_bits, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %18, ptr %iter.async.slot, align 4
  %19 = load i8, ptr %10, align 1
  %20 = icmp ne i8 %19, 0
  br i1 %20, label %loop.end, label %loop.cond
}

define internal void @"ExpressionSemantics_x3a_x3aasyncIteratorLoopReference$resume"(ptr %__uv_host_env, ptr %__uv_async_out, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_frame, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_input, ptr %__panic) {
entry:
  %0 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %coerce_bits = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %sret = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %copied_value = alloca i32, align 4
  %__bind_45_value.iter = alloca i32, align 4
  %iter.async.slot = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
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
  %2 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %iter.async.slot, align 4
  %12 = load ptr, ptr %__panic5, align 8
  br label %loop.cond

loop.end:                                         ; preds = %loop.resume, %loop.cond, %loop.cond
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %13 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %0, align 4
  %14 = load ptr, ptr %__uv_async_out2, align 8
  %15 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %13, ptr %15, align 4
  ret void

loop.cond:                                        ; preds = %loop.resume, %async.resume.start
  %16 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %iter.async.slot, align 4
  %17 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %16, 0
  switch i8 %17, label %loop.end [
    i8 0, label %loop.body
    i8 1, label %loop.end
  ]

loop.body:                                        ; preds = %loop.cond
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %16, ptr %1, align 4
  %18 = getelementptr i8, ptr %1, i64 8
  %19 = load i32, ptr %18, align 1
  store i32 %19, ptr %__bind_45_value.iter, align 4
  %20 = load i32, ptr %__bind_45_value.iter, align 1
  store i32 %20, ptr %copied_value, align 4
  br label %loop.resume

loop.resume:                                      ; preds = %loop.body
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aresume(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8] }) align 1 dereferenceable(24) %sret, ptr %iter.async.slot, ptr null, ptr %12)
  %21 = load { i8, [7 x i8], [16 x i8] }, ptr %sret, align 1
  store { i8, [7 x i8], [16 x i8] } %21, ptr %coerce_bits, align 1
  %22 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %coerce_bits, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %22, ptr %iter.async.slot, align 4
  %23 = load i8, ptr %12, align 1
  %24 = icmp ne i8 %23, 0
  br i1 %24, label %loop.end, label %loop.cond
}

define hidden void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactCompletes(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %0, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %2 = alloca i32, align 4
  %3 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %9 = load i32, ptr %value, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %10 = getelementptr i8, ptr %1, i64 8
  store i32 %9, ptr %2, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %10, ptr align 1 %2, i64 4, i1 false)
  %11 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %11, ptr %0, align 4
  ret void
}

define internal void @"ExpressionSemantics_x3a_x3aasyncCompositionArtifactCompletes$resume"(ptr %__uv_host_env, ptr %__uv_async_out, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_frame, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_input, ptr %__panic) {
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
  %2 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  store i32 0, ptr %1, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %12, ptr align 1 %1, i64 4, i1 false)
  %13 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %0, align 4
  %14 = load ptr, ptr %__uv_async_out2, align 8
  %15 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %13, ptr %15, align 4
  ret void
}

define hidden void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactMayFail(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %0, ptr noundef nonnull align 1 dereferenceable(1) %should_fail, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %2 = alloca i32, align 4
  %result = alloca i32, align 4
  %3 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %4 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %5 = alloca i8, align 1
  %6 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %__uv_async_error_0 = alloca i8, align 1
  %7 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %8 = icmp ne i8 %7, 0
  br i1 %8, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %9 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 10, ptr %10, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %11 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %11, align 1
  %12 = getelementptr i8, ptr %11, i64 4
  store i32 0, ptr %12, align 4
  %13 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %14 = icmp ne i8 %13, 0
  br i1 %14, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %15 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %15, align 1
  %16 = getelementptr i8, ptr %15, i64 4
  store i32 10, ptr %16, align 4
  ret void

poison.cont2:                                     ; preds = %poison.cont
  %17 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %18 = icmp ne i8 %17, 0
  br i1 %18, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %19 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %19, align 1
  %20 = getelementptr i8, ptr %19, i64 4
  store i32 10, ptr %20, align 4
  ret void

poison.cont4:                                     ; preds = %poison.cont2
  %21 = call { i8, [3 x i8], [4 x i8], [0 x i32] } @ExpressionSemantics_x3a_x3aeffectfulLoweringOutcome(ptr noundef nonnull align 1 dereferenceable(1) %should_fail, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %22 = load ptr, ptr %__panic, align 8
  %23 = load i8, ptr %22, align 1
  %24 = icmp ne i8 %23, 0
  br i1 %24, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  ret void

panic.cont:                                       ; preds = %poison.cont4
  %25 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %21, 0
  %26 = icmp eq i8 %25, 0
  br i1 %26, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %ifcase.case5
  store i32 %40, ptr %result, align 4
  %27 = load i32, ptr %result, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %28 = getelementptr i8, ptr %1, i64 8
  store i32 %27, ptr %2, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %28, ptr align 1 %2, i64 4, i1 false)
  %29 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %29, ptr %0, align 4
  ret void

ifcase.case:                                      ; preds = %panic.cont
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %21, ptr %6, align 4
  %30 = getelementptr i8, ptr %6, i64 4
  %31 = load i8, ptr %30, align 1
  %32 = icmp ne i8 %31, 0
  %33 = zext i1 %32 to i8
  store i8 %33, ptr %__uv_async_error_0, align 1
  %34 = load i8, ptr %__uv_async_error_0, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %4, align 4
  store i8 2, ptr %4, align 1
  %35 = getelementptr i8, ptr %4, i64 8
  store i8 %34, ptr %5, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %35, ptr align 1 %5, i64 1, i1 false)
  %36 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %4, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %36, ptr %0, align 4
  ret void

ifcase.next:                                      ; preds = %panic.cont
  %37 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %21, 0
  %38 = icmp eq i8 %37, 1
  br i1 %38, label %ifcase.case5, label %ifcase.unmatched

ifcase.case5:                                     ; preds = %ifcase.next
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %21, ptr %3, align 4
  %39 = getelementptr i8, ptr %3, i64 4
  %40 = load i32, ptr %39, align 1
  br label %ifcase.merge

ifcase.unmatched:                                 ; preds = %ifcase.next
  %41 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %41, align 1
  %42 = getelementptr i8, ptr %41, i64 4
  store i32 18, ptr %42, align 4
  ret void
}

define internal void @"ExpressionSemantics_x3a_x3aasyncCompositionArtifactMayFail$resume"(ptr %__uv_host_env, ptr %__uv_async_out, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_frame, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_input, ptr %__panic) {
entry:
  %0 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %1 = alloca i32, align 4
  %result = alloca i32, align 4
  %2 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %3 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %4 = alloca i8, align 1
  %5 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %__uv_async_error_0 = alloca i8, align 1
  %__bind_49_value.resume_prelude = alloca i32, align 4
  %__bind_48_should_fail.resume_prelude = alloca i8, align 1
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
  %6 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %7 = icmp ne i8 %6, 0
  br i1 %7, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 10, ptr %9, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %10 = load ptr, ptr %__panic5, align 8
  store i8 0, ptr %10, align 1
  %11 = getelementptr i8, ptr %10, i64 4
  store i32 0, ptr %11, align 4
  %12 = load ptr, ptr %__uv_async_frame3, align 8
  %13 = load ptr, ptr %__uv_async_input4, align 8
  %14 = getelementptr i8, ptr %12, i64 0
  %15 = load i64, ptr %14, align 4
  switch i64 %15, label %async.resume.invalid [
  ]

async.resume.invalid:                             ; preds = %poison.cont
  ret void

async.resume.start:                               ; No predecessors!
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %17 = icmp ne i8 %16, 0
  br i1 %17, label %poison.take6, label %poison.cont7

poison.take6:                                     ; preds = %async.resume.start
  %18 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %18, align 1
  %19 = getelementptr i8, ptr %18, i64 4
  store i32 10, ptr %19, align 4
  ret void

poison.cont7:                                     ; preds = %async.resume.start
  store i8 0, ptr %__bind_48_should_fail.resume_prelude, align 1
  store i32 0, ptr %__bind_49_value.resume_prelude, align 4
  %20 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %21 = icmp ne i8 %20, 0
  br i1 %21, label %poison.take8, label %poison.cont9

poison.take8:                                     ; preds = %poison.cont7
  %22 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %22, align 1
  %23 = getelementptr i8, ptr %22, i64 4
  store i32 10, ptr %23, align 4
  ret void

poison.cont9:                                     ; preds = %poison.cont7
  %24 = call { i8, [3 x i8], [4 x i8], [0 x i32] } @ExpressionSemantics_x3a_x3aeffectfulLoweringOutcome(ptr noundef nonnull align 1 dereferenceable(1) %__bind_48_should_fail.resume_prelude, ptr noundef nonnull align 4 dereferenceable(4) %__bind_49_value.resume_prelude, ptr noundef nonnull align 8 dereferenceable(8) %__panic5)
  %25 = load ptr, ptr %__panic5, align 8
  %26 = load i8, ptr %25, align 1
  %27 = icmp ne i8 %26, 0
  br i1 %27, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont9
  ret void

panic.cont:                                       ; preds = %poison.cont9
  %28 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %24, 0
  %29 = icmp eq i8 %28, 0
  br i1 %29, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %ifcase.case10
  store i32 %47, ptr %result, align 4
  %30 = load i32, ptr %result, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %31 = getelementptr i8, ptr %0, i64 8
  store i32 %30, ptr %1, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %31, ptr align 1 %1, i64 4, i1 false)
  %32 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %0, align 4
  %33 = load ptr, ptr %__uv_async_out2, align 8
  %34 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %32, ptr %34, align 4
  ret void

ifcase.case:                                      ; preds = %panic.cont
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %24, ptr %5, align 4
  %35 = getelementptr i8, ptr %5, i64 4
  %36 = load i8, ptr %35, align 1
  %37 = icmp ne i8 %36, 0
  %38 = zext i1 %37 to i8
  store i8 %38, ptr %__uv_async_error_0, align 1
  %39 = load i8, ptr %__uv_async_error_0, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %3, align 4
  store i8 2, ptr %3, align 1
  %40 = getelementptr i8, ptr %3, i64 8
  store i8 %39, ptr %4, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %40, ptr align 1 %4, i64 1, i1 false)
  %41 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %3, align 4
  %42 = load ptr, ptr %__uv_async_out2, align 8
  %43 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %41, ptr %43, align 4
  ret void

ifcase.next:                                      ; preds = %panic.cont
  %44 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %24, 0
  %45 = icmp eq i8 %44, 1
  br i1 %45, label %ifcase.case10, label %ifcase.unmatched

ifcase.case10:                                    ; preds = %ifcase.next
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %24, ptr %2, align 4
  %46 = getelementptr i8, ptr %2, i64 4
  %47 = load i32, ptr %46, align 1
  br label %ifcase.merge

ifcase.unmatched:                                 ; preds = %ifcase.next
  %48 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %48, align 1
  %49 = getelementptr i8, ptr %48, i64 4
  store i32 18, ptr %49, align 4
  ret void
}

define hidden void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactStreamMayFail(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %0, ptr noundef nonnull align 1 dereferenceable(1) %should_fail, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %2 = alloca i32, align 4
  %3 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %4 = alloca i32, align 4
  %5 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %6 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %7 = alloca i8, align 1
  %8 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %__uv_async_error_0 = alloca i8, align 1
  %__bind_55_result = alloca i32, align 4
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %10 = icmp ne i8 %9, 0
  br i1 %10, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %11 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %11, align 1
  %12 = getelementptr i8, ptr %11, i64 4
  store i32 10, ptr %12, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %13 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %13, align 1
  %14 = getelementptr i8, ptr %13, i64 4
  store i32 0, ptr %14, align 4
  %15 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %16 = icmp ne i8 %15, 0
  br i1 %16, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %17 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %17, align 1
  %18 = getelementptr i8, ptr %17, i64 4
  store i32 10, ptr %18, align 4
  ret void

poison.cont2:                                     ; preds = %poison.cont
  %19 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %20 = icmp ne i8 %19, 0
  br i1 %20, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %21 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %21, align 1
  %22 = getelementptr i8, ptr %21, i64 4
  store i32 10, ptr %22, align 4
  ret void

poison.cont4:                                     ; preds = %poison.cont2
  %23 = call { i8, [3 x i8], [4 x i8], [0 x i32] } @ExpressionSemantics_x3a_x3aeffectfulLoweringOutcome(ptr noundef nonnull align 1 dereferenceable(1) %should_fail, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %24 = load ptr, ptr %__panic, align 8
  %25 = load i8, ptr %24, align 1
  %26 = icmp ne i8 %25, 0
  br i1 %26, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  ret void

panic.cont:                                       ; preds = %poison.cont4
  %27 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %23, 0
  %28 = icmp eq i8 %27, 0
  br i1 %28, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %ifcase.case5
  store i32 %51, ptr %__bind_55_result, align 4
  %29 = load i32, ptr %__bind_55_result, align 4
  %30 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aalloc_x5fframe(i64 40, i64 8)
  %31 = getelementptr i8, ptr %30, i64 0
  store i64 0, ptr %31, align 4
  %32 = getelementptr i8, ptr %30, i64 8
  store ptr @"ExpressionSemantics_x3a_x3aasyncCompositionArtifactStreamMayFail$resume", ptr %32, align 8
  %33 = getelementptr i8, ptr %30, i64 16
  store ptr null, ptr %33, align 8
  %34 = getelementptr i8, ptr %30, i64 24
  store ptr null, ptr %34, align 8
  %35 = load i32, ptr %__bind_55_result, align 4
  %36 = getelementptr i8, ptr %30, i64 32
  store i32 %35, ptr %36, align 4
  %37 = getelementptr i8, ptr %30, i64 0
  store i64 1, ptr %37, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %3, align 4
  store i8 0, ptr %3, align 1
  %38 = getelementptr i8, ptr %3, i64 8
  store i32 %29, ptr %4, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %38, ptr align 1 %4, i64 4, i1 false)
  %39 = getelementptr i8, ptr %38, i64 8
  store ptr %30, ptr %39, align 8
  %40 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %3, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %40, ptr %0, align 4
  ret void

ifcase.case:                                      ; preds = %panic.cont
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %23, ptr %8, align 4
  %41 = getelementptr i8, ptr %8, i64 4
  %42 = load i8, ptr %41, align 1
  %43 = icmp ne i8 %42, 0
  %44 = zext i1 %43 to i8
  store i8 %44, ptr %__uv_async_error_0, align 1
  %45 = load i8, ptr %__uv_async_error_0, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %6, align 4
  store i8 2, ptr %6, align 1
  %46 = getelementptr i8, ptr %6, i64 8
  store i8 %45, ptr %7, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %46, ptr align 1 %7, i64 1, i1 false)
  %47 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %6, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %47, ptr %0, align 4
  ret void

ifcase.next:                                      ; preds = %panic.cont
  %48 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %23, 0
  %49 = icmp eq i8 %48, 1
  br i1 %49, label %ifcase.case5, label %ifcase.unmatched

ifcase.case5:                                     ; preds = %ifcase.next
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %23, ptr %5, align 4
  %50 = getelementptr i8, ptr %5, i64 4
  %51 = load i32, ptr %50, align 1
  br label %ifcase.merge

ifcase.unmatched:                                 ; preds = %ifcase.next
  %52 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %52, align 1
  %53 = getelementptr i8, ptr %52, i64 4
  store i32 18, ptr %53, align 4
  ret void

yield.cont:                                       ; No predecessors!
  %54 = load i32, ptr %__bind_55_result, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %55 = getelementptr i8, ptr %1, i64 8
  store i32 %54, ptr %2, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %55, ptr align 1 %2, i64 4, i1 false)
  %56 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %56, ptr %0, align 4
  ret void
}

define internal void @"ExpressionSemantics_x3a_x3aasyncCompositionArtifactStreamMayFail$resume"(ptr %__uv_host_env, ptr %__uv_async_out, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_frame, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_input, ptr %__panic) {
entry:
  %0 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %1 = alloca i32, align 4
  %yield_input = alloca {}, align 8
  %2 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %3 = alloca i32, align 4
  %4 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %5 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %6 = alloca i8, align 1
  %7 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %__uv_async_error_0 = alloca i8, align 1
  %__bind_53_value.resume_prelude = alloca i32, align 4
  %__bind_52_should_fail.resume_prelude = alloca i8, align 1
  %__bind_55_result = alloca i32, align 4
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
  %8 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %9 = icmp ne i8 %8, 0
  br i1 %9, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %10 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %10, align 1
  %11 = getelementptr i8, ptr %10, i64 4
  store i32 10, ptr %11, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %12 = load ptr, ptr %__panic5, align 8
  store i8 0, ptr %12, align 1
  %13 = getelementptr i8, ptr %12, i64 4
  store i32 0, ptr %13, align 4
  %14 = load ptr, ptr %__uv_async_frame3, align 8
  %15 = load ptr, ptr %__uv_async_input4, align 8
  %16 = getelementptr i8, ptr %14, i64 32
  %17 = load i32, ptr %16, align 4
  store i32 %17, ptr %__bind_55_result, align 4
  %18 = getelementptr i8, ptr %14, i64 0
  %19 = load i64, ptr %18, align 4
  switch i64 %19, label %async.resume.invalid [
    i64 1, label %yield.resume.1
  ]

async.resume.invalid:                             ; preds = %poison.cont
  ret void

async.resume.start:                               ; No predecessors!
  %20 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %21 = icmp ne i8 %20, 0
  br i1 %21, label %poison.take6, label %poison.cont7

poison.take6:                                     ; preds = %async.resume.start
  %22 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %22, align 1
  %23 = getelementptr i8, ptr %22, i64 4
  store i32 10, ptr %23, align 4
  ret void

poison.cont7:                                     ; preds = %async.resume.start
  store i8 0, ptr %__bind_52_should_fail.resume_prelude, align 1
  store i32 0, ptr %__bind_53_value.resume_prelude, align 4
  %24 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %25 = icmp ne i8 %24, 0
  br i1 %25, label %poison.take8, label %poison.cont9

poison.take8:                                     ; preds = %poison.cont7
  %26 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %26, align 1
  %27 = getelementptr i8, ptr %26, i64 4
  store i32 10, ptr %27, align 4
  ret void

poison.cont9:                                     ; preds = %poison.cont7
  %28 = call { i8, [3 x i8], [4 x i8], [0 x i32] } @ExpressionSemantics_x3a_x3aeffectfulLoweringOutcome(ptr noundef nonnull align 1 dereferenceable(1) %__bind_52_should_fail.resume_prelude, ptr noundef nonnull align 4 dereferenceable(4) %__bind_53_value.resume_prelude, ptr noundef nonnull align 8 dereferenceable(8) %__panic5)
  %29 = load ptr, ptr %__panic5, align 8
  %30 = load i8, ptr %29, align 1
  %31 = icmp ne i8 %30, 0
  br i1 %31, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont9
  ret void

panic.cont:                                       ; preds = %poison.cont9
  %32 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %28, 0
  %33 = icmp eq i8 %32, 0
  br i1 %33, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %ifcase.case10
  store i32 %54, ptr %__bind_55_result, align 4
  %34 = load i32, ptr %__bind_55_result, align 4
  %35 = load i32, ptr %__bind_55_result, align 4
  %36 = getelementptr i8, ptr %14, i64 32
  store i32 %35, ptr %36, align 4
  %37 = getelementptr i8, ptr %14, i64 0
  store i64 1, ptr %37, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %2, align 4
  store i8 0, ptr %2, align 1
  %38 = getelementptr i8, ptr %2, i64 8
  store i32 %34, ptr %3, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %38, ptr align 1 %3, i64 4, i1 false)
  %39 = getelementptr i8, ptr %38, i64 8
  store ptr %14, ptr %39, align 8
  %40 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %2, align 4
  %41 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %40, ptr %41, align 4
  ret void

ifcase.case:                                      ; preds = %panic.cont
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %28, ptr %7, align 4
  %42 = getelementptr i8, ptr %7, i64 4
  %43 = load i8, ptr %42, align 1
  %44 = icmp ne i8 %43, 0
  %45 = zext i1 %44 to i8
  store i8 %45, ptr %__uv_async_error_0, align 1
  %46 = load i8, ptr %__uv_async_error_0, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %5, align 4
  store i8 2, ptr %5, align 1
  %47 = getelementptr i8, ptr %5, i64 8
  store i8 %46, ptr %6, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %47, ptr align 1 %6, i64 1, i1 false)
  %48 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %5, align 4
  %49 = load ptr, ptr %__uv_async_out2, align 8
  %50 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %48, ptr %50, align 4
  ret void

ifcase.next:                                      ; preds = %panic.cont
  %51 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %28, 0
  %52 = icmp eq i8 %51, 1
  br i1 %52, label %ifcase.case10, label %ifcase.unmatched

ifcase.case10:                                    ; preds = %ifcase.next
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %28, ptr %4, align 4
  %53 = getelementptr i8, ptr %4, i64 4
  %54 = load i32, ptr %53, align 1
  br label %ifcase.merge

ifcase.unmatched:                                 ; preds = %ifcase.next
  %55 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %55, align 1
  %56 = getelementptr i8, ptr %55, i64 4
  store i32 18, ptr %56, align 4
  ret void

yield.cont:                                       ; preds = %yield.resume.1
  store {} zeroinitializer, ptr %yield_input, align 1
  %57 = load i32, ptr %__bind_55_result, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %58 = getelementptr i8, ptr %0, i64 8
  store i32 %57, ptr %1, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %58, ptr align 1 %1, i64 4, i1 false)
  %59 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %0, align 4
  %60 = load ptr, ptr %__uv_async_out2, align 8
  %61 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %59, ptr %61, align 4
  ret void

yield.resume.1:                                   ; preds = %poison.cont
  br label %yield.cont
}

define hidden void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactSingleOutput(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %0, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %2 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %3 = alloca i32, align 4
  %4 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %5 = icmp ne i8 %4, 0
  br i1 %5, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %6 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %6, align 1
  %7 = getelementptr i8, ptr %6, i64 4
  store i32 10, ptr %7, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  %10 = load i32, ptr %value, align 4
  %11 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aalloc_x5fframe(i64 32, i64 8)
  %12 = getelementptr i8, ptr %11, i64 0
  store i64 0, ptr %12, align 4
  %13 = getelementptr i8, ptr %11, i64 8
  store ptr @"ExpressionSemantics_x3a_x3aasyncCompositionArtifactSingleOutput$resume", ptr %13, align 8
  %14 = getelementptr i8, ptr %11, i64 16
  store ptr null, ptr %14, align 8
  %15 = getelementptr i8, ptr %11, i64 24
  store ptr null, ptr %15, align 8
  %16 = getelementptr i8, ptr %11, i64 0
  store i64 1, ptr %16, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %2, align 4
  store i8 0, ptr %2, align 1
  %17 = getelementptr i8, ptr %2, i64 8
  store i32 %10, ptr %3, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %17, ptr align 1 %3, i64 4, i1 false)
  %18 = getelementptr i8, ptr %17, i64 8
  store ptr %11, ptr %18, align 8
  %19 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %2, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %19, ptr %0, align 4
  ret void

yield.cont:                                       ; No predecessors!
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %20 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %20, ptr %0, align 4
  ret void
}

define internal void @"ExpressionSemantics_x3a_x3aasyncCompositionArtifactSingleOutput$resume"(ptr %__uv_host_env, ptr %__uv_async_out, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_frame, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_input, ptr %__panic) {
entry:
  %0 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %yield_input = alloca {}, align 8
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %2 = alloca i32, align 4
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
  %3 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %4 = icmp ne i8 %3, 0
  br i1 %4, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %5 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 10, ptr %6, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %7 = load ptr, ptr %__panic5, align 8
  store i8 0, ptr %7, align 1
  %8 = getelementptr i8, ptr %7, i64 4
  store i32 0, ptr %8, align 4
  %9 = load ptr, ptr %__uv_async_frame3, align 8
  %10 = load ptr, ptr %__uv_async_input4, align 8
  %11 = getelementptr i8, ptr %9, i64 0
  %12 = load i64, ptr %11, align 4
  switch i64 %12, label %async.resume.invalid [
    i64 1, label %yield.resume.1
  ]

async.resume.invalid:                             ; preds = %poison.cont
  ret void

async.resume.start:                               ; No predecessors!
  %13 = getelementptr i8, ptr %9, i64 0
  store i64 1, ptr %13, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 0, ptr %1, align 1
  %14 = getelementptr i8, ptr %1, i64 8
  store i32 0, ptr %2, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %14, ptr align 1 %2, i64 4, i1 false)
  %15 = getelementptr i8, ptr %14, i64 8
  store ptr %9, ptr %15, align 8
  %16 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  %17 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %16, ptr %17, align 4
  ret void

yield.cont:                                       ; preds = %yield.resume.1
  store {} zeroinitializer, ptr %yield_input, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %18 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %0, align 4
  %19 = load ptr, ptr %__uv_async_out2, align 8
  %20 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %18, ptr %20, align 4
  ret void

yield.resume.1:                                   ; preds = %poison.cont
  br label %yield.cont
}

define hidden void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactUnitSuspends(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %0, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %2 = alloca i32, align 4
  %3 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %4 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %5 = icmp ne i8 %4, 0
  br i1 %5, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %6 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %6, align 1
  %7 = getelementptr i8, ptr %6, i64 4
  store i32 10, ptr %7, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  %10 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aalloc_x5fframe(i64 40, i64 8)
  %11 = getelementptr i8, ptr %10, i64 0
  store i64 0, ptr %11, align 4
  %12 = getelementptr i8, ptr %10, i64 8
  store ptr @"ExpressionSemantics_x3a_x3aasyncCompositionArtifactUnitSuspends$resume", ptr %12, align 8
  %13 = getelementptr i8, ptr %10, i64 16
  store ptr null, ptr %13, align 8
  %14 = getelementptr i8, ptr %10, i64 24
  store ptr null, ptr %14, align 8
  %15 = load i32, ptr %value, align 4
  %16 = getelementptr i8, ptr %10, i64 32
  store i32 %15, ptr %16, align 4
  %17 = getelementptr i8, ptr %10, i64 0
  store i64 1, ptr %17, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %3, align 4
  store i8 0, ptr %3, align 1
  %18 = getelementptr i8, ptr %3, i64 8
  %19 = getelementptr i8, ptr %18, i64 8
  store ptr %10, ptr %19, align 8
  %20 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %3, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %20, ptr %0, align 4
  ret void

yield.cont:                                       ; No predecessors!
  %21 = load i32, ptr %value, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %22 = getelementptr i8, ptr %1, i64 8
  store i32 %21, ptr %2, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %22, ptr align 1 %2, i64 4, i1 false)
  %23 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %23, ptr %0, align 4
  ret void
}

define internal void @"ExpressionSemantics_x3a_x3aasyncCompositionArtifactUnitSuspends$resume"(ptr %__uv_host_env, ptr %__uv_async_out, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_frame, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_input, ptr %__panic) {
entry:
  %0 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %1 = alloca i32, align 4
  %yield_input = alloca {}, align 8
  %2 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %__bind_57_value = alloca i32, align 4
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
  %3 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %4 = icmp ne i8 %3, 0
  br i1 %4, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %5 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 10, ptr %6, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %7 = load ptr, ptr %__panic5, align 8
  store i8 0, ptr %7, align 1
  %8 = getelementptr i8, ptr %7, i64 4
  store i32 0, ptr %8, align 4
  %9 = load ptr, ptr %__uv_async_frame3, align 8
  %10 = load ptr, ptr %__uv_async_input4, align 8
  %11 = getelementptr i8, ptr %9, i64 32
  %12 = load i32, ptr %11, align 4
  store i32 %12, ptr %__bind_57_value, align 4
  %13 = getelementptr i8, ptr %9, i64 0
  %14 = load i64, ptr %13, align 4
  switch i64 %14, label %async.resume.invalid [
    i64 1, label %yield.resume.1
  ]

async.resume.invalid:                             ; preds = %poison.cont
  ret void

async.resume.start:                               ; No predecessors!
  %15 = load i32, ptr %__bind_57_value, align 4
  %16 = getelementptr i8, ptr %9, i64 32
  store i32 %15, ptr %16, align 4
  %17 = getelementptr i8, ptr %9, i64 0
  store i64 1, ptr %17, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %2, align 4
  store i8 0, ptr %2, align 1
  %18 = getelementptr i8, ptr %2, i64 8
  %19 = getelementptr i8, ptr %18, i64 8
  store ptr %9, ptr %19, align 8
  %20 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %2, align 4
  %21 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %20, ptr %21, align 4
  ret void

yield.cont:                                       ; preds = %yield.resume.1
  store {} zeroinitializer, ptr %yield_input, align 1
  %22 = load i32, ptr %__bind_57_value, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %23 = getelementptr i8, ptr %0, i64 8
  store i32 %22, ptr %1, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %23, ptr align 1 %1, i64 4, i1 false)
  %24 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %0, align 4
  %25 = load ptr, ptr %__uv_async_out2, align 8
  %26 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %24, ptr %26, align 4
  ret void

yield.resume.1:                                   ; preds = %poison.cont
  br label %yield.cont
}

define hidden void @ExpressionSemantics_x3a_x3aasyncKeyArtifactSuspends(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %0, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %2 = alloca i32, align 4
  %3 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %4 = alloca i32, align 4
  %__bind_59_resumed = alloca i32, align 4
  %5 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %6 = icmp ne i8 %5, 0
  br i1 %6, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %7 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %7, align 1
  %8 = getelementptr i8, ptr %7, i64 4
  store i32 10, ptr %8, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %9 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 0, ptr %10, align 4
  %11 = load i32, ptr %value, align 4
  %12 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aalloc_x5fframe(i64 40, i64 8)
  %13 = getelementptr i8, ptr %12, i64 0
  store i64 0, ptr %13, align 4
  %14 = getelementptr i8, ptr %12, i64 8
  store ptr @"ExpressionSemantics_x3a_x3aasyncKeyArtifactSuspends$resume", ptr %14, align 8
  %15 = getelementptr i8, ptr %12, i64 16
  store ptr null, ptr %15, align 8
  %16 = getelementptr i8, ptr %12, i64 24
  store ptr null, ptr %16, align 8
  %17 = load i32, ptr %__bind_59_resumed, align 4
  %18 = getelementptr i8, ptr %12, i64 32
  store i32 %17, ptr %18, align 4
  %19 = getelementptr i8, ptr %12, i64 0
  store i64 1, ptr %19, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %3, align 4
  store i8 0, ptr %3, align 1
  %20 = getelementptr i8, ptr %3, i64 8
  store i32 %11, ptr %4, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %20, ptr align 1 %4, i64 4, i1 false)
  %21 = getelementptr i8, ptr %20, i64 8
  store ptr %12, ptr %21, align 8
  %22 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %3, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %22, ptr %0, align 4
  ret void

yield.cont:                                       ; No predecessors!
  store i32 0, ptr %__bind_59_resumed, align 4
  %23 = load i32, ptr %__bind_59_resumed, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %24 = getelementptr i8, ptr %1, i64 8
  store i32 %23, ptr %2, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %24, ptr align 1 %2, i64 4, i1 false)
  %25 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %25, ptr %0, align 4
  ret void
}

define internal void @"ExpressionSemantics_x3a_x3aasyncKeyArtifactSuspends$resume"(ptr %__uv_host_env, ptr %__uv_async_out, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_frame, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_input, ptr %__panic) {
entry:
  %0 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %1 = alloca i32, align 4
  %yield_input = alloca i32, align 4
  %2 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %3 = alloca i32, align 4
  %__bind_59_resumed = alloca i32, align 4
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
  %4 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %5 = icmp ne i8 %4, 0
  br i1 %5, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %6 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %6, align 1
  %7 = getelementptr i8, ptr %6, i64 4
  store i32 10, ptr %7, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic5, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  %10 = load ptr, ptr %__uv_async_frame3, align 8
  %11 = load ptr, ptr %__uv_async_input4, align 8
  %12 = getelementptr i8, ptr %10, i64 32
  %13 = load i32, ptr %12, align 4
  store i32 %13, ptr %__bind_59_resumed, align 4
  %14 = getelementptr i8, ptr %10, i64 0
  %15 = load i64, ptr %14, align 4
  switch i64 %15, label %async.resume.invalid [
    i64 1, label %yield.resume.1
  ]

async.resume.invalid:                             ; preds = %poison.cont
  ret void

async.resume.start:                               ; No predecessors!
  %16 = load i32, ptr %__bind_59_resumed, align 4
  %17 = getelementptr i8, ptr %10, i64 32
  store i32 %16, ptr %17, align 4
  %18 = getelementptr i8, ptr %10, i64 0
  store i64 1, ptr %18, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %2, align 4
  store i8 0, ptr %2, align 1
  %19 = getelementptr i8, ptr %2, i64 8
  store i32 0, ptr %3, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %19, ptr align 1 %3, i64 4, i1 false)
  %20 = getelementptr i8, ptr %19, i64 8
  store ptr %10, ptr %20, align 8
  %21 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %2, align 4
  %22 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %21, ptr %22, align 4
  ret void

yield.cont:                                       ; preds = %yield.resume.1
  %23 = load i32, ptr %11, align 1
  store i32 %23, ptr %yield_input, align 4
  %24 = load i32, ptr %yield_input, align 1
  store i32 %24, ptr %__bind_59_resumed, align 4
  %25 = load i32, ptr %__bind_59_resumed, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %26 = getelementptr i8, ptr %0, i64 8
  store i32 %25, ptr %1, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %26, ptr align 1 %1, i64 4, i1 false)
  %27 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %0, align 4
  %28 = load ptr, ptr %__uv_async_out2, align 8
  %29 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %27, ptr %29, align 4
  ret void

yield.resume.1:                                   ; preds = %poison.cont
  br label %yield.cont
}

define hidden void @ExpressionSemantics_x3a_x3aasyncKeyArtifactFails(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %0, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %2 = alloca i32, align 4
  %3 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %4 = alloca i32, align 4
  %5 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %6 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %7 = alloca i8, align 1
  %8 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %__uv_async_error_0 = alloca i8, align 1
  %"ExpressionSemantics_x3a_x3aasyncKeyArtifactFails$tmp$call_ref_tmp_479" = alloca i8, align 1
  %__bind_63_result = alloca i32, align 4
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %10 = icmp ne i8 %9, 0
  br i1 %10, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %11 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %11, align 1
  %12 = getelementptr i8, ptr %11, i64 4
  store i32 10, ptr %12, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %13 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %13, align 1
  %14 = getelementptr i8, ptr %13, i64 4
  store i32 0, ptr %14, align 4
  %15 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %16 = icmp ne i8 %15, 0
  br i1 %16, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %17 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %17, align 1
  %18 = getelementptr i8, ptr %17, i64 4
  store i32 10, ptr %18, align 4
  ret void

poison.cont2:                                     ; preds = %poison.cont
  store i8 1, ptr %"ExpressionSemantics_x3a_x3aasyncKeyArtifactFails$tmp$call_ref_tmp_479", align 1
  %19 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %20 = icmp ne i8 %19, 0
  br i1 %20, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %21 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %21, align 1
  %22 = getelementptr i8, ptr %21, i64 4
  store i32 10, ptr %22, align 4
  ret void

poison.cont4:                                     ; preds = %poison.cont2
  %23 = call { i8, [3 x i8], [4 x i8], [0 x i32] } @ExpressionSemantics_x3a_x3aeffectfulLoweringOutcome(ptr noundef nonnull align 1 dereferenceable(1) %"ExpressionSemantics_x3a_x3aasyncKeyArtifactFails$tmp$call_ref_tmp_479", ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %24 = load ptr, ptr %__panic, align 8
  %25 = load i8, ptr %24, align 1
  %26 = icmp ne i8 %25, 0
  br i1 %26, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  ret void

panic.cont:                                       ; preds = %poison.cont4
  %27 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %23, 0
  %28 = icmp eq i8 %27, 0
  br i1 %28, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %ifcase.case5
  store i32 %51, ptr %__bind_63_result, align 4
  %29 = load i32, ptr %__bind_63_result, align 4
  %30 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aalloc_x5fframe(i64 40, i64 8)
  %31 = getelementptr i8, ptr %30, i64 0
  store i64 0, ptr %31, align 4
  %32 = getelementptr i8, ptr %30, i64 8
  store ptr @"ExpressionSemantics_x3a_x3aasyncKeyArtifactFails$resume", ptr %32, align 8
  %33 = getelementptr i8, ptr %30, i64 16
  store ptr null, ptr %33, align 8
  %34 = getelementptr i8, ptr %30, i64 24
  store ptr null, ptr %34, align 8
  %35 = load i32, ptr %__bind_63_result, align 4
  %36 = getelementptr i8, ptr %30, i64 32
  store i32 %35, ptr %36, align 4
  %37 = getelementptr i8, ptr %30, i64 0
  store i64 1, ptr %37, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %3, align 4
  store i8 0, ptr %3, align 1
  %38 = getelementptr i8, ptr %3, i64 8
  store i32 %29, ptr %4, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %38, ptr align 1 %4, i64 4, i1 false)
  %39 = getelementptr i8, ptr %38, i64 8
  store ptr %30, ptr %39, align 8
  %40 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %3, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %40, ptr %0, align 4
  ret void

ifcase.case:                                      ; preds = %panic.cont
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %23, ptr %8, align 4
  %41 = getelementptr i8, ptr %8, i64 4
  %42 = load i8, ptr %41, align 1
  %43 = icmp ne i8 %42, 0
  %44 = zext i1 %43 to i8
  store i8 %44, ptr %__uv_async_error_0, align 1
  %45 = load i8, ptr %__uv_async_error_0, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %6, align 4
  store i8 2, ptr %6, align 1
  %46 = getelementptr i8, ptr %6, i64 8
  store i8 %45, ptr %7, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %46, ptr align 1 %7, i64 1, i1 false)
  %47 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %6, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %47, ptr %0, align 4
  ret void

ifcase.next:                                      ; preds = %panic.cont
  %48 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %23, 0
  %49 = icmp eq i8 %48, 1
  br i1 %49, label %ifcase.case5, label %ifcase.unmatched

ifcase.case5:                                     ; preds = %ifcase.next
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %23, ptr %5, align 4
  %50 = getelementptr i8, ptr %5, i64 4
  %51 = load i32, ptr %50, align 1
  br label %ifcase.merge

ifcase.unmatched:                                 ; preds = %ifcase.next
  %52 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %52, align 1
  %53 = getelementptr i8, ptr %52, i64 4
  store i32 18, ptr %53, align 4
  ret void

yield.cont:                                       ; No predecessors!
  %54 = load i32, ptr %__bind_63_result, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %55 = getelementptr i8, ptr %1, i64 8
  store i32 %54, ptr %2, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %55, ptr align 1 %2, i64 4, i1 false)
  %56 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %56, ptr %0, align 4
  ret void
}

define internal void @"ExpressionSemantics_x3a_x3aasyncKeyArtifactFails$resume"(ptr %__uv_host_env, ptr %__uv_async_out, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_frame, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_input, ptr %__panic) {
entry:
  %0 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %1 = alloca i32, align 4
  %yield_input = alloca i32, align 4
  %2 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %3 = alloca i32, align 4
  %4 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %5 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %6 = alloca i8, align 1
  %7 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %__uv_async_error_0 = alloca i8, align 1
  %__bind_60_value.resume_prelude = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3aasyncKeyArtifactFails$tmp$call_ref_tmp_479" = alloca i8, align 1
  %__bind_63_result = alloca i32, align 4
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
  %8 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %9 = icmp ne i8 %8, 0
  br i1 %9, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %10 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %10, align 1
  %11 = getelementptr i8, ptr %10, i64 4
  store i32 10, ptr %11, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %12 = load ptr, ptr %__panic5, align 8
  store i8 0, ptr %12, align 1
  %13 = getelementptr i8, ptr %12, i64 4
  store i32 0, ptr %13, align 4
  %14 = load ptr, ptr %__uv_async_frame3, align 8
  %15 = load ptr, ptr %__uv_async_input4, align 8
  %16 = getelementptr i8, ptr %14, i64 32
  %17 = load i32, ptr %16, align 4
  store i32 %17, ptr %__bind_63_result, align 4
  %18 = getelementptr i8, ptr %14, i64 0
  %19 = load i64, ptr %18, align 4
  switch i64 %19, label %async.resume.invalid [
    i64 1, label %yield.resume.1
  ]

async.resume.invalid:                             ; preds = %poison.cont
  ret void

async.resume.start:                               ; No predecessors!
  %20 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %21 = icmp ne i8 %20, 0
  br i1 %21, label %poison.take6, label %poison.cont7

poison.take6:                                     ; preds = %async.resume.start
  %22 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %22, align 1
  %23 = getelementptr i8, ptr %22, i64 4
  store i32 10, ptr %23, align 4
  ret void

poison.cont7:                                     ; preds = %async.resume.start
  store i8 1, ptr %"ExpressionSemantics_x3a_x3aasyncKeyArtifactFails$tmp$call_ref_tmp_479", align 1
  store i32 0, ptr %__bind_60_value.resume_prelude, align 4
  %24 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %25 = icmp ne i8 %24, 0
  br i1 %25, label %poison.take8, label %poison.cont9

poison.take8:                                     ; preds = %poison.cont7
  %26 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %26, align 1
  %27 = getelementptr i8, ptr %26, i64 4
  store i32 10, ptr %27, align 4
  ret void

poison.cont9:                                     ; preds = %poison.cont7
  %28 = call { i8, [3 x i8], [4 x i8], [0 x i32] } @ExpressionSemantics_x3a_x3aeffectfulLoweringOutcome(ptr noundef nonnull align 1 dereferenceable(1) %"ExpressionSemantics_x3a_x3aasyncKeyArtifactFails$tmp$call_ref_tmp_479", ptr noundef nonnull align 4 dereferenceable(4) %__bind_60_value.resume_prelude, ptr noundef nonnull align 8 dereferenceable(8) %__panic5)
  %29 = load ptr, ptr %__panic5, align 8
  %30 = load i8, ptr %29, align 1
  %31 = icmp ne i8 %30, 0
  br i1 %31, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont9
  ret void

panic.cont:                                       ; preds = %poison.cont9
  %32 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %28, 0
  %33 = icmp eq i8 %32, 0
  br i1 %33, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %ifcase.case10
  store i32 %54, ptr %__bind_63_result, align 4
  %34 = load i32, ptr %__bind_63_result, align 4
  %35 = load i32, ptr %__bind_63_result, align 4
  %36 = getelementptr i8, ptr %14, i64 32
  store i32 %35, ptr %36, align 4
  %37 = getelementptr i8, ptr %14, i64 0
  store i64 1, ptr %37, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %2, align 4
  store i8 0, ptr %2, align 1
  %38 = getelementptr i8, ptr %2, i64 8
  store i32 %34, ptr %3, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %38, ptr align 1 %3, i64 4, i1 false)
  %39 = getelementptr i8, ptr %38, i64 8
  store ptr %14, ptr %39, align 8
  %40 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %2, align 4
  %41 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %40, ptr %41, align 4
  ret void

ifcase.case:                                      ; preds = %panic.cont
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %28, ptr %7, align 4
  %42 = getelementptr i8, ptr %7, i64 4
  %43 = load i8, ptr %42, align 1
  %44 = icmp ne i8 %43, 0
  %45 = zext i1 %44 to i8
  store i8 %45, ptr %__uv_async_error_0, align 1
  %46 = load i8, ptr %__uv_async_error_0, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %5, align 4
  store i8 2, ptr %5, align 1
  %47 = getelementptr i8, ptr %5, i64 8
  store i8 %46, ptr %6, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %47, ptr align 1 %6, i64 1, i1 false)
  %48 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %5, align 4
  %49 = load ptr, ptr %__uv_async_out2, align 8
  %50 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %48, ptr %50, align 4
  ret void

ifcase.next:                                      ; preds = %panic.cont
  %51 = extractvalue { i8, [3 x i8], [4 x i8], [0 x i32] } %28, 0
  %52 = icmp eq i8 %51, 1
  br i1 %52, label %ifcase.case10, label %ifcase.unmatched

ifcase.case10:                                    ; preds = %ifcase.next
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %28, ptr %4, align 4
  %53 = getelementptr i8, ptr %4, i64 4
  %54 = load i32, ptr %53, align 1
  br label %ifcase.merge

ifcase.unmatched:                                 ; preds = %ifcase.next
  %55 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %55, align 1
  %56 = getelementptr i8, ptr %55, i64 4
  store i32 18, ptr %56, align 4
  ret void

yield.cont:                                       ; preds = %yield.resume.1
  %57 = load i32, ptr %15, align 1
  store i32 %57, ptr %yield_input, align 4
  %58 = load i32, ptr %__bind_63_result, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %59 = getelementptr i8, ptr %0, i64 8
  store i32 %58, ptr %1, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %59, ptr align 1 %1, i64 4, i1 false)
  %60 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %0, align 4
  %61 = load ptr, ptr %__uv_async_out2, align 8
  %62 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %60, ptr %62, align 4
  ret void

yield.resume.1:                                   ; preds = %poison.cont
  br label %yield.cont
}

define { i8, [3 x i8], [4 x i8], [0 x i32] } @ExpressionSemantics_x3a_x3aasyncCompositionSyncArtifactReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %1 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %coerce_bits = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %sret5 = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %2 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %3 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %4 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %5 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %sret = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionSyncArtifactReference$tmp$call_ref_tmp_492" = alloca i32, align 4
  %6 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %7 = icmp ne i8 %6, 0
  br i1 %7, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 10, ptr %9, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

poison.cont:                                      ; preds = %entry
  %10 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %10, align 1
  %11 = getelementptr i8, ptr %10, i64 4
  store i32 0, ptr %11, align 4
  %12 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %13 = icmp ne i8 %12, 0
  br i1 %13, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %14 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %14, align 1
  %15 = getelementptr i8, ptr %14, i64 4
  store i32 10, ptr %15, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

poison.cont2:                                     ; preds = %poison.cont
  store i32 101, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionSyncArtifactReference$tmp$call_ref_tmp_492", align 4
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %17 = icmp ne i8 %16, 0
  br i1 %17, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %18 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %18, align 1
  %19 = getelementptr i8, ptr %18, i64 4
  store i32 10, ptr %19, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

poison.cont4:                                     ; preds = %poison.cont2
  call void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactUnitSuspends(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %sret, ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3aasyncCompositionSyncArtifactReference$tmp$call_ref_tmp_492", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %20 = load ptr, ptr %__panic, align 8
  %21 = load i8, ptr %20, align 1
  %22 = icmp ne i8 %21, 0
  br i1 %22, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

panic.cont:                                       ; preds = %poison.cont4
  %23 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %sret, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %23, ptr %2, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %3, align 4
  %24 = load ptr, ptr %__panic, align 8
  br label %sync.loop

sync.loop:                                        ; preds = %sync.suspended, %panic.cont
  %25 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %2, align 4
  %26 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %25, 0
  switch i8 %26, label %sync.fallback [
    i8 0, label %sync.suspended
    i8 1, label %sync.completed
    i8 2, label %sync.failed
  ]

sync.suspended:                                   ; preds = %sync.loop
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aresume(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8] }) align 1 dereferenceable(24) %sret5, ptr %2, ptr null, ptr %24)
  %27 = load { i8, [7 x i8], [16 x i8] }, ptr %sret5, align 1
  store { i8, [7 x i8], [16 x i8] } %27, ptr %coerce_bits, align 1
  %28 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %coerce_bits, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %28, ptr %2, align 4
  %29 = load i8, ptr %24, align 1
  %30 = icmp ne i8 %29, 0
  br i1 %30, label %sync.panic, label %sync.loop

sync.completed:                                   ; preds = %sync.loop
  %31 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %2, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %31, ptr %4, align 4
  %32 = getelementptr i8, ptr %4, i64 8
  %33 = load i32, ptr %32, align 1
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %1, align 4
  store i8 0, ptr %1, align 1
  %34 = getelementptr i8, ptr %1, i64 4
  %35 = getelementptr i8, ptr %34, i64 0
  store i32 %33, ptr %35, align 1
  %36 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %1, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %36, ptr %3, align 4
  br label %sync.merge

sync.failed:                                      ; preds = %sync.loop
  %37 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %2, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %37, ptr %5, align 4
  %38 = getelementptr i8, ptr %5, i64 8
  %39 = load i8, ptr %38, align 1
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %40 = getelementptr i8, ptr %0, i64 4
  %41 = getelementptr i8, ptr %40, i64 0
  store i8 %39, ptr %41, align 1
  %42 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %0, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %42, ptr %3, align 4
  br label %sync.merge

sync.fallback:                                    ; preds = %sync.loop
  %43 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %2, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %3, align 4
  br label %sync.merge

sync.panic:                                       ; preds = %sync.suspended
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %3, align 4
  br label %sync.merge

sync.merge:                                       ; preds = %sync.panic, %sync.fallback, %sync.failed, %sync.completed
  %44 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %3, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } %44
}

define { i8, [3 x i8], [8 x i8], [0 x i32] } @ExpressionSemantics_x3a_x3aasyncCompositionAllArtifactReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i8, [3 x i8], [8 x i8], [0 x i32] }, align 8
  %coerce_bits13 = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %coerce_bits = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %sret12 = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %1 = alloca { i8, [3 x i8], [8 x i8], [0 x i32] }, align 8
  %2 = alloca { i8, [3 x i8], [8 x i8], [0 x i32] }, align 8
  %3 = alloca { i8, [3 x i8], [8 x i8], [0 x i32] }, align 8
  %4 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %5 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %sret9 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionAllArtifactReference$tmp$call_ref_tmp_508" = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3aasyncCompositionAllArtifactReference$tmp$call_ref_tmp_505" = alloca i8, align 1
  %sret = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionAllArtifactReference$tmp$call_ref_tmp_501" = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3aasyncCompositionAllArtifactReference$tmp$call_ref_tmp_498" = alloca i8, align 1
  %6 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %7 = icmp ne i8 %6, 0
  br i1 %7, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 10, ptr %9, align 4
  ret { i8, [3 x i8], [8 x i8], [0 x i32] } zeroinitializer

poison.cont:                                      ; preds = %entry
  %10 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %10, align 1
  %11 = getelementptr i8, ptr %10, i64 4
  store i32 0, ptr %11, align 4
  %12 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %13 = icmp ne i8 %12, 0
  br i1 %13, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %14 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %14, align 1
  %15 = getelementptr i8, ptr %14, i64 4
  store i32 10, ptr %15, align 4
  ret { i8, [3 x i8], [8 x i8], [0 x i32] } zeroinitializer

poison.cont2:                                     ; preds = %poison.cont
  store i8 0, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionAllArtifactReference$tmp$call_ref_tmp_498", align 1
  store i32 102, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionAllArtifactReference$tmp$call_ref_tmp_501", align 4
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %17 = icmp ne i8 %16, 0
  br i1 %17, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %18 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %18, align 1
  %19 = getelementptr i8, ptr %18, i64 4
  store i32 10, ptr %19, align 4
  ret { i8, [3 x i8], [8 x i8], [0 x i32] } zeroinitializer

poison.cont4:                                     ; preds = %poison.cont2
  call void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactMayFail(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %sret, ptr noundef nonnull align 1 dereferenceable(1) %"ExpressionSemantics_x3a_x3aasyncCompositionAllArtifactReference$tmp$call_ref_tmp_498", ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3aasyncCompositionAllArtifactReference$tmp$call_ref_tmp_501", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %20 = load ptr, ptr %__panic, align 8
  %21 = load i8, ptr %20, align 1
  %22 = icmp ne i8 %21, 0
  br i1 %22, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  ret { i8, [3 x i8], [8 x i8], [0 x i32] } zeroinitializer

panic.cont:                                       ; preds = %poison.cont4
  %23 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %sret, align 4
  %24 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %25 = icmp ne i8 %24, 0
  br i1 %25, label %poison.take5, label %poison.cont6

poison.take5:                                     ; preds = %panic.cont
  %26 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %26, align 1
  %27 = getelementptr i8, ptr %26, i64 4
  store i32 10, ptr %27, align 4
  ret { i8, [3 x i8], [8 x i8], [0 x i32] } zeroinitializer

poison.cont6:                                     ; preds = %panic.cont
  store i8 0, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionAllArtifactReference$tmp$call_ref_tmp_505", align 1
  store i32 103, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionAllArtifactReference$tmp$call_ref_tmp_508", align 4
  %28 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %29 = icmp ne i8 %28, 0
  br i1 %29, label %poison.take7, label %poison.cont8

poison.take7:                                     ; preds = %poison.cont6
  %30 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %30, align 1
  %31 = getelementptr i8, ptr %30, i64 4
  store i32 10, ptr %31, align 4
  ret { i8, [3 x i8], [8 x i8], [0 x i32] } zeroinitializer

poison.cont8:                                     ; preds = %poison.cont6
  call void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactMayFail(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %sret9, ptr noundef nonnull align 1 dereferenceable(1) %"ExpressionSemantics_x3a_x3aasyncCompositionAllArtifactReference$tmp$call_ref_tmp_505", ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3aasyncCompositionAllArtifactReference$tmp$call_ref_tmp_508", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %32 = load ptr, ptr %__panic, align 8
  %33 = load i8, ptr %32, align 1
  %34 = icmp ne i8 %33, 0
  br i1 %34, label %panic.take10, label %panic.cont11

panic.take10:                                     ; preds = %poison.cont8
  ret { i8, [3 x i8], [8 x i8], [0 x i32] } zeroinitializer

panic.cont11:                                     ; preds = %poison.cont8
  %35 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %sret9, align 4
  store { i8, [3 x i8], [8 x i8], [0 x i32] } zeroinitializer, ptr %3, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %23, ptr %4, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %35, ptr %5, align 4
  %36 = load ptr, ptr %__panic, align 8
  br label %all.loop

all.loop:                                         ; preds = %all.resume.1, %all.resume.0, %panic.cont11
  br label %all.chk.failed.0

all.panic:                                        ; preds = %all.resume.1, %all.resume.0
  store { i8, [3 x i8], [8 x i8], [0 x i32] } zeroinitializer, ptr %3, align 4
  br label %all.merge

all.fallback:                                     ; preds = %all.chk.resume.2
  store { i8, [3 x i8], [8 x i8], [0 x i32] } zeroinitializer, ptr %3, align 4
  br label %all.merge

all.success:                                      ; preds = %all.chk.complete.2
  %37 = getelementptr i8, ptr %4, i64 8
  %38 = load i32, ptr %37, align 1
  %39 = insertvalue { i32, i32 } zeroinitializer, i32 %38, 0
  %40 = getelementptr i8, ptr %5, i64 8
  %41 = load i32, ptr %40, align 1
  %42 = insertvalue { i32, i32 } %39, i32 %41, 1
  store { i8, [3 x i8], [8 x i8], [0 x i32] } zeroinitializer, ptr %0, align 4
  store i8 0, ptr %0, align 1
  %43 = getelementptr i8, ptr %0, i64 4
  %44 = getelementptr i8, ptr %43, i64 0
  store { i32, i32 } %42, ptr %44, align 1
  %45 = load { i8, [3 x i8], [8 x i8], [0 x i32] }, ptr %0, align 4
  store { i8, [3 x i8], [8 x i8], [0 x i32] } %45, ptr %3, align 4
  br label %all.merge

all.merge:                                        ; preds = %all.fallback, %all.panic, %all.success, %all.failed.1, %all.failed.0
  %46 = load { i8, [3 x i8], [8 x i8], [0 x i32] }, ptr %3, align 4
  ret { i8, [3 x i8], [8 x i8], [0 x i32] } %46

all.chk.failed.0:                                 ; preds = %all.loop
  %47 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %4, align 4
  %48 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %47, 0
  %49 = icmp eq i8 %48, 2
  br i1 %49, label %all.failed.0, label %all.chk.failed.1

all.chk.complete.0:                               ; preds = %all.chk.failed.2
  %50 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %4, align 4
  %51 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %50, 0
  %52 = icmp eq i8 %51, 1
  br i1 %52, label %all.chk.complete.1, label %all.chk.resume.0

all.chk.resume.0:                                 ; preds = %all.chk.complete.1, %all.chk.complete.0
  %53 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %4, align 4
  %54 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %53, 0
  %55 = icmp eq i8 %54, 0
  br i1 %55, label %all.resume.0, label %all.chk.resume.1

all.chk.failed.1:                                 ; preds = %all.chk.failed.0
  %56 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %5, align 4
  %57 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %56, 0
  %58 = icmp eq i8 %57, 2
  br i1 %58, label %all.failed.1, label %all.chk.failed.2

all.chk.complete.1:                               ; preds = %all.chk.complete.0
  %59 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %5, align 4
  %60 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %59, 0
  %61 = icmp eq i8 %60, 1
  br i1 %61, label %all.chk.complete.2, label %all.chk.resume.0

all.chk.resume.1:                                 ; preds = %all.chk.resume.0
  %62 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %5, align 4
  %63 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %62, 0
  %64 = icmp eq i8 %63, 0
  br i1 %64, label %all.resume.1, label %all.chk.resume.2

all.chk.failed.2:                                 ; preds = %all.chk.failed.1
  br label %all.chk.complete.0

all.chk.complete.2:                               ; preds = %all.chk.complete.1
  br label %all.success

all.chk.resume.2:                                 ; preds = %all.chk.resume.1
  br label %all.fallback

all.failed.0:                                     ; preds = %all.chk.failed.0
  %65 = getelementptr i8, ptr %4, i64 8
  %66 = load i8, ptr %65, align 1
  store { i8, [3 x i8], [8 x i8], [0 x i32] } zeroinitializer, ptr %2, align 4
  store i8 1, ptr %2, align 1
  %67 = getelementptr i8, ptr %2, i64 4
  %68 = getelementptr i8, ptr %67, i64 0
  store i8 %66, ptr %68, align 1
  %69 = load { i8, [3 x i8], [8 x i8], [0 x i32] }, ptr %2, align 4
  store { i8, [3 x i8], [8 x i8], [0 x i32] } %69, ptr %3, align 4
  br label %all.merge

all.resume.0:                                     ; preds = %all.chk.resume.0
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aresume(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8] }) align 1 dereferenceable(24) %sret12, ptr %4, ptr null, ptr %36)
  %70 = load { i8, [7 x i8], [16 x i8] }, ptr %sret12, align 1
  store { i8, [7 x i8], [16 x i8] } %70, ptr %coerce_bits, align 1
  %71 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %coerce_bits, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %71, ptr %4, align 4
  %72 = load i8, ptr %36, align 1
  %73 = icmp ne i8 %72, 0
  br i1 %73, label %all.panic, label %all.loop

all.failed.1:                                     ; preds = %all.chk.failed.1
  %74 = getelementptr i8, ptr %5, i64 8
  %75 = load i8, ptr %74, align 1
  store { i8, [3 x i8], [8 x i8], [0 x i32] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %76 = getelementptr i8, ptr %1, i64 4
  %77 = getelementptr i8, ptr %76, i64 0
  store i8 %75, ptr %77, align 1
  %78 = load { i8, [3 x i8], [8 x i8], [0 x i32] }, ptr %1, align 4
  store { i8, [3 x i8], [8 x i8], [0 x i32] } %78, ptr %3, align 4
  br label %all.merge

all.resume.1:                                     ; preds = %all.chk.resume.1
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aresume(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8] }) align 1 dereferenceable(24) %sret12, ptr %5, ptr null, ptr %36)
  %79 = load { i8, [7 x i8], [16 x i8] }, ptr %sret12, align 1
  store { i8, [7 x i8], [16 x i8] } %79, ptr %coerce_bits13, align 1
  %80 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %coerce_bits13, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %80, ptr %5, align 4
  %81 = load i8, ptr %36, align 1
  %82 = icmp ne i8 %81, 0
  br i1 %82, label %all.panic, label %all.loop
}

define { i8, [3 x i8], [4 x i8], [0 x i32] } @ExpressionSemantics_x3a_x3aasyncCompositionRaceReturnArtifactReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %coerce_bits22 = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %coerce_bits = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %sret21 = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %0 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %1 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %2 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %value14 = alloca i32, align 4
  %3 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %value = alloca i32, align 4
  %sret9 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionRaceReturnArtifactReference$tmp$call_ref_tmp_535" = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3aasyncCompositionRaceReturnArtifactReference$tmp$call_ref_tmp_532" = alloca i8, align 1
  %sret = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionRaceReturnArtifactReference$tmp$call_ref_tmp_517" = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3aasyncCompositionRaceReturnArtifactReference$tmp$call_ref_tmp_514" = alloca i8, align 1
  %4 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %5 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %6 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %7 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %8 = icmp ne i8 %7, 0
  br i1 %8, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %9 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 10, ptr %10, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

poison.cont:                                      ; preds = %entry
  %11 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %11, align 1
  %12 = getelementptr i8, ptr %11, i64 4
  store i32 0, ptr %12, align 4
  %13 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %14 = icmp ne i8 %13, 0
  br i1 %14, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %15 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %15, align 1
  %16 = getelementptr i8, ptr %15, i64 4
  store i32 10, ptr %16, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

poison.cont2:                                     ; preds = %poison.cont
  store i8 0, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionRaceReturnArtifactReference$tmp$call_ref_tmp_514", align 1
  store i32 104, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionRaceReturnArtifactReference$tmp$call_ref_tmp_517", align 4
  %17 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %18 = icmp ne i8 %17, 0
  br i1 %18, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %19 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %19, align 1
  %20 = getelementptr i8, ptr %19, i64 4
  store i32 10, ptr %20, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

poison.cont4:                                     ; preds = %poison.cont2
  call void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactMayFail(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %sret, ptr noundef nonnull align 1 dereferenceable(1) %"ExpressionSemantics_x3a_x3aasyncCompositionRaceReturnArtifactReference$tmp$call_ref_tmp_514", ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3aasyncCompositionRaceReturnArtifactReference$tmp$call_ref_tmp_517", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %21 = load ptr, ptr %__panic, align 8
  %22 = load i8, ptr %21, align 1
  %23 = icmp ne i8 %22, 0
  br i1 %23, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

panic.cont:                                       ; preds = %poison.cont4
  %24 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %sret, align 4
  %25 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %26 = icmp ne i8 %25, 0
  br i1 %26, label %poison.take5, label %poison.cont6

poison.take5:                                     ; preds = %panic.cont
  %27 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %27, align 1
  %28 = getelementptr i8, ptr %27, i64 4
  store i32 10, ptr %28, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

poison.cont6:                                     ; preds = %panic.cont
  store i8 0, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionRaceReturnArtifactReference$tmp$call_ref_tmp_532", align 1
  store i32 105, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionRaceReturnArtifactReference$tmp$call_ref_tmp_535", align 4
  %29 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %30 = icmp ne i8 %29, 0
  br i1 %30, label %poison.take7, label %poison.cont8

poison.take7:                                     ; preds = %poison.cont6
  %31 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %31, align 1
  %32 = getelementptr i8, ptr %31, i64 4
  store i32 10, ptr %32, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

poison.cont8:                                     ; preds = %poison.cont6
  call void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactMayFail(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %sret9, ptr noundef nonnull align 1 dereferenceable(1) %"ExpressionSemantics_x3a_x3aasyncCompositionRaceReturnArtifactReference$tmp$call_ref_tmp_532", ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3aasyncCompositionRaceReturnArtifactReference$tmp$call_ref_tmp_535", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %33 = load ptr, ptr %__panic, align 8
  %34 = load i8, ptr %33, align 1
  %35 = icmp ne i8 %34, 0
  br i1 %35, label %panic.take10, label %panic.cont11

panic.take10:                                     ; preds = %poison.cont8
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

panic.cont11:                                     ; preds = %poison.cont8
  %36 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %sret9, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %4, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %24, ptr %5, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %36, ptr %6, align 4
  %37 = load ptr, ptr %__panic, align 8
  br label %race.return.loop

race.return.loop:                                 ; preds = %race.return.resume.1, %race.return.resume.0, %panic.cont11
  br label %race.return.chk.completed.0

race.return.fallback:                             ; preds = %race.return.chk.resume.2
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %4, align 4
  br label %race.return.merge

race.return.panic:                                ; preds = %race.return.resume.1, %race.return.resume.0
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %4, align 4
  br label %race.return.merge

race.return.merge:                                ; preds = %race.return.panic, %race.return.fallback, %race.return.failed.1, %race.return.failed.0, %op_ok19, %op_ok
  %38 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %4, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } %38

race.return.chk.completed.0:                      ; preds = %race.return.loop
  %39 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %5, align 4
  %40 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %39, 0
  %41 = icmp eq i8 %40, 1
  br i1 %41, label %race.return.completed.0, label %race.return.chk.completed.1

race.return.chk.failed.0:                         ; preds = %race.return.chk.completed.2
  %42 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %5, align 4
  %43 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %42, 0
  %44 = icmp eq i8 %43, 2
  br i1 %44, label %race.return.failed.0, label %race.return.chk.failed.1

race.return.chk.resume.0:                         ; preds = %race.return.chk.failed.2
  %45 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %5, align 4
  %46 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %45, 0
  %47 = icmp eq i8 %46, 0
  br i1 %47, label %race.return.resume.0, label %race.return.chk.resume.1

race.return.chk.completed.1:                      ; preds = %race.return.chk.completed.0
  %48 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %6, align 4
  %49 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %48, 0
  %50 = icmp eq i8 %49, 1
  br i1 %50, label %race.return.completed.1, label %race.return.chk.completed.2

race.return.chk.failed.1:                         ; preds = %race.return.chk.failed.0
  %51 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %6, align 4
  %52 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %51, 0
  %53 = icmp eq i8 %52, 2
  br i1 %53, label %race.return.failed.1, label %race.return.chk.failed.2

race.return.chk.resume.1:                         ; preds = %race.return.chk.resume.0
  %54 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %6, align 4
  %55 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %54, 0
  %56 = icmp eq i8 %55, 0
  br i1 %56, label %race.return.resume.1, label %race.return.chk.resume.2

race.return.chk.completed.2:                      ; preds = %race.return.chk.completed.1
  br label %race.return.chk.failed.0

race.return.chk.failed.2:                         ; preds = %race.return.chk.failed.1
  br label %race.return.chk.resume.0

race.return.chk.resume.2:                         ; preds = %race.return.chk.resume.1
  br label %race.return.fallback

race.return.completed.0:                          ; preds = %race.return.chk.completed.0
  %57 = getelementptr i8, ptr %5, i64 8
  %58 = load i32, ptr %57, align 1
  store i32 %58, ptr %value, align 4
  br i1 true, label %check_ok, label %check_fail

race.return.failed.0:                             ; preds = %race.return.chk.failed.0
  %59 = getelementptr i8, ptr %5, i64 8
  %60 = load i8, ptr %59, align 1
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %61 = getelementptr i8, ptr %1, i64 4
  %62 = getelementptr i8, ptr %61, i64 0
  store i8 %60, ptr %62, align 1
  %63 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %1, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %63, ptr %4, align 4
  br label %race.return.merge

race.return.resume.0:                             ; preds = %race.return.chk.resume.0
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aresume(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8] }) align 1 dereferenceable(24) %sret21, ptr %5, ptr null, ptr %37)
  %64 = load { i8, [7 x i8], [16 x i8] }, ptr %sret21, align 1
  store { i8, [7 x i8], [16 x i8] } %64, ptr %coerce_bits, align 1
  %65 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %coerce_bits, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %65, ptr %5, align 4
  %66 = load i8, ptr %37, align 1
  %67 = icmp ne i8 %66, 0
  br i1 %67, label %race.return.panic, label %race.return.loop

race.return.completed.1:                          ; preds = %race.return.chk.completed.1
  %68 = getelementptr i8, ptr %6, i64 8
  %69 = load i32, ptr %68, align 1
  store i32 %69, ptr %value14, align 4
  br i1 true, label %check_ok15, label %check_fail16

race.return.failed.1:                             ; preds = %race.return.chk.failed.1
  %70 = getelementptr i8, ptr %6, i64 8
  %71 = load i8, ptr %70, align 1
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %72 = getelementptr i8, ptr %0, i64 4
  %73 = getelementptr i8, ptr %72, i64 0
  store i8 %71, ptr %73, align 1
  %74 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %0, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %74, ptr %4, align 4
  br label %race.return.merge

race.return.resume.1:                             ; preds = %race.return.chk.resume.1
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aresume(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8] }) align 1 dereferenceable(24) %sret21, ptr %6, ptr null, ptr %37)
  %75 = load { i8, [7 x i8], [16 x i8] }, ptr %sret21, align 1
  store { i8, [7 x i8], [16 x i8] } %75, ptr %coerce_bits22, align 1
  %76 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %coerce_bits22, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %76, ptr %6, align 4
  %77 = load i8, ptr %37, align 1
  %78 = icmp ne i8 %77, 0
  br i1 %78, label %race.return.panic, label %race.return.loop

check_ok:                                         ; preds = %check_fail, %race.return.completed.0
  %79 = load ptr, ptr %__panic, align 8
  %80 = load i8, ptr %79, align 1
  %81 = icmp ne i8 %80, 0
  br i1 %81, label %panic.take12, label %panic.cont13

check_fail:                                       ; preds = %race.return.completed.0
  %82 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %82, align 1
  %83 = getelementptr i8, ptr %82, i64 4
  store i32 4, ptr %83, align 4
  br label %check_ok

panic.take12:                                     ; preds = %check_ok
  %84 = load ptr, ptr %__panic, align 8
  %85 = getelementptr i8, ptr %84, i64 0
  %86 = load i8, ptr %85, align 1
  %87 = load ptr, ptr %__panic, align 8
  %88 = getelementptr i8, ptr %87, i64 4
  %89 = load i32, ptr %88, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

panic.cont13:                                     ; preds = %check_ok
  %90 = load i32, ptr %value, align 4
  %91 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %90, i32 1)
  %92 = extractvalue { i32, i1 } %91, 0
  %93 = extractvalue { i32, i1 } %91, 1
  %94 = freeze i32 %92
  br i1 %93, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont13
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %3, align 4
  store i8 0, ptr %3, align 1
  %95 = getelementptr i8, ptr %3, i64 4
  %96 = getelementptr i8, ptr %95, i64 0
  store i32 %94, ptr %96, align 1
  %97 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %3, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %97, ptr %4, align 4
  br label %race.return.merge

op_fail:                                          ; preds = %panic.cont13
  %98 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %98, align 1
  %99 = getelementptr i8, ptr %98, i64 4
  store i32 4, ptr %99, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

check_ok15:                                       ; preds = %check_fail16, %race.return.completed.1
  %100 = load ptr, ptr %__panic, align 8
  %101 = load i8, ptr %100, align 1
  %102 = icmp ne i8 %101, 0
  br i1 %102, label %panic.take17, label %panic.cont18

check_fail16:                                     ; preds = %race.return.completed.1
  %103 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %103, align 1
  %104 = getelementptr i8, ptr %103, i64 4
  store i32 4, ptr %104, align 4
  br label %check_ok15

panic.take17:                                     ; preds = %check_ok15
  %105 = load ptr, ptr %__panic, align 8
  %106 = getelementptr i8, ptr %105, i64 0
  %107 = load i8, ptr %106, align 1
  %108 = load ptr, ptr %__panic, align 8
  %109 = getelementptr i8, ptr %108, i64 4
  %110 = load i32, ptr %109, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

panic.cont18:                                     ; preds = %check_ok15
  %111 = load i32, ptr %value14, align 4
  %112 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %111, i32 1)
  %113 = extractvalue { i32, i1 } %112, 0
  %114 = extractvalue { i32, i1 } %112, 1
  %115 = freeze i32 %113
  br i1 %114, label %op_fail20, label %op_ok19

op_ok19:                                          ; preds = %panic.cont18
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %2, align 4
  store i8 0, ptr %2, align 1
  %116 = getelementptr i8, ptr %2, i64 4
  %117 = getelementptr i8, ptr %116, i64 0
  store i32 %115, ptr %117, align 1
  %118 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %2, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %118, ptr %4, align 4
  br label %race.return.merge

op_fail20:                                        ; preds = %panic.cont18
  %119 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %119, align 1
  %120 = getelementptr i8, ptr %119, i64 4
  store i32 4, ptr %120, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer
}

define void @ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference(ptr noalias noundef nonnull sret({ i8, [7 x i8], [24 x i8], [0 x i64] }) align 8 dereferenceable(32) %0, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %1 = alloca { i8, [7 x i8], [24 x i8], [0 x i64] }, align 8
  %2 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %3 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %4 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %5 = alloca i8, align 1
  %6 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %7 = alloca i8, align 1
  %output12 = alloca i32, align 4
  %output = alloca i32, align 4
  %sret9 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_566" = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_563" = alloca i8, align 1
  %sret = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_555" = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_552" = alloca i8, align 1
  %8 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %9 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %10 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %11 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %12 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %13 = alloca i32, align 4
  %14 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %15 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %16 = alloca i32, align 4
  %17 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %18 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %19 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %20 = icmp ne i8 %19, 0
  br i1 %20, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %21 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %21, align 1
  %22 = getelementptr i8, ptr %21, i64 4
  store i32 10, ptr %22, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %23 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %23, align 1
  %24 = getelementptr i8, ptr %23, i64 4
  store i32 0, ptr %24, align 4
  %25 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %26 = icmp ne i8 %25, 0
  br i1 %26, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %27 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %27, align 1
  %28 = getelementptr i8, ptr %27, i64 4
  store i32 10, ptr %28, align 4
  ret void

poison.cont2:                                     ; preds = %poison.cont
  store i8 0, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_552", align 1
  store i32 106, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_555", align 4
  %29 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %30 = icmp ne i8 %29, 0
  br i1 %30, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %31 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %31, align 1
  %32 = getelementptr i8, ptr %31, i64 4
  store i32 10, ptr %32, align 4
  ret void

poison.cont4:                                     ; preds = %poison.cont2
  call void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactStreamMayFail(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %sret, ptr noundef nonnull align 1 dereferenceable(1) %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_552", ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_555", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %33 = load ptr, ptr %__panic, align 8
  %34 = load i8, ptr %33, align 1
  %35 = icmp ne i8 %34, 0
  br i1 %35, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  ret void

panic.cont:                                       ; preds = %poison.cont4
  %36 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %sret, align 4
  %37 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %38 = icmp ne i8 %37, 0
  br i1 %38, label %poison.take5, label %poison.cont6

poison.take5:                                     ; preds = %panic.cont
  %39 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %39, align 1
  %40 = getelementptr i8, ptr %39, i64 4
  store i32 10, ptr %40, align 4
  ret void

poison.cont6:                                     ; preds = %panic.cont
  store i8 0, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_563", align 1
  store i32 107, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_566", align 4
  %41 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %42 = icmp ne i8 %41, 0
  br i1 %42, label %poison.take7, label %poison.cont8

poison.take7:                                     ; preds = %poison.cont6
  %43 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %43, align 1
  %44 = getelementptr i8, ptr %43, i64 4
  store i32 10, ptr %44, align 4
  ret void

poison.cont8:                                     ; preds = %poison.cont6
  call void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactStreamMayFail(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %sret9, ptr noundef nonnull align 1 dereferenceable(1) %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_563", ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_566", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %45 = load ptr, ptr %__panic, align 8
  %46 = load i8, ptr %45, align 1
  %47 = icmp ne i8 %46, 0
  br i1 %47, label %panic.take10, label %panic.cont11

panic.take10:                                     ; preds = %poison.cont8
  ret void

panic.cont11:                                     ; preds = %poison.cont8
  %48 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %sret9, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %8, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %36, ptr %9, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %48, ptr %10, align 4
  br label %race.yield.chk.suspended.0

race.yield.chk.suspended.0:                       ; preds = %panic.cont11
  %49 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %9, align 4
  %50 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %49, 0
  %51 = icmp eq i8 %50, 0
  br i1 %51, label %race.yield.suspended.0, label %race.yield.chk.suspended.1

race.yield.chk.failed.0:                          ; preds = %race.yield.chk.suspended.2
  %52 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %9, align 4
  %53 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %52, 0
  %54 = icmp eq i8 %53, 2
  br i1 %54, label %race.yield.failed.0, label %race.yield.chk.failed.1

race.yield.chk.suspended.1:                       ; preds = %race.yield.chk.suspended.0
  %55 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %10, align 4
  %56 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %55, 0
  %57 = icmp eq i8 %56, 0
  br i1 %57, label %race.yield.suspended.1, label %race.yield.chk.suspended.2

race.yield.chk.failed.1:                          ; preds = %race.yield.chk.failed.0
  %58 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %10, align 4
  %59 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %58, 0
  %60 = icmp eq i8 %59, 2
  br i1 %60, label %race.yield.failed.1, label %race.yield.chk.failed.2

race.yield.chk.suspended.2:                       ; preds = %race.yield.chk.suspended.1
  br label %race.yield.chk.failed.0

race.yield.chk.failed.2:                          ; preds = %race.yield.chk.failed.1
  br label %race.yield.completed

race.yield.suspended.0:                           ; preds = %race.yield.chk.suspended.0
  %61 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %9, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %61, ptr %11, align 4
  %62 = getelementptr i8, ptr %11, i64 8
  %63 = load i32, ptr %62, align 1
  store i32 %63, ptr %output, align 4
  %64 = load i32, ptr %output, align 4
  %65 = getelementptr i8, ptr %9, i64 8
  %66 = getelementptr i8, ptr %65, i64 8
  %67 = load ptr, ptr %66, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %12, align 4
  store i8 0, ptr %12, align 1
  %68 = getelementptr i8, ptr %12, i64 8
  store i32 %64, ptr %13, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %68, ptr align 1 %13, i64 4, i1 false)
  %69 = getelementptr i8, ptr %12, i64 8
  %70 = getelementptr i8, ptr %69, i64 8
  store ptr %67, ptr %70, align 8
  %71 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %12, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %71, ptr %8, align 4
  br label %race.yield.merge

race.yield.failed.0:                              ; preds = %race.yield.chk.failed.0
  %72 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %9, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %72, ptr %17, align 4
  %73 = getelementptr i8, ptr %17, i64 8
  %74 = load i8, ptr %73, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %6, align 4
  store i8 2, ptr %6, align 1
  %75 = getelementptr i8, ptr %6, i64 8
  store i8 %74, ptr %7, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %75, ptr align 1 %7, i64 1, i1 false)
  %76 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %6, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %76, ptr %8, align 4
  br label %race.yield.merge

race.yield.suspended.1:                           ; preds = %race.yield.chk.suspended.1
  %77 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %10, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %77, ptr %14, align 4
  %78 = getelementptr i8, ptr %14, i64 8
  %79 = load i32, ptr %78, align 1
  store i32 %79, ptr %output12, align 4
  %80 = load i32, ptr %output12, align 4
  %81 = getelementptr i8, ptr %10, i64 8
  %82 = getelementptr i8, ptr %81, i64 8
  %83 = load ptr, ptr %82, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %15, align 4
  store i8 0, ptr %15, align 1
  %84 = getelementptr i8, ptr %15, i64 8
  store i32 %80, ptr %16, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %84, ptr align 1 %16, i64 4, i1 false)
  %85 = getelementptr i8, ptr %15, i64 8
  %86 = getelementptr i8, ptr %85, i64 8
  store ptr %83, ptr %86, align 8
  %87 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %15, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %87, ptr %8, align 4
  br label %race.yield.merge

race.yield.failed.1:                              ; preds = %race.yield.chk.failed.1
  %88 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %10, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %88, ptr %18, align 4
  %89 = getelementptr i8, ptr %18, i64 8
  %90 = load i8, ptr %89, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %4, align 4
  store i8 2, ptr %4, align 1
  %91 = getelementptr i8, ptr %4, i64 8
  store i8 %90, ptr %5, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %91, ptr align 1 %5, i64 1, i1 false)
  %92 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %4, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %92, ptr %8, align 4
  br label %race.yield.merge

race.yield.completed:                             ; preds = %race.yield.chk.failed.2
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %3, align 4
  store i8 1, ptr %3, align 1
  %93 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %3, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %93, ptr %8, align 4
  br label %race.yield.merge

race.yield.merge:                                 ; preds = %race.yield.completed, %race.yield.failed.1, %race.yield.failed.0, %race.yield.suspended.1, %race.yield.suspended.0
  %94 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %8, align 4
  store { i8, [7 x i8], [24 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %95 = getelementptr i8, ptr %1, i64 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %94, ptr %2, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %95, ptr align 1 %2, i64 24, i1 false)
  %96 = load { i8, [7 x i8], [24 x i8], [0 x i64] }, ptr %1, align 4
  store { i8, [7 x i8], [24 x i8], [0 x i64] } %96, ptr %0, align 4
  ret void
}

define internal void @"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$resume"(ptr %__uv_host_env, ptr %__uv_async_out, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_frame, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_input, ptr %__panic) {
entry:
  %0 = alloca { i8, [7 x i8], [24 x i8], [0 x i64] }, align 8
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %2 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %3 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %4 = alloca i8, align 1
  %5 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %6 = alloca i8, align 1
  %output17 = alloca i32, align 4
  %output = alloca i32, align 4
  %sret14 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_566" = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_563" = alloca i8, align 1
  %sret = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_555" = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_552" = alloca i8, align 1
  %7 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %8 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %9 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %10 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %11 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %12 = alloca i32, align 4
  %13 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %14 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %15 = alloca i32, align 4
  %16 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %17 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
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
  %18 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %24 = load ptr, ptr %__uv_async_frame3, align 8
  %25 = load ptr, ptr %__uv_async_input4, align 8
  %26 = getelementptr i8, ptr %24, i64 0
  %27 = load i64, ptr %26, align 4
  switch i64 %27, label %async.resume.invalid [
  ]

async.resume.invalid:                             ; preds = %poison.cont
  ret void

async.resume.start:                               ; No predecessors!
  %28 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %29 = icmp ne i8 %28, 0
  br i1 %29, label %poison.take6, label %poison.cont7

poison.take6:                                     ; preds = %async.resume.start
  %30 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %30, align 1
  %31 = getelementptr i8, ptr %30, i64 4
  store i32 10, ptr %31, align 4
  ret void

poison.cont7:                                     ; preds = %async.resume.start
  store i8 0, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_552", align 1
  store i32 106, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_555", align 4
  %32 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %33 = icmp ne i8 %32, 0
  br i1 %33, label %poison.take8, label %poison.cont9

poison.take8:                                     ; preds = %poison.cont7
  %34 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %34, align 1
  %35 = getelementptr i8, ptr %34, i64 4
  store i32 10, ptr %35, align 4
  ret void

poison.cont9:                                     ; preds = %poison.cont7
  call void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactStreamMayFail(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %sret, ptr noundef nonnull align 1 dereferenceable(1) %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_552", ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_555", ptr noundef nonnull align 8 dereferenceable(8) %__panic5)
  %36 = load ptr, ptr %__panic5, align 8
  %37 = load i8, ptr %36, align 1
  %38 = icmp ne i8 %37, 0
  br i1 %38, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont9
  ret void

panic.cont:                                       ; preds = %poison.cont9
  %39 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %sret, align 4
  %40 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %41 = icmp ne i8 %40, 0
  br i1 %41, label %poison.take10, label %poison.cont11

poison.take10:                                    ; preds = %panic.cont
  %42 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %42, align 1
  %43 = getelementptr i8, ptr %42, i64 4
  store i32 10, ptr %43, align 4
  ret void

poison.cont11:                                    ; preds = %panic.cont
  store i8 0, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_563", align 1
  store i32 107, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_566", align 4
  %44 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %45 = icmp ne i8 %44, 0
  br i1 %45, label %poison.take12, label %poison.cont13

poison.take12:                                    ; preds = %poison.cont11
  %46 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %46, align 1
  %47 = getelementptr i8, ptr %46, i64 4
  store i32 10, ptr %47, align 4
  ret void

poison.cont13:                                    ; preds = %poison.cont11
  call void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactStreamMayFail(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %sret14, ptr noundef nonnull align 1 dereferenceable(1) %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_563", ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3aasyncCompositionRaceStreamArtifactReference$tmp$call_ref_tmp_566", ptr noundef nonnull align 8 dereferenceable(8) %__panic5)
  %48 = load ptr, ptr %__panic5, align 8
  %49 = load i8, ptr %48, align 1
  %50 = icmp ne i8 %49, 0
  br i1 %50, label %panic.take15, label %panic.cont16

panic.take15:                                     ; preds = %poison.cont13
  ret void

panic.cont16:                                     ; preds = %poison.cont13
  %51 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %sret14, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %7, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %39, ptr %8, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %51, ptr %9, align 4
  br label %race.yield.chk.suspended.0

race.yield.chk.suspended.0:                       ; preds = %panic.cont16
  %52 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %8, align 4
  %53 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %52, 0
  %54 = icmp eq i8 %53, 0
  br i1 %54, label %race.yield.suspended.0, label %race.yield.chk.suspended.1

race.yield.chk.failed.0:                          ; preds = %race.yield.chk.suspended.2
  %55 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %8, align 4
  %56 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %55, 0
  %57 = icmp eq i8 %56, 2
  br i1 %57, label %race.yield.failed.0, label %race.yield.chk.failed.1

race.yield.chk.suspended.1:                       ; preds = %race.yield.chk.suspended.0
  %58 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %9, align 4
  %59 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %58, 0
  %60 = icmp eq i8 %59, 0
  br i1 %60, label %race.yield.suspended.1, label %race.yield.chk.suspended.2

race.yield.chk.failed.1:                          ; preds = %race.yield.chk.failed.0
  %61 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %9, align 4
  %62 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %61, 0
  %63 = icmp eq i8 %62, 2
  br i1 %63, label %race.yield.failed.1, label %race.yield.chk.failed.2

race.yield.chk.suspended.2:                       ; preds = %race.yield.chk.suspended.1
  br label %race.yield.chk.failed.0

race.yield.chk.failed.2:                          ; preds = %race.yield.chk.failed.1
  br label %race.yield.completed

race.yield.suspended.0:                           ; preds = %race.yield.chk.suspended.0
  %64 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %8, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %64, ptr %10, align 4
  %65 = getelementptr i8, ptr %10, i64 8
  %66 = load i32, ptr %65, align 1
  store i32 %66, ptr %output, align 4
  %67 = load i32, ptr %output, align 4
  %68 = getelementptr i8, ptr %8, i64 8
  %69 = getelementptr i8, ptr %68, i64 8
  %70 = load ptr, ptr %69, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %11, align 4
  store i8 0, ptr %11, align 1
  %71 = getelementptr i8, ptr %11, i64 8
  store i32 %67, ptr %12, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %71, ptr align 1 %12, i64 4, i1 false)
  %72 = getelementptr i8, ptr %11, i64 8
  %73 = getelementptr i8, ptr %72, i64 8
  store ptr %70, ptr %73, align 8
  %74 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %11, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %74, ptr %7, align 4
  br label %race.yield.merge

race.yield.failed.0:                              ; preds = %race.yield.chk.failed.0
  %75 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %8, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %75, ptr %16, align 4
  %76 = getelementptr i8, ptr %16, i64 8
  %77 = load i8, ptr %76, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %5, align 4
  store i8 2, ptr %5, align 1
  %78 = getelementptr i8, ptr %5, i64 8
  store i8 %77, ptr %6, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %78, ptr align 1 %6, i64 1, i1 false)
  %79 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %5, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %79, ptr %7, align 4
  br label %race.yield.merge

race.yield.suspended.1:                           ; preds = %race.yield.chk.suspended.1
  %80 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %9, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %80, ptr %13, align 4
  %81 = getelementptr i8, ptr %13, i64 8
  %82 = load i32, ptr %81, align 1
  store i32 %82, ptr %output17, align 4
  %83 = load i32, ptr %output17, align 4
  %84 = getelementptr i8, ptr %9, i64 8
  %85 = getelementptr i8, ptr %84, i64 8
  %86 = load ptr, ptr %85, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %14, align 4
  store i8 0, ptr %14, align 1
  %87 = getelementptr i8, ptr %14, i64 8
  store i32 %83, ptr %15, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %87, ptr align 1 %15, i64 4, i1 false)
  %88 = getelementptr i8, ptr %14, i64 8
  %89 = getelementptr i8, ptr %88, i64 8
  store ptr %86, ptr %89, align 8
  %90 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %14, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %90, ptr %7, align 4
  br label %race.yield.merge

race.yield.failed.1:                              ; preds = %race.yield.chk.failed.1
  %91 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %9, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %91, ptr %17, align 4
  %92 = getelementptr i8, ptr %17, i64 8
  %93 = load i8, ptr %92, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %3, align 4
  store i8 2, ptr %3, align 1
  %94 = getelementptr i8, ptr %3, i64 8
  store i8 %93, ptr %4, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %94, ptr align 1 %4, i64 1, i1 false)
  %95 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %3, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %95, ptr %7, align 4
  br label %race.yield.merge

race.yield.completed:                             ; preds = %race.yield.chk.failed.2
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %2, align 4
  store i8 1, ptr %2, align 1
  %96 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %2, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %96, ptr %7, align 4
  br label %race.yield.merge

race.yield.merge:                                 ; preds = %race.yield.completed, %race.yield.failed.1, %race.yield.failed.0, %race.yield.suspended.1, %race.yield.suspended.0
  %97 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %7, align 4
  store { i8, [7 x i8], [24 x i8], [0 x i64] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %98 = getelementptr i8, ptr %0, i64 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %97, ptr %1, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %98, ptr align 1 %1, i64 24, i1 false)
  %99 = load { i8, [7 x i8], [24 x i8], [0 x i64] }, ptr %0, align 4
  %100 = load ptr, ptr %__uv_async_out2, align 8
  %101 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [24 x i8], [0 x i64] } %99, ptr %101, align 4
  ret void
}

define i8 @ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %chained_result = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 4
  %0 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %1 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %coerce_bits52 = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %2 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %3 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %4 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %5 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %chained44 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %6 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %7 = alloca i8, align 1
  %sret43 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %coerce_bits42 = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %8 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %9 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_move_tmp_663" = alloca ptr, align 8
  %sret39 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_640" = alloca i32, align 4
  %chained = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %folded_result = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 4
  %10 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %11 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %coerce_bits34 = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %12 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %13 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %14 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %15 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %folded33 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %16 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %17 = alloca i8, align 1
  %18 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %19 = alloca i32, align 4
  %coerce_bits32 = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %byref_arg31 = alloca i32, align 4
  %20 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %21 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %22 = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_move_tmp_635" = alloca ptr, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_move_tmp_627" = alloca i32, align 4
  %sret28 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_618" = alloca i32, align 4
  %folded = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %taken23 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %coerce_bits22 = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %23 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %24 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_move_tmp_614" = alloca i64, align 8
  %sret19 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_605" = alloca i32, align 4
  %taken = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %filtered14 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %coerce_bits = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %sret13 = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %25 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %26 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_move_tmp_601" = alloca ptr, align 8
  %sret10 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_591" = alloca i32, align 4
  %filtered = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %mapped5 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %byref_arg = alloca i32, align 4
  %27 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %28 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_move_tmp_587" = alloca ptr, align 8
  %sret = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_577" = alloca i32, align 4
  %mapped = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %29 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %30 = icmp ne i8 %29, 0
  br i1 %30, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %31 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %31, align 1
  %32 = getelementptr i8, ptr %31, i64 4
  store i32 10, ptr %32, align 4
  %33 = load ptr, ptr %__panic, align 8
  %34 = getelementptr i8, ptr %33, i64 4
  %35 = load i32, ptr %34, align 4
  %36 = trunc i32 %35 to i8
  ret i8 %36

poison.cont:                                      ; preds = %entry
  %37 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %37, align 1
  %38 = getelementptr i8, ptr %37, i64 4
  store i32 0, ptr %38, align 4
  %39 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %40 = icmp ne i8 %39, 0
  br i1 %40, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %41 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %41, align 1
  %42 = getelementptr i8, ptr %41, i64 4
  store i32 10, ptr %42, align 4
  %43 = load ptr, ptr %__panic, align 8
  %44 = getelementptr i8, ptr %43, i64 4
  %45 = load i32, ptr %44, align 4
  %46 = trunc i32 %45 to i8
  ret i8 %46

poison.cont2:                                     ; preds = %poison.cont
  store i32 108, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_577", align 4
  %47 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %48 = icmp ne i8 %47, 0
  br i1 %48, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %49 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %49, align 1
  %50 = getelementptr i8, ptr %49, i64 4
  store i32 10, ptr %50, align 4
  %51 = load ptr, ptr %__panic, align 8
  %52 = getelementptr i8, ptr %51, i64 4
  %53 = load i32, ptr %52, align 4
  %54 = trunc i32 %53 to i8
  ret i8 %54

poison.cont4:                                     ; preds = %poison.cont2
  call void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactSingleOutput(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %sret, ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_577", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %55 = load ptr, ptr %__panic, align 8
  %56 = load i8, ptr %55, align 1
  %57 = icmp ne i8 %56, 0
  br i1 %57, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  %58 = load ptr, ptr %__panic, align 8
  %59 = getelementptr i8, ptr %58, i64 4
  %60 = load i32, ptr %59, align 4
  %61 = trunc i32 %60 to i8
  ret i8 %61

panic.cont:                                       ; preds = %poison.cont4
  store ptr @ExpressionSemantics_x5fx3a_x5fx3aasyncCompositionCombinatorArtifactReference_x3a_x3a_x5fclosure0, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_move_tmp_587", align 8
  %62 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %sret, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %62, ptr %27, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %28, align 4
  %63 = load ptr, ptr %__panic, align 8
  %64 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %27, align 4
  %65 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %64, 0
  %66 = icmp eq i8 %65, 0
  br i1 %66, label %ac.map.suspended, label %ac.map.merge

ac.map.suspended:                                 ; preds = %panic.cont
  %67 = getelementptr i8, ptr %27, i64 8
  %68 = load i32, ptr %67, align 1
  %69 = load ptr, ptr %__panic, align 8
  store i32 %68, ptr %byref_arg, align 4
  %70 = call i32 @ExpressionSemantics_x5fx3a_x5fx3aasyncCompositionCombinatorArtifactReference_x3a_x3a_x5fclosure0(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %71 = getelementptr i8, ptr %27, i64 8
  store i32 %70, ptr %71, align 1
  br label %ac.map.merge

ac.map.merge:                                     ; preds = %ac.map.suspended, %panic.cont
  %72 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %27, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %72, ptr %28, align 4
  %73 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %28, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %73, ptr %mapped5, align 4
  %74 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %75 = icmp ne i8 %74, 0
  br i1 %75, label %poison.take6, label %poison.cont7

poison.take6:                                     ; preds = %ac.map.merge
  %76 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %76, align 1
  %77 = getelementptr i8, ptr %76, i64 4
  store i32 10, ptr %77, align 4
  %78 = load ptr, ptr %__panic, align 8
  %79 = getelementptr i8, ptr %78, i64 4
  %80 = load i32, ptr %79, align 4
  %81 = trunc i32 %80 to i8
  ret i8 %81

poison.cont7:                                     ; preds = %ac.map.merge
  store i32 109, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_591", align 4
  %82 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %83 = icmp ne i8 %82, 0
  br i1 %83, label %poison.take8, label %poison.cont9

poison.take8:                                     ; preds = %poison.cont7
  %84 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %84, align 1
  %85 = getelementptr i8, ptr %84, i64 4
  store i32 10, ptr %85, align 4
  %86 = load ptr, ptr %__panic, align 8
  %87 = getelementptr i8, ptr %86, i64 4
  %88 = load i32, ptr %87, align 4
  %89 = trunc i32 %88 to i8
  ret i8 %89

poison.cont9:                                     ; preds = %poison.cont7
  call void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactSingleOutput(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %sret10, ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_591", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %90 = load ptr, ptr %__panic, align 8
  %91 = load i8, ptr %90, align 1
  %92 = icmp ne i8 %91, 0
  br i1 %92, label %panic.take11, label %panic.cont12

panic.take11:                                     ; preds = %poison.cont9
  %93 = load ptr, ptr %__panic, align 8
  %94 = getelementptr i8, ptr %93, i64 0
  %95 = load i8, ptr %94, align 1
  %96 = load ptr, ptr %__panic, align 8
  %97 = getelementptr i8, ptr %96, i64 4
  %98 = load i32, ptr %97, align 4
  %99 = load ptr, ptr %__panic, align 8
  %100 = getelementptr i8, ptr %99, i64 4
  %101 = load i32, ptr %100, align 4
  %102 = trunc i32 %101 to i8
  ret i8 %102

panic.cont12:                                     ; preds = %poison.cont9
  store ptr @ExpressionSemantics_x5fx3a_x5fx3aasyncCompositionCombinatorArtifactReference_x3a_x3a_x5fclosure1, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_move_tmp_601", align 8
  %103 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %sret10, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %103, ptr %25, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %26, align 4
  %104 = load ptr, ptr %__panic, align 8
  br label %ac.filter.loop

ac.filter.loop:                                   ; preds = %ac.filter.resume, %panic.cont12
  %105 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %25, align 4
  %106 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %105, 0
  switch i8 %106, label %ac.filter.exit [
    i8 0, label %ac.filter.suspended
    i8 1, label %ac.filter.exit
    i8 2, label %ac.filter.exit
  ]

ac.filter.suspended:                              ; preds = %ac.filter.loop
  %107 = getelementptr i8, ptr %25, i64 8
  %108 = load i32, ptr %107, align 1
  %109 = load ptr, ptr %__panic, align 8
  store i32 %108, ptr %byref_arg, align 4
  %110 = call i8 @ExpressionSemantics_x5fx3a_x5fx3aasyncCompositionCombinatorArtifactReference_x3a_x3a_x5fclosure1(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %111 = icmp ne i8 %110, 0
  br i1 %111, label %ac.filter.exit, label %ac.filter.resume

ac.filter.resume:                                 ; preds = %ac.filter.suspended
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aresume(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8] }) align 1 dereferenceable(24) %sret13, ptr %25, ptr null, ptr %104)
  %112 = load { i8, [7 x i8], [16 x i8] }, ptr %sret13, align 1
  store { i8, [7 x i8], [16 x i8] } %112, ptr %coerce_bits, align 1
  %113 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %coerce_bits, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %113, ptr %25, align 4
  br label %ac.filter.loop

ac.filter.exit:                                   ; preds = %ac.filter.suspended, %ac.filter.loop, %ac.filter.loop, %ac.filter.loop
  %114 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %25, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %114, ptr %26, align 4
  %115 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %26, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %115, ptr %filtered14, align 4
  %116 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %117 = icmp ne i8 %116, 0
  br i1 %117, label %poison.take15, label %poison.cont16

poison.take15:                                    ; preds = %ac.filter.exit
  %118 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %118, align 1
  %119 = getelementptr i8, ptr %118, i64 4
  store i32 10, ptr %119, align 4
  %120 = load ptr, ptr %__panic, align 8
  %121 = getelementptr i8, ptr %120, i64 4
  %122 = load i32, ptr %121, align 4
  %123 = trunc i32 %122 to i8
  ret i8 %123

poison.cont16:                                    ; preds = %ac.filter.exit
  store i32 110, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_605", align 4
  %124 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %125 = icmp ne i8 %124, 0
  br i1 %125, label %poison.take17, label %poison.cont18

poison.take17:                                    ; preds = %poison.cont16
  %126 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %126, align 1
  %127 = getelementptr i8, ptr %126, i64 4
  store i32 10, ptr %127, align 4
  %128 = load ptr, ptr %__panic, align 8
  %129 = getelementptr i8, ptr %128, i64 4
  %130 = load i32, ptr %129, align 4
  %131 = trunc i32 %130 to i8
  ret i8 %131

poison.cont18:                                    ; preds = %poison.cont16
  call void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactSingleOutput(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %sret19, ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_605", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %132 = load ptr, ptr %__panic, align 8
  %133 = load i8, ptr %132, align 1
  %134 = icmp ne i8 %133, 0
  br i1 %134, label %panic.take20, label %panic.cont21

panic.take20:                                     ; preds = %poison.cont18
  %135 = load ptr, ptr %__panic, align 8
  %136 = getelementptr i8, ptr %135, i64 0
  %137 = load i8, ptr %136, align 1
  %138 = load ptr, ptr %__panic, align 8
  %139 = getelementptr i8, ptr %138, i64 4
  %140 = load i32, ptr %139, align 4
  %141 = load ptr, ptr %__panic, align 8
  %142 = getelementptr i8, ptr %141, i64 4
  %143 = load i32, ptr %142, align 4
  %144 = trunc i32 %143 to i8
  ret i8 %144

panic.cont21:                                     ; preds = %poison.cont18
  store i64 1, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_move_tmp_614", align 4
  %145 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %sret19, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %145, ptr %23, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %24, align 4
  %146 = load ptr, ptr %__panic, align 8
  %take.count = load i64, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_move_tmp_614", align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3atake(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8] }) align 1 dereferenceable(24) %sret13, ptr %23, i64 %take.count, ptr %146)
  %147 = load { i8, [7 x i8], [16 x i8] }, ptr %sret13, align 1
  store { i8, [7 x i8], [16 x i8] } %147, ptr %coerce_bits22, align 1
  %148 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %coerce_bits22, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %148, ptr %24, align 4
  %149 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %24, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %149, ptr %taken23, align 4
  %150 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %151 = icmp ne i8 %150, 0
  br i1 %151, label %poison.take24, label %poison.cont25

poison.take24:                                    ; preds = %panic.cont21
  %152 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %152, align 1
  %153 = getelementptr i8, ptr %152, i64 4
  store i32 10, ptr %153, align 4
  %154 = load ptr, ptr %__panic, align 8
  %155 = getelementptr i8, ptr %154, i64 4
  %156 = load i32, ptr %155, align 4
  %157 = trunc i32 %156 to i8
  ret i8 %157

poison.cont25:                                    ; preds = %panic.cont21
  store i32 111, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_618", align 4
  %158 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %159 = icmp ne i8 %158, 0
  br i1 %159, label %poison.take26, label %poison.cont27

poison.take26:                                    ; preds = %poison.cont25
  %160 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %160, align 1
  %161 = getelementptr i8, ptr %160, i64 4
  store i32 10, ptr %161, align 4
  %162 = load ptr, ptr %__panic, align 8
  %163 = getelementptr i8, ptr %162, i64 4
  %164 = load i32, ptr %163, align 4
  %165 = trunc i32 %164 to i8
  ret i8 %165

poison.cont27:                                    ; preds = %poison.cont25
  call void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactSingleOutput(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %sret28, ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_618", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %166 = load ptr, ptr %__panic, align 8
  %167 = load i8, ptr %166, align 1
  %168 = icmp ne i8 %167, 0
  br i1 %168, label %panic.take29, label %panic.cont30

panic.take29:                                     ; preds = %poison.cont27
  %169 = load ptr, ptr %__panic, align 8
  %170 = getelementptr i8, ptr %169, i64 0
  %171 = load i8, ptr %170, align 1
  %172 = load ptr, ptr %__panic, align 8
  %173 = getelementptr i8, ptr %172, i64 4
  %174 = load i32, ptr %173, align 4
  %175 = load ptr, ptr %__panic, align 8
  %176 = getelementptr i8, ptr %175, i64 4
  %177 = load i32, ptr %176, align 4
  %178 = trunc i32 %177 to i8
  ret i8 %178

panic.cont30:                                     ; preds = %poison.cont27
  store i32 10, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_move_tmp_627", align 4
  store ptr @ExpressionSemantics_x5fx3a_x5fx3aasyncCompositionCombinatorArtifactReference_x3a_x3a_x5fclosure2, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_move_tmp_635", align 8
  %179 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %sret28, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %179, ptr %20, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %21, align 4
  %180 = load ptr, ptr %__panic, align 8
  %fold.acc.init = load i32, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_move_tmp_627", align 4
  store i32 %fold.acc.init, ptr %22, align 4
  br label %ac.fold.loop

ac.fold.loop:                                     ; preds = %ac.fold.resume, %panic.cont30
  %181 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %20, align 4
  %182 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %181, 0
  switch i8 %182, label %ac.fold.completed [
    i8 0, label %ac.fold.suspended
    i8 1, label %ac.fold.completed
    i8 2, label %ac.fold.failed
  ]

ac.fold.suspended:                                ; preds = %ac.fold.loop
  %183 = getelementptr i8, ptr %20, i64 8
  %184 = load i32, ptr %183, align 1
  %185 = load i32, ptr %22, align 4
  %186 = load ptr, ptr %__panic, align 8
  store i32 %185, ptr %byref_arg, align 4
  store i32 %184, ptr %byref_arg31, align 4
  %187 = call i32 @ExpressionSemantics_x5fx3a_x5fx3aasyncCompositionCombinatorArtifactReference_x3a_x3a_x5fclosure2(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg, ptr noundef nonnull align 4 dereferenceable(4) %byref_arg31, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  store i32 %187, ptr %22, align 4
  br label %ac.fold.resume

ac.fold.resume:                                   ; preds = %ac.fold.suspended
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aresume(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8] }) align 1 dereferenceable(24) %sret13, ptr %20, ptr null, ptr %180)
  %188 = load { i8, [7 x i8], [16 x i8] }, ptr %sret13, align 1
  store { i8, [7 x i8], [16 x i8] } %188, ptr %coerce_bits32, align 1
  %189 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %coerce_bits32, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %189, ptr %20, align 4
  br label %ac.fold.loop

ac.fold.completed:                                ; preds = %ac.fold.loop, %ac.fold.loop
  %190 = load i32, ptr %22, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %18, align 4
  store i8 1, ptr %18, align 1
  %191 = getelementptr i8, ptr %18, i64 8
  store i32 %190, ptr %19, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %191, ptr align 1 %19, i64 4, i1 false)
  %192 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %18, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %192, ptr %21, align 4
  br label %ac.fold.merge

ac.fold.failed:                                   ; preds = %ac.fold.loop
  %193 = getelementptr i8, ptr %20, i64 8
  %194 = load i8, ptr %193, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %16, align 4
  store i8 2, ptr %16, align 1
  %195 = getelementptr i8, ptr %16, i64 8
  store i8 %194, ptr %17, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %195, ptr align 1 %17, i64 1, i1 false)
  %196 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %16, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %196, ptr %21, align 4
  br label %ac.fold.merge

ac.fold.merge:                                    ; preds = %ac.fold.failed, %ac.fold.completed
  %197 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %21, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %197, ptr %folded33, align 4
  %198 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %folded33, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %198, ptr %12, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %13, align 4
  %199 = load ptr, ptr %__panic, align 8
  br label %sync.loop

sync.loop:                                        ; preds = %sync.suspended, %ac.fold.merge
  %200 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %12, align 4
  %201 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %200, 0
  switch i8 %201, label %sync.fallback [
    i8 0, label %sync.suspended
    i8 1, label %sync.completed
    i8 2, label %sync.failed
  ]

sync.suspended:                                   ; preds = %sync.loop
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aresume(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8] }) align 1 dereferenceable(24) %sret13, ptr %12, ptr null, ptr %199)
  %202 = load { i8, [7 x i8], [16 x i8] }, ptr %sret13, align 1
  store { i8, [7 x i8], [16 x i8] } %202, ptr %coerce_bits34, align 1
  %203 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %coerce_bits34, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %203, ptr %12, align 4
  %204 = load i8, ptr %199, align 1
  %205 = icmp ne i8 %204, 0
  br i1 %205, label %sync.panic, label %sync.loop

sync.completed:                                   ; preds = %sync.loop
  %206 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %12, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %206, ptr %14, align 4
  %207 = getelementptr i8, ptr %14, i64 8
  %208 = load i32, ptr %207, align 1
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %11, align 4
  store i8 0, ptr %11, align 1
  %209 = getelementptr i8, ptr %11, i64 4
  %210 = getelementptr i8, ptr %209, i64 0
  store i32 %208, ptr %210, align 1
  %211 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %11, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %211, ptr %13, align 4
  br label %sync.merge

sync.failed:                                      ; preds = %sync.loop
  %212 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %12, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %212, ptr %15, align 4
  %213 = getelementptr i8, ptr %15, i64 8
  %214 = load i8, ptr %213, align 1
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %10, align 4
  store i8 1, ptr %10, align 1
  %215 = getelementptr i8, ptr %10, i64 4
  %216 = getelementptr i8, ptr %215, i64 0
  store i8 %214, ptr %216, align 1
  %217 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %10, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %217, ptr %13, align 4
  br label %sync.merge

sync.fallback:                                    ; preds = %sync.loop
  %218 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %12, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %13, align 4
  br label %sync.merge

sync.panic:                                       ; preds = %sync.suspended
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %13, align 4
  br label %sync.merge

sync.merge:                                       ; preds = %sync.panic, %sync.fallback, %sync.failed, %sync.completed
  %219 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %13, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %219, ptr %folded_result, align 4
  %220 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %221 = icmp ne i8 %220, 0
  br i1 %221, label %poison.take35, label %poison.cont36

poison.take35:                                    ; preds = %sync.merge
  %222 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %222, align 1
  %223 = getelementptr i8, ptr %222, i64 4
  store i32 10, ptr %223, align 4
  %224 = load ptr, ptr %__panic, align 8
  %225 = getelementptr i8, ptr %224, i64 4
  %226 = load i32, ptr %225, align 4
  %227 = trunc i32 %226 to i8
  ret i8 %227

poison.cont36:                                    ; preds = %sync.merge
  store i32 112, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_640", align 4
  %228 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %229 = icmp ne i8 %228, 0
  br i1 %229, label %poison.take37, label %poison.cont38

poison.take37:                                    ; preds = %poison.cont36
  %230 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %230, align 1
  %231 = getelementptr i8, ptr %230, i64 4
  store i32 10, ptr %231, align 4
  %232 = load ptr, ptr %__panic, align 8
  %233 = getelementptr i8, ptr %232, i64 4
  %234 = load i32, ptr %233, align 4
  %235 = trunc i32 %234 to i8
  ret i8 %235

poison.cont38:                                    ; preds = %poison.cont36
  call void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactCompletes(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %sret39, ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_640", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %236 = load ptr, ptr %__panic, align 8
  %237 = load i8, ptr %236, align 1
  %238 = icmp ne i8 %237, 0
  br i1 %238, label %panic.take40, label %panic.cont41

panic.take40:                                     ; preds = %poison.cont38
  %239 = load ptr, ptr %__panic, align 8
  %240 = getelementptr i8, ptr %239, i64 0
  %241 = load i8, ptr %240, align 1
  %242 = load ptr, ptr %__panic, align 8
  %243 = getelementptr i8, ptr %242, i64 4
  %244 = load i32, ptr %243, align 4
  %245 = load ptr, ptr %__panic, align 8
  %246 = getelementptr i8, ptr %245, i64 4
  %247 = load i32, ptr %246, align 4
  %248 = trunc i32 %247 to i8
  ret i8 %248

panic.cont41:                                     ; preds = %poison.cont38
  store ptr @ExpressionSemantics_x5fx3a_x5fx3aasyncCompositionCombinatorArtifactReference_x3a_x3a_x5fclosure3, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_move_tmp_663", align 8
  %249 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %sret39, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %249, ptr %8, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %9, align 4
  %250 = load ptr, ptr %__panic, align 8
  br label %ac.chain.loop

ac.chain.loop:                                    ; preds = %ac.chain.resume, %panic.cont41
  %251 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %8, align 4
  %252 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %251, 0
  switch i8 %252, label %ac.chain.completed [
    i8 0, label %ac.chain.suspended
    i8 1, label %ac.chain.completed
    i8 2, label %ac.chain.failed
  ]

ac.chain.suspended:                               ; preds = %ac.chain.loop
  br label %ac.chain.resume

ac.chain.resume:                                  ; preds = %ac.chain.suspended
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aresume(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8] }) align 1 dereferenceable(24) %sret13, ptr %8, ptr null, ptr %250)
  %253 = load { i8, [7 x i8], [16 x i8] }, ptr %sret13, align 1
  store { i8, [7 x i8], [16 x i8] } %253, ptr %coerce_bits42, align 1
  %254 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %coerce_bits42, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %254, ptr %8, align 4
  br label %ac.chain.loop

ac.chain.completed:                               ; preds = %ac.chain.loop, %ac.chain.loop
  %255 = getelementptr i8, ptr %8, i64 8
  %256 = load i32, ptr %255, align 1
  %257 = load ptr, ptr %__panic, align 8
  store i32 %256, ptr %byref_arg, align 4
  call void @ExpressionSemantics_x5fx3a_x5fx3aasyncCompositionCombinatorArtifactReference_x3a_x3a_x5fclosure3(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %sret43, ptr noundef nonnull align 4 dereferenceable(4) %byref_arg, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %258 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %sret43, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %258, ptr %9, align 4
  br label %ac.chain.merge

ac.chain.failed:                                  ; preds = %ac.chain.loop
  %259 = getelementptr i8, ptr %8, i64 8
  %260 = load i8, ptr %259, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %6, align 4
  store i8 2, ptr %6, align 1
  %261 = getelementptr i8, ptr %6, i64 8
  store i8 %260, ptr %7, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %261, ptr align 1 %7, i64 1, i1 false)
  %262 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %6, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %262, ptr %9, align 4
  br label %ac.chain.merge

ac.chain.merge:                                   ; preds = %ac.chain.failed, %ac.chain.completed
  %263 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %9, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %263, ptr %chained44, align 4
  %264 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %chained44, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %264, ptr %2, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %3, align 4
  %265 = load ptr, ptr %__panic, align 8
  br label %sync.loop45

sync.loop45:                                      ; preds = %sync.suspended46, %ac.chain.merge
  %266 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %2, align 4
  %267 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %266, 0
  switch i8 %267, label %sync.fallback49 [
    i8 0, label %sync.suspended46
    i8 1, label %sync.completed47
    i8 2, label %sync.failed48
  ]

sync.suspended46:                                 ; preds = %sync.loop45
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aresume(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8] }) align 1 dereferenceable(24) %sret13, ptr %2, ptr null, ptr %265)
  %268 = load { i8, [7 x i8], [16 x i8] }, ptr %sret13, align 1
  store { i8, [7 x i8], [16 x i8] } %268, ptr %coerce_bits52, align 1
  %269 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %coerce_bits52, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %269, ptr %2, align 4
  %270 = load i8, ptr %265, align 1
  %271 = icmp ne i8 %270, 0
  br i1 %271, label %sync.panic50, label %sync.loop45

sync.completed47:                                 ; preds = %sync.loop45
  %272 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %2, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %272, ptr %4, align 4
  %273 = getelementptr i8, ptr %4, i64 8
  %274 = load i32, ptr %273, align 1
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %1, align 4
  store i8 0, ptr %1, align 1
  %275 = getelementptr i8, ptr %1, i64 4
  %276 = getelementptr i8, ptr %275, i64 0
  store i32 %274, ptr %276, align 1
  %277 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %1, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %277, ptr %3, align 4
  br label %sync.merge51

sync.failed48:                                    ; preds = %sync.loop45
  %278 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %2, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %278, ptr %5, align 4
  %279 = getelementptr i8, ptr %5, i64 8
  %280 = load i8, ptr %279, align 1
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %281 = getelementptr i8, ptr %0, i64 4
  %282 = getelementptr i8, ptr %281, i64 0
  store i8 %280, ptr %282, align 1
  %283 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %0, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %283, ptr %3, align 4
  br label %sync.merge51

sync.fallback49:                                  ; preds = %sync.loop45
  %284 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %2, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %3, align 4
  br label %sync.merge51

sync.panic50:                                     ; preds = %sync.suspended46
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %3, align 4
  br label %sync.merge51

sync.merge51:                                     ; preds = %sync.panic50, %sync.fallback49, %sync.failed48, %sync.completed47
  %285 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %3, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %285, ptr %chained_result, align 4
  ret i8 1
}

define internal i32 @ExpressionSemantics_x5fx3a_x5fx3aasyncCompositionCombinatorArtifactReference_x3a_x3a_x5fclosure0(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %15 = getelementptr i8, ptr %14, i64 0
  %16 = load i8, ptr %15, align 1
  %17 = load ptr, ptr %__panic, align 8
  %18 = getelementptr i8, ptr %17, i64 4
  %19 = load i32, ptr %18, align 4
  %20 = load ptr, ptr %__panic, align 8
  %21 = getelementptr i8, ptr %20, i64 4
  %22 = load i32, ptr %21, align 4
  ret i32 %22

panic.cont:                                       ; preds = %check_ok
  %23 = load i32, ptr %value, align 4
  %24 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %23, i32 1)
  %25 = extractvalue { i32, i1 } %24, 0
  %26 = extractvalue { i32, i1 } %24, 1
  %27 = freeze i32 %25
  br i1 %26, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  ret i32 %27

op_fail:                                          ; preds = %panic.cont
  %28 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %28, align 1
  %29 = getelementptr i8, ptr %28, i64 4
  store i32 4, ptr %29, align 4
  ret i32 0
}

define internal i8 @ExpressionSemantics_x5fx3a_x5fx3aasyncCompositionCombinatorArtifactReference_x3a_x3a_x5fclosure1(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %10 = load i32, ptr %value, align 4
  %11 = icmp eq i32 %10, 109
  %12 = zext i1 %11 to i8
  %13 = icmp ne i8 %12, 0
  %14 = zext i1 %13 to i8
  ret i8 %14
}

define internal i32 @ExpressionSemantics_x5fx3a_x5fx3aasyncCompositionCombinatorArtifactReference_x3a_x3a_x5fclosure2(ptr noundef nonnull align 4 dereferenceable(4) %acc, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %15 = getelementptr i8, ptr %14, i64 0
  %16 = load i8, ptr %15, align 1
  %17 = load ptr, ptr %__panic, align 8
  %18 = getelementptr i8, ptr %17, i64 4
  %19 = load i32, ptr %18, align 4
  %20 = load ptr, ptr %__panic, align 8
  %21 = getelementptr i8, ptr %20, i64 4
  %22 = load i32, ptr %21, align 4
  ret i32 %22

panic.cont:                                       ; preds = %check_ok
  %23 = load i32, ptr %acc, align 4
  %24 = load i32, ptr %value, align 4
  %25 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %23, i32 %24)
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

define internal void @ExpressionSemantics_x5fx3a_x5fx3aasyncCompositionCombinatorArtifactReference_x3a_x3a_x5fclosure3(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %0, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_655" = alloca i32, align 4
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %2 = icmp ne i8 %1, 0
  br i1 %2, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %3 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %3, align 1
  %4 = getelementptr i8, ptr %3, i64 4
  store i32 10, ptr %4, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %5 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 0, ptr %6, align 4
  %7 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %8 = icmp ne i8 %7, 0
  br i1 %8, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %9 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 10, ptr %10, align 4
  ret void

poison.cont2:                                     ; preds = %poison.cont
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont2
  %11 = load ptr, ptr %__panic, align 8
  %12 = load i8, ptr %11, align 1
  %13 = icmp ne i8 %12, 0
  br i1 %13, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont2
  %14 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %14, align 1
  %15 = getelementptr i8, ptr %14, i64 4
  store i32 4, ptr %15, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %16 = load ptr, ptr %__panic, align 8
  %17 = getelementptr i8, ptr %16, i64 0
  %18 = load i8, ptr %17, align 1
  %19 = load ptr, ptr %__panic, align 8
  %20 = getelementptr i8, ptr %19, i64 4
  %21 = load i32, ptr %20, align 4
  ret void

panic.cont:                                       ; preds = %check_ok
  %22 = load i32, ptr %value, align 4
  %23 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %22, i32 1)
  %24 = extractvalue { i32, i1 } %23, 0
  %25 = extractvalue { i32, i1 } %23, 1
  %26 = freeze i32 %24
  br i1 %25, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  store i32 %26, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_655", align 4
  %27 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %28 = icmp ne i8 %27, 0
  br i1 %28, label %poison.take3, label %poison.cont4

op_fail:                                          ; preds = %panic.cont
  %29 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %29, align 1
  %30 = getelementptr i8, ptr %29, i64 4
  store i32 4, ptr %30, align 4
  ret void

poison.take3:                                     ; preds = %op_ok
  %31 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %31, align 1
  %32 = getelementptr i8, ptr %31, i64 4
  store i32 10, ptr %32, align 4
  ret void

poison.cont4:                                     ; preds = %op_ok
  call void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactCompletes(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %0, ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_655", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %33 = load ptr, ptr %__panic, align 8
  %34 = load i8, ptr %33, align 1
  %35 = icmp ne i8 %34, 0
  br i1 %35, label %panic.take5, label %panic.cont6

panic.take5:                                      ; preds = %poison.cont4
  %36 = load ptr, ptr %__panic, align 8
  %37 = getelementptr i8, ptr %36, i64 0
  %38 = load i8, ptr %37, align 1
  %39 = load ptr, ptr %__panic, align 8
  %40 = getelementptr i8, ptr %39, i64 4
  %41 = load i32, ptr %40, align 4
  ret void

panic.cont6:                                      ; preds = %poison.cont4
  ret void
}

define internal void @"ExpressionSemantics_x5fx3a_x5fx3aasyncCompositionCombinatorArtifactReference_x3a_x3a_x5fclosure3$resume"(ptr %__uv_host_env, ptr %__uv_async_out, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_frame, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_input, ptr %__panic) {
entry:
  %sret = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_655" = alloca i32, align 4
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
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %6 = load ptr, ptr %__uv_async_frame3, align 8
  %7 = load ptr, ptr %__uv_async_input4, align 8
  %8 = getelementptr i8, ptr %6, i64 0
  %9 = load i64, ptr %8, align 4
  switch i64 %9, label %async.resume.invalid [
  ]

async.resume.invalid:                             ; preds = %poison.cont
  ret void

async.resume.start:                               ; No predecessors!
  %10 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %11 = icmp ne i8 %10, 0
  br i1 %11, label %poison.take6, label %poison.cont7

poison.take6:                                     ; preds = %async.resume.start
  %12 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %12, align 1
  %13 = getelementptr i8, ptr %12, i64 4
  store i32 10, ptr %13, align 4
  ret void

poison.cont7:                                     ; preds = %async.resume.start
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont7
  %14 = load ptr, ptr %__panic5, align 8
  %15 = load i8, ptr %14, align 1
  %16 = icmp ne i8 %15, 0
  br i1 %16, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont7
  %17 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %17, align 1
  %18 = getelementptr i8, ptr %17, i64 4
  store i32 4, ptr %18, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %19 = load ptr, ptr %__panic5, align 8
  %20 = getelementptr i8, ptr %19, i64 0
  %21 = load i8, ptr %20, align 1
  %22 = load ptr, ptr %__panic5, align 8
  %23 = getelementptr i8, ptr %22, i64 4
  %24 = load i32, ptr %23, align 4
  ret void

panic.cont:                                       ; preds = %check_ok
  %25 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 0, i32 1)
  %26 = extractvalue { i32, i1 } %25, 0
  %27 = extractvalue { i32, i1 } %25, 1
  %28 = freeze i32 %26
  br i1 %27, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  store i32 %28, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_655", align 4
  %29 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %30 = icmp ne i8 %29, 0
  br i1 %30, label %poison.take8, label %poison.cont9

op_fail:                                          ; preds = %panic.cont
  %31 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %31, align 1
  %32 = getelementptr i8, ptr %31, i64 4
  store i32 4, ptr %32, align 4
  ret void

poison.take8:                                     ; preds = %op_ok
  %33 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %33, align 1
  %34 = getelementptr i8, ptr %33, i64 4
  store i32 10, ptr %34, align 4
  ret void

poison.cont9:                                     ; preds = %op_ok
  call void @ExpressionSemantics_x3a_x3aasyncCompositionArtifactCompletes(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %sret, ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3aasyncCompositionCombinatorArtifactReference$tmp$call_ref_tmp_655", ptr noundef nonnull align 8 dereferenceable(8) %__panic5)
  %35 = load ptr, ptr %__panic5, align 8
  %36 = load i8, ptr %35, align 1
  %37 = icmp ne i8 %36, 0
  br i1 %37, label %panic.take10, label %panic.cont11

panic.take10:                                     ; preds = %poison.cont9
  %38 = load ptr, ptr %__panic5, align 8
  %39 = getelementptr i8, ptr %38, i64 0
  %40 = load i8, ptr %39, align 1
  %41 = load ptr, ptr %__panic5, align 8
  %42 = getelementptr i8, ptr %41, i64 4
  %43 = load i32, ptr %42, align 4
  ret void

panic.cont11:                                     ; preds = %poison.cont9
  %44 = load ptr, ptr %__uv_async_out2, align 8
  call void @llvm.memmove.p0.p0.i64(ptr align 8 %44, ptr align 8 %sret, i64 24, i1 false)
  ret void
}

define { i8, [3 x i8], [4 x i8], [0 x i32] } @ExpressionSemantics_x3a_x3aasyncCompositionUntilArtifactReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %coerce_bits5 = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %sret = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %2 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %3 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %4 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %5 = alloca i32, align 4
  %if.result = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionUntilArtifactReference$tmp$until_recv_683" = alloca { i32, [0 x i32] }, align 4
  %coerce_bits2 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionUntilArtifactReference$tmp$__uv_implicit_key_scope_6781" = alloca ptr, align 8
  %"ExpressionSemantics_x3a_x3aasyncCompositionUntilArtifactReference$tmp$__uv_implicit_key_scope_678" = alloca ptr, align 8
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %cell = alloca { i32, [0 x i32] }, align 4
  %6 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %7 = icmp ne i8 %6, 0
  br i1 %7, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 10, ptr %9, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

poison.cont:                                      ; preds = %entry
  %10 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %10, align 1
  %11 = getelementptr i8, ptr %10, i64 4
  store i32 0, ptr %11, align 4
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 4, i1 false)
  store i32 113, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %cell, ptr align 4 %aggregate.literal, i64 4, i1 false)
  fence seq_cst
  fence seq_cst
  %12 = call ptr @uv_key_scope_enter()
  store ptr %12, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionUntilArtifactReference$tmp$__uv_implicit_key_scope_6781", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fD98B1E90FD653435, i64 4 }, ptr %coerce_bits, align 8
  %13 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %13, i8 0)
  %14 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionUntilArtifactReference$tmp$__uv_implicit_key_scope_6781", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fD98B1E90FD653435, i64 4 }, ptr %coerce_bits2, align 8
  %15 = load { i64, i64 }, ptr %coerce_bits2, align 1
  call void @uv_key_acquire(ptr %14, { i64, i64 } %15, i8 0)
  fence seq_cst
  %16 = load { i32, [0 x i32] }, ptr %cell, align 4
  store { i32, [0 x i32] } %16, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionUntilArtifactReference$tmp$until_recv_683", align 4
  %17 = call i8 @ExpressionSemantics_x5fx3a_x5fx3aasyncCompositionUntilArtifactReference_x3a_x3a_x5fclosure0(ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3aasyncCompositionUntilArtifactReference$tmp$until_recv_683", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %18 = load ptr, ptr %__panic, align 8
  %19 = load i8, ptr %18, align 1
  %20 = icmp ne i8 %19, 0
  br i1 %20, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont
  %21 = load ptr, ptr %__panic, align 8
  %22 = getelementptr i8, ptr %21, i64 0
  %23 = load i8, ptr %22, align 1
  %24 = load ptr, ptr %__panic, align 8
  %25 = getelementptr i8, ptr %24, i64 4
  %26 = load i32, ptr %25, align 4
  %27 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionUntilArtifactReference$tmp$__uv_implicit_key_scope_6781", align 8
  call void @uv_key_scope_exit(ptr %27)
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

panic.cont:                                       ; preds = %poison.cont
  %28 = icmp ne i8 %17, 0
  %29 = icmp ne i8 %17, 0
  br i1 %29, label %if.then, label %if.else

if.then:                                          ; preds = %panic.cont
  %30 = call i32 @ExpressionSemantics_x5fx3a_x5fx3aasyncCompositionUntilArtifactReference_x3a_x3a_x5fclosure1(ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3aasyncCompositionUntilArtifactReference$tmp$until_recv_683", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %31 = load ptr, ptr %__panic, align 8
  %32 = load i8, ptr %31, align 1
  %33 = icmp ne i8 %32, 0
  br i1 %33, label %panic.take3, label %panic.cont4

if.else:                                          ; preds = %panic.cont
  %34 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %34, align 1
  %35 = getelementptr i8, ptr %34, i64 4
  store i32 13, ptr %35, align 4
  %36 = load ptr, ptr %__panic, align 8
  %37 = getelementptr i8, ptr %36, i64 0
  %38 = load i8, ptr %37, align 1
  %39 = load ptr, ptr %__panic, align 8
  %40 = getelementptr i8, ptr %39, i64 4
  %41 = load i32, ptr %40, align 4
  %42 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionUntilArtifactReference$tmp$__uv_implicit_key_scope_6781", align 8
  call void @uv_key_scope_exit(ptr %42)
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

if.merge:                                         ; preds = %panic.cont4
  fence seq_cst
  %43 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %if.result, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %43, ptr %1, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %2, align 4
  %44 = load ptr, ptr %__panic, align 8
  br label %sync.loop

panic.take3:                                      ; preds = %if.then
  %45 = load ptr, ptr %__panic, align 8
  %46 = getelementptr i8, ptr %45, i64 0
  %47 = load i8, ptr %46, align 1
  %48 = load ptr, ptr %__panic, align 8
  %49 = getelementptr i8, ptr %48, i64 4
  %50 = load i32, ptr %49, align 4
  %51 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionUntilArtifactReference$tmp$__uv_implicit_key_scope_6781", align 8
  call void @uv_key_scope_exit(ptr %51)
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

panic.cont4:                                      ; preds = %if.then
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %4, align 4
  store i8 1, ptr %4, align 1
  %52 = getelementptr i8, ptr %4, i64 8
  store i32 %30, ptr %5, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %52, ptr align 1 %5, i64 4, i1 false)
  %53 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %4, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %53, ptr %if.result, align 1
  br label %if.merge

sync.loop:                                        ; preds = %sync.suspended, %if.merge
  %54 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  %55 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %54, 0
  switch i8 %55, label %sync.fallback [
    i8 0, label %sync.suspended
    i8 1, label %sync.completed
  ]

sync.suspended:                                   ; preds = %sync.loop
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aresume(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8] }) align 1 dereferenceable(24) %sret, ptr %1, ptr null, ptr %44)
  %56 = load { i8, [7 x i8], [16 x i8] }, ptr %sret, align 1
  store { i8, [7 x i8], [16 x i8] } %56, ptr %coerce_bits5, align 1
  %57 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %coerce_bits5, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %57, ptr %1, align 4
  %58 = load i8, ptr %44, align 1
  %59 = icmp ne i8 %58, 0
  br i1 %59, label %sync.panic, label %sync.loop

sync.completed:                                   ; preds = %sync.loop
  %60 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %60, ptr %3, align 4
  %61 = getelementptr i8, ptr %3, i64 8
  %62 = load i32, ptr %61, align 1
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %0, align 4
  store i8 0, ptr %0, align 1
  %63 = getelementptr i8, ptr %0, i64 4
  %64 = getelementptr i8, ptr %63, i64 0
  store i32 %62, ptr %64, align 1
  %65 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %0, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %65, ptr %2, align 4
  br label %sync.merge

sync.fallback:                                    ; preds = %sync.loop
  %66 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %2, align 4
  br label %sync.merge

sync.panic:                                       ; preds = %sync.suspended
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %2, align 4
  br label %sync.merge

sync.merge:                                       ; preds = %sync.panic, %sync.fallback, %sync.completed
  %67 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %2, align 4
  %68 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncCompositionUntilArtifactReference$tmp$__uv_implicit_key_scope_6781", align 8
  call void @uv_key_scope_exit(ptr %68)
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } %67
}

define internal i8 @ExpressionSemantics_x5fx3a_x5fx3aasyncCompositionUntilArtifactReference_x3a_x3a_x5fclosure0(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %10 = getelementptr i8, ptr %value, i64 0
  %11 = load i32, ptr %10, align 1
  %12 = icmp eq i32 %11, 113
  %13 = zext i1 %12 to i8
  %14 = icmp ne i8 %13, 0
  %15 = zext i1 %14 to i8
  ret i8 %15
}

define internal i32 @ExpressionSemantics_x5fx3a_x5fx3aasyncCompositionUntilArtifactReference_x3a_x3a_x5fclosure1(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %15 = getelementptr i8, ptr %14, i64 0
  %16 = load i8, ptr %15, align 1
  %17 = load ptr, ptr %__panic, align 8
  %18 = getelementptr i8, ptr %17, i64 4
  %19 = load i32, ptr %18, align 4
  %20 = load ptr, ptr %__panic, align 8
  %21 = getelementptr i8, ptr %20, i64 4
  %22 = load i32, ptr %21, align 4
  ret i32 %22

panic.cont:                                       ; preds = %check_ok
  %23 = getelementptr i8, ptr %value, i64 0
  %24 = load i32, ptr %23, align 1
  %25 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %24, i32 1)
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

define void @ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseArtifactReference(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %0, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %2 = alloca i32, align 4
  %3 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %4 = alloca i32, align 4
  %coerce_bits1 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %shared_value = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseArtifactReference$tmp$__uv_key_scope_732" = alloca ptr, align 8
  %__bind_116_resumed = alloca i32, align 4
  %5 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %6 = icmp ne i8 %5, 0
  br i1 %6, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %7 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %7, align 1
  %8 = getelementptr i8, ptr %7, i64 4
  store i32 10, ptr %8, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %9 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 0, ptr %10, align 4
  %11 = load i32, ptr %value, align 1
  store i32 %11, ptr %shared_value, align 4
  %12 = call ptr @uv_key_scope_enter()
  store ptr %12, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseArtifactReference$tmp$__uv_key_scope_732", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits, align 8
  %13 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %13, i8 0)
  %14 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseArtifactReference$tmp$__uv_key_scope_732", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits1, align 8
  %15 = load { i64, i64 }, ptr %coerce_bits1, align 1
  call void @uv_key_acquire(ptr %14, { i64, i64 } %15, i8 0)
  %16 = load i32, ptr %value, align 4
  %17 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aalloc_x5fframe(i64 48, i64 8)
  %18 = getelementptr i8, ptr %17, i64 0
  store i64 0, ptr %18, align 4
  %19 = getelementptr i8, ptr %17, i64 8
  store ptr @"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseArtifactReference$resume", ptr %19, align 8
  %20 = getelementptr i8, ptr %17, i64 16
  store ptr null, ptr %20, align 8
  %21 = getelementptr i8, ptr %17, i64 24
  store ptr null, ptr %21, align 8
  %22 = call ptr @uv_key_release_all()
  %23 = getelementptr i8, ptr %17, i64 24
  store ptr %22, ptr %23, align 8
  %24 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseArtifactReference$tmp$__uv_key_scope_732", align 8
  %25 = getelementptr i8, ptr %17, i64 32
  store ptr %24, ptr %25, align 8
  %26 = load i32, ptr %__bind_116_resumed, align 4
  %27 = getelementptr i8, ptr %17, i64 40
  store i32 %26, ptr %27, align 4
  %28 = getelementptr i8, ptr %17, i64 0
  store i64 1, ptr %28, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %3, align 4
  store i8 0, ptr %3, align 1
  %29 = getelementptr i8, ptr %3, i64 8
  store i32 %16, ptr %4, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %29, ptr align 1 %4, i64 4, i1 false)
  %30 = getelementptr i8, ptr %29, i64 8
  store ptr %17, ptr %30, align 8
  %31 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %3, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %31, ptr %0, align 4
  ret void

yield.cont:                                       ; No predecessors!
  store i32 0, ptr %__bind_116_resumed, align 4
  %32 = load i32, ptr %__bind_116_resumed, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %33 = getelementptr i8, ptr %1, i64 8
  store i32 %32, ptr %2, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %33, ptr align 1 %2, i64 4, i1 false)
  %34 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  %35 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseArtifactReference$tmp$__uv_key_scope_732", align 8
  call void @uv_key_scope_exit(ptr %35)
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %34, ptr %0, align 4
  ret void
}

define internal void @"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseArtifactReference$resume"(ptr %__uv_host_env, ptr %__uv_async_out, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_frame, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_input, ptr %__panic) {
entry:
  %0 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %1 = alloca i32, align 4
  %yield_input = alloca i32, align 4
  %2 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %3 = alloca i32, align 4
  %coerce_bits6 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %shared_value = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseArtifactReference$tmp$__uv_key_scope_732" = alloca ptr, align 8
  %__bind_116_resumed = alloca i32, align 4
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
  %4 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %5 = icmp ne i8 %4, 0
  br i1 %5, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %6 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %6, align 1
  %7 = getelementptr i8, ptr %6, i64 4
  store i32 10, ptr %7, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic5, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  %10 = load ptr, ptr %__uv_async_frame3, align 8
  %11 = load ptr, ptr %__uv_async_input4, align 8
  %12 = getelementptr i8, ptr %10, i64 32
  %13 = load ptr, ptr %12, align 8
  store ptr %13, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseArtifactReference$tmp$__uv_key_scope_732", align 8
  %14 = getelementptr i8, ptr %10, i64 40
  %15 = load i32, ptr %14, align 4
  store i32 %15, ptr %__bind_116_resumed, align 4
  %16 = getelementptr i8, ptr %10, i64 0
  %17 = load i64, ptr %16, align 4
  switch i64 %17, label %async.resume.invalid [
    i64 1, label %yield.resume.1
  ]

async.resume.invalid:                             ; preds = %poison.cont
  ret void

async.resume.start:                               ; No predecessors!
  store i32 0, ptr %shared_value, align 4
  %18 = call ptr @uv_key_scope_enter()
  store ptr %18, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseArtifactReference$tmp$__uv_key_scope_732", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits, align 8
  %19 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %19, i8 0)
  %20 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseArtifactReference$tmp$__uv_key_scope_732", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits6, align 8
  %21 = load { i64, i64 }, ptr %coerce_bits6, align 1
  call void @uv_key_acquire(ptr %20, { i64, i64 } %21, i8 0)
  %22 = call ptr @uv_key_release_all()
  %23 = getelementptr i8, ptr %10, i64 24
  store ptr %22, ptr %23, align 8
  %24 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseArtifactReference$tmp$__uv_key_scope_732", align 8
  %25 = getelementptr i8, ptr %10, i64 32
  store ptr %24, ptr %25, align 8
  %26 = load i32, ptr %__bind_116_resumed, align 4
  %27 = getelementptr i8, ptr %10, i64 40
  store i32 %26, ptr %27, align 4
  %28 = getelementptr i8, ptr %10, i64 0
  store i64 1, ptr %28, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %2, align 4
  store i8 0, ptr %2, align 1
  %29 = getelementptr i8, ptr %2, i64 8
  store i32 0, ptr %3, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %29, ptr align 1 %3, i64 4, i1 false)
  %30 = getelementptr i8, ptr %29, i64 8
  store ptr %10, ptr %30, align 8
  %31 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %2, align 4
  %32 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %31, ptr %32, align 4
  ret void

yield.cont:                                       ; preds = %yield.resume.1
  %33 = getelementptr i8, ptr %10, i64 24
  %34 = load ptr, ptr %33, align 8
  call void @uv_key_reacquire(ptr %34)
  %35 = getelementptr i8, ptr %10, i64 24
  store ptr null, ptr %35, align 8
  %36 = load i32, ptr %11, align 1
  store i32 %36, ptr %yield_input, align 4
  %37 = load i32, ptr %yield_input, align 1
  store i32 %37, ptr %__bind_116_resumed, align 4
  %38 = load i32, ptr %__bind_116_resumed, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %39 = getelementptr i8, ptr %0, i64 8
  store i32 %38, ptr %1, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %39, ptr align 1 %1, i64 4, i1 false)
  %40 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %0, align 4
  %41 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseArtifactReference$tmp$__uv_key_scope_732", align 8
  call void @uv_key_scope_exit(ptr %41)
  %42 = load ptr, ptr %__uv_async_out2, align 8
  %43 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %40, ptr %43, align 4
  ret void

yield.resume.1:                                   ; preds = %poison.cont
  br label %yield.cont
}

define void @ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %0, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  %4 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %5 = alloca i32, align 4
  %6 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %7 = alloca i8, align 1
  %coerce_bits1 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %shared_value = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$__uv_key_scope_748" = alloca ptr, align 8
  %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759" = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %__bind_120_resumed = alloca i32, align 4
  %8 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %9 = icmp ne i8 %8, 0
  br i1 %9, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %10 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %10, align 1
  %11 = getelementptr i8, ptr %10, i64 4
  store i32 10, ptr %11, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %12 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %12, align 1
  %13 = getelementptr i8, ptr %12, i64 4
  store i32 0, ptr %13, align 4
  %14 = load i32, ptr %value, align 1
  store i32 %14, ptr %shared_value, align 4
  %15 = call ptr @uv_key_scope_enter()
  store ptr %15, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$__uv_key_scope_748", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits, align 8
  %16 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %16, i8 0)
  %17 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$__uv_key_scope_748", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits1, align 8
  %18 = load { i64, i64 }, ptr %coerce_bits1, align 1
  call void @uv_key_acquire(ptr %17, { i64, i64 } %18, i8 0)
  %19 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %20 = icmp ne i8 %19, 0
  br i1 %20, label %poison.take2, label %poison.cont3

poison.take2:                                     ; preds = %poison.cont
  %21 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %21, align 1
  %22 = getelementptr i8, ptr %21, i64 4
  store i32 10, ptr %22, align 4
  ret void

poison.cont3:                                     ; preds = %poison.cont
  %23 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %24 = icmp ne i8 %23, 0
  br i1 %24, label %poison.take4, label %poison.cont5

poison.take4:                                     ; preds = %poison.cont3
  %25 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %25, align 1
  %26 = getelementptr i8, ptr %25, i64 4
  store i32 10, ptr %26, align 4
  ret void

poison.cont5:                                     ; preds = %poison.cont3
  call void @ExpressionSemantics_x3a_x3aasyncKeyArtifactSuspends(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %27 = load ptr, ptr %__panic, align 8
  %28 = load i8, ptr %27, align 1
  %29 = icmp ne i8 %28, 0
  br i1 %29, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont5
  %30 = load ptr, ptr %__panic, align 8
  %31 = getelementptr i8, ptr %30, i64 0
  %32 = load i8, ptr %31, align 1
  %33 = load ptr, ptr %__panic, align 8
  %34 = getelementptr i8, ptr %33, i64 4
  %35 = load i32, ptr %34, align 4
  %36 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$__uv_key_scope_748", align 8
  call void @uv_key_scope_exit(ptr %36)
  ret void

panic.cont:                                       ; preds = %poison.cont5
  store i32 0, ptr %3, align 4
  %37 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %37, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", align 4
  br label %yield_from.loop

yield_from.loop:                                  ; preds = %panic.cont
  %38 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", align 4
  %39 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %38, 0
  switch i8 %39, label %yield_from.fallback [
    i8 0, label %yield_from.suspended
    i8 1, label %yield_from.completed
    i8 2, label %yield_from.failed
  ]

yield_from.suspended:                             ; preds = %yield_from.loop
  %40 = getelementptr i8, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", i64 8
  %41 = load i32, ptr %40, align 1
  %42 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aalloc_x5fframe(i64 72, i64 8)
  %43 = getelementptr i8, ptr %42, i64 0
  store i64 0, ptr %43, align 4
  %44 = getelementptr i8, ptr %42, i64 8
  store ptr @"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$resume", ptr %44, align 8
  %45 = getelementptr i8, ptr %42, i64 16
  store ptr null, ptr %45, align 8
  %46 = getelementptr i8, ptr %42, i64 24
  store ptr null, ptr %46, align 8
  %47 = call ptr @uv_key_release_all()
  %48 = getelementptr i8, ptr %42, i64 24
  store ptr %47, ptr %48, align 8
  %49 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$__uv_key_scope_748", align 8
  %50 = getelementptr i8, ptr %42, i64 32
  store ptr %49, ptr %50, align 8
  %51 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", align 4
  %52 = getelementptr i8, ptr %42, i64 40
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %51, ptr %52, align 8
  %53 = load i32, ptr %__bind_120_resumed, align 4
  %54 = getelementptr i8, ptr %42, i64 64
  store i32 %53, ptr %54, align 4
  %55 = getelementptr i8, ptr %42, i64 0
  store i64 1, ptr %55, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %4, align 4
  store i8 0, ptr %4, align 1
  %56 = getelementptr i8, ptr %4, i64 8
  store i32 %41, ptr %5, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %56, ptr align 1 %5, i64 4, i1 false)
  %57 = getelementptr i8, ptr %56, i64 8
  store ptr %42, ptr %57, align 8
  %58 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %4, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %58, ptr %0, align 4
  ret void

yield_from.completed:                             ; preds = %yield_from.loop
  %59 = getelementptr i8, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", i64 8
  %60 = load i32, ptr %59, align 1
  store i32 %60, ptr %3, align 4
  br label %yield_from.cont

yield_from.failed:                                ; preds = %yield_from.loop
  %61 = getelementptr i8, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", i64 8
  %62 = load i8, ptr %61, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %6, align 4
  store i8 2, ptr %6, align 1
  %63 = getelementptr i8, ptr %6, i64 8
  store i8 %62, ptr %7, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %63, ptr align 1 %7, i64 1, i1 false)
  %64 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %6, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %64, ptr %0, align 4
  ret void

yield_from.fallback:                              ; preds = %yield_from.loop
  %65 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", align 4
  store i32 0, ptr %3, align 4
  br label %yield_from.cont

yield_from.panic:                                 ; No predecessors!
  store i32 0, ptr %3, align 4
  br label %yield_from.cont

yield_from.cont:                                  ; preds = %yield_from.panic, %yield_from.fallback, %yield_from.completed
  %66 = load i32, ptr %3, align 4
  store i32 %66, ptr %__bind_120_resumed, align 4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %yield_from.cont
  %67 = load ptr, ptr %__panic, align 8
  %68 = load i8, ptr %67, align 1
  %69 = icmp ne i8 %68, 0
  br i1 %69, label %panic.take6, label %panic.cont7

check_fail:                                       ; preds = %yield_from.cont
  %70 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %70, align 1
  %71 = getelementptr i8, ptr %70, i64 4
  store i32 4, ptr %71, align 4
  br label %check_ok

panic.take6:                                      ; preds = %check_ok
  %72 = load ptr, ptr %__panic, align 8
  %73 = getelementptr i8, ptr %72, i64 0
  %74 = load i8, ptr %73, align 1
  %75 = load ptr, ptr %__panic, align 8
  %76 = getelementptr i8, ptr %75, i64 4
  %77 = load i32, ptr %76, align 4
  %78 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$__uv_key_scope_748", align 8
  call void @uv_key_scope_exit(ptr %78)
  ret void

panic.cont7:                                      ; preds = %check_ok
  %79 = load i32, ptr %__bind_120_resumed, align 4
  %80 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %79, i32 1)
  %81 = extractvalue { i32, i1 } %80, 0
  %82 = extractvalue { i32, i1 } %80, 1
  %83 = freeze i32 %81
  br i1 %82, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont7
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %84 = getelementptr i8, ptr %1, i64 8
  store i32 %83, ptr %2, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %84, ptr align 1 %2, i64 4, i1 false)
  %85 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  %86 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$__uv_key_scope_748", align 8
  call void @uv_key_scope_exit(ptr %86)
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %85, ptr %0, align 4
  ret void

op_fail:                                          ; preds = %panic.cont7
  %87 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %87, align 1
  %88 = getelementptr i8, ptr %87, i64 4
  store i32 4, ptr %88, align 4
  ret void
}

define internal void @"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$resume"(ptr %__uv_host_env, ptr %__uv_async_out, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_frame, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_input, ptr %__panic) {
entry:
  %0 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %1 = alloca i32, align 4
  %coerce_bits11 = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %sret = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %2 = alloca i32, align 4
  %3 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %4 = alloca i32, align 4
  %5 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %6 = alloca i8, align 1
  %__bind_117_value.resume_prelude = alloca i32, align 4
  %coerce_bits6 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %shared_value = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$__uv_key_scope_748" = alloca ptr, align 8
  %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759" = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %__bind_120_resumed = alloca i32, align 4
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
  %7 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %8 = icmp ne i8 %7, 0
  br i1 %8, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %9 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 10, ptr %10, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %11 = load ptr, ptr %__panic5, align 8
  store i8 0, ptr %11, align 1
  %12 = getelementptr i8, ptr %11, i64 4
  store i32 0, ptr %12, align 4
  %13 = load ptr, ptr %__uv_async_frame3, align 8
  %14 = load ptr, ptr %__uv_async_input4, align 8
  %15 = getelementptr i8, ptr %13, i64 32
  %16 = load ptr, ptr %15, align 8
  store ptr %16, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$__uv_key_scope_748", align 8
  %17 = getelementptr i8, ptr %13, i64 40
  %18 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %17, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %18, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", align 4
  %19 = getelementptr i8, ptr %13, i64 64
  %20 = load i32, ptr %19, align 4
  store i32 %20, ptr %__bind_120_resumed, align 4
  %21 = getelementptr i8, ptr %13, i64 0
  %22 = load i64, ptr %21, align 4
  switch i64 %22, label %async.resume.invalid [
    i64 1, label %yield_from.resume.1
  ]

async.resume.invalid:                             ; preds = %poison.cont
  ret void

async.resume.start:                               ; No predecessors!
  store i32 0, ptr %shared_value, align 4
  %23 = call ptr @uv_key_scope_enter()
  store ptr %23, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$__uv_key_scope_748", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits, align 8
  %24 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %24, i8 0)
  %25 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$__uv_key_scope_748", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits6, align 8
  %26 = load { i64, i64 }, ptr %coerce_bits6, align 1
  call void @uv_key_acquire(ptr %25, { i64, i64 } %26, i8 0)
  %27 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %28 = icmp ne i8 %27, 0
  br i1 %28, label %poison.take7, label %poison.cont8

poison.take7:                                     ; preds = %async.resume.start
  %29 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %29, align 1
  %30 = getelementptr i8, ptr %29, i64 4
  store i32 10, ptr %30, align 4
  ret void

poison.cont8:                                     ; preds = %async.resume.start
  store i32 0, ptr %__bind_117_value.resume_prelude, align 4
  %31 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %32 = icmp ne i8 %31, 0
  br i1 %32, label %poison.take9, label %poison.cont10

poison.take9:                                     ; preds = %poison.cont8
  %33 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %33, align 1
  %34 = getelementptr i8, ptr %33, i64 4
  store i32 10, ptr %34, align 4
  ret void

poison.cont10:                                    ; preds = %poison.cont8
  call void @ExpressionSemantics_x3a_x3aasyncKeyArtifactSuspends(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", ptr noundef nonnull align 4 dereferenceable(4) %__bind_117_value.resume_prelude, ptr noundef nonnull align 8 dereferenceable(8) %__panic5)
  %35 = load ptr, ptr %__panic5, align 8
  %36 = load i8, ptr %35, align 1
  %37 = icmp ne i8 %36, 0
  br i1 %37, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont10
  %38 = load ptr, ptr %__panic5, align 8
  %39 = getelementptr i8, ptr %38, i64 0
  %40 = load i8, ptr %39, align 1
  %41 = load ptr, ptr %__panic5, align 8
  %42 = getelementptr i8, ptr %41, i64 4
  %43 = load i32, ptr %42, align 4
  %44 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$__uv_key_scope_748", align 8
  call void @uv_key_scope_exit(ptr %44)
  ret void

panic.cont:                                       ; preds = %poison.cont10
  store i32 0, ptr %2, align 4
  %45 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %45, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", align 4
  br label %yield_from.loop

yield_from.loop:                                  ; preds = %yield_from.resume.1, %panic.cont
  %46 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", align 4
  %47 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %46, 0
  switch i8 %47, label %yield_from.fallback [
    i8 0, label %yield_from.suspended
    i8 1, label %yield_from.completed
    i8 2, label %yield_from.failed
  ]

yield_from.suspended:                             ; preds = %yield_from.loop
  %48 = getelementptr i8, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", i64 8
  %49 = load i32, ptr %48, align 1
  %50 = call ptr @uv_key_release_all()
  %51 = getelementptr i8, ptr %13, i64 24
  store ptr %50, ptr %51, align 8
  %52 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$__uv_key_scope_748", align 8
  %53 = getelementptr i8, ptr %13, i64 32
  store ptr %52, ptr %53, align 8
  %54 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", align 4
  %55 = getelementptr i8, ptr %13, i64 40
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %54, ptr %55, align 8
  %56 = load i32, ptr %__bind_120_resumed, align 4
  %57 = getelementptr i8, ptr %13, i64 64
  store i32 %56, ptr %57, align 4
  %58 = getelementptr i8, ptr %13, i64 0
  store i64 1, ptr %58, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %3, align 4
  store i8 0, ptr %3, align 1
  %59 = getelementptr i8, ptr %3, i64 8
  store i32 %49, ptr %4, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %59, ptr align 1 %4, i64 4, i1 false)
  %60 = getelementptr i8, ptr %59, i64 8
  store ptr %13, ptr %60, align 8
  %61 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %3, align 4
  %62 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %61, ptr %62, align 4
  ret void

yield_from.completed:                             ; preds = %yield_from.loop
  %63 = getelementptr i8, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", i64 8
  %64 = load i32, ptr %63, align 1
  store i32 %64, ptr %2, align 4
  br label %yield_from.cont

yield_from.failed:                                ; preds = %yield_from.loop
  %65 = getelementptr i8, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", i64 8
  %66 = load i8, ptr %65, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %5, align 4
  store i8 2, ptr %5, align 1
  %67 = getelementptr i8, ptr %5, i64 8
  store i8 %66, ptr %6, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %67, ptr align 1 %6, i64 1, i1 false)
  %68 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %5, align 4
  %69 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %68, ptr %69, align 4
  ret void

yield_from.fallback:                              ; preds = %yield_from.loop
  %70 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", align 4
  store i32 0, ptr %2, align 4
  br label %yield_from.cont

yield_from.panic:                                 ; preds = %yield_from.resume.1
  store i32 0, ptr %2, align 4
  br label %yield_from.cont

yield_from.cont:                                  ; preds = %yield_from.panic, %yield_from.fallback, %yield_from.completed
  %71 = load i32, ptr %2, align 4
  store i32 %71, ptr %__bind_120_resumed, align 4
  br i1 true, label %check_ok, label %check_fail

yield_from.resume.1:                              ; preds = %poison.cont
  %72 = getelementptr i8, ptr %13, i64 24
  %73 = load ptr, ptr %72, align 8
  call void @uv_key_reacquire(ptr %73)
  %74 = getelementptr i8, ptr %13, i64 24
  store ptr null, ptr %74, align 8
  %75 = load ptr, ptr %__panic5, align 8
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aresume(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8] }) align 1 dereferenceable(24) %sret, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", ptr %14, ptr %75)
  %76 = load { i8, [7 x i8], [16 x i8] }, ptr %sret, align 1
  store { i8, [7 x i8], [16 x i8] } %76, ptr %coerce_bits11, align 1
  %77 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %coerce_bits11, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %77, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$yield_from_source_759", align 4
  %78 = load i8, ptr %75, align 1
  %79 = icmp ne i8 %78, 0
  br i1 %79, label %yield_from.panic, label %yield_from.loop

check_ok:                                         ; preds = %check_fail, %yield_from.cont
  %80 = load ptr, ptr %__panic5, align 8
  %81 = load i8, ptr %80, align 1
  %82 = icmp ne i8 %81, 0
  br i1 %82, label %panic.take12, label %panic.cont13

check_fail:                                       ; preds = %yield_from.cont
  %83 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %83, align 1
  %84 = getelementptr i8, ptr %83, i64 4
  store i32 4, ptr %84, align 4
  br label %check_ok

panic.take12:                                     ; preds = %check_ok
  %85 = load ptr, ptr %__panic5, align 8
  %86 = getelementptr i8, ptr %85, i64 0
  %87 = load i8, ptr %86, align 1
  %88 = load ptr, ptr %__panic5, align 8
  %89 = getelementptr i8, ptr %88, i64 4
  %90 = load i32, ptr %89, align 4
  %91 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$__uv_key_scope_748", align 8
  call void @uv_key_scope_exit(ptr %91)
  ret void

panic.cont13:                                     ; preds = %check_ok
  %92 = load i32, ptr %__bind_120_resumed, align 4
  %93 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %92, i32 1)
  %94 = extractvalue { i32, i1 } %93, 0
  %95 = extractvalue { i32, i1 } %93, 1
  %96 = freeze i32 %94
  br i1 %95, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont13
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %97 = getelementptr i8, ptr %0, i64 8
  store i32 %96, ptr %1, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %97, ptr align 1 %1, i64 4, i1 false)
  %98 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %0, align 4
  %99 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFromArtifactReference$tmp$__uv_key_scope_748", align 8
  call void @uv_key_scope_exit(ptr %99)
  %100 = load ptr, ptr %__uv_async_out2, align 8
  %101 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %98, ptr %101, align 4
  ret void

op_fail:                                          ; preds = %panic.cont13
  %102 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %102, align 1
  %103 = getelementptr i8, ptr %102, i64 4
  store i32 4, ptr %103, align 4
  ret void
}

define void @ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %0, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %1 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  %4 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %5 = alloca i32, align 4
  %6 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %7 = alloca i8, align 1
  %coerce_bits1 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %shared_value = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$__uv_key_scope_784" = alloca ptr, align 8
  %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795" = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %__bind_124_result = alloca i32, align 4
  %8 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %9 = icmp ne i8 %8, 0
  br i1 %9, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %10 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %10, align 1
  %11 = getelementptr i8, ptr %10, i64 4
  store i32 10, ptr %11, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %12 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %12, align 1
  %13 = getelementptr i8, ptr %12, i64 4
  store i32 0, ptr %13, align 4
  %14 = load i32, ptr %value, align 1
  store i32 %14, ptr %shared_value, align 4
  %15 = call ptr @uv_key_scope_enter()
  store ptr %15, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$__uv_key_scope_784", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits, align 8
  %16 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %16, i8 0)
  %17 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$__uv_key_scope_784", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits1, align 8
  %18 = load { i64, i64 }, ptr %coerce_bits1, align 1
  call void @uv_key_acquire(ptr %17, { i64, i64 } %18, i8 0)
  %19 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %20 = icmp ne i8 %19, 0
  br i1 %20, label %poison.take2, label %poison.cont3

poison.take2:                                     ; preds = %poison.cont
  %21 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %21, align 1
  %22 = getelementptr i8, ptr %21, i64 4
  store i32 10, ptr %22, align 4
  ret void

poison.cont3:                                     ; preds = %poison.cont
  %23 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %24 = icmp ne i8 %23, 0
  br i1 %24, label %poison.take4, label %poison.cont5

poison.take4:                                     ; preds = %poison.cont3
  %25 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %25, align 1
  %26 = getelementptr i8, ptr %25, i64 4
  store i32 10, ptr %26, align 4
  ret void

poison.cont5:                                     ; preds = %poison.cont3
  call void @ExpressionSemantics_x3a_x3aasyncKeyArtifactFails(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %27 = load ptr, ptr %__panic, align 8
  %28 = load i8, ptr %27, align 1
  %29 = icmp ne i8 %28, 0
  br i1 %29, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont5
  %30 = load ptr, ptr %__panic, align 8
  %31 = getelementptr i8, ptr %30, i64 0
  %32 = load i8, ptr %31, align 1
  %33 = load ptr, ptr %__panic, align 8
  %34 = getelementptr i8, ptr %33, i64 4
  %35 = load i32, ptr %34, align 4
  %36 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$__uv_key_scope_784", align 8
  call void @uv_key_scope_exit(ptr %36)
  ret void

panic.cont:                                       ; preds = %poison.cont5
  store i32 0, ptr %3, align 4
  %37 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %37, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", align 4
  br label %yield_from.loop

yield_from.loop:                                  ; preds = %panic.cont
  %38 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", align 4
  %39 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %38, 0
  switch i8 %39, label %yield_from.fallback [
    i8 0, label %yield_from.suspended
    i8 1, label %yield_from.completed
    i8 2, label %yield_from.failed
  ]

yield_from.suspended:                             ; preds = %yield_from.loop
  %40 = getelementptr i8, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", i64 8
  %41 = load i32, ptr %40, align 1
  %42 = call ptr @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aalloc_x5fframe(i64 72, i64 8)
  %43 = getelementptr i8, ptr %42, i64 0
  store i64 0, ptr %43, align 4
  %44 = getelementptr i8, ptr %42, i64 8
  store ptr @"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$resume", ptr %44, align 8
  %45 = getelementptr i8, ptr %42, i64 16
  store ptr null, ptr %45, align 8
  %46 = getelementptr i8, ptr %42, i64 24
  store ptr null, ptr %46, align 8
  %47 = call ptr @uv_key_release_all()
  %48 = getelementptr i8, ptr %42, i64 24
  store ptr %47, ptr %48, align 8
  %49 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$__uv_key_scope_784", align 8
  %50 = getelementptr i8, ptr %42, i64 32
  store ptr %49, ptr %50, align 8
  %51 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", align 4
  %52 = getelementptr i8, ptr %42, i64 40
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %51, ptr %52, align 8
  %53 = load i32, ptr %__bind_124_result, align 4
  %54 = getelementptr i8, ptr %42, i64 64
  store i32 %53, ptr %54, align 4
  %55 = getelementptr i8, ptr %42, i64 0
  store i64 1, ptr %55, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %4, align 4
  store i8 0, ptr %4, align 1
  %56 = getelementptr i8, ptr %4, i64 8
  store i32 %41, ptr %5, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %56, ptr align 1 %5, i64 4, i1 false)
  %57 = getelementptr i8, ptr %56, i64 8
  store ptr %42, ptr %57, align 8
  %58 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %4, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %58, ptr %0, align 4
  ret void

yield_from.completed:                             ; preds = %yield_from.loop
  %59 = getelementptr i8, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", i64 8
  %60 = load i32, ptr %59, align 1
  store i32 %60, ptr %3, align 4
  br label %yield_from.cont

yield_from.failed:                                ; preds = %yield_from.loop
  %61 = getelementptr i8, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", i64 8
  %62 = load i8, ptr %61, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %6, align 4
  store i8 2, ptr %6, align 1
  %63 = getelementptr i8, ptr %6, i64 8
  store i8 %62, ptr %7, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %63, ptr align 1 %7, i64 1, i1 false)
  %64 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %6, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %64, ptr %0, align 4
  ret void

yield_from.fallback:                              ; preds = %yield_from.loop
  %65 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", align 4
  store i32 0, ptr %3, align 4
  br label %yield_from.cont

yield_from.panic:                                 ; No predecessors!
  store i32 0, ptr %3, align 4
  br label %yield_from.cont

yield_from.cont:                                  ; preds = %yield_from.panic, %yield_from.fallback, %yield_from.completed
  %66 = load i32, ptr %3, align 4
  store i32 %66, ptr %__bind_124_result, align 4
  %67 = load i32, ptr %__bind_124_result, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %1, align 4
  store i8 1, ptr %1, align 1
  %68 = getelementptr i8, ptr %1, i64 8
  store i32 %67, ptr %2, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %68, ptr align 1 %2, i64 4, i1 false)
  %69 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %1, align 4
  %70 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$__uv_key_scope_784", align 8
  call void @uv_key_scope_exit(ptr %70)
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %69, ptr %0, align 4
  ret void
}

define internal void @"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$resume"(ptr %__uv_host_env, ptr %__uv_async_out, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_frame, ptr noundef nonnull align 1 dereferenceable(1) %__uv_async_input, ptr %__panic) {
entry:
  %0 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %1 = alloca i32, align 4
  %coerce_bits11 = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %sret = alloca { i8, [7 x i8], [16 x i8] }, align 1
  %2 = alloca i32, align 4
  %3 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %4 = alloca i32, align 4
  %5 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %6 = alloca i8, align 1
  %__bind_121_value.resume_prelude = alloca i32, align 4
  %coerce_bits6 = alloca { ptr, i64 }, align 8
  %coerce_bits = alloca { ptr, i64 }, align 8
  %shared_value = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$__uv_key_scope_784" = alloca ptr, align 8
  %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795" = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %__bind_124_result = alloca i32, align 4
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
  %7 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %8 = icmp ne i8 %7, 0
  br i1 %8, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %9 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 10, ptr %10, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %11 = load ptr, ptr %__panic5, align 8
  store i8 0, ptr %11, align 1
  %12 = getelementptr i8, ptr %11, i64 4
  store i32 0, ptr %12, align 4
  %13 = load ptr, ptr %__uv_async_frame3, align 8
  %14 = load ptr, ptr %__uv_async_input4, align 8
  %15 = getelementptr i8, ptr %13, i64 32
  %16 = load ptr, ptr %15, align 8
  store ptr %16, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$__uv_key_scope_784", align 8
  %17 = getelementptr i8, ptr %13, i64 40
  %18 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %17, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %18, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", align 4
  %19 = getelementptr i8, ptr %13, i64 64
  %20 = load i32, ptr %19, align 4
  store i32 %20, ptr %__bind_124_result, align 4
  %21 = getelementptr i8, ptr %13, i64 0
  %22 = load i64, ptr %21, align 4
  switch i64 %22, label %async.resume.invalid [
    i64 1, label %yield_from.resume.1
  ]

async.resume.invalid:                             ; preds = %poison.cont
  ret void

async.resume.start:                               ; No predecessors!
  store i32 0, ptr %shared_value, align 4
  %23 = call ptr @uv_key_scope_enter()
  store ptr %23, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$__uv_key_scope_784", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits, align 8
  %24 = load { i64, i64 }, ptr %coerce_bits, align 1
  call void @uv_key_check_conflict({ i64, i64 } %24, i8 0)
  %25 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$__uv_key_scope_784", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5fE98D14EAB37216DE, i64 12 }, ptr %coerce_bits6, align 8
  %26 = load { i64, i64 }, ptr %coerce_bits6, align 1
  call void @uv_key_acquire(ptr %25, { i64, i64 } %26, i8 0)
  %27 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %28 = icmp ne i8 %27, 0
  br i1 %28, label %poison.take7, label %poison.cont8

poison.take7:                                     ; preds = %async.resume.start
  %29 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %29, align 1
  %30 = getelementptr i8, ptr %29, i64 4
  store i32 10, ptr %30, align 4
  ret void

poison.cont8:                                     ; preds = %async.resume.start
  store i32 0, ptr %__bind_121_value.resume_prelude, align 4
  %31 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %32 = icmp ne i8 %31, 0
  br i1 %32, label %poison.take9, label %poison.cont10

poison.take9:                                     ; preds = %poison.cont8
  %33 = load ptr, ptr %__panic5, align 8
  store i8 1, ptr %33, align 1
  %34 = getelementptr i8, ptr %33, i64 4
  store i32 10, ptr %34, align 4
  ret void

poison.cont10:                                    ; preds = %poison.cont8
  call void @ExpressionSemantics_x3a_x3aasyncKeyArtifactFails(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8], [0 x i64] }) align 8 dereferenceable(24) %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", ptr noundef nonnull align 4 dereferenceable(4) %__bind_121_value.resume_prelude, ptr noundef nonnull align 8 dereferenceable(8) %__panic5)
  %35 = load ptr, ptr %__panic5, align 8
  %36 = load i8, ptr %35, align 1
  %37 = icmp ne i8 %36, 0
  br i1 %37, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont10
  %38 = load ptr, ptr %__panic5, align 8
  %39 = getelementptr i8, ptr %38, i64 0
  %40 = load i8, ptr %39, align 1
  %41 = load ptr, ptr %__panic5, align 8
  %42 = getelementptr i8, ptr %41, i64 4
  %43 = load i32, ptr %42, align 4
  %44 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$__uv_key_scope_784", align 8
  call void @uv_key_scope_exit(ptr %44)
  ret void

panic.cont:                                       ; preds = %poison.cont10
  store i32 0, ptr %2, align 4
  %45 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %45, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", align 4
  br label %yield_from.loop

yield_from.loop:                                  ; preds = %yield_from.resume.1, %panic.cont
  %46 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", align 4
  %47 = extractvalue { i8, [7 x i8], [16 x i8], [0 x i64] } %46, 0
  switch i8 %47, label %yield_from.fallback [
    i8 0, label %yield_from.suspended
    i8 1, label %yield_from.completed
    i8 2, label %yield_from.failed
  ]

yield_from.suspended:                             ; preds = %yield_from.loop
  %48 = getelementptr i8, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", i64 8
  %49 = load i32, ptr %48, align 1
  %50 = call ptr @uv_key_release_all()
  %51 = getelementptr i8, ptr %13, i64 24
  store ptr %50, ptr %51, align 8
  %52 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$__uv_key_scope_784", align 8
  %53 = getelementptr i8, ptr %13, i64 32
  store ptr %52, ptr %53, align 8
  %54 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", align 4
  %55 = getelementptr i8, ptr %13, i64 40
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %54, ptr %55, align 8
  %56 = load i32, ptr %__bind_124_result, align 4
  %57 = getelementptr i8, ptr %13, i64 64
  store i32 %56, ptr %57, align 4
  %58 = getelementptr i8, ptr %13, i64 0
  store i64 1, ptr %58, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %3, align 4
  store i8 0, ptr %3, align 1
  %59 = getelementptr i8, ptr %3, i64 8
  store i32 %49, ptr %4, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %59, ptr align 1 %4, i64 4, i1 false)
  %60 = getelementptr i8, ptr %59, i64 8
  store ptr %13, ptr %60, align 8
  %61 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %3, align 4
  %62 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %61, ptr %62, align 4
  ret void

yield_from.completed:                             ; preds = %yield_from.loop
  %63 = getelementptr i8, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", i64 8
  %64 = load i32, ptr %63, align 1
  store i32 %64, ptr %2, align 4
  br label %yield_from.cont

yield_from.failed:                                ; preds = %yield_from.loop
  %65 = getelementptr i8, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", i64 8
  %66 = load i8, ptr %65, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %5, align 4
  store i8 2, ptr %5, align 1
  %67 = getelementptr i8, ptr %5, i64 8
  store i8 %66, ptr %6, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %67, ptr align 1 %6, i64 1, i1 false)
  %68 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %5, align 4
  %69 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %68, ptr %69, align 4
  ret void

yield_from.fallback:                              ; preds = %yield_from.loop
  %70 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", align 4
  store i32 0, ptr %2, align 4
  br label %yield_from.cont

yield_from.panic:                                 ; preds = %yield_from.resume.1
  store i32 0, ptr %2, align 4
  br label %yield_from.cont

yield_from.cont:                                  ; preds = %yield_from.panic, %yield_from.fallback, %yield_from.completed
  %71 = load i32, ptr %2, align 4
  store i32 %71, ptr %__bind_124_result, align 4
  %72 = load i32, ptr %__bind_124_result, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %73 = getelementptr i8, ptr %0, i64 8
  store i32 %72, ptr %1, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %73, ptr align 1 %1, i64 4, i1 false)
  %74 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %0, align 4
  %75 = load ptr, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$__uv_key_scope_784", align 8
  call void @uv_key_scope_exit(ptr %75)
  %76 = load ptr, ptr %__uv_async_out2, align 8
  %77 = load ptr, ptr %__uv_async_out2, align 8
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %74, ptr %77, align 4
  ret void

yield_from.resume.1:                              ; preds = %poison.cont
  %78 = getelementptr i8, ptr %13, i64 24
  %79 = load ptr, ptr %78, align 8
  call void @uv_key_reacquire(ptr %79)
  %80 = getelementptr i8, ptr %13, i64 24
  store ptr null, ptr %80, align 8
  %81 = load ptr, ptr %__panic5, align 8
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3aresume(ptr noalias noundef nonnull sret({ i8, [7 x i8], [16 x i8] }) align 1 dereferenceable(24) %sret, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", ptr %14, ptr %81)
  %82 = load { i8, [7 x i8], [16 x i8] }, ptr %sret, align 1
  store { i8, [7 x i8], [16 x i8] } %82, ptr %coerce_bits11, align 1
  %83 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %coerce_bits11, align 1
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %83, ptr %"ExpressionSemantics_x3a_x3aasyncKeyYieldReleaseFailureArtifactReference$tmp$yield_from_source_795", align 4
  %84 = load i8, ptr %81, align 1
  %85 = icmp ne i8 %84, 0
  br i1 %85, label %yield_from.panic, label %yield_from.loop
}

define i32 @ExpressionSemantics_x3a_x3aLowerMethodCallFamilyReceiver_x3a_x3areadValue(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  %2 = getelementptr i8, ptr %self, i64 0
  %3 = load i32, ptr %2, align 1
  ret i32 %3
}

define i32 @ExpressionSemantics_x3a_x3alowerMethodCallFamilyReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %receiver = alloca { i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  store i32 71, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %receiver, ptr align 4 %aggregate.literal, i64 4, i1 false)
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %16 = call i32 @ExpressionSemantics_x3a_x3aLowerMethodCallFamilyReceiver_x3a_x3areadValue(ptr noundef nonnull align 4 dereferenceable(4) %receiver, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %17 = load ptr, ptr %__panic, align 8
  %18 = load i8, ptr %17, align 1
  %19 = icmp ne i8 %18, 0
  br i1 %19, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont2
  %20 = load ptr, ptr %__panic, align 8
  %21 = getelementptr i8, ptr %20, i64 0
  %22 = load i8, ptr %21, align 1
  %23 = load ptr, ptr %__panic, align 8
  %24 = getelementptr i8, ptr %23, i64 4
  %25 = load i32, ptr %24, align 4
  %26 = load ptr, ptr %__panic, align 8
  %27 = getelementptr i8, ptr %26, i64 4
  %28 = load i32, ptr %27, align 4
  ret i32 %28

panic.cont:                                       ; preds = %poison.cont2
  ret i32 %16
}

define i32 @ExpressionSemantics_x3a_x3acallTypeArgsLoweringElaborationReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %flag4 = alloca i8, align 1
  %"ExpressionSemantics_x3a_x3acallTypeArgsLoweringElaborationReference$tmp$call_ref_tmp_835" = alloca i8, align 1
  %flag = alloca i8, align 1
  %value1 = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3acallTypeArgsLoweringElaborationReference$tmp$call_ref_tmp_829" = alloca i32, align 4
  %value = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  store i32 73, ptr %"ExpressionSemantics_x3a_x3acallTypeArgsLoweringElaborationReference$tmp$call_ref_tmp_829", align 4
  %9 = call i32 @ExpressionSemantics_x3a_x3alowerGenericIdentity_x3a_x3ainst_x3a_x3ai32(ptr noundef nonnull align 4 dereferenceable(4) %"ExpressionSemantics_x3a_x3acallTypeArgsLoweringElaborationReference$tmp$call_ref_tmp_829", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %10 = load ptr, ptr %__panic, align 8
  %11 = load i8, ptr %10, align 1
  %12 = icmp ne i8 %11, 0
  br i1 %12, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont
  %13 = load ptr, ptr %__panic, align 8
  %14 = getelementptr i8, ptr %13, i64 4
  %15 = load i32, ptr %14, align 4
  ret i32 %15

panic.cont:                                       ; preds = %poison.cont
  store i32 %9, ptr %value1, align 4
  store i8 1, ptr %"ExpressionSemantics_x3a_x3acallTypeArgsLoweringElaborationReference$tmp$call_ref_tmp_835", align 1
  %16 = call i8 @ExpressionSemantics_x3a_x3alowerGenericIdentity_x3a_x3ainst_x3a_x3abool(ptr noundef nonnull align 1 dereferenceable(1) %"ExpressionSemantics_x3a_x3acallTypeArgsLoweringElaborationReference$tmp$call_ref_tmp_835", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %17 = load ptr, ptr %__panic, align 8
  %18 = load i8, ptr %17, align 1
  %19 = icmp ne i8 %18, 0
  br i1 %19, label %panic.take2, label %panic.cont3

panic.take2:                                      ; preds = %panic.cont
  %20 = load ptr, ptr %__panic, align 8
  %21 = getelementptr i8, ptr %20, i64 4
  %22 = load i32, ptr %21, align 4
  ret i32 %22

panic.cont3:                                      ; preds = %panic.cont
  %23 = icmp ne i8 %16, 0
  %24 = zext i1 %23 to i8
  store i8 %24, ptr %flag4, align 1
  %25 = load i8, ptr %flag4, align 1
  %26 = icmp ne i8 %25, 0
  %27 = load i8, ptr %flag4, align 1
  %28 = icmp ne i8 %27, 0
  br i1 %28, label %if.then, label %if.else

if.then:                                          ; preds = %panic.cont3
  %29 = load i32, ptr %value1, align 4
  ret i32 %29

if.else:                                          ; preds = %panic.cont3
  br label %if.merge

if.merge:                                         ; preds = %if.else
  ret i32 0
}

define hidden i32 @ExpressionSemantics_x3a_x3alowerGenericIdentity_x3a_x3ainst_x3a_x3ai32(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"ExpressionSemantics_x3a_x3alowerGenericIdentity_x3a_x3ainst_x3a_x3ai32$tmp$return_snapshot_826" = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %9 = load i32, ptr %value, align 1
  store i32 %9, ptr %"ExpressionSemantics_x3a_x3alowerGenericIdentity_x3a_x3ainst_x3a_x3ai32$tmp$return_snapshot_826", align 4
  %10 = load i32, ptr %"ExpressionSemantics_x3a_x3alowerGenericIdentity_x3a_x3ainst_x3a_x3ai32$tmp$return_snapshot_826", align 4
  ret i32 %10
}

define hidden i8 @ExpressionSemantics_x3a_x3alowerGenericIdentity_x3a_x3ainst_x3a_x3abool(ptr noundef nonnull align 1 dereferenceable(1) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"ExpressionSemantics_x3a_x3alowerGenericIdentity_x3a_x3ainst_x3a_x3abool$tmp$return_snapshot_832" = alloca i8, align 1
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %10 = load i8, ptr %value, align 1
  %11 = icmp ne i8 %10, 0
  %12 = zext i1 %11 to i8
  store i8 %12, ptr %"ExpressionSemantics_x3a_x3alowerGenericIdentity_x3a_x3ainst_x3a_x3abool$tmp$return_snapshot_832", align 1
  %13 = load i8, ptr %"ExpressionSemantics_x3a_x3alowerGenericIdentity_x3a_x3ainst_x3a_x3abool$tmp$return_snapshot_832", align 1
  %14 = icmp ne i8 %13, 0
  %15 = zext i1 %14 to i8
  ret i8 %15
}

define i32 @ExpressionSemantics_x3a_x3aconsumeMoveDomainPayload(ptr noundef nonnull align 4 dereferenceable(4) %payload, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"ExpressionSemantics_x3a_x3aconsumeMoveDomainPayload$tmp$return_snapshot_844" = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %9 = getelementptr i8, ptr %payload, i64 0
  %10 = load i32, ptr %9, align 1
  store i32 %10, ptr %"ExpressionSemantics_x3a_x3aconsumeMoveDomainPayload$tmp$return_snapshot_844", align 4
  %11 = load i32, ptr %"ExpressionSemantics_x3a_x3aconsumeMoveDomainPayload$tmp$return_snapshot_844", align 4
  ret i32 %11
}

define i32 @ExpressionSemantics_x3a_x3aconsumeNoDropDomainPayload(ptr noundef nonnull align 4 dereferenceable(4) %payload, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"ExpressionSemantics_x3a_x3aconsumeNoDropDomainPayload$tmp$return_snapshot_853" = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %9 = getelementptr i8, ptr %payload, i64 0
  %10 = load i32, ptr %9, align 1
  store i32 %10, ptr %"ExpressionSemantics_x3a_x3aconsumeNoDropDomainPayload$tmp$return_snapshot_853", align 4
  %11 = load i32, ptr %"ExpressionSemantics_x3a_x3aconsumeNoDropDomainPayload$tmp$return_snapshot_853", align 4
  ret i32 %11
}

define i32 @ExpressionSemantics_x3a_x3amoveParameterByReferenceReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %payload = alloca { i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  store i32 17, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %payload, ptr align 4 %aggregate.literal, i64 4, i1 false)
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %23 = call i32 @ExpressionSemantics_x3a_x3aconsumeMoveDomainPayload(ptr noundef nonnull align 4 dereferenceable(4) %payload, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
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
  ret i32 %23
}

define i32 @ExpressionSemantics_x3a_x3acopyCreatesIndependentProvenanceReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %duplicate = alloca { i32, [0 x i32] }, align 4
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %source = alloca { i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  store i32 19, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %source, ptr align 4 %aggregate.literal, i64 4, i1 false)
  %9 = load { i32, [0 x i32] }, ptr %source, align 4
  store { i32, [0 x i32] } %9, ptr %duplicate, align 4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %10 = load ptr, ptr %__panic, align 8
  %11 = load i8, ptr %10, align 1
  %12 = icmp ne i8 %11, 0
  br i1 %12, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %13 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %13, align 1
  %14 = getelementptr i8, ptr %13, i64 4
  store i32 4, ptr %14, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %15 = load ptr, ptr %__panic, align 8
  %16 = getelementptr i8, ptr %15, i64 0
  %17 = load i8, ptr %16, align 1
  %18 = load ptr, ptr %__panic, align 8
  %19 = getelementptr i8, ptr %18, i64 4
  %20 = load i32, ptr %19, align 4
  %21 = load ptr, ptr %__panic, align 8
  %22 = getelementptr i8, ptr %21, i64 4
  %23 = load i32, ptr %22, align 4
  ret i32 %23

panic.cont:                                       ; preds = %check_ok
  %24 = getelementptr i8, ptr %source, i64 0
  %25 = load i32, ptr %24, align 1
  %26 = getelementptr i8, ptr %duplicate, i64 0
  %27 = load i32, ptr %26, align 1
  %28 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %25, i32 %27)
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

define i32 @ExpressionSemantics_x3a_x3abindingMovabilityOperatorReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %immovable = alloca { i32, [0 x i32] }, align 4
  %field_alias = alloca i32, align 4
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %movable = alloca { i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  store i32 13, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %movable, ptr align 4 %aggregate.literal, i64 4, i1 false)
  %9 = getelementptr i8, ptr %movable, i64 0
  %10 = load i32, ptr %9, align 1
  store i32 %10, ptr %field_alias, align 4
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 4, i1 false)
  store i32 17, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %immovable, ptr align 4 %aggregate.literal, i64 4, i1 false)
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %11 = load ptr, ptr %__panic, align 8
  %12 = load i8, ptr %11, align 1
  %13 = icmp ne i8 %12, 0
  br i1 %13, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %14 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %14, align 1
  %15 = getelementptr i8, ptr %14, i64 4
  store i32 4, ptr %15, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %16 = load ptr, ptr %__panic, align 8
  %17 = getelementptr i8, ptr %16, i64 0
  %18 = load i8, ptr %17, align 1
  %19 = load ptr, ptr %__panic, align 8
  %20 = getelementptr i8, ptr %19, i64 4
  %21 = load i32, ptr %20, align 4
  %22 = load ptr, ptr %__panic, align 8
  %23 = getelementptr i8, ptr %22, i64 4
  %24 = load i32, ptr %23, align 4
  ret i32 %24

panic.cont:                                       ; preds = %check_ok
  %25 = load i32, ptr %field_alias, align 4
  %26 = getelementptr i8, ptr %immovable, i64 0
  %27 = load i32, ptr %26, align 1
  %28 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %25, i32 %27)
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

define { i32, [0 x i32] } @ExpressionSemantics_x3a_x3areturnMoveOwnershipDestinationReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"ExpressionSemantics_x3a_x3areturnMoveOwnershipDestinationReference$tmp$return_snapshot_904" = alloca { i32, [0 x i32] }, align 4
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %payload = alloca { i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret { i32, [0 x i32] } zeroinitializer

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 0, ptr %5, align 4
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 4, i1 false)
  store i32 23, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %payload, ptr align 4 %aggregate.literal, i64 4, i1 false)
  %6 = load { i32, [0 x i32] }, ptr %payload, align 4
  store { i32, [0 x i32] } %6, ptr %"ExpressionSemantics_x3a_x3areturnMoveOwnershipDestinationReference$tmp$return_snapshot_904", align 4
  %7 = load { i32, [0 x i32] }, ptr %"ExpressionSemantics_x3a_x3areturnMoveOwnershipDestinationReference$tmp$return_snapshot_904", align 4
  ret { i32, [0 x i32] } %7
}

define { i32, [0 x i32] } @ExpressionSemantics_x3a_x3areturnConstructedOwnershipDestinationReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %"ExpressionSemantics_x3a_x3areturnConstructedOwnershipDestinationReference$tmp$return_snapshot_908" = alloca { i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret { i32, [0 x i32] } zeroinitializer

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 0, ptr %5, align 4
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 4, i1 false)
  store i32 29, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %"ExpressionSemantics_x3a_x3areturnConstructedOwnershipDestinationReference$tmp$return_snapshot_908", ptr align 4 %aggregate.literal, i64 4, i1 false)
  %6 = load { i32, [0 x i32] }, ptr %"ExpressionSemantics_x3a_x3areturnConstructedOwnershipDestinationReference$tmp$return_snapshot_908", align 4
  ret { i32, [0 x i32] } %6
}

define { i32, [0 x i32] } @ExpressionSemantics_x3a_x3areturnCopiedPayloadReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %source = alloca { i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret { i32, [0 x i32] } zeroinitializer

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 0, ptr %5, align 4
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 4, i1 false)
  store i32 31, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %source, ptr align 4 %aggregate.literal, i64 4, i1 false)
  %6 = load { i32, [0 x i32] }, ptr %source, align 4
  ret { i32, [0 x i32] } %6
}

define i32 @ExpressionSemantics_x3a_x3anoDropFinalOwnerDomainReleaseReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %payload = alloca { i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  store i32 41, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %payload, ptr align 4 %aggregate.literal, i64 4, i1 false)
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %23 = call i32 @ExpressionSemantics_x3a_x3aconsumeNoDropDomainPayload(ptr noundef nonnull align 4 dereferenceable(4) %payload, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
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
  ret i32 %23
}

define i32 @ExpressionSemantics_x3a_x3arecordDefaultCtorReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %aggregate.literal = alloca { i32, i8, [3 x i8], [0 x i32] }, align 4
  %payload = alloca { i32, i8, [3 x i8], [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 8, i1 false)
  store i32 61, ptr %aggregate.literal, align 4
  %9 = getelementptr i8, ptr %aggregate.literal, i64 4
  store i8 1, ptr %9, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %payload, ptr align 4 %aggregate.literal, i64 8, i1 false)
  %10 = getelementptr i8, ptr %payload, i64 4
  %11 = load i8, ptr %10, align 1
  %12 = icmp ne i8 %11, 0
  %13 = getelementptr i8, ptr %payload, i64 4
  %14 = load i8, ptr %13, align 1
  %15 = icmp ne i8 %14, 0
  br i1 %15, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  %16 = getelementptr i8, ptr %payload, i64 0
  %17 = load i32, ptr %16, align 1
  ret i32 %17

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else
  ret i32 0
}

define i32 @ExpressionSemantics_x3a_x3aoperatorRangeTypingReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %inclusive = alloca { ptr, i64 }, align 8
  %exclusive = alloca { ptr, i64 }, align 8
  %from_slice = alloca { ptr, i64 }, align 8
  %to_inclusive = alloca { ptr, i64 }, align 8
  %to = alloca { ptr, i64 }, align 8
  %full = alloca { ptr, i64 }, align 8
  %aggregate.literal = alloca [5 x i32], align 4
  %values = alloca [5 x i32], align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %9 = getelementptr [5 x i32], ptr %aggregate.literal, i64 0, i64 0
  store i32 10, ptr %9, align 4
  %10 = getelementptr [5 x i32], ptr %aggregate.literal, i64 0, i64 1
  store i32 20, ptr %10, align 4
  %11 = getelementptr [5 x i32], ptr %aggregate.literal, i64 0, i64 2
  store i32 30, ptr %11, align 4
  %12 = getelementptr [5 x i32], ptr %aggregate.literal, i64 0, i64 3
  store i32 40, ptr %12, align 4
  %13 = getelementptr [5 x i32], ptr %aggregate.literal, i64 0, i64 4
  store i32 50, ptr %13, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %values, ptr align 4 %aggregate.literal, i64 20, i1 false)
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %14 = load ptr, ptr %__panic, align 8
  %15 = load i8, ptr %14, align 1
  %16 = icmp ne i8 %15, 0
  br i1 %16, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %17 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %17, align 1
  %18 = getelementptr i8, ptr %17, i64 4
  store i32 6, ptr %18, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %19 = load ptr, ptr %__panic, align 8
  %20 = getelementptr i8, ptr %19, i64 0
  %21 = load i8, ptr %20, align 1
  %22 = load ptr, ptr %__panic, align 8
  %23 = getelementptr i8, ptr %22, i64 4
  %24 = load i32, ptr %23, align 4
  %25 = load ptr, ptr %__panic, align 8
  %26 = getelementptr i8, ptr %25, i64 4
  %27 = load i32, ptr %26, align 4
  ret i32 %27

panic.cont:                                       ; preds = %check_ok
  %28 = getelementptr i32, ptr %values, i64 0
  %29 = insertvalue { ptr, i64 } zeroinitializer, ptr %28, 0
  %30 = insertvalue { ptr, i64 } %29, i64 5, 1
  store { ptr, i64 } %30, ptr %full, align 8
  br i1 true, label %check_ok1, label %check_fail2

check_ok1:                                        ; preds = %check_fail2, %panic.cont
  %31 = load ptr, ptr %__panic, align 8
  %32 = load i8, ptr %31, align 1
  %33 = icmp ne i8 %32, 0
  br i1 %33, label %panic.take3, label %panic.cont4

check_fail2:                                      ; preds = %panic.cont
  %34 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %34, align 1
  %35 = getelementptr i8, ptr %34, i64 4
  store i32 6, ptr %35, align 4
  br label %check_ok1

panic.take3:                                      ; preds = %check_ok1
  %36 = load ptr, ptr %__panic, align 8
  %37 = getelementptr i8, ptr %36, i64 0
  %38 = load i8, ptr %37, align 1
  %39 = load ptr, ptr %__panic, align 8
  %40 = getelementptr i8, ptr %39, i64 4
  %41 = load i32, ptr %40, align 4
  %42 = load ptr, ptr %__panic, align 8
  %43 = getelementptr i8, ptr %42, i64 4
  %44 = load i32, ptr %43, align 4
  ret i32 %44

panic.cont4:                                      ; preds = %check_ok1
  %45 = getelementptr i32, ptr %values, i64 0
  %46 = insertvalue { ptr, i64 } zeroinitializer, ptr %45, 0
  %47 = insertvalue { ptr, i64 } %46, i64 3, 1
  store { ptr, i64 } %47, ptr %to, align 8
  br i1 true, label %check_ok5, label %check_fail6

check_ok5:                                        ; preds = %check_fail6, %panic.cont4
  %48 = load ptr, ptr %__panic, align 8
  %49 = load i8, ptr %48, align 1
  %50 = icmp ne i8 %49, 0
  br i1 %50, label %panic.take7, label %panic.cont8

check_fail6:                                      ; preds = %panic.cont4
  %51 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %51, align 1
  %52 = getelementptr i8, ptr %51, i64 4
  store i32 6, ptr %52, align 4
  br label %check_ok5

panic.take7:                                      ; preds = %check_ok5
  %53 = load ptr, ptr %__panic, align 8
  %54 = getelementptr i8, ptr %53, i64 0
  %55 = load i8, ptr %54, align 1
  %56 = load ptr, ptr %__panic, align 8
  %57 = getelementptr i8, ptr %56, i64 4
  %58 = load i32, ptr %57, align 4
  %59 = load ptr, ptr %__panic, align 8
  %60 = getelementptr i8, ptr %59, i64 4
  %61 = load i32, ptr %60, align 4
  ret i32 %61

panic.cont8:                                      ; preds = %check_ok5
  %62 = getelementptr i32, ptr %values, i64 0
  %63 = insertvalue { ptr, i64 } zeroinitializer, ptr %62, 0
  %64 = insertvalue { ptr, i64 } %63, i64 3, 1
  store { ptr, i64 } %64, ptr %to_inclusive, align 8
  br i1 true, label %check_ok9, label %check_fail10

check_ok9:                                        ; preds = %check_fail10, %panic.cont8
  %65 = load ptr, ptr %__panic, align 8
  %66 = load i8, ptr %65, align 1
  %67 = icmp ne i8 %66, 0
  br i1 %67, label %panic.take11, label %panic.cont12

check_fail10:                                     ; preds = %panic.cont8
  %68 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %68, align 1
  %69 = getelementptr i8, ptr %68, i64 4
  store i32 6, ptr %69, align 4
  br label %check_ok9

panic.take11:                                     ; preds = %check_ok9
  %70 = load ptr, ptr %__panic, align 8
  %71 = getelementptr i8, ptr %70, i64 0
  %72 = load i8, ptr %71, align 1
  %73 = load ptr, ptr %__panic, align 8
  %74 = getelementptr i8, ptr %73, i64 4
  %75 = load i32, ptr %74, align 4
  %76 = load ptr, ptr %__panic, align 8
  %77 = getelementptr i8, ptr %76, i64 4
  %78 = load i32, ptr %77, align 4
  ret i32 %78

panic.cont12:                                     ; preds = %check_ok9
  %79 = getelementptr i32, ptr %values, i64 2
  %80 = insertvalue { ptr, i64 } zeroinitializer, ptr %79, 0
  %81 = insertvalue { ptr, i64 } %80, i64 3, 1
  store { ptr, i64 } %81, ptr %from_slice, align 8
  br i1 true, label %check_ok13, label %check_fail14

check_ok13:                                       ; preds = %check_fail14, %panic.cont12
  %82 = load ptr, ptr %__panic, align 8
  %83 = load i8, ptr %82, align 1
  %84 = icmp ne i8 %83, 0
  br i1 %84, label %panic.take15, label %panic.cont16

check_fail14:                                     ; preds = %panic.cont12
  %85 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %85, align 1
  %86 = getelementptr i8, ptr %85, i64 4
  store i32 6, ptr %86, align 4
  br label %check_ok13

panic.take15:                                     ; preds = %check_ok13
  %87 = load ptr, ptr %__panic, align 8
  %88 = getelementptr i8, ptr %87, i64 0
  %89 = load i8, ptr %88, align 1
  %90 = load ptr, ptr %__panic, align 8
  %91 = getelementptr i8, ptr %90, i64 4
  %92 = load i32, ptr %91, align 4
  %93 = load ptr, ptr %__panic, align 8
  %94 = getelementptr i8, ptr %93, i64 4
  %95 = load i32, ptr %94, align 4
  ret i32 %95

panic.cont16:                                     ; preds = %check_ok13
  %96 = getelementptr i32, ptr %values, i64 1
  %97 = insertvalue { ptr, i64 } zeroinitializer, ptr %96, 0
  %98 = insertvalue { ptr, i64 } %97, i64 2, 1
  store { ptr, i64 } %98, ptr %exclusive, align 8
  br i1 true, label %check_ok17, label %check_fail18

check_ok17:                                       ; preds = %check_fail18, %panic.cont16
  %99 = load ptr, ptr %__panic, align 8
  %100 = load i8, ptr %99, align 1
  %101 = icmp ne i8 %100, 0
  br i1 %101, label %panic.take19, label %panic.cont20

check_fail18:                                     ; preds = %panic.cont16
  %102 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %102, align 1
  %103 = getelementptr i8, ptr %102, i64 4
  store i32 6, ptr %103, align 4
  br label %check_ok17

panic.take19:                                     ; preds = %check_ok17
  %104 = load ptr, ptr %__panic, align 8
  %105 = getelementptr i8, ptr %104, i64 0
  %106 = load i8, ptr %105, align 1
  %107 = load ptr, ptr %__panic, align 8
  %108 = getelementptr i8, ptr %107, i64 4
  %109 = load i32, ptr %108, align 4
  %110 = load ptr, ptr %__panic, align 8
  %111 = getelementptr i8, ptr %110, i64 4
  %112 = load i32, ptr %111, align 4
  ret i32 %112

panic.cont20:                                     ; preds = %check_ok17
  %113 = getelementptr i32, ptr %values, i64 1
  %114 = insertvalue { ptr, i64 } zeroinitializer, ptr %113, 0
  %115 = insertvalue { ptr, i64 } %114, i64 3, 1
  store { ptr, i64 } %115, ptr %inclusive, align 8
  %116 = load { ptr, i64 }, ptr %full, align 8
  %117 = extractvalue { ptr, i64 } %116, 1
  %118 = icmp ult i64 0, %117
  br i1 %118, label %check_ok21, label %check_fail22

check_ok21:                                       ; preds = %check_fail22, %panic.cont20
  %119 = load ptr, ptr %__panic, align 8
  %120 = load i8, ptr %119, align 1
  %121 = icmp ne i8 %120, 0
  br i1 %121, label %panic.take23, label %panic.cont24

check_fail22:                                     ; preds = %panic.cont20
  %122 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %122, align 1
  %123 = getelementptr i8, ptr %122, i64 4
  store i32 6, ptr %123, align 4
  br label %check_ok21

panic.take23:                                     ; preds = %check_ok21
  %124 = load ptr, ptr %__panic, align 8
  %125 = getelementptr i8, ptr %124, i64 0
  %126 = load i8, ptr %125, align 1
  %127 = load ptr, ptr %__panic, align 8
  %128 = getelementptr i8, ptr %127, i64 4
  %129 = load i32, ptr %128, align 4
  %130 = load ptr, ptr %__panic, align 8
  %131 = getelementptr i8, ptr %130, i64 4
  %132 = load i32, ptr %131, align 4
  ret i32 %132

panic.cont24:                                     ; preds = %check_ok21
  %133 = load { ptr, i64 }, ptr %to, align 8
  %134 = extractvalue { ptr, i64 } %133, 1
  %135 = icmp ult i64 1, %134
  br i1 %135, label %check_ok25, label %check_fail26

check_ok25:                                       ; preds = %check_fail26, %panic.cont24
  %136 = load ptr, ptr %__panic, align 8
  %137 = load i8, ptr %136, align 1
  %138 = icmp ne i8 %137, 0
  br i1 %138, label %panic.take27, label %panic.cont28

check_fail26:                                     ; preds = %panic.cont24
  %139 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %139, align 1
  %140 = getelementptr i8, ptr %139, i64 4
  store i32 6, ptr %140, align 4
  br label %check_ok25

panic.take27:                                     ; preds = %check_ok25
  %141 = load ptr, ptr %__panic, align 8
  %142 = getelementptr i8, ptr %141, i64 0
  %143 = load i8, ptr %142, align 1
  %144 = load ptr, ptr %__panic, align 8
  %145 = getelementptr i8, ptr %144, i64 4
  %146 = load i32, ptr %145, align 4
  %147 = load ptr, ptr %__panic, align 8
  %148 = getelementptr i8, ptr %147, i64 4
  %149 = load i32, ptr %148, align 4
  ret i32 %149

panic.cont28:                                     ; preds = %check_ok25
  br i1 true, label %check_ok29, label %check_fail30

check_ok29:                                       ; preds = %check_fail30, %panic.cont28
  %150 = load ptr, ptr %__panic, align 8
  %151 = load i8, ptr %150, align 1
  %152 = icmp ne i8 %151, 0
  br i1 %152, label %panic.take31, label %panic.cont32

check_fail30:                                     ; preds = %panic.cont28
  %153 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %153, align 1
  %154 = getelementptr i8, ptr %153, i64 4
  store i32 4, ptr %154, align 4
  br label %check_ok29

panic.take31:                                     ; preds = %check_ok29
  %155 = load ptr, ptr %__panic, align 8
  %156 = getelementptr i8, ptr %155, i64 0
  %157 = load i8, ptr %156, align 1
  %158 = load ptr, ptr %__panic, align 8
  %159 = getelementptr i8, ptr %158, i64 4
  %160 = load i32, ptr %159, align 4
  %161 = load ptr, ptr %__panic, align 8
  %162 = getelementptr i8, ptr %161, i64 4
  %163 = load i32, ptr %162, align 4
  ret i32 %163

panic.cont32:                                     ; preds = %check_ok29
  %164 = load { ptr, i64 }, ptr %full, align 8
  %165 = extractvalue { ptr, i64 } %164, 0
  %166 = getelementptr i32, ptr %165, i64 0
  %167 = load i32, ptr %166, align 4
  %168 = load { ptr, i64 }, ptr %to, align 8
  %169 = extractvalue { ptr, i64 } %168, 0
  %170 = getelementptr i32, ptr %169, i64 1
  %171 = load i32, ptr %170, align 4
  %172 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %167, i32 %171)
  %173 = extractvalue { i32, i1 } %172, 0
  %174 = extractvalue { i32, i1 } %172, 1
  %175 = freeze i32 %173
  br i1 %174, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont32
  %176 = load { ptr, i64 }, ptr %to_inclusive, align 8
  %177 = extractvalue { ptr, i64 } %176, 1
  %178 = icmp ult i64 2, %177
  br i1 %178, label %check_ok33, label %check_fail34

op_fail:                                          ; preds = %panic.cont32
  %179 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %179, align 1
  %180 = getelementptr i8, ptr %179, i64 4
  store i32 4, ptr %180, align 4
  ret i32 0

check_ok33:                                       ; preds = %check_fail34, %op_ok
  %181 = load ptr, ptr %__panic, align 8
  %182 = load i8, ptr %181, align 1
  %183 = icmp ne i8 %182, 0
  br i1 %183, label %panic.take35, label %panic.cont36

check_fail34:                                     ; preds = %op_ok
  %184 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %184, align 1
  %185 = getelementptr i8, ptr %184, i64 4
  store i32 6, ptr %185, align 4
  br label %check_ok33

panic.take35:                                     ; preds = %check_ok33
  %186 = load ptr, ptr %__panic, align 8
  %187 = getelementptr i8, ptr %186, i64 0
  %188 = load i8, ptr %187, align 1
  %189 = load ptr, ptr %__panic, align 8
  %190 = getelementptr i8, ptr %189, i64 4
  %191 = load i32, ptr %190, align 4
  %192 = load ptr, ptr %__panic, align 8
  %193 = getelementptr i8, ptr %192, i64 4
  %194 = load i32, ptr %193, align 4
  ret i32 %194

panic.cont36:                                     ; preds = %check_ok33
  br i1 true, label %check_ok37, label %check_fail38

check_ok37:                                       ; preds = %check_fail38, %panic.cont36
  %195 = load ptr, ptr %__panic, align 8
  %196 = load i8, ptr %195, align 1
  %197 = icmp ne i8 %196, 0
  br i1 %197, label %panic.take39, label %panic.cont40

check_fail38:                                     ; preds = %panic.cont36
  %198 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %198, align 1
  %199 = getelementptr i8, ptr %198, i64 4
  store i32 4, ptr %199, align 4
  br label %check_ok37

panic.take39:                                     ; preds = %check_ok37
  %200 = load ptr, ptr %__panic, align 8
  %201 = getelementptr i8, ptr %200, i64 0
  %202 = load i8, ptr %201, align 1
  %203 = load ptr, ptr %__panic, align 8
  %204 = getelementptr i8, ptr %203, i64 4
  %205 = load i32, ptr %204, align 4
  %206 = load ptr, ptr %__panic, align 8
  %207 = getelementptr i8, ptr %206, i64 4
  %208 = load i32, ptr %207, align 4
  ret i32 %208

panic.cont40:                                     ; preds = %check_ok37
  %209 = load { ptr, i64 }, ptr %to_inclusive, align 8
  %210 = extractvalue { ptr, i64 } %209, 0
  %211 = getelementptr i32, ptr %210, i64 2
  %212 = load i32, ptr %211, align 4
  %213 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %175, i32 %212)
  %214 = extractvalue { i32, i1 } %213, 0
  %215 = extractvalue { i32, i1 } %213, 1
  %216 = freeze i32 %214
  br i1 %215, label %op_fail42, label %op_ok41

op_ok41:                                          ; preds = %panic.cont40
  %217 = load { ptr, i64 }, ptr %from_slice, align 8
  %218 = extractvalue { ptr, i64 } %217, 1
  %219 = icmp ult i64 0, %218
  br i1 %219, label %check_ok43, label %check_fail44

op_fail42:                                        ; preds = %panic.cont40
  %220 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %220, align 1
  %221 = getelementptr i8, ptr %220, i64 4
  store i32 4, ptr %221, align 4
  ret i32 0

check_ok43:                                       ; preds = %check_fail44, %op_ok41
  %222 = load ptr, ptr %__panic, align 8
  %223 = load i8, ptr %222, align 1
  %224 = icmp ne i8 %223, 0
  br i1 %224, label %panic.take45, label %panic.cont46

check_fail44:                                     ; preds = %op_ok41
  %225 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %225, align 1
  %226 = getelementptr i8, ptr %225, i64 4
  store i32 6, ptr %226, align 4
  br label %check_ok43

panic.take45:                                     ; preds = %check_ok43
  %227 = load ptr, ptr %__panic, align 8
  %228 = getelementptr i8, ptr %227, i64 0
  %229 = load i8, ptr %228, align 1
  %230 = load ptr, ptr %__panic, align 8
  %231 = getelementptr i8, ptr %230, i64 4
  %232 = load i32, ptr %231, align 4
  %233 = load ptr, ptr %__panic, align 8
  %234 = getelementptr i8, ptr %233, i64 4
  %235 = load i32, ptr %234, align 4
  ret i32 %235

panic.cont46:                                     ; preds = %check_ok43
  br i1 true, label %check_ok47, label %check_fail48

check_ok47:                                       ; preds = %check_fail48, %panic.cont46
  %236 = load ptr, ptr %__panic, align 8
  %237 = load i8, ptr %236, align 1
  %238 = icmp ne i8 %237, 0
  br i1 %238, label %panic.take49, label %panic.cont50

check_fail48:                                     ; preds = %panic.cont46
  %239 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %239, align 1
  %240 = getelementptr i8, ptr %239, i64 4
  store i32 4, ptr %240, align 4
  br label %check_ok47

panic.take49:                                     ; preds = %check_ok47
  %241 = load ptr, ptr %__panic, align 8
  %242 = getelementptr i8, ptr %241, i64 0
  %243 = load i8, ptr %242, align 1
  %244 = load ptr, ptr %__panic, align 8
  %245 = getelementptr i8, ptr %244, i64 4
  %246 = load i32, ptr %245, align 4
  %247 = load ptr, ptr %__panic, align 8
  %248 = getelementptr i8, ptr %247, i64 4
  %249 = load i32, ptr %248, align 4
  ret i32 %249

panic.cont50:                                     ; preds = %check_ok47
  %250 = load { ptr, i64 }, ptr %from_slice, align 8
  %251 = extractvalue { ptr, i64 } %250, 0
  %252 = getelementptr i32, ptr %251, i64 0
  %253 = load i32, ptr %252, align 4
  %254 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %216, i32 %253)
  %255 = extractvalue { i32, i1 } %254, 0
  %256 = extractvalue { i32, i1 } %254, 1
  %257 = freeze i32 %255
  br i1 %256, label %op_fail52, label %op_ok51

op_ok51:                                          ; preds = %panic.cont50
  %258 = load { ptr, i64 }, ptr %exclusive, align 8
  %259 = extractvalue { ptr, i64 } %258, 1
  %260 = icmp ult i64 1, %259
  br i1 %260, label %check_ok53, label %check_fail54

op_fail52:                                        ; preds = %panic.cont50
  %261 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %261, align 1
  %262 = getelementptr i8, ptr %261, i64 4
  store i32 4, ptr %262, align 4
  ret i32 0

check_ok53:                                       ; preds = %check_fail54, %op_ok51
  %263 = load ptr, ptr %__panic, align 8
  %264 = load i8, ptr %263, align 1
  %265 = icmp ne i8 %264, 0
  br i1 %265, label %panic.take55, label %panic.cont56

check_fail54:                                     ; preds = %op_ok51
  %266 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %266, align 1
  %267 = getelementptr i8, ptr %266, i64 4
  store i32 6, ptr %267, align 4
  br label %check_ok53

panic.take55:                                     ; preds = %check_ok53
  %268 = load ptr, ptr %__panic, align 8
  %269 = getelementptr i8, ptr %268, i64 0
  %270 = load i8, ptr %269, align 1
  %271 = load ptr, ptr %__panic, align 8
  %272 = getelementptr i8, ptr %271, i64 4
  %273 = load i32, ptr %272, align 4
  %274 = load ptr, ptr %__panic, align 8
  %275 = getelementptr i8, ptr %274, i64 4
  %276 = load i32, ptr %275, align 4
  ret i32 %276

panic.cont56:                                     ; preds = %check_ok53
  br i1 true, label %check_ok57, label %check_fail58

check_ok57:                                       ; preds = %check_fail58, %panic.cont56
  %277 = load ptr, ptr %__panic, align 8
  %278 = load i8, ptr %277, align 1
  %279 = icmp ne i8 %278, 0
  br i1 %279, label %panic.take59, label %panic.cont60

check_fail58:                                     ; preds = %panic.cont56
  %280 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %280, align 1
  %281 = getelementptr i8, ptr %280, i64 4
  store i32 4, ptr %281, align 4
  br label %check_ok57

panic.take59:                                     ; preds = %check_ok57
  %282 = load ptr, ptr %__panic, align 8
  %283 = getelementptr i8, ptr %282, i64 0
  %284 = load i8, ptr %283, align 1
  %285 = load ptr, ptr %__panic, align 8
  %286 = getelementptr i8, ptr %285, i64 4
  %287 = load i32, ptr %286, align 4
  %288 = load ptr, ptr %__panic, align 8
  %289 = getelementptr i8, ptr %288, i64 4
  %290 = load i32, ptr %289, align 4
  ret i32 %290

panic.cont60:                                     ; preds = %check_ok57
  %291 = load { ptr, i64 }, ptr %exclusive, align 8
  %292 = extractvalue { ptr, i64 } %291, 0
  %293 = getelementptr i32, ptr %292, i64 1
  %294 = load i32, ptr %293, align 4
  %295 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %257, i32 %294)
  %296 = extractvalue { i32, i1 } %295, 0
  %297 = extractvalue { i32, i1 } %295, 1
  %298 = freeze i32 %296
  br i1 %297, label %op_fail62, label %op_ok61

op_ok61:                                          ; preds = %panic.cont60
  %299 = load { ptr, i64 }, ptr %inclusive, align 8
  %300 = extractvalue { ptr, i64 } %299, 1
  %301 = icmp ult i64 2, %300
  br i1 %301, label %check_ok63, label %check_fail64

op_fail62:                                        ; preds = %panic.cont60
  %302 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %302, align 1
  %303 = getelementptr i8, ptr %302, i64 4
  store i32 4, ptr %303, align 4
  ret i32 0

check_ok63:                                       ; preds = %check_fail64, %op_ok61
  %304 = load ptr, ptr %__panic, align 8
  %305 = load i8, ptr %304, align 1
  %306 = icmp ne i8 %305, 0
  br i1 %306, label %panic.take65, label %panic.cont66

check_fail64:                                     ; preds = %op_ok61
  %307 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %307, align 1
  %308 = getelementptr i8, ptr %307, i64 4
  store i32 6, ptr %308, align 4
  br label %check_ok63

panic.take65:                                     ; preds = %check_ok63
  %309 = load ptr, ptr %__panic, align 8
  %310 = getelementptr i8, ptr %309, i64 0
  %311 = load i8, ptr %310, align 1
  %312 = load ptr, ptr %__panic, align 8
  %313 = getelementptr i8, ptr %312, i64 4
  %314 = load i32, ptr %313, align 4
  %315 = load ptr, ptr %__panic, align 8
  %316 = getelementptr i8, ptr %315, i64 4
  %317 = load i32, ptr %316, align 4
  ret i32 %317

panic.cont66:                                     ; preds = %check_ok63
  br i1 true, label %check_ok67, label %check_fail68

check_ok67:                                       ; preds = %check_fail68, %panic.cont66
  %318 = load ptr, ptr %__panic, align 8
  %319 = load i8, ptr %318, align 1
  %320 = icmp ne i8 %319, 0
  br i1 %320, label %panic.take69, label %panic.cont70

check_fail68:                                     ; preds = %panic.cont66
  %321 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %321, align 1
  %322 = getelementptr i8, ptr %321, i64 4
  store i32 4, ptr %322, align 4
  br label %check_ok67

panic.take69:                                     ; preds = %check_ok67
  %323 = load ptr, ptr %__panic, align 8
  %324 = getelementptr i8, ptr %323, i64 0
  %325 = load i8, ptr %324, align 1
  %326 = load ptr, ptr %__panic, align 8
  %327 = getelementptr i8, ptr %326, i64 4
  %328 = load i32, ptr %327, align 4
  %329 = load ptr, ptr %__panic, align 8
  %330 = getelementptr i8, ptr %329, i64 4
  %331 = load i32, ptr %330, align 4
  ret i32 %331

panic.cont70:                                     ; preds = %check_ok67
  %332 = load { ptr, i64 }, ptr %inclusive, align 8
  %333 = extractvalue { ptr, i64 } %332, 0
  %334 = getelementptr i32, ptr %333, i64 2
  %335 = load i32, ptr %334, align 4
  %336 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %298, i32 %335)
  %337 = extractvalue { i32, i1 } %336, 0
  %338 = extractvalue { i32, i1 } %336, 1
  %339 = freeze i32 %337
  br i1 %338, label %op_fail72, label %op_ok71

op_ok71:                                          ; preds = %panic.cont70
  ret i32 %339

op_fail72:                                        ; preds = %panic.cont70
  %340 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %340, align 1
  %341 = getelementptr i8, ptr %340, i64 4
  store i32 4, ptr %341, align 4
  ret i32 0
}

define { i64, i64 } @ExpressionSemantics_x3a_x3aoperatorRangeLoweringReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret { i64, i64 } zeroinitializer

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 0, ptr %5, align 4
  ret { i64, i64 } { i64 1, i64 3 }
}

define { i32, i8, [3 x i8] } @ExpressionSemantics_x3a_x3atupleLoweringReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i32, i8, [3 x i8] }, align 8
  %1 = alloca { i32, i8, [3 x i8] }, align 8
  %2 = alloca { i32, i8, [3 x i8] }, align 8
  %3 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %4 = icmp ne i8 %3, 0
  br i1 %4, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %5 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 10, ptr %6, align 4
  ret { i32, i8, [3 x i8] } zeroinitializer

poison.cont:                                      ; preds = %entry
  %7 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %7, align 1
  %8 = getelementptr i8, ptr %7, i64 4
  store i32 0, ptr %8, align 4
  store { i32, i8, [3 x i8] } zeroinitializer, ptr %2, align 4
  %9 = getelementptr i8, ptr %2, i64 0
  store i32 37, ptr %9, align 1
  %10 = getelementptr i8, ptr %2, i64 4
  store i8 1, ptr %10, align 1
  %11 = load { i32, i8, [3 x i8] }, ptr %2, align 4
  store { i32, i8, [3 x i8] } %11, ptr %0, align 4
  call void @llvm.memset.p0.i64(ptr align 4 %1, i8 0, i64 8, i1 false)
  %12 = load i32, ptr %0, align 1
  store i32 %12, ptr %1, align 1
  %13 = getelementptr i8, ptr %0, i64 4
  %14 = getelementptr i8, ptr %1, i64 4
  %15 = load i8, ptr %13, align 1
  %16 = icmp ne i8 %15, 0
  %17 = zext i1 %16 to i8
  store i8 %17, ptr %14, align 1
  %18 = load { i32, i8, [3 x i8] }, ptr %1, align 4
  ret { i32, i8, [3 x i8] } %18
}

define i32 @ExpressionSemantics_x3a_x3atupleAccessLoweringReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i32, i32 }, align 8
  %1 = alloca { i32, i32 }, align 8
  %2 = alloca { i32, i32 }, align 8
  %3 = alloca { i32, i32 }, align 8
  %value = alloca { i32, i32 }, align 4
  %4 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %5 = icmp ne i8 %4, 0
  br i1 %5, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %6 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %6, align 1
  %7 = getelementptr i8, ptr %6, i64 4
  store i32 10, ptr %7, align 4
  %8 = load ptr, ptr %__panic, align 8
  %9 = getelementptr i8, ptr %8, i64 4
  %10 = load i32, ptr %9, align 4
  ret i32 %10

poison.cont:                                      ; preds = %entry
  %11 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %11, align 1
  %12 = getelementptr i8, ptr %11, i64 4
  store i32 0, ptr %12, align 4
  store { i32, i32 } zeroinitializer, ptr %3, align 4
  %13 = getelementptr i8, ptr %3, i64 0
  store i32 37, ptr %13, align 1
  %14 = getelementptr i8, ptr %3, i64 4
  store i32 41, ptr %14, align 1
  %15 = load { i32, i32 }, ptr %3, align 4
  store { i32, i32 } %15, ptr %1, align 4
  call void @llvm.memset.p0.i64(ptr align 4 %2, i8 0, i64 8, i1 false)
  %16 = load i32, ptr %1, align 1
  store i32 %16, ptr %2, align 1
  %17 = getelementptr i8, ptr %1, i64 4
  %18 = getelementptr i8, ptr %2, i64 4
  %19 = load i32, ptr %17, align 1
  store i32 %19, ptr %18, align 1
  %20 = load { i32, i32 }, ptr %2, align 4
  store { i32, i32 } %20, ptr %value, align 4
  %21 = load { i32, i32 }, ptr %value, align 4
  store { i32, i32 } %21, ptr %0, align 4
  %22 = getelementptr i8, ptr %0, i64 4
  %23 = load i32, ptr %22, align 1
  ret i32 %23
}

define i32 @ExpressionSemantics_x3a_x3aarrayLoweringReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %aggregate.literal = alloca [3 x i32], align 4
  %values = alloca [3 x i32], align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %9 = getelementptr [3 x i32], ptr %aggregate.literal, i64 0, i64 0
  store i32 3, ptr %9, align 4
  %10 = getelementptr [3 x i32], ptr %aggregate.literal, i64 0, i64 1
  store i32 5, ptr %10, align 4
  %11 = getelementptr [3 x i32], ptr %aggregate.literal, i64 0, i64 2
  store i32 7, ptr %11, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %values, ptr align 4 %aggregate.literal, i64 12, i1 false)
  %12 = getelementptr i32, ptr %values, i64 1
  %13 = load i32, ptr %12, align 4
  ret i32 %13
}

define i32 @ExpressionSemantics_x3a_x3aenumLoweringReference(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %named = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 4
  %tuple = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 4
  %aggregate.literal = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 4
  %unit = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 8, i1 false)
  store i8 0, ptr %aggregate.literal, align 1
  %9 = getelementptr i8, ptr %aggregate.literal, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %unit, ptr align 4 %aggregate.literal, i64 8, i1 false)
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 8, i1 false)
  store i8 1, ptr %aggregate.literal, align 1
  %10 = getelementptr i8, ptr %aggregate.literal, i64 4
  %11 = getelementptr i8, ptr %10, i64 0
  store i32 11, ptr %11, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %tuple, ptr align 4 %aggregate.literal, i64 8, i1 false)
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 8, i1 false)
  store i8 2, ptr %aggregate.literal, align 1
  %12 = getelementptr i8, ptr %aggregate.literal, i64 4
  %13 = getelementptr i8, ptr %12, i64 0
  store i32 13, ptr %13, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %named, ptr align 4 %aggregate.literal, i64 8, i1 false)
  %14 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %15 = icmp ne i8 %14, 0
  br i1 %15, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %16 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %16, align 1
  %17 = getelementptr i8, ptr %16, i64 4
  store i32 10, ptr %17, align 4
  %18 = load ptr, ptr %__panic, align 8
  %19 = getelementptr i8, ptr %18, i64 4
  %20 = load i32, ptr %19, align 4
  ret i32 %20

poison.cont2:                                     ; preds = %poison.cont
  %21 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %22 = icmp ne i8 %21, 0
  br i1 %22, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %23 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %23, align 1
  %24 = getelementptr i8, ptr %23, i64 4
  store i32 10, ptr %24, align 4
  %25 = load ptr, ptr %__panic, align 8
  %26 = getelementptr i8, ptr %25, i64 4
  %27 = load i32, ptr %26, align 4
  ret i32 %27

poison.cont4:                                     ; preds = %poison.cont2
  %28 = call i32 @ExpressionSemantics_x3a_x3aenumLoweringPayloadValue(ptr noundef nonnull align 4 dereferenceable(8) %unit, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %29 = load ptr, ptr %__panic, align 8
  %30 = load i8, ptr %29, align 1
  %31 = icmp ne i8 %30, 0
  br i1 %31, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  %32 = load ptr, ptr %__panic, align 8
  %33 = getelementptr i8, ptr %32, i64 0
  %34 = load i8, ptr %33, align 1
  %35 = load ptr, ptr %__panic, align 8
  %36 = getelementptr i8, ptr %35, i64 4
  %37 = load i32, ptr %36, align 4
  %38 = load ptr, ptr %__panic, align 8
  %39 = getelementptr i8, ptr %38, i64 4
  %40 = load i32, ptr %39, align 4
  ret i32 %40

panic.cont:                                       ; preds = %poison.cont4
  %41 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %42 = icmp ne i8 %41, 0
  br i1 %42, label %poison.take5, label %poison.cont6

poison.take5:                                     ; preds = %panic.cont
  %43 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %43, align 1
  %44 = getelementptr i8, ptr %43, i64 4
  store i32 10, ptr %44, align 4
  %45 = load ptr, ptr %__panic, align 8
  %46 = getelementptr i8, ptr %45, i64 4
  %47 = load i32, ptr %46, align 4
  ret i32 %47

poison.cont6:                                     ; preds = %panic.cont
  %48 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %49 = icmp ne i8 %48, 0
  br i1 %49, label %poison.take7, label %poison.cont8

poison.take7:                                     ; preds = %poison.cont6
  %50 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %50, align 1
  %51 = getelementptr i8, ptr %50, i64 4
  store i32 10, ptr %51, align 4
  %52 = load ptr, ptr %__panic, align 8
  %53 = getelementptr i8, ptr %52, i64 4
  %54 = load i32, ptr %53, align 4
  ret i32 %54

poison.cont8:                                     ; preds = %poison.cont6
  %55 = call i32 @ExpressionSemantics_x3a_x3aenumLoweringPayloadValue(ptr noundef nonnull align 4 dereferenceable(8) %tuple, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %56 = load ptr, ptr %__panic, align 8
  %57 = load i8, ptr %56, align 1
  %58 = icmp ne i8 %57, 0
  br i1 %58, label %panic.take9, label %panic.cont10

panic.take9:                                      ; preds = %poison.cont8
  %59 = load ptr, ptr %__panic, align 8
  %60 = getelementptr i8, ptr %59, i64 0
  %61 = load i8, ptr %60, align 1
  %62 = load ptr, ptr %__panic, align 8
  %63 = getelementptr i8, ptr %62, i64 4
  %64 = load i32, ptr %63, align 4
  %65 = load ptr, ptr %__panic, align 8
  %66 = getelementptr i8, ptr %65, i64 4
  %67 = load i32, ptr %66, align 4
  ret i32 %67

panic.cont10:                                     ; preds = %poison.cont8
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %panic.cont10
  %68 = load ptr, ptr %__panic, align 8
  %69 = load i8, ptr %68, align 1
  %70 = icmp ne i8 %69, 0
  br i1 %70, label %panic.take11, label %panic.cont12

check_fail:                                       ; preds = %panic.cont10
  %71 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %71, align 1
  %72 = getelementptr i8, ptr %71, i64 4
  store i32 4, ptr %72, align 4
  br label %check_ok

panic.take11:                                     ; preds = %check_ok
  %73 = load ptr, ptr %__panic, align 8
  %74 = getelementptr i8, ptr %73, i64 0
  %75 = load i8, ptr %74, align 1
  %76 = load ptr, ptr %__panic, align 8
  %77 = getelementptr i8, ptr %76, i64 4
  %78 = load i32, ptr %77, align 4
  %79 = load ptr, ptr %__panic, align 8
  %80 = getelementptr i8, ptr %79, i64 4
  %81 = load i32, ptr %80, align 4
  ret i32 %81

panic.cont12:                                     ; preds = %check_ok
  %82 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %28, i32 %55)
  %83 = extractvalue { i32, i1 } %82, 0
  %84 = extractvalue { i32, i1 } %82, 1
  %85 = freeze i32 %83
  br i1 %84, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont12
  %86 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %87 = icmp ne i8 %86, 0
  br i1 %87, label %poison.take13, label %poison.cont14

op_fail:                                          ; preds = %panic.cont12
  %88 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %88, align 1
  %89 = getelementptr i8, ptr %88, i64 4
  store i32 4, ptr %89, align 4
  ret i32 0

poison.take13:                                    ; preds = %op_ok
  %90 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %90, align 1
  %91 = getelementptr i8, ptr %90, i64 4
  store i32 10, ptr %91, align 4
  %92 = load ptr, ptr %__panic, align 8
  %93 = getelementptr i8, ptr %92, i64 4
  %94 = load i32, ptr %93, align 4
  ret i32 %94

poison.cont14:                                    ; preds = %op_ok
  %95 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %96 = icmp ne i8 %95, 0
  br i1 %96, label %poison.take15, label %poison.cont16

poison.take15:                                    ; preds = %poison.cont14
  %97 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %97, align 1
  %98 = getelementptr i8, ptr %97, i64 4
  store i32 10, ptr %98, align 4
  %99 = load ptr, ptr %__panic, align 8
  %100 = getelementptr i8, ptr %99, i64 4
  %101 = load i32, ptr %100, align 4
  ret i32 %101

poison.cont16:                                    ; preds = %poison.cont14
  %102 = call i32 @ExpressionSemantics_x3a_x3aenumLoweringPayloadValue(ptr noundef nonnull align 4 dereferenceable(8) %named, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %103 = load ptr, ptr %__panic, align 8
  %104 = load i8, ptr %103, align 1
  %105 = icmp ne i8 %104, 0
  br i1 %105, label %panic.take17, label %panic.cont18

panic.take17:                                     ; preds = %poison.cont16
  %106 = load ptr, ptr %__panic, align 8
  %107 = getelementptr i8, ptr %106, i64 0
  %108 = load i8, ptr %107, align 1
  %109 = load ptr, ptr %__panic, align 8
  %110 = getelementptr i8, ptr %109, i64 4
  %111 = load i32, ptr %110, align 4
  %112 = load ptr, ptr %__panic, align 8
  %113 = getelementptr i8, ptr %112, i64 4
  %114 = load i32, ptr %113, align 4
  ret i32 %114

panic.cont18:                                     ; preds = %poison.cont16
  br i1 true, label %check_ok19, label %check_fail20

check_ok19:                                       ; preds = %check_fail20, %panic.cont18
  %115 = load ptr, ptr %__panic, align 8
  %116 = load i8, ptr %115, align 1
  %117 = icmp ne i8 %116, 0
  br i1 %117, label %panic.take21, label %panic.cont22

check_fail20:                                     ; preds = %panic.cont18
  %118 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %118, align 1
  %119 = getelementptr i8, ptr %118, i64 4
  store i32 4, ptr %119, align 4
  br label %check_ok19

panic.take21:                                     ; preds = %check_ok19
  %120 = load ptr, ptr %__panic, align 8
  %121 = getelementptr i8, ptr %120, i64 0
  %122 = load i8, ptr %121, align 1
  %123 = load ptr, ptr %__panic, align 8
  %124 = getelementptr i8, ptr %123, i64 4
  %125 = load i32, ptr %124, align 4
  %126 = load ptr, ptr %__panic, align 8
  %127 = getelementptr i8, ptr %126, i64 4
  %128 = load i32, ptr %127, align 4
  ret i32 %128

panic.cont22:                                     ; preds = %check_ok19
  %129 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %85, i32 %102)
  %130 = extractvalue { i32, i1 } %129, 0
  %131 = extractvalue { i32, i1 } %129, 1
  %132 = freeze i32 %130
  br i1 %131, label %op_fail24, label %op_ok23

op_ok23:                                          ; preds = %panic.cont22
  ret i32 %132

op_fail24:                                        ; preds = %panic.cont22
  %133 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %133, align 1
  %134 = getelementptr i8, ptr %133, i64 4
  store i32 4, ptr %134, align 4
  ret i32 0
}

define i32 @ExpressionSemantics_x3a_x3acontrolLoweringReference(ptr noundef nonnull align 1 dereferenceable(1) %flag, ptr noundef nonnull align 4 dereferenceable(8) %input, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %block_value = alloca i32, align 4
  %local_value = alloca i32, align 4
  %__bind_172_value.iter = alloca i32, align 4
  %iter.seq.idx = alloca i64, align 8
  %iter.seq.elem = alloca i32, align 4
  %iter_total = alloca i32, align 4
  %aggregate.literal = alloca [2 x i32], align 4
  %iter_values = alloca [2 x i32], align 4
  %conditional_count = alloca i32, align 4
  %infinite_count = alloca i32, align 4
  %if_cases_value = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3acontrolLoweringReference$tmp$if_case_clause_result_1168" = alloca i32, align 4
  %input11 = alloca i8, align 1
  %case_flag = alloca i8, align 1
  %"ExpressionSemantics_x3a_x3acontrolLoweringReference$tmp$if_case_clause_result_1162" = alloca i32, align 4
  %input8 = alloca i32, align 4
  %value7 = alloca i32, align 4
  %if_is_value = alloca i32, align 4
  %"ExpressionSemantics_x3a_x3acontrolLoweringReference$tmp$if_case_else_result_1158" = alloca i32, align 4
  %input3 = alloca i8, align 1
  %"ExpressionSemantics_x3a_x3acontrolLoweringReference$tmp$if_case_clause_result_1155" = alloca i32, align 4
  %input1 = alloca i32, align 4
  %value = alloca i32, align 4
  %if_value = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %9 = load i8, ptr %flag, align 1
  %10 = icmp ne i8 %9, 0
  %11 = load i8, ptr %flag, align 1
  %12 = icmp ne i8 %11, 0
  br i1 %12, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  br label %if.merge

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %"ExpressionSemantics_x3a_x3acontrolLoweringReference$tmp$if_1152" = phi i32 [ 1, %if.then ], [ 2, %if.else ]
  store i32 %"ExpressionSemantics_x3a_x3acontrolLoweringReference$tmp$if_1152", ptr %if_value, align 4
  %13 = load i8, ptr %input, align 1
  %14 = icmp eq i8 %13, 1
  br i1 %14, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %ifcase.case2, %ifcase.case
  %ifcase.result = phi i32 [ %25, %ifcase.case ], [ %31, %ifcase.case2 ]
  store i32 %ifcase.result, ptr %if_is_value, align 4
  %15 = load i8, ptr %input, align 1
  %16 = icmp eq i8 %15, 1
  br i1 %16, label %ifcase.case5, label %ifcase.next6

ifcase.case:                                      ; preds = %if.merge
  %17 = getelementptr i8, ptr %input, i64 4
  %18 = getelementptr i8, ptr %17, i64 0
  %19 = load i32, ptr %18, align 1
  store i32 %19, ptr %value, align 4
  %20 = getelementptr i8, ptr %input, i64 4
  %21 = getelementptr i8, ptr %20, i64 0
  %22 = load i32, ptr %21, align 1
  store i32 %22, ptr %input1, align 4
  %23 = load i32, ptr %value, align 4
  %24 = load i32, ptr %value, align 1
  store i32 %24, ptr %"ExpressionSemantics_x3a_x3acontrolLoweringReference$tmp$if_case_clause_result_1155", align 4
  %25 = load i32, ptr %"ExpressionSemantics_x3a_x3acontrolLoweringReference$tmp$if_case_clause_result_1155", align 4
  br label %ifcase.merge

ifcase.next:                                      ; preds = %if.merge
  br i1 true, label %ifcase.case2, label %ifcase.unmatched

ifcase.case2:                                     ; preds = %ifcase.next
  %26 = getelementptr i8, ptr %input, i64 4
  %27 = getelementptr i8, ptr %26, i64 0
  %28 = load i8, ptr %27, align 1
  %29 = icmp ne i8 %28, 0
  %30 = zext i1 %29 to i8
  store i8 %30, ptr %input3, align 1
  store i32 3, ptr %"ExpressionSemantics_x3a_x3acontrolLoweringReference$tmp$if_case_else_result_1158", align 4
  %31 = load i32, ptr %"ExpressionSemantics_x3a_x3acontrolLoweringReference$tmp$if_case_else_result_1158", align 4
  br label %ifcase.merge

ifcase.unmatched:                                 ; preds = %ifcase.next
  %32 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %32, align 1
  %33 = getelementptr i8, ptr %32, i64 4
  store i32 18, ptr %33, align 4
  %34 = load ptr, ptr %__panic, align 8
  %35 = getelementptr i8, ptr %34, i64 4
  %36 = load i32, ptr %35, align 4
  ret i32 %36

ifcase.merge4:                                    ; preds = %if.merge14, %ifcase.case5
  %ifcase.result15 = phi i32 [ %45, %ifcase.case5 ], [ %67, %if.merge14 ]
  store i32 %ifcase.result15, ptr %if_cases_value, align 4
  store i32 0, ptr %infinite_count, align 4
  br label %loop.head

ifcase.case5:                                     ; preds = %ifcase.merge
  %37 = getelementptr i8, ptr %input, i64 4
  %38 = getelementptr i8, ptr %37, i64 0
  %39 = load i32, ptr %38, align 1
  store i32 %39, ptr %value7, align 4
  %40 = getelementptr i8, ptr %input, i64 4
  %41 = getelementptr i8, ptr %40, i64 0
  %42 = load i32, ptr %41, align 1
  store i32 %42, ptr %input8, align 4
  %43 = load i32, ptr %value7, align 4
  %44 = load i32, ptr %value7, align 1
  store i32 %44, ptr %"ExpressionSemantics_x3a_x3acontrolLoweringReference$tmp$if_case_clause_result_1162", align 4
  %45 = load i32, ptr %"ExpressionSemantics_x3a_x3acontrolLoweringReference$tmp$if_case_clause_result_1162", align 4
  br label %ifcase.merge4

ifcase.next6:                                     ; preds = %ifcase.merge
  %46 = load i8, ptr %input, align 1
  %47 = icmp eq i8 %46, 0
  br i1 %47, label %ifcase.case9, label %ifcase.unmatched10

ifcase.case9:                                     ; preds = %ifcase.next6
  %48 = getelementptr i8, ptr %input, i64 4
  %49 = getelementptr i8, ptr %48, i64 0
  %50 = load i8, ptr %49, align 1
  %51 = icmp ne i8 %50, 0
  %52 = zext i1 %51 to i8
  store i8 %52, ptr %case_flag, align 1
  %53 = getelementptr i8, ptr %input, i64 4
  %54 = getelementptr i8, ptr %53, i64 0
  %55 = load i8, ptr %54, align 1
  %56 = icmp ne i8 %55, 0
  %57 = zext i1 %56 to i8
  store i8 %57, ptr %input11, align 1
  %58 = load i8, ptr %case_flag, align 1
  %59 = icmp ne i8 %58, 0
  %60 = load i8, ptr %case_flag, align 1
  %61 = icmp ne i8 %60, 0
  br i1 %61, label %if.then12, label %if.else13

ifcase.unmatched10:                               ; preds = %ifcase.next6
  %62 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %62, align 1
  %63 = getelementptr i8, ptr %62, i64 4
  store i32 18, ptr %63, align 4
  %64 = load ptr, ptr %__panic, align 8
  %65 = getelementptr i8, ptr %64, i64 4
  %66 = load i32, ptr %65, align 4
  ret i32 %66

if.then12:                                        ; preds = %ifcase.case9
  br label %if.merge14

if.else13:                                        ; preds = %ifcase.case9
  br label %if.merge14

if.merge14:                                       ; preds = %if.else13, %if.then12
  %"ExpressionSemantics_x3a_x3acontrolLoweringReference$tmp$if_1167" = phi i32 [ 4, %if.then12 ], [ 5, %if.else13 ]
  store i32 %"ExpressionSemantics_x3a_x3acontrolLoweringReference$tmp$if_1167", ptr %"ExpressionSemantics_x3a_x3acontrolLoweringReference$tmp$if_case_clause_result_1168", align 4
  %67 = load i32, ptr %"ExpressionSemantics_x3a_x3acontrolLoweringReference$tmp$if_case_clause_result_1168", align 4
  br label %ifcase.merge4

loop.end:                                         ; preds = %if.then16
  store i32 0, ptr %conditional_count, align 4
  br label %loop.cond

loop.head:                                        ; preds = %if.merge18, %ifcase.merge4
  br label %loop.body

loop.body:                                        ; preds = %loop.head
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %loop.body
  %68 = load ptr, ptr %__panic, align 8
  %69 = load i8, ptr %68, align 1
  %70 = icmp ne i8 %69, 0
  br i1 %70, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %loop.body
  %71 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %71, align 1
  %72 = getelementptr i8, ptr %71, i64 4
  store i32 4, ptr %72, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %73 = load ptr, ptr %__panic, align 8
  %74 = getelementptr i8, ptr %73, i64 0
  %75 = load i8, ptr %74, align 1
  %76 = load ptr, ptr %__panic, align 8
  %77 = getelementptr i8, ptr %76, i64 4
  %78 = load i32, ptr %77, align 4
  %79 = load ptr, ptr %__panic, align 8
  %80 = getelementptr i8, ptr %79, i64 4
  %81 = load i32, ptr %80, align 4
  ret i32 %81

panic.cont:                                       ; preds = %check_ok
  %82 = load i32, ptr %infinite_count, align 4
  %83 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %82, i32 1)
  %84 = extractvalue { i32, i1 } %83, 0
  %85 = extractvalue { i32, i1 } %83, 1
  %86 = freeze i32 %84
  br i1 %85, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  store i32 %86, ptr %infinite_count, align 4
  %87 = load i32, ptr %infinite_count, align 4
  %88 = icmp eq i32 %87, 1
  %89 = zext i1 %88 to i8
  %90 = icmp ne i8 %89, 0
  %91 = icmp ne i8 %89, 0
  br i1 %91, label %if.then16, label %if.else17

op_fail:                                          ; preds = %panic.cont
  %92 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %92, align 1
  %93 = getelementptr i8, ptr %92, i64 4
  store i32 4, ptr %93, align 4
  ret i32 0

if.then16:                                        ; preds = %op_ok
  br label %loop.end

if.else17:                                        ; preds = %op_ok
  br label %if.merge18

if.merge18:                                       ; preds = %if.else17
  br label %loop.head

loop.end19:                                       ; preds = %loop.cond
  %94 = getelementptr [2 x i32], ptr %aggregate.literal, i64 0, i64 0
  store i32 1, ptr %94, align 4
  %95 = getelementptr [2 x i32], ptr %aggregate.literal, i64 0, i64 1
  store i32 2, ptr %95, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %iter_values, ptr align 4 %aggregate.literal, i64 8, i1 false)
  store i32 0, ptr %iter_total, align 4
  store i64 0, ptr %iter.seq.idx, align 4
  store i32 0, ptr %iter.seq.elem, align 4
  br label %loop.cond28

loop.cond:                                        ; preds = %op_ok25, %loop.end
  %96 = load i32, ptr %conditional_count, align 4
  %97 = icmp slt i32 %96, 1
  %98 = zext i1 %97 to i8
  %99 = icmp ne i8 %98, 0
  br i1 %99, label %loop.body20, label %loop.end19

loop.body20:                                      ; preds = %loop.cond
  br i1 true, label %check_ok21, label %check_fail22

check_ok21:                                       ; preds = %check_fail22, %loop.body20
  %100 = load ptr, ptr %__panic, align 8
  %101 = load i8, ptr %100, align 1
  %102 = icmp ne i8 %101, 0
  br i1 %102, label %panic.take23, label %panic.cont24

check_fail22:                                     ; preds = %loop.body20
  %103 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %103, align 1
  %104 = getelementptr i8, ptr %103, i64 4
  store i32 4, ptr %104, align 4
  br label %check_ok21

panic.take23:                                     ; preds = %check_ok21
  %105 = load ptr, ptr %__panic, align 8
  %106 = getelementptr i8, ptr %105, i64 0
  %107 = load i8, ptr %106, align 1
  %108 = load ptr, ptr %__panic, align 8
  %109 = getelementptr i8, ptr %108, i64 4
  %110 = load i32, ptr %109, align 4
  %111 = load ptr, ptr %__panic, align 8
  %112 = getelementptr i8, ptr %111, i64 4
  %113 = load i32, ptr %112, align 4
  ret i32 %113

panic.cont24:                                     ; preds = %check_ok21
  %114 = load i32, ptr %conditional_count, align 4
  %115 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %114, i32 1)
  %116 = extractvalue { i32, i1 } %115, 0
  %117 = extractvalue { i32, i1 } %115, 1
  %118 = freeze i32 %116
  br i1 %117, label %op_fail26, label %op_ok25

op_ok25:                                          ; preds = %panic.cont24
  store i32 %118, ptr %conditional_count, align 4
  br label %loop.cond

op_fail26:                                        ; preds = %panic.cont24
  %119 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %119, align 1
  %120 = getelementptr i8, ptr %119, i64 4
  store i32 4, ptr %120, align 4
  ret i32 0

loop.end27:                                       ; preds = %iter.seq.next.cont
  store i32 6, ptr %local_value, align 4
  br i1 true, label %check_ok36, label %check_fail37

loop.cond28:                                      ; preds = %loop.inc, %loop.end19
  %121 = load i64, ptr %iter.seq.idx, align 4
  %122 = icmp ult i64 %121, 2
  br i1 %122, label %iter.seq.next.ok, label %iter.seq.next.done

loop.body29:                                      ; preds = %iter.seq.next.cont
  %123 = load i32, ptr %iter.seq.elem, align 4
  store i32 %123, ptr %__bind_172_value.iter, align 4
  br i1 true, label %check_ok30, label %check_fail31

loop.inc:                                         ; preds = %op_ok34
  br label %loop.cond28

iter.seq.next.ok:                                 ; preds = %loop.cond28
  %124 = getelementptr [2 x i32], ptr %iter_values, i64 0, i64 %121
  %125 = load i32, ptr %124, align 4
  store i32 %125, ptr %iter.seq.elem, align 4
  %126 = add i64 %121, 1
  store i64 %126, ptr %iter.seq.idx, align 4
  br label %iter.seq.next.cont

iter.seq.next.done:                               ; preds = %loop.cond28
  br label %iter.seq.next.cont

iter.seq.next.cont:                               ; preds = %iter.seq.next.done, %iter.seq.next.ok
  %127 = phi i1 [ true, %iter.seq.next.ok ], [ false, %iter.seq.next.done ]
  br i1 %127, label %loop.body29, label %loop.end27

check_ok30:                                       ; preds = %check_fail31, %loop.body29
  %128 = load ptr, ptr %__panic, align 8
  %129 = load i8, ptr %128, align 1
  %130 = icmp ne i8 %129, 0
  br i1 %130, label %panic.take32, label %panic.cont33

check_fail31:                                     ; preds = %loop.body29
  %131 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %131, align 1
  %132 = getelementptr i8, ptr %131, i64 4
  store i32 4, ptr %132, align 4
  br label %check_ok30

panic.take32:                                     ; preds = %check_ok30
  %133 = load ptr, ptr %__panic, align 8
  %134 = getelementptr i8, ptr %133, i64 0
  %135 = load i8, ptr %134, align 1
  %136 = load ptr, ptr %__panic, align 8
  %137 = getelementptr i8, ptr %136, i64 4
  %138 = load i32, ptr %137, align 4
  %139 = load ptr, ptr %__panic, align 8
  %140 = getelementptr i8, ptr %139, i64 4
  %141 = load i32, ptr %140, align 4
  ret i32 %141

panic.cont33:                                     ; preds = %check_ok30
  %142 = load i32, ptr %iter_total, align 4
  %143 = load i32, ptr %__bind_172_value.iter, align 4
  %144 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %142, i32 %143)
  %145 = extractvalue { i32, i1 } %144, 0
  %146 = extractvalue { i32, i1 } %144, 1
  %147 = freeze i32 %145
  br i1 %146, label %op_fail35, label %op_ok34

op_ok34:                                          ; preds = %panic.cont33
  store i32 %147, ptr %iter_total, align 4
  br label %loop.inc

op_fail35:                                        ; preds = %panic.cont33
  %148 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %148, align 1
  %149 = getelementptr i8, ptr %148, i64 4
  store i32 4, ptr %149, align 4
  ret i32 0

check_ok36:                                       ; preds = %check_fail37, %loop.end27
  %150 = load ptr, ptr %__panic, align 8
  %151 = load i8, ptr %150, align 1
  %152 = icmp ne i8 %151, 0
  br i1 %152, label %panic.take38, label %panic.cont39

check_fail37:                                     ; preds = %loop.end27
  %153 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %153, align 1
  %154 = getelementptr i8, ptr %153, i64 4
  store i32 4, ptr %154, align 4
  br label %check_ok36

panic.take38:                                     ; preds = %check_ok36
  %155 = load ptr, ptr %__panic, align 8
  %156 = getelementptr i8, ptr %155, i64 0
  %157 = load i8, ptr %156, align 1
  %158 = load ptr, ptr %__panic, align 8
  %159 = getelementptr i8, ptr %158, i64 4
  %160 = load i32, ptr %159, align 4
  %161 = load ptr, ptr %__panic, align 8
  %162 = getelementptr i8, ptr %161, i64 4
  %163 = load i32, ptr %162, align 4
  ret i32 %163

panic.cont39:                                     ; preds = %check_ok36
  %164 = load i32, ptr %local_value, align 4
  %165 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %164, i32 1)
  %166 = extractvalue { i32, i1 } %165, 0
  %167 = extractvalue { i32, i1 } %165, 1
  %168 = freeze i32 %166
  br i1 %167, label %op_fail41, label %op_ok40

op_ok40:                                          ; preds = %panic.cont39
  store i32 %168, ptr %block_value, align 4
  br i1 true, label %check_ok42, label %check_fail43

op_fail41:                                        ; preds = %panic.cont39
  %169 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %169, align 1
  %170 = getelementptr i8, ptr %169, i64 4
  store i32 4, ptr %170, align 4
  ret i32 0

check_ok42:                                       ; preds = %check_fail43, %op_ok40
  %171 = load ptr, ptr %__panic, align 8
  %172 = load i8, ptr %171, align 1
  %173 = icmp ne i8 %172, 0
  br i1 %173, label %panic.take44, label %panic.cont45

check_fail43:                                     ; preds = %op_ok40
  %174 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %174, align 1
  %175 = getelementptr i8, ptr %174, i64 4
  store i32 4, ptr %175, align 4
  br label %check_ok42

panic.take44:                                     ; preds = %check_ok42
  %176 = load ptr, ptr %__panic, align 8
  %177 = getelementptr i8, ptr %176, i64 0
  %178 = load i8, ptr %177, align 1
  %179 = load ptr, ptr %__panic, align 8
  %180 = getelementptr i8, ptr %179, i64 4
  %181 = load i32, ptr %180, align 4
  %182 = load ptr, ptr %__panic, align 8
  %183 = getelementptr i8, ptr %182, i64 4
  %184 = load i32, ptr %183, align 4
  ret i32 %184

panic.cont45:                                     ; preds = %check_ok42
  %185 = load i32, ptr %if_value, align 4
  %186 = load i32, ptr %if_is_value, align 4
  %187 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %185, i32 %186)
  %188 = extractvalue { i32, i1 } %187, 0
  %189 = extractvalue { i32, i1 } %187, 1
  %190 = freeze i32 %188
  br i1 %189, label %op_fail47, label %op_ok46

op_ok46:                                          ; preds = %panic.cont45
  br i1 true, label %check_ok48, label %check_fail49

op_fail47:                                        ; preds = %panic.cont45
  %191 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %191, align 1
  %192 = getelementptr i8, ptr %191, i64 4
  store i32 4, ptr %192, align 4
  ret i32 0

check_ok48:                                       ; preds = %check_fail49, %op_ok46
  %193 = load ptr, ptr %__panic, align 8
  %194 = load i8, ptr %193, align 1
  %195 = icmp ne i8 %194, 0
  br i1 %195, label %panic.take50, label %panic.cont51

check_fail49:                                     ; preds = %op_ok46
  %196 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %196, align 1
  %197 = getelementptr i8, ptr %196, i64 4
  store i32 4, ptr %197, align 4
  br label %check_ok48

panic.take50:                                     ; preds = %check_ok48
  %198 = load ptr, ptr %__panic, align 8
  %199 = getelementptr i8, ptr %198, i64 0
  %200 = load i8, ptr %199, align 1
  %201 = load ptr, ptr %__panic, align 8
  %202 = getelementptr i8, ptr %201, i64 4
  %203 = load i32, ptr %202, align 4
  %204 = load ptr, ptr %__panic, align 8
  %205 = getelementptr i8, ptr %204, i64 4
  %206 = load i32, ptr %205, align 4
  ret i32 %206

panic.cont51:                                     ; preds = %check_ok48
  %207 = load i32, ptr %if_cases_value, align 4
  %208 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %190, i32 %207)
  %209 = extractvalue { i32, i1 } %208, 0
  %210 = extractvalue { i32, i1 } %208, 1
  %211 = freeze i32 %209
  br i1 %210, label %op_fail53, label %op_ok52

op_ok52:                                          ; preds = %panic.cont51
  br i1 true, label %check_ok54, label %check_fail55

op_fail53:                                        ; preds = %panic.cont51
  %212 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %212, align 1
  %213 = getelementptr i8, ptr %212, i64 4
  store i32 4, ptr %213, align 4
  ret i32 0

check_ok54:                                       ; preds = %check_fail55, %op_ok52
  %214 = load ptr, ptr %__panic, align 8
  %215 = load i8, ptr %214, align 1
  %216 = icmp ne i8 %215, 0
  br i1 %216, label %panic.take56, label %panic.cont57

check_fail55:                                     ; preds = %op_ok52
  %217 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %217, align 1
  %218 = getelementptr i8, ptr %217, i64 4
  store i32 4, ptr %218, align 4
  br label %check_ok54

panic.take56:                                     ; preds = %check_ok54
  %219 = load ptr, ptr %__panic, align 8
  %220 = getelementptr i8, ptr %219, i64 0
  %221 = load i8, ptr %220, align 1
  %222 = load ptr, ptr %__panic, align 8
  %223 = getelementptr i8, ptr %222, i64 4
  %224 = load i32, ptr %223, align 4
  %225 = load ptr, ptr %__panic, align 8
  %226 = getelementptr i8, ptr %225, i64 4
  %227 = load i32, ptr %226, align 4
  ret i32 %227

panic.cont57:                                     ; preds = %check_ok54
  %228 = load i32, ptr %infinite_count, align 4
  %229 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %211, i32 %228)
  %230 = extractvalue { i32, i1 } %229, 0
  %231 = extractvalue { i32, i1 } %229, 1
  %232 = freeze i32 %230
  br i1 %231, label %op_fail59, label %op_ok58

op_ok58:                                          ; preds = %panic.cont57
  br i1 true, label %check_ok60, label %check_fail61

op_fail59:                                        ; preds = %panic.cont57
  %233 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %233, align 1
  %234 = getelementptr i8, ptr %233, i64 4
  store i32 4, ptr %234, align 4
  ret i32 0

check_ok60:                                       ; preds = %check_fail61, %op_ok58
  %235 = load ptr, ptr %__panic, align 8
  %236 = load i8, ptr %235, align 1
  %237 = icmp ne i8 %236, 0
  br i1 %237, label %panic.take62, label %panic.cont63

check_fail61:                                     ; preds = %op_ok58
  %238 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %238, align 1
  %239 = getelementptr i8, ptr %238, i64 4
  store i32 4, ptr %239, align 4
  br label %check_ok60

panic.take62:                                     ; preds = %check_ok60
  %240 = load ptr, ptr %__panic, align 8
  %241 = getelementptr i8, ptr %240, i64 0
  %242 = load i8, ptr %241, align 1
  %243 = load ptr, ptr %__panic, align 8
  %244 = getelementptr i8, ptr %243, i64 4
  %245 = load i32, ptr %244, align 4
  %246 = load ptr, ptr %__panic, align 8
  %247 = getelementptr i8, ptr %246, i64 4
  %248 = load i32, ptr %247, align 4
  ret i32 %248

panic.cont63:                                     ; preds = %check_ok60
  %249 = load i32, ptr %conditional_count, align 4
  %250 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %232, i32 %249)
  %251 = extractvalue { i32, i1 } %250, 0
  %252 = extractvalue { i32, i1 } %250, 1
  %253 = freeze i32 %251
  br i1 %252, label %op_fail65, label %op_ok64

op_ok64:                                          ; preds = %panic.cont63
  br i1 true, label %check_ok66, label %check_fail67

op_fail65:                                        ; preds = %panic.cont63
  %254 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %254, align 1
  %255 = getelementptr i8, ptr %254, i64 4
  store i32 4, ptr %255, align 4
  ret i32 0

check_ok66:                                       ; preds = %check_fail67, %op_ok64
  %256 = load ptr, ptr %__panic, align 8
  %257 = load i8, ptr %256, align 1
  %258 = icmp ne i8 %257, 0
  br i1 %258, label %panic.take68, label %panic.cont69

check_fail67:                                     ; preds = %op_ok64
  %259 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %259, align 1
  %260 = getelementptr i8, ptr %259, i64 4
  store i32 4, ptr %260, align 4
  br label %check_ok66

panic.take68:                                     ; preds = %check_ok66
  %261 = load ptr, ptr %__panic, align 8
  %262 = getelementptr i8, ptr %261, i64 0
  %263 = load i8, ptr %262, align 1
  %264 = load ptr, ptr %__panic, align 8
  %265 = getelementptr i8, ptr %264, i64 4
  %266 = load i32, ptr %265, align 4
  %267 = load ptr, ptr %__panic, align 8
  %268 = getelementptr i8, ptr %267, i64 4
  %269 = load i32, ptr %268, align 4
  ret i32 %269

panic.cont69:                                     ; preds = %check_ok66
  %270 = load i32, ptr %iter_total, align 4
  %271 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %253, i32 %270)
  %272 = extractvalue { i32, i1 } %271, 0
  %273 = extractvalue { i32, i1 } %271, 1
  %274 = freeze i32 %272
  br i1 %273, label %op_fail71, label %op_ok70

op_ok70:                                          ; preds = %panic.cont69
  br i1 true, label %check_ok72, label %check_fail73

op_fail71:                                        ; preds = %panic.cont69
  %275 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %275, align 1
  %276 = getelementptr i8, ptr %275, i64 4
  store i32 4, ptr %276, align 4
  ret i32 0

check_ok72:                                       ; preds = %check_fail73, %op_ok70
  %277 = load ptr, ptr %__panic, align 8
  %278 = load i8, ptr %277, align 1
  %279 = icmp ne i8 %278, 0
  br i1 %279, label %panic.take74, label %panic.cont75

check_fail73:                                     ; preds = %op_ok70
  %280 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %280, align 1
  %281 = getelementptr i8, ptr %280, i64 4
  store i32 4, ptr %281, align 4
  br label %check_ok72

panic.take74:                                     ; preds = %check_ok72
  %282 = load ptr, ptr %__panic, align 8
  %283 = getelementptr i8, ptr %282, i64 0
  %284 = load i8, ptr %283, align 1
  %285 = load ptr, ptr %__panic, align 8
  %286 = getelementptr i8, ptr %285, i64 4
  %287 = load i32, ptr %286, align 4
  %288 = load ptr, ptr %__panic, align 8
  %289 = getelementptr i8, ptr %288, i64 4
  %290 = load i32, ptr %289, align 4
  ret i32 %290

panic.cont75:                                     ; preds = %check_ok72
  %291 = load i32, ptr %block_value, align 4
  %292 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %274, i32 %291)
  %293 = extractvalue { i32, i1 } %292, 0
  %294 = extractvalue { i32, i1 } %292, 1
  %295 = freeze i32 %293
  br i1 %294, label %op_fail77, label %op_ok76

op_ok76:                                          ; preds = %panic.cont75
  ret i32 %295

op_fail77:                                        ; preds = %panic.cont75
  %296 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %296, align 1
  %297 = getelementptr i8, ptr %296, i64 4
  store i32 4, ptr %297, align 4
  ret i32 0
}

define i8 @ExpressionSemantics_x3a_x3aoperatorUnaryTypingReference(ptr noundef nonnull align 1 dereferenceable(1) %flag, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %float_value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %negative_float = alloca double, align 8
  %negative_int = alloca i32, align 4
  %bitwise_not = alloca i32, align 4
  %logical_not = alloca i8, align 1
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  %10 = load i8, ptr %flag, align 1
  %11 = icmp ne i8 %10, 0
  %12 = xor i1 %11, true
  %13 = zext i1 %12 to i8
  %14 = icmp ne i8 %13, 0
  %15 = zext i1 %14 to i8
  store i8 %15, ptr %logical_not, align 1
  %16 = load i32, ptr %value, align 4
  %17 = xor i32 %16, -1
  store i32 %17, ptr %bitwise_not, align 4
  %18 = load i32, ptr %value, align 4
  %19 = icmp ne i32 %18, -2147483648
  br i1 %19, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %20 = load ptr, ptr %__panic, align 8
  %21 = load i8, ptr %20, align 1
  %22 = icmp ne i8 %21, 0
  br i1 %22, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %23 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %23, align 1
  %24 = getelementptr i8, ptr %23, i64 4
  store i32 4, ptr %24, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %25 = load ptr, ptr %__panic, align 8
  %26 = getelementptr i8, ptr %25, i64 0
  %27 = load i8, ptr %26, align 1
  %28 = load ptr, ptr %__panic, align 8
  %29 = getelementptr i8, ptr %28, i64 4
  %30 = load i32, ptr %29, align 4
  %31 = load ptr, ptr %__panic, align 8
  %32 = getelementptr i8, ptr %31, i64 4
  %33 = load i32, ptr %32, align 4
  %34 = trunc i32 %33 to i8
  ret i8 %34

panic.cont:                                       ; preds = %check_ok
  %35 = load i32, ptr %value, align 4
  %36 = sub i32 0, %35
  store i32 %36, ptr %negative_int, align 4
  %37 = load double, ptr %float_value, align 8
  %38 = fneg double %37
  store double %38, ptr %negative_float, align 8
  %39 = load i8, ptr %logical_not, align 1
  %40 = icmp ne i8 %39, 0
  %41 = load i8, ptr %logical_not, align 1
  %42 = icmp ne i8 %41, 0
  br i1 %42, label %if.then, label %if.else

if.then:                                          ; preds = %panic.cont
  br label %if.merge

if.else:                                          ; preds = %panic.cont
  %43 = load i32, ptr %bitwise_not, align 4
  %44 = load i32, ptr %negative_int, align 4
  %45 = icmp eq i32 %43, %44
  %46 = zext i1 %45 to i8
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %"ExpressionSemantics_x3a_x3aoperatorUnaryTypingReference$tmp$or_1272" = phi i8 [ 1, %if.then ], [ %46, %if.else ]
  %47 = icmp ne i8 %"ExpressionSemantics_x3a_x3aoperatorUnaryTypingReference$tmp$or_1272", 0
  %48 = icmp ne i8 %"ExpressionSemantics_x3a_x3aoperatorUnaryTypingReference$tmp$or_1272", 0
  br i1 %48, label %if.then1, label %if.else2

if.then1:                                         ; preds = %if.merge
  br label %if.merge3

if.else2:                                         ; preds = %if.merge
  %49 = load double, ptr %negative_float, align 8
  %50 = fcmp olt double %49, 0.000000e+00
  %51 = zext i1 %50 to i8
  br label %if.merge3

if.merge3:                                        ; preds = %if.else2, %if.then1
  %"ExpressionSemantics_x3a_x3aoperatorUnaryTypingReference$tmp$or_1275" = phi i8 [ 1, %if.then1 ], [ %51, %if.else2 ]
  %52 = icmp ne i8 %"ExpressionSemantics_x3a_x3aoperatorUnaryTypingReference$tmp$or_1275", 0
  %53 = zext i1 %52 to i8
  ret i8 %53
}

define i8 @ExpressionSemantics_x3a_x3aoperatorBinaryTypingReference(ptr noundef nonnull align 4 dereferenceable(4) %left, ptr noundef nonnull align 4 dereferenceable(4) %right, ptr noundef nonnull align 4 dereferenceable(4) %shift_count, ptr noundef nonnull align 1 dereferenceable(1) %left_flag, ptr noundef nonnull align 1 dereferenceable(1) %right_flag, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %logical = alloca i8, align 1
  %ordered = alloca i8, align 1
  %equal = alloca i8, align 1
  %shifted = alloca i32, align 4
  %bitwise = alloca i32, align 4
  %arithmetic = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
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
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %10 = load ptr, ptr %__panic, align 8
  %11 = load i8, ptr %10, align 1
  %12 = icmp ne i8 %11, 0
  br i1 %12, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %13 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %13, align 1
  %14 = getelementptr i8, ptr %13, i64 4
  store i32 4, ptr %14, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %15 = load ptr, ptr %__panic, align 8
  %16 = getelementptr i8, ptr %15, i64 4
  %17 = load i32, ptr %16, align 4
  %18 = trunc i32 %17 to i8
  ret i8 %18

panic.cont:                                       ; preds = %check_ok
  %19 = load i32, ptr %left, align 4
  %20 = load i32, ptr %right, align 4
  %21 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %19, i32 %20)
  %22 = extractvalue { i32, i1 } %21, 0
  %23 = extractvalue { i32, i1 } %21, 1
  %24 = freeze i32 %22
  br i1 %23, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  store i32 %24, ptr %arithmetic, align 4
  %25 = load i32, ptr %left, align 4
  %26 = load i32, ptr %right, align 4
  %27 = and i32 %25, %26
  store i32 %27, ptr %bitwise, align 4
  %28 = load i32, ptr %left, align 4
  %29 = load i32, ptr %shift_count, align 4
  %30 = zext i32 %29 to i64
  %31 = icmp ult i64 %30, 32
  br i1 %31, label %check_ok1, label %check_fail2

op_fail:                                          ; preds = %panic.cont
  %32 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %32, align 1
  %33 = getelementptr i8, ptr %32, i64 4
  store i32 4, ptr %33, align 4
  ret i8 0

check_ok1:                                        ; preds = %check_fail2, %op_ok
  %34 = load ptr, ptr %__panic, align 8
  %35 = load i8, ptr %34, align 1
  %36 = icmp ne i8 %35, 0
  br i1 %36, label %panic.take3, label %panic.cont4

check_fail2:                                      ; preds = %op_ok
  %37 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %37, align 1
  %38 = getelementptr i8, ptr %37, i64 4
  store i32 5, ptr %38, align 4
  br label %check_ok1

panic.take3:                                      ; preds = %check_ok1
  %39 = load ptr, ptr %__panic, align 8
  %40 = getelementptr i8, ptr %39, i64 0
  %41 = load i8, ptr %40, align 1
  %42 = load ptr, ptr %__panic, align 8
  %43 = getelementptr i8, ptr %42, i64 4
  %44 = load i32, ptr %43, align 4
  %45 = load ptr, ptr %__panic, align 8
  %46 = getelementptr i8, ptr %45, i64 4
  %47 = load i32, ptr %46, align 4
  %48 = trunc i32 %47 to i8
  ret i8 %48

panic.cont4:                                      ; preds = %check_ok1
  %49 = load i32, ptr %left, align 4
  %50 = load i32, ptr %shift_count, align 4
  %51 = icmp ult i32 %50, 32
  br i1 %51, label %check_ok5, label %check_fail6

check_ok5:                                        ; preds = %panic.cont4
  %52 = shl i32 %49, %50
  %53 = freeze i32 %52
  store i32 %53, ptr %shifted, align 4
  %54 = load i32, ptr %left, align 4
  %55 = load i32, ptr %right, align 4
  %56 = icmp eq i32 %54, %55
  %57 = zext i1 %56 to i8
  %58 = icmp ne i8 %57, 0
  %59 = zext i1 %58 to i8
  store i8 %59, ptr %equal, align 1
  %60 = load i32, ptr %left, align 4
  %61 = load i32, ptr %right, align 4
  %62 = icmp slt i32 %60, %61
  %63 = zext i1 %62 to i8
  %64 = icmp ne i8 %63, 0
  %65 = zext i1 %64 to i8
  store i8 %65, ptr %ordered, align 1
  %66 = load i8, ptr %left_flag, align 1
  %67 = icmp ne i8 %66, 0
  %68 = load i8, ptr %left_flag, align 1
  %69 = icmp ne i8 %68, 0
  br i1 %69, label %if.then, label %if.else

check_fail6:                                      ; preds = %panic.cont4
  %70 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %70, align 1
  %71 = getelementptr i8, ptr %70, i64 4
  store i32 5, ptr %71, align 4
  %72 = load ptr, ptr %__panic, align 8
  %73 = getelementptr i8, ptr %72, i64 4
  %74 = load i32, ptr %73, align 4
  %75 = trunc i32 %74 to i8
  ret i8 %75

if.then:                                          ; preds = %check_ok5
  %76 = load i8, ptr %right_flag, align 1
  %77 = load i8, ptr %right_flag, align 1
  br label %if.merge

if.else:                                          ; preds = %check_ok5
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %"ExpressionSemantics_x3a_x3aoperatorBinaryTypingReference$tmp$and_1293" = phi i8 [ %77, %if.then ], [ 0, %if.else ]
  %78 = icmp ne i8 %"ExpressionSemantics_x3a_x3aoperatorBinaryTypingReference$tmp$and_1293", 0
  %79 = zext i1 %78 to i8
  store i8 %79, ptr %logical, align 1
  %80 = load i32, ptr %arithmetic, align 4
  %81 = load i32, ptr %bitwise, align 4
  %82 = icmp eq i32 %80, %81
  %83 = xor i1 %82, true
  %84 = zext i1 %83 to i8
  %85 = icmp ne i8 %84, 0
  %86 = icmp ne i8 %84, 0
  br i1 %86, label %if.then7, label %if.else8

if.then7:                                         ; preds = %if.merge
  br label %if.merge9

if.else8:                                         ; preds = %if.merge
  %87 = load i32, ptr %shifted, align 4
  %88 = load i32, ptr %left, align 4
  %89 = icmp eq i32 %87, %88
  %90 = zext i1 %89 to i8
  br label %if.merge9

if.merge9:                                        ; preds = %if.else8, %if.then7
  %"ExpressionSemantics_x3a_x3aoperatorBinaryTypingReference$tmp$or_1296" = phi i8 [ 1, %if.then7 ], [ %90, %if.else8 ]
  %91 = icmp ne i8 %"ExpressionSemantics_x3a_x3aoperatorBinaryTypingReference$tmp$or_1296", 0
  %92 = icmp ne i8 %"ExpressionSemantics_x3a_x3aoperatorBinaryTypingReference$tmp$or_1296", 0
  br i1 %92, label %if.then10, label %if.else11

if.then10:                                        ; preds = %if.merge9
  br label %if.merge12

if.else11:                                        ; preds = %if.merge9
  %93 = load i8, ptr %equal, align 1
  %94 = load i8, ptr %equal, align 1
  br label %if.merge12

if.merge12:                                       ; preds = %if.else11, %if.then10
  %"ExpressionSemantics_x3a_x3aoperatorBinaryTypingReference$tmp$or_1297" = phi i8 [ 1, %if.then10 ], [ %94, %if.else11 ]
  %95 = icmp ne i8 %"ExpressionSemantics_x3a_x3aoperatorBinaryTypingReference$tmp$or_1297", 0
  %96 = icmp ne i8 %"ExpressionSemantics_x3a_x3aoperatorBinaryTypingReference$tmp$or_1297", 0
  br i1 %96, label %if.then13, label %if.else14

if.then13:                                        ; preds = %if.merge12
  br label %if.merge15

if.else14:                                        ; preds = %if.merge12
  %97 = load i8, ptr %ordered, align 1
  %98 = load i8, ptr %ordered, align 1
  br label %if.merge15

if.merge15:                                       ; preds = %if.else14, %if.then13
  %"ExpressionSemantics_x3a_x3aoperatorBinaryTypingReference$tmp$or_1298" = phi i8 [ 1, %if.then13 ], [ %98, %if.else14 ]
  %99 = icmp ne i8 %"ExpressionSemantics_x3a_x3aoperatorBinaryTypingReference$tmp$or_1298", 0
  %100 = icmp ne i8 %"ExpressionSemantics_x3a_x3aoperatorBinaryTypingReference$tmp$or_1298", 0
  br i1 %100, label %if.then16, label %if.else17

if.then16:                                        ; preds = %if.merge15
  br label %if.merge18

if.else17:                                        ; preds = %if.merge15
  %101 = load i8, ptr %logical, align 1
  %102 = load i8, ptr %logical, align 1
  br label %if.merge18

if.merge18:                                       ; preds = %if.else17, %if.then16
  %"ExpressionSemantics_x3a_x3aoperatorBinaryTypingReference$tmp$or_1299" = phi i8 [ 1, %if.then16 ], [ %102, %if.else17 ]
  %103 = icmp ne i8 %"ExpressionSemantics_x3a_x3aoperatorBinaryTypingReference$tmp$or_1299", 0
  %104 = zext i1 %103 to i8
  ret i8 %104
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aExpressionSemantics(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  %2 = load ptr, ptr %__panic, align 8
  %3 = load i8, ptr %2, align 1
  %4 = icmp ne i8 %3, 0
  br i1 %4, label %init.panic.take, label %init.panic.cont

init.panic.take:                                  ; preds = %entry
  store i8 1, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aExpressionSemantics, align 1
  %5 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 10, ptr %6, align 4
  ret void

init.panic.cont:                                  ; preds = %entry
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aExpressionSemantics(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #2

declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3anew_x5fscoped(ptr)

declare ptr @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aalloc(ptr, ptr, ptr)

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #3

declare void @ultraviolet_x3a_x3aruntime_x3a_x3aasync_x3a_x3atake(ptr, ptr, i64, ptr)

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memmove.p0.p0.i64(ptr writeonly captures(none), ptr readonly captures(none), i64, i1 immarg) #3

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #4

define hidden void @__cx_lifecycle_init_ExpressionSemantics(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aExpressionSemantics(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_ExpressionSemantics(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aExpressionSemantics(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aExpressionSemantics(ptr %dll_attach_panic_out)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aExpressionSemantics(ptr %dll_detach_panic_out)
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
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #3 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #4 = { nocallback nofree nounwind willreturn memory(argmem: write) }
