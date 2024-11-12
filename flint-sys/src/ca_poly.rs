/* automatically generated by rust-bindgen 0.70.1 */

#![allow(non_camel_case_types)]
use crate::deps::*;
use libc::{c_char, c_int, c_uint, c_void, size_t, FILE};


#[repr(C)]
pub struct ca_poly_struct {
    pub coeffs: *mut ca_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ca_poly_struct"][::std::mem::size_of::<ca_poly_struct>() - 24usize];
    ["Alignment of ca_poly_struct"][::std::mem::align_of::<ca_poly_struct>() - 8usize];
    ["Offset of field: ca_poly_struct::coeffs"]
        [::std::mem::offset_of!(ca_poly_struct, coeffs) - 0usize];
    ["Offset of field: ca_poly_struct::alloc"]
        [::std::mem::offset_of!(ca_poly_struct, alloc) - 8usize];
    ["Offset of field: ca_poly_struct::length"]
        [::std::mem::offset_of!(ca_poly_struct, length) - 16usize];
};
impl Default for ca_poly_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type ca_poly_t = [ca_poly_struct; 1usize];
#[repr(C)]
pub struct ca_poly_vec_struct {
    pub entries: *mut ca_poly_struct,
    pub length: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ca_poly_vec_struct"][::std::mem::size_of::<ca_poly_vec_struct>() - 24usize];
    ["Alignment of ca_poly_vec_struct"][::std::mem::align_of::<ca_poly_vec_struct>() - 8usize];
    ["Offset of field: ca_poly_vec_struct::entries"]
        [::std::mem::offset_of!(ca_poly_vec_struct, entries) - 0usize];
    ["Offset of field: ca_poly_vec_struct::length"]
        [::std::mem::offset_of!(ca_poly_vec_struct, length) - 8usize];
    ["Offset of field: ca_poly_vec_struct::alloc"]
        [::std::mem::offset_of!(ca_poly_vec_struct, alloc) - 16usize];
};
impl Default for ca_poly_vec_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type ca_poly_vec_t = [ca_poly_vec_struct; 1usize];
extern "C" {
    pub fn ca_poly_init(poly: *mut ca_poly_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_poly_init2(poly: *mut ca_poly_struct, len: mp_limb_signed_t, ctx: *mut ca_ctx_struct);
    pub fn ca_poly_clear(poly: *mut ca_poly_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_poly_fit_length(
        poly: *mut ca_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_set_length(
        poly: *mut ca_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_normalise(poly: *mut ca_poly_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_poly_set_ca(poly: *mut ca_poly_struct, x: *const ca_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_poly_set_si(poly: *mut ca_poly_struct, x: mp_limb_signed_t, ctx: *mut ca_ctx_struct);
    pub fn ca_poly_set(
        res: *mut ca_poly_struct,
        src: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_set_fmpz_poly(
        res: *mut ca_poly_struct,
        src: *const fmpz_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_set_fmpq_poly(
        res: *mut ca_poly_struct,
        src: *const fmpq_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_transfer(
        res: *mut ca_poly_struct,
        res_ctx: *mut ca_ctx_struct,
        src: *const ca_poly_struct,
        src_ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_set_coeff_ca(
        poly: *mut ca_poly_struct,
        n: mp_limb_signed_t,
        x: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_randtest(
        poly: *mut ca_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        depth: mp_limb_signed_t,
        bits: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_randtest_rational(
        poly: *mut ca_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        bits: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_print(poly: *const ca_poly_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_poly_printn(
        poly: *const ca_poly_struct,
        digits: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_is_proper(
        poly: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn ca_poly_make_monic(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _ca_poly_reverse(
        res: ca_ptr,
        poly: ca_srcptr,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_reverse(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_check_equal(
        poly1: ca_srcptr,
        len1: mp_limb_signed_t,
        poly2: ca_srcptr,
        len2: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    ) -> truth_t;
    pub fn ca_poly_check_equal(
        poly1: *const ca_poly_struct,
        poly2: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> truth_t;
    pub fn ca_poly_check_is_zero(poly: *const ca_poly_struct, ctx: *mut ca_ctx_struct) -> truth_t;
    pub fn ca_poly_check_is_one(poly: *const ca_poly_struct, ctx: *mut ca_ctx_struct) -> truth_t;
    pub fn _ca_poly_shift_left(
        res: ca_ptr,
        poly: ca_srcptr,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_shift_left(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_shift_right(
        res: ca_ptr,
        poly: ca_srcptr,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_shift_right(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_neg(
        res: *mut ca_poly_struct,
        src: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_add(
        res: ca_ptr,
        poly1: ca_srcptr,
        len1: mp_limb_signed_t,
        poly2: ca_srcptr,
        len2: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_add(
        res: *mut ca_poly_struct,
        poly1: *const ca_poly_struct,
        poly2: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_sub(
        res: ca_ptr,
        poly1: ca_srcptr,
        len1: mp_limb_signed_t,
        poly2: ca_srcptr,
        len2: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_sub(
        res: *mut ca_poly_struct,
        poly1: *const ca_poly_struct,
        poly2: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_mul(
        C: ca_ptr,
        A: ca_srcptr,
        lenA: mp_limb_signed_t,
        B: ca_srcptr,
        lenB: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_mul(
        res: *mut ca_poly_struct,
        poly1: *const ca_poly_struct,
        poly2: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_mullow_same_nf(
        C: ca_ptr,
        A: ca_srcptr,
        Alen: mp_limb_signed_t,
        B: ca_srcptr,
        Blen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        K: *mut ca_field_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_mullow(
        C: ca_ptr,
        A: ca_srcptr,
        lenA: mp_limb_signed_t,
        B: ca_srcptr,
        lenB: mp_limb_signed_t,
        n: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_mullow(
        res: *mut ca_poly_struct,
        poly1: *const ca_poly_struct,
        poly2: *const ca_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_divrem_basecase(
        Q: ca_ptr,
        R: ca_ptr,
        A: ca_srcptr,
        lenA: mp_limb_signed_t,
        B: ca_srcptr,
        lenB: mp_limb_signed_t,
        invB: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_divrem_basecase(
        Q: *mut ca_poly_struct,
        R: *mut ca_poly_struct,
        A: *const ca_poly_struct,
        B: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _ca_poly_divrem(
        Q: ca_ptr,
        R: ca_ptr,
        A: ca_srcptr,
        lenA: mp_limb_signed_t,
        B: ca_srcptr,
        lenB: mp_limb_signed_t,
        invB: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_divrem(
        Q: *mut ca_poly_struct,
        R: *mut ca_poly_struct,
        A: *const ca_poly_struct,
        B: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn ca_poly_div(
        Q: *mut ca_poly_struct,
        A: *const ca_poly_struct,
        B: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn ca_poly_rem(
        R: *mut ca_poly_struct,
        A: *const ca_poly_struct,
        B: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _ca_poly_pow_ui_trunc(
        res: ca_ptr,
        f: ca_srcptr,
        flen: mp_limb_signed_t,
        exp: mp_limb_t,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_pow_ui_trunc(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        exp: mp_limb_t,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_pow_ui(
        res: ca_ptr,
        f: ca_srcptr,
        flen: mp_limb_signed_t,
        exp: mp_limb_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_pow_ui(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        exp: mp_limb_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_evaluate_horner(
        res: *mut ca_struct,
        f: ca_srcptr,
        len: mp_limb_signed_t,
        x: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_evaluate_horner(
        res: *mut ca_struct,
        f: *const ca_poly_struct,
        a: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_evaluate(
        res: *mut ca_struct,
        f: ca_srcptr,
        len: mp_limb_signed_t,
        x: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_evaluate(
        res: *mut ca_struct,
        f: *const ca_poly_struct,
        a: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_compose(
        res: ca_ptr,
        poly1: ca_srcptr,
        len1: mp_limb_signed_t,
        poly2: ca_srcptr,
        len2: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_compose(
        res: *mut ca_poly_struct,
        poly1: *const ca_poly_struct,
        poly2: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_derivative(
        res: ca_ptr,
        poly: ca_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_derivative(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_integral(
        res: ca_ptr,
        poly: ca_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_integral(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_gcd_euclidean(
        G: ca_ptr,
        A: ca_srcptr,
        lenA: mp_limb_signed_t,
        B: ca_srcptr,
        lenB: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn ca_poly_gcd_euclidean(
        G: *mut ca_poly_struct,
        A: *const ca_poly_struct,
        B: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _ca_poly_gcd(
        G: ca_ptr,
        A: ca_srcptr,
        lenA: mp_limb_signed_t,
        B: ca_srcptr,
        lenB: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn ca_poly_gcd(
        G: *mut ca_poly_struct,
        A: *const ca_poly_struct,
        B: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _ca_poly_inv_series(
        res: ca_ptr,
        f: ca_srcptr,
        flen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_inv_series(
        res: *mut ca_poly_struct,
        f: *const ca_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_div_series(
        res: ca_ptr,
        f: ca_srcptr,
        flen: mp_limb_signed_t,
        g: ca_srcptr,
        glen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_div_series(
        res: *mut ca_poly_struct,
        f: *const ca_poly_struct,
        g: *const ca_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_exp_series(
        res: ca_ptr,
        f: ca_srcptr,
        flen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_exp_series(
        res: *mut ca_poly_struct,
        f: *const ca_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_log_series(
        res: ca_ptr,
        f: ca_srcptr,
        flen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_log_series(
        res: *mut ca_poly_struct,
        f: *const ca_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_vec_init(len: mp_limb_signed_t, ctx: *mut ca_ctx_struct)
        -> *mut ca_poly_struct;
    pub fn ca_poly_vec_init(
        res: *mut ca_poly_vec_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_vec_fit_length(
        vec: *mut ca_poly_vec_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_vec_set_length(
        vec: *mut ca_poly_vec_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_vec_clear(
        v: *mut ca_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_vec_clear(vec: *mut ca_poly_vec_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_poly_vec_append(
        vec: *mut ca_poly_vec_struct,
        f: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_factor_squarefree(
        c: *mut ca_struct,
        fac: *mut ca_poly_vec_struct,
        exp: *mut mp_limb_t,
        F: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn ca_poly_squarefree_part(
        res: *mut ca_poly_struct,
        poly: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _ca_poly_set_roots(
        poly: ca_ptr,
        roots: ca_srcptr,
        exp: *const mp_limb_t,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_poly_set_roots(
        poly: *mut ca_poly_struct,
        roots: *mut ca_vec_struct,
        exp: *const mp_limb_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_poly_roots(
        roots: ca_ptr,
        poly: ca_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn ca_poly_roots(
        roots: *mut ca_vec_struct,
        exp: *mut mp_limb_t,
        poly: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
