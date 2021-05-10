use crate::bindings::jq::__va_list_tag;
use crate::bindings::FILE;

pub const jv_kind_JV_KIND_INVALID: jv_kind = 0;
pub const jv_kind_JV_KIND_NULL: jv_kind = 1;
pub const jv_kind_JV_KIND_FALSE: jv_kind = 2;
pub const jv_kind_JV_KIND_TRUE: jv_kind = 3;
pub const jv_kind_JV_KIND_NUMBER: jv_kind = 4;
pub const jv_kind_JV_KIND_STRING: jv_kind = 5;
pub const jv_kind_JV_KIND_ARRAY: jv_kind = 6;
pub const jv_kind_JV_KIND_OBJECT: jv_kind = 7;
pub type jv_kind = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jv_refcnt {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jv {
    pub kind_flags: ::std::os::raw::c_uchar,
    pub pad_: ::std::os::raw::c_uchar,
    pub offset: ::std::os::raw::c_ushort,
    pub size: ::std::os::raw::c_int,
    pub u: jv__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union jv__bindgen_ty_1 {
    pub ptr: *mut jv_refcnt,
    pub number: f64,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_jv__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<jv__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(jv__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<jv__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(jv__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jv__bindgen_ty_1>())).ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jv__bindgen_ty_1),
            "::",
            stringify!(ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jv__bindgen_ty_1>())).number as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jv__bindgen_ty_1),
            "::",
            stringify!(number)
        )
    );
}
#[test]
fn bindgen_test_layout_jv() {
    assert_eq!(
        ::std::mem::size_of::<jv>(),
        16usize,
        concat!("Size of: ", stringify!(jv))
    );
    assert_eq!(
        ::std::mem::align_of::<jv>(),
        8usize,
        concat!("Alignment of ", stringify!(jv))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jv>())).kind_flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jv),
            "::",
            stringify!(kind_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jv>())).pad_ as *const _ as usize },
        1usize,
        concat!("Offset of field: ", stringify!(jv), "::", stringify!(pad_))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jv>())).offset as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(jv),
            "::",
            stringify!(offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jv>())).size as *const _ as usize },
        4usize,
        concat!("Offset of field: ", stringify!(jv), "::", stringify!(size))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jv>())).u as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(jv), "::", stringify!(u))
    );
}
extern "C" {
    pub fn jv_get_kind(arg1: jv) -> jv_kind;
}
extern "C" {
    pub fn jv_kind_name(arg1: jv_kind) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn jv_copy(arg1: jv) -> jv;
}
extern "C" {
    pub fn jv_free(arg1: jv);
}
extern "C" {
    pub fn jv_get_refcnt(arg1: jv) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jv_equal(arg1: jv, arg2: jv) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jv_identical(arg1: jv, arg2: jv) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jv_contains(arg1: jv, arg2: jv) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jv_invalid() -> jv;
}
extern "C" {
    pub fn jv_invalid_with_msg(arg1: jv) -> jv;
}
extern "C" {
    pub fn jv_invalid_get_msg(arg1: jv) -> jv;
}
extern "C" {
    pub fn jv_invalid_has_msg(arg1: jv) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jv_null() -> jv;
}
extern "C" {
    pub fn jv_true() -> jv;
}
extern "C" {
    pub fn jv_false() -> jv;
}
extern "C" {
    pub fn jv_bool(arg1: ::std::os::raw::c_int) -> jv;
}
extern "C" {
    pub fn jv_number(arg1: f64) -> jv;
}
extern "C" {
    pub fn jv_number_value(arg1: jv) -> f64;
}
extern "C" {
    pub fn jv_is_integer(arg1: jv) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jv_array() -> jv;
}
extern "C" {
    pub fn jv_array_sized(arg1: ::std::os::raw::c_int) -> jv;
}
extern "C" {
    pub fn jv_array_length(arg1: jv) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jv_array_get(arg1: jv, arg2: ::std::os::raw::c_int) -> jv;
}
extern "C" {
    pub fn jv_array_set(arg1: jv, arg2: ::std::os::raw::c_int, arg3: jv) -> jv;
}
extern "C" {
    pub fn jv_array_append(arg1: jv, arg2: jv) -> jv;
}
extern "C" {
    pub fn jv_array_concat(arg1: jv, arg2: jv) -> jv;
}
extern "C" {
    pub fn jv_array_slice(arg1: jv, arg2: ::std::os::raw::c_int, arg3: ::std::os::raw::c_int)
        -> jv;
}
extern "C" {
    pub fn jv_array_indexes(arg1: jv, arg2: jv) -> jv;
}
extern "C" {
    pub fn jv_string(arg1: *const ::std::os::raw::c_char) -> jv;
}
extern "C" {
    pub fn jv_string_sized(arg1: *const ::std::os::raw::c_char, arg2: ::std::os::raw::c_int) -> jv;
}
extern "C" {
    pub fn jv_string_empty(len: ::std::os::raw::c_int) -> jv;
}
extern "C" {
    pub fn jv_string_length_bytes(arg1: jv) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jv_string_length_codepoints(arg1: jv) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jv_string_hash(arg1: jv) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn jv_string_value(arg1: jv) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn jv_string_indexes(j: jv, k: jv) -> jv;
}
extern "C" {
    pub fn jv_string_slice(j: jv, start: ::std::os::raw::c_int, end: ::std::os::raw::c_int) -> jv;
}
extern "C" {
    pub fn jv_string_concat(arg1: jv, arg2: jv) -> jv;
}
extern "C" {
    pub fn jv_string_vfmt(arg1: *const ::std::os::raw::c_char, arg2: *mut __va_list_tag) -> jv;
}
extern "C" {
    pub fn jv_string_fmt(arg1: *const ::std::os::raw::c_char, ...) -> jv;
}
extern "C" {
    pub fn jv_string_append_codepoint(a: jv, c: u32) -> jv;
}
extern "C" {
    pub fn jv_string_append_buf(
        a: jv,
        buf: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> jv;
}
extern "C" {
    pub fn jv_string_append_str(a: jv, str: *const ::std::os::raw::c_char) -> jv;
}
extern "C" {
    pub fn jv_string_split(j: jv, sep: jv) -> jv;
}
extern "C" {
    pub fn jv_string_explode(j: jv) -> jv;
}
extern "C" {
    pub fn jv_string_implode(j: jv) -> jv;
}
extern "C" {
    pub fn jv_object() -> jv;
}
extern "C" {
    pub fn jv_object_get(object: jv, key: jv) -> jv;
}
extern "C" {
    pub fn jv_object_has(object: jv, key: jv) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jv_object_set(object: jv, key: jv, value: jv) -> jv;
}
extern "C" {
    pub fn jv_object_delete(object: jv, key: jv) -> jv;
}
extern "C" {
    pub fn jv_object_length(object: jv) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jv_object_merge(arg1: jv, arg2: jv) -> jv;
}
extern "C" {
    pub fn jv_object_merge_recursive(arg1: jv, arg2: jv) -> jv;
}
extern "C" {
    pub fn jv_object_iter(arg1: jv) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jv_object_iter_next(arg1: jv, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jv_object_iter_valid(arg1: jv, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jv_object_iter_key(arg1: jv, arg2: ::std::os::raw::c_int) -> jv;
}
extern "C" {
    pub fn jv_object_iter_value(arg1: jv, arg2: ::std::os::raw::c_int) -> jv;
}
pub const jv_print_flags_JV_PRINT_PRETTY: jv_print_flags = 1;
pub const jv_print_flags_JV_PRINT_ASCII: jv_print_flags = 2;
pub const jv_print_flags_JV_PRINT_COLOR: jv_print_flags = 4;
pub const jv_print_flags_JV_PRINT_COLOUR: jv_print_flags = 4;
pub const jv_print_flags_JV_PRINT_SORTED: jv_print_flags = 8;
pub const jv_print_flags_JV_PRINT_INVALID: jv_print_flags = 16;
pub const jv_print_flags_JV_PRINT_REFCOUNT: jv_print_flags = 32;
pub const jv_print_flags_JV_PRINT_TAB: jv_print_flags = 64;
pub const jv_print_flags_JV_PRINT_ISATTY: jv_print_flags = 128;
pub const jv_print_flags_JV_PRINT_SPACE0: jv_print_flags = 256;
pub const jv_print_flags_JV_PRINT_SPACE1: jv_print_flags = 512;
pub const jv_print_flags_JV_PRINT_SPACE2: jv_print_flags = 1024;
pub type jv_print_flags = u32;
extern "C" {
    pub fn jv_dumpf(arg1: jv, f: *mut FILE, flags: ::std::os::raw::c_int);
}
extern "C" {
    pub fn jv_dump(arg1: jv, flags: ::std::os::raw::c_int);
}
extern "C" {
    pub fn jv_show(arg1: jv, flags: ::std::os::raw::c_int);
}
extern "C" {
    pub fn jv_dump_string(arg1: jv, flags: ::std::os::raw::c_int) -> jv;
}
extern "C" {
    pub fn jv_dump_string_trunc(
        x: jv,
        outbuf: *mut ::std::os::raw::c_char,
        bufsize: usize,
    ) -> *mut ::std::os::raw::c_char;
}
pub const JV_PARSE_SEQ: _bindgen_ty_1 = 1;
pub const JV_PARSE_STREAMING: _bindgen_ty_1 = 2;
pub const JV_PARSE_STREAM_ERRORS: _bindgen_ty_1 = 4;
pub type _bindgen_ty_1 = u32;
extern "C" {
    pub fn jv_parse(string: *const ::std::os::raw::c_char) -> jv;
}
extern "C" {
    pub fn jv_parse_sized(
        string: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
    ) -> jv;
}
pub type jv_nomem_handler_f =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>;
extern "C" {
    pub fn jv_nomem_handler(arg1: jv_nomem_handler_f, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn jv_load_file(arg1: *const ::std::os::raw::c_char, arg2: ::std::os::raw::c_int) -> jv;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jv_parser {
    _unused: [u8; 0],
}
extern "C" {
    pub fn jv_parser_new(arg1: ::std::os::raw::c_int) -> *mut jv_parser;
}
extern "C" {
    pub fn jv_parser_set_buf(
        arg1: *mut jv_parser,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn jv_parser_remaining(arg1: *mut jv_parser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jv_parser_next(arg1: *mut jv_parser) -> jv;
}
extern "C" {
    pub fn jv_parser_free(arg1: *mut jv_parser);
}
extern "C" {
    pub fn jv_get(arg1: jv, arg2: jv) -> jv;
}
extern "C" {
    pub fn jv_set(arg1: jv, arg2: jv, arg3: jv) -> jv;
}
extern "C" {
    pub fn jv_has(arg1: jv, arg2: jv) -> jv;
}
extern "C" {
    pub fn jv_setpath(arg1: jv, arg2: jv, arg3: jv) -> jv;
}
extern "C" {
    pub fn jv_getpath(arg1: jv, arg2: jv) -> jv;
}
extern "C" {
    pub fn jv_delpaths(arg1: jv, arg2: jv) -> jv;
}
extern "C" {
    pub fn jv_keys(arg1: jv) -> jv;
}
extern "C" {
    pub fn jv_keys_unsorted(arg1: jv) -> jv;
}
extern "C" {
    pub fn jv_cmp(arg1: jv, arg2: jv) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jv_group(arg1: jv, arg2: jv) -> jv;
}
extern "C" {
    pub fn jv_sort(arg1: jv, arg2: jv) -> jv;
}
