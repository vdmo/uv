; ==== Source/Geometry/Shapes_x3a_x3aGeometry.ll
; ModuleID = 'Shapes::Geometry'
source_filename = "Shapes::Geometry"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aShapes_x3a_x3aGeometry = hidden global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f8D1ACE904A398D17 = internal constant [4 x i8] c"\02\00\00\00", align 1

define i32 @Shapes_x3a_x3aGeometry_x3a_x3arectangleArea(ptr noundef nonnull align 4 dereferenceable(4) %width, ptr noundef nonnull align 4 dereferenceable(4) %height, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aShapes_x3a_x3aGeometry, align 1
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
  %17 = load i32, ptr %width, align 4
  %18 = load i32, ptr %height, align 4
  %19 = call { i32, i1 } @llvm.smul.with.overflow.i32(i32 %17, i32 %18)
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

define i32 @Shapes_x3a_x3aGeometry_x3a_x3arectanglePerimeter(ptr noundef nonnull align 4 dereferenceable(4) %width, ptr noundef nonnull align 4 dereferenceable(4) %height, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aShapes_x3a_x3aGeometry, align 1
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
  %17 = load i32, ptr %width, align 4
  %18 = load i32, ptr %height, align 4
  %19 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %17, i32 %18)
  %20 = extractvalue { i32, i1 } %19, 0
  %21 = extractvalue { i32, i1 } %19, 1
  %22 = freeze i32 %20
  br i1 %21, label %op_fail, label %op_ok

op_ok:                                            ; preds = %panic.cont
  br i1 true, label %check_ok1, label %check_fail2

op_fail:                                          ; preds = %panic.cont
  %23 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %23, align 1
  %24 = getelementptr i8, ptr %23, i64 4
  store i32 4, ptr %24, align 4
  ret i32 0

check_ok1:                                        ; preds = %check_fail2, %op_ok
  %25 = load ptr, ptr %__panic, align 8
  %26 = load i8, ptr %25, align 1
  %27 = icmp ne i8 %26, 0
  br i1 %27, label %panic.take3, label %panic.cont4

check_fail2:                                      ; preds = %op_ok
  %28 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %28, align 1
  %29 = getelementptr i8, ptr %28, i64 4
  store i32 4, ptr %29, align 4
  br label %check_ok1

panic.take3:                                      ; preds = %check_ok1
  %30 = load ptr, ptr %__panic, align 8
  %31 = getelementptr i8, ptr %30, i64 4
  %32 = load i32, ptr %31, align 4
  ret i32 %32

panic.cont4:                                      ; preds = %check_ok1
  %33 = call { i32, i1 } @llvm.smul.with.overflow.i32(i32 %22, i32 2)
  %34 = extractvalue { i32, i1 } %33, 0
  %35 = extractvalue { i32, i1 } %33, 1
  %36 = freeze i32 %34
  br i1 %35, label %op_fail6, label %op_ok5

op_ok5:                                           ; preds = %panic.cont4
  ret i32 %36

op_fail6:                                         ; preds = %panic.cont4
  %37 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %37, align 1
  %38 = getelementptr i8, ptr %37, i64 4
  store i32 4, ptr %38, align 4
  ret i32 0
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aShapes_x3a_x3aGeometry(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aShapes_x3a_x3aGeometry(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.smul.with.overflow.i32(i32, i32) #0

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #0

define hidden void @__cx_lifecycle_init_Shapes_x3a_x3aGeometry(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aShapes_x3a_x3aGeometry(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_Shapes_x3a_x3aGeometry(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aShapes_x3a_x3aGeometry(ptr %0)
  ret void
}

attributes #0 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
; ==== Source/Shapes.ll
; ModuleID = 'Shapes'
source_filename = "Shapes"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aShapes = hidden global i8 0
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5f4D25767F9DCE13F5 = internal constant [4 x i8] zeroinitializer, align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fED202287F403D086 = internal constant [4 x i8] c"\03\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3aliteral_x3a_x3aint_x5fCD3AC65E44F721B1 = internal constant [4 x i8] c"\04\00\00\00", align 1
@ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aShapes_x3a_x3aGeometry = external global i8
@__uv_library_attached = internal global i1 false
@__uv_image_panic_record = common hidden global { i8, [3 x i8], i32, [0 x i32] } zeroinitializer, align 4
@llvm.global_ctors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_ctor, ptr null }]
@llvm.global_dtors = appending hidden global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__uv_library_dtor, ptr null }]

define { i32, i32, [0 x i32] } @Shapes_x3a_x3amakeRectangle(ptr noundef nonnull align 4 dereferenceable(4) %width, ptr noundef nonnull align 4 dereferenceable(4) %height, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i32, i32, [0 x i32] }, align 8
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aShapes, align 1
  %2 = icmp ne i8 %1, 0
  br i1 %2, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %3 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %3, align 1
  %4 = getelementptr i8, ptr %3, i64 4
  store i32 10, ptr %4, align 4
  ret { i32, i32, [0 x i32] } zeroinitializer

poison.cont:                                      ; preds = %entry
  %5 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 0, ptr %6, align 4
  store { i32, i32, [0 x i32] } zeroinitializer, ptr %0, align 4
  %7 = load i32, ptr %width, align 4
  %8 = getelementptr i8, ptr %0, i64 0
  store i32 %7, ptr %8, align 1
  %9 = load i32, ptr %height, align 4
  %10 = getelementptr i8, ptr %0, i64 4
  store i32 %9, ptr %10, align 1
  %11 = load { i32, i32, [0 x i32] }, ptr %0, align 4
  ret { i32, i32, [0 x i32] } %11
}

