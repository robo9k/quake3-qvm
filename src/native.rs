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
/// NOTE: Requires `lazy_static!` dependency in the crate using this macro, since const-fns are still unstable.
#[macro_export]
macro_rules! native_qvm {
    ($ty:ident) => {
        lazy_static! {
            static ref _QVM_IMPL: std::sync::Mutex<std::cell::RefCell<Option<Box<QVM>>>> = std::sync::Mutex::new(std::cell::RefCell::new(None));
        }

        #[doc(hidden)]
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn dllEntry(syscall: Syscall) {
            let QVM_IMPL = _QVM_IMPL.lock().unwrap();
            *QVM_IMPL.borrow_mut() = Some($ty::dll_entry(syscall));
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
            let data = _QVM_IMPL.lock().unwrap();
            let mut data = data.borrow_mut();
            let qvm_impl = data.as_mut().unwrap();

            (*qvm_impl).vm_main(command, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
        }
    }
}
