/* automatically generated by rust-bindgen 0.70.1 */

#![allow(non_camel_case_types)]
use crate::deps::*;
use libc::{c_char, c_int, c_uint, c_void, size_t, FILE};


pub const FEXPR_TYPE_BITS: u32 = 4;
pub const FEXPR_SMALL_SYMBOL_LEN: u32 = 7;
pub const FEXPR_LATEX_SMALL: u32 = 1;
pub const FEXPR_LATEX_LOGIC: u32 = 2;
#[repr(C)]
pub struct fexpr_struct {
    pub data: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fexpr_struct"][::std::mem::size_of::<fexpr_struct>() - 16usize];
    ["Alignment of fexpr_struct"][::std::mem::align_of::<fexpr_struct>() - 8usize];
    ["Offset of field: fexpr_struct::data"][::std::mem::offset_of!(fexpr_struct, data) - 0usize];
    ["Offset of field: fexpr_struct::alloc"][::std::mem::offset_of!(fexpr_struct, alloc) - 8usize];
};
impl Default for fexpr_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fexpr_t = [fexpr_struct; 1usize];
pub type fexpr_ptr = *mut fexpr_struct;
pub type fexpr_srcptr = *const fexpr_struct;
#[repr(C)]
pub struct fexpr_vec_struct {
    pub entries: *mut fexpr_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fexpr_vec_struct"][::std::mem::size_of::<fexpr_vec_struct>() - 24usize];
    ["Alignment of fexpr_vec_struct"][::std::mem::align_of::<fexpr_vec_struct>() - 8usize];
    ["Offset of field: fexpr_vec_struct::entries"]
        [::std::mem::offset_of!(fexpr_vec_struct, entries) - 0usize];
    ["Offset of field: fexpr_vec_struct::alloc"]
        [::std::mem::offset_of!(fexpr_vec_struct, alloc) - 8usize];
    ["Offset of field: fexpr_vec_struct::length"]
        [::std::mem::offset_of!(fexpr_vec_struct, length) - 16usize];
};
impl Default for fexpr_vec_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fexpr_vec_t = [fexpr_vec_struct; 1usize];
extern "C" {
    pub fn fexpr_equal_si(expr: *const fexpr_struct, c: mp_limb_signed_t) -> ::std::os::raw::c_int;
    pub fn fexpr_equal_ui(expr: *const fexpr_struct, c: mp_limb_t) -> ::std::os::raw::c_int;
    pub fn fexpr_hash(expr: *const fexpr_struct) -> mp_limb_t;
    pub fn fexpr_cmp_fast(a: *const fexpr_struct, b: *const fexpr_struct) -> ::std::os::raw::c_int;
    pub fn _fexpr_vec_sort_fast(vec: fexpr_ptr, len: mp_limb_signed_t);
    pub fn fexpr_is_neg_integer(expr: *const fexpr_struct) -> ::std::os::raw::c_int;
    pub fn fexpr_set_si(res: *mut fexpr_struct, c: mp_limb_signed_t);
    pub fn fexpr_set_ui(res: *mut fexpr_struct, c: mp_limb_t);
    pub fn fexpr_set_fmpz(res: *mut fexpr_struct, c: *const fmpz);
    pub fn fexpr_get_fmpz(c: *mut fmpz, x: *const fexpr_struct) -> ::std::os::raw::c_int;
    pub fn fexpr_set_fmpq(res: *mut fexpr_struct, x: *const fmpq);
    pub fn fexpr_set_symbol_str(res: *mut fexpr_struct, s: *const ::std::os::raw::c_char);
    pub fn fexpr_get_symbol_str(expr: *const fexpr_struct) -> *mut ::std::os::raw::c_char;
    pub fn fexpr_set_string(res: *mut fexpr_struct, s: *const ::std::os::raw::c_char);
    pub fn fexpr_get_string(expr: *const fexpr_struct) -> *mut ::std::os::raw::c_char;
    pub fn fexpr_depth(expr: *const fexpr_struct) -> mp_limb_signed_t;
    pub fn fexpr_num_leaves(expr: *const fexpr_struct) -> mp_limb_signed_t;
    pub fn fexpr_is_builtin_call(
        expr: *const fexpr_struct,
        i: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn fexpr_is_any_builtin_call(expr: *const fexpr_struct) -> ::std::os::raw::c_int;
    pub fn fexpr_func(res: *mut fexpr_struct, expr: *const fexpr_struct);
    pub fn fexpr_view_func(res: *mut fexpr_struct, expr: *const fexpr_struct);
    pub fn fexpr_arg(res: *mut fexpr_struct, expr: *const fexpr_struct, i: mp_limb_signed_t);
    pub fn fexpr_view_arg(res: *mut fexpr_struct, expr: *const fexpr_struct, i: mp_limb_signed_t);
    pub fn fexpr_call0(res: *mut fexpr_struct, f: *const fexpr_struct);
    pub fn fexpr_call1(res: *mut fexpr_struct, f: *const fexpr_struct, x1: *const fexpr_struct);
    pub fn fexpr_call2(
        res: *mut fexpr_struct,
        f: *const fexpr_struct,
        x1: *const fexpr_struct,
        x2: *const fexpr_struct,
    );
    pub fn fexpr_call3(
        res: *mut fexpr_struct,
        f: *const fexpr_struct,
        x1: *const fexpr_struct,
        x2: *const fexpr_struct,
        x3: *const fexpr_struct,
    );
    pub fn fexpr_call4(
        res: *mut fexpr_struct,
        f: *const fexpr_struct,
        x1: *const fexpr_struct,
        x2: *const fexpr_struct,
        x3: *const fexpr_struct,
        x4: *const fexpr_struct,
    );
    pub fn fexpr_call_vec(
        res: *mut fexpr_struct,
        f: *const fexpr_struct,
        args: fexpr_srcptr,
        len: mp_limb_signed_t,
    );
    pub fn fexpr_call_builtin1(res: *mut fexpr_struct, f: mp_limb_signed_t, x: *const fexpr_struct);
    pub fn fexpr_call_builtin2(
        res: *mut fexpr_struct,
        f: mp_limb_signed_t,
        x: *const fexpr_struct,
        y: *const fexpr_struct,
    );
    pub fn fexpr_contains(
        expr: *const fexpr_struct,
        x: *const fexpr_struct,
    ) -> ::std::os::raw::c_int;
    pub fn fexpr_replace(
        res: *mut fexpr_struct,
        expr: *const fexpr_struct,
        x: *const fexpr_struct,
        y: *const fexpr_struct,
    ) -> ::std::os::raw::c_int;
    pub fn fexpr_replace2(
        res: *mut fexpr_struct,
        expr: *const fexpr_struct,
        x1: *const fexpr_struct,
        y1: *const fexpr_struct,
        x2: *const fexpr_struct,
        y2: *const fexpr_struct,
    ) -> ::std::os::raw::c_int;
    pub fn fexpr_replace_vec(
        res: *mut fexpr_struct,
        expr: *const fexpr_struct,
        xs: *const fexpr_vec_struct,
        ys: *const fexpr_vec_struct,
    ) -> ::std::os::raw::c_int;
    pub fn fexpr_write(stream: *mut calcium_stream_struct, expr: *const fexpr_struct);
    pub fn fexpr_print(expr: *const fexpr_struct);
    pub fn fexpr_get_str(expr: *const fexpr_struct) -> *mut ::std::os::raw::c_char;
    pub fn fexpr_write_latex(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_print_latex(expr: *const fexpr_struct, flags: mp_limb_t);
    pub fn fexpr_get_str_latex(
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    ) -> *mut ::std::os::raw::c_char;
    pub fn fexpr_write_latex_call(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_subscript(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_subscript_call(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_infix(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_mul(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_div(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_neg_pos(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_add(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_sub(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_pow(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_exp(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_factorial(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_integral(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_sum_product(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_divsum(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_limit(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_derivative(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_logic(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_collection(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_matrix(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_simple(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_simple2(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_simple2_small(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_alg_structure(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_setop(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_cases(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_where(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_show_form(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_range(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_decimal(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_call1_optional_derivative(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_call2_optional_derivative(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_sub1_call1_optional_derivative(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_sub1_call2_optional_derivative(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_misc_special(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_write_latex_residue(
        out: *mut calcium_stream_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    );
    pub fn fexpr_set_arf(res: *mut fexpr_struct, x: *const arf_struct);
    pub fn fexpr_set_d(res: *mut fexpr_struct, x: f64);
    pub fn fexpr_set_re_im_d(res: *mut fexpr_struct, x: f64, y: f64);
    pub fn fexpr_neg(res: *mut fexpr_struct, a: *const fexpr_struct);
    pub fn fexpr_add(res: *mut fexpr_struct, a: *const fexpr_struct, b: *const fexpr_struct);
    pub fn fexpr_sub(res: *mut fexpr_struct, a: *const fexpr_struct, b: *const fexpr_struct);
    pub fn fexpr_mul(res: *mut fexpr_struct, a: *const fexpr_struct, b: *const fexpr_struct);
    pub fn fexpr_div(res: *mut fexpr_struct, a: *const fexpr_struct, b: *const fexpr_struct);
    pub fn fexpr_pow(res: *mut fexpr_struct, a: *const fexpr_struct, b: *const fexpr_struct);
    pub fn fexpr_is_arithmetic_operation(expr: *const fexpr_struct) -> ::std::os::raw::c_int;
    pub fn fexpr_arithmetic_nodes(nodes: *mut fexpr_vec_struct, expr: *const fexpr_struct);
    pub fn fexpr_get_fmpz_mpoly_q(
        res: *mut fmpz_mpoly_q_struct,
        expr: *const fexpr_struct,
        vars: *const fexpr_vec_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn fexpr_set_fmpz_mpoly(
        res: *mut fexpr_struct,
        poly: *const fmpz_mpoly_struct,
        vars: *const fexpr_vec_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fexpr_set_fmpz_mpoly_q(
        res: *mut fexpr_struct,
        frac: *const fmpz_mpoly_q_struct,
        vars: *const fexpr_vec_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fexpr_expanded_normal_form(
        res: *mut fexpr_struct,
        expr: *const fexpr_struct,
        flags: mp_limb_t,
    ) -> ::std::os::raw::c_int;
}
