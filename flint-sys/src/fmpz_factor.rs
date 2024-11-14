/* automatically generated by rust-bindgen 0.70.1 */

use libc::*;
use crate::deps::*;
use crate::flint::*;
use crate::fmpz_types::*;


#[repr(C)]
pub struct ecm_s {
    pub t: mp_ptr,
    pub u: mp_ptr,
    pub v: mp_ptr,
    pub w: mp_ptr,
    pub x: mp_ptr,
    pub z: mp_ptr,
    pub a24: mp_ptr,
    pub ninv: mp_ptr,
    pub one: mp_ptr,
    pub GCD_table: *mut libc::c_uchar,
    pub prime_table: *mut *mut libc::c_uchar,
    pub n_size: mp_limb_t,
    pub normbits: mp_limb_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ecm_s"][::std::mem::size_of::<ecm_s>() - 104usize];
    ["Alignment of ecm_s"][::std::mem::align_of::<ecm_s>() - 8usize];
    ["Offset of field: ecm_s::t"][::std::mem::offset_of!(ecm_s, t) - 0usize];
    ["Offset of field: ecm_s::u"][::std::mem::offset_of!(ecm_s, u) - 8usize];
    ["Offset of field: ecm_s::v"][::std::mem::offset_of!(ecm_s, v) - 16usize];
    ["Offset of field: ecm_s::w"][::std::mem::offset_of!(ecm_s, w) - 24usize];
    ["Offset of field: ecm_s::x"][::std::mem::offset_of!(ecm_s, x) - 32usize];
    ["Offset of field: ecm_s::z"][::std::mem::offset_of!(ecm_s, z) - 40usize];
    ["Offset of field: ecm_s::a24"][::std::mem::offset_of!(ecm_s, a24) - 48usize];
    ["Offset of field: ecm_s::ninv"][::std::mem::offset_of!(ecm_s, ninv) - 56usize];
    ["Offset of field: ecm_s::one"][::std::mem::offset_of!(ecm_s, one) - 64usize];
    ["Offset of field: ecm_s::GCD_table"][::std::mem::offset_of!(ecm_s, GCD_table) - 72usize];
    ["Offset of field: ecm_s::prime_table"][::std::mem::offset_of!(ecm_s, prime_table) - 80usize];
    ["Offset of field: ecm_s::n_size"][::std::mem::offset_of!(ecm_s, n_size) - 88usize];
    ["Offset of field: ecm_s::normbits"][::std::mem::offset_of!(ecm_s, normbits) - 96usize];
};
impl Default for ecm_s {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type ecm_t = [ecm_s; 1usize];
extern "C" {
    pub fn fmpz_factor_init(factor: *mut fmpz_factor_struct);
    pub fn fmpz_factor_clear(factor: *mut fmpz_factor_struct);
    pub fn _fmpz_factor_fit_length(factor: *mut fmpz_factor_struct, len: mp_limb_signed_t);
    pub fn _fmpz_factor_set_length(factor: *mut fmpz_factor_struct, newlen: mp_limb_signed_t);
    pub fn _fmpz_factor_append(factor: *mut fmpz_factor_struct, p: *const fmpz, exp: mp_limb_t);
    pub fn _fmpz_factor_append_ui(factor: *mut fmpz_factor_struct, p: mp_limb_t, exp: mp_limb_t);
    pub fn _fmpz_factor_concat(
        factor1: *mut fmpz_factor_struct,
        factor2: *mut fmpz_factor_struct,
        exp: mp_limb_t,
    );
    pub fn fmpz_factor_fprint(fs: *mut FILE, factor: *const fmpz_factor_struct) -> libc::c_int;
    pub fn fmpz_factor_print(factor: *const fmpz_factor_struct) -> libc::c_int;
    pub fn _fmpz_factor_extend_factor_ui(factor: *mut fmpz_factor_struct, n: mp_limb_t);
    pub fn fmpz_factor_trial_range(
        factor: *mut fmpz_factor_struct,
        n: *const fmpz,
        start: mp_limb_t,
        num_primes: mp_limb_t,
    ) -> libc::c_int;
    pub fn fmpz_factor_trial(
        factor: *mut fmpz_factor_struct,
        n: *const fmpz,
        num_primes: mp_limb_signed_t,
    ) -> libc::c_int;
    pub fn fmpz_factor_no_trial(factor: *mut fmpz_factor_struct, n: *const fmpz);
    pub fn fmpz_factor_si(factor: *mut fmpz_factor_struct, n: mp_limb_signed_t);
    pub fn fmpz_factor(factor: *mut fmpz_factor_struct, n: *const fmpz);
    pub fn fmpz_factor_smooth(
        factor: *mut fmpz_factor_struct,
        n: *const fmpz,
        bits: mp_limb_signed_t,
        proved: libc::c_int,
    ) -> libc::c_int;
    pub fn fmpz_factor_pp1(
        factor: *mut fmpz,
        n: *const fmpz,
        B1: mp_limb_t,
        B2_sqrt: mp_limb_t,
        c: mp_limb_t,
    ) -> libc::c_int;
    pub fn fmpz_factor_refine(res: *mut fmpz_factor_struct, f: *const fmpz_factor_struct);
    pub fn flint_mpn_sqr_and_add_a(
        y: mp_ptr,
        a: mp_ptr,
        n: mp_ptr,
        n_size: mp_limb_t,
        ninv: mp_ptr,
        normbits: mp_limb_t,
    );
    pub fn flint_mpn_factor_pollard_brent_single(
        factor: mp_ptr,
        n: mp_ptr,
        ninv: mp_ptr,
        a: mp_ptr,
        y: mp_ptr,
        n_size: mp_limb_t,
        normbits: mp_limb_t,
        max_iters: mp_limb_t,
    ) -> libc::c_int;
    pub fn fmpz_factor_pollard_brent_single(
        p_factor: *mut fmpz,
        n_in: *mut fmpz,
        yi: *mut fmpz,
        ai: *mut fmpz,
        max_iters: mp_limb_t,
    ) -> libc::c_int;
    pub fn fmpz_factor_pollard_brent(
        factor: *mut fmpz,
        state: *mut flint_rand_s,
        n: *mut fmpz,
        max_tries: mp_limb_t,
        max_iters: mp_limb_t,
    ) -> libc::c_int;
    pub fn fmpz_factor_expand_iterative(n: *mut fmpz, factor: *const fmpz_factor_struct);
    pub fn fmpz_factor_expand_multiexp(n: *mut fmpz, factor: *const fmpz_factor_struct);
    pub fn fmpz_factor_expand(n: *mut fmpz, factor: *const fmpz_factor_struct);
    pub fn fmpz_factor_euler_phi(res: *mut fmpz, fac: *const fmpz_factor_struct);
    pub fn fmpz_factor_moebius_mu(fac: *const fmpz_factor_struct) -> libc::c_int;
    pub fn fmpz_factor_divisor_sigma(res: *mut fmpz, k: mp_limb_t, fac: *const fmpz_factor_struct);
    pub fn fmpz_factor_ecm_init(ecm_inf: *mut ecm_s, sz: mp_limb_t);
    pub fn fmpz_factor_ecm_clear(ecm_inf: *mut ecm_s);
    pub fn fmpz_factor_ecm_addmod(a: mp_ptr, b: mp_ptr, c: mp_ptr, n: mp_ptr, n_size: mp_limb_t);
    pub fn fmpz_factor_ecm_submod(x: mp_ptr, a: mp_ptr, b: mp_ptr, n: mp_ptr, n_size: mp_limb_t);
    pub fn fmpz_factor_ecm_double(
        x: mp_ptr,
        z: mp_ptr,
        x0: mp_ptr,
        z0: mp_ptr,
        n: mp_ptr,
        ecm_inf: *mut ecm_s,
    );
    pub fn fmpz_factor_ecm_add(
        x: mp_ptr,
        z: mp_ptr,
        x1: mp_ptr,
        z1: mp_ptr,
        x2: mp_ptr,
        z2: mp_ptr,
        x0: mp_ptr,
        z0: mp_ptr,
        n: mp_ptr,
        ecm_inf: *mut ecm_s,
    );
    pub fn fmpz_factor_ecm_mul_montgomery_ladder(
        x: mp_ptr,
        z: mp_ptr,
        x0: mp_ptr,
        z0: mp_ptr,
        k: mp_limb_t,
        n: mp_ptr,
        ecm_inf: *mut ecm_s,
    );
    pub fn fmpz_factor_ecm_select_curve(
        f: mp_ptr,
        sig: mp_ptr,
        n: mp_ptr,
        ecm_inf: *mut ecm_s,
    ) -> libc::c_int;
    pub fn fmpz_factor_ecm_stage_I(
        f: mp_ptr,
        prime_array: *const mp_limb_t,
        num: mp_limb_t,
        B1: mp_limb_t,
        n: mp_ptr,
        ecm_inf: *mut ecm_s,
    ) -> libc::c_int;
    pub fn fmpz_factor_ecm_stage_II(
        f: mp_ptr,
        B1: mp_limb_t,
        B2: mp_limb_t,
        P: mp_limb_t,
        n: mp_ptr,
        ecm_inf: *mut ecm_s,
    ) -> libc::c_int;
    pub fn fmpz_factor_ecm(
        f: *mut fmpz,
        curves: mp_limb_t,
        B1: mp_limb_t,
        B2: mp_limb_t,
        state: *mut flint_rand_s,
        n_in: *const fmpz,
    ) -> libc::c_int;
    pub fn fmpz_factor_get_fmpz(
        z: *mut fmpz,
        factor: *const fmpz_factor_struct,
        i: mp_limb_signed_t,
    );
}
