; ModuleID = 'probe6.12965ebac08da1c4-cgu.0'
source_filename = "probe6.12965ebac08da1c4-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

@alloc_bdf610a73ad9acbd2297710a4c3a4902 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/8c74a5d27c644a0f7a22bb2fa8dd3ff8257bc220/library/core/src/num/mod.rs" }>, align 1
@alloc_3c41efcb289ecaab79c67cf9f4724a3a = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_bdf610a73ad9acbd2297710a4c3a4902, [12 x i8] c"K\00\00\00~\04\00\00\05\00\00\00" }>, align 4
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe6::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe65probe17h7bd8f3e994daf293E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hdf7a1af21ddc7cd9E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h28405f09b8547c36E(ptr align 1 @str.0, i32 25, ptr align 4 @alloc_3c41efcb289ecaab79c67cf9f4724a3a) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hdf7a1af21ddc7cd9E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare hidden i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core9panicking5panic17h28405f09b8547c36E(ptr align 1, i32, ptr align 4) unnamed_addr #2

attributes #0 = { nounwind "target-cpu"="generic" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn nounwind "target-cpu"="generic" }
attributes #3 = { noreturn nounwind }
