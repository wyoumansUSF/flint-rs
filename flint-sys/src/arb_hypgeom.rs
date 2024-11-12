/* automatically generated by rust-bindgen 0.70.1 */

#![allow(non_camel_case_types)]
use crate::deps::*;
use libc::{c_char, c_int, c_uint, c_void, size_t, FILE};


pub const ARB_HYPGEOM_GAMMA_TAB_NUM: u32 = 536;
pub const ARB_HYPGEOM_GAMMA_TAB_PREC: u32 = 3456;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct arb_hypgeom_gamma_coeff_t {
    pub exp: ::std::os::raw::c_short,
    pub tab_pos: ::std::os::raw::c_short,
    pub nlimbs: ::std::os::raw::c_char,
    pub negative: ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of arb_hypgeom_gamma_coeff_t"]
        [::std::mem::size_of::<arb_hypgeom_gamma_coeff_t>() - 6usize];
    ["Alignment of arb_hypgeom_gamma_coeff_t"]
        [::std::mem::align_of::<arb_hypgeom_gamma_coeff_t>() - 2usize];
    ["Offset of field: arb_hypgeom_gamma_coeff_t::exp"]
        [::std::mem::offset_of!(arb_hypgeom_gamma_coeff_t, exp) - 0usize];
    ["Offset of field: arb_hypgeom_gamma_coeff_t::tab_pos"]
        [::std::mem::offset_of!(arb_hypgeom_gamma_coeff_t, tab_pos) - 2usize];
    ["Offset of field: arb_hypgeom_gamma_coeff_t::nlimbs"]
        [::std::mem::offset_of!(arb_hypgeom_gamma_coeff_t, nlimbs) - 4usize];
    ["Offset of field: arb_hypgeom_gamma_coeff_t::negative"]
        [::std::mem::offset_of!(arb_hypgeom_gamma_coeff_t, negative) - 5usize];
};
extern "C" {
    pub fn _arb_hypgeom_rising_coeffs_1(c: *mut mp_limb_t, k: mp_limb_t, l: mp_limb_signed_t);
    pub fn _arb_hypgeom_rising_coeffs_2(c: *mut mp_limb_t, k: mp_limb_t, l: mp_limb_signed_t);
    pub fn _arb_hypgeom_rising_coeffs_fmpz(c: *mut fmpz, k: mp_limb_t, l: mp_limb_signed_t);
    pub fn arb_hypgeom_rising_ui_forward(
        res: *mut arb_struct,
        x: *const arb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_rising_ui_rs(
        res: *mut arb_struct,
        x: *const arb_struct,
        n: mp_limb_t,
        m: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_rising_ui_bs(
        res: *mut arb_struct,
        x: *const arb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_rising_ui_rec(
        res: *mut arb_struct,
        x: *const arb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_rising_ui(
        y: *mut arb_struct,
        x: *const arb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_rising(
        y: *mut arb_struct,
        x: *const arb_struct,
        n: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_rising_ui_jet_powsum(
        res: arb_ptr,
        x: *const arb_struct,
        n: mp_limb_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_rising_ui_jet_rs(
        res: arb_ptr,
        x: *const arb_struct,
        n: mp_limb_t,
        m: mp_limb_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_rising_ui_jet_bs(
        res: arb_ptr,
        x: *const arb_struct,
        n: mp_limb_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_rising_ui_jet(
        res: arb_ptr,
        x: *const arb_struct,
        n: mp_limb_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_gamma_stirling_term_bounds(
        bound: *mut mp_limb_signed_t,
        zinv: *const mag_struct,
        N: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_gamma_stirling_sum_horner(
        s: *mut arb_struct,
        z: *const arb_struct,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_gamma_stirling_sum_improved(
        s: *mut arb_struct,
        z: *const arb_struct,
        N: mp_limb_signed_t,
        K: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub static mut arb_hypgeom_gamma_coeffs: [arb_hypgeom_gamma_coeff_t; 536usize];
    pub fn _arb_hypgeom_gamma_coeff_shallow(
        c: *mut arf_struct,
        err: *mut mag_struct,
        i: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn arb_hypgeom_gamma_stirling(
        res: *mut arb_struct,
        x: *const arb_struct,
        reciprocal: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_gamma_taylor(
        res: *mut arb_struct,
        x: *const arb_struct,
        reciprocal: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn arb_hypgeom_gamma(y: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_hypgeom_rgamma(y: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_hypgeom_lgamma(y: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_hypgeom_gamma_fmpq(y: *mut arb_struct, x: *const fmpq, prec: mp_limb_signed_t);
    pub fn arb_hypgeom_gamma_fmpz(y: *mut arb_struct, x: *const fmpz, prec: mp_limb_signed_t);
    pub fn arb_hypgeom_pfq(
        res: *mut arb_struct,
        a: arb_srcptr,
        p: mp_limb_signed_t,
        b: arb_srcptr,
        q: mp_limb_signed_t,
        z: *const arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_0f1(
        res: *mut arb_struct,
        a: *const arb_struct,
        z: *const arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_m(
        res: *mut arb_struct,
        a: *const arb_struct,
        b: *const arb_struct,
        z: *const arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_1f1(
        res: *mut arb_struct,
        a: *const arb_struct,
        b: *const arb_struct,
        z: *const arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_u(
        res: *mut arb_struct,
        a: *const arb_struct,
        b: *const arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_2f1(
        res: *mut arb_struct,
        a: *const arb_struct,
        b: *const arb_struct,
        c: *const arb_struct,
        z: *const arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_1f1_integration(
        res: *mut arb_struct,
        a: *const arb_struct,
        b: *const arb_struct,
        z: *const arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_u_integration(
        res: *mut arb_struct,
        a: *const arb_struct,
        b: *const arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_2f1_integration(
        res: *mut arb_struct,
        a: *const arb_struct,
        b: *const arb_struct,
        c: *const arb_struct,
        z: *const arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_erf(res: *mut arb_struct, z: *const arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_hypgeom_erf_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_erf_series(
        g: *mut arb_poly_struct,
        h: *const arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_erfc(res: *mut arb_struct, z: *const arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_hypgeom_erfc_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_erfc_series(
        g: *mut arb_poly_struct,
        h: *const arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_erfi(res: *mut arb_struct, z: *const arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_hypgeom_erfi_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_erfi_series(
        g: *mut arb_poly_struct,
        h: *const arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_erfinv(res: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_hypgeom_erfcinv(res: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_hypgeom_fresnel(
        res1: *mut arb_struct,
        res2: *mut arb_struct,
        z: *const arb_struct,
        normalized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_fresnel_series(
        s: arb_ptr,
        c: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        normalized: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_fresnel_series(
        s: *mut arb_poly_struct,
        c: *mut arb_poly_struct,
        h: *const arb_poly_struct,
        normalized: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_ei(res: *mut arb_struct, z: *const arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_hypgeom_ei_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_ei_series(
        g: *mut arb_poly_struct,
        h: *const arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_si_asymp(
        res: *mut arb_struct,
        z: *const arb_struct,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_si_1f2(
        res: *mut arb_struct,
        z: *const arb_struct,
        N: mp_limb_signed_t,
        wp: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_si(res: *mut arb_struct, z: *const arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_hypgeom_si_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_si_series(
        g: *mut arb_poly_struct,
        h: *const arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_ci_asymp(
        res: *mut arb_struct,
        z: *const arb_struct,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_ci_2f3(
        res: *mut arb_struct,
        z: *const arb_struct,
        N: mp_limb_signed_t,
        wp: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_ci(res: *mut arb_struct, z: *const arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_hypgeom_ci_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_ci_series(
        g: *mut arb_poly_struct,
        h: *const arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_shi(res: *mut arb_struct, z: *const arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_hypgeom_shi_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_shi_series(
        g: *mut arb_poly_struct,
        h: *const arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_chi(res: *mut arb_struct, z: *const arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_hypgeom_chi_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_chi_series(
        g: *mut arb_poly_struct,
        h: *const arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_li(
        res: *mut arb_struct,
        z: *const arb_struct,
        offset: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_li_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        offset: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_li_series(
        g: *mut arb_poly_struct,
        h: *const arb_poly_struct,
        offset: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_bessel_j(
        res: *mut arb_struct,
        nu: *const arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_bessel_y(
        res: *mut arb_struct,
        nu: *const arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_bessel_jy(
        res1: *mut arb_struct,
        res2: *mut arb_struct,
        nu: *const arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_bessel_i(
        res: *mut arb_struct,
        nu: *const arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_bessel_k(
        res: *mut arb_struct,
        nu: *const arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_bessel_i_scaled(
        res: *mut arb_struct,
        nu: *const arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_bessel_k_scaled(
        res: *mut arb_struct,
        nu: *const arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_bessel_i_integration(
        res: *mut arb_struct,
        nu: *const arb_struct,
        z: *const arb_struct,
        scaled: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_bessel_k_integration(
        res: *mut arb_struct,
        nu: *const arb_struct,
        z: *const arb_struct,
        scaled: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_airy(
        ai: *mut arb_struct,
        aip: *mut arb_struct,
        bi: *mut arb_struct,
        bip: *mut arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_airy_jet(
        ai: arb_ptr,
        bi: arb_ptr,
        z: *const arb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_airy_series(
        ai: *mut arb_poly_struct,
        ai_prime: *mut arb_poly_struct,
        bi: *mut arb_poly_struct,
        bi_prime: *mut arb_poly_struct,
        z: *const arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_airy_series(
        ai: arb_ptr,
        ai_prime: arb_ptr,
        bi: arb_ptr,
        bi_prime: arb_ptr,
        z: arb_srcptr,
        zlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_airy_zero(
        ai: *mut arb_struct,
        aip: *mut arb_struct,
        bi: *mut arb_struct,
        bip: *mut arb_struct,
        n: *const fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_coulomb(
        F: *mut arb_struct,
        G: *mut arb_struct,
        l: *const arb_struct,
        eta: *const arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_coulomb_jet(
        F: arb_ptr,
        G: arb_ptr,
        l: *const arb_struct,
        eta: *const arb_struct,
        z: *const arb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_coulomb_series(
        F: arb_ptr,
        G: arb_ptr,
        l: *const arb_struct,
        eta: *const arb_struct,
        z: arb_srcptr,
        zlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_coulomb_series(
        F: *mut arb_poly_struct,
        G: *mut arb_poly_struct,
        l: *const arb_struct,
        eta: *const arb_struct,
        z: *const arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_expint(
        res: *mut arb_struct,
        s: *const arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_gamma_lower(
        res: *mut arb_struct,
        s: *const arb_struct,
        z: *const arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_gamma_lower_series(
        g: arb_ptr,
        s: *const arb_struct,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        regularized: ::std::os::raw::c_int,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_gamma_lower_series(
        g: *mut arb_poly_struct,
        s: *const arb_struct,
        h: *const arb_poly_struct,
        regularized: ::std::os::raw::c_int,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_gamma_upper(
        res: *mut arb_struct,
        s: *const arb_struct,
        z: *const arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_gamma_upper_series(
        g: arb_ptr,
        s: *const arb_struct,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        regularized: ::std::os::raw::c_int,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_gamma_upper_series(
        g: *mut arb_poly_struct,
        s: *const arb_struct,
        h: *const arb_poly_struct,
        regularized: ::std::os::raw::c_int,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_gamma_upper_integration(
        res: *mut arb_struct,
        s: *const arb_struct,
        z: *const arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_beta_lower(
        res: *mut arb_struct,
        a: *const arb_struct,
        c: *const arb_struct,
        z: *const arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_beta_lower_series(
        res: *mut arb_poly_struct,
        a: *const arb_struct,
        b: *const arb_struct,
        z: *const arb_poly_struct,
        regularized: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_beta_lower_series(
        res: arb_ptr,
        a: *const arb_struct,
        b: *const arb_struct,
        z: arb_srcptr,
        zlen: mp_limb_signed_t,
        regularized: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_chebyshev_t(
        res: *mut arb_struct,
        nu: *const arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_chebyshev_u(
        res: *mut arb_struct,
        nu: *const arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_jacobi_p(
        res: *mut arb_struct,
        n: *const arb_struct,
        a: *const arb_struct,
        b: *const arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_gegenbauer_c(
        res: *mut arb_struct,
        n: *const arb_struct,
        m: *const arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_laguerre_l(
        res: *mut arb_struct,
        n: *const arb_struct,
        m: *const arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_hermite_h(
        res: *mut arb_struct,
        nu: *const arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_legendre_p(
        res: *mut arb_struct,
        n: *const arb_struct,
        m: *const arb_struct,
        z: *const arb_struct,
        type_: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_legendre_q(
        res: *mut arb_struct,
        n: *const arb_struct,
        m: *const arb_struct,
        z: *const arb_struct,
        type_: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_legendre_p_ui_deriv_bound(
        dp: *mut mag_struct,
        dp2: *mut mag_struct,
        n: mp_limb_t,
        x: *const arb_struct,
        x2sub1: *const arb_struct,
    );
    pub fn arb_hypgeom_legendre_p_ui_rec(
        res: *mut arb_struct,
        res_prime: *mut arb_struct,
        n: mp_limb_t,
        x: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_legendre_p_ui_asymp(
        res: *mut arb_struct,
        res2: *mut arb_struct,
        n: mp_limb_t,
        x: *const arb_struct,
        K: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_legendre_p_ui_one(
        res: *mut arb_struct,
        res2: *mut arb_struct,
        n: mp_limb_t,
        x: *const arb_struct,
        K: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_legendre_p_ui_zero(
        res: *mut arb_struct,
        res2: *mut arb_struct,
        n: mp_limb_t,
        x: *const arb_struct,
        K: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_legendre_p_ui(
        res: *mut arb_struct,
        res_prime: *mut arb_struct,
        n: mp_limb_t,
        x: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_legendre_p_ui_root(
        res: *mut arb_struct,
        weight: *mut arb_struct,
        n: mp_limb_t,
        k: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_central_bin_ui(res: *mut arb_struct, n: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_hypgeom_dilog(res: *mut arb_struct, z: *const arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_hypgeom_gamma_lower_sum_rs_1(
        res: *mut arb_struct,
        p: mp_limb_t,
        q: mp_limb_t,
        z: *const arb_struct,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_gamma_upper_sum_rs_1(
        res: *mut arb_struct,
        p: mp_limb_t,
        q: mp_limb_t,
        z: *const arb_struct,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_gamma_upper_fmpq_inf_choose_N(
        err: *mut mag_struct,
        a: *const fmpq,
        z: *const arb_struct,
        abs_tol: *const mag_struct,
    ) -> mp_limb_signed_t;
    pub fn _arb_hypgeom_gamma_upper_fmpq_inf_bsplit(
        res: *mut arb_struct,
        a: *const fmpq,
        z: *const arb_struct,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_gamma_lower_fmpq_0_choose_N(
        err: *mut mag_struct,
        a: *const fmpq,
        z: *const arb_struct,
        abs_tol: *const mag_struct,
    ) -> mp_limb_signed_t;
    pub fn _arb_hypgeom_gamma_lower_fmpq_0_bsplit(
        res: *mut arb_struct,
        a: *const fmpq,
        z: *const arb_struct,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_gamma_upper_fmpq_step_bsplit(
        Gz1: *mut arb_struct,
        a: *const fmpq,
        z0: *const arb_struct,
        z1: *const arb_struct,
        Gz0: *const arb_struct,
        expmz0: *const arb_struct,
        abs_tol: *const mag_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_gamma_upper_singular_si_choose_N(
        err: *mut mag_struct,
        n: mp_limb_signed_t,
        z: *const arb_struct,
        abs_tol: *const mag_struct,
    ) -> mp_limb_signed_t;
    pub fn _arb_hypgeom_gamma_upper_singular_si_bsplit(
        res: *mut arb_struct,
        n: mp_limb_signed_t,
        z: *const arb_struct,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_erf_bb(
        res: *mut arb_struct,
        z: *const arb_struct,
        complementary: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn arb_hypgeom_sum_fmpq_arb_forward(
        res: *mut arb_struct,
        a: *const fmpq,
        alen: mp_limb_signed_t,
        b: *const fmpq,
        blen: mp_limb_signed_t,
        z: *const arb_struct,
        reciprocal: ::std::os::raw::c_int,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_sum_fmpq_arb_rs(
        res: *mut arb_struct,
        a: *const fmpq,
        alen: mp_limb_signed_t,
        b: *const fmpq,
        blen: mp_limb_signed_t,
        z: *const arb_struct,
        reciprocal: ::std::os::raw::c_int,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_sum_fmpq_arb_bs(
        res: *mut arb_struct,
        a: *const fmpq,
        alen: mp_limb_signed_t,
        b: *const fmpq,
        blen: mp_limb_signed_t,
        z: *const arb_struct,
        reciprocal: ::std::os::raw::c_int,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_sum_fmpq_arb(
        res: *mut arb_struct,
        a: *const fmpq,
        alen: mp_limb_signed_t,
        b: *const fmpq,
        blen: mp_limb_signed_t,
        z: *const arb_struct,
        reciprocal: ::std::os::raw::c_int,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_sum_fmpq_imag_arb_forward(
        res1: *mut arb_struct,
        res2: *mut arb_struct,
        a: *const fmpq,
        alen: mp_limb_signed_t,
        b: *const fmpq,
        blen: mp_limb_signed_t,
        z: *const arb_struct,
        reciprocal: ::std::os::raw::c_int,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_sum_fmpq_imag_arb_rs(
        res1: *mut arb_struct,
        res2: *mut arb_struct,
        a: *const fmpq,
        alen: mp_limb_signed_t,
        b: *const fmpq,
        blen: mp_limb_signed_t,
        z: *const arb_struct,
        reciprocal: ::std::os::raw::c_int,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_sum_fmpq_imag_arb_bs(
        res_real: *mut arb_struct,
        res_imag: *mut arb_struct,
        a: *const fmpq,
        alen: mp_limb_signed_t,
        b: *const fmpq,
        blen: mp_limb_signed_t,
        z: *const arb_struct,
        reciprocal: ::std::os::raw::c_int,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_sum_fmpq_imag_arb(
        res1: *mut arb_struct,
        res2: *mut arb_struct,
        a: *const fmpq,
        alen: mp_limb_signed_t,
        b: *const fmpq,
        blen: mp_limb_signed_t,
        z: *const arb_struct,
        reciprocal: ::std::os::raw::c_int,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
}
