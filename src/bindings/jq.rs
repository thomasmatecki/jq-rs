use crate::bindings::jv::jv;
use crate::bindings::jv::jv_parser;
use std::os::raw::c_int;
use std::os::raw::c_uchar;
use std::os::raw::c_uint;
use std::os::raw::c_ushort;
use std::os::raw::c_void;

pub const JQ_DEBUG_TRACE: _bindgen_ty_2 = 1;
pub const JQ_DEBUG_TRACE_DETAIL: _bindgen_ty_2 = 2;
pub const JQ_DEBUG_TRACE_ALL: _bindgen_ty_2 = 3;
pub type _bindgen_ty_2 = u32;

pub type cfunction_ptr = Option<unsafe extern "C" fn() -> ()>;

pub type stack_ptr = c_uchar;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack {
    pub mem_end: *mut c_uchar,
    pub bound: stack_ptr,
    pub limit: stack_ptr,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfunction {
    pub fptr: cfunction_ptr,
    pub name: *const c_uchar,
    pub nargs: c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct symbol_table {
    pub cfunctions: *mut cfunction,
    pub ncfunctions: c_int,
    pub cfunc_names: jv,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct bytecode {
    pub code: *mut c_ushort,
    pub codelen: c_int,
    pub nlocals: c_int,
    pub nclosures: c_int,
    pub constants: jv,
    pub globals: *mut symbol_table,
    pub subfunctions: *mut *mut bytecode,
    pub nsubfunctions: c_int,
    pub parent: *mut bytecode,
    pub debuginfo: jv,
}

pub type jq_msg_cb = Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: jv)>;

pub type jq_input_cb =
    Option<unsafe extern "C" fn(arg1: *mut jq_state, arg2: *mut ::std::os::raw::c_void) -> jv>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jq_state {
    pub nomem_handler: Option<unsafe extern "C" fn(_: *mut c_void) -> ()>,
    pub nomem_handler_data: *mut c_void,
    pub bc: *mut bytecode,
    pub err_cb: jq_msg_cb,
    pub err_cb_data: *mut c_void,
    pub error: jv,
    pub stk: stack,
    pub curr_frame: stack_ptr,
    pub stk_top: stack_ptr,
    pub fork_top: stack_ptr,
    pub path: jv,
    pub value_at_path: jv,
    pub subexp_nest: c_int,
    pub debug_trace_enabled: c_int,
    pub initial_execution: c_int,
    pub next_label: c_uint,
    pub halted: c_int,
    pub exit_code: jv,
    pub error_message: jv,
    pub attrs: jv,
    pub input_cb: jq_input_cb,
    pub input_cb_data: *mut c_void,
    pub debug_cb: jq_msg_cb,
    pub debug_cb_data: *mut c_void,
}
extern "C" {
    pub fn jq_init() -> *mut jq_state;
}
extern "C" {
    pub fn jq_set_error_cb(arg1: *mut jq_state, arg2: jq_msg_cb, arg3: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn jq_get_error_cb(
        arg1: *mut jq_state,
        arg2: *mut jq_msg_cb,
        arg3: *mut *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn jq_set_nomem_handler(
        arg1: *mut jq_state,
        arg2: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        arg3: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn jq_format_error(msg: jv) -> jv;
}
extern "C" {
    pub fn jq_report_error(arg1: *mut jq_state, arg2: jv);
}
extern "C" {
    pub fn jq_compile(
        arg1: *mut jq_state,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jq_compile_args(
        arg1: *mut jq_state,
        arg2: *const ::std::os::raw::c_char,
        arg3: jv,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jq_dump_disassembly(arg1: *mut jq_state, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn jq_start(arg1: *mut jq_state, value: jv, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn jq_next(arg1: *mut jq_state) -> jv;
}
extern "C" {
    pub fn jq_teardown(arg1: *mut *mut jq_state);
}
extern "C" {
    pub fn jq_halt(arg1: *mut jq_state, arg2: jv, arg3: jv);
}
extern "C" {
    pub fn jq_halted(arg1: *mut jq_state) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jq_get_exit_code(arg1: *mut jq_state) -> jv;
}
extern "C" {
    pub fn jq_get_error_message(arg1: *mut jq_state) -> jv;
}
extern "C" {
    pub fn jq_set_input_cb(
        arg1: *mut jq_state,
        arg2: jq_input_cb,
        arg3: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn jq_get_input_cb(
        arg1: *mut jq_state,
        arg2: *mut jq_input_cb,
        arg3: *mut *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn jq_set_debug_cb(arg1: *mut jq_state, arg2: jq_msg_cb, arg3: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn jq_get_debug_cb(
        arg1: *mut jq_state,
        arg2: *mut jq_msg_cb,
        arg3: *mut *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn jq_set_attrs(arg1: *mut jq_state, arg2: jv);
}
extern "C" {
    pub fn jq_get_attrs(arg1: *mut jq_state) -> jv;
}
extern "C" {
    pub fn jq_get_jq_origin(arg1: *mut jq_state) -> jv;
}
extern "C" {
    pub fn jq_get_prog_origin(arg1: *mut jq_state) -> jv;
}
extern "C" {
    pub fn jq_get_lib_dirs(arg1: *mut jq_state) -> jv;
}
extern "C" {
    pub fn jq_set_attr(arg1: *mut jq_state, arg2: jv, arg3: jv);
}
extern "C" {
    pub fn jq_get_attr(arg1: *mut jq_state, arg2: jv) -> jv;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jq_util_input_state {
    _unused: [u8; 0],
}
pub type jq_util_msg_cb = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: *const ::std::os::raw::c_char),
>;
extern "C" {
    pub fn jq_util_input_init(
        arg1: jq_util_msg_cb,
        arg2: *mut ::std::os::raw::c_void,
    ) -> *mut jq_util_input_state;
}
extern "C" {
    pub fn jq_util_input_set_parser(
        arg1: *mut jq_util_input_state,
        arg2: *mut jv_parser,
        arg3: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn jq_util_input_free(arg1: *mut *mut jq_util_input_state);
}
extern "C" {
    pub fn jq_util_input_add_input(
        arg1: *mut jq_util_input_state,
        arg2: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn jq_util_input_errors(arg1: *mut jq_util_input_state) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jq_util_input_next_input(arg1: *mut jq_util_input_state) -> jv;
}
extern "C" {
    pub fn jq_util_input_next_input_cb(
        arg1: *mut jq_state,
        arg2: *mut ::std::os::raw::c_void,
    ) -> jv;
}
extern "C" {
    pub fn jq_util_input_get_position(arg1: *mut jq_state) -> jv;
}
extern "C" {
    pub fn jq_util_input_get_current_filename(arg1: *mut jq_state) -> jv;
}
extern "C" {
    pub fn jq_util_input_get_current_line(arg1: *mut jq_state) -> jv;
}
extern "C" {
    pub fn jq_set_colors(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag() {
    assert_eq!(
        ::std::mem::size_of::<__va_list_tag>(),
        24usize,
        concat!("Size of: ", stringify!(__va_list_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<__va_list_tag>(),
        8usize,
        concat!("Alignment of ", stringify!(__va_list_tag))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).gp_offset as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(gp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).fp_offset as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(fp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).overflow_arg_area as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(overflow_arg_area)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).reg_save_area as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(reg_save_area)
        )
    );
}