define { i8, [3 x i8], [4 x i8], [0 x i32] } @Shapes_x3a_x3adescribeRectangle(ptr noundef nonnull align 4 dereferenceable(8) %shape, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = alloca { i8, [3 x i8], [4 x i8], [0 x i32] }, align 8
  %area5 = alloca i32, align 4
  %area = alloca i32, align 4
  %1 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aShapes, align 1
  %2 = icmp ne i8 %1, 0
  br i1 %2, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %3 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %3, align 1
  %4 = getelementptr i8, ptr %3, i64 4
  store i32 10, ptr %4, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

poison.cont:                                      ; preds = %entry
  %5 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %5, align 1
  %6 = getelementptr i8, ptr %5, i64 4
  store i32 0, ptr %6, align 4
  %7 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aShapes_x3a_x3aGeometry, align 1
  %8 = icmp ne i8 %7, 0
  br i1 %8, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %9 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %9, align 1
  %10 = getelementptr i8, ptr %9, i64 4
  store i32 10, ptr %10, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

poison.cont2:                                     ; preds = %poison.cont
  %11 = getelementptr i8, ptr %shape, i64 0
  %12 = getelementptr i8, ptr %shape, i64 4
  %13 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aShapes_x3a_x3aGeometry, align 1
  %14 = icmp ne i8 %13, 0
  br i1 %14, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %15 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %15, align 1
  %16 = getelementptr i8, ptr %15, i64 4
  store i32 10, ptr %16, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

poison.cont4:                                     ; preds = %poison.cont2
  %17 = getelementptr i8, ptr %shape, i64 0
  %18 = getelementptr i8, ptr %shape, i64 4
  %19 = call i32 @Shapes_x3a_x3aGeometry_x3a_x3arectangleArea(ptr noundef nonnull align 4 dereferenceable(4) %17, ptr noundef nonnull align 4 dereferenceable(4) %18, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %20 = load ptr, ptr %__panic, align 8
  %21 = load i8, ptr %20, align 1
  %22 = icmp ne i8 %21, 0
  br i1 %22, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

panic.cont:                                       ; preds = %poison.cont4
  store i32 %19, ptr %area5, align 4
  store { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer, ptr %0, align 4
  store i8 1, ptr %0, align 1
  %23 = getelementptr i8, ptr %0, i64 4
  %24 = load i32, ptr %area5, align 4
  %25 = getelementptr i8, ptr %23, i64 0
  store i32 %24, ptr %25, align 1
  %26 = load { i8, [3 x i8], [4 x i8], [0 x i32] }, ptr %0, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } %26
}

define i32 @Shapes_x3a_x3areportArea(ptr noundef nonnull align 4 dereferenceable(8) %report, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %"Shapes_x3a_x3areportArea$tmp$if_case_clause_result_18" = alloca i32, align 4
  %area = alloca i32, align 4
  %"Shapes_x3a_x3areportArea$tmp$if_case_clause_result_16" = alloca i32, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aShapes, align 1
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
  %9 = load i8, ptr %report, align 1
  %10 = icmp eq i8 %9, 0
  br i1 %10, label %ifcase.case, label %ifcase.next

ifcase.merge:                                     ; preds = %ifcase.case1, %ifcase.case
  %ifcase.result = phi i32 [ %11, %ifcase.case ], [ %23, %ifcase.case1 ]
  ret i32 %ifcase.result

ifcase.case:                                      ; preds = %poison.cont
  store i32 0, ptr %"Shapes_x3a_x3areportArea$tmp$if_case_clause_result_16", align 4
  %11 = load i32, ptr %"Shapes_x3a_x3areportArea$tmp$if_case_clause_result_16", align 4
  br label %ifcase.merge

ifcase.next:                                      ; preds = %poison.cont
  %12 = load i8, ptr %report, align 1
  %13 = icmp eq i8 %12, 1
  %14 = getelementptr i8, ptr %report, i64 4
  %15 = getelementptr i8, ptr %14, i64 0
  %16 = load i32, ptr %15, align 1
  %17 = and i1 %13, true
  br i1 %17, label %ifcase.case1, label %ifcase.unmatched

ifcase.case1:                                     ; preds = %ifcase.next
  %18 = getelementptr i8, ptr %report, i64 4
  %19 = getelementptr i8, ptr %18, i64 0
  %20 = load i32, ptr %19, align 1
  store i32 %20, ptr %area, align 4
  %21 = load i32, ptr %area, align 4
  %22 = load i32, ptr %area, align 1
  store i32 %22, ptr %"Shapes_x3a_x3areportArea$tmp$if_case_clause_result_18", align 4
  %23 = load i32, ptr %"Shapes_x3a_x3areportArea$tmp$if_case_clause_result_18", align 4
  br label %ifcase.merge

ifcase.unmatched:                                 ; preds = %ifcase.next
  %24 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %24, align 1
  %25 = getelementptr i8, ptr %24, i64 4
  store i32 18, ptr %25, align 4
  %26 = load ptr, ptr %__panic, align 8
  %27 = getelementptr i8, ptr %26, i64 4
  %28 = load i32, ptr %27, align 4
  ret i32 %28
}

define i32 @Shapes_x3a_x3arectangleEdges(ptr noundef nonnull align 4 dereferenceable(8) %shape, ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aShapes, align 1
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
  %9 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aShapes_x3a_x3aGeometry, align 1
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
  %16 = getelementptr i8, ptr %shape, i64 0
  %17 = getelementptr i8, ptr %shape, i64 4
  %18 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aShapes_x3a_x3aGeometry, align 1
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
  %25 = getelementptr i8, ptr %shape, i64 0
  %26 = getelementptr i8, ptr %shape, i64 4
  %27 = call i32 @Shapes_x3a_x3aGeometry_x3a_x3arectanglePerimeter(ptr noundef nonnull align 4 dereferenceable(4) %25, ptr noundef nonnull align 4 dereferenceable(4) %26, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %28 = load ptr, ptr %__panic, align 8
  %29 = load i8, ptr %28, align 1
  %30 = icmp ne i8 %29, 0
  br i1 %30, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  %31 = load ptr, ptr %__panic, align 8
  %32 = getelementptr i8, ptr %31, i64 4
  %33 = load i32, ptr %32, align 4
  ret i32 %33

panic.cont:                                       ; preds = %poison.cont4
  ret i32 %27
}

define { i8, [3 x i8], [4 x i8], [0 x i32] } @Shapes_x3a_x3asampleReport(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %shape5 = alloca { i32, i32, [0 x i32] }, align 4
  %"Shapes_x3a_x3asampleReport$tmp$call_ref_tmp_31" = alloca i32, align 4
  %"Shapes_x3a_x3asampleReport$tmp$call_ref_tmp_28" = alloca i32, align 4
  %shape = alloca { i32, i32, [0 x i32] }, align 4
  %0 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aShapes, align 1
  %1 = icmp ne i8 %0, 0
  br i1 %1, label %poison.take, label %poison.cont

poison.take:                                      ; preds = %entry
  %2 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %2, align 1
  %3 = getelementptr i8, ptr %2, i64 4
  store i32 10, ptr %3, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

poison.cont:                                      ; preds = %entry
  %4 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %4, align 1
  %5 = getelementptr i8, ptr %4, i64 4
  store i32 0, ptr %5, align 4
  %6 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aShapes, align 1
  %7 = icmp ne i8 %6, 0
  br i1 %7, label %poison.take1, label %poison.cont2

poison.take1:                                     ; preds = %poison.cont
  %8 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %8, align 1
  %9 = getelementptr i8, ptr %8, i64 4
  store i32 10, ptr %9, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

poison.cont2:                                     ; preds = %poison.cont
  store i32 3, ptr %"Shapes_x3a_x3asampleReport$tmp$call_ref_tmp_28", align 4
  store i32 4, ptr %"Shapes_x3a_x3asampleReport$tmp$call_ref_tmp_31", align 4
  %10 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aShapes, align 1
  %11 = icmp ne i8 %10, 0
  br i1 %11, label %poison.take3, label %poison.cont4

poison.take3:                                     ; preds = %poison.cont2
  %12 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %12, align 1
  %13 = getelementptr i8, ptr %12, i64 4
  store i32 10, ptr %13, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

poison.cont4:                                     ; preds = %poison.cont2
  %14 = call { i32, i32, [0 x i32] } @Shapes_x3a_x3amakeRectangle(ptr noundef nonnull align 4 dereferenceable(4) %"Shapes_x3a_x3asampleReport$tmp$call_ref_tmp_28", ptr noundef nonnull align 4 dereferenceable(4) %"Shapes_x3a_x3asampleReport$tmp$call_ref_tmp_31", ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %15 = load ptr, ptr %__panic, align 8
  %16 = load i8, ptr %15, align 1
  %17 = icmp ne i8 %16, 0
  br i1 %17, label %panic.take, label %panic.cont

panic.take:                                       ; preds = %poison.cont4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

panic.cont:                                       ; preds = %poison.cont4
  store { i32, i32, [0 x i32] } %14, ptr %shape5, align 4
  %18 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aShapes, align 1
  %19 = icmp ne i8 %18, 0
  br i1 %19, label %poison.take6, label %poison.cont7

poison.take6:                                     ; preds = %panic.cont
  %20 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %20, align 1
  %21 = getelementptr i8, ptr %20, i64 4
  store i32 10, ptr %21, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

poison.cont7:                                     ; preds = %panic.cont
  %22 = load i8, ptr @ultraviolet_x3a_x3aruntime_x3a_x3apoison_x3a_x3aShapes, align 1
  %23 = icmp ne i8 %22, 0
  br i1 %23, label %poison.take8, label %poison.cont9

poison.take8:                                     ; preds = %poison.cont7
  %24 = load ptr, ptr %__panic, align 8
  store i8 1, ptr %24, align 1
  %25 = getelementptr i8, ptr %24, i64 4
  store i32 10, ptr %25, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

poison.cont9:                                     ; preds = %poison.cont7
  %26 = call { i8, [3 x i8], [4 x i8], [0 x i32] } @Shapes_x3a_x3adescribeRectangle(ptr noundef nonnull align 4 dereferenceable(8) %shape5, ptr noundef nonnull align 8 dereferenceable(8) %__panic)
  %27 = load ptr, ptr %__panic, align 8
  %28 = load i8, ptr %27, align 1
  %29 = icmp ne i8 %28, 0
  br i1 %29, label %panic.take10, label %panic.cont11

panic.take10:                                     ; preds = %poison.cont9
  %30 = load ptr, ptr %__panic, align 8
  %31 = getelementptr i8, ptr %30, i64 0
  %32 = load i8, ptr %31, align 1
  %33 = load ptr, ptr %__panic, align 8
  %34 = getelementptr i8, ptr %33, i64 4
  %35 = load i32, ptr %34, align 4
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } zeroinitializer

panic.cont11:                                     ; preds = %poison.cont9
  ret { i8, [3 x i8], [4 x i8], [0 x i32] } %26
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aShapes(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

define internal void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aShapes(ptr noundef nonnull align 8 dereferenceable(8) %__panic) {
entry:
  %0 = load ptr, ptr %__panic, align 8
  store i8 0, ptr %0, align 1
  %1 = getelementptr i8, ptr %0, i64 4
  store i32 0, ptr %1, align 4
  ret void
}

declare i32 @Shapes_x3a_x3aGeometry_x3a_x3arectangleArea(ptr, ptr, ptr)

declare i32 @Shapes_x3a_x3aGeometry_x3a_x3arectanglePerimeter(ptr, ptr, ptr)

define hidden void @__cx_lifecycle_init_Shapes(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aShapes(ptr %0)
  ret void
}

define hidden void @__cx_lifecycle_deinit_Shapes(ptr %0) {
entry:
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aShapes(ptr %0)
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
  call void @ultraviolet_x3a_x3aruntime_x3a_x3ainit_x3a_x3aShapes(ptr %dll_attach_panic_out)
  %4 = load i8, ptr @__uv_image_panic_record, align 1
  %5 = icmp ne i8 %4, 0
  br i1 %5, label %dll.attach.fail, label %dll.attach.cont

dll.attach.done:                                  ; preds = %dll.attach
  ret i32 1

dll.attach.cont:                                  ; preds = %dll.attach.work
  call void @__cx_lifecycle_init_Shapes_x3a_x3aGeometry(ptr %dll_attach_panic_out)
  %6 = load i8, ptr @__uv_image_panic_record, align 1
  %7 = icmp ne i8 %6, 0
  br i1 %7, label %dll.attach.fail2, label %dll.attach.cont1

dll.attach.fail:                                  ; preds = %dll.attach.work
  store i8 0, ptr @__uv_image_panic_record, align 1
  store i32 0, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  store i1 false, ptr @__uv_library_attached, align 1
  ret i32 0

dll.attach.cont1:                                 ; preds = %dll.attach.cont
  store i1 true, ptr @__uv_library_attached, align 1
  ret i32 1

dll.attach.fail2:                                 ; preds = %dll.attach.cont
  store i8 0, ptr @__uv_image_panic_record, align 1
  store i32 0, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aShapes(ptr %dll_attach_panic_out)
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
  call void @__cx_lifecycle_deinit_Shapes_x3a_x3aGeometry(ptr %dll_detach_panic_out)
  %8 = load i8, ptr @__uv_image_panic_record, align 1
  %9 = icmp ne i8 %8, 0
  br i1 %9, label %dll.detach.panic.capture, label %dll.detach.cont

dll.detach.done:                                  ; preds = %dll.detach
  ret i32 1

dll.detach.panic.capture:                         ; preds = %dll.detach.work
  %10 = load i1, ptr %dll_detach_panic_seen, align 1
  %11 = load i32, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  br i1 %10, label %dll.detach.panic.clear, label %dll.detach.panic.store

dll.detach.cont:                                  ; preds = %dll.detach.panic.clear, %dll.detach.work
  call void @ultraviolet_x3a_x3aruntime_x3a_x3adeinit_x3a_x3aShapes(ptr %dll_detach_panic_out)
  %12 = load i8, ptr @__uv_image_panic_record, align 1
  %13 = icmp ne i8 %12, 0
  br i1 %13, label %dll.detach.panic.capture3, label %dll.detach.cont4

dll.detach.panic.store:                           ; preds = %dll.detach.panic.capture
  store i1 true, ptr %dll_detach_panic_seen, align 1
  store i32 %11, ptr %dll_detach_panic_code, align 4
  br label %dll.detach.panic.clear

dll.detach.panic.clear:                           ; preds = %dll.detach.panic.store, %dll.detach.panic.capture
  store i8 0, ptr @__uv_image_panic_record, align 1
  store i32 0, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  br label %dll.detach.cont

dll.detach.panic.capture3:                        ; preds = %dll.detach.cont
  %14 = load i1, ptr %dll_detach_panic_seen, align 1
  %15 = load i32, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  br i1 %14, label %dll.detach.panic.clear6, label %dll.detach.panic.store5

dll.detach.cont4:                                 ; preds = %dll.detach.panic.clear6, %dll.detach.cont
  store i1 false, ptr @__uv_library_attached, align 1
  %16 = load i1, ptr %dll_detach_panic_seen, align 1
  br i1 %16, label %dll.detach.fail, label %dll.detach.success

dll.detach.panic.store5:                          ; preds = %dll.detach.panic.capture3
  store i1 true, ptr %dll_detach_panic_seen, align 1
  store i32 %15, ptr %dll_detach_panic_code, align 4
  br label %dll.detach.panic.clear6

dll.detach.panic.clear6:                          ; preds = %dll.detach.panic.store5, %dll.detach.panic.capture3
  store i8 0, ptr @__uv_image_panic_record, align 1
  store i32 0, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  br label %dll.detach.cont4

dll.detach.fail:                                  ; preds = %dll.detach.cont4
  %17 = load i32, ptr %dll_detach_panic_code, align 4
  store i8 1, ptr @__uv_image_panic_record, align 1
  store i32 %17, ptr getelementptr (i8, ptr @__uv_image_panic_record, i64 4), align 4
  ret i32 0

dll.detach.success:                               ; preds = %dll.detach.cont4
  ret i32 1
}

declare void @__cx_lifecycle_init_Shapes_x3a_x3aGeometry(ptr)

declare void @__cx_lifecycle_deinit_Shapes_x3a_x3aGeometry(ptr)

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
