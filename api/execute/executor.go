package execute

/*
#cgo LDFLAGS: -L../../ffi/target/x86_64-pc-windows-gnu/release -lengine
#include <stdlib.h>

extern char* execute_sql(const char* sql);
extern void free_string(char* ptr);
*/
import "C"
import "unsafe"

func Execute(query string) string {
    cQuery := C.CString(query)
    defer C.free(unsafe.Pointer(cQuery))

    res := C.execute_sql(cQuery)
    if res == nil {
        return "Error: Rust returned null"
    }
    defer C.free_string(res)

    return C.GoString(res)
}
