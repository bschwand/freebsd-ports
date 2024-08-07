--- src/webrtc/src/third_party/perfetto/src/tracing/track.cc.orig	2023-05-31 08:12:17 UTC
+++ src/webrtc/src/third_party/perfetto/src/tracing/track.cc
@@ -143,7 +143,7 @@ namespace internal {
 namespace {
 
 uint64_t GetProcessStartTime() {
-#if !PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
+#if !PERFETTO_BUILDFLAG(PERFETTO_OS_WIN) && !PERFETTO_BUILDFLAG(PERFETTO_OS_BSD)
   std::string stat;
   if (!base::ReadFile("/proc/self/stat", &stat))
     return 0u;
