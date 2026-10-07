; ==== Source/EmitLlLibrary.ll
; ModuleID = 'EmitLlLibrary'
source_filename = "EmitLlLibrary"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary = global i8 0
@vtable_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlDynamicDefaultOwner_x3a_x3acl_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlDynamicDefaultReadable = internal constant { i64, i64, ptr, ptr } { i64 4, i64 4, ptr @ultraviolet_x3a_x3aruntime_x3a_x3adrop_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlDynamicDefaultOwner, ptr @default_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlDynamicDefaultOwner_x3a_x3acl_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlDynamicDefaultReadable_x3a_x3adefaultValue }
@vtable_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlDynamicOwner_x3a_x3acl_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlDynamicReadable = internal constant { i64, i64, ptr, ptr, ptr } { i64 4, i64 4, ptr @ultraviolet_x3a_x3aruntime_x3a_x3adrop_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlDynamicOwner, ptr @EmitLlLibrary_x3a_x3aEmitLlDynamicOwner_x3a_x3areadValue, ptr @EmitLlLibrary_x3a_x3aEmitLlDynamicOwner_x3a_x3ashiftedValue }
@EmitLlLibrary_x3a_x3aEMIT_x5fLL_x5fSTATIC_x5fCLEANUP_x5fPROBE = global { i32, i32, [0 x i32] } zeroinitializer, align 4
@EmitLlLibrary_x3a_x3aEMIT_x5fLL_x5fCONST_x5fBYTES_x5fINT = constant i32 73, align 4
@EmitLlLibrary_x3a_x3aEMIT_x5fLL_x5fCONST_x5fBYTES_x5fFLOAT = constant double 6.250000e+00, align 8
@EmitLlLibrary_x3a_x3aEMIT_x5fLL_x5fCONST_x5fBYTES_x5fNULL = constant ptr null, align 8
@EmitLlLibrary_x3a_x3aEMIT_x5fLL_x5fCONST_x5fBYTES_x5fUNIT = constant {} zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fACD58AFCAAF42074 = internal constant [4 x i8] c"\11\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3abytes_x5f1AE6ABC406237AB7 = internal constant [18 x i8] c"EmitLlHashLawValue", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fA8C7F832281A39C5 = internal constant [8 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f1222D3BA48A5C686 = internal constant [8 x i8] c"\B3\01\00\00\00\01\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f89CD31291D2AEFA4 = internal constant [8 x i8] c"\01\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fECF582CAA5B1B50E = internal constant [4 x i8] c"\0B\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAF63BD4C8601B7DF = internal constant [1 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3afloat_x5fA876283227D4812D = internal constant [8 x i8] c"\00\00\00\00\00\00\18@", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAF63C54C8601C577 = internal constant [1 x i8] c"\08", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAF63B94C8601B113 = internal constant [1 x i8] c"\04", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAD2ACA7747985764 = internal constant [4 x i8] c"\01\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2DEA994B2809D300 = internal constant [4 x i8] c"%\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3achar_x5fAB808D12386344B4 = internal constant [4 x i8] c"Q\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3afloat_x5fA8C83832281AA685 = internal constant [8 x i8] c"\00\00\00\00\00\00\00@", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fADAAA9AF328EA9CC = internal constant [4 x i8] c")\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fEDA001BFDEFA22EE = internal constant [4 x i8] c"+\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAF63CD4C8601D30F = internal constant [1 x i8] c"\10", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f6D3572669B2CDE42 = internal constant [4 x i8] c"\07\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fED202287F403D086 = internal constant [4 x i8] c"\03\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fCD3AC65E44F721B1 = internal constant [4 x i8] c"\04\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2D401A55EEC16520 = internal constant [4 x i8] c"\05\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAF63D54C8601E0A7 = internal constant [1 x i8] c"\18", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAF639D4C8601817F = internal constant [1 x i8] c" ", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f9EBDE9309D2D8F4B = internal constant [8 x i8] c"\CE\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fD489AF8B0A867495 = internal constant [8 x i8] c"\D0\00\00\00\00\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2D157A98A06F49A8 = internal constant [4 x i8] c"\0D\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f8D1ACE904A398D17 = internal constant [4 x i8] c"\02\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f6CB5932EB0368BDA = internal constant [4 x i8] c"\1F\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f5239BDFD9C37AC77 = internal constant [15 x i8] c"E-HUV-LLVM-SRET", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f98910A3CC091FDAD = internal constant [31 x i8] c"regression.24.LLVMCallSRetAttrs", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fABAB2CCF86B5602C = internal constant [4 x i8] c"I\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3afloat_x5fA873423227D27784 = internal constant [8 x i8] c"\00\00\00\00\00\00\19@", align 1

; Function Attrs: nounwind
declare void @File_x3a_x3aWrite_x3a_x3aclose(ptr noundef nonnull align 8 dereferenceable(8)) #0

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3abytes_x3a_x3aas_x5fslice(ptr noundef nonnull align 8 dereferenceable(16)) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3abytes_x3a_x3adrop_x5fmanaged(ptr noundef nonnull align 8 dereferenceable(24)) #0

; Function Attrs: nounwind
declare i64 @ultraviolet_x3a_x3aruntime_x3a_x3abytes_x3a_x3alength(ptr noundef nonnull align 8 dereferenceable(16)) #0

; Function Attrs: nounwind
declare { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aio_x3a_x3acreate_x5fwrite(ptr noundef nonnull align 8 dereferenceable(16), ptr noundef nonnull align 8 dereferenceable(16)) #0

; Function Attrs: noreturn nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4)) #1

; Function Attrs: nounwind
declare i8 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5fis_x5factive(ptr) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5ftag_x5fscope(ptr, i64) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fenter(i64) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fexit(i64) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3astring_x3a_x3adrop_x5fmanaged(ptr noundef nonnull align 8 dereferenceable(24)) #0

; Function Attrs: nounwind
declare i64 @ultraviolet_x3a_x3aruntime_x3a_x3astring_x3a_x3alength(ptr noundef nonnull align 8 dereferenceable(16)) #0

; Function Attrs: nounwind
declare void @ultraviolet_x3a_x3aruntime_x3a_x3asystem_x3a_x3aexit(ptr noundef nonnull align 8 dereferenceable(16), ptr noundef nonnull align 4 dereferenceable(4)) #0

define void @EmitLlLibrary_x3a_x3aEmitLlDroppingFfiByValueRecord_x3a_x3adrop(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define void @EmitLlLibrary_x3a_x3aEmitLlCleanupProbe_x3a_x3adrop(ptr noundef nonnull align 4 dereferenceable(8) %self, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define i32 @EmitLlLibrary_x3a_x3aEmitLlPermissionAdmissibilityRecord_x3a_x3areadValue(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  %2 = getelementptr i8, ptr %self, i64 0
  %3 = load i32, ptr %2, align 1
  ret i32 %3
}

define i32 @EmitLlLibrary_x3a_x3aEmitLlPermissionAdmissibilityRecord_x3a_x3areplaceValue(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  %2 = getelementptr i8, ptr %self, i64 0
  %3 = getelementptr i8, ptr %self, i64 0
  %4 = load i32, ptr %value, align 4
  store i32 %4, ptr %3, align 4
  %5 = getelementptr i8, ptr %self, i64 0
  %6 = load i32, ptr %5, align 1
  ret i32 %6
}

define i32 @EmitLlLibrary_x3a_x3aEmitLlDynamicOwner_x3a_x3areadValue(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  %2 = getelementptr i8, ptr %self, i64 0
  %3 = load i32, ptr %2, align 1
  ret i32 %3
}

define i32 @EmitLlLibrary_x3a_x3aEmitLlDynamicOwner_x3a_x3ashiftedValue(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 4 dereferenceable(4) %delta, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
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
  %12 = load i32, ptr %delta, align 4
  %13 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %11, i32 %12)
  %14 = extractvalue { i32, i1 } %13, 0
  %15 = extractvalue { i32, i1 } %13, 1
  %16 = freeze i32 %14
  br i1 %15, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  ret i32 %16

op_fail:                                          ; preds = %panic.cont
  %17 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %17, align 1
  %18 = getelementptr i8, ptr %17, i64 4
  store i32 4, ptr %18, align 4
  ret i32 0
}

define i32 @default_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlDynamicDefaultOwner_x3a_x3acl_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlDynamicDefaultReadable_x3a_x3adefaultValue(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret i32 17
}

define i32 @default_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlFirstFieldOwner_x3a_x3acl_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlFirstFieldSelection_x3a_x3aselectedFirstField(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  %2 = getelementptr i8, ptr %self, i64 0
  %3 = load i32, ptr %2, align 1
  ret i32 %3
}

define i32 @EmitLlLibrary_x3a_x3aEmitLlAssociatedOwner_x3a_x3aassociatedCarrier(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
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
  %12 = load i32, ptr %value, align 4
  %13 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %11, i32 %12)
  %14 = extractvalue { i32, i1 } %13, 0
  %15 = extractvalue { i32, i1 } %13, 1
  %16 = freeze i32 %14
  br i1 %15, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  ret i32 %16

op_fail:                                          ; preds = %panic.cont
  %17 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %17, align 1
  %18 = getelementptr i8, ptr %17, i64 4
  store i32 4, ptr %18, align 4
  ret i32 0
}

define i32 @EmitLlLibrary_x3a_x3aEmitLlOpaqueOwner_x3a_x3areadOpaqueValue(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  %2 = getelementptr i8, ptr %self, i64 0
  %3 = load i32, ptr %2, align 1
  ret i32 %3
}

define i8 @EmitLlLibrary_x3a_x3aEmitLlHashLawValue_x3a_x3aeq(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 4 dereferenceable(4) %other, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  %2 = getelementptr i8, ptr %self, i64 0
  %3 = load i32, ptr %2, align 1
  %4 = getelementptr i8, ptr %other, i64 0
  %5 = load i32, ptr %4, align 1
  %6 = icmp eq i32 %3, %5
  %7 = zext i1 %6 to i8
  %8 = icmp ne i8 %7, 0
  %9 = zext i1 %8 to i8
  ret i8 %9
}

define void @EmitLlLibrary_x3a_x3aEmitLlHashLawValue_x3a_x3ahash(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 8 dereferenceable(16) %hasher, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"EmitLlLibrary_x3a_x3aEmitLlHashLawValue_x3a_x3ahash$tmp$call_ref_tmp_28" = alloca { ptr, i64 }, align 8
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3abytes_x5f1AE6ABC406237AB7, i64 18 }, ptr %"EmitLlLibrary_x3a_x3aEmitLlHashLawValue_x3a_x3ahash$tmp$call_ref_tmp_28", align 8
  %2 = load { ptr, ptr }, ptr %hasher, align 8
  %3 = extractvalue { ptr, ptr } %2, 0
  %4 = extractvalue { ptr, ptr } %2, 1
  %5 = getelementptr ptr, ptr %4, i64 3
  %6 = load ptr, ptr %5, align 8
  %7 = call i8 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5fis_x5factive(ptr %3)
  %8 = icmp ne i8 %7, 0
  br i1 %8, label %dynrecv.addr.valid, label %dynrecv.addr.expired

dynrecv.addr.valid:                               ; preds = %entry
  call void %6(ptr %3, ptr noundef nonnull align 8 dereferenceable(16) %"EmitLlLibrary_x3a_x3aEmitLlHashLawValue_x3a_x3ahash$tmp$call_ref_tmp_28", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  br label %dynrecv.addr.join

dynrecv.addr.expired:                             ; preds = %entry
  %9 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 9, ptr %10, align 4
  br label %dynrecv.addr.join

dynrecv.addr.join:                                ; preds = %dynrecv.addr.valid, %dynrecv.addr.expired
  %dynrecv.result = phi {} [ zeroinitializer, %dynrecv.addr.valid ], [ zeroinitializer, %dynrecv.addr.expired ]
  %11 = load ptr, ptr %__panic, align 8
  %12 = load i8, ptr %11, align 1
  %13 = icmp ne i8 %12, 0
  br i1 %13, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %dynrecv.addr.join
  ret void

panic.cont:                                       ; preds = %dynrecv.addr.join
  ret void
}

define void @EmitLlLibrary_x3a_x3aEmitLlFnvHasher_x3a_x3awrite(ptr noundef nonnull align 8 dereferenceable(8) %self, ptr noundef nonnull align 8 dereferenceable(16) %data, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %index = alloca i64, align 8
  %data_slice1 = alloca { ptr, i64 }, align 8
  %data_slice = alloca { ptr, i64 }, align 8
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  %bytes.as_slice = load { ptr, i64 }, ptr %data, align 8
  %2 = extractvalue { ptr, i64 } %bytes.as_slice, 0
  %3 = extractvalue { ptr, i64 } %bytes.as_slice, 1
  %4 = insertvalue { ptr, i64 } zeroinitializer, ptr %2, 0
  %5 = insertvalue { ptr, i64 } %4, i64 %3, 1
  store { ptr, i64 } %5, ptr %data_slice1, align 8
  store i64 0, ptr %index, align 4
  br label %loop.cond

loop.end:                                         ; preds = %loop.cond
  ret void

loop.cond:                                        ; preds = %op_ok12, %entry
  %view.prefix = load { ptr, i64 }, ptr %data, align 8
  %view.len = extractvalue { ptr, i64 } %view.prefix, 1
  %6 = load i64, ptr %index, align 4
  %7 = icmp ult i64 %6, %view.len
  %8 = zext i1 %7 to i8
  %9 = icmp ne i8 %8, 0
  br i1 %9, label %loop.body, label %loop.end

loop.body:                                        ; preds = %loop.cond
  %10 = load { ptr, i64 }, ptr %data_slice1, align 8
  %11 = extractvalue { ptr, i64 } %10, 1
  %12 = load i64, ptr %index, align 4
  %13 = icmp ult i64 %12, %11
  br i1 %13, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %loop.body
  %14 = load ptr, ptr %__panic, align 8
  %15 = load i8, ptr %14, align 1
  %16 = icmp ne i8 %15, 0
  br i1 %16, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %loop.body
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
  ret void

panic.cont:                                       ; preds = %check_ok
  %25 = load i64, ptr %index, align 4
  %26 = load { ptr, i64 }, ptr %data_slice1, align 8
  %27 = extractvalue { ptr, i64 } %26, 0
  %28 = getelementptr i8, ptr %27, i64 %25
  %29 = load i8, ptr %28, align 1
  %30 = load ptr, ptr %__panic, align 8
  %31 = load i8, ptr %30, align 1
  %32 = icmp ne i8 %31, 0
  br i1 %32, label %panic.take2, label %panic.cont3

panic.take2:                                      ; preds = %panic.cont
  %33 = load ptr, ptr %__panic, align 8
  %34 = getelementptr i8, ptr %33, i64 0
  %35 = load i8, ptr %34, align 1
  %36 = load ptr, ptr %__panic, align 8
  %37 = getelementptr i8, ptr %36, i64 4
  %38 = load i32, ptr %37, align 4
  ret void

panic.cont3:                                      ; preds = %panic.cont
  %39 = load i64, ptr %index, align 4
  %40 = load { ptr, i64 }, ptr %data_slice1, align 8
  %41 = extractvalue { ptr, i64 } %40, 0
  %42 = getelementptr i8, ptr %41, i64 %39
  %43 = load i8, ptr %42, align 1
  %44 = zext i8 %43 to i64
  %45 = getelementptr i8, ptr %self, i64 0
  %46 = load i64, ptr %45, align 1
  %47 = xor i64 %46, %44
  br i1 true, label %check_ok4, label %check_fail5

check_ok4:                                        ; preds = %check_fail5, %panic.cont3
  %48 = load ptr, ptr %__panic, align 8
  %49 = load i8, ptr %48, align 1
  %50 = icmp ne i8 %49, 0
  br i1 %50, label %panic.take6, label %panic.cont7

check_fail5:                                      ; preds = %panic.cont3
  %51 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %51, align 1
  %52 = getelementptr i8, ptr %51, i64 4
  store i32 4, ptr %52, align 4
  br label %check_ok4

panic.take6:                                      ; preds = %check_ok4
  %53 = load ptr, ptr %__panic, align 8
  %54 = getelementptr i8, ptr %53, i64 0
  %55 = load i8, ptr %54, align 1
  %56 = load ptr, ptr %__panic, align 8
  %57 = getelementptr i8, ptr %56, i64 4
  %58 = load i32, ptr %57, align 4
  ret void

panic.cont7:                                      ; preds = %check_ok4
  %59 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %47, i64 1099511628211)
  %60 = extractvalue { i64, i1 } %59, 0
  %61 = extractvalue { i64, i1 } %59, 1
  %62 = freeze i64 %60
  br i1 %61, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont7
  %63 = getelementptr i8, ptr %self, i64 0
  %64 = getelementptr i8, ptr %self, i64 0
  store i64 %62, ptr %64, align 4
  br i1 true, label %check_ok8, label %check_fail9

op_fail:                                          ; preds = %panic.cont7
  %65 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %65, align 1
  %66 = getelementptr i8, ptr %65, i64 4
  store i32 4, ptr %66, align 4
  ret void

check_ok8:                                        ; preds = %check_fail9, %op_ok
  %67 = load ptr, ptr %__panic, align 8
  %68 = load i8, ptr %67, align 1
  %69 = icmp ne i8 %68, 0
  br i1 %69, label %panic.take10, label %panic.cont11

check_fail9:                                      ; preds = %op_ok
  %70 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %70, align 1
  %71 = getelementptr i8, ptr %70, i64 4
  store i32 4, ptr %71, align 4
  br label %check_ok8

panic.take10:                                     ; preds = %check_ok8
  %72 = load ptr, ptr %__panic, align 8
  %73 = getelementptr i8, ptr %72, i64 0
  %74 = load i8, ptr %73, align 1
  %75 = load ptr, ptr %__panic, align 8
  %76 = getelementptr i8, ptr %75, i64 4
  %77 = load i32, ptr %76, align 4
  ret void

panic.cont11:                                     ; preds = %check_ok8
  %78 = load i64, ptr %index, align 4
  %79 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %78, i64 1)
  %80 = extractvalue { i64, i1 } %79, 0
  %81 = extractvalue { i64, i1 } %79, 1
  %82 = freeze i64 %80
  br i1 %81, label %op_fail13, label %op_ok12

op_ok12:                                          ; preds = %panic.cont11
  store i64 %82, ptr %index, align 4
  br label %loop.cond

op_fail13:                                        ; preds = %panic.cont11
  %83 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %83, align 1
  %84 = getelementptr i8, ptr %83, i64 4
  store i32 4, ptr %84, align 4
  ret void
}

define i64 @EmitLlLibrary_x3a_x3aEmitLlFnvHasher_x3a_x3afinish(ptr noundef nonnull align 8 dereferenceable(8) %self, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  %2 = getelementptr i8, ptr %self, i64 0
  %3 = load i64, ptr %2, align 1
  ret i64 %3
}

define i32 @EmitLlLibrary_x3a_x3aEmitLlTaggedModal_x3a_x3aSmall_x3a_x3areadValue(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  %2 = getelementptr i8, ptr %self, i64 0
  %3 = load i32, ptr %2, align 1
  ret i32 %3
}

define { i64, i32, [4 x i8], [0 x i64] } @EmitLlLibrary_x3a_x3aEmitLlTaggedModal_x3a_x3aSmall_x3a_x3afinishWide(ptr noundef nonnull align 4 dereferenceable(4) %self, ptr noundef nonnull align 4 dereferenceable(4) %right, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i64, i32, [4 x i8], [0 x i64] }, align 8
  %1 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %1, align 1
  %2 = getelementptr i8, ptr %1, i64 4
  store i32 0, ptr %2, align 4
  %3 = getelementptr i8, ptr %self, i64 0
  %4 = load i32, ptr %3, align 1
  %5 = load ptr, ptr %__panic, align 8
  %6 = load i8, ptr %5, align 1
  %7 = icmp ne i8 %6, 0
  br i1 %7, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  %9 = getelementptr i8, ptr %8, i64 0
  %10 = load i8, ptr %9, align 1
  %11 = load ptr, ptr %__panic, align 8
  %12 = getelementptr i8, ptr %11, i64 4
  %13 = load i32, ptr %12, align 4
  ret { i64, i32, [4 x i8], [0 x i64] } zeroinitializer

panic.cont:                                       ; preds = %entry
  %14 = getelementptr i8, ptr %self, i64 0
  %15 = load i32, ptr %14, align 1
  %16 = sext i32 %15 to i64
  store { i64, i32, [4 x i8], [0 x i64] } zeroinitializer, ptr %0, align 4
  %17 = getelementptr i8, ptr %0, i64 0
  store i64 %16, ptr %17, align 1
  %18 = load i32, ptr %right, align 4
  %19 = getelementptr i8, ptr %0, i64 8
  store i32 %18, ptr %19, align 1
  %20 = load { i64, i32, [4 x i8], [0 x i64] }, ptr %0, align 4
  ret { i64, i32, [4 x i8], [0 x i64] } %20
}

define i32 @EmitLlLibrary_x3a_x3aemitLlLibraryValue(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  ret i32 11
}

define i32 @EmitLlLibrary_x3a_x3aemitLlConstBytesEncodingProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %empty_text_size3 = alloca i64, align 8
  %empty_text_size = alloca i64, align 8
  %empty_text = alloca { ptr, i64 }, align 8
  %empty_size = alloca i64, align 8
  %empty_value = alloca {}, align 1
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %16 = load {}, ptr @EmitLlLibrary_x3a_x3aEMIT_x5fLL_x5fCONST_x5fBYTES_x5fUNIT, align 1
  store {} %16, ptr %empty_value, align 1
  store i64 0, ptr %empty_size, align 4
  store { ptr, i64 } zeroinitializer, ptr %empty_text, align 8
  %view.prefix = load { ptr, i64 }, ptr %empty_text, align 8
  %view.len = extractvalue { ptr, i64 } %view.prefix, 1
  store i64 %view.len, ptr %empty_text_size3, align 4
  %17 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %18 = icmp ne i8 %17, 0
  br i1 %18, label %poison.take4, label %poison.cont5

poison.take4:                                     ; preds = %poison.cont2
  %19 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %19, align 1
  %20 = getelementptr i8, ptr %19, i64 4
  store i32 10, ptr %20, align 4
  %21 = load ptr, ptr %__panic, align 8
  %22 = getelementptr i8, ptr %21, i64 4
  %23 = load i32, ptr %22, align 4
  ret i32 %23

poison.cont5:                                     ; preds = %poison.cont2
  %24 = load ptr, ptr @EmitLlLibrary_x3a_x3aEMIT_x5fLL_x5fCONST_x5fBYTES_x5fNULL, align 8
  %25 = icmp eq ptr %24, null
  %26 = zext i1 %25 to i8
  %27 = icmp ne i8 %26, 0
  %28 = icmp ne i8 %26, 0
  br i1 %28, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont5
  %29 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %30 = icmp ne i8 %29, 0
  br i1 %30, label %poison.take6, label %poison.cont7

if.else:                                          ; preds = %poison.cont5
  br label %if.merge

if.merge:                                         ; preds = %if.else, %poison.cont7
  %"EmitLlLibrary_x3a_x3aemitLlConstBytesEncodingProbe$tmp$and_110" = phi i8 [ %40, %poison.cont7 ], [ 0, %if.else ]
  %31 = icmp ne i8 %"EmitLlLibrary_x3a_x3aemitLlConstBytesEncodingProbe$tmp$and_110", 0
  %32 = icmp ne i8 %"EmitLlLibrary_x3a_x3aemitLlConstBytesEncodingProbe$tmp$and_110", 0
  br i1 %32, label %if.then8, label %if.else9

poison.take6:                                     ; preds = %if.then
  %33 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %33, align 1
  %34 = getelementptr i8, ptr %33, i64 4
  store i32 10, ptr %34, align 4
  %35 = load ptr, ptr %__panic, align 8
  %36 = getelementptr i8, ptr %35, i64 4
  %37 = load i32, ptr %36, align 4
  ret i32 %37

poison.cont7:                                     ; preds = %if.then
  %38 = load double, ptr @EmitLlLibrary_x3a_x3aEMIT_x5fLL_x5fCONST_x5fBYTES_x5fFLOAT, align 8
  %39 = fcmp ogt double %38, 6.000000e+00
  %40 = zext i1 %39 to i8
  br label %if.merge

if.then8:                                         ; preds = %if.merge
  %41 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %42 = icmp ne i8 %41, 0
  br i1 %42, label %poison.take11, label %poison.cont12

if.else9:                                         ; preds = %if.merge
  br label %if.merge10

if.merge10:                                       ; preds = %if.else9
  br i1 true, label %check_ok, label %check_fail

poison.take11:                                    ; preds = %if.then8
  %43 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %43, align 1
  %44 = getelementptr i8, ptr %43, i64 4
  store i32 10, ptr %44, align 4
  %45 = load ptr, ptr %__panic, align 8
  %46 = getelementptr i8, ptr %45, i64 4
  %47 = load i32, ptr %46, align 4
  ret i32 %47

poison.cont12:                                    ; preds = %if.then8
  %48 = load i32, ptr @EmitLlLibrary_x3a_x3aEMIT_x5fLL_x5fCONST_x5fBYTES_x5fINT, align 4
  ret i32 %48

check_ok:                                         ; preds = %check_fail, %if.merge10
  %49 = load ptr, ptr %__panic, align 8
  %50 = load i8, ptr %49, align 1
  %51 = icmp ne i8 %50, 0
  br i1 %51, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %if.merge10
  %52 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %52, align 1
  %53 = getelementptr i8, ptr %52, i64 4
  store i32 4, ptr %53, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %54 = load ptr, ptr %__panic, align 8
  %55 = getelementptr i8, ptr %54, i64 0
  %56 = load i8, ptr %55, align 1
  %57 = load ptr, ptr %__panic, align 8
  %58 = getelementptr i8, ptr %57, i64 4
  %59 = load i32, ptr %58, align 4
  %60 = load ptr, ptr %__panic, align 8
  %61 = getelementptr i8, ptr %60, i64 4
  %62 = load i32, ptr %61, align 4
  ret i32 %62

panic.cont:                                       ; preds = %check_ok
  %63 = load i64, ptr %empty_size, align 4
  %64 = load i64, ptr %empty_text_size3, align 4
  %65 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %63, i64 %64)
  %66 = extractvalue { i64, i1 } %65, 0
  %67 = extractvalue { i64, i1 } %65, 1
  %68 = freeze i64 %66
  br i1 %67, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  %69 = trunc i64 %68 to i32
  %70 = zext i32 %69 to i64
  %71 = icmp eq i64 %68, %70
  br i1 %71, label %check_ok13, label %check_fail14

op_fail:                                          ; preds = %panic.cont
  %72 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %72, align 1
  %73 = getelementptr i8, ptr %72, i64 4
  store i32 4, ptr %73, align 4
  ret i32 0

check_ok13:                                       ; preds = %check_fail14, %op_ok
  %74 = load ptr, ptr %__panic, align 8
  %75 = load i8, ptr %74, align 1
  %76 = icmp ne i8 %75, 0
  br i1 %76, label %panic.take15, label %panic.cont16

check_fail14:                                     ; preds = %op_ok
  %77 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %77, align 1
  %78 = getelementptr i8, ptr %77, i64 4
  store i32 7, ptr %78, align 4
  br label %check_ok13

panic.take15:                                     ; preds = %check_ok13
  %79 = load ptr, ptr %__panic, align 8
  %80 = getelementptr i8, ptr %79, i64 0
  %81 = load i8, ptr %80, align 1
  %82 = load ptr, ptr %__panic, align 8
  %83 = getelementptr i8, ptr %82, i64 4
  %84 = load i32, ptr %83, align 4
  %85 = load ptr, ptr %__panic, align 8
  %86 = getelementptr i8, ptr %85, i64 4
  %87 = load i32, ptr %86, align 4
  ret i32 %87

panic.cont16:                                     ; preds = %check_ok13
  %88 = trunc i64 %68 to i32
  ret i32 %88
}

define void @EmitLlLibrary_x3a_x3aemitLlCleanupNoop(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 0, ptr %5, align 4
  ret void
}

define i32 @EmitLlLibrary_x3a_x3aemitLlLocalDropCleanupProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %aggregate.literal = alloca { i32, i32, [0 x i32] }, align 4
  %payload = alloca { i32, i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  store i32 0, ptr %aggregate.literal, align 4
  %9 = getelementptr i8, ptr %aggregate.literal, i64 4
  %10 = load i32, ptr %value, align 4
  store i32 %10, ptr %9, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %payload, ptr align 4 %aggregate.literal, i64 8, i1 false)
  %11 = getelementptr i8, ptr %payload, i64 4
  %12 = load i32, ptr %11, align 1
  ret i32 %12
}

define i32 @EmitLlLibrary_x3a_x3aemitLlStaticCleanupProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i32, i32, [0 x i32] }, align 8
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %10 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  ret i32 %16

poison.cont2:                                     ; preds = %poison.cont
  %17 = load { i32, i32, [0 x i32] }, ptr @EmitLlLibrary_x3a_x3aEMIT_x5fLL_x5fSTATIC_x5fCLEANUP_x5fPROBE, align 4
  store { i32, i32, [0 x i32] } %17, ptr %0, align 4
  %18 = getelementptr i8, ptr %0, i64 4
  %19 = load i32, ptr %18, align 1
  ret i32 %19
}

define i32 @EmitLlLibrary_x3a_x3aemitLlDeferCleanupProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %byref_arg = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %9 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 0, ptr %10, align 4
  %11 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %12 = icmp ne i8 %11, 0
  br i1 %12, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %13 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %13, align 1
  %14 = getelementptr i8, ptr %13, i64 4
  store i32 10, ptr %14, align 4
  %15 = load ptr, ptr %__panic, align 8
  %16 = getelementptr i8, ptr %15, i64 4
  %17 = load i32, ptr %16, align 4
  ret i32 %17

poison.cont2:                                     ; preds = %poison.cont
  %18 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  ret i32 %24

poison.cont4:                                     ; preds = %poison.cont2
  call void @EmitLlLibrary_x3a_x3aemitLlCleanupNoop(ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %25 = load ptr, ptr %__panic, align 8
  %26 = load i8, ptr %25, align 1
  %27 = icmp ne i8 %26, 0
  br i1 %27, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  %28 = load ptr, ptr %__panic, align 8
  %29 = getelementptr i8, ptr %28, i64 4
  %30 = load i32, ptr %29, align 4
  ret i32 %30

panic.cont:                                       ; preds = %poison.cont4
  %31 = load ptr, ptr %__panic, align 8
  %32 = getelementptr i8, ptr %31, i64 0
  %33 = load i8, ptr %32, align 1
  %34 = load ptr, ptr %__panic, align 8
  %35 = getelementptr i8, ptr %34, i64 4
  %36 = load i32, ptr %35, align 4
  %37 = icmp ne i8 %33, 0
  %38 = zext i1 %37 to i8
  %39 = icmp ne i8 %38, 0
  %40 = and i1 false, %39
  br i1 %40, label %if.then, label %if.else

if.then:                                          ; preds = %panic.cont
  store i32 %36, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else:                                          ; preds = %panic.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else
  %41 = icmp ne i8 %33, 0
  %42 = xor i1 %41, true
  %43 = and i1 false, %42
  br i1 %43, label %if.then5, label %if.else6

if.then5:                                         ; preds = %if.merge
  %44 = load ptr, ptr %__panic, align 8
  %45 = getelementptr i8, ptr %44, i64 0
  store i8 1, ptr %45, align 1
  %46 = load ptr, ptr %__panic, align 8
  %47 = getelementptr i8, ptr %46, i64 4
  store i32 0, ptr %47, align 4
  br label %if.merge7

if.else6:                                         ; preds = %if.merge
  br label %if.merge7

if.merge7:                                        ; preds = %if.else6, %if.then5
  %if.result = phi i64 [ 0, %if.then5 ], [ 0, %if.else6 ]
  %48 = icmp ne i8 %33, 0
  %49 = zext i1 %48 to i8
  %50 = icmp ne i8 %49, 0
  %51 = or i1 false, %50
  %52 = icmp ne i8 %33, 0
  %53 = icmp ne i8 %33, 0
  br i1 %53, label %if.then8, label %if.else9

if.then8:                                         ; preds = %if.merge7
  br label %if.merge10

if.else9:                                         ; preds = %if.merge7
  br label %if.merge10

if.merge10:                                       ; preds = %if.else9, %if.then8
  %if.result11 = phi i32 [ %36, %if.then8 ], [ 0, %if.else9 ]
  %54 = load ptr, ptr %__panic, align 8
  %55 = load i8, ptr %54, align 1
  %56 = icmp ne i8 %55, 0
  br i1 %56, label %panic.take12, label %panic.cont13

panic.take12:                                     ; preds = %if.merge10
  %57 = load ptr, ptr %__panic, align 8
  %58 = getelementptr i8, ptr %57, i64 4
  %59 = load i32, ptr %58, align 4
  ret i32 %59

panic.cont13:                                     ; preds = %if.merge10
  %60 = load i32, ptr %value, align 4
  ret i32 %60
}

