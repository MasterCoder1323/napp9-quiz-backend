package main

/*
#cgo LDFLAGS: -L. -ldebug 
#include <stdlib.h>

extern void root(char* buffer);
extern void user_list(char* buffer);
extern void signup(char* buffer);
extern void login(char* buffer);
extern void get_user(char* buffer);
extern void init(char* buffer);
*/
import "C"

// Rust function wrappers
func rustRoot(buf *C.char)      { C.root(buf) }
func rustUserList(buf *C.char)  { C.user_list(buf) }
func rustSignup(buf *C.char)    { C.signup(buf) }
func rustLogin(buf *C.char)     { C.login(buf) }
func rustGetUser(buf *C.char)   { C.get_user(buf) }
func rustInit(buf *C.char)      { C.init(buf) }
