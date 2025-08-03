package main

import (
	"C"
	"log"
	"net/http"
	"time"
	"unsafe"
)

func logRequest(r *http.Request) {
	ip := r.RemoteAddr
	if ip == "" {
		ip = "unknown"
	}
	log.Printf("[%s] %s %s", time.Now().Format(time.RFC3339), ip, r.Method+" "+r.URL.Path)
}

func corsWrapper(handler http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		// CORS headers
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type, Authorization")

		// Handle preflight
		if r.Method == http.MethodOptions {
			w.WriteHeader(http.StatusNoContent)
			return
		}

		logRequest(r)
		handler(w, r)
	}
}

func main() {
	// Init Rust state once at startup
	log.Println("Calling Rust init() at startup...")
	buf := make([]byte, bufSize)
	cbuf := (*C.char)(unsafe.Pointer(&buf[0]))
	rustInit(cbuf)
	log.Printf("Rust init response: %s\n", C.GoString(cbuf))

	// Route handlers with CORS wrapper
	http.HandleFunc("/", corsWrapper(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			http.Error(w, "only GET", http.StatusMethodNotAllowed)
			return
		}
		callRustNoInput(rustRoot, w)
	}))

	http.HandleFunc("/user-list", corsWrapper(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			http.Error(w, "only GET", http.StatusMethodNotAllowed)
			return
		}
		callRustNoInput(rustUserList, w)
	}))

	http.HandleFunc("/signup", corsWrapper(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "only POST", http.StatusMethodNotAllowed)
			return
		}
		callRustWithJSONBody(rustSignup, w, r)
	}))

	http.HandleFunc("/login", corsWrapper(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "only POST", http.StatusMethodNotAllowed)
			return
		}
		callRustWithJSONBody(rustLogin, w, r)
	}))

	http.HandleFunc("/get_user", corsWrapper(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "only POST", http.StatusMethodNotAllowed)
			return
		}
		callRustWithJSONBody(rustGetUser, w, r)
	}))

	log.Println("Starting server on :8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
