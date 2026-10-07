; ==== Source/Alpha/ModuleDiscoveryOrdering_x3a_x3aAlpha.ll
; ModuleID = 'ModuleDiscoveryOrdering::Alpha'
source_filename = "ModuleDiscoveryOrdering::Alpha"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aModuleDiscoveryOrdering_x3a_x3aAlpha = global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fED202287F403D086 = internal constant [4 x i8] c"\03\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f8D1ACE904A398D17 = internal constant [4 x i8] c"\02\00\00\00", align 1

define i32 @ModuleDiscoveryOrdering_x3a_x3aAlpha_x3a_x3amoduleDiscoveryOrderingAlphaExtraValue(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aModuleDiscoveryOrdering_x3a_x3aAlpha, align 1
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
  ret i32 3
}

define i32 @ModuleDiscoveryOrdering_x3a_x3aAlpha_x3a_x3amoduleDiscoveryOrderingAlphaValue(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aModuleDiscoveryOrdering_x3a_x3aAlpha, align 1
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
  ret i32 2
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aModuleDiscoveryOrdering_x3a_x3aAlpha(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aModuleDiscoveryOrdering_x3a_x3aAlpha(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define void @__cx_lifecycle_init_ModuleDiscoveryOrdering_x3a_x3aAlpha(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aModuleDiscoveryOrdering_x3a_x3aAlpha(ptr %0)
  ret void
}

define void @__cx_lifecycle_deinit_ModuleDiscoveryOrdering_x3a_x3aAlpha(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aModuleDiscoveryOrdering_x3a_x3aAlpha(ptr %0)
  ret void
}
; ==== Source/Beta/ModuleDiscoveryOrdering_x3a_x3aBeta.ll
; ModuleID = 'ModuleDiscoveryOrdering::Beta'
source_filename = "ModuleDiscoveryOrdering::Beta"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aModuleDiscoveryOrdering_x3a_x3aBeta = global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fCD3AC65E44F721B1 = internal constant [4 x i8] c"\04\00\00\00", align 1

define i32 @ModuleDiscoveryOrdering_x3a_x3aBeta_x3a_x3amoduleDiscoveryOrderingBetaValue(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aModuleDiscoveryOrdering_x3a_x3aBeta, align 1
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
  ret i32 4
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aModuleDiscoveryOrdering_x3a_x3aBeta(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aModuleDiscoveryOrdering_x3a_x3aBeta(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define void @__cx_lifecycle_init_ModuleDiscoveryOrdering_x3a_x3aBeta(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aModuleDiscoveryOrdering_x3a_x3aBeta(ptr %0)
  ret void
}

define void @__cx_lifecycle_deinit_ModuleDiscoveryOrdering_x3a_x3aBeta(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aModuleDiscoveryOrdering_x3a_x3aBeta(ptr %0)
  ret void
}
; ==== Source/Beta/Nested/ModuleDiscoveryOrdering_x3a_x3aBeta_x3a_x3aNested.ll
; ModuleID = 'ModuleDiscoveryOrdering::Beta::Nested'
source_filename = "ModuleDiscoveryOrdering::Beta::Nested"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aModuleDiscoveryOrdering_x3a_x3aBeta_x3a_x3aNested = global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f2D401A55EEC16520 = internal constant [4 x i8] c"\05\00\00\00", align 1

define i32 @ModuleDiscoveryOrdering_x3a_x3aBeta_x3a_x3aNested_x3a_x3amoduleDiscoveryOrderingNestedValue(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aModuleDiscoveryOrdering_x3a_x3aBeta_x3a_x3aNested, align 1
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
  ret i32 5
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aModuleDiscoveryOrdering_x3a_x3aBeta_x3a_x3aNested(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aModuleDiscoveryOrdering_x3a_x3aBeta_x3a_x3aNested(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define void @__cx_lifecycle_init_ModuleDiscoveryOrdering_x3a_x3aBeta_x3a_x3aNested(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aModuleDiscoveryOrdering_x3a_x3aBeta_x3a_x3aNested(ptr %0)
  ret void
}

define void @__cx_lifecycle_deinit_ModuleDiscoveryOrdering_x3a_x3aBeta_x3a_x3aNested(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aModuleDiscoveryOrdering_x3a_x3aBeta_x3a_x3aNested(ptr %0)
  ret void
}
; ==== Source/ModuleDiscoveryOrdering.ll
; ModuleID = 'ModuleDiscoveryOrdering'
source_filename = "ModuleDiscoveryOrdering"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aModuleDiscoveryOrdering = global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fAD2ACA7747985764 = internal constant [4 x i8] c"\01\00\00\00", align 1

define i32 @ModuleDiscoveryOrdering_x3a_x3amoduleDiscoveryOrderingRootValue(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aModuleDiscoveryOrdering, align 1
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
  ret i32 1
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aModuleDiscoveryOrdering(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aModuleDiscoveryOrdering(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define void @__cx_lifecycle_init_ModuleDiscoveryOrdering(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aModuleDiscoveryOrdering(ptr %0)
  ret void
}

define void @__cx_lifecycle_deinit_ModuleDiscoveryOrdering(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aModuleDiscoveryOrdering(ptr %0)
  ret void
}
