from cffi import FFI
ffi = FFI()
ffi.cdef("""
    int inc(int);
""")

C = ffi.dlopen("../rust/target/debug/libdyn.so")

print(C.inc(5))