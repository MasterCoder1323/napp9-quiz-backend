package main

import (
	"log"
	"net/http"
	"time"
	"unsafe"
	"C"
)

func logRequest(r *http.Request) {
	ip := r.RemoteAddr
	if ip == "" {
		ip = "unknown"
	}
	log.Printf("[%s] %s %s", time.Now().Format(time.RFC3339), ip, r.Method+" "+r.URL.Path)
}

func main() {
	// Init Rust state once at startup
	log.Println("Calling Rust init() at startup...")
	buf := make([]byte, bufSize)
	cbuf := (*C.char)(unsafe.Pointer(&buf[0]))
	rustInit(cbuf)
	log.Printf("Rust init response: %s\n", C.GoString(cbuf))

	// Wrap handler functions to add logging
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		logRequest(r)
		if r.Method != http.MethodGet {
			http.Error(w, "only GET", http.StatusMethodNotAllowed)
			return
		}
		callRustNoInput(rustRoot, w)
	})

	http.HandleFunc("/user-list", func(w http.ResponseWriter, r *http.Request) {
		logRequest(r)
		if r.Method != http.MethodGet {
			http.Error(w, "only GET", http.StatusMethodNotAllowed)
			return
		}
		callRustNoInput(rustUserList, w)
	})

	http.HandleFunc("/signup", func(w http.ResponseWriter, r *http.Request) {
		logRequest(r)
		if r.Method != http.MethodPost {
			http.Error(w, "only POST", http.StatusMethodNotAllowed)
			return
		}
		callRustWithJSONBody(rustSignup, w, r)
	})

	http.HandleFunc("/login", func(w http.ResponseWriter, r *http.Request) {
		logRequest(r)
		if r.Method != http.MethodPost {
			http.Error(w, "only POST", http.StatusMethodNotAllowed)
			return
		}
		callRustWithJSONBody(rustLogin, w, r)
	})

	http.HandleFunc("/get_user", func(w http.ResponseWriter, r *http.Request) {
		logRequest(r)
		if r.Method != http.MethodPost {
			http.Error(w, "only POST", http.StatusMethodNotAllowed)
			return
		}
		callRustWithJSONBody(rustGetUser, w, r)
	})

	log.Println("Starting server on :8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
