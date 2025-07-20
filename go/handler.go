package main

import (
	"C"
	"io"
	"net/http"
	"unsafe"
)

const bufSize = 4096

func callRustNoInput(fn func(*C.char), w http.ResponseWriter) {
	buf := make([]byte, bufSize)
	cbuf := (*C.char)(unsafe.Pointer(&buf[0]))
	fn(cbuf)
	goStr := C.GoString(cbuf)
	w.Write([]byte(goStr))
}

func callRustWithJSONBody(fn func(*C.char), w http.ResponseWriter, r *http.Request) {
	buf := make([]byte, bufSize)
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "failed to read body", http.StatusBadRequest)
		return
	}
	if len(body) >= bufSize {
		http.Error(w, "body too large", http.StatusRequestEntityTooLarge)
		return
	}
	copy(buf, body)
	cbuf := (*C.char)(unsafe.Pointer(&buf[0]))
	fn(cbuf)
	goStr := C.GoString(cbuf)
	w.Write([]byte(goStr))
}