define i32 @EmitLlLibrary_x3a_x3aemitLlAssociatedTypeErasureProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %owner = alloca { i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %owner, ptr align 4 %aggregate.literal, i64 4, i1 false)
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %16 = call i32 @EmitLlLibrary_x3a_x3aEmitLlAssociatedOwner_x3a_x3aassociatedCarrier(ptr noundef nonnull align 4 dereferenceable(4) %owner, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
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

define { i32, [0 x i32] } @EmitLlLibrary_x3a_x3aemitLlOpaqueConcreteReturnProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i32, [0 x i32] }, align 8
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %2 = icmp ne i8 %1, 0
  br i1 %2, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %3 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %3, align 1
  %4 = getelementptr i8, ptr %3, i64 4
  store i32 10, ptr %4, align 4
  ret { i32, [0 x i32] } zeroinitializer

poison.cont:                                      ; preds = %entry
  %5 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 0, ptr %6, align 4
  store { i32, [0 x i32] } zeroinitializer, ptr %0, align 4
  %7 = load i32, ptr %value, align 4
  %8 = getelementptr i8, ptr %0, i64 0
  store i32 %7, ptr %8, align 1
  %9 = load { i32, [0 x i32] }, ptr %0, align 4
  ret { i32, [0 x i32] } %9
}

define i32 @EmitLlLibrary_x3a_x3aemitLlOpaqueConcreteProjectProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %16 = call i32 @EmitLlLibrary_x3a_x3aEmitLlOpaqueOwner_x3a_x3areadOpaqueValue(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %17 = load ptr, ptr %__panic, align 8
  %18 = load i8, ptr %17, align 1
  %19 = icmp ne i8 %18, 0
  br i1 %19, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont2
  %20 = load ptr, ptr %__panic, align 8
  %21 = getelementptr i8, ptr %20, i64 4
  %22 = load i32, ptr %21, align 4
  ret i32 %22

panic.cont:                                       ; preds = %poison.cont2
  ret i32 %16
}

define i32 @EmitLlLibrary_x3a_x3aemitLlPrimitiveTypeProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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

define i32 @EmitLlLibrary_x3a_x3aemitLlAliasTypeProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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

define i32 @EmitLlLibrary_x3a_x3aemitLlRefinementTypeProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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

define { i8, [3 x i8], i32, [0 x i32] } @EmitLlLibrary_x3a_x3aemitLlRecordTypeProbe(ptr noundef nonnull align 1 dereferenceable(1) %flag, ptr noundef nonnull align 4 dereferenceable(4) %count, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i8, [3 x i8], i32, [0 x i32] }, align 8
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %2 = icmp ne i8 %1, 0
  br i1 %2, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %3 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %3, align 1
  %4 = getelementptr i8, ptr %3, i64 4
  store i32 10, ptr %4, align 4
  ret { i8, [3 x i8], i32, [0 x i32] } zeroinitializer

poison.cont:                                      ; preds = %entry
  %5 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 0, ptr %6, align 4
  store { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, ptr %0, align 4
  %7 = load i8, ptr %flag, align 1
  %8 = icmp ne i8 %7, 0
  %9 = zext i1 %8 to i8
  %10 = getelementptr i8, ptr %0, i64 0
  store i8 %9, ptr %10, align 1
  %11 = load i32, ptr %count, align 4
  %12 = getelementptr i8, ptr %0, i64 4
  store i32 %11, ptr %12, align 1
  %13 = load { i8, [3 x i8], i32, [0 x i32] }, ptr %0, align 4
  ret { i8, [3 x i8], i32, [0 x i32] } %13
}

define i64 @EmitLlLibrary_x3a_x3aemitLlPermissionLayoutProbe(ptr noundef nonnull align 4 dereferenceable(8) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

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
  %18 = zext i32 %17 to i64
  ret i64 %18

panic.cont:                                       ; preds = %check_ok
  %19 = call { i8, i1 } @llvm.sadd.with.overflow.i8(i8 8, i8 4)
  %20 = extractvalue { i8, i1 } %19, 0
  %21 = extractvalue { i8, i1 } %19, 1
  %22 = freeze i8 %20
  br i1 %21, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  %23 = zext i8 %22 to i64
  br i1 true, label %check_ok1, label %check_fail2

op_fail:                                          ; preds = %panic.cont
  %24 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %24, align 1
  %25 = getelementptr i8, ptr %24, i64 4
  store i32 4, ptr %25, align 4
  ret i64 0

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
  %34 = zext i32 %33 to i64
  ret i64 %34

panic.cont4:                                      ; preds = %check_ok1
  %35 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %23, i64 8)
  %36 = extractvalue { i64, i1 } %35, 0
  %37 = extractvalue { i64, i1 } %35, 1
  %38 = freeze i64 %36
  br i1 %37, label %op_fail6, label %op_ok5

op_ok5:                                           ; preds = %panic.cont4
  br i1 true, label %check_ok7, label %check_fail8

op_fail6:                                         ; preds = %panic.cont4
  %39 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %39, align 1
  %40 = getelementptr i8, ptr %39, i64 4
  store i32 4, ptr %40, align 4
  ret i64 0

check_ok7:                                        ; preds = %check_fail8, %op_ok5
  %41 = load ptr, ptr %__panic, align 8
  %42 = load i8, ptr %41, align 1
  %43 = icmp ne i8 %42, 0
  br i1 %43, label %panic.take9, label %panic.cont10

check_fail8:                                      ; preds = %op_ok5
  %44 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %44, align 1
  %45 = getelementptr i8, ptr %44, i64 4
  store i32 4, ptr %45, align 4
  br label %check_ok7

panic.take9:                                      ; preds = %check_ok7
  %46 = load ptr, ptr %__panic, align 8
  %47 = getelementptr i8, ptr %46, i64 4
  %48 = load i32, ptr %47, align 4
  %49 = zext i32 %48 to i64
  ret i64 %49

panic.cont10:                                     ; preds = %check_ok7
  %50 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %38, i64 4)
  %51 = extractvalue { i64, i1 } %50, 0
  %52 = extractvalue { i64, i1 } %50, 1
  %53 = freeze i64 %51
  br i1 %52, label %op_fail12, label %op_ok11

op_ok11:                                          ; preds = %panic.cont10
  br i1 true, label %check_ok13, label %check_fail14

op_fail12:                                        ; preds = %panic.cont10
  %54 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %54, align 1
  %55 = getelementptr i8, ptr %54, i64 4
  store i32 4, ptr %55, align 4
  ret i64 0

check_ok13:                                       ; preds = %check_fail14, %op_ok11
  %56 = load ptr, ptr %__panic, align 8
  %57 = load i8, ptr %56, align 1
  %58 = icmp ne i8 %57, 0
  br i1 %58, label %panic.take15, label %panic.cont16

check_fail14:                                     ; preds = %op_ok11
  %59 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %59, align 1
  %60 = getelementptr i8, ptr %59, i64 4
  store i32 4, ptr %60, align 4
  br label %check_ok13

panic.take15:                                     ; preds = %check_ok13
  %61 = load ptr, ptr %__panic, align 8
  %62 = getelementptr i8, ptr %61, i64 4
  %63 = load i32, ptr %62, align 4
  %64 = zext i32 %63 to i64
  ret i64 %64

panic.cont16:                                     ; preds = %check_ok13
  %65 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %53, i64 8)
  %66 = extractvalue { i64, i1 } %65, 0
  %67 = extractvalue { i64, i1 } %65, 1
  %68 = freeze i64 %66
  br i1 %67, label %op_fail18, label %op_ok17

op_ok17:                                          ; preds = %panic.cont16
  br i1 true, label %check_ok19, label %check_fail20

op_fail18:                                        ; preds = %panic.cont16
  %69 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %69, align 1
  %70 = getelementptr i8, ptr %69, i64 4
  store i32 4, ptr %70, align 4
  ret i64 0

check_ok19:                                       ; preds = %check_fail20, %op_ok17
  %71 = load ptr, ptr %__panic, align 8
  %72 = load i8, ptr %71, align 1
  %73 = icmp ne i8 %72, 0
  br i1 %73, label %panic.take21, label %panic.cont22

check_fail20:                                     ; preds = %op_ok17
  %74 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %74, align 1
  %75 = getelementptr i8, ptr %74, i64 4
  store i32 4, ptr %75, align 4
  br label %check_ok19

panic.take21:                                     ; preds = %check_ok19
  %76 = load ptr, ptr %__panic, align 8
  %77 = getelementptr i8, ptr %76, i64 4
  %78 = load i32, ptr %77, align 4
  %79 = zext i32 %78 to i64
  ret i64 %79

panic.cont22:                                     ; preds = %check_ok19
  %80 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %68, i64 4)
  %81 = extractvalue { i64, i1 } %80, 0
  %82 = extractvalue { i64, i1 } %80, 1
  %83 = freeze i64 %81
  br i1 %82, label %op_fail24, label %op_ok23

op_ok23:                                          ; preds = %panic.cont22
  br i1 true, label %check_ok25, label %check_fail26

op_fail24:                                        ; preds = %panic.cont22
  %84 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %84, align 1
  %85 = getelementptr i8, ptr %84, i64 4
  store i32 4, ptr %85, align 4
  ret i64 0

check_ok25:                                       ; preds = %check_fail26, %op_ok23
  %86 = load ptr, ptr %__panic, align 8
  %87 = load i8, ptr %86, align 1
  %88 = icmp ne i8 %87, 0
  br i1 %88, label %panic.take27, label %panic.cont28

check_fail26:                                     ; preds = %op_ok23
  %89 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %89, align 1
  %90 = getelementptr i8, ptr %89, i64 4
  store i32 4, ptr %90, align 4
  br label %check_ok25

panic.take27:                                     ; preds = %check_ok25
  %91 = load ptr, ptr %__panic, align 8
  %92 = getelementptr i8, ptr %91, i64 4
  %93 = load i32, ptr %92, align 4
  %94 = zext i32 %93 to i64
  ret i64 %94

panic.cont28:                                     ; preds = %check_ok25
  %95 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %83, i64 8)
  %96 = extractvalue { i64, i1 } %95, 0
  %97 = extractvalue { i64, i1 } %95, 1
  %98 = freeze i64 %96
  br i1 %97, label %op_fail30, label %op_ok29

op_ok29:                                          ; preds = %panic.cont28
  br i1 true, label %check_ok31, label %check_fail32

op_fail30:                                        ; preds = %panic.cont28
  %99 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %99, align 1
  %100 = getelementptr i8, ptr %99, i64 4
  store i32 4, ptr %100, align 4
  ret i64 0

check_ok31:                                       ; preds = %check_fail32, %op_ok29
  %101 = load ptr, ptr %__panic, align 8
  %102 = load i8, ptr %101, align 1
  %103 = icmp ne i8 %102, 0
  br i1 %103, label %panic.take33, label %panic.cont34

check_fail32:                                     ; preds = %op_ok29
  %104 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %104, align 1
  %105 = getelementptr i8, ptr %104, i64 4
  store i32 4, ptr %105, align 4
  br label %check_ok31

panic.take33:                                     ; preds = %check_ok31
  %106 = load ptr, ptr %__panic, align 8
  %107 = getelementptr i8, ptr %106, i64 4
  %108 = load i32, ptr %107, align 4
  %109 = zext i32 %108 to i64
  ret i64 %109

panic.cont34:                                     ; preds = %check_ok31
  %110 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %98, i64 4)
  %111 = extractvalue { i64, i1 } %110, 0
  %112 = extractvalue { i64, i1 } %110, 1
  %113 = freeze i64 %111
  br i1 %112, label %op_fail36, label %op_ok35

op_ok35:                                          ; preds = %panic.cont34
  ret i64 %113

op_fail36:                                        ; preds = %panic.cont34
  %114 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %114, align 1
  %115 = getelementptr i8, ptr %114, i64 4
  store i32 4, ptr %115, align 4
  ret i64 0
}

define i32 @EmitLlLibrary_x3a_x3aemitLlPermissionAdmissibilityReceiverProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"EmitLlLibrary_x3a_x3aemitLlPermissionAdmissibilityReceiverProbe$tmp$call_ref_tmp_220" = alloca i32, align 4
  %observed3 = alloca i32, align 4
  %observed = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %16 = call i32 @EmitLlLibrary_x3a_x3aEmitLlPermissionAdmissibilityRecord_x3a_x3areadValue(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %17 = load ptr, ptr %__panic, align 8
  %18 = load i8, ptr %17, align 1
  %19 = icmp ne i8 %18, 0
  br i1 %19, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont2
  %20 = load ptr, ptr %__panic, align 8
  %21 = getelementptr i8, ptr %20, i64 4
  %22 = load i32, ptr %21, align 4
  ret i32 %22

panic.cont:                                       ; preds = %poison.cont2
  store i32 %16, ptr %observed3, align 4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %panic.cont
  %23 = load ptr, ptr %__panic, align 8
  %24 = load i8, ptr %23, align 1
  %25 = icmp ne i8 %24, 0
  br i1 %25, label %panic.take4, label %panic.cont5

check_fail:                                       ; preds = %panic.cont
  %26 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %26, align 1
  %27 = getelementptr i8, ptr %26, i64 4
  store i32 4, ptr %27, align 4
  br label %check_ok

panic.take4:                                      ; preds = %check_ok
  %28 = load ptr, ptr %__panic, align 8
  %29 = getelementptr i8, ptr %28, i64 4
  %30 = load i32, ptr %29, align 4
  ret i32 %30

panic.cont5:                                      ; preds = %check_ok
  %31 = load i32, ptr %observed3, align 4
  %32 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %31, i32 1)
  %33 = extractvalue { i32, i1 } %32, 0
  %34 = extractvalue { i32, i1 } %32, 1
  %35 = freeze i32 %33
  br i1 %34, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont5
  store i32 %35, ptr %"EmitLlLibrary_x3a_x3aemitLlPermissionAdmissibilityReceiverProbe$tmp$call_ref_tmp_220", align 4
  %36 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %37 = icmp ne i8 %36, 0
  br i1 %37, label %poison.take6, label %poison.cont7

op_fail:                                          ; preds = %panic.cont5
  %38 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %38, align 1
  %39 = getelementptr i8, ptr %38, i64 4
  store i32 4, ptr %39, align 4
  ret i32 0

poison.take6:                                     ; preds = %op_ok
  %40 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %40, align 1
  %41 = getelementptr i8, ptr %40, i64 4
  store i32 10, ptr %41, align 4
  %42 = load ptr, ptr %__panic, align 8
  %43 = getelementptr i8, ptr %42, i64 4
  %44 = load i32, ptr %43, align 4
  ret i32 %44

poison.cont7:                                     ; preds = %op_ok
  %45 = call i32 @EmitLlLibrary_x3a_x3aEmitLlPermissionAdmissibilityRecord_x3a_x3areplaceValue(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 4 dereferenceable(4) %"EmitLlLibrary_x3a_x3aemitLlPermissionAdmissibilityReceiverProbe$tmp$call_ref_tmp_220", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %46 = load ptr, ptr %__panic, align 8
  %47 = load i8, ptr %46, align 1
  %48 = icmp ne i8 %47, 0
  br i1 %48, label %panic.take8, label %panic.cont9

panic.take8:                                      ; preds = %poison.cont7
  %49 = load ptr, ptr %__panic, align 8
  %50 = getelementptr i8, ptr %49, i64 4
  %51 = load i32, ptr %50, align 4
  ret i32 %51

panic.cont9:                                      ; preds = %poison.cont7
  ret i32 %45
}

define { i32, i8, [3 x i8] } @EmitLlLibrary_x3a_x3aemitLlTupleTypeProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 1 dereferenceable(1) %flag, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i32, i8, [3 x i8] }, align 8
  %1 = alloca { i32, i8, [3 x i8] }, align 8
  %2 = alloca { i32, i8, [3 x i8] }, align 8
  %3 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %9 = load i32, ptr %value, align 4
  %10 = getelementptr i8, ptr %2, i64 0
  store i32 %9, ptr %10, align 1
  %11 = load i8, ptr %flag, align 1
  %12 = icmp ne i8 %11, 0
  %13 = zext i1 %12 to i8
  %14 = getelementptr i8, ptr %2, i64 4
  store i8 %13, ptr %14, align 1
  %15 = load { i32, i8, [3 x i8] }, ptr %2, align 4
  store { i32, i8, [3 x i8] } %15, ptr %0, align 4
  call void @llvm.memset.p0.i64(ptr align 4 %1, i8 0, i64 8, i1 false)
  %16 = load i32, ptr %0, align 1
  store i32 %16, ptr %1, align 1
  %17 = getelementptr i8, ptr %0, i64 4
  %18 = getelementptr i8, ptr %1, i64 4
  %19 = load i8, ptr %17, align 1
  %20 = icmp ne i8 %19, 0
  %21 = zext i1 %20 to i8
  store i8 %21, ptr %18, align 1
  %22 = load { i32, i8, [3 x i8] }, ptr %1, align 4
  ret { i32, i8, [3 x i8] } %22
}

define i64 @EmitLlLibrary_x3a_x3aemitLlTupleValueBitsConsume(ptr noundef nonnull align 8 dereferenceable(16) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i8, [7 x i8], i64 }, align 8
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %8 = zext i32 %7 to i64
  ret i64 %8

poison.cont:                                      ; preds = %entry
  %9 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 0, ptr %10, align 4
  %11 = load { i8, [7 x i8], i64 }, ptr %value, align 4
  store { i8, [7 x i8], i64 } %11, ptr %0, align 4
  %12 = getelementptr i8, ptr %0, i64 8
  %13 = load i64, ptr %12, align 1
  ret i64 %13
}

define i64 @EmitLlLibrary_x3a_x3aemitLlTupleValueBitsProbe(ptr noundef nonnull align 1 dereferenceable(1) %seed, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i8, [7 x i8], i64 }, align 8
  %1 = alloca { i8, [7 x i8], i64 }, align 8
  %2 = alloca { i8, [7 x i8], i64 }, align 8
  %"EmitLlLibrary_x3a_x3aemitLlTupleValueBitsProbe$tmp$call_ref_tmp_231" = alloca { i8, [7 x i8], i64 }, align 8
  %3 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %4 = icmp ne i8 %3, 0
  br i1 %4, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %5 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 10, ptr %6, align 4
  %7 = load ptr, ptr %__panic, align 8
  %8 = getelementptr i8, ptr %7, i64 4
  %9 = load i32, ptr %8, align 4
  %10 = zext i32 %9 to i64
  ret i64 %10

poison.cont:                                      ; preds = %entry
  %11 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %11, align 1
  %12 = getelementptr i8, ptr %11, i64 4
  store i32 0, ptr %12, align 4
  %13 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %14 = icmp ne i8 %13, 0
  br i1 %14, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %15 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %15, align 1
  %16 = getelementptr i8, ptr %15, i64 4
  store i32 10, ptr %16, align 4
  %17 = load ptr, ptr %__panic, align 8
  %18 = getelementptr i8, ptr %17, i64 4
  %19 = load i32, ptr %18, align 4
  %20 = zext i32 %19 to i64
  ret i64 %20

poison.cont2:                                     ; preds = %poison.cont
  %21 = load ptr, ptr %__panic, align 8
  %22 = load i8, ptr %21, align 1
  %23 = icmp ne i8 %22, 0
  br i1 %23, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont2
  %24 = load ptr, ptr %__panic, align 8
  %25 = getelementptr i8, ptr %24, i64 4
  %26 = load i32, ptr %25, align 4
  %27 = zext i32 %26 to i64
  ret i64 %27

panic.cont:                                       ; preds = %poison.cont2
  store { i8, [7 x i8], i64 } zeroinitializer, ptr %2, align 4
  %28 = load i8, ptr %seed, align 1
  %29 = getelementptr i8, ptr %2, i64 0
  store i8 %28, ptr %29, align 1
  %30 = getelementptr i8, ptr %2, i64 8
  store i64 37, ptr %30, align 1
  %31 = load { i8, [7 x i8], i64 }, ptr %2, align 4
  store { i8, [7 x i8], i64 } %31, ptr %0, align 4
  call void @llvm.memset.p0.i64(ptr align 8 %1, i8 0, i64 16, i1 false)
  %32 = load i8, ptr %0, align 1
  store i8 %32, ptr %1, align 1
  %33 = getelementptr i8, ptr %0, i64 8
  %34 = getelementptr i8, ptr %1, i64 8
  %35 = load i64, ptr %33, align 1
  store i64 %35, ptr %34, align 1
  %36 = load { i8, [7 x i8], i64 }, ptr %1, align 4
  store { i8, [7 x i8], i64 } %36, ptr %"EmitLlLibrary_x3a_x3aemitLlTupleValueBitsProbe$tmp$call_ref_tmp_231", align 4
  %37 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %38 = icmp ne i8 %37, 0
  br i1 %38, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %panic.cont
  %39 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %39, align 1
  %40 = getelementptr i8, ptr %39, i64 4
  store i32 10, ptr %40, align 4
  %41 = load ptr, ptr %__panic, align 8
  %42 = getelementptr i8, ptr %41, i64 4
  %43 = load i32, ptr %42, align 4
  %44 = zext i32 %43 to i64
  ret i64 %44

poison.cont4:                                     ; preds = %panic.cont
  %45 = call i64 @EmitLlLibrary_x3a_x3aemitLlTupleValueBitsConsume(ptr noundef nonnull align 8 dereferenceable(16) %"EmitLlLibrary_x3a_x3aemitLlTupleValueBitsProbe$tmp$call_ref_tmp_231", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %46 = load ptr, ptr %__panic, align 8
  %47 = load i8, ptr %46, align 1
  %48 = icmp ne i8 %47, 0
  br i1 %48, label %panic.take5, label %panic.cont6

panic.take5:                                      ; preds = %poison.cont4
  %49 = load ptr, ptr %__panic, align 8
  %50 = getelementptr i8, ptr %49, i64 4
  %51 = load i32, ptr %50, align 4
  %52 = zext i32 %51 to i64
  ret i64 %52

panic.cont6:                                      ; preds = %poison.cont4
  ret i64 %45
}

define i32 @EmitLlLibrary_x3a_x3aemitLlPrimitiveValueBitsConsume(ptr noundef nonnull align 8 dereferenceable(24) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i8, [3 x i8], i32, {}, i32, [4 x i8], double }, align 8
  %1 = alloca { i8, [3 x i8], i32, {}, i32, [4 x i8], double }, align 8
  %2 = alloca { i8, [3 x i8], i32, {}, i32, [4 x i8], double }, align 8
  %3 = alloca { i8, [3 x i8], i32, {}, i32, [4 x i8], double }, align 8
  %4 = alloca { i8, [3 x i8], i32, {}, i32, [4 x i8], double }, align 8
  %5 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %6 = icmp ne i8 %5, 0
  br i1 %6, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %7 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %7, align 1
  %8 = getelementptr i8, ptr %7, i64 4
  store i32 10, ptr %8, align 4
  %9 = load ptr, ptr %__panic, align 8
  %10 = getelementptr i8, ptr %9, i64 4
  %11 = load i32, ptr %10, align 4
  ret i32 %11

poison.cont:                                      ; preds = %entry
  %12 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %12, align 1
  %13 = getelementptr i8, ptr %12, i64 4
  store i32 0, ptr %13, align 4
  %14 = load { i8, [3 x i8], i32, {}, i32, [4 x i8], double }, ptr %value, align 8
  store { i8, [3 x i8], i32, {}, i32, [4 x i8], double } %14, ptr %4, align 8
  %15 = getelementptr i8, ptr %4, i64 0
  %16 = load i8, ptr %15, align 1
  %17 = icmp ne i8 %16, 0
  %18 = load { i8, [3 x i8], i32, {}, i32, [4 x i8], double }, ptr %value, align 8
  store { i8, [3 x i8], i32, {}, i32, [4 x i8], double } %18, ptr %3, align 8
  %19 = getelementptr i8, ptr %3, i64 0
  %20 = load i8, ptr %19, align 1
  %21 = icmp ne i8 %20, 0
  br i1 %21, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  %22 = load { i8, [3 x i8], i32, {}, i32, [4 x i8], double }, ptr %value, align 8
  store { i8, [3 x i8], i32, {}, i32, [4 x i8], double } %22, ptr %2, align 8
  %23 = getelementptr i8, ptr %2, i64 4
  %24 = load i32, ptr %23, align 1
  %25 = icmp eq i32 %24, 81
  %26 = zext i1 %25 to i8
  br label %if.merge

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %"EmitLlLibrary_x3a_x3aemitLlPrimitiveValueBitsConsume$tmp$and_239" = phi i8 [ %26, %if.then ], [ 0, %if.else ]
  %27 = icmp ne i8 %"EmitLlLibrary_x3a_x3aemitLlPrimitiveValueBitsConsume$tmp$and_239", 0
  %28 = icmp ne i8 %"EmitLlLibrary_x3a_x3aemitLlPrimitiveValueBitsConsume$tmp$and_239", 0
  br i1 %28, label %if.then1, label %if.else2

if.then1:                                         ; preds = %if.merge
  %29 = load { i8, [3 x i8], i32, {}, i32, [4 x i8], double }, ptr %value, align 8
  store { i8, [3 x i8], i32, {}, i32, [4 x i8], double } %29, ptr %1, align 8
  %30 = getelementptr i8, ptr %1, i64 16
  %31 = load double, ptr %30, align 1
  %32 = fcmp ogt double %31, 2.000000e+00
  %33 = zext i1 %32 to i8
  br label %if.merge3

if.else2:                                         ; preds = %if.merge
  br label %if.merge3

if.merge3:                                        ; preds = %if.else2, %if.then1
  %"EmitLlLibrary_x3a_x3aemitLlPrimitiveValueBitsConsume$tmp$and_243" = phi i8 [ %33, %if.then1 ], [ 0, %if.else2 ]
  %34 = icmp ne i8 %"EmitLlLibrary_x3a_x3aemitLlPrimitiveValueBitsConsume$tmp$and_243", 0
  %35 = icmp ne i8 %"EmitLlLibrary_x3a_x3aemitLlPrimitiveValueBitsConsume$tmp$and_243", 0
  br i1 %35, label %if.then4, label %if.else5

if.then4:                                         ; preds = %if.merge3
  %36 = load { i8, [3 x i8], i32, {}, i32, [4 x i8], double }, ptr %value, align 8
  store { i8, [3 x i8], i32, {}, i32, [4 x i8], double } %36, ptr %0, align 8
  %37 = getelementptr i8, ptr %0, i64 8
  %38 = load i32, ptr %37, align 1
  ret i32 %38

if.else5:                                         ; preds = %if.merge3
  br label %if.merge6

if.merge6:                                        ; preds = %if.else5
  ret i32 0
}

define i32 @EmitLlLibrary_x3a_x3aemitLlPrimitiveValueBitsProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i8, [3 x i8], i32, {}, i32, [4 x i8], double }, align 8
  %1 = alloca { i8, [3 x i8], i32, {}, i32, [4 x i8], double }, align 8
  %2 = alloca { i8, [3 x i8], i32, {}, i32, [4 x i8], double }, align 8
  %"EmitLlLibrary_x3a_x3aemitLlPrimitiveValueBitsProbe$tmp$call_ref_tmp_265" = alloca { i8, [3 x i8], i32, {}, i32, [4 x i8], double }, align 8
  %coerce_bits = alloca i64, align 8
  %raw_null = alloca ptr, align 8
  %3 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %4 = icmp ne i8 %3, 0
  br i1 %4, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %5 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 10, ptr %6, align 4
  %7 = load ptr, ptr %__panic, align 8
  %8 = getelementptr i8, ptr %7, i64 4
  %9 = load i32, ptr %8, align 4
  ret i32 %9

poison.cont:                                      ; preds = %entry
  %10 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %10, align 1
  %11 = getelementptr i8, ptr %10, i64 4
  store i32 0, ptr %11, align 4
  store i64 0, ptr %coerce_bits, align 8
  %12 = load ptr, ptr %coerce_bits, align 1
  store ptr %12, ptr %raw_null, align 8
  %13 = load ptr, ptr %raw_null, align 8
  %14 = icmp eq ptr %13, null
  %15 = zext i1 %14 to i8
  %16 = icmp ne i8 %15, 0
  %17 = icmp ne i8 %15, 0
  br i1 %17, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  %18 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %19 = icmp ne i8 %18, 0
  br i1 %19, label %poison.take1, label %poison.cont2

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else
  ret i32 0

poison.take1:                                     ; preds = %if.then
  %20 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %20, align 1
  %21 = getelementptr i8, ptr %20, i64 4
  store i32 10, ptr %21, align 4
  %22 = load ptr, ptr %__panic, align 8
  %23 = getelementptr i8, ptr %22, i64 4
  %24 = load i32, ptr %23, align 4
  ret i32 %24

poison.cont2:                                     ; preds = %if.then
  store { i8, [3 x i8], i32, {}, i32, [4 x i8], double } zeroinitializer, ptr %2, align 8
  %25 = getelementptr i8, ptr %2, i64 0
  store i8 1, ptr %25, align 1
  %26 = getelementptr i8, ptr %2, i64 4
  store i32 81, ptr %26, align 1
  %27 = getelementptr i8, ptr %2, i64 8
  store {} zeroinitializer, ptr %27, align 1
  %28 = getelementptr i8, ptr %2, i64 8
  store i32 19, ptr %28, align 1
  %29 = getelementptr i8, ptr %2, i64 16
  store double 2.500000e+00, ptr %29, align 1
  %30 = load { i8, [3 x i8], i32, {}, i32, [4 x i8], double }, ptr %2, align 8
  store { i8, [3 x i8], i32, {}, i32, [4 x i8], double } %30, ptr %0, align 8
  call void @llvm.memset.p0.i64(ptr align 8 %1, i8 0, i64 24, i1 false)
  %31 = load i8, ptr %0, align 1
  %32 = icmp ne i8 %31, 0
  %33 = zext i1 %32 to i8
  store i8 %33, ptr %1, align 1
  %34 = getelementptr i8, ptr %0, i64 4
  %35 = getelementptr i8, ptr %1, i64 4
  %36 = load i32, ptr %34, align 1
  store i32 %36, ptr %35, align 1
  %37 = getelementptr i8, ptr %0, i64 8
  %38 = getelementptr i8, ptr %1, i64 8
  %39 = load {}, ptr %37, align 1
  store {} %39, ptr %38, align 1
  %40 = getelementptr i8, ptr %0, i64 8
  %41 = getelementptr i8, ptr %1, i64 8
  %42 = load i32, ptr %40, align 1
  store i32 %42, ptr %41, align 1
  %43 = getelementptr i8, ptr %0, i64 16
  %44 = getelementptr i8, ptr %1, i64 16
  %45 = load double, ptr %43, align 1
  store double %45, ptr %44, align 1
  %46 = load { i8, [3 x i8], i32, {}, i32, [4 x i8], double }, ptr %1, align 8
  store { i8, [3 x i8], i32, {}, i32, [4 x i8], double } %46, ptr %"EmitLlLibrary_x3a_x3aemitLlPrimitiveValueBitsProbe$tmp$call_ref_tmp_265", align 8
  %47 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  ret i32 %53

