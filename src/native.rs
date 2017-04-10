//! QVM as native shared library.

use libc;

/// Engine's syscall function type.
///
/// For communication from QVM to the engine's syscall handler for this QVM.
///
/// NOTE: The function is not really variadic, the actual number of arguments is an implementation
/// detail. See `MAX_VMSYSCALL_ARGS` in ioquake3's [qcommon/vm_local.h](https://github.com/ioquake/ioq3/blob/master/code/qcommon/vm_local.h).
pub type Syscall = extern "C" fn(arg: libc::intptr_t, ...) -> libc::intptr_t;

/// Interface for native QVM implementations.
// TODO: Find a better name than `QVM`
pub trait QVM: 'static + Sync + Send {
    /// Initialization function.
    ///
    /// `syscall` is a callback into the engine.
    fn dll_entry(syscall: Syscall) -> Box<Self> where Self: Sized;

    /// QVM dispatcher function.
    ///
    /// Engine calls this for game logic.
    fn vm_main(&self,
               command: libc::c_int,
               arg0: libc::c_int,
               arg1: libc::c_int,
               arg2: libc::c_int,
               arg3: libc::c_int,
               arg4: libc::c_int,
               arg5: libc::c_int,
               arg6: libc::c_int,
               arg7: libc::c_int,
               arg8: libc::c_int,
               arg9: libc::c_int,
               arg10: libc::c_int,
               arg11: libc::c_int)
               -> libc::intptr_t;
}

/// Creates the required plumbing to use an `impl QVM` as a native shared library.
///
/// # Examples
///
/// Add the following section to your `Cargo.toml`:
///
/// ```toml
/// [lib]
/// name = "q3hi"
/// crate-type = ["cdylib"]
/// ```
///
/// Then implement a QVM by using the macro as such:
///
/// ```rust
/// #[macro_use]
/// extern crate quake3_qvm;
/// // Needed for C FFI
/// extern crate libc;
/// // Needed to initialize internal `static mut` until const-fns are stable
/// #[macro_use]
/// extern crate lazy_static;
///
/// use std::ffi::CString;
///
/// struct HelloQuake3 {
///    syscall: Syscall,
/// }
///
/// use quake3_qvm::native::*;
///
/// /// See ioquake3's [game/g_public.h](https://github.com/ioquake/ioq3/blob/master/code/game/g_public.h)
/// const G_ERROR: libc::intptr_t = 1;
/// const GAME_INIT: libc::c_int = 0;
/// const GAME_SHUTDOWN: libc::c_int = 1;
///
/// impl QVM for HelloQuake3 {
///    fn dll_entry(syscall: Syscall) -> Box<HelloQuake3> {
///        Box::new(HelloQuake3 { syscall: syscall })
///    }
///
///    fn vm_main(&self,
///               command: libc::c_int,
///               arg0: libc::c_int,
///               arg1: libc::c_int,
///               arg2: libc::c_int,
///               arg3: libc::c_int,
///               arg4: libc::c_int,
///               arg5: libc::c_int,
///               arg6: libc::c_int,
///               arg7: libc::c_int,
///               arg8: libc::c_int,
///               arg9: libc::c_int,
///               arg10: libc::c_int,
///               arg11: libc::c_int)
///               -> libc::intptr_t {
///        match command {
///            GAME_INIT => {
///                (self.syscall)(G_ERROR, CString::new("Hello, World!").unwrap().as_ptr());
///                unreachable!()
///            }
///            GAME_SHUTDOWN => {
///                // Just return a dummy value here for clean shutdown
///                0
///            },
///            _ => panic!("Game command not implemented"),
///        }
///    }
/// }
///
/// # fn main() {
/// native_qvm!(HelloQuake3);
/// # }
/// ```
///
/// Finally build the QVM, put it in the right place for Quake 3 and load it:
///
/// ```sh
/// cargo build
/// mkdir -p ~/.q3a/rust/
/// cp target/debug/libq3hi.so ~/.q3a/rust/qagamex86_64.so
/// ioq3ded +set fs_game rust +set vm_game 0 +map q3dm6
/// ```
#[macro_export]
macro_rules! native_qvm {
    ($ty:ident) => {
        lazy_static! {
            static ref _QVM_IMPL: std::sync::Arc<std::sync::RwLock<Option<Box<QVM>>>> = std::sync::Arc::new(std::sync::RwLock::new(None));
        }

        #[doc(hidden)]
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn dllEntry(syscall: Syscall) {
            let mut QVM_IMPL = _QVM_IMPL.write().unwrap();
            *QVM_IMPL = Some($ty::dll_entry(syscall));
        }

        #[doc(hidden)]
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn vmMain(command: libc::c_int,
                                 arg0: libc::c_int,
                                 arg1: libc::c_int,
                                 arg2: libc::c_int,
                                 arg3: libc::c_int,
                                 arg4: libc::c_int,
                                 arg5: libc::c_int,
                                 arg6: libc::c_int,
                                 arg7: libc::c_int,
                                 arg8: libc::c_int,
                                 arg9: libc::c_int,
                                 arg10: libc::c_int,
                                 arg11: libc::c_int)
                                 -> libc::intptr_t {
            let data = _QVM_IMPL.read().unwrap();
            data.as_ref().unwrap().vm_main(command,
                               arg0,
                               arg1,
                               arg2,
                               arg3,
                               arg4,
                               arg5,
                               arg6,
                               arg7,
                               arg8,
                               arg9,
                               arg10,
                               arg11)
        }
    }
}
