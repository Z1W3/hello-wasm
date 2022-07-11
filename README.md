# hello-wasm

### wasm32-unknown-emscripten

```text
  install ./include/openssl/whrlpool.h -> /Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-unknown-emscripten/debug/build/openssl-sys-b67cdc9bfaf9e556/out/openssl-build/install/include/openssl/whrlpool.h
  install ./include/openssl/x509.h -> /Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-unknown-emscripten/debug/build/openssl-sys-b67cdc9bfaf9e556/out/openssl-build/install/include/openssl/x509.h
  install ./include/openssl/x509_vfy.h -> /Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-unknown-emscripten/debug/build/openssl-sys-b67cdc9bfaf9e556/out/openssl-build/install/include/openssl/x509_vfy.h
  install ./include/openssl/x509err.h -> /Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-unknown-emscripten/debug/build/openssl-sys-b67cdc9bfaf9e556/out/openssl-build/install/include/openssl/x509err.h
  install ./include/openssl/x509v3.h -> /Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-unknown-emscripten/debug/build/openssl-sys-b67cdc9bfaf9e556/out/openssl-build/install/include/openssl/x509v3.h
  install ./include/openssl/x509v3err.h -> /Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-unknown-emscripten/debug/build/openssl-sys-b67cdc9bfaf9e556/out/openssl-build/install/include/openssl/x509v3err.h
  install libcrypto.a -> /Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-unknown-emscripten/debug/build/openssl-sys-b67cdc9bfaf9e556/out/openssl-build/install/lib/libcrypto.a

  --- stderr
  ar: creating archive apps/libapps.a
  warning: /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ranlib: archive library: apps/libapps.a the table of contents is empty (no object file members in the library define global symbols)
  ar: creating archive libcrypto.a
  warning: /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ranlib: archive library: libcrypto.a the table of contents is empty (no object file members in the library define global symbols)
  ar: creating archive libssl.a
  warning: /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ranlib: archive library: libssl.a the table of contents is empty (no object file members in the library define global symbols)
  LLVM ERROR: malformed uleb128, extends past end
  /bin/sh: line 1: 34426 Abort trap: 6           ranlib "/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-unknown-emscripten/debug/build/openssl-sys-b67cdc9bfaf9e556/out/openssl-build/install/lib/$fn.new"
  make: *** [install_dev] Error 134
  thread 'main' panicked at '


  Error installing OpenSSL:
      Command: "make" "install_dev"
      Exit status: exit status: 2


      ', /Users/catt.zhi/.cargo/registry/src/github.com-1ecc6299db9ec823/openssl-src-111.22.0+1.1.1q/src/lib.rs:490:13
  stack backtrace:
     0: rust_begin_unwind
               at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc/library/std/src/panicking.rs:584:5
     1: core::panicking::panic_fmt
               at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc/library/core/src/panicking.rs:142:14
     2: openssl_src::Build::run_command
               at /Users/catt.zhi/.cargo/registry/src/github.com-1ecc6299db9ec823/openssl-src-111.22.0+1.1.1q/src/lib.rs:490:13
     3: openssl_src::Build::build
               at /Users/catt.zhi/.cargo/registry/src/github.com-1ecc6299db9ec823/openssl-src-111.22.0+1.1.1q/src/lib.rs:466:13
     4: build_script_main::find_vendored::get_openssl
               at ./build/find_vendored.rs:5:21
     5: build_script_main::find_openssl
               at ./build/main.rs:57:20
     6: build_script_main::main
               at ./build/main.rs:68:35
     7: core::ops::function::FnOnce::call_once
               at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc/library/core/src/ops/function.rs:248:5
```

### wasm32-wasi