poison.cont4:                                     ; preds = %poison.cont2
  %54 = call i32 @EmitLlLibrary_x3a_x3aemitLlPrimitiveValueBitsConsume(ptr noundef nonnull align 8 dereferenceable(24) %"EmitLlLibrary_x3a_x3aemitLlPrimitiveValueBitsProbe$tmp$call_ref_tmp_265", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %55 = load ptr, ptr %__panic, align 8
  %56 = load i8, ptr %55, align 1
  %57 = icmp ne i8 %56, 0
  br i1 %57, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  %58 = load ptr, ptr %__panic, align 8
  %59 = getelementptr i8, ptr %58, i64 4
  %60 = load i32, ptr %59, align 4
  ret i32 %60

panic.cont:                                       ; preds = %poison.cont4
  ret i32 %54
}

define void @EmitLlLibrary_x3a_x3aemitLlNeverReturnTypeProbe(ptr noundef nonnull align 8 dereferenceable(16) %context, ptr noundef nonnull align 4 dereferenceable(4) %code, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 0, ptr %5, align 4
  %6 = getelementptr i8, ptr %context, i64 0
  %7 = getelementptr i8, ptr %context, i64 0
  call void @ultraviolet_x3a_x3aruntime_x3a_x3asystem_x3a_x3aexit(ptr noundef nonnull align 8 dereferenceable(16) %7, ptr noundef nonnull align 4 dereferenceable(4) %code)
  unreachable
}

define i32 @EmitLlLibrary_x3a_x3aemitLlPermissionValueBitsConsume(ptr noundef nonnull align 4 dereferenceable(8) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i32, i8, [3 x i8] }, align 8
  %1 = alloca { i32, i8, [3 x i8] }, align 8
  %2 = alloca { i32, i8, [3 x i8] }, align 8
  %3 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %4 = icmp ne i8 %3, 0
  br i1 %4, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %5 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 10, ptr %6, align 4
  %7 = load ptr, ptr %__panic, align 8
  %8 = getelementptr i8, ptr %7, i64 4
  %9 = load i32, ptr %8, align 4
  ret i32 %9

poison.cont:                                      ; preds = %entry
  %10 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %10, align 1
  %11 = getelementptr i8, ptr %10, i64 4
  store i32 0, ptr %11, align 4
  %12 = load { i32, i8, [3 x i8] }, ptr %value, align 4
  store { i32, i8, [3 x i8] } %12, ptr %2, align 4
  %13 = getelementptr i8, ptr %2, i64 4
  %14 = load i8, ptr %13, align 1
  %15 = icmp ne i8 %14, 0
  %16 = load { i32, i8, [3 x i8] }, ptr %value, align 4
  store { i32, i8, [3 x i8] } %16, ptr %1, align 4
  %17 = getelementptr i8, ptr %1, i64 4
  %18 = load i8, ptr %17, align 1
  %19 = icmp ne i8 %18, 0
  br i1 %19, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  %20 = load { i32, i8, [3 x i8] }, ptr %value, align 4
  store { i32, i8, [3 x i8] } %20, ptr %0, align 4
  %21 = getelementptr i8, ptr %0, i64 0
  %22 = load i32, ptr %21, align 1
  ret i32 %22

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else
  ret i32 0
}

define i32 @EmitLlLibrary_x3a_x3aemitLlPermissionValueBitsProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i32, i8, [3 x i8] }, align 8
  %1 = alloca { i32, i8, [3 x i8] }, align 8
  %2 = alloca { i32, i8, [3 x i8] }, align 8
  %"EmitLlLibrary_x3a_x3aemitLlPermissionValueBitsProbe$tmp$call_ref_tmp_288" = alloca { i32, i8, [3 x i8] }, align 4
  %3 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %4 = icmp ne i8 %3, 0
  br i1 %4, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %5 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 10, ptr %6, align 4
  %7 = load ptr, ptr %__panic, align 8
  %8 = getelementptr i8, ptr %7, i64 4
  %9 = load i32, ptr %8, align 4
  ret i32 %9

poison.cont:                                      ; preds = %entry
  %10 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %10, align 1
  %11 = getelementptr i8, ptr %10, i64 4
  store i32 0, ptr %11, align 4
  %12 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  ret i32 %18

poison.cont2:                                     ; preds = %poison.cont
  store { i32, i8, [3 x i8] } zeroinitializer, ptr %2, align 4
  %19 = getelementptr i8, ptr %2, i64 0
  store i32 23, ptr %19, align 1
  %20 = getelementptr i8, ptr %2, i64 4
  store i8 1, ptr %20, align 1
  %21 = load { i32, i8, [3 x i8] }, ptr %2, align 4
  store { i32, i8, [3 x i8] } %21, ptr %0, align 4
  call void @llvm.memset.p0.i64(ptr align 4 %1, i8 0, i64 8, i1 false)
  %22 = load i32, ptr %0, align 1
  store i32 %22, ptr %1, align 1
  %23 = getelementptr i8, ptr %0, i64 4
  %24 = getelementptr i8, ptr %1, i64 4
  %25 = load i8, ptr %23, align 1
  %26 = icmp ne i8 %25, 0
  %27 = zext i1 %26 to i8
  store i8 %27, ptr %24, align 1
  %28 = load { i32, i8, [3 x i8] }, ptr %1, align 4
  store { i32, i8, [3 x i8] } %28, ptr %"EmitLlLibrary_x3a_x3aemitLlPermissionValueBitsProbe$tmp$call_ref_tmp_288", align 4
  %29 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %30 = icmp ne i8 %29, 0
  br i1 %30, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %31 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %31, align 1
  %32 = getelementptr i8, ptr %31, i64 4
  store i32 10, ptr %32, align 4
  %33 = load ptr, ptr %__panic, align 8
  %34 = getelementptr i8, ptr %33, i64 4
  %35 = load i32, ptr %34, align 4
  ret i32 %35

poison.cont4:                                     ; preds = %poison.cont2
  %36 = call i32 @EmitLlLibrary_x3a_x3aemitLlPermissionValueBitsConsume(ptr noundef nonnull align 4 dereferenceable(8) %"EmitLlLibrary_x3a_x3aemitLlPermissionValueBitsProbe$tmp$call_ref_tmp_288", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %37 = load ptr, ptr %__panic, align 8
  %38 = load i8, ptr %37, align 1
  %39 = icmp ne i8 %38, 0
  br i1 %39, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  %40 = load ptr, ptr %__panic, align 8
  %41 = getelementptr i8, ptr %40, i64 4
  %42 = load i32, ptr %41, align 4
  ret i32 %42

panic.cont:                                       ; preds = %poison.cont4
  ret i32 %36
}

define i64 @EmitLlLibrary_x3a_x3aemitLlEnumValueBitsConsume(ptr noundef nonnull align 8 dereferenceable(24) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsConsume$tmp$if_case_clause_result_316" = alloca i64, align 8
  %payload_value16 = alloca i64, align 8
  %is_ready = alloca i8, align 1
  %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsConsume$tmp$if_case_clause_result_304" = alloca i64, align 8
  %payload_value = alloca i64, align 8
  %seed_value = alloca i8, align 1
  %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsConsume$tmp$if_case_clause_result_294" = alloca i64, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  %10 = load i8, ptr %value, align 1
  %11 = icmp eq i8 %10, 0
  br i1 %11, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %if.merge, %op_ok13, %panic.cont
  %ifcase.result = phi i64 [ %28, %panic.cont ], [ %93, %op_ok13 ], [ %117, %if.merge ]
  ret i64 %ifcase.result

ifcase.case:                                      ; preds = %poison.cont
  %12 = load ptr, ptr %__panic, align 8
  %13 = load i8, ptr %12, align 1
  %14 = icmp ne i8 %13, 0
  br i1 %14, label %panic.take, label %panic.cont

ifcase.next:                                      ; preds = %poison.cont
  %15 = load i8, ptr %value, align 1
  %16 = icmp eq i8 %15, 1
  %17 = getelementptr i8, ptr %value, i64 8
  %18 = getelementptr i8, ptr %17, i64 0
  %19 = load i8, ptr %18, align 1
  %20 = getelementptr i8, ptr %value, i64 8
  %21 = getelementptr i8, ptr %20, i64 8
  %22 = load i64, ptr %21, align 1
  %23 = and i1 %16, true
  br i1 %23, label %ifcase.case1, label %ifcase.next2

panic.take:                                       ; preds = %ifcase.case
  %24 = load ptr, ptr %__panic, align 8
  %25 = getelementptr i8, ptr %24, i64 4
  %26 = load i32, ptr %25, align 4
  %27 = zext i32 %26 to i64
  ret i64 %27

panic.cont:                                       ; preds = %ifcase.case
  store i64 0, ptr %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsConsume$tmp$if_case_clause_result_294", align 4
  %28 = load i64, ptr %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsConsume$tmp$if_case_clause_result_294", align 4
  br label %ifcase.merge

ifcase.case1:                                     ; preds = %ifcase.next
  %29 = getelementptr i8, ptr %value, i64 8
  %30 = getelementptr i8, ptr %29, i64 0
  %31 = load i8, ptr %30, align 1
  store i8 %31, ptr %seed_value, align 1
  %32 = getelementptr i8, ptr %value, i64 8
  %33 = getelementptr i8, ptr %32, i64 8
  %34 = load i64, ptr %33, align 1
  store i64 %34, ptr %payload_value, align 4
  %35 = load i8, ptr %seed_value, align 1
  %36 = load ptr, ptr %__panic, align 8
  %37 = load i8, ptr %36, align 1
  %38 = icmp ne i8 %37, 0
  br i1 %38, label %panic.take3, label %panic.cont4

ifcase.next2:                                     ; preds = %ifcase.next
  %39 = load i8, ptr %value, align 1
  %40 = icmp eq i8 %39, 2
  %41 = getelementptr i8, ptr %value, i64 8
  %42 = getelementptr i8, ptr %41, i64 0
  %43 = load i8, ptr %42, align 1
  %44 = getelementptr i8, ptr %value, i64 8
  %45 = getelementptr i8, ptr %44, i64 8
  %46 = load i64, ptr %45, align 1
  %47 = and i1 %40, true
  br i1 %47, label %ifcase.case15, label %ifcase.unmatched

panic.take3:                                      ; preds = %ifcase.case1
  %48 = load ptr, ptr %__panic, align 8
  %49 = getelementptr i8, ptr %48, i64 4
  %50 = load i32, ptr %49, align 4
  %51 = zext i32 %50 to i64
  ret i64 %51

panic.cont4:                                      ; preds = %ifcase.case1
  %52 = load i8, ptr %seed_value, align 1
  %53 = sext i8 %52 to i64
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %panic.cont4
  %54 = load ptr, ptr %__panic, align 8
  %55 = load i8, ptr %54, align 1
  %56 = icmp ne i8 %55, 0
  br i1 %56, label %panic.take5, label %panic.cont6

check_fail:                                       ; preds = %panic.cont4
  %57 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %57, align 1
  %58 = getelementptr i8, ptr %57, i64 4
  store i32 4, ptr %58, align 4
  br label %check_ok

panic.take5:                                      ; preds = %check_ok
  %59 = load ptr, ptr %__panic, align 8
  %60 = getelementptr i8, ptr %59, i64 4
  %61 = load i32, ptr %60, align 4
  %62 = zext i32 %61 to i64
  ret i64 %62

panic.cont6:                                      ; preds = %check_ok
  %63 = load i64, ptr %payload_value, align 4
  %64 = call { i64, i1 } @llvm.sadd.with.overflow.i64(i64 %63, i64 %53)
  %65 = extractvalue { i64, i1 } %64, 0
  %66 = extractvalue { i64, i1 } %64, 1
  %67 = freeze i64 %65
  br i1 %66, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont6
  %68 = load i8, ptr %seed_value, align 1
  %69 = load ptr, ptr %__panic, align 8
  %70 = load i8, ptr %69, align 1
  %71 = icmp ne i8 %70, 0
  br i1 %71, label %panic.take7, label %panic.cont8

op_fail:                                          ; preds = %panic.cont6
  %72 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %72, align 1
  %73 = getelementptr i8, ptr %72, i64 4
  store i32 4, ptr %73, align 4
  ret i64 0

panic.take7:                                      ; preds = %op_ok
  %74 = load ptr, ptr %__panic, align 8
  %75 = getelementptr i8, ptr %74, i64 4
  %76 = load i32, ptr %75, align 4
  %77 = zext i32 %76 to i64
  ret i64 %77

panic.cont8:                                      ; preds = %op_ok
  %78 = load i8, ptr %seed_value, align 1
  %79 = sext i8 %78 to i64
  br i1 true, label %check_ok9, label %check_fail10

check_ok9:                                        ; preds = %check_fail10, %panic.cont8
  %80 = load ptr, ptr %__panic, align 8
  %81 = load i8, ptr %80, align 1
  %82 = icmp ne i8 %81, 0
  br i1 %82, label %panic.take11, label %panic.cont12

check_fail10:                                     ; preds = %panic.cont8
  %83 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %83, align 1
  %84 = getelementptr i8, ptr %83, i64 4
  store i32 4, ptr %84, align 4
  br label %check_ok9

panic.take11:                                     ; preds = %check_ok9
  %85 = load ptr, ptr %__panic, align 8
  %86 = getelementptr i8, ptr %85, i64 4
  %87 = load i32, ptr %86, align 4
  %88 = zext i32 %87 to i64
  ret i64 %88

panic.cont12:                                     ; preds = %check_ok9
  %89 = call { i64, i1 } @llvm.ssub.with.overflow.i64(i64 %67, i64 %79)
  %90 = extractvalue { i64, i1 } %89, 0
  %91 = extractvalue { i64, i1 } %89, 1
  %92 = freeze i64 %90
  br i1 %91, label %op_fail14, label %op_ok13

op_ok13:                                          ; preds = %panic.cont12
  store i64 %92, ptr %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsConsume$tmp$if_case_clause_result_304", align 4
  %93 = load i64, ptr %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsConsume$tmp$if_case_clause_result_304", align 4
  br label %ifcase.merge

op_fail14:                                        ; preds = %panic.cont12
  %94 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %94, align 1
  %95 = getelementptr i8, ptr %94, i64 4
  store i32 4, ptr %95, align 4
  ret i64 0

ifcase.case15:                                    ; preds = %ifcase.next2
  %96 = getelementptr i8, ptr %value, i64 8
  %97 = getelementptr i8, ptr %96, i64 0
  %98 = load i8, ptr %97, align 1
  %99 = icmp ne i8 %98, 0
  %100 = zext i1 %99 to i8
  store i8 %100, ptr %is_ready, align 1
  %101 = getelementptr i8, ptr %value, i64 8
  %102 = getelementptr i8, ptr %101, i64 8
  %103 = load i64, ptr %102, align 1
  store i64 %103, ptr %payload_value16, align 4
  %104 = load i8, ptr %is_ready, align 1
  %105 = icmp ne i8 %104, 0
  %106 = load i8, ptr %is_ready, align 1
  %107 = icmp ne i8 %106, 0
  br i1 %107, label %if.then, label %if.else

ifcase.unmatched:                                 ; preds = %ifcase.next2
  %108 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %108, align 1
  %109 = getelementptr i8, ptr %108, i64 4
  store i32 18, ptr %109, align 4
  %110 = load ptr, ptr %__panic, align 8
  %111 = getelementptr i8, ptr %110, i64 4
  %112 = load i32, ptr %111, align 4
  %113 = zext i32 %112 to i64
  ret i64 %113

if.then:                                          ; preds = %ifcase.case15
  %114 = load i64, ptr %payload_value16, align 4
  %115 = load i64, ptr %payload_value16, align 4
  %116 = load i64, ptr %payload_value16, align 4
  br label %if.merge

if.else:                                          ; preds = %ifcase.case15
  br i1 true, label %check_ok17, label %check_fail18

if.merge:                                         ; preds = %op_ok21, %if.then
  %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsConsume$tmp$if_315" = phi i64 [ %116, %if.then ], [ %131, %op_ok21 ]
  store i64 %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsConsume$tmp$if_315", ptr %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsConsume$tmp$if_case_clause_result_316", align 4
  %117 = load i64, ptr %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsConsume$tmp$if_case_clause_result_316", align 4
  br label %ifcase.merge

check_ok17:                                       ; preds = %check_fail18, %if.else
  %118 = load ptr, ptr %__panic, align 8
  %119 = load i8, ptr %118, align 1
  %120 = icmp ne i8 %119, 0
  br i1 %120, label %panic.take19, label %panic.cont20

check_fail18:                                     ; preds = %if.else
  %121 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %121, align 1
  %122 = getelementptr i8, ptr %121, i64 4
  store i32 4, ptr %122, align 4
  br label %check_ok17

panic.take19:                                     ; preds = %check_ok17
  %123 = load ptr, ptr %__panic, align 8
  %124 = getelementptr i8, ptr %123, i64 4
  %125 = load i32, ptr %124, align 4
  %126 = zext i32 %125 to i64
  ret i64 %126

panic.cont20:                                     ; preds = %check_ok17
  %127 = load i64, ptr %payload_value16, align 4
  %128 = call { i64, i1 } @llvm.ssub.with.overflow.i64(i64 %127, i64 1)
  %129 = extractvalue { i64, i1 } %128, 0
  %130 = extractvalue { i64, i1 } %128, 1
  %131 = freeze i64 %129
  br i1 %130, label %op_fail22, label %op_ok21

op_ok21:                                          ; preds = %panic.cont20
  br label %if.merge

op_fail22:                                        ; preds = %panic.cont20
  %132 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %132, align 1
  %133 = getelementptr i8, ptr %132, i64 4
  store i32 4, ptr %133, align 4
  ret i64 0
}

define i64 @EmitLlLibrary_x3a_x3aemitLlEnumValueBitsUnitProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %aggregate.literal = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsUnitProbe$tmp$call_ref_tmp_320" = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  %10 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %17 = zext i32 %16 to i64
  ret i64 %17

poison.cont2:                                     ; preds = %poison.cont
  call void @llvm.memset.p0.i64(ptr align 8 %aggregate.literal, i8 0, i64 24, i1 false)
  store i8 0, ptr %aggregate.literal, align 1
  %18 = getelementptr i8, ptr %aggregate.literal, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsUnitProbe$tmp$call_ref_tmp_320", ptr align 8 %aggregate.literal, i64 24, i1 false)
  %19 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %20 = icmp ne i8 %19, 0
  br i1 %20, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %21 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %21, align 1
  %22 = getelementptr i8, ptr %21, i64 4
  store i32 10, ptr %22, align 4
  %23 = load ptr, ptr %__panic, align 8
  %24 = getelementptr i8, ptr %23, i64 4
  %25 = load i32, ptr %24, align 4
  %26 = zext i32 %25 to i64
  ret i64 %26

poison.cont4:                                     ; preds = %poison.cont2
  %27 = call i64 @EmitLlLibrary_x3a_x3aemitLlEnumValueBitsConsume(ptr noundef nonnull align 8 dereferenceable(24) %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsUnitProbe$tmp$call_ref_tmp_320", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %28 = load ptr, ptr %__panic, align 8
  %29 = load i8, ptr %28, align 1
  %30 = icmp ne i8 %29, 0
  br i1 %30, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  %31 = load ptr, ptr %__panic, align 8
  %32 = getelementptr i8, ptr %31, i64 4
  %33 = load i32, ptr %32, align 4
  %34 = zext i32 %33 to i64
  ret i64 %34

panic.cont:                                       ; preds = %poison.cont4
  ret i64 %27
}

define i64 @EmitLlLibrary_x3a_x3aemitLlEnumValueBitsTupleProbe(ptr noundef nonnull align 1 dereferenceable(1) %seed, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %aggregate.literal = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsTupleProbe$tmp$call_ref_tmp_327" = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  %10 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %17 = zext i32 %16 to i64
  ret i64 %17

poison.cont2:                                     ; preds = %poison.cont
  %18 = load ptr, ptr %__panic, align 8
  %19 = load i8, ptr %18, align 1
  %20 = icmp ne i8 %19, 0
  br i1 %20, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont2
  %21 = load ptr, ptr %__panic, align 8
  %22 = getelementptr i8, ptr %21, i64 4
  %23 = load i32, ptr %22, align 4
  %24 = zext i32 %23 to i64
  ret i64 %24

panic.cont:                                       ; preds = %poison.cont2
  call void @llvm.memset.p0.i64(ptr align 8 %aggregate.literal, i8 0, i64 24, i1 false)
  store i8 1, ptr %aggregate.literal, align 1
  %25 = getelementptr i8, ptr %aggregate.literal, i64 8
  %26 = getelementptr i8, ptr %25, i64 0
  %27 = load i8, ptr %seed, align 1
  store i8 %27, ptr %26, align 1
  %28 = getelementptr i8, ptr %25, i64 8
  store i64 41, ptr %28, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsTupleProbe$tmp$call_ref_tmp_327", ptr align 8 %aggregate.literal, i64 24, i1 false)
  %29 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %30 = icmp ne i8 %29, 0
  br i1 %30, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %panic.cont
  %31 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %31, align 1
  %32 = getelementptr i8, ptr %31, i64 4
  store i32 10, ptr %32, align 4
  %33 = load ptr, ptr %__panic, align 8
  %34 = getelementptr i8, ptr %33, i64 4
  %35 = load i32, ptr %34, align 4
  %36 = zext i32 %35 to i64
  ret i64 %36

poison.cont4:                                     ; preds = %panic.cont
  %37 = call i64 @EmitLlLibrary_x3a_x3aemitLlEnumValueBitsConsume(ptr noundef nonnull align 8 dereferenceable(24) %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsTupleProbe$tmp$call_ref_tmp_327", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %38 = load ptr, ptr %__panic, align 8
  %39 = load i8, ptr %38, align 1
  %40 = icmp ne i8 %39, 0
  br i1 %40, label %panic.take5, label %panic.cont6

panic.take5:                                      ; preds = %poison.cont4
  %41 = load ptr, ptr %__panic, align 8
  %42 = getelementptr i8, ptr %41, i64 4
  %43 = load i32, ptr %42, align 4
  %44 = zext i32 %43 to i64
  ret i64 %44

panic.cont6:                                      ; preds = %poison.cont4
  ret i64 %37
}

define i64 @EmitLlLibrary_x3a_x3aemitLlEnumValueBitsRecordProbe(ptr noundef nonnull align 1 dereferenceable(1) %flag, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %aggregate.literal = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsRecordProbe$tmp$call_ref_tmp_334" = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  %10 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %17 = zext i32 %16 to i64
  ret i64 %17

poison.cont2:                                     ; preds = %poison.cont
  %18 = load ptr, ptr %__panic, align 8
  %19 = load i8, ptr %18, align 1
  %20 = icmp ne i8 %19, 0
  br i1 %20, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont2
  %21 = load ptr, ptr %__panic, align 8
  %22 = getelementptr i8, ptr %21, i64 4
  %23 = load i32, ptr %22, align 4
  %24 = zext i32 %23 to i64
  ret i64 %24

panic.cont:                                       ; preds = %poison.cont2
  call void @llvm.memset.p0.i64(ptr align 8 %aggregate.literal, i8 0, i64 24, i1 false)
  store i8 2, ptr %aggregate.literal, align 1
  %25 = getelementptr i8, ptr %aggregate.literal, i64 8
  %26 = getelementptr i8, ptr %25, i64 0
  %27 = load i8, ptr %flag, align 1
  store i8 %27, ptr %26, align 1
  %28 = getelementptr i8, ptr %25, i64 8
  store i64 43, ptr %28, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsRecordProbe$tmp$call_ref_tmp_334", ptr align 8 %aggregate.literal, i64 24, i1 false)
  %29 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %30 = icmp ne i8 %29, 0
  br i1 %30, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %panic.cont
  %31 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %31, align 1
  %32 = getelementptr i8, ptr %31, i64 4
  store i32 10, ptr %32, align 4
  %33 = load ptr, ptr %__panic, align 8
  %34 = getelementptr i8, ptr %33, i64 4
  %35 = load i32, ptr %34, align 4
  %36 = zext i32 %35 to i64
  ret i64 %36

poison.cont4:                                     ; preds = %panic.cont
  %37 = call i64 @EmitLlLibrary_x3a_x3aemitLlEnumValueBitsConsume(ptr noundef nonnull align 8 dereferenceable(24) %"EmitLlLibrary_x3a_x3aemitLlEnumValueBitsRecordProbe$tmp$call_ref_tmp_334", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %38 = load ptr, ptr %__panic, align 8
  %39 = load i8, ptr %38, align 1
  %40 = icmp ne i8 %39, 0
  br i1 %40, label %panic.take5, label %panic.cont6

panic.take5:                                      ; preds = %poison.cont4
  %41 = load ptr, ptr %__panic, align 8
  %42 = getelementptr i8, ptr %41, i64 4
  %43 = load i32, ptr %42, align 4
  %44 = zext i32 %43 to i64
  ret i64 %44

panic.cont6:                                      ; preds = %poison.cont4
  ret i64 %37
}

define [4 x i32] @EmitLlLibrary_x3a_x3aemitLlArrayTypeProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret [4 x i32] zeroinitializer

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 0, ptr %5, align 4
  ret [4 x i32] [i32 1, i32 2, i32 3, i32 4]
}

define i64 @EmitLlLibrary_x3a_x3aemitLlSliceTypeProbe(ptr noundef nonnull align 8 dereferenceable(16) %values, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  ret i64 16
}

define { i64, i64 } @EmitLlLibrary_x3a_x3aemitLlRangeTypeProbe(ptr noundef nonnull align 8 dereferenceable(16) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %6 = load { i64, i64 }, ptr %value, align 4
  ret { i64, i64 } %6
}

define { i64, i64 } @EmitLlLibrary_x3a_x3aemitLlRangeInclusiveTypeProbe(ptr noundef nonnull align 8 dereferenceable(16) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %6 = load { i64, i64 }, ptr %value, align 4
  ret { i64, i64 } %6
}

define { i64 } @EmitLlLibrary_x3a_x3aemitLlRangeFromTypeProbe(ptr noundef nonnull align 8 dereferenceable(8) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret { i64 } zeroinitializer

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 0, ptr %5, align 4
  %6 = load { i64 }, ptr %value, align 4
  ret { i64 } %6
}

define { i64 } @EmitLlLibrary_x3a_x3aemitLlRangeToTypeProbe(ptr noundef nonnull align 8 dereferenceable(8) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret { i64 } zeroinitializer

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 0, ptr %5, align 4
  %6 = load { i64 }, ptr %value, align 4
  ret { i64 } %6
}

define { i64 } @EmitLlLibrary_x3a_x3aemitLlRangeToInclusiveTypeProbe(ptr noundef nonnull align 8 dereferenceable(8) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret { i64 } zeroinitializer

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 0, ptr %5, align 4
  %6 = load { i64 }, ptr %value, align 4
  ret { i64 } %6
}

define void @EmitLlLibrary_x3a_x3aemitLlRangeFullTypeProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %value = alloca {}, align 8
  store {} zeroinitializer, ptr %value, align 1
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 0, ptr %5, align 4
  %6 = load {}, ptr %value, align 1
  ret void
}

define i64 @EmitLlLibrary_x3a_x3aemitLlRangeLowerOptProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %inclusive = alloca { i64, i64 }, align 8
  %exclusive = alloca { i64, i64 }, align 8
  %to_inclusive_end = alloca { i64 }, align 8
  %to_end = alloca { i64 }, align 8
  %from_start = alloca { i64 }, align 8
  %full = alloca {}, align 1
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  store {} zeroinitializer, ptr %full, align 1
  store { i64 } { i64 1 }, ptr %from_start, align 4
  store { i64 } { i64 3 }, ptr %to_end, align 4
  store { i64 } { i64 4 }, ptr %to_inclusive_end, align 4
  store { i64, i64 } { i64 1, i64 5 }, ptr %exclusive, align 4
  store { i64, i64 } { i64 2, i64 6 }, ptr %inclusive, align 4
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
  %24 = zext i32 %23 to i64
  ret i64 %24

panic.cont:                                       ; preds = %check_ok
  %25 = call { i8, i1 } @llvm.sadd.with.overflow.i8(i8 0, i8 8)
  %26 = extractvalue { i8, i1 } %25, 0
  %27 = extractvalue { i8, i1 } %25, 1
  %28 = freeze i8 %26
  br i1 %27, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  %29 = zext i8 %28 to i64
  br i1 true, label %check_ok1, label %check_fail2

op_fail:                                          ; preds = %panic.cont
  %30 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %30, align 1
  %31 = getelementptr i8, ptr %30, i64 4
  store i32 4, ptr %31, align 4
  ret i64 0

check_ok1:                                        ; preds = %check_fail2, %op_ok
  %32 = load ptr, ptr %__panic, align 8
  %33 = load i8, ptr %32, align 1
  %34 = icmp ne i8 %33, 0
  br i1 %34, label %panic.take3, label %panic.cont4

check_fail2:                                      ; preds = %op_ok
  %35 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %35, align 1
  %36 = getelementptr i8, ptr %35, i64 4
  store i32 4, ptr %36, align 4
  br label %check_ok1

panic.take3:                                      ; preds = %check_ok1
  %37 = load ptr, ptr %__panic, align 8
  %38 = getelementptr i8, ptr %37, i64 0
  %39 = load i8, ptr %38, align 1
  %40 = load ptr, ptr %__panic, align 8
  %41 = getelementptr i8, ptr %40, i64 4
  %42 = load i32, ptr %41, align 4
  %43 = load ptr, ptr %__panic, align 8
  %44 = getelementptr i8, ptr %43, i64 4
  %45 = load i32, ptr %44, align 4
  %46 = zext i32 %45 to i64
  ret i64 %46

panic.cont4:                                      ; preds = %check_ok1
  %47 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %29, i64 8)
  %48 = extractvalue { i64, i1 } %47, 0
  %49 = extractvalue { i64, i1 } %47, 1
  %50 = freeze i64 %48
  br i1 %49, label %op_fail6, label %op_ok5

op_ok5:                                           ; preds = %panic.cont4
  br i1 true, label %check_ok7, label %check_fail8

op_fail6:                                         ; preds = %panic.cont4
  %51 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %51, align 1
  %52 = getelementptr i8, ptr %51, i64 4
  store i32 4, ptr %52, align 4
  ret i64 0

check_ok7:                                        ; preds = %check_fail8, %op_ok5
  %53 = load ptr, ptr %__panic, align 8
  %54 = load i8, ptr %53, align 1
  %55 = icmp ne i8 %54, 0
  br i1 %55, label %panic.take9, label %panic.cont10

check_fail8:                                      ; preds = %op_ok5
  %56 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %56, align 1
  %57 = getelementptr i8, ptr %56, i64 4
  store i32 4, ptr %57, align 4
  br label %check_ok7

