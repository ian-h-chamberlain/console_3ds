/* automatically generated by rust-bindgen 0.59.2 */

pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 1;
pub const _IONBF: u32 = 2;
pub type __uint32_t = ::libc::c_uint;
pub type __int64_t = ::libc::c_longlong;
pub type _off_t = __int64_t;
pub type __mode_t = __uint32_t;
pub type __off_t = _off_t;
pub type _ssize_t = ::libc::c_int;
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct _mbstate_t {
    pub _bindgen_opaque_blob: [u32; 2usize],
}
#[repr(C)]
#[repr(align(4))]
#[derive(Copy, Clone)]
pub union _mbstate_t__bindgen_ty_1 {
    pub _bindgen_opaque_blob: u32,
}
impl Default for _mbstate_t__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct __locale_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct _Bigint {
    pub _bindgen_opaque_blob: [u32; 6usize],
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct __tm {
    pub _bindgen_opaque_blob: [u32; 9usize],
}
#[repr(C)]
#[repr(align(4))]
pub struct _atexit {
    pub _bindgen_opaque_blob: [u32; 100usize],
}
impl Default for _atexit {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct __sFILE {
    pub _bindgen_opaque_blob: [u64; 15usize],
}
pub type __FILE = __sFILE;
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct _glue {
    pub _bindgen_opaque_blob: [u32; 3usize],
}
#[repr(C)]
#[repr(align(2))]
#[derive(Debug, Default, Copy, Clone)]
pub struct _rand48 {
    pub _bindgen_opaque_blob: [u16; 7usize],
}
#[repr(C)]
pub struct _reent {
    pub _errno: ::libc::c_int,
    pub _stdin: *mut __FILE,
    pub _stdout: *mut __FILE,
    pub _stderr: *mut __FILE,
    pub _inc: ::libc::c_int,
    pub _emergency: [::libc::c_char; 25usize],
    pub _unspecified_locale_info: ::libc::c_int,
    pub _locale: *mut __locale_t,
    pub __sdidinit: ::libc::c_int,
    pub __cleanup: ::core::option::Option<unsafe extern "C" fn(arg1: *mut _reent)>,
    pub _result: *mut _Bigint,
    pub _result_k: ::libc::c_int,
    pub _p5s: *mut _Bigint,
    pub _freelist: *mut *mut _Bigint,
    pub _cvtlen: ::libc::c_int,
    pub _cvtbuf: *mut ::libc::c_char,
    pub _new: _reent__bindgen_ty_1,
    pub _atexit: *mut _atexit,
    pub _atexit0: _atexit,
    pub _sig_func: *mut ::core::option::Option<unsafe extern "C" fn(arg1: ::libc::c_int)>,
    pub __sglue: _glue,
    pub __sf: [__FILE; 3usize],
    pub deviceData: *mut ::libc::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _reent__bindgen_ty_1 {
    pub _reent: _reent__bindgen_ty_1__bindgen_ty_1,
    pub _unused: _reent__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _reent__bindgen_ty_1__bindgen_ty_1 {
    pub _unused_rand: ::libc::c_uint,
    pub _strtok_last: *mut ::libc::c_char,
    pub _asctime_buf: [::libc::c_char; 26usize],
    pub _localtime_buf: __tm,
    pub _gamma_signgam: ::libc::c_int,
    pub _rand_next: ::libc::c_ulonglong,
    pub _r48: _rand48,
    pub _mblen_state: _mbstate_t,
    pub _mbtowc_state: _mbstate_t,
    pub _wctomb_state: _mbstate_t,
    pub _l64a_buf: [::libc::c_char; 8usize],
    pub _signal_buf: [::libc::c_char; 24usize],
    pub _getdate_err: ::libc::c_int,
    pub _mbrlen_state: _mbstate_t,
    pub _mbrtowc_state: _mbstate_t,
    pub _mbsrtowcs_state: _mbstate_t,
    pub _wcrtomb_state: _mbstate_t,
    pub _wcsrtombs_state: _mbstate_t,
    pub _h_errno: ::libc::c_int,
}
impl Default for _reent__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _reent__bindgen_ty_1__bindgen_ty_2 {
    pub _nextf: [*mut ::libc::c_uchar; 30usize],
    pub _nmalloc: [::libc::c_uint; 30usize],
}
impl Default for _reent__bindgen_ty_1__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for _reent__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for _reent {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub fn __getreent() -> *mut _reent;
}
extern "C" {
    pub fn __errno() -> *mut ::libc::c_int;
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct timeval {
    pub _bindgen_opaque_blob: [u64; 2usize],
}
pub type off_t = __off_t;
pub type mode_t = __mode_t;
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct stat {
    pub _bindgen_opaque_blob: [u64; 12usize],
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct statvfs {
    pub _bindgen_opaque_blob: [u64; 7usize],
}
pub const STD_IN: ::libc::c_uint = 0;
pub const STD_OUT: ::libc::c_uint = 1;
pub const STD_ERR: ::libc::c_uint = 2;
pub const STD_MAX: ::libc::c_uint = 35;
pub type _bindgen_ty_1 = ::libc::c_uint;
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct DIR_ITER {
    pub _bindgen_opaque_blob: [u32; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct devoptab_t {
    pub name: *const ::libc::c_char,
    pub structSize: usize,
    pub open_r: ::core::option::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            fileStruct: *mut ::libc::c_void,
            path: *const ::libc::c_char,
            flags: ::libc::c_int,
            mode: ::libc::c_int,
        ) -> ::libc::c_int,
    >,
    pub close_r: ::core::option::Option<
        unsafe extern "C" fn(r: *mut _reent, fd: *mut ::libc::c_void) -> ::libc::c_int,
    >,
    pub write_r: ::core::option::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            fd: *mut ::libc::c_void,
            ptr: *const ::libc::c_char,
            len: usize,
        ) -> isize,
    >,
    pub read_r: ::core::option::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            fd: *mut ::libc::c_void,
            ptr: *mut ::libc::c_char,
            len: usize,
        ) -> isize,
    >,
    pub seek_r: ::core::option::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            fd: *mut ::libc::c_void,
            pos: off_t,
            dir: ::libc::c_int,
        ) -> off_t,
    >,
    pub fstat_r: ::core::option::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            fd: *mut ::libc::c_void,
            st: *mut stat,
        ) -> ::libc::c_int,
    >,
    pub stat_r: ::core::option::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            file: *const ::libc::c_char,
            st: *mut stat,
        ) -> ::libc::c_int,
    >,
    pub link_r: ::core::option::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            existing: *const ::libc::c_char,
            newLink: *const ::libc::c_char,
        ) -> ::libc::c_int,
    >,
    pub unlink_r: ::core::option::Option<
        unsafe extern "C" fn(r: *mut _reent, name: *const ::libc::c_char) -> ::libc::c_int,
    >,
    pub chdir_r: ::core::option::Option<
        unsafe extern "C" fn(r: *mut _reent, name: *const ::libc::c_char) -> ::libc::c_int,
    >,
    pub rename_r: ::core::option::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            oldName: *const ::libc::c_char,
            newName: *const ::libc::c_char,
        ) -> ::libc::c_int,
    >,
    pub mkdir_r: ::core::option::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            path: *const ::libc::c_char,
            mode: ::libc::c_int,
        ) -> ::libc::c_int,
    >,
    pub dirStateSize: usize,
    pub diropen_r: ::core::option::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            dirState: *mut DIR_ITER,
            path: *const ::libc::c_char,
        ) -> *mut DIR_ITER,
    >,
    pub dirreset_r: ::core::option::Option<
        unsafe extern "C" fn(r: *mut _reent, dirState: *mut DIR_ITER) -> ::libc::c_int,
    >,
    pub dirnext_r: ::core::option::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            dirState: *mut DIR_ITER,
            filename: *mut ::libc::c_char,
            filestat: *mut stat,
        ) -> ::libc::c_int,
    >,
    pub dirclose_r: ::core::option::Option<
        unsafe extern "C" fn(r: *mut _reent, dirState: *mut DIR_ITER) -> ::libc::c_int,
    >,
    pub statvfs_r: ::core::option::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            path: *const ::libc::c_char,
            buf: *mut statvfs,
        ) -> ::libc::c_int,
    >,
    pub ftruncate_r: ::core::option::Option<
        unsafe extern "C" fn(r: *mut _reent, fd: *mut ::libc::c_void, len: off_t) -> ::libc::c_int,
    >,
    pub fsync_r: ::core::option::Option<
        unsafe extern "C" fn(r: *mut _reent, fd: *mut ::libc::c_void) -> ::libc::c_int,
    >,
    pub deviceData: *mut ::libc::c_void,
    pub chmod_r: ::core::option::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            path: *const ::libc::c_char,
            mode: mode_t,
        ) -> ::libc::c_int,
    >,
    pub fchmod_r: ::core::option::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            fd: *mut ::libc::c_void,
            mode: mode_t,
        ) -> ::libc::c_int,
    >,
    pub rmdir_r: ::core::option::Option<
        unsafe extern "C" fn(r: *mut _reent, name: *const ::libc::c_char) -> ::libc::c_int,
    >,
    pub lstat_r: ::core::option::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            file: *const ::libc::c_char,
            st: *mut stat,
        ) -> ::libc::c_int,
    >,
    pub utimes_r: ::core::option::Option<
        unsafe extern "C" fn(
            r: *mut _reent,
            filename: *const ::libc::c_char,
            times: *const timeval,
        ) -> ::libc::c_int,
    >,
}
impl Default for devoptab_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub static mut devoptab_list: [*const devoptab_t; 0usize];
}