```text
   Compiling openssl-sys v0.9.75
error: failed to run custom build command for `openssl-sys v0.9.75`

Caused by:
  process didn't exit successfully: `/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/debug/build/openssl-sys-142d7fbe1da909f9/build-script-main` (exit status: 101)
  --- stdout
  cargo:rustc-cfg=const_fn
  cargo:rerun-if-env-changed=WASM32_WASI_OPENSSL_NO_VENDOR
  WASM32_WASI_OPENSSL_NO_VENDOR unset
  cargo:rerun-if-env-changed=OPENSSL_NO_VENDOR
  OPENSSL_NO_VENDOR unset
  CC_wasm32-wasi = None
  CC_wasm32_wasi = None
  TARGET_CC = None
  CC = None
  CFLAGS_wasm32-wasi = None
  CFLAGS_wasm32_wasi = None
  TARGET_CFLAGS = None
  CFLAGS = None
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some("true")
  running "perl" "./Configure" "--prefix=/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-wasi/debug/build/openssl-sys-a7bd188ca3d6e119/out/openssl-build/install" "--openssldir=/usr/local/ssl" "no-dso" "no-shared" "no-ssl3" "no-unit-test" "no-comp" "no-zlib" "no-zlib-dynamic" "no-md2" "no-rc5" "no-weak-ssl-ciphers" "no-camellia" "no-idea" "no-seed" "gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=wasm32-wasi"
  Configuring OpenSSL version 1.1.1q (0x1010111fL) for gcc
  Using os-specific seed configuration
  Creating configdata.pm
  Creating Makefile

  The library could not be configured for supporting multi-threaded
  applications as the compiler options required on this system are not known.
  See file INSTALL for details if you need multi-threading.

  **********************************************************************
  ***                                                                ***
  ***   OpenSSL has been successfully configured                     ***
  ***                                                                ***
  ***   If you encounter a problem while building, please open an    ***
  ***   issue on GitHub <https://github.com/openssl/openssl/issues>  ***
  ***   and include the output from the following command:           ***
  ***                                                                ***
  ***       perl configdata.pm --dump                                ***
  ***                                                                ***
  ***   (If you are new to OpenSSL, you might want to consult the    ***
  ***   'Troubleshooting' section in the INSTALL file first)         ***
  ***                                                                ***
  **********************************************************************
  running "make" "depend"
  running "make" "build_libs"
  perl "-I." -Mconfigdata "util/dofile.pl" \
            "-oMakefile" include/crypto/bn_conf.h.in > include/crypto/bn_conf.h
  perl "-I." -Mconfigdata "util/dofile.pl" \
            "-oMakefile" include/crypto/dso_conf.h.in > include/crypto/dso_conf.h
  perl "-I." -Mconfigdata "util/dofile.pl" \
            "-oMakefile" include/openssl/opensslconf.h.in > include/openssl/opensslconf.h
  /Applications/Xcode.app/Contents/Developer/usr/bin/make depend && /Applications/Xcode.app/Contents/Developer/usr/bin/make _build_libs
  clang  -I. -Iinclude  -O3 -O2 -ffunction-sections -fdata-sections -fPIC -g -fno-omit-frame-pointer --target=wasm32-wasi -DOPENSSLDIR="\"/usr/local/ssl\"" -DENGINESDIR="\"/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-wasi/debug/build/openssl-sys-a7bd188ca3d6e119/out/openssl-build/install/lib/engines-1.1\"" -DNDEBUG -I/usr/local/opt/openssl@3/include -MMD -MF apps/app_rand.d.tmp -MT apps/app_rand.o -c -o apps/app_rand.o apps/app_rand.c
  clang  -I. -Iinclude  -O3 -O2 -ffunction-sections -fdata-sections -fPIC -g -fno-omit-frame-pointer --target=wasm32-wasi -DOPENSSLDIR="\"/usr/local/ssl\"" -DENGINESDIR="\"/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-wasi/debug/build/openssl-sys-a7bd188ca3d6e119/out/openssl-build/install/lib/engines-1.1\"" -DNDEBUG -I/usr/local/opt/openssl@3/include -MMD -MF apps/apps.d.tmp -MT apps/apps.o -c -o apps/apps.o apps/apps.c
  clang  -I. -Iinclude  -O3 -O2 -ffunction-sections -fdata-sections -fPIC -g -fno-omit-frame-pointer --target=wasm32-wasi -DOPENSSLDIR="\"/usr/local/ssl\"" -DENGINESDIR="\"/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-wasi/debug/build/openssl-sys-a7bd188ca3d6e119/out/openssl-build/install/lib/engines-1.1\"" -DNDEBUG -I/usr/local/opt/openssl@3/include -MMD -MF apps/bf_prefix.d.tmp -MT apps/bf_prefix.o -c -o apps/bf_prefix.o apps/bf_prefix.c
  clang  -I. -Iinclude  -O3 -O2 -ffunction-sections -fdata-sections -fPIC -g -fno-omit-frame-pointer --target=wasm32-wasi -DOPENSSLDIR="\"/usr/local/ssl\"" -DENGINESDIR="\"/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-wasi/debug/build/openssl-sys-a7bd188ca3d6e119/out/openssl-build/install/lib/engines-1.1\"" -DNDEBUG -I/usr/local/opt/openssl@3/include -MMD -MF apps/opt.d.tmp -MT apps/opt.o -c -o apps/opt.o apps/opt.c
  clang  -I. -Iinclude  -O3 -O2 -ffunction-sections -fdata-sections -fPIC -g -fno-omit-frame-pointer --target=wasm32-wasi -DOPENSSLDIR="\"/usr/local/ssl\"" -DENGINESDIR="\"/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-wasi/debug/build/openssl-sys-a7bd188ca3d6e119/out/openssl-build/install/lib/engines-1.1\"" -DNDEBUG -I/usr/local/opt/openssl@3/include -MMD -MF apps/s_cb.d.tmp -MT apps/s_cb.o -c -o apps/s_cb.o apps/s_cb.c
  clang  -I. -Iinclude  -O3 -O2 -ffunction-sections -fdata-sections -fPIC -g -fno-omit-frame-pointer --target=wasm32-wasi -DOPENSSLDIR="\"/usr/local/ssl\"" -DENGINESDIR="\"/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-wasi/debug/build/openssl-sys-a7bd188ca3d6e119/out/openssl-build/install/lib/engines-1.1\"" -DNDEBUG -I/usr/local/opt/openssl@3/include -MMD -MF apps/s_socket.d.tmp -MT apps/s_socket.o -c -o apps/s_socket.o apps/s_socket.c
  clang  -I. -Iinclude  -O3 -O2 -ffunction-sections -fdata-sections -fPIC -g -fno-omit-frame-pointer --target=wasm32-wasi -DOPENSSLDIR="\"/usr/local/ssl\"" -DENGINESDIR="\"/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-wasi/debug/build/openssl-sys-a7bd188ca3d6e119/out/openssl-build/install/lib/engines-1.1\"" -DNDEBUG -I/usr/local/opt/openssl@3/include -MMD -MF crypto/aes/aes_cbc.d.tmp -MT crypto/aes/aes_cbc.o -c -o crypto/aes/aes_cbc.o crypto/aes/aes_cbc.c
  clang  -I. -Iinclude  -O3 -O2 -ffunction-sections -fdata-sections -fPIC -g -fno-omit-frame-pointer --target=wasm32-wasi -DOPENSSLDIR="\"/usr/local/ssl\"" -DENGINESDIR="\"/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-wasi/debug/build/openssl-sys-a7bd188ca3d6e119/out/openssl-build/install/lib/engines-1.1\"" -DNDEBUG -I/usr/local/opt/openssl@3/include -MMD -MF crypto/aes/aes_cfb.d.tmp -MT crypto/aes/aes_cfb.o -c -o crypto/aes/aes_cfb.o crypto/aes/aes_cfb.c
  clang  -I. -Iinclude  -O3 -O2 -ffunction-sections -fdata-sections -fPIC -g -fno-omit-frame-pointer --target=wasm32-wasi -DOPENSSLDIR="\"/usr/local/ssl\"" -DENGINESDIR="\"/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-wasi/debug/build/openssl-sys-a7bd188ca3d6e119/out/openssl-build/install/lib/engines-1.1\"" -DNDEBUG -I/usr/local/opt/openssl@3/include -MMD -MF crypto/aes/aes_core.d.tmp -MT crypto/aes/aes_core.o -c -o crypto/aes/aes_core.o crypto/aes/aes_core.c
  clang  -I. -Iinclude  -O3 -O2 -ffunction-sections -fdata-sections -fPIC -g -fno-omit-frame-pointer --target=wasm32-wasi -DOPENSSLDIR="\"/usr/local/ssl\"" -DENGINESDIR="\"/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-wasi/debug/build/openssl-sys-a7bd188ca3d6e119/out/openssl-build/install/lib/engines-1.1\"" -DNDEBUG -I/usr/local/opt/openssl@3/include -MMD -MF crypto/aes/aes_ecb.d.tmp -MT crypto/aes/aes_ecb.o -c -o crypto/aes/aes_ecb.o crypto/aes/aes_ecb.c
  clang  -I. -Iinclude  -O3 -O2 -ffunction-sections -fdata-sections -fPIC -g -fno-omit-frame-pointer --target=wasm32-wasi -DOPENSSLDIR="\"/usr/local/ssl\"" -DENGINESDIR="\"/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-wasi/debug/build/openssl-sys-a7bd188ca3d6e119/out/openssl-build/install/lib/engines-1.1\"" -DNDEBUG -I/usr/local/opt/openssl@3/include -MMD -MF crypto/aes/aes_ige.d.tmp -MT crypto/aes/aes_ige.o -c -o crypto/aes/aes_ige.o crypto/aes/aes_ige.c
  clang  -I. -Iinclude  -O3 -O2 -ffunction-sections -fdata-sections -fPIC -g -fno-omit-frame-pointer --target=wasm32-wasi -DOPENSSLDIR="\"/usr/local/ssl\"" -DENGINESDIR="\"/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-wasi/debug/build/openssl-sys-a7bd188ca3d6e119/out/openssl-build/install/lib/engines-1.1\"" -DNDEBUG -I/usr/local/opt/openssl@3/include -MMD -MF crypto/aes/aes_misc.d.tmp -MT crypto/aes/aes_misc.o -c -o crypto/aes/aes_misc.o crypto/aes/aes_misc.c
  clang  -I. -Iinclude  -O3 -O2 -ffunction-sections -fdata-sections -fPIC -g -fno-omit-frame-pointer --target=wasm32-wasi -DOPENSSLDIR="\"/usr/local/ssl\"" -DENGINESDIR="\"/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-wasi/debug/build/openssl-sys-a7bd188ca3d6e119/out/openssl-build/install/lib/engines-1.1\"" -DNDEBUG -I/usr/local/opt/openssl@3/include -MMD -MF crypto/aes/aes_ofb.d.tmp -MT crypto/aes/aes_ofb.o -c -o crypto/aes/aes_ofb.o crypto/aes/aes_ofb.c
  clang  -I. -Iinclude  -O3 -O2 -ffunction-sections -fdata-sections -fPIC -g -fno-omit-frame-pointer --target=wasm32-wasi -DOPENSSLDIR="\"/usr/local/ssl\"" -DENGINESDIR="\"/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-wasi/debug/build/openssl-sys-a7bd188ca3d6e119/out/openssl-build/install/lib/engines-1.1\"" -DNDEBUG -I/usr/local/opt/openssl@3/include -MMD -MF crypto/aes/aes_wrap.d.tmp -MT crypto/aes/aes_wrap.o -c -o crypto/aes/aes_wrap.o crypto/aes/aes_wrap.c
  clang  -I. -Iinclude  -O3 -O2 -ffunction-sections -fdata-sections -fPIC -g -fno-omit-frame-pointer --target=wasm32-wasi -DOPENSSLDIR="\"/usr/local/ssl\"" -DENGINESDIR="\"/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-wasi/debug/build/openssl-sys-a7bd188ca3d6e119/out/openssl-build/install/lib/engines-1.1\"" -DNDEBUG -I/usr/local/opt/openssl@3/include -MMD -MF crypto/aria/aria.d.tmp -MT crypto/aria/aria.o -c -o crypto/aria/aria.o crypto/aria/aria.c
  clang  -I. -Iinclude  -O3 -O2 -ffunction-sections -fdata-sections -fPIC -g -fno-omit-frame-pointer --target=wasm32-wasi -DOPENSSLDIR="\"/usr/local/ssl\"" -DENGINESDIR="\"/Users/catt.zhi/Projects/AndroidStudioProjects/payease/fork_projects/ehking-common-android/hello-wasm4/target/wasm32-wasi/debug/build/openssl-sys-a7bd188ca3d6e119/out/openssl-build/install/lib/engines-1.1\"" -DNDEBUG -I/usr/local/opt/openssl@3/include -MMD -MF crypto/asn1/a_bitstr.d.tmp -MT crypto/asn1/a_bitstr.o -c -o crypto/asn1/a_bitstr.o crypto/asn1/a_bitstr.c

  --- stderr
  apps/s_cb.c:11:10: fatal error: 'stdio.h' file not found
  #include <stdio.h>
           ^~~~~~~~~
  In file included from apps/app_rand.c:10:
  In file included from apps/apps.h:13:
  In file included from ./e_os.h:16:
  In file included from include/openssl/e_os2.h:243:
  /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/clang/13.1.6/include/inttypes.h:21:15: fatal error: 'inttypes.h' file not found
  #include_next <inttypes.h>
                ^~~~~~~~~~~~
  apps/s_socket.c:11:10: fatal error: 'stdio.h' file not found
  #include <stdio.h>
           ^~~~~~~~~
  In file included from apps/opt.c:9:
  In file included from apps/apps.h:13:
  In file included from ./e_os.h:16:
  In file included from apps/bf_prefix.c:include/openssl/e_os2.h10::24310:
  : fatal error: /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/clang/13.1.6/include/inttypes.h'stdio.h' file not found:21:15: fatal error: 'inttypes.h' file not found

  #include_next <inttypes.h>
                ^~~~~~~~~~~~
  #include <stdio.h>
           ^~~~~~~~~
  crypto/aes/aes_ecb.c:10:10: fatal error: 'assert.h' file not found
  #include <assert.h>
           ^~~~~~~~~~
  In file included from crypto/aes/aes_wrap.c:10:
  include/internal/cryptlib.h:13:11: fatal error: 'stdlib.h' file not found
  In file included from crypto/aes/aes_ige.c:10:
  include/internal/cryptlib.h:13:11: fatal error: 'stdlib.h' file not found
  # include <stdlib.h>
            ^~~~~~~~~~
  # include <stdlib.h>
            ^~~~~~~~~~
  crypto/aes/aes_core.c:39:10: fatal error: 'assert.h' file not found
  #include <assert.h>
           ^~~~~~~~~~
  In file included from crypto/aria/aria.c:21:
  In file included from include/openssl/e_os2.h:243:
  /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/clang/13.1.6/include/inttypes.h:21:15: fatal error: 'inttypes.h' file not found
  #include_next <inttypes.h>
                ^~~~~~~~~~~~
  crypto/asn1/a_bitstr.c:11:10: fatal error: 'stdio.h' file not found
  #include <stdio.h>
           ^~~~~~~~~
  apps/apps.c:18:10: fatal error: 'stdio.h' file not found
  #include <stdio.h>
           ^~~~~~~~~
  In file included from crypto/aes/aes_misc.c:12:
  In file included from crypto/aes/aes_local.h:13:
  In file included from include/openssl/e_os2.h:243:
  /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/clang/13.1.6/include/inttypes.h:21:15: fatal error: 'inttypes.h' file not found
  #include_next <inttypes.h>
                ^~~~~~~~~~~~
  1 error generated.
  1 error generated.
  error: unable to create target: 'No available targets are compatible with triple "wasm32-unknown-wasi"'error: unable to create target: 'No available targets are compatible with triple "wasm32-unknown-wasi"'

  error: unable to create target: 'No available targets are compatible with triple "wasm32-unknown-wasi"'
  make[1]: *** [crypto/aes/aes_misc.o] Error 1
  make[1]: *** Waiting for unfinished jobs....
  make[1]: *** [crypto/aes/aes_ecb.o] Error 1
  1 error generated.
  1 error generated.
  1 error generated.
  1 error generated.
  make[1]: *** [crypto/aes/aes_ofb.o] Error 1
  make[1]: *** [crypto/aes/aes_cfb.o] Error 1
  make[1]: *** [crypto/aes/aes_cbc.o] Error 1
  make[1]: *** [crypto/aria/aria.o] Error 1
  1 error generated.
  1 error generated.
  make[1]: *** [crypto/aes/aes_core.o] Error 1
  1 error generated.
  make[1]: *** [crypto/aes/aes_wrap.o] Error 1
  make[1]: *** [crypto/aes/aes_ige.o] Error 1
  1 error generated.
  make[1]: *** [crypto/asn1/a_bitstr.o] Error 1
  1 error generated.
  1 error generated.
  make[1]: *** [apps/app_rand.o] Error 1
  make[1]: *** [apps/bf_prefix.o] Error 1
  1 error generated.
  make[1]: *** [apps/opt.o] Error 1
  1 error generated.
  make[1]: *** [apps/s_socket.o] Error 1
  1 error generated.
  make[1]: *** [apps/s_cb.o] Error 1
  1 error generated.
  make[1]: *** [apps/apps.o] Error 1
  make: *** [build_libs] Error 2
  thread 'main' panicked at '


  Error building OpenSSL:
      Command: "make" "build_libs"
      Exit status: exit status: 2


      ', /Users/catt.zhi/.cargo/registry/src/github.com-1ecc6299db9ec823/openssl-src-111.22.0+1.1.1q/src/lib.rs:490:13
  stack backtrace:
     0: rust_begin_unwind
               at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc/library/std/src/panicking.rs:584:5
     1: core::panicking::panic_fmt
               at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc/library/core/src/panicking.rs:142:14
     2: openssl_src::Build::run_command
               at /Users/catt.zhi/.cargo/registry/src/github.com-1ecc6299db9ec823/openssl-src-111.22.0+1.1.1q/src/lib.rs:490:13
     3: openssl_src::Build::build
               at /Users/catt.zhi/.cargo/registry/src/github.com-1ecc6299db9ec823/openssl-src-111.22.0+1.1.1q/src/lib.rs:462:13
     4: build_script_main::find_vendored::get_openssl
               at ./build/find_vendored.rs:5:21
     5: build_script_main::find_openssl
               at ./build/main.rs:57:20
     6: build_script_main::main
               at ./build/main.rs:68:35
     7: core::ops::function::FnOnce::call_once
               at /rustc/a8314ef7d0ec7b75c336af2c9857bfaf43002bfc/library/core/src/ops/function.rs:248:5
```