panic.take9:                                      ; preds = %check_ok7
  %58 = load ptr, ptr %__panic, align 8
  %59 = getelementptr i8, ptr %58, i64 0
  %60 = load i8, ptr %59, align 1
  %61 = load ptr, ptr %__panic, align 8
  %62 = getelementptr i8, ptr %61, i64 4
  %63 = load i32, ptr %62, align 4
  %64 = load ptr, ptr %__panic, align 8
  %65 = getelementptr i8, ptr %64, i64 4
  %66 = load i32, ptr %65, align 4
  %67 = zext i32 %66 to i64
  ret i64 %67

panic.cont10:                                     ; preds = %check_ok7
  %68 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %50, i64 8)
  %69 = extractvalue { i64, i1 } %68, 0
  %70 = extractvalue { i64, i1 } %68, 1
  %71 = freeze i64 %69
  br i1 %70, label %op_fail12, label %op_ok11

op_ok11:                                          ; preds = %panic.cont10
  br i1 true, label %check_ok13, label %check_fail14

op_fail12:                                        ; preds = %panic.cont10
  %72 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %72, align 1
  %73 = getelementptr i8, ptr %72, i64 4
  store i32 4, ptr %73, align 4
  ret i64 0

check_ok13:                                       ; preds = %check_fail14, %op_ok11
  %74 = load ptr, ptr %__panic, align 8
  %75 = load i8, ptr %74, align 1
  %76 = icmp ne i8 %75, 0
  br i1 %76, label %panic.take15, label %panic.cont16

check_fail14:                                     ; preds = %op_ok11
  %77 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %77, align 1
  %78 = getelementptr i8, ptr %77, i64 4
  store i32 4, ptr %78, align 4
  br label %check_ok13

panic.take15:                                     ; preds = %check_ok13
  %79 = load ptr, ptr %__panic, align 8
  %80 = getelementptr i8, ptr %79, i64 0
  %81 = load i8, ptr %80, align 1
  %82 = load ptr, ptr %__panic, align 8
  %83 = getelementptr i8, ptr %82, i64 4
  %84 = load i32, ptr %83, align 4
  %85 = load ptr, ptr %__panic, align 8
  %86 = getelementptr i8, ptr %85, i64 4
  %87 = load i32, ptr %86, align 4
  %88 = zext i32 %87 to i64
  ret i64 %88

panic.cont16:                                     ; preds = %check_ok13
  %89 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %71, i64 16)
  %90 = extractvalue { i64, i1 } %89, 0
  %91 = extractvalue { i64, i1 } %89, 1
  %92 = freeze i64 %90
  br i1 %91, label %op_fail18, label %op_ok17

op_ok17:                                          ; preds = %panic.cont16
  br i1 true, label %check_ok19, label %check_fail20

op_fail18:                                        ; preds = %panic.cont16
  %93 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %93, align 1
  %94 = getelementptr i8, ptr %93, i64 4
  store i32 4, ptr %94, align 4
  ret i64 0

check_ok19:                                       ; preds = %check_fail20, %op_ok17
  %95 = load ptr, ptr %__panic, align 8
  %96 = load i8, ptr %95, align 1
  %97 = icmp ne i8 %96, 0
  br i1 %97, label %panic.take21, label %panic.cont22

check_fail20:                                     ; preds = %op_ok17
  %98 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %98, align 1
  %99 = getelementptr i8, ptr %98, i64 4
  store i32 4, ptr %99, align 4
  br label %check_ok19

panic.take21:                                     ; preds = %check_ok19
  %100 = load ptr, ptr %__panic, align 8
  %101 = getelementptr i8, ptr %100, i64 0
  %102 = load i8, ptr %101, align 1
  %103 = load ptr, ptr %__panic, align 8
  %104 = getelementptr i8, ptr %103, i64 4
  %105 = load i32, ptr %104, align 4
  %106 = load ptr, ptr %__panic, align 8
  %107 = getelementptr i8, ptr %106, i64 4
  %108 = load i32, ptr %107, align 4
  %109 = zext i32 %108 to i64
  ret i64 %109

panic.cont22:                                     ; preds = %check_ok19
  %110 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %92, i64 16)
  %111 = extractvalue { i64, i1 } %110, 0
  %112 = extractvalue { i64, i1 } %110, 1
  %113 = freeze i64 %111
  br i1 %112, label %op_fail24, label %op_ok23

op_ok23:                                          ; preds = %panic.cont22
  ret i64 %113

op_fail24:                                        ; preds = %panic.cont22
  %114 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %114, align 1
  %115 = getelementptr i8, ptr %114, i64 4
  store i32 4, ptr %115, align 4
  ret i64 0
}

define i32 @EmitLlLibrary_x3a_x3aemitLlEnumTypeProbe(ptr noundef nonnull align 4 dereferenceable(12) %tagged, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"EmitLlLibrary_x3a_x3aemitLlEnumTypeProbe$tmp$if_case_clause_result_409" = alloca i32, align 4
  %is_ready = alloca i8, align 1
  %value = alloca i32, align 4
  %"EmitLlLibrary_x3a_x3aemitLlEnumTypeProbe$tmp$if_case_clause_result_404" = alloca i32, align 4
  %count = alloca i32, align 4
  %"EmitLlLibrary_x3a_x3aemitLlEnumTypeProbe$tmp$if_case_clause_result_402" = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %9 = load i8, ptr %tagged, align 1
  %10 = icmp eq i8 %9, 0
  br i1 %10, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %if.merge, %ifcase.case1, %ifcase.case
  %ifcase.result = phi i32 [ %11, %ifcase.case ], [ %23, %ifcase.case1 ], [ %47, %if.merge ]
  ret i32 %ifcase.result

ifcase.case:                                      ; preds = %poison.cont
  store i32 0, ptr %"EmitLlLibrary_x3a_x3aemitLlEnumTypeProbe$tmp$if_case_clause_result_402", align 4
  %11 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlEnumTypeProbe$tmp$if_case_clause_result_402", align 4
  br label %ifcase.merge

ifcase.next:                                      ; preds = %poison.cont
  %12 = load i8, ptr %tagged, align 1
  %13 = icmp eq i8 %12, 1
  %14 = getelementptr i8, ptr %tagged, i64 4
  %15 = getelementptr i8, ptr %14, i64 0
  %16 = load i32, ptr %15, align 1
  %17 = and i1 %13, true
  br i1 %17, label %ifcase.case1, label %ifcase.next2

ifcase.case1:                                     ; preds = %ifcase.next
  %18 = getelementptr i8, ptr %tagged, i64 4
  %19 = getelementptr i8, ptr %18, i64 0
  %20 = load i32, ptr %19, align 1
  store i32 %20, ptr %count, align 4
  %21 = load i32, ptr %count, align 4
  %22 = load i32, ptr %count, align 1
  store i32 %22, ptr %"EmitLlLibrary_x3a_x3aemitLlEnumTypeProbe$tmp$if_case_clause_result_404", align 4
  %23 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlEnumTypeProbe$tmp$if_case_clause_result_404", align 4
  br label %ifcase.merge

ifcase.next2:                                     ; preds = %ifcase.next
  %24 = load i8, ptr %tagged, align 1
  %25 = icmp eq i8 %24, 2
  %26 = and i1 %25, true
  br i1 %26, label %ifcase.case3, label %ifcase.unmatched

ifcase.case3:                                     ; preds = %ifcase.next2
  %27 = getelementptr i8, ptr %tagged, i64 4
  %28 = getelementptr i8, ptr %27, i64 0
  %29 = load i32, ptr %28, align 1
  store i32 %29, ptr %value, align 4
  %30 = getelementptr i8, ptr %tagged, i64 4
  %31 = getelementptr i8, ptr %30, i64 4
  %32 = load i8, ptr %31, align 1
  %33 = icmp ne i8 %32, 0
  %34 = zext i1 %33 to i8
  store i8 %34, ptr %is_ready, align 1
  %35 = load i8, ptr %is_ready, align 1
  %36 = icmp ne i8 %35, 0
  %37 = load i8, ptr %is_ready, align 1
  %38 = icmp ne i8 %37, 0
  br i1 %38, label %if.then, label %if.else

ifcase.unmatched:                                 ; preds = %ifcase.next2
  %39 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %39, align 1
  %40 = getelementptr i8, ptr %39, i64 4
  store i32 18, ptr %40, align 4
  %41 = load ptr, ptr %__panic, align 8
  %42 = getelementptr i8, ptr %41, i64 4
  %43 = load i32, ptr %42, align 4
  ret i32 %43

if.then:                                          ; preds = %ifcase.case3
  %44 = load i32, ptr %value, align 4
  %45 = load i32, ptr %value, align 4
  %46 = load i32, ptr %value, align 4
  br label %if.merge

if.else:                                          ; preds = %ifcase.case3
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %"EmitLlLibrary_x3a_x3aemitLlEnumTypeProbe$tmp$if_408" = phi i32 [ %46, %if.then ], [ 0, %if.else ]
  store i32 %"EmitLlLibrary_x3a_x3aemitLlEnumTypeProbe$tmp$if_408", ptr %"EmitLlLibrary_x3a_x3aemitLlEnumTypeProbe$tmp$if_case_clause_result_409", align 4
  %47 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlEnumTypeProbe$tmp$if_case_clause_result_409", align 4
  br label %ifcase.merge
}

define i32 @EmitLlLibrary_x3a_x3aemitLlTaggedUnionTypeProbe(ptr noundef nonnull align 4 dereferenceable(8) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"EmitLlLibrary_x3a_x3aemitLlTaggedUnionTypeProbe$tmp$if_case_clause_result_420" = alloca i32, align 4
  %value3 = alloca i8, align 1
  %flag = alloca i8, align 1
  %"EmitLlLibrary_x3a_x3aemitLlTaggedUnionTypeProbe$tmp$if_case_clause_result_414" = alloca i32, align 4
  %value1 = alloca i32, align 4
  %number = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %10 = icmp eq i8 %9, 1
  br i1 %10, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %if.merge, %ifcase.case
  %ifcase.result = phi i32 [ %19, %ifcase.case ], [ %41, %if.merge ]
  ret i32 %ifcase.result

ifcase.case:                                      ; preds = %poison.cont
  %11 = getelementptr i8, ptr %value, i64 4
  %12 = getelementptr i8, ptr %11, i64 0
  %13 = load i32, ptr %12, align 1
  store i32 %13, ptr %number, align 4
  %14 = getelementptr i8, ptr %value, i64 4
  %15 = getelementptr i8, ptr %14, i64 0
  %16 = load i32, ptr %15, align 1
  store i32 %16, ptr %value1, align 4
  %17 = load i32, ptr %number, align 4
  %18 = load i32, ptr %number, align 1
  store i32 %18, ptr %"EmitLlLibrary_x3a_x3aemitLlTaggedUnionTypeProbe$tmp$if_case_clause_result_414", align 4
  %19 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlTaggedUnionTypeProbe$tmp$if_case_clause_result_414", align 4
  br label %ifcase.merge

ifcase.next:                                      ; preds = %poison.cont
  %20 = load i8, ptr %value, align 1
  %21 = icmp eq i8 %20, 0
  br i1 %21, label %ifcase.case2, label %ifcase.unmatched

ifcase.case2:                                     ; preds = %ifcase.next
  %22 = getelementptr i8, ptr %value, i64 4
  %23 = getelementptr i8, ptr %22, i64 0
  %24 = load i8, ptr %23, align 1
  %25 = icmp ne i8 %24, 0
  %26 = zext i1 %25 to i8
  store i8 %26, ptr %flag, align 1
  %27 = getelementptr i8, ptr %value, i64 4
  %28 = getelementptr i8, ptr %27, i64 0
  %29 = load i8, ptr %28, align 1
  %30 = icmp ne i8 %29, 0
  %31 = zext i1 %30 to i8
  store i8 %31, ptr %value3, align 1
  %32 = load i8, ptr %flag, align 1
  %33 = icmp ne i8 %32, 0
  %34 = load i8, ptr %flag, align 1
  %35 = icmp ne i8 %34, 0
  br i1 %35, label %if.then, label %if.else

ifcase.unmatched:                                 ; preds = %ifcase.next
  %36 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %36, align 1
  %37 = getelementptr i8, ptr %36, i64 4
  store i32 18, ptr %37, align 4
  %38 = load ptr, ptr %__panic, align 8
  %39 = getelementptr i8, ptr %38, i64 4
  %40 = load i32, ptr %39, align 4
  ret i32 %40

if.then:                                          ; preds = %ifcase.case2
  br label %if.merge

if.else:                                          ; preds = %ifcase.case2
  br label %if.merge

if.merge:                                         ; preds = %if.else, %if.then
  %"EmitLlLibrary_x3a_x3aemitLlTaggedUnionTypeProbe$tmp$if_419" = phi i32 [ 1, %if.then ], [ 0, %if.else ]
  store i32 %"EmitLlLibrary_x3a_x3aemitLlTaggedUnionTypeProbe$tmp$if_419", ptr %"EmitLlLibrary_x3a_x3aemitLlTaggedUnionTypeProbe$tmp$if_case_clause_result_420", align 4
  %41 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlTaggedUnionTypeProbe$tmp$if_case_clause_result_420", align 4
  br label %ifcase.merge
}

define i64 @EmitLlLibrary_x3a_x3aemitLlNicheUnionTypeProbe(ptr noundef nonnull align 8 dereferenceable(8) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  ret i64 8
}

define i32 @EmitLlLibrary_x3a_x3aemitLlClosureTypeProbe(ptr noundef nonnull align 8 dereferenceable(16) %callback, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"EmitLlLibrary_x3a_x3aemitLlClosureTypeProbe$tmp$closure_call_value_4281" = alloca i32, align 4
  %"EmitLlLibrary_x3a_x3aemitLlClosureTypeProbe$tmp$closure_call_value_428" = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %9 = load { ptr, ptr }, ptr %callback, align 8
  %10 = extractvalue { ptr, ptr } %9, 1
  %11 = load { ptr, ptr }, ptr %callback, align 8
  %12 = extractvalue { ptr, ptr } %11, 0
  %13 = call i32 %10(ptr %12, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %14 = load ptr, ptr %__panic, align 8
  %15 = load i8, ptr %14, align 1
  %16 = icmp ne i8 %15, 0
  br i1 %16, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont
  %17 = load ptr, ptr %__panic, align 8
  %18 = getelementptr i8, ptr %17, i64 4
  %19 = load i32, ptr %18, align 4
  ret i32 %19

panic.cont:                                       ; preds = %poison.cont
  store i32 %13, ptr %"EmitLlLibrary_x3a_x3aemitLlClosureTypeProbe$tmp$closure_call_value_4281", align 4
  %20 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlClosureTypeProbe$tmp$closure_call_value_4281", align 4
  ret i32 %20
}

define i32 @EmitLlLibrary_x3a_x3aemitLlClosureEnvReferenceCaptureProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"EmitLlLibrary_x3a_x3aemitLlClosureEnvReferenceCaptureProbe$tmp$closure_call_value_4741" = alloca i32, align 4
  %"EmitLlLibrary_x3a_x3aemitLlClosureEnvReferenceCaptureProbe$tmp$closure_call_value_474" = alloca i32, align 4
  %closure = alloca { ptr, ptr }, align 8
  %"EmitLlLibrary_x3a_x3aemitLlClosureEnvReferenceCaptureProbe$tmp$closure_env_storage_434" = alloca { ptr, ptr }, align 8
  %is_enabled = alloca i8, align 1
  %left_bias = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  store i32 7, ptr %left_bias, align 4
  store i8 1, ptr %is_enabled, align 1
  store { ptr, ptr } zeroinitializer, ptr %"EmitLlLibrary_x3a_x3aemitLlClosureEnvReferenceCaptureProbe$tmp$closure_env_storage_434", align 8
  %9 = getelementptr i8, ptr %"EmitLlLibrary_x3a_x3aemitLlClosureEnvReferenceCaptureProbe$tmp$closure_env_storage_434", i64 0
  store ptr %is_enabled, ptr %9, align 8
  %10 = getelementptr i8, ptr %"EmitLlLibrary_x3a_x3aemitLlClosureEnvReferenceCaptureProbe$tmp$closure_env_storage_434", i64 8
  store ptr %left_bias, ptr %10, align 8
  %11 = insertvalue { ptr, ptr } zeroinitializer, ptr %"EmitLlLibrary_x3a_x3aemitLlClosureEnvReferenceCaptureProbe$tmp$closure_env_storage_434", 0
  %12 = insertvalue { ptr, ptr } %11, ptr @EmitLlLibrary_x5fx3a_x5fx3aemitLlClosureEnvReferenceCaptureProbe_x3a_x3a_x5fclosure0, 1
  store { ptr, ptr } %12, ptr %closure, align 8
  %13 = load { ptr, ptr }, ptr %closure, align 8
  %14 = extractvalue { ptr, ptr } %13, 1
  %15 = load { ptr, ptr }, ptr %closure, align 8
  %16 = extractvalue { ptr, ptr } %15, 0
  %17 = call i32 @EmitLlLibrary_x5fx3a_x5fx3aemitLlClosureEnvReferenceCaptureProbe_x3a_x3a_x5fclosure0(ptr %16, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
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
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 4
  %29 = load i32, ptr %28, align 4
  ret i32 %29

panic.cont:                                       ; preds = %poison.cont
  store i32 %17, ptr %"EmitLlLibrary_x3a_x3aemitLlClosureEnvReferenceCaptureProbe$tmp$closure_call_value_4741", align 4
  %30 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlClosureEnvReferenceCaptureProbe$tmp$closure_call_value_4741", align 4
  ret i32 %30
}

define internal i32 @EmitLlLibrary_x5fx3a_x5fx3aemitLlClosureEnvReferenceCaptureProbe_x3a_x3a_x5fclosure0(ptr %__env, ptr noundef nonnull align 4 dereferenceable(4) %input, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %11 = load i8, ptr %10, align 1
  %12 = icmp ne i8 %11, 0
  %13 = icmp ne i8 %11, 0
  br i1 %13, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  %14 = getelementptr i8, ptr %__env, i64 8
  %15 = load ptr, ptr %14, align 8
  %16 = load i32, ptr %15, align 4
  br i1 true, label %check_ok, label %check_fail

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else
  %17 = load i32, ptr %input, align 4
  ret i32 %17

check_ok:                                         ; preds = %check_fail, %if.then
  %18 = load ptr, ptr %__panic, align 8
  %19 = load i8, ptr %18, align 1
  %20 = icmp ne i8 %19, 0
  br i1 %20, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %if.then
  %21 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %21, align 1
  %22 = getelementptr i8, ptr %21, i64 4
  store i32 4, ptr %22, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %23 = load ptr, ptr %__panic, align 8
  %24 = getelementptr i8, ptr %23, i64 0
  %25 = load i8, ptr %24, align 1
  %26 = load ptr, ptr %__panic, align 8
  %27 = getelementptr i8, ptr %26, i64 4
  %28 = load i32, ptr %27, align 4
  %29 = load ptr, ptr %__panic, align 8
  %30 = getelementptr i8, ptr %29, i64 4
  %31 = load i32, ptr %30, align 4
  ret i32 %31

panic.cont:                                       ; preds = %check_ok
  %32 = load i32, ptr %input, align 4
  %33 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %32, i32 %16)
  %34 = extractvalue { i32, i1 } %33, 0
  %35 = extractvalue { i32, i1 } %33, 1
  %36 = freeze i32 %34
  br i1 %35, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  ret i32 %36

op_fail:                                          ; preds = %panic.cont
  %37 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %37, align 1
  %38 = getelementptr i8, ptr %37, i64 4
  store i32 4, ptr %38, align 4
  ret i32 0
}

define i32 @EmitLlLibrary_x3a_x3aemitLlClosureEnvEmptyProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"EmitLlLibrary_x3a_x3aemitLlClosureEnvEmptyProbe$tmp$closure_call_value_4991" = alloca i32, align 4
  %"EmitLlLibrary_x3a_x3aemitLlClosureEnvEmptyProbe$tmp$closure_call_value_499" = alloca i32, align 4
  %coerce_bits = alloca { i64, ptr }, align 8
  %closure = alloca { ptr, ptr }, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  store { i64, ptr } { i64 0, ptr @EmitLlLibrary_x5fx3a_x5fx3aemitLlClosureEnvEmptyProbe_x3a_x3a_x5fclosure0 }, ptr %coerce_bits, align 8
  %9 = load { ptr, ptr }, ptr %coerce_bits, align 1
  store { ptr, ptr } %9, ptr %closure, align 8
  %10 = load { ptr, ptr }, ptr %closure, align 8
  %11 = extractvalue { ptr, ptr } %10, 1
  %12 = load { ptr, ptr }, ptr %closure, align 8
  %13 = extractvalue { ptr, ptr } %12, 0
  %14 = call i32 @EmitLlLibrary_x5fx3a_x5fx3aemitLlClosureEnvEmptyProbe_x3a_x3a_x5fclosure0(ptr %13, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %15 = load ptr, ptr %__panic, align 8
  %16 = load i8, ptr %15, align 1
  %17 = icmp ne i8 %16, 0
  br i1 %17, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont
  %18 = load ptr, ptr %__panic, align 8
  %19 = getelementptr i8, ptr %18, i64 0
  %20 = load i8, ptr %19, align 1
  %21 = load ptr, ptr %__panic, align 8
  %22 = getelementptr i8, ptr %21, i64 4
  %23 = load i32, ptr %22, align 4
  %24 = load ptr, ptr %__panic, align 8
  %25 = getelementptr i8, ptr %24, i64 4
  %26 = load i32, ptr %25, align 4
  ret i32 %26

panic.cont:                                       ; preds = %poison.cont
  store i32 %14, ptr %"EmitLlLibrary_x3a_x3aemitLlClosureEnvEmptyProbe$tmp$closure_call_value_4991", align 4
  %27 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlClosureEnvEmptyProbe$tmp$closure_call_value_4991", align 4
  ret i32 %27
}

define internal i32 @EmitLlLibrary_x5fx3a_x5fx3aemitLlClosureEnvEmptyProbe_x3a_x3a_x5fclosure0(ptr %__env, ptr noundef nonnull align 4 dereferenceable(4) %input, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %23 = load i32, ptr %input, align 4
  %24 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %23, i32 3)
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

define i32 @EmitLlLibrary_x3a_x3aemitLlClosureEnvMoveCaptureProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"EmitLlLibrary_x3a_x3aemitLlClosureEnvMoveCaptureProbe$tmp$closure_call_value_5273" = alloca i32, align 4
  %"EmitLlLibrary_x3a_x3aemitLlClosureEnvMoveCaptureProbe$tmp$closure_call_value_527" = alloca i32, align 4
  %closure = alloca { ptr, ptr }, align 8
  %"EmitLlLibrary_x3a_x3aemitLlClosureEnvMoveCaptureProbe$tmp$closure_env_storage_511" = alloca { i32 }, align 4
  %captured = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %17 = load i32, ptr %value, align 4
  %18 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %17, i32 4)
  %19 = extractvalue { i32, i1 } %18, 0
  %20 = extractvalue { i32, i1 } %18, 1
  %21 = freeze i32 %19
  br i1 %20, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  store i32 %21, ptr %captured, align 4
  store { i32 } zeroinitializer, ptr %"EmitLlLibrary_x3a_x3aemitLlClosureEnvMoveCaptureProbe$tmp$closure_env_storage_511", align 4
  %22 = getelementptr i8, ptr %"EmitLlLibrary_x3a_x3aemitLlClosureEnvMoveCaptureProbe$tmp$closure_env_storage_511", i64 0
  %23 = load i32, ptr %captured, align 4
  store i32 %23, ptr %22, align 4
  %24 = insertvalue { ptr, ptr } zeroinitializer, ptr %"EmitLlLibrary_x3a_x3aemitLlClosureEnvMoveCaptureProbe$tmp$closure_env_storage_511", 0
  %25 = insertvalue { ptr, ptr } %24, ptr @EmitLlLibrary_x5fx3a_x5fx3aemitLlClosureEnvMoveCaptureProbe_x3a_x3a_x5fclosure0, 1
  store { ptr, ptr } %25, ptr %closure, align 8
  %26 = load { ptr, ptr }, ptr %closure, align 8
  %27 = extractvalue { ptr, ptr } %26, 1
  %28 = load { ptr, ptr }, ptr %closure, align 8
  %29 = extractvalue { ptr, ptr } %28, 0
  %30 = call i32 @EmitLlLibrary_x5fx3a_x5fx3aemitLlClosureEnvMoveCaptureProbe_x3a_x3a_x5fclosure0(ptr %29, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %31 = load ptr, ptr %__panic, align 8
  %32 = load i8, ptr %31, align 1
  %33 = icmp ne i8 %32, 0
  br i1 %33, label %panic.take1, label %panic.cont2

op_fail:                                          ; preds = %panic.cont
  %34 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %34, align 1
  %35 = getelementptr i8, ptr %34, i64 4
  store i32 4, ptr %35, align 4
  ret i32 0

panic.take1:                                      ; preds = %op_ok
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

panic.cont2:                                      ; preds = %op_ok
  store i32 %30, ptr %"EmitLlLibrary_x3a_x3aemitLlClosureEnvMoveCaptureProbe$tmp$closure_call_value_5273", align 4
  %45 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlClosureEnvMoveCaptureProbe$tmp$closure_call_value_5273", align 4
  ret i32 %45
}

define internal i32 @EmitLlLibrary_x5fx3a_x5fx3aemitLlClosureEnvMoveCaptureProbe_x3a_x3a_x5fclosure0(ptr %__env, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %10 = load i32, ptr %9, align 4
  ret i32 %10
}

define i32 @EmitLlLibrary_x3a_x3aemitLlPipelineIncrement(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %17 = load i32, ptr %value, align 4
  %18 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %17, i32 1)
  %19 = extractvalue { i32, i1 } %18, 0
  %20 = extractvalue { i32, i1 } %18, 1
  %21 = freeze i32 %19
  br i1 %20, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  ret i32 %21

op_fail:                                          ; preds = %panic.cont
  %22 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %22, align 1
  %23 = getelementptr i8, ptr %22, i64 4
  store i32 4, ptr %23, align 4
  ret i32 0
}

