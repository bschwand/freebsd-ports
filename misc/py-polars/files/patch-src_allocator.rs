- workaround for https://github.com/pola-rs/polars/issues/17034

--- src/allocator.rs.orig	2024-08-04 11:07:42 UTC
+++ src/allocator.rs
@@ -1,14 +1,3 @@
-#[cfg(all(
-    target_family = "unix",
-    not(allocator = "default"),
-    not(allocator = "mimalloc"),
-))]
-use jemallocator::Jemalloc;
-#[cfg(all(
-    not(debug_assertions),
-    not(allocator = "default"),
-    any(not(target_family = "unix"), allocator = "mimalloc"),
-))]
 use mimalloc::MiMalloc;
 
 #[cfg(all(
@@ -22,32 +11,10 @@ use crate::memory::TracemallocAllocator;
 #[global_allocator]
 #[cfg(all(
     not(debug_assertions),
-    not(allocator = "mimalloc"),
     not(allocator = "default"),
-    target_family = "unix",
-))]
-static ALLOC: Jemalloc = Jemalloc;
-
-#[global_allocator]
-#[cfg(all(
-    not(debug_assertions),
-    not(allocator = "default"),
     any(not(target_family = "unix"), allocator = "mimalloc"),
 ))]
 static ALLOC: MiMalloc = MiMalloc;
-
-// On Windows tracemalloc does work. However, we build abi3 wheels, and the
-// relevant C APIs are not part of the limited stable CPython API. As a result,
-// linking breaks on Windows if we use tracemalloc C APIs. So we only use this
-// on Unix for now.
-#[global_allocator]
-#[cfg(all(
-    debug_assertions,
-    target_family = "unix",
-    not(allocator = "default"),
-    not(allocator = "mimalloc"),
-))]
-static ALLOC: TracemallocAllocator<Jemalloc> = TracemallocAllocator::new(Jemalloc);
 
 use std::alloc::Layout;
 use std::ffi::{c_char, c_void};
