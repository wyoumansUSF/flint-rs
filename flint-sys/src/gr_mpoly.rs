/* automatically generated by rust-bindgen 0.70.1 */

#![allow(non_camel_case_types)]
use crate::deps::*;
use libc::{c_char, c_int, c_uint, c_void, size_t, FILE};


#[repr(C)]
pub struct gr_mpoly_struct {
    pub coeffs: gr_ptr,
    pub exps: *mut mp_limb_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_t,
    pub coeffs_alloc: mp_limb_signed_t,
    pub exps_alloc: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of gr_mpoly_struct"][::std::mem::size_of::<gr_mpoly_struct>() - 48usize];
    ["Alignment of gr_mpoly_struct"][::std::mem::align_of::<gr_mpoly_struct>() - 8usize];
    ["Offset of field: gr_mpoly_struct::coeffs"]
        [::std::mem::offset_of!(gr_mpoly_struct, coeffs) - 0usize];
    ["Offset of field: gr_mpoly_struct::exps"]
        [::std::mem::offset_of!(gr_mpoly_struct, exps) - 8usize];
    ["Offset of field: gr_mpoly_struct::length"]
        [::std::mem::offset_of!(gr_mpoly_struct, length) - 16usize];
    ["Offset of field: gr_mpoly_struct::bits"]
        [::std::mem::offset_of!(gr_mpoly_struct, bits) - 24usize];
    ["Offset of field: gr_mpoly_struct::coeffs_alloc"]
        [::std::mem::offset_of!(gr_mpoly_struct, coeffs_alloc) - 32usize];
    ["Offset of field: gr_mpoly_struct::exps_alloc"]
        [::std::mem::offset_of!(gr_mpoly_struct, exps_alloc) - 40usize];
};
impl Default for gr_mpoly_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type gr_mpoly_t = [gr_mpoly_struct; 1usize];
extern "C" {
    pub fn gr_mpoly_init3(
        A: *mut gr_mpoly_struct,
        alloc: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    );
    pub fn gr_mpoly_init2(
        A: *mut gr_mpoly_struct,
        alloc: mp_limb_signed_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    );
    pub fn _gr_mpoly_fit_length(
        coeffs: *mut gr_ptr,
        coeffs_alloc: *mut mp_limb_signed_t,
        exps: *mut *mut mp_limb_t,
        exps_alloc: *mut mp_limb_signed_t,
        N: mp_limb_signed_t,
        length: mp_limb_signed_t,
        cctx: *mut gr_ctx_struct,
    );
    pub fn gr_mpoly_fit_length(
        A: *mut gr_mpoly_struct,
        len: mp_limb_signed_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    );
    pub fn gr_mpoly_fit_bits(
        A: *mut gr_mpoly_struct,
        bits: mp_limb_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    );
    pub fn gr_mpoly_fit_length_fit_bits(
        A: *mut gr_mpoly_struct,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    );
    pub fn gr_mpoly_fit_length_reset_bits(
        A: *mut gr_mpoly_struct,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    );
    pub fn gr_mpoly_set(
        A: *mut gr_mpoly_struct,
        B: *const gr_mpoly_struct,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_gen(
        A: *mut gr_mpoly_struct,
        var: mp_limb_signed_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_is_gen(
        A: *const gr_mpoly_struct,
        var: mp_limb_signed_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> truth_t;
    pub fn gr_mpoly_equal(
        A: *const gr_mpoly_struct,
        B: *const gr_mpoly_struct,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> truth_t;
    pub fn _gr_mpoly_push_exp_ui(
        A: *mut gr_mpoly_struct,
        exp: *const mp_limb_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    );
    pub fn gr_mpoly_push_term_scalar_ui(
        A: *mut gr_mpoly_struct,
        c: gr_srcptr,
        exp: *const mp_limb_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _gr_mpoly_push_exp_fmpz(
        A: *mut gr_mpoly_struct,
        exp: *const fmpz,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    );
    pub fn gr_mpoly_push_term_scalar_fmpz(
        A: *mut gr_mpoly_struct,
        c: gr_srcptr,
        exp: *const fmpz,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_sort_terms(
        A: *mut gr_mpoly_struct,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    );
    pub fn gr_mpoly_combine_like_terms(
        A: *mut gr_mpoly_struct,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_is_canonical(
        A: *const gr_mpoly_struct,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> truth_t;
    pub fn gr_mpoly_assert_canonical(
        A: *const gr_mpoly_struct,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    );
    pub fn gr_mpoly_randtest_bits(
        A: *mut gr_mpoly_struct,
        state: *mut flint_rand_s,
        length: mp_limb_signed_t,
        exp_bits: mp_limb_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_write_pretty(
        out: *mut gr_stream_struct,
        A: *const gr_mpoly_struct,
        x_in: *mut *const ::std::os::raw::c_char,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_print_pretty(
        A: *const gr_mpoly_struct,
        x_in: *mut *const ::std::os::raw::c_char,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_set_scalar(
        A: *mut gr_mpoly_struct,
        c: gr_srcptr,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_set_ui(
        A: *mut gr_mpoly_struct,
        c: mp_limb_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_set_si(
        A: *mut gr_mpoly_struct,
        c: mp_limb_signed_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_set_fmpz(
        A: *mut gr_mpoly_struct,
        c: *const fmpz,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_set_fmpq(
        A: *mut gr_mpoly_struct,
        c: *const fmpq,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_get_coeff_scalar_fmpz(
        c: gr_ptr,
        A: *const gr_mpoly_struct,
        exp: *const fmpz,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_get_coeff_scalar_ui(
        c: gr_ptr,
        A: *const gr_mpoly_struct,
        exp: *const mp_limb_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_set_coeff_scalar_fmpz(
        A: *mut gr_mpoly_struct,
        c: gr_srcptr,
        exp: *const fmpz,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_set_coeff_ui_fmpz(
        A: *mut gr_mpoly_struct,
        c: mp_limb_t,
        exp: *const fmpz,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_set_coeff_si_fmpz(
        A: *mut gr_mpoly_struct,
        c: mp_limb_signed_t,
        exp: *const fmpz,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_set_coeff_fmpz_fmpz(
        A: *mut gr_mpoly_struct,
        c: *const fmpz,
        exp: *const fmpz,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_set_coeff_fmpq_fmpz(
        A: *mut gr_mpoly_struct,
        c: *const fmpq,
        exp: *const fmpz,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_set_coeff_scalar_ui(
        poly: *mut gr_mpoly_struct,
        c: gr_srcptr,
        exp: *const mp_limb_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_set_coeff_ui_ui(
        A: *mut gr_mpoly_struct,
        c: mp_limb_t,
        exp: *const mp_limb_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_set_coeff_si_ui(
        A: *mut gr_mpoly_struct,
        c: mp_limb_signed_t,
        exp: *const mp_limb_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_set_coeff_fmpz_ui(
        A: *mut gr_mpoly_struct,
        c: *const fmpz,
        exp: *const mp_limb_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_set_coeff_fmpq_ui(
        A: *mut gr_mpoly_struct,
        c: *const fmpq,
        exp: *const mp_limb_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_neg(
        A: *mut gr_mpoly_struct,
        B: *const gr_mpoly_struct,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_add(
        A: *mut gr_mpoly_struct,
        B: *const gr_mpoly_struct,
        C: *const gr_mpoly_struct,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_sub(
        A: *mut gr_mpoly_struct,
        B: *const gr_mpoly_struct,
        C: *const gr_mpoly_struct,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_mul(
        poly1: *mut gr_mpoly_struct,
        poly2: *const gr_mpoly_struct,
        poly3: *const gr_mpoly_struct,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_mul_johnson(
        poly1: *mut gr_mpoly_struct,
        poly2: *const gr_mpoly_struct,
        poly3: *const gr_mpoly_struct,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_mul_monomial(
        A: *mut gr_mpoly_struct,
        B: *const gr_mpoly_struct,
        C: *const gr_mpoly_struct,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_mul_scalar(
        A: *mut gr_mpoly_struct,
        B: *const gr_mpoly_struct,
        c: gr_srcptr,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_mul_si(
        A: *mut gr_mpoly_struct,
        B: *const gr_mpoly_struct,
        c: mp_limb_signed_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_mul_ui(
        A: *mut gr_mpoly_struct,
        B: *const gr_mpoly_struct,
        c: mp_limb_t,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_mul_fmpz(
        A: *mut gr_mpoly_struct,
        B: *const gr_mpoly_struct,
        c: *const fmpz,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_mpoly_mul_fmpq(
        A: *mut gr_mpoly_struct,
        B: *const gr_mpoly_struct,
        c: *const fmpq,
        mctx: *const mpoly_ctx_struct,
        cctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