define i32 @EmitLlLibrary_x3a_x3aemitLlPipelineFunctionProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %23 = call i32 @EmitLlLibrary_x3a_x3aemitLlPipelineIncrement(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
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

define i32 @EmitLlLibrary_x3a_x3aemitLlPipelineClosureProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %closure = alloca { ptr, ptr }, align 8
  %"EmitLlLibrary_x3a_x3aemitLlPipelineClosureProbe$tmp$closure_env_storage_543" = alloca { ptr }, align 8
  %bias = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  store i32 5, ptr %bias, align 4
  store { ptr } zeroinitializer, ptr %"EmitLlLibrary_x3a_x3aemitLlPipelineClosureProbe$tmp$closure_env_storage_543", align 8
  %9 = getelementptr i8, ptr %"EmitLlLibrary_x3a_x3aemitLlPipelineClosureProbe$tmp$closure_env_storage_543", i64 0
  store ptr %bias, ptr %9, align 8
  %10 = insertvalue { ptr, ptr } zeroinitializer, ptr %"EmitLlLibrary_x3a_x3aemitLlPipelineClosureProbe$tmp$closure_env_storage_543", 0
  %11 = insertvalue { ptr, ptr } %10, ptr @EmitLlLibrary_x5fx3a_x5fx3aemitLlPipelineClosureProbe_x3a_x3a_x5fclosure0, 1
  store { ptr, ptr } %11, ptr %closure, align 8
  %12 = load { ptr, ptr }, ptr %closure, align 8
  %13 = extractvalue { ptr, ptr } %12, 1
  %14 = load { ptr, ptr }, ptr %closure, align 8
  %15 = extractvalue { ptr, ptr } %14, 0
  %16 = call i32 @EmitLlLibrary_x5fx3a_x5fx3aemitLlPipelineClosureProbe_x3a_x3a_x5fclosure0(ptr %15, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
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
  ret i32 %16
}

define internal i32 @EmitLlLibrary_x5fx3a_x5fx3aemitLlPipelineClosureProbe_x3a_x3a_x5fclosure0(ptr %__env, ptr noundef nonnull align 4 dereferenceable(4) %input, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %26 = load i32, ptr %input, align 4
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

define i64 @EmitLlLibrary_x3a_x3aemitLlStringViewTypeProbe(ptr noundef nonnull align 8 dereferenceable(16) %text, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  ret i64 16
}

define i64 @EmitLlLibrary_x3a_x3aemitLlStringManagedTypeProbe(ptr noundef nonnull align 8 dereferenceable(24) %text, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %byref_arg = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  %10 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %10, align 1
  %11 = getelementptr i8, ptr %10, i64 4
  store i32 0, ptr %11, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3astring_x3a_x3adrop_x5fmanaged(ptr noundef nonnull align 8 dereferenceable(24) %text)
  %12 = load ptr, ptr %__panic, align 8
  %13 = getelementptr i8, ptr %12, i64 0
  %14 = load i8, ptr %13, align 1
  %15 = load ptr, ptr %__panic, align 8
  %16 = getelementptr i8, ptr %15, i64 4
  %17 = load i32, ptr %16, align 4
  %18 = icmp ne i8 %14, 0
  %19 = zext i1 %18 to i8
  %20 = icmp ne i8 %19, 0
  %21 = and i1 false, %20
  br i1 %21, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  store i32 %17, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else
  %22 = icmp ne i8 %14, 0
  %23 = xor i1 %22, true
  %24 = and i1 false, %23
  br i1 %24, label %if.then1, label %if.else2

if.then1:                                         ; preds = %if.merge
  %25 = load ptr, ptr %__panic, align 8
  %26 = getelementptr i8, ptr %25, i64 0
  store i8 1, ptr %26, align 1
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 4
  store i32 0, ptr %28, align 4
  br label %if.merge3

if.else2:                                         ; preds = %if.merge
  br label %if.merge3

if.merge3:                                        ; preds = %if.else2, %if.then1
  %if.result = phi i64 [ 0, %if.then1 ], [ 0, %if.else2 ]
  %29 = icmp ne i8 %14, 0
  %30 = zext i1 %29 to i8
  %31 = icmp ne i8 %30, 0
  %32 = or i1 false, %31
  %33 = icmp ne i8 %14, 0
  %34 = icmp ne i8 %14, 0
  br i1 %34, label %if.then4, label %if.else5

if.then4:                                         ; preds = %if.merge3
  br label %if.merge6

if.else5:                                         ; preds = %if.merge3
  br label %if.merge6

if.merge6:                                        ; preds = %if.else5, %if.then4
  %if.result7 = phi i32 [ %17, %if.then4 ], [ 0, %if.else5 ]
  %35 = load ptr, ptr %__panic, align 8
  %36 = load i8, ptr %35, align 1
  %37 = icmp ne i8 %36, 0
  br i1 %37, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %if.merge6
  %38 = load ptr, ptr %__panic, align 8
  %39 = getelementptr i8, ptr %38, i64 4
  %40 = load i32, ptr %39, align 4
  %41 = zext i32 %40 to i64
  ret i64 %41

panic.cont:                                       ; preds = %if.merge6
  ret i64 24
}

define i64 @EmitLlLibrary_x3a_x3aemitLlBytesViewTypeProbe(ptr noundef nonnull align 8 dereferenceable(16) %data, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  ret i64 16
}

define i64 @EmitLlLibrary_x3a_x3aemitLlBytesManagedTypeProbe(ptr noundef nonnull align 8 dereferenceable(24) %data, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %byref_arg = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  %10 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %10, align 1
  %11 = getelementptr i8, ptr %10, i64 4
  store i32 0, ptr %11, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3abytes_x3a_x3adrop_x5fmanaged(ptr noundef nonnull align 8 dereferenceable(24) %data)
  %12 = load ptr, ptr %__panic, align 8
  %13 = getelementptr i8, ptr %12, i64 0
  %14 = load i8, ptr %13, align 1
  %15 = load ptr, ptr %__panic, align 8
  %16 = getelementptr i8, ptr %15, i64 4
  %17 = load i32, ptr %16, align 4
  %18 = icmp ne i8 %14, 0
  %19 = zext i1 %18 to i8
  %20 = icmp ne i8 %19, 0
  %21 = and i1 false, %20
  br i1 %21, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  store i32 %17, ptr %byref_arg, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg)
  unreachable

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else
  %22 = icmp ne i8 %14, 0
  %23 = xor i1 %22, true
  %24 = and i1 false, %23
  br i1 %24, label %if.then1, label %if.else2

if.then1:                                         ; preds = %if.merge
  %25 = load ptr, ptr %__panic, align 8
  %26 = getelementptr i8, ptr %25, i64 0
  store i8 1, ptr %26, align 1
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 4
  store i32 0, ptr %28, align 4
  br label %if.merge3

if.else2:                                         ; preds = %if.merge
  br label %if.merge3

if.merge3:                                        ; preds = %if.else2, %if.then1
  %if.result = phi i64 [ 0, %if.then1 ], [ 0, %if.else2 ]
  %29 = icmp ne i8 %14, 0
  %30 = zext i1 %29 to i8
  %31 = icmp ne i8 %30, 0
  %32 = or i1 false, %31
  %33 = icmp ne i8 %14, 0
  %34 = icmp ne i8 %14, 0
  br i1 %34, label %if.then4, label %if.else5

if.then4:                                         ; preds = %if.merge3
  br label %if.merge6

if.else5:                                         ; preds = %if.merge3
  br label %if.merge6

if.merge6:                                        ; preds = %if.else5, %if.then4
  %if.result7 = phi i32 [ %17, %if.then4 ], [ 0, %if.else5 ]
  %35 = load ptr, ptr %__panic, align 8
  %36 = load i8, ptr %35, align 1
  %37 = icmp ne i8 %36, 0
  br i1 %37, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %if.merge6
  %38 = load ptr, ptr %__panic, align 8
  %39 = getelementptr i8, ptr %38, i64 4
  %40 = load i32, ptr %39, align 4
  %41 = zext i32 %40 to i64
  ret i64 %41

panic.cont:                                       ; preds = %if.merge6
  ret i64 24
}

define i64 @EmitLlLibrary_x3a_x3aemitLlModalStringTypeProbe(ptr noundef nonnull align 8 dereferenceable(32) %text, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  ret i64 32
}

define i64 @EmitLlLibrary_x3a_x3aemitLlModalBytesTypeProbe(ptr noundef nonnull align 8 dereferenceable(32) %data, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  ret i64 32
}

define i64 @EmitLlLibrary_x3a_x3aemitLlNicheModalTypeProbe(ptr noundef nonnull align 8 dereferenceable(8) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  ret i64 8
}

define i64 @EmitLlLibrary_x3a_x3aemitLlTaggedModalTypeProbe(ptr noundef nonnull align 8 dereferenceable(24) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  ret i64 24
}

define i32 @EmitLlLibrary_x3a_x3aemitLlModalStateTypeProbe(ptr noundef nonnull align 8 dereferenceable(16) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %9 = getelementptr i8, ptr %value, i64 8
  %10 = load i32, ptr %9, align 1
  ret i32 %10
}

define i32 @EmitLlLibrary_x3a_x3aemitLlNicheModalLayoutProbe(ptr noundef nonnull align 8 dereferenceable(8) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"EmitLlLibrary_x3a_x3aemitLlNicheModalLayoutProbe$tmp$if_case_clause_result_641" = alloca i32, align 4
  %0 = alloca ptr, align 8
  %pointer = alloca ptr, align 8
  %"EmitLlLibrary_x3a_x3aemitLlNicheModalLayoutProbe$tmp$if_case_clause_result_637" = alloca i32, align 4
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %10 = load ptr, ptr %value, align 8
  br i1 false, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %panic.cont5, %ifcase.case
  %ifcase.result = phi i32 [ %11, %ifcase.case ], [ %44, %panic.cont5 ]
  ret i32 %ifcase.result

ifcase.case:                                      ; preds = %poison.cont
  store i32 0, ptr %"EmitLlLibrary_x3a_x3aemitLlNicheModalLayoutProbe$tmp$if_case_clause_result_637", align 4
  %11 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlNicheModalLayoutProbe$tmp$if_case_clause_result_637", align 4
  br label %ifcase.merge

ifcase.next:                                      ; preds = %poison.cont
  br i1 true, label %ifcase.case1, label %ifcase.unmatched

ifcase.case1:                                     ; preds = %ifcase.next
  %12 = getelementptr i8, ptr %value, i64 0
  %13 = load ptr, ptr %value, align 8
  store ptr %13, ptr %0, align 8
  %14 = getelementptr i8, ptr %0, i64 0
  %15 = load ptr, ptr %14, align 1
  store ptr %15, ptr %pointer, align 8
  %16 = load ptr, ptr %pointer, align 8
  %17 = icmp ne ptr %16, null
  br i1 %17, label %check_ok, label %check_fail

ifcase.unmatched:                                 ; preds = %ifcase.next
  %18 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %18, align 1
  %19 = getelementptr i8, ptr %18, i64 4
  store i32 18, ptr %19, align 4
  %20 = load ptr, ptr %__panic, align 8
  %21 = getelementptr i8, ptr %20, i64 4
  %22 = load i32, ptr %21, align 4
  ret i32 %22

check_ok:                                         ; preds = %check_fail, %ifcase.case1
  %23 = load ptr, ptr %__panic, align 8
  %24 = load i8, ptr %23, align 1
  %25 = icmp ne i8 %24, 0
  br i1 %25, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %ifcase.case1
  %26 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %26, align 1
  %27 = getelementptr i8, ptr %26, i64 4
  store i32 8, ptr %27, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %28 = load ptr, ptr %__panic, align 8
  %29 = getelementptr i8, ptr %28, i64 4
  %30 = load i32, ptr %29, align 4
  ret i32 %30

panic.cont:                                       ; preds = %check_ok
  %31 = load ptr, ptr %pointer, align 8
  %32 = call i8 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5fis_x5factive(ptr %31)
  %33 = icmp ne i8 %32, 0
  br i1 %33, label %check_ok2, label %check_fail3

check_ok2:                                        ; preds = %check_fail3, %panic.cont
  %34 = load ptr, ptr %__panic, align 8
  %35 = load i8, ptr %34, align 1
  %36 = icmp ne i8 %35, 0
  br i1 %36, label %panic.take4, label %panic.cont5

check_fail3:                                      ; preds = %panic.cont
  %37 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %37, align 1
  %38 = getelementptr i8, ptr %37, i64 4
  store i32 9, ptr %38, align 4
  br label %check_ok2

panic.take4:                                      ; preds = %check_ok2
  %39 = load ptr, ptr %__panic, align 8
  %40 = getelementptr i8, ptr %39, i64 4
  %41 = load i32, ptr %40, align 4
  ret i32 %41

panic.cont5:                                      ; preds = %check_ok2
  %42 = load ptr, ptr %pointer, align 8
  %43 = load i32, ptr %42, align 4
  store i32 %43, ptr %"EmitLlLibrary_x3a_x3aemitLlNicheModalLayoutProbe$tmp$if_case_clause_result_641", align 4
  %44 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlNicheModalLayoutProbe$tmp$if_case_clause_result_641", align 4
  br label %ifcase.merge
}

define i32 @EmitLlLibrary_x3a_x3aemitLlTaggedModalLayoutProbe(ptr noundef nonnull align 8 dereferenceable(24) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"EmitLlLibrary_x3a_x3aemitLlTaggedModalLayoutProbe$tmp$if_case_clause_result_652" = alloca i32, align 4
  %right = alloca i32, align 4
  %left = alloca i64, align 8
  %"EmitLlLibrary_x3a_x3aemitLlTaggedModalLayoutProbe$tmp$if_case_clause_result_647" = alloca i32, align 4
  %small_value = alloca i32, align 4
  %"EmitLlLibrary_x3a_x3aemitLlTaggedModalLayoutProbe$tmp$if_case_clause_result_645" = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %11 = and i1 %10, true
  br i1 %11, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %op_ok, %ifcase.case1, %ifcase.case
  %ifcase.result = phi i32 [ %12, %ifcase.case ], [ %24, %ifcase.case1 ], [ %66, %op_ok ]
  ret i32 %ifcase.result

ifcase.case:                                      ; preds = %poison.cont
  store i32 0, ptr %"EmitLlLibrary_x3a_x3aemitLlTaggedModalLayoutProbe$tmp$if_case_clause_result_645", align 4
  %12 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlTaggedModalLayoutProbe$tmp$if_case_clause_result_645", align 4
  br label %ifcase.merge

ifcase.next:                                      ; preds = %poison.cont
  %13 = load i8, ptr %value, align 1
  %14 = icmp eq i8 %13, 1
  %15 = getelementptr i8, ptr %value, i64 8
  %16 = getelementptr i8, ptr %15, i64 0
  %17 = load i32, ptr %16, align 1
  %18 = and i1 %14, true
  br i1 %18, label %ifcase.case1, label %ifcase.next2

ifcase.case1:                                     ; preds = %ifcase.next
  %19 = getelementptr i8, ptr %value, i64 8
  %20 = getelementptr i8, ptr %19, i64 0
  %21 = load i32, ptr %20, align 1
  store i32 %21, ptr %small_value, align 4
  %22 = load i32, ptr %small_value, align 4
  %23 = load i32, ptr %small_value, align 1
  store i32 %23, ptr %"EmitLlLibrary_x3a_x3aemitLlTaggedModalLayoutProbe$tmp$if_case_clause_result_647", align 4
  %24 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlTaggedModalLayoutProbe$tmp$if_case_clause_result_647", align 4
  br label %ifcase.merge

ifcase.next2:                                     ; preds = %ifcase.next
  %25 = load i8, ptr %value, align 1
  %26 = icmp eq i8 %25, 2
  %27 = and i1 %26, true
  br i1 %27, label %ifcase.case3, label %ifcase.unmatched

ifcase.case3:                                     ; preds = %ifcase.next2
  %28 = getelementptr i8, ptr %value, i64 8
  %29 = getelementptr i8, ptr %28, i64 0
  %30 = load i64, ptr %29, align 1
  store i64 %30, ptr %left, align 4
  %31 = getelementptr i8, ptr %value, i64 8
  %32 = getelementptr i8, ptr %31, i64 8
  %33 = load i32, ptr %32, align 1
  store i32 %33, ptr %right, align 4
  %34 = load i64, ptr %left, align 4
  %35 = trunc i64 %34 to i32
  %36 = sext i32 %35 to i64
  %37 = icmp eq i64 %34, %36
  br i1 %37, label %check_ok, label %check_fail

ifcase.unmatched:                                 ; preds = %ifcase.next2
  %38 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %38, align 1
  %39 = getelementptr i8, ptr %38, i64 4
  store i32 18, ptr %39, align 4
  %40 = load ptr, ptr %__panic, align 8
  %41 = getelementptr i8, ptr %40, i64 4
  %42 = load i32, ptr %41, align 4
  ret i32 %42

check_ok:                                         ; preds = %check_fail, %ifcase.case3
  %43 = load ptr, ptr %__panic, align 8
  %44 = load i8, ptr %43, align 1
  %45 = icmp ne i8 %44, 0
  br i1 %45, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %ifcase.case3
  %46 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %46, align 1
  %47 = getelementptr i8, ptr %46, i64 4
  store i32 7, ptr %47, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %48 = load ptr, ptr %__panic, align 8
  %49 = getelementptr i8, ptr %48, i64 4
  %50 = load i32, ptr %49, align 4
  ret i32 %50

panic.cont:                                       ; preds = %check_ok
  %51 = load i64, ptr %left, align 4
  %52 = trunc i64 %51 to i32
  br i1 true, label %check_ok4, label %check_fail5

check_ok4:                                        ; preds = %check_fail5, %panic.cont
  %53 = load ptr, ptr %__panic, align 8
  %54 = load i8, ptr %53, align 1
  %55 = icmp ne i8 %54, 0
  br i1 %55, label %panic.take6, label %panic.cont7

check_fail5:                                      ; preds = %panic.cont
  %56 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %56, align 1
  %57 = getelementptr i8, ptr %56, i64 4
  store i32 4, ptr %57, align 4
  br label %check_ok4

panic.take6:                                      ; preds = %check_ok4
  %58 = load ptr, ptr %__panic, align 8
  %59 = getelementptr i8, ptr %58, i64 4
  %60 = load i32, ptr %59, align 4
  ret i32 %60

panic.cont7:                                      ; preds = %check_ok4
  %61 = load i32, ptr %right, align 4
  %62 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %52, i32 %61)
  %63 = extractvalue { i32, i1 } %62, 0
  %64 = extractvalue { i32, i1 } %62, 1
  %65 = freeze i32 %63
  br i1 %64, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont7
  store i32 %65, ptr %"EmitLlLibrary_x3a_x3aemitLlTaggedModalLayoutProbe$tmp$if_case_clause_result_652", align 4
  %66 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlTaggedModalLayoutProbe$tmp$if_case_clause_result_652", align 4
  br label %ifcase.merge

op_fail:                                          ; preds = %panic.cont7
  %67 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %67, align 1
  %68 = getelementptr i8, ptr %67, i64 4
  store i32 4, ptr %68, align 4
  ret i32 0
}

define i32 @EmitLlLibrary_x3a_x3aemitLlNicheModalWidenProbe(ptr noundef nonnull align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %general = alloca ptr, align 8
  %0 = alloca ptr, align 8
  %1 = alloca { ptr, [0 x i64] }, align 8
  %aggregate.literal = alloca { ptr, [0 x i64] }, align 8
  %present = alloca { ptr, [0 x i64] }, align 8
  %2 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  ret i32 %8

poison.cont:                                      ; preds = %entry
  %9 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 0, ptr %10, align 4
  call void @llvm.memset.p0.i64(ptr align 8 %aggregate.literal, i8 0, i64 8, i1 false)
  %11 = load ptr, ptr %pointer, align 8
  store ptr %11, ptr %aggregate.literal, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %present, ptr align 8 %aggregate.literal, i64 8, i1 false)
  %12 = load { ptr, [0 x i64] }, ptr %present, align 8
  store ptr null, ptr %0, align 8
  store { ptr, [0 x i64] } %12, ptr %1, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %0, ptr align 1 %1, i64 8, i1 false)
  %13 = load ptr, ptr %0, align 8
  store ptr %13, ptr %general, align 8
  %14 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %21 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %28 = call i32 @EmitLlLibrary_x3a_x3aemitLlNicheModalLayoutProbe(ptr noundef nonnull align 8 dereferenceable(8) %general, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
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
  ret i32 %28
}

define i32 @EmitLlLibrary_x3a_x3aemitLlNicheModalAbsentWidenProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %general = alloca ptr, align 8
  %0 = alloca ptr, align 8
  %1 = alloca {}, align 8
  %absent = alloca {}, align 1
  %2 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  ret i32 %8

poison.cont:                                      ; preds = %entry
  %9 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 0, ptr %10, align 4
  %11 = load {}, ptr %absent, align 1
  store ptr null, ptr %0, align 8
  store {} %11, ptr %1, align 1
  %12 = load ptr, ptr %0, align 8
  store ptr %12, ptr %general, align 8
  %13 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %14 = icmp ne i8 %13, 0
  br i1 %14, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %15 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %15, align 1
  %16 = getelementptr i8, ptr %15, i64 4
  store i32 10, ptr %16, align 4
  %17 = load ptr, ptr %__panic, align 8
  %18 = getelementptr i8, ptr %17, i64 4
  %19 = load i32, ptr %18, align 4
  ret i32 %19

poison.cont2:                                     ; preds = %poison.cont
  %20 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  ret i32 %26

poison.cont4:                                     ; preds = %poison.cont2
  %27 = call i32 @EmitLlLibrary_x3a_x3aemitLlNicheModalLayoutProbe(ptr noundef nonnull align 8 dereferenceable(8) %general, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %28 = load ptr, ptr %__panic, align 8
  %29 = load i8, ptr %28, align 1
  %30 = icmp ne i8 %29, 0
  br i1 %30, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
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

panic.cont:                                       ; preds = %poison.cont4
  ret i32 %27
}

define i32 @EmitLlLibrary_x3a_x3aemitLlTaggedModalWidenProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %general = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %0 = alloca { i8, [7 x i8], [16 x i8], [0 x i64] }, align 8
  %1 = alloca { i64, i32, [4 x i8], [0 x i64] }, align 8
  %aggregate.literal = alloca { i64, i32, [4 x i8], [0 x i64] }, align 8
  %wide = alloca { i64, i32, [4 x i8], [0 x i64] }, align 8
  %2 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  ret i32 %8

poison.cont:                                      ; preds = %entry
  %9 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 0, ptr %10, align 4
  call void @llvm.memset.p0.i64(ptr align 8 %aggregate.literal, i8 0, i64 16, i1 false)
  store i64 40, ptr %aggregate.literal, align 8
  %11 = getelementptr i8, ptr %aggregate.literal, i64 8
  store i32 3, ptr %11, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %wide, ptr align 8 %aggregate.literal, i64 16, i1 false)
  %12 = load { i64, i32, [4 x i8], [0 x i64] }, ptr %wide, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } zeroinitializer, ptr %0, align 4
  store i8 2, ptr %0, align 1
  %13 = getelementptr i8, ptr %0, i64 8
  store { i64, i32, [4 x i8], [0 x i64] } %12, ptr %1, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %13, ptr align 1 %1, i64 16, i1 false)
  %14 = load { i8, [7 x i8], [16 x i8], [0 x i64] }, ptr %0, align 4
  store { i8, [7 x i8], [16 x i8], [0 x i64] } %14, ptr %general, align 4
  %15 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %16 = icmp ne i8 %15, 0
  br i1 %16, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %17 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %17, align 1
  %18 = getelementptr i8, ptr %17, i64 4
  store i32 10, ptr %18, align 4
  %19 = load ptr, ptr %__panic, align 8
  %20 = getelementptr i8, ptr %19, i64 4
  %21 = load i32, ptr %20, align 4
  ret i32 %21

poison.cont2:                                     ; preds = %poison.cont
  %22 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %23 = icmp ne i8 %22, 0
  br i1 %23, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %24 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %24, align 1
  %25 = getelementptr i8, ptr %24, i64 4
  store i32 10, ptr %25, align 4
  %26 = load ptr, ptr %__panic, align 8
  %27 = getelementptr i8, ptr %26, i64 4
  %28 = load i32, ptr %27, align 4
  ret i32 %28

poison.cont4:                                     ; preds = %poison.cont2
  %29 = call i32 @EmitLlLibrary_x3a_x3aemitLlTaggedModalLayoutProbe(ptr noundef nonnull align 8 dereferenceable(24) %general, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %30 = load ptr, ptr %__panic, align 8
  %31 = load i8, ptr %30, align 1
  %32 = icmp ne i8 %31, 0
  br i1 %32, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
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

panic.cont:                                       ; preds = %poison.cont4
  ret i32 %29
}

define i32 @EmitLlLibrary_x3a_x3aemitLlModalMethodAndTransitionParamsProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %wide8 = alloca { i64, i32, [4 x i8], [0 x i64] }, align 8
  %wide = alloca { i64, i32, [4 x i8], [0 x i64] }, align 8
  %small_for_transition = alloca { i32, [0 x i32] }, align 4
  %read_value3 = alloca i32, align 4
  %read_value = alloca i32, align 4
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %small_for_read = alloca { i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %small_for_read, ptr align 4 %aggregate.literal, i64 4, i1 false)
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %16 = call i32 @EmitLlLibrary_x3a_x3aEmitLlTaggedModal_x3a_x3aSmall_x3a_x3areadValue(ptr noundef nonnull align 4 dereferenceable(4) %small_for_read, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
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
  store i32 %16, ptr %read_value3, align 4
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 4, i1 false)
  store i32 23, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %small_for_transition, ptr align 4 %aggregate.literal, i64 4, i1 false)
  %29 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %30 = icmp ne i8 %29, 0
  br i1 %30, label %poison.take4, label %poison.cont5

poison.take4:                                     ; preds = %panic.cont
  %31 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %31, align 1
  %32 = getelementptr i8, ptr %31, i64 4
  store i32 10, ptr %32, align 4
  %33 = load ptr, ptr %__panic, align 8
  %34 = getelementptr i8, ptr %33, i64 4
  %35 = load i32, ptr %34, align 4
  ret i32 %35

poison.cont5:                                     ; preds = %panic.cont
  %36 = call { i64, i32, [4 x i8], [0 x i64] } @EmitLlLibrary_x3a_x3aEmitLlTaggedModal_x3a_x3aSmall_x3a_x3afinishWide(ptr noundef nonnull align 4 dereferenceable(4) %small_for_transition, ptr noundef nonnull align 4 dereferenceable(4) %read_value3, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %37 = load ptr, ptr %__panic, align 8
  %38 = load i8, ptr %37, align 1
  %39 = icmp ne i8 %38, 0
  br i1 %39, label %panic.take6, label %panic.cont7

panic.take6:                                      ; preds = %poison.cont5
  %40 = load ptr, ptr %__panic, align 8
  %41 = getelementptr i8, ptr %40, i64 0
  %42 = load i8, ptr %41, align 1
  %43 = load ptr, ptr %__panic, align 8
  %44 = getelementptr i8, ptr %43, i64 4
  %45 = load i32, ptr %44, align 4
  %46 = load ptr, ptr %__panic, align 8
  %47 = getelementptr i8, ptr %46, i64 4
  %48 = load i32, ptr %47, align 4
  ret i32 %48

panic.cont7:                                      ; preds = %poison.cont5
  store { i64, i32, [4 x i8], [0 x i64] } %36, ptr %wide8, align 4
  %49 = getelementptr i8, ptr %wide8, i64 8
  %50 = load i32, ptr %49, align 1
  ret i32 %50
}

define i64 @EmitLlLibrary_x3a_x3aemitLlDynamicTypeProbe(ptr noundef nonnull align 8 dereferenceable(16) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  ret i64 16
}

define i32 @EmitLlLibrary_x3a_x3aemitLlDynamicDispatchProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 4 dereferenceable(4) %delta, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %dynamic_owner = alloca { ptr, ptr }, align 8
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %owner = alloca { i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fenter(i64 206)
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 4, i1 false)
  %9 = load i32, ptr %value, align 4
  store i32 %9, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %owner, ptr align 4 %aggregate.literal, i64 4, i1 false)
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5ftag_x5fscope(ptr %owner, i64 206)
  %10 = insertvalue { ptr, ptr } zeroinitializer, ptr %owner, 0
  %11 = insertvalue { ptr, ptr } %10, ptr @vtable_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlDynamicOwner_x3a_x3acl_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlDynamicReadable, 1
  store { ptr, ptr } %11, ptr %dynamic_owner, align 8
  %12 = load { ptr, ptr }, ptr %dynamic_owner, align 8
  %13 = extractvalue { ptr, ptr } %12, 0
  %14 = extractvalue { ptr, ptr } %12, 1
  %15 = getelementptr ptr, ptr %14, i64 3
  %16 = load ptr, ptr %15, align 8
  %17 = call i8 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5fis_x5factive(ptr %13)
  %18 = icmp ne i8 %17, 0
  br i1 %18, label %dynrecv.addr.valid, label %dynrecv.addr.expired

dynrecv.addr.valid:                               ; preds = %poison.cont
  %19 = call i32 %16(ptr %13, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  br label %dynrecv.addr.join

dynrecv.addr.expired:                             ; preds = %poison.cont
  %20 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %20, align 1
  %21 = getelementptr i8, ptr %20, i64 4
  store i32 9, ptr %21, align 4
  br label %dynrecv.addr.join

dynrecv.addr.join:                                ; preds = %dynrecv.addr.valid, %dynrecv.addr.expired
  %dynrecv.result = phi i32 [ %19, %dynrecv.addr.valid ], [ 0, %dynrecv.addr.expired ]
  %22 = load ptr, ptr %__panic, align 8
  %23 = load i8, ptr %22, align 1
  %24 = icmp ne i8 %23, 0
  br i1 %24, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %dynrecv.addr.join
  %25 = load ptr, ptr %__panic, align 8
  %26 = getelementptr i8, ptr %25, i64 0
  %27 = load i8, ptr %26, align 1
  %28 = load ptr, ptr %__panic, align 8
  %29 = getelementptr i8, ptr %28, i64 4
  %30 = load i32, ptr %29, align 4
  %31 = load ptr, ptr %__panic, align 8
  %32 = getelementptr i8, ptr %31, i64 4
  %33 = load i32, ptr %32, align 4
  ret i32 %33

panic.cont:                                       ; preds = %dynrecv.addr.join
  %34 = load { ptr, ptr }, ptr %dynamic_owner, align 8
  %35 = extractvalue { ptr, ptr } %34, 0
  %36 = extractvalue { ptr, ptr } %34, 1
  %37 = getelementptr ptr, ptr %36, i64 4
  %38 = load ptr, ptr %37, align 8
  %39 = call i8 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5fis_x5factive(ptr %35)
  %40 = icmp ne i8 %39, 0
  br i1 %40, label %dynrecv.addr.valid1, label %dynrecv.addr.expired2

dynrecv.addr.valid1:                              ; preds = %panic.cont
  %41 = call i32 %38(ptr %35, ptr noundef nonnull align 4 dereferenceable(4) %delta, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  br label %dynrecv.addr.join3

dynrecv.addr.expired2:                            ; preds = %panic.cont
  %42 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %42, align 1
  %43 = getelementptr i8, ptr %42, i64 4
  store i32 9, ptr %43, align 4
  br label %dynrecv.addr.join3

dynrecv.addr.join3:                               ; preds = %dynrecv.addr.valid1, %dynrecv.addr.expired2
  %dynrecv.result4 = phi i32 [ %41, %dynrecv.addr.valid1 ], [ 0, %dynrecv.addr.expired2 ]
  %44 = load ptr, ptr %__panic, align 8
  %45 = load i8, ptr %44, align 1
  %46 = icmp ne i8 %45, 0
  br i1 %46, label %panic.take5, label %panic.cont6

panic.take5:                                      ; preds = %dynrecv.addr.join3
  %47 = load ptr, ptr %__panic, align 8
  %48 = getelementptr i8, ptr %47, i64 0
  %49 = load i8, ptr %48, align 1
  %50 = load ptr, ptr %__panic, align 8
  %51 = getelementptr i8, ptr %50, i64 4
  %52 = load i32, ptr %51, align 4
  %53 = load ptr, ptr %__panic, align 8
  %54 = getelementptr i8, ptr %53, i64 4
  %55 = load i32, ptr %54, align 4
  ret i32 %55

panic.cont6:                                      ; preds = %dynrecv.addr.join3
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %panic.cont6
  %56 = load ptr, ptr %__panic, align 8
  %57 = load i8, ptr %56, align 1
  %58 = icmp ne i8 %57, 0
  br i1 %58, label %panic.take7, label %panic.cont8

check_fail:                                       ; preds = %panic.cont6
  %59 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %59, align 1
  %60 = getelementptr i8, ptr %59, i64 4
  store i32 4, ptr %60, align 4
  br label %check_ok

panic.take7:                                      ; preds = %check_ok
  %61 = load ptr, ptr %__panic, align 8
  %62 = getelementptr i8, ptr %61, i64 0
  %63 = load i8, ptr %62, align 1
  %64 = load ptr, ptr %__panic, align 8
  %65 = getelementptr i8, ptr %64, i64 4
  %66 = load i32, ptr %65, align 4
  %67 = load ptr, ptr %__panic, align 8
  %68 = getelementptr i8, ptr %67, i64 4
  %69 = load i32, ptr %68, align 4
  ret i32 %69

panic.cont8:                                      ; preds = %check_ok
  %70 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %dynrecv.result, i32 %dynrecv.result4)
  %71 = extractvalue { i32, i1 } %70, 0
  %72 = extractvalue { i32, i1 } %70, 1
  %73 = freeze i32 %71
  br i1 %72, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont8
  ret i32 %73

op_fail:                                          ; preds = %panic.cont8
  %74 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %74, align 1
  %75 = getelementptr i8, ptr %74, i64 4
  store i32 4, ptr %75, align 4
  ret i32 0
}

define i32 @EmitLlLibrary_x3a_x3aemitLlDynamicDefaultDispatchProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %dynamic_owner = alloca { ptr, ptr }, align 8
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %owner = alloca { i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3ascope_x5fenter(i64 208)
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 4, i1 false)
  %9 = load i32, ptr %value, align 4
  store i32 %9, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %owner, ptr align 4 %aggregate.literal, i64 4, i1 false)
  call void @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5ftag_x5fscope(ptr %owner, i64 208)
  %10 = insertvalue { ptr, ptr } zeroinitializer, ptr %owner, 0
  %11 = insertvalue { ptr, ptr } %10, ptr @vtable_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlDynamicDefaultOwner_x3a_x3acl_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlDynamicDefaultReadable, 1
  store { ptr, ptr } %11, ptr %dynamic_owner, align 8
  %12 = load { ptr, ptr }, ptr %dynamic_owner, align 8
  %13 = extractvalue { ptr, ptr } %12, 0
  %14 = extractvalue { ptr, ptr } %12, 1
  %15 = getelementptr ptr, ptr %14, i64 3
  %16 = load ptr, ptr %15, align 8
  %17 = call i8 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5fis_x5factive(ptr %13)
  %18 = icmp ne i8 %17, 0
  br i1 %18, label %dynrecv.addr.valid, label %dynrecv.addr.expired

dynrecv.addr.valid:                               ; preds = %poison.cont
  %19 = call i32 %16(ptr %13, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  br label %dynrecv.addr.join

dynrecv.addr.expired:                             ; preds = %poison.cont
  %20 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %20, align 1
  %21 = getelementptr i8, ptr %20, i64 4
  store i32 9, ptr %21, align 4
  br label %dynrecv.addr.join

dynrecv.addr.join:                                ; preds = %dynrecv.addr.valid, %dynrecv.addr.expired
  %dynrecv.result = phi i32 [ %19, %dynrecv.addr.valid ], [ 0, %dynrecv.addr.expired ]
  %22 = load ptr, ptr %__panic, align 8
  %23 = load i8, ptr %22, align 1
  %24 = icmp ne i8 %23, 0
  br i1 %24, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %dynrecv.addr.join
  %25 = load ptr, ptr %__panic, align 8
  %26 = getelementptr i8, ptr %25, i64 0
  %27 = load i8, ptr %26, align 1
  %28 = load ptr, ptr %__panic, align 8
  %29 = getelementptr i8, ptr %28, i64 4
  %30 = load i32, ptr %29, align 4
  %31 = load ptr, ptr %__panic, align 8
  %32 = getelementptr i8, ptr %31, i64 4
  %33 = load i32, ptr %32, align 4
  ret i32 %33

panic.cont:                                       ; preds = %dynrecv.addr.join
  ret i32 %dynrecv.result
}

define i8 @EmitLlLibrary_x3a_x3aemitLlHashRequiresEqProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %byref_arg4 = alloca { ptr, ptr }, align 8
  %byref_arg = alloca { ptr, ptr }, align 8
  %second_hasher_owner = alloca { i64, [0 x i64] }, align 8
  %aggregate.literal1 = alloca { i64, [0 x i64] }, align 8
  %first_hasher_owner = alloca { i64, [0 x i64] }, align 8
  %right = alloca { i32, [0 x i32] }, align 4
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %left = alloca { i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 4, i1 false)
  %10 = load i32, ptr %value, align 4
  store i32 %10, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %left, ptr align 4 %aggregate.literal, i64 4, i1 false)
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 4, i1 false)
  %11 = load i32, ptr %value, align 4
  store i32 %11, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %right, ptr align 4 %aggregate.literal, i64 4, i1 false)
  call void @llvm.memset.p0.i64(ptr align 8 %aggregate.literal1, i8 0, i64 8, i1 false)
  store i64 -3750763034362895579, ptr %aggregate.literal1, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %first_hasher_owner, ptr align 8 %aggregate.literal1, i64 8, i1 false)
  call void @llvm.memset.p0.i64(ptr align 8 %aggregate.literal1, i8 0, i64 8, i1 false)
  store i64 -3750763034362895579, ptr %aggregate.literal1, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %second_hasher_owner, ptr align 8 %aggregate.literal1, i64 8, i1 false)
  %12 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %13 = icmp ne i8 %12, 0
  br i1 %13, label %poison.take2, label %poison.cont3

poison.take2:                                     ; preds = %poison.cont
  %14 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %14, align 1
  %15 = getelementptr i8, ptr %14, i64 4
  store i32 10, ptr %15, align 4
  %16 = load ptr, ptr %__panic, align 8
  %17 = getelementptr i8, ptr %16, i64 4
  %18 = load i32, ptr %17, align 4
  %19 = trunc i32 %18 to i8
  ret i8 %19

poison.cont3:                                     ; preds = %poison.cont
  %20 = load { i64, [0 x i64] }, ptr %first_hasher_owner, align 4
  store { ptr, ptr } zeroinitializer, ptr %byref_arg4, align 8
  call void @EmitLlLibrary_x3a_x3aEmitLlHashLawValue_x3a_x3ahash(ptr noundef nonnull align 4 dereferenceable(4) %left, ptr noundef nonnull align 8 dereferenceable(16) %byref_arg4, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %21 = load ptr, ptr %__panic, align 8
  %22 = load i8, ptr %21, align 1
  %23 = icmp ne i8 %22, 0
  br i1 %23, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont3
  %24 = load ptr, ptr %__panic, align 8
  %25 = getelementptr i8, ptr %24, i64 0
  %26 = load i8, ptr %25, align 1
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 4
  %29 = load i32, ptr %28, align 4
  %30 = load ptr, ptr %__panic, align 8
  %31 = getelementptr i8, ptr %30, i64 4
  %32 = load i32, ptr %31, align 4
  %33 = trunc i32 %32 to i8
  ret i8 %33

panic.cont:                                       ; preds = %poison.cont3
  %34 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %35 = icmp ne i8 %34, 0
  br i1 %35, label %poison.take5, label %poison.cont6

poison.take5:                                     ; preds = %panic.cont
  %36 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %36, align 1
  %37 = getelementptr i8, ptr %36, i64 4
  store i32 10, ptr %37, align 4
  %38 = load ptr, ptr %__panic, align 8
  %39 = getelementptr i8, ptr %38, i64 4
  %40 = load i32, ptr %39, align 4
  %41 = trunc i32 %40 to i8
  ret i8 %41

poison.cont6:                                     ; preds = %panic.cont
  %42 = load { i64, [0 x i64] }, ptr %second_hasher_owner, align 4
  store { ptr, ptr } zeroinitializer, ptr %byref_arg4, align 8
  call void @EmitLlLibrary_x3a_x3aEmitLlHashLawValue_x3a_x3ahash(ptr noundef nonnull align 4 dereferenceable(4) %right, ptr noundef nonnull align 8 dereferenceable(16) %byref_arg4, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %43 = load ptr, ptr %__panic, align 8
  %44 = load i8, ptr %43, align 1
  %45 = icmp ne i8 %44, 0
  br i1 %45, label %panic.take7, label %panic.cont8

panic.take7:                                      ; preds = %poison.cont6
  %46 = load ptr, ptr %__panic, align 8
  %47 = getelementptr i8, ptr %46, i64 0
  %48 = load i8, ptr %47, align 1
  %49 = load ptr, ptr %__panic, align 8
  %50 = getelementptr i8, ptr %49, i64 4
  %51 = load i32, ptr %50, align 4
  %52 = load ptr, ptr %__panic, align 8
  %53 = getelementptr i8, ptr %52, i64 4
  %54 = load i32, ptr %53, align 4
  %55 = trunc i32 %54 to i8
  ret i8 %55

panic.cont8:                                      ; preds = %poison.cont6
  %56 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %57 = icmp ne i8 %56, 0
  br i1 %57, label %poison.take9, label %poison.cont10

poison.take9:                                     ; preds = %panic.cont8
  %58 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %58, align 1
  %59 = getelementptr i8, ptr %58, i64 4
  store i32 10, ptr %59, align 4
  %60 = load ptr, ptr %__panic, align 8
  %61 = getelementptr i8, ptr %60, i64 4
  %62 = load i32, ptr %61, align 4
  %63 = trunc i32 %62 to i8
  ret i8 %63

poison.cont10:                                    ; preds = %panic.cont8
  %64 = call i8 @EmitLlLibrary_x3a_x3aEmitLlHashLawValue_x3a_x3aeq(ptr noundef nonnull align 4 dereferenceable(4) %left, ptr noundef nonnull align 4 dereferenceable(4) %right, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %65 = load ptr, ptr %__panic, align 8
  %66 = load i8, ptr %65, align 1
  %67 = icmp ne i8 %66, 0
  br i1 %67, label %panic.take11, label %panic.cont12

panic.take11:                                     ; preds = %poison.cont10
  %68 = load ptr, ptr %__panic, align 8
  %69 = getelementptr i8, ptr %68, i64 0
  %70 = load i8, ptr %69, align 1
  %71 = load ptr, ptr %__panic, align 8
  %72 = getelementptr i8, ptr %71, i64 4
  %73 = load i32, ptr %72, align 4
  %74 = load ptr, ptr %__panic, align 8
  %75 = getelementptr i8, ptr %74, i64 4
  %76 = load i32, ptr %75, align 4
  %77 = trunc i32 %76 to i8
  ret i8 %77

panic.cont12:                                     ; preds = %poison.cont10
  %78 = icmp ne i8 %64, 0
  %79 = icmp ne i8 %64, 0
  br i1 %79, label %if.then, label %if.else

if.then:                                          ; preds = %panic.cont12
  %80 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %81 = icmp ne i8 %80, 0
  br i1 %81, label %poison.take13, label %poison.cont14

if.else:                                          ; preds = %panic.cont12
  br label %if.merge

if.merge:                                         ; preds = %if.else, %panic.cont20
  %"EmitLlLibrary_x3a_x3aemitLlHashRequiresEqProbe$tmp$and_831" = phi i8 [ %127, %panic.cont20 ], [ 0, %if.else ]
  %82 = icmp ne i8 %"EmitLlLibrary_x3a_x3aemitLlHashRequiresEqProbe$tmp$and_831", 0
  %83 = zext i1 %82 to i8
  ret i8 %83

poison.take13:                                    ; preds = %if.then
  %84 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %84, align 1
  %85 = getelementptr i8, ptr %84, i64 4
  store i32 10, ptr %85, align 4
  %86 = load ptr, ptr %__panic, align 8
  %87 = getelementptr i8, ptr %86, i64 4
  %88 = load i32, ptr %87, align 4
  %89 = trunc i32 %88 to i8
  ret i8 %89

poison.cont14:                                    ; preds = %if.then
  %90 = call i64 @EmitLlLibrary_x3a_x3aEmitLlFnvHasher_x3a_x3afinish(ptr noundef nonnull align 8 dereferenceable(8) %first_hasher_owner, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %91 = load ptr, ptr %__panic, align 8
  %92 = load i8, ptr %91, align 1
  %93 = icmp ne i8 %92, 0
  br i1 %93, label %panic.take15, label %panic.cont16

panic.take15:                                     ; preds = %poison.cont14
  %94 = load ptr, ptr %__panic, align 8
  %95 = getelementptr i8, ptr %94, i64 0
  %96 = load i8, ptr %95, align 1
  %97 = load ptr, ptr %__panic, align 8
  %98 = getelementptr i8, ptr %97, i64 4
  %99 = load i32, ptr %98, align 4
  %100 = load ptr, ptr %__panic, align 8
  %101 = getelementptr i8, ptr %100, i64 4
  %102 = load i32, ptr %101, align 4
  %103 = trunc i32 %102 to i8
  ret i8 %103

panic.cont16:                                     ; preds = %poison.cont14
  %104 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %105 = icmp ne i8 %104, 0
  br i1 %105, label %poison.take17, label %poison.cont18

poison.take17:                                    ; preds = %panic.cont16
  %106 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %106, align 1
  %107 = getelementptr i8, ptr %106, i64 4
  store i32 10, ptr %107, align 4
  %108 = load ptr, ptr %__panic, align 8
  %109 = getelementptr i8, ptr %108, i64 4
  %110 = load i32, ptr %109, align 4
  %111 = trunc i32 %110 to i8
  ret i8 %111

poison.cont18:                                    ; preds = %panic.cont16
  %112 = call i64 @EmitLlLibrary_x3a_x3aEmitLlFnvHasher_x3a_x3afinish(ptr noundef nonnull align 8 dereferenceable(8) %second_hasher_owner, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %113 = load ptr, ptr %__panic, align 8
  %114 = load i8, ptr %113, align 1
  %115 = icmp ne i8 %114, 0
  br i1 %115, label %panic.take19, label %panic.cont20

panic.take19:                                     ; preds = %poison.cont18
  %116 = load ptr, ptr %__panic, align 8
  %117 = getelementptr i8, ptr %116, i64 0
  %118 = load i8, ptr %117, align 1
  %119 = load ptr, ptr %__panic, align 8
  %120 = getelementptr i8, ptr %119, i64 4
  %121 = load i32, ptr %120, align 4
  %122 = load ptr, ptr %__panic, align 8
  %123 = getelementptr i8, ptr %122, i64 4
  %124 = load i32, ptr %123, align 4
  %125 = trunc i32 %124 to i8
  ret i8 %125

panic.cont20:                                     ; preds = %poison.cont18
  %126 = icmp eq i64 %90, %112
  %127 = zext i1 %126 to i8
  br label %if.merge
}

define i8 @EmitLlLibrary_x3a_x3aemitLlFoundationalEqIntrinsicProbe(ptr noundef nonnull align 4 dereferenceable(4) %left, ptr noundef nonnull align 4 dereferenceable(4) %right, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %10 = load i32, ptr %left, align 4
  %11 = load i32, ptr %right, align 4
  %12 = icmp eq i32 %10, %11
  %13 = zext i1 %12 to i8
  ret i8 %13
}

define i32 @EmitLlLibrary_x3a_x3aemitLlFoundationalDiscreteIntrinsicProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %predecessor_value = alloca i32, align 4
  %"EmitLlLibrary_x3a_x3aemitLlFoundationalDiscreteIntrinsicProbe$tmp$if_case_clause_result_855" = alloca i32, align 4
  %predecessor16 = alloca {}, align 1
  %empty_value15 = alloca {}, align 1
  %"EmitLlLibrary_x3a_x3aemitLlFoundationalDiscreteIntrinsicProbe$tmp$if_case_clause_result_852" = alloca i32, align 4
  %predecessor12 = alloca i32, align 4
  %previous_value = alloca i32, align 4
  %successor_value = alloca i32, align 4
  %"EmitLlLibrary_x3a_x3aemitLlFoundationalDiscreteIntrinsicProbe$tmp$if_case_clause_result_848" = alloca i32, align 4
  %successor8 = alloca {}, align 1
  %empty_value = alloca {}, align 1
  %"EmitLlLibrary_x3a_x3aemitLlFoundationalDiscreteIntrinsicProbe$tmp$if_case_clause_result_845" = alloca i32, align 4
  %successor6 = alloca i32, align 4
  %next_value = alloca i32, align 4
  %predecessor5 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 4
  %0 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %predecessor = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 4
  %successor1 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 4
  %1 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %successor = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 4
  %2 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  ret i32 %8

poison.cont:                                      ; preds = %entry
  %9 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 0, ptr %10, align 4
  %11 = load i32, ptr %value, align 4
  %12 = icmp ne i32 %11, 2147483647
  %13 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %11, i32 1)
  %14 = extractvalue { i32, i1 } %13, 0
  %15 = extractvalue { i32, i1 } %13, 1
  %16 = freeze i32 %14
  %17 = freeze i1 %15
  %18 = xor i1 %17, true
  %19 = and i1 %12, %18
  store { i8, [3 x i8], [4 x i8], [0 x i32] } { i8 1, [3 x i8] zeroinitializer, [4 x i8] zeroinitializer, [0 x i32] zeroinitializer }, ptr %1, align 4
  %20 = getelementptr i8, ptr %1, i64 4
  store i32 %16, ptr %20, align 1
  %21 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %1, align 4
  br i1 %19, label %step.some, label %step.none

step.some:                                        ; preds = %poison.cont
  br label %step.merge

step.none:                                        ; preds = %poison.cont
  br label %step.merge

step.merge:                                       ; preds = %step.none, %step.some
  %22 = phi { i8, [3 x i8], [4 x i8], [0 x i32] } [ %21, %step.some ], [ zeroinitializer, %step.none ]
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %22, ptr %successor1, align 4
  %23 = load i32, ptr %value, align 4
  %24 = icmp ne i32 %23, -2147483648
  %25 = call { i32, i1 } @llvm.ssub.with.overflow.i32(i32 %23, i32 1)
  %26 = extractvalue { i32, i1 } %25, 0
  %27 = extractvalue { i32, i1 } %25, 1
  %28 = freeze i32 %26
  %29 = freeze i1 %27
  %30 = xor i1 %29, true
  %31 = and i1 %24, %30
  store { i8, [3 x i8], [4 x i8], [0 x i32] } { i8 1, [3 x i8] zeroinitializer, [4 x i8] zeroinitializer, [0 x i32] zeroinitializer }, ptr %0, align 4
  %32 = getelementptr i8, ptr %0, i64 4
  store i32 %28, ptr %32, align 1
  %33 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %0, align 4
  br i1 %31, label %step.some2, label %step.none3

step.some2:                                       ; preds = %step.merge
  br label %step.merge4

step.none3:                                       ; preds = %step.merge
  br label %step.merge4

step.merge4:                                      ; preds = %step.none3, %step.some2
  %34 = phi { i8, [3 x i8], [4 x i8], [0 x i32] } [ %33, %step.some2 ], [ zeroinitializer, %step.none3 ]
  store { i8, [3 x i8], [4 x i8], [0 x i32] } %34, ptr %predecessor5, align 4
  %35 = load i8, ptr %successor1, align 1
  %36 = icmp eq i8 %35, 1
  br i1 %36, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %ifcase.case7, %ifcase.case
  %ifcase.result = phi i32 [ %47, %ifcase.case ], [ %56, %ifcase.case7 ]
  store i32 %ifcase.result, ptr %successor_value, align 4
  %37 = load i8, ptr %predecessor5, align 1
  %38 = icmp eq i8 %37, 1
  br i1 %38, label %ifcase.case10, label %ifcase.next11

ifcase.case:                                      ; preds = %step.merge4
  %39 = getelementptr i8, ptr %successor1, i64 4
  %40 = getelementptr i8, ptr %39, i64 0
  %41 = load i32, ptr %40, align 1
  store i32 %41, ptr %next_value, align 4
  %42 = getelementptr i8, ptr %successor1, i64 4
  %43 = getelementptr i8, ptr %42, i64 0
  %44 = load i32, ptr %43, align 1
  store i32 %44, ptr %successor6, align 4
  %45 = load i32, ptr %next_value, align 4
  %46 = load i32, ptr %next_value, align 1
  store i32 %46, ptr %"EmitLlLibrary_x3a_x3aemitLlFoundationalDiscreteIntrinsicProbe$tmp$if_case_clause_result_845", align 4
  %47 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlFoundationalDiscreteIntrinsicProbe$tmp$if_case_clause_result_845", align 4
  br label %ifcase.merge

ifcase.next:                                      ; preds = %step.merge4
  %48 = load i8, ptr %successor1, align 1
  %49 = icmp eq i8 %48, 0
  br i1 %49, label %ifcase.case7, label %ifcase.unmatched

ifcase.case7:                                     ; preds = %ifcase.next
  %50 = getelementptr i8, ptr %successor1, i64 4
  %51 = getelementptr i8, ptr %50, i64 0
  %52 = getelementptr i8, ptr %successor1, i64 4
  %53 = getelementptr i8, ptr %52, i64 0
  %54 = load i32, ptr %value, align 4
  %55 = load i32, ptr %value, align 1
  store i32 %55, ptr %"EmitLlLibrary_x3a_x3aemitLlFoundationalDiscreteIntrinsicProbe$tmp$if_case_clause_result_848", align 4
  %56 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlFoundationalDiscreteIntrinsicProbe$tmp$if_case_clause_result_848", align 4
  br label %ifcase.merge

ifcase.unmatched:                                 ; preds = %ifcase.next
  %57 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %57, align 1
  %58 = getelementptr i8, ptr %57, i64 4
  store i32 18, ptr %58, align 4
  %59 = load ptr, ptr %__panic, align 8
  %60 = getelementptr i8, ptr %59, i64 4
  %61 = load i32, ptr %60, align 4
  ret i32 %61

ifcase.merge9:                                    ; preds = %ifcase.case13, %ifcase.case10
  %ifcase.result17 = phi i32 [ %70, %ifcase.case10 ], [ %79, %ifcase.case13 ]
  store i32 %ifcase.result17, ptr %predecessor_value, align 4
  br i1 true, label %check_ok, label %check_fail

ifcase.case10:                                    ; preds = %ifcase.merge
  %62 = getelementptr i8, ptr %predecessor5, i64 4
  %63 = getelementptr i8, ptr %62, i64 0
  %64 = load i32, ptr %63, align 1
  store i32 %64, ptr %previous_value, align 4
  %65 = getelementptr i8, ptr %predecessor5, i64 4
  %66 = getelementptr i8, ptr %65, i64 0
  %67 = load i32, ptr %66, align 1
  store i32 %67, ptr %predecessor12, align 4
  %68 = load i32, ptr %previous_value, align 4
  %69 = load i32, ptr %previous_value, align 1
  store i32 %69, ptr %"EmitLlLibrary_x3a_x3aemitLlFoundationalDiscreteIntrinsicProbe$tmp$if_case_clause_result_852", align 4
  %70 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlFoundationalDiscreteIntrinsicProbe$tmp$if_case_clause_result_852", align 4
  br label %ifcase.merge9

ifcase.next11:                                    ; preds = %ifcase.merge
  %71 = load i8, ptr %predecessor5, align 1
  %72 = icmp eq i8 %71, 0
  br i1 %72, label %ifcase.case13, label %ifcase.unmatched14

ifcase.case13:                                    ; preds = %ifcase.next11
  %73 = getelementptr i8, ptr %predecessor5, i64 4
  %74 = getelementptr i8, ptr %73, i64 0
  %75 = getelementptr i8, ptr %predecessor5, i64 4
  %76 = getelementptr i8, ptr %75, i64 0
  %77 = load i32, ptr %value, align 4
  %78 = load i32, ptr %value, align 1
  store i32 %78, ptr %"EmitLlLibrary_x3a_x3aemitLlFoundationalDiscreteIntrinsicProbe$tmp$if_case_clause_result_855", align 4
  %79 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlFoundationalDiscreteIntrinsicProbe$tmp$if_case_clause_result_855", align 4
  br label %ifcase.merge9

ifcase.unmatched14:                               ; preds = %ifcase.next11
  %80 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %80, align 1
  %81 = getelementptr i8, ptr %80, i64 4
  store i32 18, ptr %81, align 4
  %82 = load ptr, ptr %__panic, align 8
  %83 = getelementptr i8, ptr %82, i64 4
  %84 = load i32, ptr %83, align 4
  ret i32 %84

check_ok:                                         ; preds = %check_fail, %ifcase.merge9
  %85 = load ptr, ptr %__panic, align 8
  %86 = load i8, ptr %85, align 1
  %87 = icmp ne i8 %86, 0
  br i1 %87, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %ifcase.merge9
  %88 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %88, align 1
  %89 = getelementptr i8, ptr %88, i64 4
  store i32 4, ptr %89, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %90 = load ptr, ptr %__panic, align 8
  %91 = getelementptr i8, ptr %90, i64 0
  %92 = load i8, ptr %91, align 1
  %93 = load ptr, ptr %__panic, align 8
  %94 = getelementptr i8, ptr %93, i64 4
  %95 = load i32, ptr %94, align 4
  %96 = load ptr, ptr %__panic, align 8
  %97 = getelementptr i8, ptr %96, i64 4
  %98 = load i32, ptr %97, align 4
  ret i32 %98

panic.cont:                                       ; preds = %check_ok
  %99 = load i32, ptr %successor_value, align 4
  %100 = load i32, ptr %predecessor_value, align 4
  %101 = call { i32, i1 } @llvm.ssub.with.overflow.i32(i32 %99, i32 %100)
  %102 = extractvalue { i32, i1 } %101, 0
  %103 = extractvalue { i32, i1 } %101, 1
  %104 = freeze i32 %102
  br i1 %103, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  ret i32 %104

op_fail:                                          ; preds = %panic.cont
  %105 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %105, align 1
  %106 = getelementptr i8, ptr %105, i64 4
  store i32 4, ptr %106, align 4
  ret i32 0
}

define i32 @EmitLlLibrary_x3a_x3aemitLlFirstFieldByNameProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %aggregate.literal = alloca { i32, [0 x i32] }, align 4
  %owner = alloca { i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %9 = load i32, ptr %value, align 4
  store i32 %9, ptr %aggregate.literal, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %owner, ptr align 4 %aggregate.literal, i64 4, i1 false)
  %10 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  ret i32 %16

poison.cont2:                                     ; preds = %poison.cont
  %17 = call i32 @default_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlFirstFieldOwner_x3a_x3acl_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlFirstFieldSelection_x3a_x3aselectedFirstField(ptr noundef nonnull align 4 dereferenceable(4) %owner, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %18 = load ptr, ptr %__panic, align 8
  %19 = load i8, ptr %18, align 1
  %20 = icmp ne i8 %19, 0
  br i1 %20, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont2
  %21 = load ptr, ptr %__panic, align 8
  %22 = getelementptr i8, ptr %21, i64 0
  %23 = load i8, ptr %22, align 1
  %24 = load ptr, ptr %__panic, align 8
  %25 = getelementptr i8, ptr %24, i64 4
  %26 = load i32, ptr %25, align 4
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 4
  %29 = load i32, ptr %28, align 4
  ret i32 %29

panic.cont:                                       ; preds = %poison.cont2
  ret i32 %17
}

define i64 @EmitLlLibrary_x3a_x3aemitLlUnknownPointerAttrsProbe(ptr noundef nonnull align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  ret i64 8
}

define i64 @EmitLlLibrary_x3a_x3aemitLlValidPointerAttrsProbe(ptr noundef nonnull align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  ret i64 8
}

define i64 @EmitLlLibrary_x3a_x3aemitLlConstPointerArgAttrsProbe(ptr noundef nonnull readonly align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  ret i64 8
}

define i64 @EmitLlLibrary_x3a_x3aemitLlUniquePointerArgAttrsProbe(ptr noalias noundef nonnull align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  ret i64 8
}

define i64 @EmitLlLibrary_x3a_x3aemitLlPermissionAdmissibilityArgumentProbe(ptr noalias noundef nonnull align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %unique_size12 = alloca i64, align 8
  %unique_size = alloca i64, align 8
  %const_size5 = alloca i64, align 8
  %const_size = alloca i64, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  %10 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %17 = zext i32 %16 to i64
  ret i64 %17

poison.cont2:                                     ; preds = %poison.cont
  %18 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %25 = zext i32 %24 to i64
  ret i64 %25

poison.cont4:                                     ; preds = %poison.cont2
  %26 = call i64 @EmitLlLibrary_x3a_x3aemitLlConstPointerArgAttrsProbe(ptr noundef nonnull readonly align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %27 = load ptr, ptr %__panic, align 8
  %28 = load i8, ptr %27, align 1
  %29 = icmp ne i8 %28, 0
  br i1 %29, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  %30 = load ptr, ptr %__panic, align 8
  %31 = getelementptr i8, ptr %30, i64 4
  %32 = load i32, ptr %31, align 4
  %33 = zext i32 %32 to i64
  ret i64 %33

panic.cont:                                       ; preds = %poison.cont4
  store i64 %26, ptr %const_size5, align 4
  %34 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %41 = zext i32 %40 to i64
  ret i64 %41

poison.cont7:                                     ; preds = %panic.cont
  %42 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %49 = zext i32 %48 to i64
  ret i64 %49

poison.cont9:                                     ; preds = %poison.cont7
  %50 = call i64 @EmitLlLibrary_x3a_x3aemitLlUniquePointerArgAttrsProbe(ptr noalias noundef nonnull align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
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
  %63 = zext i32 %62 to i64
  ret i64 %63

panic.cont11:                                     ; preds = %poison.cont9
  store i64 %50, ptr %unique_size12, align 4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %panic.cont11
  %64 = load ptr, ptr %__panic, align 8
  %65 = load i8, ptr %64, align 1
  %66 = icmp ne i8 %65, 0
  br i1 %66, label %panic.take13, label %panic.cont14

check_fail:                                       ; preds = %panic.cont11
  %67 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %67, align 1
  %68 = getelementptr i8, ptr %67, i64 4
  store i32 4, ptr %68, align 4
  br label %check_ok

panic.take13:                                     ; preds = %check_ok
  %69 = load ptr, ptr %__panic, align 8
  %70 = getelementptr i8, ptr %69, i64 0
  %71 = load i8, ptr %70, align 1
  %72 = load ptr, ptr %__panic, align 8
  %73 = getelementptr i8, ptr %72, i64 4
  %74 = load i32, ptr %73, align 4
  %75 = load ptr, ptr %__panic, align 8
  %76 = getelementptr i8, ptr %75, i64 4
  %77 = load i32, ptr %76, align 4
  %78 = zext i32 %77 to i64
  ret i64 %78

panic.cont14:                                     ; preds = %check_ok
  %79 = load i64, ptr %const_size5, align 4
  %80 = load i64, ptr %unique_size12, align 4
  %81 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %79, i64 %80)
  %82 = extractvalue { i64, i1 } %81, 0
  %83 = extractvalue { i64, i1 } %81, 1
  %84 = freeze i64 %82
  br i1 %83, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont14
  ret i64 %84

op_fail:                                          ; preds = %panic.cont14
  %85 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %85, align 1
  %86 = getelementptr i8, ptr %85, i64 4
  store i32 4, ptr %86, align 4
  ret i64 0
}

define i64 @EmitLlLibrary_x3a_x3aemitLlRawPointerAttrsProbe(ptr noundef nonnull align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  ret i64 8
}

define i64 @uv_emit_ll_raw_pointer_foreign_attrs_probe(ptr %pointer) {
entry:
  %runtime_panic_code = alloca i32, align 4
  %__panic = alloca ptr, align 8
  %__uv_panic_record = alloca { i8, [3 x i8], i32, [0 x i32] }, align 8
  %pointer1 = alloca ptr, align 8
  store ptr %pointer, ptr %pointer1, align 8
  store { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, ptr %__uv_panic_record, align 4
  store ptr %__uv_panic_record, ptr %__panic, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  store i32 %6, ptr %runtime_panic_code, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %runtime_panic_code)
  unreachable

poison.cont:                                      ; preds = %entry
  ret i64 8
}

define i32 @EmitLlLibrary_x3a_x3aemitLlSelectedOverload_x3a_x3a_x24overload_x3a_x3a0(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %17 = load i32, ptr %value, align 4
  %18 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %17, i32 1)
  %19 = extractvalue { i32, i1 } %18, 0
  %20 = extractvalue { i32, i1 } %18, 1
  %21 = freeze i32 %19
  br i1 %20, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  ret i32 %21

op_fail:                                          ; preds = %panic.cont
  %22 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %22, align 1
  %23 = getelementptr i8, ptr %22, i64 4
  store i32 4, ptr %23, align 4
  ret i32 0
}

define i32 @EmitLlLibrary_x3a_x3aemitLlSelectedOverload_x3a_x3a_x24overload_x3a_x3a1(ptr noundef nonnull align 1 dereferenceable(1) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %10 = icmp ne i8 %9, 0
  %11 = load i8, ptr %value, align 1
  %12 = icmp ne i8 %11, 0
  br i1 %12, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  ret i32 13

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else
  ret i32 0
}

define i32 @EmitLlLibrary_x3a_x3aemitLlNoRuntimeOverloadSearchProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"EmitLlLibrary_x3a_x3aemitLlNoRuntimeOverloadSearchProbe$tmp$call_ref_tmp_925" = alloca i8, align 1
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %16 = call i32 @EmitLlLibrary_x3a_x3aemitLlSelectedOverload_x3a_x3a_x24overload_x3a_x3a0(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %17 = load ptr, ptr %__panic, align 8
  %18 = load i8, ptr %17, align 1
  %19 = icmp ne i8 %18, 0
  br i1 %19, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont2
  %20 = load ptr, ptr %__panic, align 8
  %21 = getelementptr i8, ptr %20, i64 4
  %22 = load i32, ptr %21, align 4
  ret i32 %22

panic.cont:                                       ; preds = %poison.cont2
  store i8 1, ptr %"EmitLlLibrary_x3a_x3aemitLlNoRuntimeOverloadSearchProbe$tmp$call_ref_tmp_925", align 1
  %23 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %24 = icmp ne i8 %23, 0
  br i1 %24, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %panic.cont
  %25 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %25, align 1
  %26 = getelementptr i8, ptr %25, i64 4
  store i32 10, ptr %26, align 4
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 4
  %29 = load i32, ptr %28, align 4
  ret i32 %29

poison.cont4:                                     ; preds = %panic.cont
  %30 = call i32 @EmitLlLibrary_x3a_x3aemitLlSelectedOverload_x3a_x3a_x24overload_x3a_x3a1(ptr noundef nonnull align 1 dereferenceable(1) %"EmitLlLibrary_x3a_x3aemitLlNoRuntimeOverloadSearchProbe$tmp$call_ref_tmp_925", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %31 = load ptr, ptr %__panic, align 8
  %32 = load i8, ptr %31, align 1
  %33 = icmp ne i8 %32, 0
  br i1 %33, label %panic.take5, label %panic.cont6

panic.take5:                                      ; preds = %poison.cont4
  %34 = load ptr, ptr %__panic, align 8
  %35 = getelementptr i8, ptr %34, i64 4
  %36 = load i32, ptr %35, align 4
  ret i32 %36

panic.cont6:                                      ; preds = %poison.cont4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %panic.cont6
  %37 = load ptr, ptr %__panic, align 8
  %38 = load i8, ptr %37, align 1
  %39 = icmp ne i8 %38, 0
  br i1 %39, label %panic.take7, label %panic.cont8

check_fail:                                       ; preds = %panic.cont6
  %40 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %40, align 1
  %41 = getelementptr i8, ptr %40, i64 4
  store i32 4, ptr %41, align 4
  br label %check_ok

panic.take7:                                      ; preds = %check_ok
  %42 = load ptr, ptr %__panic, align 8
  %43 = getelementptr i8, ptr %42, i64 4
  %44 = load i32, ptr %43, align 4
  ret i32 %44

panic.cont8:                                      ; preds = %check_ok
  %45 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %16, i32 %30)
  %46 = extractvalue { i32, i1 } %45, 0
  %47 = extractvalue { i32, i1 } %45, 1
  %48 = freeze i32 %46
  br i1 %47, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont8
  ret i32 %48

op_fail:                                          ; preds = %panic.cont8
  %49 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %49, align 1
  %50 = getelementptr i8, ptr %49, i64 4
  store i32 4, ptr %50, align 4
  ret i32 0
}

define i32 @EmitLlLibrary_x3a_x3aemitLlLoopInvariantRuntimeProbe(ptr noundef nonnull align 4 dereferenceable(4) %limit, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %index = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  store i32 0, ptr %index, align 4
  %9 = load i32, ptr %index, align 4
  %10 = icmp sge i32 %9, 0
  %11 = zext i1 %10 to i8
  %12 = icmp ne i8 %11, 0
  %13 = icmp ne i8 %11, 0
  br i1 %13, label %if.then, label %if.else

loop.end:                                         ; preds = %loop.cond
  %14 = load i32, ptr %index, align 4
  ret i32 %14

loop.cond:                                        ; preds = %if.merge6, %if.merge
  %15 = load i32, ptr %index, align 4
  %16 = load i32, ptr %limit, align 4
  %17 = icmp slt i32 %15, %16
  %18 = zext i1 %17 to i8
  %19 = icmp ne i8 %18, 0
  br i1 %19, label %loop.body, label %loop.end

loop.body:                                        ; preds = %loop.cond
  br i1 true, label %check_ok, label %check_fail

loop.invariant.backedge:                          ; preds = %if.merge3, %if.then1
  %20 = load i32, ptr %index, align 4
  %21 = icmp sge i32 %20, 0
  %22 = zext i1 %21 to i8
  %23 = icmp ne i8 %22, 0
  %24 = icmp ne i8 %22, 0
  br i1 %24, label %if.then4, label %if.else5

if.then:                                          ; preds = %poison.cont
  br label %if.merge

if.else:                                          ; preds = %poison.cont
  %25 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %25, align 1
  %26 = getelementptr i8, ptr %25, i64 4
  store i32 17, ptr %26, align 4
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 4
  %29 = load i32, ptr %28, align 4
  ret i32 %29

if.merge:                                         ; preds = %if.then
  br label %loop.cond

check_ok:                                         ; preds = %check_fail, %loop.body
  %30 = load ptr, ptr %__panic, align 8
  %31 = load i8, ptr %30, align 1
  %32 = icmp ne i8 %31, 0
  br i1 %32, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %loop.body
  %33 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %33, align 1
  %34 = getelementptr i8, ptr %33, i64 4
  store i32 4, ptr %34, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %35 = load ptr, ptr %__panic, align 8
  %36 = getelementptr i8, ptr %35, i64 4
  %37 = load i32, ptr %36, align 4
  ret i32 %37

panic.cont:                                       ; preds = %check_ok
  %38 = load i32, ptr %index, align 4
  %39 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %38, i32 1)
  %40 = extractvalue { i32, i1 } %39, 0
  %41 = extractvalue { i32, i1 } %39, 1
  %42 = freeze i32 %40
  br i1 %41, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  store i32 %42, ptr %index, align 4
  %43 = load i32, ptr %index, align 4
  %44 = icmp slt i32 %43, 2
  %45 = zext i1 %44 to i8
  %46 = icmp ne i8 %45, 0
  %47 = icmp ne i8 %45, 0
  br i1 %47, label %if.then1, label %if.else2

op_fail:                                          ; preds = %panic.cont
  %48 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %48, align 1
  %49 = getelementptr i8, ptr %48, i64 4
  store i32 4, ptr %49, align 4
  ret i32 0

if.then1:                                         ; preds = %op_ok
  br label %loop.invariant.backedge

if.else2:                                         ; preds = %op_ok
  br label %if.merge3

if.merge3:                                        ; preds = %if.else2
  br label %loop.invariant.backedge

if.then4:                                         ; preds = %loop.invariant.backedge
  br label %if.merge6

if.else5:                                         ; preds = %loop.invariant.backedge
  %50 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %50, align 1
  %51 = getelementptr i8, ptr %50, i64 4
  store i32 17, ptr %51, align 4
  %52 = load ptr, ptr %__panic, align 8
  %53 = getelementptr i8, ptr %52, i64 4
  %54 = load i32, ptr %53, align 4
  ret i32 %54

if.merge6:                                        ; preds = %if.then4
  br label %loop.cond
}

define i32 @EmitLlLibrary_x3a_x3aemitLlDynamicProcedureContractProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"EmitLlLibrary_x3a_x3aemitLlDynamicProcedureContractProbe$tmp$return_snapshot_956" = alloca i32, align 4
  %__uv_entry_capture_0 = alloca i32, align 4
  %"EmitLlLibrary_x3a_x3aemitLlDynamicProcedureContractProbe$tmp$contract_param_entry_953" = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %10 = icmp sge i32 %9, 0
  %11 = zext i1 %10 to i8
  %12 = icmp ne i8 %11, 0
  %13 = icmp ne i8 %11, 0
  br i1 %13, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  br label %if.merge

if.else:                                          ; preds = %poison.cont
  %14 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %14, align 1
  %15 = getelementptr i8, ptr %14, i64 4
  store i32 11, ptr %15, align 4
  %16 = load ptr, ptr %__panic, align 8
  %17 = getelementptr i8, ptr %16, i64 4
  %18 = load i32, ptr %17, align 4
  ret i32 %18

if.merge:                                         ; preds = %if.then
  %19 = load i32, ptr %value, align 1
  store i32 %19, ptr %"EmitLlLibrary_x3a_x3aemitLlDynamicProcedureContractProbe$tmp$contract_param_entry_953", align 4
  %20 = load i32, ptr %value, align 1
  store i32 %20, ptr %__uv_entry_capture_0, align 4
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %if.merge
  %21 = load ptr, ptr %__panic, align 8
  %22 = load i8, ptr %21, align 1
  %23 = icmp ne i8 %22, 0
  br i1 %23, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %if.merge
  %24 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %24, align 1
  %25 = getelementptr i8, ptr %24, i64 4
  store i32 4, ptr %25, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %26 = load ptr, ptr %__panic, align 8
  %27 = getelementptr i8, ptr %26, i64 4
  %28 = load i32, ptr %27, align 4
  ret i32 %28

panic.cont:                                       ; preds = %check_ok
  %29 = load i32, ptr %value, align 4
  %30 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %29, i32 1)
  %31 = extractvalue { i32, i1 } %30, 0
  %32 = extractvalue { i32, i1 } %30, 1
  %33 = freeze i32 %31
  br i1 %32, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  store i32 %33, ptr %"EmitLlLibrary_x3a_x3aemitLlDynamicProcedureContractProbe$tmp$return_snapshot_956", align 4
  br i1 true, label %check_ok1, label %check_fail2

op_fail:                                          ; preds = %panic.cont
  %34 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %34, align 1
  %35 = getelementptr i8, ptr %34, i64 4
  store i32 4, ptr %35, align 4
  ret i32 0

check_ok1:                                        ; preds = %check_fail2, %op_ok
  %36 = load ptr, ptr %__panic, align 8
  %37 = load i8, ptr %36, align 1
  %38 = icmp ne i8 %37, 0
  br i1 %38, label %panic.take3, label %panic.cont4

check_fail2:                                      ; preds = %op_ok
  %39 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %39, align 1
  %40 = getelementptr i8, ptr %39, i64 4
  store i32 4, ptr %40, align 4
  br label %check_ok1

panic.take3:                                      ; preds = %check_ok1
  %41 = load ptr, ptr %__panic, align 8
  %42 = getelementptr i8, ptr %41, i64 4
  %43 = load i32, ptr %42, align 4
  ret i32 %43

panic.cont4:                                      ; preds = %check_ok1
  %44 = load i32, ptr %__uv_entry_capture_0, align 4
  %45 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %44, i32 1)
  %46 = extractvalue { i32, i1 } %45, 0
  %47 = extractvalue { i32, i1 } %45, 1
  %48 = freeze i32 %46
  br i1 %47, label %op_fail6, label %op_ok5

op_ok5:                                           ; preds = %panic.cont4
  %49 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlDynamicProcedureContractProbe$tmp$return_snapshot_956", align 4
  %50 = icmp eq i32 %49, %48
  %51 = zext i1 %50 to i8
  %52 = icmp ne i8 %51, 0
  %53 = icmp ne i8 %51, 0
  br i1 %53, label %if.then7, label %if.else8

op_fail6:                                         ; preds = %panic.cont4
  %54 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %54, align 1
  %55 = getelementptr i8, ptr %54, i64 4
  store i32 4, ptr %55, align 4
  ret i32 0

if.then7:                                         ; preds = %op_ok5
  br label %if.merge9

if.else8:                                         ; preds = %op_ok5
  %56 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %56, align 1
  %57 = getelementptr i8, ptr %56, i64 4
  store i32 12, ptr %57, align 4
  %58 = load ptr, ptr %__panic, align 8
  %59 = getelementptr i8, ptr %58, i64 4
  %60 = load i32, ptr %59, align 4
  ret i32 %60

if.merge9:                                        ; preds = %if.then7
  %61 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlDynamicProcedureContractProbe$tmp$return_snapshot_956", align 4
  ret i32 %61
}

declare i32 @emitLlDynamicForeignContract(i32)

declare ptr @emitLlNullableForeignContract(i32)

declare i32 @emitLlStaticForeignContract(i32)

declare i32 @emitLlUnwindAbortImport(i32)

declare i32 @emitLlUnwindCatchImport(i32)

define i32 @EmitLlLibrary_x3a_x3aemitLlDynamicForeignContractProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) personality ptr @__gxx_personality_v0 {
entry:
  %ffi_unwind_panic_code = alloca i32, align 4
  %"EmitLlLibrary_x3a_x3aemitLlDynamicForeignContractProbe$tmp$call_move_tmp_962" = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  store i32 %9, ptr %"EmitLlLibrary_x3a_x3aemitLlDynamicForeignContractProbe$tmp$call_move_tmp_962", align 4
  %10 = icmp sge ptr %"EmitLlLibrary_x3a_x3aemitLlDynamicForeignContractProbe$tmp$call_move_tmp_962", null
  %11 = zext i1 %10 to i8
  %12 = icmp ne i8 %11, 0
  %13 = xor i1 %12, true
  %14 = zext i1 %13 to i8
  %15 = icmp ne i8 %14, 0
  %16 = icmp ne i8 %14, 0
  br i1 %16, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont
  %17 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %17, align 1
  %18 = getelementptr i8, ptr %17, i64 4
  store i32 14, ptr %18, align 4
  %19 = load ptr, ptr %__panic, align 8
  %20 = getelementptr i8, ptr %19, i64 4
  %21 = load i32, ptr %20, align 4
  ret i32 %21

if.else:                                          ; preds = %poison.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else
  %22 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlDynamicForeignContractProbe$tmp$call_move_tmp_962", align 4
  %23 = invoke i32 @emitLlDynamicForeignContract(i32 %22)
          to label %ffi.invoke.cont unwind label %ffi.invoke.unwind

ffi.invoke.cont:                                  ; preds = %if.merge
  %24 = icmp sge i32 %23, 0
  %25 = zext i1 %24 to i8
  %26 = icmp ne i8 %25, 0
  %27 = xor i1 %26, true
  %28 = zext i1 %27 to i8
  %29 = icmp ne i8 %28, 0
  %30 = zext i1 %29 to i8
  %31 = icmp ne i8 %30, 0
  %32 = and i1 true, %31
  %33 = zext i1 %32 to i8
  %34 = icmp ne i8 %33, 0
  %35 = icmp ne i8 %33, 0
  br i1 %35, label %if.then1, label %if.else2

ffi.invoke.unwind:                                ; preds = %if.merge
  %ffi_lpad = landingpad { ptr, i32 }
          cleanup
          catch ptr null
  store i32 255, ptr %ffi_unwind_panic_code, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %ffi_unwind_panic_code)
  unreachable

if.then1:                                         ; preds = %ffi.invoke.cont
  %36 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %36, align 1
  %37 = getelementptr i8, ptr %36, i64 4
  store i32 15, ptr %37, align 4
  %38 = load ptr, ptr %__panic, align 8
  %39 = getelementptr i8, ptr %38, i64 4
  %40 = load i32, ptr %39, align 4
  ret i32 %40

if.else2:                                         ; preds = %ffi.invoke.cont
  br label %if.merge3

if.merge3:                                        ; preds = %if.else2
  ret i32 %23
}

define ptr @EmitLlLibrary_x3a_x3aemitLlNullableForeignContractProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) personality ptr @__gxx_personality_v0 {
entry:
  %ffi_unwind_panic_code = alloca i32, align 4
  %"EmitLlLibrary_x3a_x3aemitLlNullableForeignContractProbe$tmp$call_move_tmp_976" = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %6 = load i32, ptr %value, align 1
  store i32 %6, ptr %"EmitLlLibrary_x3a_x3aemitLlNullableForeignContractProbe$tmp$call_move_tmp_976", align 4
  %7 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlNullableForeignContractProbe$tmp$call_move_tmp_976", align 4
  %8 = invoke ptr @emitLlNullableForeignContract(i32 %7)
          to label %ffi.invoke.cont unwind label %ffi.invoke.unwind

ffi.invoke.cont:                                  ; preds = %poison.cont
  %9 = icmp eq ptr %8, null
  %10 = zext i1 %9 to i8
  %11 = icmp eq ptr %8, null
  %12 = zext i1 %11 to i8
  %13 = icmp ne i8 %12, 0
  %14 = xor i1 %13, true
  %15 = zext i1 %14 to i8
  %16 = icmp ne i8 %10, 0
  %17 = zext i1 %16 to i8
  %18 = icmp ne i8 %15, 0
  %19 = zext i1 %18 to i8
  %20 = icmp ne i8 %19, 0
  %21 = icmp ne i8 %17, 0
  %22 = and i1 %21, %20
  %23 = zext i1 %22 to i8
  %24 = icmp ne i8 %23, 0
  %25 = icmp ne i8 %23, 0
  br i1 %25, label %if.then, label %if.else

ffi.invoke.unwind:                                ; preds = %poison.cont
  %ffi_lpad = landingpad { ptr, i32 }
          cleanup
          catch ptr null
  store i32 255, ptr %ffi_unwind_panic_code, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %ffi_unwind_panic_code)
  unreachable

if.then:                                          ; preds = %ffi.invoke.cont
  %26 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %26, align 1
  %27 = getelementptr i8, ptr %26, i64 4
  store i32 15, ptr %27, align 4
  ret ptr null

if.else:                                          ; preds = %ffi.invoke.cont
  br label %if.merge

if.merge:                                         ; preds = %if.else
  ret ptr %8
}

define i32 @EmitLlLibrary_x3a_x3aemitLlStaticForeignContractProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic) personality ptr @__gxx_personality_v0 {
entry:
  %refined = alloca i32, align 4
  %result1 = alloca i32, align 4
  %ffi_unwind_panic_code = alloca i32, align 4
  %"EmitLlLibrary_x3a_x3aemitLlStaticForeignContractProbe$tmp$call_move_tmp_988" = alloca i32, align 4
  %result = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  store i32 1, ptr %"EmitLlLibrary_x3a_x3aemitLlStaticForeignContractProbe$tmp$call_move_tmp_988", align 4
  %9 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlStaticForeignContractProbe$tmp$call_move_tmp_988", align 4
  %10 = invoke i32 @emitLlStaticForeignContract(i32 %9)
          to label %ffi.invoke.cont unwind label %ffi.invoke.unwind

ffi.invoke.cont:                                  ; preds = %poison.cont
  store i32 %10, ptr %result1, align 4
  %11 = load i32, ptr %result1, align 1
  store i32 %11, ptr %refined, align 4
  %12 = load i32, ptr %refined, align 4
  ret i32 %12

ffi.invoke.unwind:                                ; preds = %poison.cont
  %ffi_lpad = landingpad { ptr, i32 }
          cleanup
          catch ptr null
  store i32 255, ptr %ffi_unwind_panic_code, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %ffi_unwind_panic_code)
  unreachable
}

define i32 @EmitLlLibrary_x3a_x3aemitLlUnwindAbortImportProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) personality ptr @__gxx_personality_v0 {
entry:
  %ffi_unwind_panic_code = alloca i32, align 4
  %"EmitLlLibrary_x3a_x3aemitLlUnwindAbortImportProbe$tmp$call_move_tmp_998" = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  store i32 %9, ptr %"EmitLlLibrary_x3a_x3aemitLlUnwindAbortImportProbe$tmp$call_move_tmp_998", align 4
  %10 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlUnwindAbortImportProbe$tmp$call_move_tmp_998", align 4
  %11 = invoke i32 @emitLlUnwindAbortImport(i32 %10)
          to label %ffi.invoke.cont unwind label %ffi.invoke.unwind

ffi.invoke.cont:                                  ; preds = %poison.cont
  ret i32 %11

ffi.invoke.unwind:                                ; preds = %poison.cont
  %ffi_lpad = landingpad { ptr, i32 }
          cleanup
          catch ptr null
  store i32 255, ptr %ffi_unwind_panic_code, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %ffi_unwind_panic_code)
  unreachable
}

define i32 @EmitLlLibrary_x3a_x3aemitLlUnwindCatchImportProbe(ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) personality ptr @__gxx_personality_v0 {
entry:
  %"EmitLlLibrary_x3a_x3aemitLlUnwindCatchImportProbe$tmp$call_move_tmp_1002" = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  store i32 %9, ptr %"EmitLlLibrary_x3a_x3aemitLlUnwindCatchImportProbe$tmp$call_move_tmp_1002", align 4
  %10 = load i32, ptr %"EmitLlLibrary_x3a_x3aemitLlUnwindCatchImportProbe$tmp$call_move_tmp_1002", align 4
  %11 = invoke i32 @emitLlUnwindCatchImport(i32 %10)
          to label %ffi.invoke.cont unwind label %ffi.invoke.unwind

ffi.invoke.cont:                                  ; preds = %ffi.invoke.unwind, %poison.cont
  %ffi_invoke_result = phi i32 [ %11, %poison.cont ], [ 0, %ffi.invoke.unwind ]
  ret i32 %ffi_invoke_result

ffi.invoke.unwind:                                ; preds = %poison.cont
  %ffi_lpad = landingpad { ptr, i32 }
          cleanup
          catch ptr null
  %12 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %12, align 1
  %13 = getelementptr i8, ptr %12, i64 4
  store i32 255, ptr %13, align 4
  br label %ffi.invoke.cont
}

define i32 @uv_emit_ll_unwind_abort_export_probe(i64 %index) {
entry:
  %runtime_panic_code2 = alloca i32, align 4
  %aggregate.literal = alloca [1 x i32], align 4
  %values = alloca [1 x i32], align 4
  %runtime_panic_code = alloca i32, align 4
  %__panic = alloca ptr, align 8
  %__uv_panic_record = alloca { i8, [3 x i8], i32, [0 x i32] }, align 8
  %index1 = alloca i64, align 8
  store i64 %index, ptr %index1, align 4
  store { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, ptr %__uv_panic_record, align 4
  store ptr %__uv_panic_record, ptr %__panic, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  store i32 %6, ptr %runtime_panic_code, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %runtime_panic_code)
  unreachable

poison.cont:                                      ; preds = %entry
  %7 = getelementptr [1 x i32], ptr %aggregate.literal, i64 0, i64 0
  store i32 17, ptr %7, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %values, ptr align 4 %aggregate.literal, i64 4, i1 false)
  %8 = load i64, ptr %index1, align 4
  %9 = icmp ult i64 %8, 1
  br i1 %9, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %10 = load ptr, ptr %__panic, align 8
  %11 = load i8, ptr %10, align 1
  %12 = icmp ne i8 %11, 0
  br i1 %12, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %13 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %13, align 1
  %14 = getelementptr i8, ptr %13, i64 4
  store i32 6, ptr %14, align 4
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
  store i32 %23, ptr %runtime_panic_code2, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %runtime_panic_code2)
  unreachable

panic.cont:                                       ; preds = %check_ok
  %24 = load i64, ptr %index1, align 4
  %25 = getelementptr i32, ptr %values, i64 %24
  %26 = load i32, ptr %25, align 4
  ret i32 %26
}

define i32 @uv_emit_ll_unwind_catch_export_probe(i64 %index) {
entry:
  %aggregate.literal = alloca [1 x i32], align 4
  %values = alloca [1 x i32], align 4
  %__panic = alloca ptr, align 8
  %__uv_panic_record = alloca { i8, [3 x i8], i32, [0 x i32] }, align 8
  %index1 = alloca i64, align 8
  store i64 %index, ptr %index1, align 4
  store { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, ptr %__uv_panic_record, align 4
  store ptr %__uv_panic_record, ptr %__panic, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret i32 0

poison.cont:                                      ; preds = %entry
  %4 = getelementptr [1 x i32], ptr %aggregate.literal, i64 0, i64 0
  store i32 19, ptr %4, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %values, ptr align 4 %aggregate.literal, i64 4, i1 false)
  %5 = load i64, ptr %index1, align 4
  %6 = icmp ult i64 %5, 1
  br i1 %6, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %poison.cont
  %7 = load ptr, ptr %__panic, align 8
  %8 = load i8, ptr %7, align 1
  %9 = icmp ne i8 %8, 0
  br i1 %9, label %panic.take, label %panic.cont

check_fail:                                       ; preds = %poison.cont
  %10 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %10, align 1
  %11 = getelementptr i8, ptr %10, i64 4
  store i32 6, ptr %11, align 4
  br label %check_ok

panic.take:                                       ; preds = %check_ok
  %12 = load ptr, ptr %__panic, align 8
  %13 = getelementptr i8, ptr %12, i64 0
  %14 = load i8, ptr %13, align 1
  %15 = load ptr, ptr %__panic, align 8
  %16 = getelementptr i8, ptr %15, i64 4
  %17 = load i32, ptr %16, align 4
  ret i32 0

panic.cont:                                       ; preds = %check_ok
  %18 = load ptr, ptr %__panic, align 8
  %19 = load i8, ptr %18, align 1
  %20 = icmp ne i8 %19, 0
  br i1 %20, label %catch_export_panic, label %catch_export_ok

catch_export_panic:                               ; preds = %panic.cont
  %21 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %21, align 1
  %22 = getelementptr i8, ptr %21, i64 4
  store i32 0, ptr %22, align 4
  ret i32 0

catch_export_ok:                                  ; preds = %panic.cont
  %23 = load i64, ptr %index1, align 4
  %24 = getelementptr i32, ptr %values, i64 %23
  %25 = load i32, ptr %24, align 4
  ret i32 %25
}

define i64 @uv_emit_ll_foreign_byref_record_probe(ptr noundef nonnull byval({ i64, i64, i64, [0 x i64] }) align 8 dereferenceable(24) %payload) {
entry:
  %runtime_panic_code = alloca i32, align 4
  %__panic = alloca ptr, align 8
  %__uv_panic_record = alloca { i8, [3 x i8], i32, [0 x i32] }, align 8
  store { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, ptr %__uv_panic_record, align 4
  store ptr %__uv_panic_record, ptr %__panic, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  store i32 %6, ptr %runtime_panic_code, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %runtime_panic_code)
  unreachable

poison.cont:                                      ; preds = %entry
  %7 = getelementptr i8, ptr %payload, i64 0
  %8 = load i64, ptr %7, align 1
  ret i64 %8
}

define i32 @uv_emit_ll_dropping_by_value_probe(i32 %payload) {
entry:
  %runtime_panic_code = alloca i32, align 4
  %__panic = alloca ptr, align 8
  %__uv_panic_record = alloca { i8, [3 x i8], i32, [0 x i32] }, align 8
  %coerce_bits = alloca i32, align 4
  %payload1 = alloca { i32, [0 x i32] }, align 8
  store i32 %payload, ptr %coerce_bits, align 4
  %0 = load { i32, [0 x i32] }, ptr %coerce_bits, align 1
  store { i32, [0 x i32] } %0, ptr %payload1, align 4
  store { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, ptr %__uv_panic_record, align 4
  store ptr %__uv_panic_record, ptr %__panic, align 8
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  store i32 %7, ptr %runtime_panic_code, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %runtime_panic_code)
  unreachable

poison.cont:                                      ; preds = %entry
  ret i32 31
}

define i32 @uv_emit_ll_generic_ffi_box_probe(i32 %payload) {
entry:
  %runtime_panic_code = alloca i32, align 4
  %__panic = alloca ptr, align 8
  %__uv_panic_record = alloca { i8, [3 x i8], i32, [0 x i32] }, align 8
  %coerce_bits = alloca i32, align 4
  %payload1 = alloca { i32, [0 x i32] }, align 8
  store i32 %payload, ptr %coerce_bits, align 4
  %0 = load { i32, [0 x i32] }, ptr %coerce_bits, align 1
  store { i32, [0 x i32] } %0, ptr %payload1, align 4
  store { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, ptr %__uv_panic_record, align 4
  store ptr %__uv_panic_record, ptr %__panic, align 8
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  store i32 %7, ptr %runtime_panic_code, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr %runtime_panic_code)
  unreachable

poison.cont:                                      ; preds = %entry
  %8 = getelementptr i8, ptr %payload1, i64 0
  %9 = load i32, ptr %8, align 1
  ret i32 %9
}

define i32 @uv_emit_ll_raw_catch_return_probe(i32 %value) {
entry:
  %__panic = alloca ptr, align 8
  %__uv_panic_record = alloca { i8, [3 x i8], i32, [0 x i32] }, align 8
  %value1 = alloca i32, align 4
  store i32 %value, ptr %value1, align 4
  store { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, ptr %__uv_panic_record, align 4
  store ptr %__uv_panic_record, ptr %__panic, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret i32 0

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  %5 = load i8, ptr %4, align 1
  %6 = icmp ne i8 %5, 0
  br i1 %6, label %catch_export_panic, label %catch_export_ok

catch_export_panic:                               ; preds = %poison.cont
  %7 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %7, align 1
  %8 = getelementptr i8, ptr %7, i64 4
  store i32 0, ptr %8, align 4
  ret i32 0

catch_export_ok:                                  ; preds = %poison.cont
  %9 = load i32, ptr %value1, align 4
  ret i32 %9
}

define ptr @EmitLlLibrary_x3a_x3aemitLlNullPointerConstructorProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %coerce_bits = alloca i64, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  store i64 0, ptr %coerce_bits, align 8
  %6 = load ptr, ptr %coerce_bits, align 1
  ret ptr %6
}

define ptr @EmitLlLibrary_x3a_x3aemitLlExpiredPointerConstructorProbe(ptr noundef nonnull align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  ret ptr %6
}

define i32 @EmitLlLibrary_x3a_x3aemitLlSafePointerReadProbe(ptr noundef nonnull align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %20 = call i8 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5fis_x5factive(ptr %19)
  %21 = icmp ne i8 %20, 0
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
  store i32 9, ptr %26, align 4
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

define i32 @EmitLlLibrary_x3a_x3aemitLlSafePointerWriteProbe(ptr noundef nonnull align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %20 = call i8 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5fis_x5factive(ptr %19)
  %21 = icmp ne i8 %20, 0
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
  store i32 9, ptr %26, align 4
  br label %check_ok1

panic.take3:                                      ; preds = %check_ok1
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 4
  %29 = load i32, ptr %28, align 4
  ret i32 %29

panic.cont4:                                      ; preds = %check_ok1
  %30 = load ptr, ptr %pointer, align 8
  %31 = load i32, ptr %value, align 4
  store i32 %31, ptr %30, align 4
  %32 = load i32, ptr %value, align 4
  ret i32 %32
}

define i32 @EmitLlLibrary_x3a_x3aemitLlSafePointerNullReadProbe(ptr noundef nonnull align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %20 = call i8 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5fis_x5factive(ptr %19)
  %21 = icmp ne i8 %20, 0
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
  store i32 9, ptr %26, align 4
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

define i32 @EmitLlLibrary_x3a_x3aemitLlSafePointerNullWriteProbe(ptr noundef nonnull align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %20 = call i8 @ultraviolet_x3a_x3aruntime_x3a_x3aregion_x3a_x3aaddr_x5fis_x5factive(ptr %19)
  %21 = icmp ne i8 %20, 0
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
  store i32 9, ptr %26, align 4
  br label %check_ok1

panic.take3:                                      ; preds = %check_ok1
  %27 = load ptr, ptr %__panic, align 8
  %28 = getelementptr i8, ptr %27, i64 4
  %29 = load i32, ptr %28, align 4
  ret i32 %29

panic.cont4:                                      ; preds = %check_ok1
  %30 = load ptr, ptr %pointer, align 8
  %31 = load i32, ptr %value, align 4
  store i32 %31, ptr %30, align 4
  %32 = load i32, ptr %value, align 4
  ret i32 %32
}

define i32 @EmitLlLibrary_x3a_x3aemitLlRawPointerReadProbe(ptr noundef nonnull align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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

define i32 @EmitLlLibrary_x3a_x3aemitLlRawPointerWriteProbe(ptr noundef nonnull align 8 dereferenceable(8) %pointer, ptr noundef nonnull align 4 dereferenceable(4) %value, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %20 = load i32, ptr %value, align 4
  store i32 %20, ptr %19, align 4
  %21 = load i32, ptr %value, align 4
  ret i32 %21
}

define i64 @EmitLlLibrary_x3a_x3aemitLlConstFunctionArgAttrsProbe(ptr noundef nonnull readonly align 8 dereferenceable(8) %callback, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  ret i64 8
}

define void @EmitLlLibrary_x3a_x3aemitLlSRetPair(ptr noalias noundef nonnull sret({ { ptr, i64 }, { ptr, i64 }, [0 x i64] }) align 8 dereferenceable(32) %0, ptr noundef nonnull align 8 dereferenceable(16) %left, ptr noundef nonnull align 8 dereferenceable(16) %right, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %aggregate.literal = alloca { { ptr, i64 }, { ptr, i64 }, [0 x i64] }, align 8
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  call void @llvm.memset.p0.i64(ptr align 8 %aggregate.literal, i8 0, i64 32, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %aggregate.literal, ptr align 8 %left, i64 16, i1 false)
  %7 = getelementptr i8, ptr %aggregate.literal, i64 16
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %7, ptr align 8 %right, i64 16, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %0, ptr align 8 %aggregate.literal, i64 32, i1 false)
  ret void
}

define i64 @EmitLlLibrary_x3a_x3aemitLlSRetCallSiteProbe(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"EmitLlLibrary_x3a_x3aemitLlSRetCallSiteProbe$tmp$call_ref_tmp_1058" = alloca { ptr, i64 }, align 8
  %"EmitLlLibrary_x3a_x3aemitLlSRetCallSiteProbe$tmp$call_ref_tmp_1055" = alloca { ptr, i64 }, align 8
  %pair = alloca { { ptr, i64 }, { ptr, i64 }, [0 x i64] }, align 8
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %7 = zext i32 %6 to i64
  ret i64 %7

poison.cont:                                      ; preds = %entry
  %8 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 0, ptr %9, align 4
  %10 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %17 = zext i32 %16 to i64
  ret i64 %17

poison.cont2:                                     ; preds = %poison.cont
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f5239BDFD9C37AC77, i64 15 }, ptr %"EmitLlLibrary_x3a_x3aemitLlSRetCallSiteProbe$tmp$call_ref_tmp_1055", align 8
  store { ptr, i64 } { ptr @ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3astring_x5f98910A3CC091FDAD, i64 31 }, ptr %"EmitLlLibrary_x3a_x3aemitLlSRetCallSiteProbe$tmp$call_ref_tmp_1058", align 8
  %18 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %25 = zext i32 %24 to i64
  ret i64 %25

poison.cont4:                                     ; preds = %poison.cont2
  call void @EmitLlLibrary_x3a_x3aemitLlSRetPair(ptr noalias noundef nonnull sret({ { ptr, i64 }, { ptr, i64 }, [0 x i64] }) align 8 dereferenceable(32) %pair, ptr noundef nonnull align 8 dereferenceable(16) %"EmitLlLibrary_x3a_x3aemitLlSRetCallSiteProbe$tmp$call_ref_tmp_1055", ptr noundef nonnull align 8 dereferenceable(16) %"EmitLlLibrary_x3a_x3aemitLlSRetCallSiteProbe$tmp$call_ref_tmp_1058", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %26 = load ptr, ptr %__panic, align 8
  %27 = load i8, ptr %26, align 1
  %28 = icmp ne i8 %27, 0
  br i1 %28, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  %29 = load ptr, ptr %__panic, align 8
  %30 = getelementptr i8, ptr %29, i64 4
  %31 = load i32, ptr %30, align 4
  %32 = zext i32 %31 to i64
  ret i64 %32

panic.cont:                                       ; preds = %poison.cont4
  %33 = getelementptr i8, ptr %pair, i64 0
  %34 = getelementptr i8, ptr %pair, i64 0
  %view.prefix = load { ptr, i64 }, ptr %34, align 8
  %view.len = extractvalue { ptr, i64 } %view.prefix, 1
  %35 = getelementptr i8, ptr %pair, i64 16
  %36 = getelementptr i8, ptr %pair, i64 16
  %view.prefix5 = load { ptr, i64 }, ptr %36, align 8
  %view.len6 = extractvalue { ptr, i64 } %view.prefix5, 1
  br i1 true, label %check_ok, label %check_fail

check_ok:                                         ; preds = %check_fail, %panic.cont
  %37 = load ptr, ptr %__panic, align 8
  %38 = load i8, ptr %37, align 1
  %39 = icmp ne i8 %38, 0
  br i1 %39, label %panic.take7, label %panic.cont8

check_fail:                                       ; preds = %panic.cont
  %40 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %40, align 1
  %41 = getelementptr i8, ptr %40, i64 4
  store i32 4, ptr %41, align 4
  br label %check_ok

panic.take7:                                      ; preds = %check_ok
  %42 = load ptr, ptr %__panic, align 8
  %43 = getelementptr i8, ptr %42, i64 0
  %44 = load i8, ptr %43, align 1
  %45 = load ptr, ptr %__panic, align 8
  %46 = getelementptr i8, ptr %45, i64 4
  %47 = load i32, ptr %46, align 4
  %48 = load ptr, ptr %__panic, align 8
  %49 = getelementptr i8, ptr %48, i64 4
  %50 = load i32, ptr %49, align 4
  %51 = zext i32 %50 to i64
  ret i64 %51

panic.cont8:                                      ; preds = %check_ok
  %52 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %view.len, i64 %view.len6)
  %53 = extractvalue { i64, i1 } %52, 0
  %54 = extractvalue { i64, i1 } %52, 1
  %55 = freeze i64 %53
  br i1 %54, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont8
  ret i64 %55

op_fail:                                          ; preds = %panic.cont8
  %56 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %56, align 1
  %57 = getelementptr i8, ptr %56, i64 4
  store i32 4, ptr %57, align 4
  ret i64 0
}

define i8 @EmitLlLibrary_x3a_x3aemitLlRuntimeCreateWriteCarrierProbe(ptr noundef nonnull align 8 dereferenceable(16) %context, ptr noundef nonnull align 8 dereferenceable(16) %path, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"EmitLlLibrary_x3a_x3aemitLlRuntimeCreateWriteCarrierProbe$tmp$if_case_clause_result_1093" = alloca i8, align 1
  %"EmitLlLibrary_x3a_x3aemitLlRuntimeCreateWriteCarrierProbe$tmp$if_case_clause_result_1090" = alloca i8, align 1
  %closed2 = alloca {}, align 1
  %closed = alloca {}, align 1
  %0 = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %value = alloca { i64, [0 x i64] }, align 8
  %created1 = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %abi_return = alloca { i64, i64 }, align 8
  %created = alloca { i8, [7 x i8], [8 x i8], [0 x i64] }, align 8
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
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
  %11 = getelementptr i8, ptr %context, i64 0
  %12 = getelementptr i8, ptr %context, i64 0
  %13 = call { i64, i64 } @ultraviolet_x3a_x3aruntime_x3a_x3aio_x3a_x3acreate_x5fwrite(ptr noundef nonnull align 8 dereferenceable(16) %12, ptr noundef nonnull align 8 dereferenceable(16) %path)
  store { i64, i64 } %13, ptr %abi_return, align 8
  %14 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %abi_return, align 1
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %14, ptr %created1, align 4
  %15 = load i8, ptr %created1, align 1
  %16 = icmp eq i8 %15, 0
  %17 = getelementptr i8, ptr %created1, i64 8
  %18 = getelementptr i8, ptr %17, i64 0
  %19 = and i1 %16, true
  br i1 %19, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %ifcase.case3, %ifcase.case
  %ifcase.result = phi i8 [ %30, %ifcase.case ], [ %39, %ifcase.case3 ]
  %20 = icmp ne i8 %ifcase.result, 0
  %21 = zext i1 %20 to i8
  ret i8 %21

ifcase.case:                                      ; preds = %poison.cont
  %22 = getelementptr i8, ptr %created1, i64 8
  %23 = getelementptr i8, ptr %22, i64 0
  %24 = load { i8, [7 x i8], [8 x i8], [0 x i64] }, ptr %created1, align 4
  store { i8, [7 x i8], [8 x i8], [0 x i64] } %24, ptr %0, align 4
  %25 = getelementptr i8, ptr %0, i64 8
  %26 = getelementptr i8, ptr %25, i64 0
  %27 = load { i64, [0 x i64] }, ptr %26, align 1
  store { i64, [0 x i64] } %27, ptr %value, align 4
  call void @File_x3a_x3aWrite_x3a_x3aclose(ptr noundef nonnull align 8 dereferenceable(8) %value)
  store {} zeroinitializer, ptr %closed2, align 1
  store i8 1, ptr %"EmitLlLibrary_x3a_x3aemitLlRuntimeCreateWriteCarrierProbe$tmp$if_case_clause_result_1090", align 1
  %28 = load i8, ptr %"EmitLlLibrary_x3a_x3aemitLlRuntimeCreateWriteCarrierProbe$tmp$if_case_clause_result_1090", align 1
  %29 = icmp ne i8 %28, 0
  %30 = zext i1 %29 to i8
  br label %ifcase.merge

ifcase.next:                                      ; preds = %poison.cont
  %31 = load i8, ptr %created1, align 1
  %32 = icmp eq i8 %31, 1
  %33 = getelementptr i8, ptr %created1, i64 8
  %34 = getelementptr i8, ptr %33, i64 0
  %35 = load i8, ptr %34, align 1
  %36 = and i1 %32, true
  br i1 %36, label %ifcase.case3, label %ifcase.unmatched

ifcase.case3:                                     ; preds = %ifcase.next
  store i8 0, ptr %"EmitLlLibrary_x3a_x3aemitLlRuntimeCreateWriteCarrierProbe$tmp$if_case_clause_result_1093", align 1
  %37 = load i8, ptr %"EmitLlLibrary_x3a_x3aemitLlRuntimeCreateWriteCarrierProbe$tmp$if_case_clause_result_1093", align 1
  %38 = icmp ne i8 %37, 0
  %39 = zext i1 %38 to i8
  br label %ifcase.merge

ifcase.unmatched:                                 ; preds = %ifcase.next
  %40 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %40, align 1
  %41 = getelementptr i8, ptr %40, i64 4
  store i32 18, ptr %41, align 4
  %42 = load ptr, ptr %__panic, align 8
  %43 = getelementptr i8, ptr %42, i64 4
  %44 = load i32, ptr %43, align 4
  %45 = trunc i32 %44 to i8
  ret i8 %45
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aEmitLlLibrary(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %byref_arg3 = alloca i32, align 4
  %byref_arg = alloca { i32, i32, [0 x i32] }, align 4
  %aggregate.literal = alloca { i32, i32, [0 x i32] }, align 4
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  call void @llvm.memset.p0.i64(ptr align 4 %aggregate.literal, i8 0, i64 8, i1 false)
  store i32 67, ptr %aggregate.literal, align 4
  %2 = getelementptr i8, ptr %aggregate.literal, i64 4
  store i32 67, ptr %2, align 4
  call void @llvm.memmove.p0.p0.i64(ptr align 4 @EmitLlLibrary_x3a_x3aEMIT_x5fLL_x5fSTATIC_x5fCLEANUP_x5fPROBE, ptr align 4 %aggregate.literal, i64 8, i1 false)
  %3 = load ptr, ptr %__panic, align 8
  %4 = load i8, ptr %3, align 1
  %5 = icmp ne i8 %4, 0
  br i1 %5, label %init.panic.take, label %init.panic.cont

init.panic.take:                                  ; preds = %entry
  store i8 1, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %6 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %6, align 1
  %7 = getelementptr i8, ptr %6, i64 4
  store i32 10, ptr %7, align 4
  %8 = load ptr, ptr %__panic, align 8
  %9 = getelementptr i8, ptr %8, i64 0
  %10 = load i8, ptr %9, align 1
  %11 = load ptr, ptr %__panic, align 8
  %12 = getelementptr i8, ptr %11, i64 4
  %13 = load i32, ptr %12, align 4
  %14 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %14, align 1
  %15 = getelementptr i8, ptr %14, i64 4
  store i32 0, ptr %15, align 4
  %16 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %17 = icmp ne i8 %16, 0
  br i1 %17, label %poison.take, label %poison.cont

init.panic.cont:                                  ; preds = %entry
  %18 = load ptr, ptr %__panic, align 8
  %19 = load i8, ptr %18, align 1
  %20 = icmp ne i8 %19, 0
  br i1 %20, label %init.panic.take11, label %init.panic.cont12

poison.take:                                      ; preds = %init.panic.take
  %21 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %21, align 1
  %22 = getelementptr i8, ptr %21, i64 4
  store i32 10, ptr %22, align 4
  ret void

poison.cont:                                      ; preds = %init.panic.take
  %23 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %24 = icmp ne i8 %23, 0
  br i1 %24, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %25 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %25, align 1
  %26 = getelementptr i8, ptr %25, i64 4
  store i32 10, ptr %26, align 4
  ret void

poison.cont2:                                     ; preds = %poison.cont
  %27 = load { i32, i32, [0 x i32] }, ptr @EmitLlLibrary_x3a_x3aEMIT_x5fLL_x5fSTATIC_x5fCLEANUP_x5fPROBE, align 4
  store { i32, i32, [0 x i32] } %27, ptr %byref_arg, align 4
  call void @EmitLlLibrary_x3a_x3aEmitLlCleanupProbe_x3a_x3adrop(ptr noundef nonnull align 4 dereferenceable(8) %byref_arg, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %28 = load ptr, ptr %__panic, align 8
  %29 = getelementptr i8, ptr %28, i64 0
  %30 = load i8, ptr %29, align 1
  %31 = load ptr, ptr %__panic, align 8
  %32 = getelementptr i8, ptr %31, i64 4
  %33 = load i32, ptr %32, align 4
  %34 = icmp ne i8 %30, 0
  %35 = zext i1 %34 to i8
  %36 = icmp ne i8 %35, 0
  %37 = and i1 true, %36
  br i1 %37, label %if.then, label %if.else

if.then:                                          ; preds = %poison.cont2
  store i32 %33, ptr %byref_arg3, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg3)
  unreachable

if.else:                                          ; preds = %poison.cont2
  br label %if.merge

if.merge:                                         ; preds = %if.else
  %38 = icmp ne i8 %30, 0
  %39 = xor i1 %38, true
  %40 = and i1 true, %39
  br i1 %40, label %if.then4, label %if.else5

if.then4:                                         ; preds = %if.merge
  %41 = load ptr, ptr %__panic, align 8
  %42 = getelementptr i8, ptr %41, i64 0
  store i8 1, ptr %42, align 1
  %43 = load ptr, ptr %__panic, align 8
  %44 = getelementptr i8, ptr %43, i64 4
  store i32 %13, ptr %44, align 4
  br label %if.merge6

if.else5:                                         ; preds = %if.merge
  br label %if.merge6

if.merge6:                                        ; preds = %if.else5, %if.then4
  %if.result = phi i64 [ 0, %if.then4 ], [ 0, %if.else5 ]
  %45 = icmp ne i8 %30, 0
  %46 = zext i1 %45 to i8
  %47 = icmp ne i8 %46, 0
  %48 = or i1 true, %47
  %49 = icmp ne i8 %30, 0
  %50 = icmp ne i8 %30, 0
  br i1 %50, label %if.then7, label %if.else8

if.then7:                                         ; preds = %if.merge6
  br label %if.merge9

if.else8:                                         ; preds = %if.merge6
  br label %if.merge9

if.merge9:                                        ; preds = %if.else8, %if.then7
  %if.result10 = phi i32 [ %33, %if.then7 ], [ %13, %if.else8 ]
  ret void

init.panic.take11:                                ; preds = %init.panic.cont
  store i8 1, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %51 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %51, align 1
  %52 = getelementptr i8, ptr %51, i64 4
  store i32 10, ptr %52, align 4
  %53 = load ptr, ptr %__panic, align 8
  %54 = getelementptr i8, ptr %53, i64 0
  %55 = load i8, ptr %54, align 1
  %56 = load ptr, ptr %__panic, align 8
  %57 = getelementptr i8, ptr %56, i64 4
  %58 = load i32, ptr %57, align 4
  %59 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %59, align 1
  %60 = getelementptr i8, ptr %59, i64 4
  store i32 0, ptr %60, align 4
  %61 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %62 = icmp ne i8 %61, 0
  br i1 %62, label %poison.take13, label %poison.cont14

init.panic.cont12:                                ; preds = %init.panic.cont
  %63 = load ptr, ptr %__panic, align 8
  %64 = load i8, ptr %63, align 1
  %65 = icmp ne i8 %64, 0
  br i1 %65, label %init.panic.take28, label %init.panic.cont29

poison.take13:                                    ; preds = %init.panic.take11
  %66 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %66, align 1
  %67 = getelementptr i8, ptr %66, i64 4
  store i32 10, ptr %67, align 4
  ret void

poison.cont14:                                    ; preds = %init.panic.take11
  %68 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %69 = icmp ne i8 %68, 0
  br i1 %69, label %poison.take15, label %poison.cont16

poison.take15:                                    ; preds = %poison.cont14
  %70 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %70, align 1
  %71 = getelementptr i8, ptr %70, i64 4
  store i32 10, ptr %71, align 4
  ret void

poison.cont16:                                    ; preds = %poison.cont14
  %72 = load { i32, i32, [0 x i32] }, ptr @EmitLlLibrary_x3a_x3aEMIT_x5fLL_x5fSTATIC_x5fCLEANUP_x5fPROBE, align 4
  store { i32, i32, [0 x i32] } %72, ptr %byref_arg, align 4
  call void @EmitLlLibrary_x3a_x3aEmitLlCleanupProbe_x3a_x3adrop(ptr noundef nonnull align 4 dereferenceable(8) %byref_arg, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %73 = load ptr, ptr %__panic, align 8
  %74 = getelementptr i8, ptr %73, i64 0
  %75 = load i8, ptr %74, align 1
  %76 = load ptr, ptr %__panic, align 8
  %77 = getelementptr i8, ptr %76, i64 4
  %78 = load i32, ptr %77, align 4
  %79 = icmp ne i8 %75, 0
  %80 = zext i1 %79 to i8
  %81 = icmp ne i8 %80, 0
  %82 = and i1 true, %81
  br i1 %82, label %if.then17, label %if.else18

if.then17:                                        ; preds = %poison.cont16
  store i32 %78, ptr %byref_arg3, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg3)
  unreachable

if.else18:                                        ; preds = %poison.cont16
  br label %if.merge19

if.merge19:                                       ; preds = %if.else18
  %83 = icmp ne i8 %75, 0
  %84 = xor i1 %83, true
  %85 = and i1 true, %84
  br i1 %85, label %if.then20, label %if.else21

if.then20:                                        ; preds = %if.merge19
  %86 = load ptr, ptr %__panic, align 8
  %87 = getelementptr i8, ptr %86, i64 0
  store i8 1, ptr %87, align 1
  %88 = load ptr, ptr %__panic, align 8
  %89 = getelementptr i8, ptr %88, i64 4
  store i32 %58, ptr %89, align 4
  br label %if.merge22

if.else21:                                        ; preds = %if.merge19
  br label %if.merge22

if.merge22:                                       ; preds = %if.else21, %if.then20
  %if.result23 = phi i64 [ 0, %if.then20 ], [ 0, %if.else21 ]
  %90 = icmp ne i8 %75, 0
  %91 = zext i1 %90 to i8
  %92 = icmp ne i8 %91, 0
  %93 = or i1 true, %92
  %94 = icmp ne i8 %75, 0
  %95 = icmp ne i8 %75, 0
  br i1 %95, label %if.then24, label %if.else25

if.then24:                                        ; preds = %if.merge22
  br label %if.merge26

if.else25:                                        ; preds = %if.merge22
  br label %if.merge26

if.merge26:                                       ; preds = %if.else25, %if.then24
  %if.result27 = phi i32 [ %78, %if.then24 ], [ %58, %if.else25 ]
  ret void

init.panic.take28:                                ; preds = %init.panic.cont12
  store i8 1, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %96 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %96, align 1
  %97 = getelementptr i8, ptr %96, i64 4
  store i32 10, ptr %97, align 4
  %98 = load ptr, ptr %__panic, align 8
  %99 = getelementptr i8, ptr %98, i64 0
  %100 = load i8, ptr %99, align 1
  %101 = load ptr, ptr %__panic, align 8
  %102 = getelementptr i8, ptr %101, i64 4
  %103 = load i32, ptr %102, align 4
  %104 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %104, align 1
  %105 = getelementptr i8, ptr %104, i64 4
  store i32 0, ptr %105, align 4
  %106 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %107 = icmp ne i8 %106, 0
  br i1 %107, label %poison.take30, label %poison.cont31

init.panic.cont29:                                ; preds = %init.panic.cont12
  %108 = load ptr, ptr %__panic, align 8
  %109 = load i8, ptr %108, align 1
  %110 = icmp ne i8 %109, 0
  br i1 %110, label %init.panic.take45, label %init.panic.cont46

poison.take30:                                    ; preds = %init.panic.take28
  %111 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %111, align 1
  %112 = getelementptr i8, ptr %111, i64 4
  store i32 10, ptr %112, align 4
  ret void

poison.cont31:                                    ; preds = %init.panic.take28
  %113 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %114 = icmp ne i8 %113, 0
  br i1 %114, label %poison.take32, label %poison.cont33

poison.take32:                                    ; preds = %poison.cont31
  %115 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %115, align 1
  %116 = getelementptr i8, ptr %115, i64 4
  store i32 10, ptr %116, align 4
  ret void

poison.cont33:                                    ; preds = %poison.cont31
  %117 = load { i32, i32, [0 x i32] }, ptr @EmitLlLibrary_x3a_x3aEMIT_x5fLL_x5fSTATIC_x5fCLEANUP_x5fPROBE, align 4
  store { i32, i32, [0 x i32] } %117, ptr %byref_arg, align 4
  call void @EmitLlLibrary_x3a_x3aEmitLlCleanupProbe_x3a_x3adrop(ptr noundef nonnull align 4 dereferenceable(8) %byref_arg, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %118 = load ptr, ptr %__panic, align 8
  %119 = getelementptr i8, ptr %118, i64 0
  %120 = load i8, ptr %119, align 1
  %121 = load ptr, ptr %__panic, align 8
  %122 = getelementptr i8, ptr %121, i64 4
  %123 = load i32, ptr %122, align 4
  %124 = icmp ne i8 %120, 0
  %125 = zext i1 %124 to i8
  %126 = icmp ne i8 %125, 0
  %127 = and i1 true, %126
  br i1 %127, label %if.then34, label %if.else35

if.then34:                                        ; preds = %poison.cont33
  store i32 %123, ptr %byref_arg3, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg3)
  unreachable

if.else35:                                        ; preds = %poison.cont33
  br label %if.merge36

if.merge36:                                       ; preds = %if.else35
  %128 = icmp ne i8 %120, 0
  %129 = xor i1 %128, true
  %130 = and i1 true, %129
  br i1 %130, label %if.then37, label %if.else38

if.then37:                                        ; preds = %if.merge36
  %131 = load ptr, ptr %__panic, align 8
  %132 = getelementptr i8, ptr %131, i64 0
  store i8 1, ptr %132, align 1
  %133 = load ptr, ptr %__panic, align 8
  %134 = getelementptr i8, ptr %133, i64 4
  store i32 %103, ptr %134, align 4
  br label %if.merge39

if.else38:                                        ; preds = %if.merge36
  br label %if.merge39

if.merge39:                                       ; preds = %if.else38, %if.then37
  %if.result40 = phi i64 [ 0, %if.then37 ], [ 0, %if.else38 ]
  %135 = icmp ne i8 %120, 0
  %136 = zext i1 %135 to i8
  %137 = icmp ne i8 %136, 0
  %138 = or i1 true, %137
  %139 = icmp ne i8 %120, 0
  %140 = icmp ne i8 %120, 0
  br i1 %140, label %if.then41, label %if.else42

if.then41:                                        ; preds = %if.merge39
  br label %if.merge43

if.else42:                                        ; preds = %if.merge39
  br label %if.merge43

if.merge43:                                       ; preds = %if.else42, %if.then41
  %if.result44 = phi i32 [ %123, %if.then41 ], [ %103, %if.else42 ]
  ret void

init.panic.take45:                                ; preds = %init.panic.cont29
  store i8 1, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %141 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %141, align 1
  %142 = getelementptr i8, ptr %141, i64 4
  store i32 10, ptr %142, align 4
  %143 = load ptr, ptr %__panic, align 8
  %144 = getelementptr i8, ptr %143, i64 0
  %145 = load i8, ptr %144, align 1
  %146 = load ptr, ptr %__panic, align 8
  %147 = getelementptr i8, ptr %146, i64 4
  %148 = load i32, ptr %147, align 4
  %149 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %149, align 1
  %150 = getelementptr i8, ptr %149, i64 4
  store i32 0, ptr %150, align 4
  %151 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %152 = icmp ne i8 %151, 0
  br i1 %152, label %poison.take47, label %poison.cont48

init.panic.cont46:                                ; preds = %init.panic.cont29
  %153 = load ptr, ptr %__panic, align 8
  %154 = load i8, ptr %153, align 1
  %155 = icmp ne i8 %154, 0
  br i1 %155, label %init.panic.take62, label %init.panic.cont63

poison.take47:                                    ; preds = %init.panic.take45
  %156 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %156, align 1
  %157 = getelementptr i8, ptr %156, i64 4
  store i32 10, ptr %157, align 4
  ret void

poison.cont48:                                    ; preds = %init.panic.take45
  %158 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %159 = icmp ne i8 %158, 0
  br i1 %159, label %poison.take49, label %poison.cont50

poison.take49:                                    ; preds = %poison.cont48
  %160 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %160, align 1
  %161 = getelementptr i8, ptr %160, i64 4
  store i32 10, ptr %161, align 4
  ret void

poison.cont50:                                    ; preds = %poison.cont48
  %162 = load { i32, i32, [0 x i32] }, ptr @EmitLlLibrary_x3a_x3aEMIT_x5fLL_x5fSTATIC_x5fCLEANUP_x5fPROBE, align 4
  store { i32, i32, [0 x i32] } %162, ptr %byref_arg, align 4
  call void @EmitLlLibrary_x3a_x3aEmitLlCleanupProbe_x3a_x3adrop(ptr noundef nonnull align 4 dereferenceable(8) %byref_arg, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %163 = load ptr, ptr %__panic, align 8
  %164 = getelementptr i8, ptr %163, i64 0
  %165 = load i8, ptr %164, align 1
  %166 = load ptr, ptr %__panic, align 8
  %167 = getelementptr i8, ptr %166, i64 4
  %168 = load i32, ptr %167, align 4
  %169 = icmp ne i8 %165, 0
  %170 = zext i1 %169 to i8
  %171 = icmp ne i8 %170, 0
  %172 = and i1 true, %171
  br i1 %172, label %if.then51, label %if.else52

if.then51:                                        ; preds = %poison.cont50
  store i32 %168, ptr %byref_arg3, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg3)
  unreachable

if.else52:                                        ; preds = %poison.cont50
  br label %if.merge53

if.merge53:                                       ; preds = %if.else52
  %173 = icmp ne i8 %165, 0
  %174 = xor i1 %173, true
  %175 = and i1 true, %174
  br i1 %175, label %if.then54, label %if.else55

if.then54:                                        ; preds = %if.merge53
  %176 = load ptr, ptr %__panic, align 8
  %177 = getelementptr i8, ptr %176, i64 0
  store i8 1, ptr %177, align 1
  %178 = load ptr, ptr %__panic, align 8
  %179 = getelementptr i8, ptr %178, i64 4
  store i32 %148, ptr %179, align 4
  br label %if.merge56

if.else55:                                        ; preds = %if.merge53
  br label %if.merge56

if.merge56:                                       ; preds = %if.else55, %if.then54
  %if.result57 = phi i64 [ 0, %if.then54 ], [ 0, %if.else55 ]
  %180 = icmp ne i8 %165, 0
  %181 = zext i1 %180 to i8
  %182 = icmp ne i8 %181, 0
  %183 = or i1 true, %182
  %184 = icmp ne i8 %165, 0
  %185 = icmp ne i8 %165, 0
  br i1 %185, label %if.then58, label %if.else59

if.then58:                                        ; preds = %if.merge56
  br label %if.merge60

if.else59:                                        ; preds = %if.merge56
  br label %if.merge60

if.merge60:                                       ; preds = %if.else59, %if.then58
  %if.result61 = phi i32 [ %168, %if.then58 ], [ %148, %if.else59 ]
  ret void

init.panic.take62:                                ; preds = %init.panic.cont46
  store i8 1, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %186 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %186, align 1
  %187 = getelementptr i8, ptr %186, i64 4
  store i32 10, ptr %187, align 4
  %188 = load ptr, ptr %__panic, align 8
  %189 = getelementptr i8, ptr %188, i64 0
  %190 = load i8, ptr %189, align 1
  %191 = load ptr, ptr %__panic, align 8
  %192 = getelementptr i8, ptr %191, i64 4
  %193 = load i32, ptr %192, align 4
  %194 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %194, align 1
  %195 = getelementptr i8, ptr %194, i64 4
  store i32 0, ptr %195, align 4
  %196 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %197 = icmp ne i8 %196, 0
  br i1 %197, label %poison.take64, label %poison.cont65

init.panic.cont63:                                ; preds = %init.panic.cont46
  ret void

poison.take64:                                    ; preds = %init.panic.take62
  %198 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %198, align 1
  %199 = getelementptr i8, ptr %198, i64 4
  store i32 10, ptr %199, align 4
  ret void

poison.cont65:                                    ; preds = %init.panic.take62
  %200 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %201 = icmp ne i8 %200, 0
  br i1 %201, label %poison.take66, label %poison.cont67

poison.take66:                                    ; preds = %poison.cont65
  %202 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %202, align 1
  %203 = getelementptr i8, ptr %202, i64 4
  store i32 10, ptr %203, align 4
  ret void

poison.cont67:                                    ; preds = %poison.cont65
  %204 = load { i32, i32, [0 x i32] }, ptr @EmitLlLibrary_x3a_x3aEMIT_x5fLL_x5fSTATIC_x5fCLEANUP_x5fPROBE, align 4
  store { i32, i32, [0 x i32] } %204, ptr %byref_arg, align 4
  call void @EmitLlLibrary_x3a_x3aEmitLlCleanupProbe_x3a_x3adrop(ptr noundef nonnull align 4 dereferenceable(8) %byref_arg, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %205 = load ptr, ptr %__panic, align 8
  %206 = getelementptr i8, ptr %205, i64 0
  %207 = load i8, ptr %206, align 1
  %208 = load ptr, ptr %__panic, align 8
  %209 = getelementptr i8, ptr %208, i64 4
  %210 = load i32, ptr %209, align 4
  %211 = icmp ne i8 %207, 0
  %212 = zext i1 %211 to i8
  %213 = icmp ne i8 %212, 0
  %214 = and i1 true, %213
  br i1 %214, label %if.then68, label %if.else69

if.then68:                                        ; preds = %poison.cont67
  store i32 %210, ptr %byref_arg3, align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3apanic(ptr noundef nonnull align 4 dereferenceable(4) %byref_arg3)
  unreachable

if.else69:                                        ; preds = %poison.cont67
  br label %if.merge70

if.merge70:                                       ; preds = %if.else69
  %215 = icmp ne i8 %207, 0
  %216 = xor i1 %215, true
  %217 = and i1 true, %216
  br i1 %217, label %if.then71, label %if.else72

if.then71:                                        ; preds = %if.merge70
  %218 = load ptr, ptr %__panic, align 8
  %219 = getelementptr i8, ptr %218, i64 0
  store i8 1, ptr %219, align 1
  %220 = load ptr, ptr %__panic, align 8
  %221 = getelementptr i8, ptr %220, i64 4
  store i32 %193, ptr %221, align 4
  br label %if.merge73

if.else72:                                        ; preds = %if.merge70
  br label %if.merge73

if.merge73:                                       ; preds = %if.else72, %if.then71
  %if.result74 = phi i64 [ 0, %if.then71 ], [ 0, %if.else72 ]
  %222 = icmp ne i8 %207, 0
  %223 = zext i1 %222 to i8
  %224 = icmp ne i8 %223, 0
  %225 = or i1 true, %224
  %226 = icmp ne i8 %207, 0
  %227 = icmp ne i8 %207, 0
  br i1 %227, label %if.then75, label %if.else76

if.then75:                                        ; preds = %if.merge73
  br label %if.merge77

if.else76:                                        ; preds = %if.merge73
  br label %if.merge77

if.merge77:                                       ; preds = %if.else76, %if.then75
  %if.result78 = phi i32 [ %210, %if.then75 ], [ %193, %if.else76 ]
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aEmitLlLibrary(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %drop.arg = alloca ptr, align 8
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  store ptr @EmitLlLibrary_x3a_x3aEMIT_x5fLL_x5fSTATIC_x5fCLEANUP_x5fPROBE, ptr %drop.arg, align 8
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adrop_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlCleanupProbe(ptr noundef nonnull align 8 dereferenceable(8) %drop.arg, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adrop_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlCleanupProbe(ptr noundef nonnull align 8 dereferenceable(8) %data, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  %2 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aEmitLlLibrary, align 1
  %3 = icmp ne i8 %2, 0
  br i1 %3, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 10, ptr %5, align 4
  ret void

poison.cont:                                      ; preds = %entry
  %6 = load ptr, ptr %data, align 8
  call void @EmitLlLibrary_x3a_x3aEmitLlCleanupProbe_x3a_x3adrop(ptr noundef nonnull align 4 dereferenceable(8) %6, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adrop_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlDynamicDefaultOwner(ptr noundef nonnull align 8 dereferenceable(8) %data, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adrop_x3a_x3aEmitLlLibrary_x3a_x3aEmitLlDynamicOwner(ptr noundef nonnull align 8 dereferenceable(8) %data, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #2

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #2

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64) #2

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #3

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #4

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i8, i1 } @llvm.sadd.with.overflow.i8(i8, i8) #2

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.sadd.with.overflow.i64(i64, i64) #2

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.ssub.with.overflow.i64(i64, i64) #2

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.ssub.with.overflow.i32(i32, i32) #2

declare i32 @__gxx_personality_v0(...)

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memmove.p0.p0.i64(ptr writeonly captures(none), ptr readonly captures(none), i64, i1 immarg) #4

define void @__cx_lifecycle_init_EmitLlLibrary(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aEmitLlLibrary(ptr %0)
  ret void
}

define void @__cx_lifecycle_deinit_EmitLlLibrary(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aEmitLlLibrary(ptr %0)
  ret void
}

attributes #0 = { nounwind }
attributes #1 = { noreturn nounwind }
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #3 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #4 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
