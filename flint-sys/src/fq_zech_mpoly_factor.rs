/* automatically generated by rust-bindgen 0.70.1 */

use libc::*;
use crate::deps::*;
use crate::bindgen::*;
use crate::acb::*;
use crate::acb_calc::*;
use crate::acb_dft::*;
use crate::acb_dirichlet::*;
use crate::acb_elliptic::*;
use crate::acb_hypgeom::*;
use crate::acb_mat::*;
use crate::acb_modular::*;
use crate::acb_poly::*;
use crate::acb_theta::*;
use crate::acb_types::*;
use crate::acf::*;
use crate::acf_types::*;
use crate::aprcl::*;
use crate::arb::*;
use crate::arb_calc::*;
use crate::arb_fmpz_poly::*;
use crate::arb_fpwrap::*;
use crate::arb_hypgeom::*;
use crate::arb_mat::*;
use crate::arb_poly::*;
use crate::arb_types::*;
use crate::arf::*;
use crate::arf_types::*;
use crate::arith::*;
use crate::bernoulli::*;
use crate::bool_mat::*;
use crate::ca::*;
use crate::ca_ext::*;
use crate::ca_field::*;
use crate::ca_mat::*;
use crate::ca_poly::*;
use crate::ca_vec::*;
use crate::calcium::*;
use crate::d_mat::*;
use crate::d_vec::*;
use crate::dirichlet::*;
use crate::dlog::*;
use crate::double_extras::*;
use crate::double_interval::*;
use crate::fexpr::*;
use crate::fexpr_builtin::*;
use crate::fft::*;
use crate::fft_tuning::*;
use crate::flint_config::*;
use crate::flint::*;
use crate::fmpq::*;
use crate::fmpq_mat::*;
use crate::fmpq_mpoly::*;
use crate::fmpq_mpoly_factor::*;
use crate::fmpq_poly::*;
use crate::fmpq_types::*;
use crate::fmpq_vec::*;
use crate::fmpz::*;
use crate::fmpz_extras::*;
use crate::fmpz_factor::*;
use crate::fmpz_lll::*;
use crate::fmpz_mat::*;
use crate::fmpz_mod::*;
use crate::fmpz_mod_mat::*;
use crate::fmpz_mod_mpoly::*;
use crate::fmpz_mod_mpoly_factor::*;
use crate::fmpz_mod_poly::*;
use crate::fmpz_mod_poly_factor::*;
use crate::fmpz_mod_types::*;
use crate::fmpz_mod_vec::*;
use crate::fmpz_mpoly::*;
use crate::fmpz_mpoly_factor::*;
use crate::fmpz_mpoly_q::*;
use crate::fmpz_poly::*;
use crate::fmpz_poly_factor::*;
use crate::fmpz_poly_mat::*;
use crate::fmpz_poly_q::*;
use crate::fmpz_types::*;
use crate::fmpz_vec::*;
use crate::fmpzi::*;
use crate::fq::*;
use crate::fq_default::*;
use crate::fq_default_mat::*;
use crate::fq_default_poly::*;
use crate::fq_default_poly_factor::*;
use crate::fq_embed::*;
use crate::fq_embed_templates::*;
use crate::fq_mat::*;
use crate::fq_mat_templates::*;
use crate::fq_nmod::*;
use crate::fq_nmod_embed::*;
use crate::fq_nmod_mat::*;
use crate::fq_nmod_mpoly::*;
use crate::fq_nmod_mpoly_factor::*;
use crate::fq_nmod_poly::*;
use crate::fq_nmod_poly_factor::*;
use crate::fq_nmod_types::*;
use crate::fq_nmod_vec::*;
use crate::fq_poly::*;
use crate::fq_poly_factor::*;
use crate::fq_poly_factor_templates::*;
use crate::fq_poly_templates::*;
use crate::fq_templates::*;
use crate::fq_types::*;
use crate::fq_vec::*;
use crate::fq_vec_templates::*;
use crate::fq_zech::*;
use crate::fq_zech_embed::*;
use crate::fq_zech_mat::*;
use crate::fq_zech_mpoly::*;
use crate::fq_zech_poly::*;
use crate::fq_zech_poly_factor::*;
use crate::fq_zech_types::*;
use crate::fq_zech_vec::*;
use crate::gmpcompat::*;
use crate::gr::*;
use crate::gr_generic::*;
use crate::gr_mat::*;
use crate::gr_mpoly::*;
use crate::gr_poly::*;
use crate::gr_special::*;
use crate::gr_vec::*;
use crate::hypgeom::*;
use crate::limb_types::*;
use crate::long_extras::*;
use crate::longlong::*;
use crate::mag::*;
use crate::mpf_impl::*;
use crate::mpfr_mat::*;
use crate::mpfr_vec::*;
use crate::mpoly::*;
use crate::mpoly_types::*;
use crate::n_poly::*;
use crate::n_poly_types::*;
use crate::nf::*;
use crate::nf_elem::*;
use crate::nmod::*;
use crate::nmod_mat::*;
use crate::nmod_mpoly::*;
use crate::nmod_mpoly_factor::*;
use crate::nmod_poly::*;
use crate::nmod_poly_factor::*;
use crate::nmod_poly_mat::*;
use crate::nmod_types::*;
use crate::nmod_vec::*;
use crate::padic::*;
use crate::padic_mat::*;
use crate::padic_poly::*;
use crate::padic_types::*;
use crate::partitions::*;
use crate::perm::*;
use crate::qadic::*;
use crate::qfb::*;
use crate::qqbar::*;
use crate::qsieve::*;
use crate::templates::*;
use crate::thread_pool::*;
use crate::thread_support::*;
use crate::ulong_extras::*;


#[repr(C)]
pub struct fq_zech_bpoly_struct {
    pub coeffs: *mut fq_zech_poly_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_zech_bpoly_struct"][::std::mem::size_of::<fq_zech_bpoly_struct>() - 24usize];
    ["Alignment of fq_zech_bpoly_struct"][::std::mem::align_of::<fq_zech_bpoly_struct>() - 8usize];
    ["Offset of field: fq_zech_bpoly_struct::coeffs"]
        [::std::mem::offset_of!(fq_zech_bpoly_struct, coeffs) - 0usize];
    ["Offset of field: fq_zech_bpoly_struct::alloc"]
        [::std::mem::offset_of!(fq_zech_bpoly_struct, alloc) - 8usize];
    ["Offset of field: fq_zech_bpoly_struct::length"]
        [::std::mem::offset_of!(fq_zech_bpoly_struct, length) - 16usize];
};
impl Default for fq_zech_bpoly_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_zech_bpoly_t = [fq_zech_bpoly_struct; 1usize];
#[repr(C)]
pub struct fq_zech_tpoly_struct {
    pub coeffs: *mut fq_zech_bpoly_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_zech_tpoly_struct"][::std::mem::size_of::<fq_zech_tpoly_struct>() - 24usize];
    ["Alignment of fq_zech_tpoly_struct"][::std::mem::align_of::<fq_zech_tpoly_struct>() - 8usize];
    ["Offset of field: fq_zech_tpoly_struct::coeffs"]
        [::std::mem::offset_of!(fq_zech_tpoly_struct, coeffs) - 0usize];
    ["Offset of field: fq_zech_tpoly_struct::alloc"]
        [::std::mem::offset_of!(fq_zech_tpoly_struct, alloc) - 8usize];
    ["Offset of field: fq_zech_tpoly_struct::length"]
        [::std::mem::offset_of!(fq_zech_tpoly_struct, length) - 16usize];
};
impl Default for fq_zech_tpoly_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_zech_tpoly_t = [fq_zech_tpoly_struct; 1usize];
#[repr(C)]
pub struct fq_zech_polyu_struct {
    pub exps: *mut mp_limb_t,
    pub coeffs: *mut fq_zech_struct,
    pub length: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_zech_polyu_struct"][::std::mem::size_of::<fq_zech_polyu_struct>() - 32usize];
    ["Alignment of fq_zech_polyu_struct"][::std::mem::align_of::<fq_zech_polyu_struct>() - 8usize];
    ["Offset of field: fq_zech_polyu_struct::exps"]
        [::std::mem::offset_of!(fq_zech_polyu_struct, exps) - 0usize];
    ["Offset of field: fq_zech_polyu_struct::coeffs"]
        [::std::mem::offset_of!(fq_zech_polyu_struct, coeffs) - 8usize];
    ["Offset of field: fq_zech_polyu_struct::length"]
        [::std::mem::offset_of!(fq_zech_polyu_struct, length) - 16usize];
    ["Offset of field: fq_zech_polyu_struct::alloc"]
        [::std::mem::offset_of!(fq_zech_polyu_struct, alloc) - 24usize];
};
impl Default for fq_zech_polyu_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_zech_polyu_t = [fq_zech_polyu_struct; 1usize];
#[repr(C)]
pub struct fq_zech_polyun_struct {
    pub coeffs: *mut fq_zech_poly_struct,
    pub exps: *mut mp_limb_t,
    pub length: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_zech_polyun_struct"][::std::mem::size_of::<fq_zech_polyun_struct>() - 32usize];
    ["Alignment of fq_zech_polyun_struct"]
        [::std::mem::align_of::<fq_zech_polyun_struct>() - 8usize];
    ["Offset of field: fq_zech_polyun_struct::coeffs"]
        [::std::mem::offset_of!(fq_zech_polyun_struct, coeffs) - 0usize];
    ["Offset of field: fq_zech_polyun_struct::exps"]
        [::std::mem::offset_of!(fq_zech_polyun_struct, exps) - 8usize];
    ["Offset of field: fq_zech_polyun_struct::length"]
        [::std::mem::offset_of!(fq_zech_polyun_struct, length) - 16usize];
    ["Offset of field: fq_zech_polyun_struct::alloc"]
        [::std::mem::offset_of!(fq_zech_polyun_struct, alloc) - 24usize];
};
impl Default for fq_zech_polyun_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_zech_polyun_t = [fq_zech_polyun_struct; 1usize];
#[repr(C)]
pub struct fq_zech_mpoly_factor_struct {
    pub constant: fq_zech_t,
    pub poly: *mut fq_zech_mpoly_struct,
    pub exp: *mut fmpz,
    pub num: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_zech_mpoly_factor_struct"]
        [::std::mem::size_of::<fq_zech_mpoly_factor_struct>() - 40usize];
    ["Alignment of fq_zech_mpoly_factor_struct"]
        [::std::mem::align_of::<fq_zech_mpoly_factor_struct>() - 8usize];
    ["Offset of field: fq_zech_mpoly_factor_struct::constant"]
        [::std::mem::offset_of!(fq_zech_mpoly_factor_struct, constant) - 0usize];
    ["Offset of field: fq_zech_mpoly_factor_struct::poly"]
        [::std::mem::offset_of!(fq_zech_mpoly_factor_struct, poly) - 8usize];
    ["Offset of field: fq_zech_mpoly_factor_struct::exp"]
        [::std::mem::offset_of!(fq_zech_mpoly_factor_struct, exp) - 16usize];
    ["Offset of field: fq_zech_mpoly_factor_struct::num"]
        [::std::mem::offset_of!(fq_zech_mpoly_factor_struct, num) - 24usize];
    ["Offset of field: fq_zech_mpoly_factor_struct::alloc"]
        [::std::mem::offset_of!(fq_zech_mpoly_factor_struct, alloc) - 32usize];
};
impl Default for fq_zech_mpoly_factor_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_zech_mpoly_factor_t = [fq_zech_mpoly_factor_struct; 1usize];
#[repr(C)]
pub struct fq_zech_mpolyv_struct {
    pub coeffs: *mut fq_zech_mpoly_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_zech_mpolyv_struct"][::std::mem::size_of::<fq_zech_mpolyv_struct>() - 24usize];
    ["Alignment of fq_zech_mpolyv_struct"]
        [::std::mem::align_of::<fq_zech_mpolyv_struct>() - 8usize];
    ["Offset of field: fq_zech_mpolyv_struct::coeffs"]
        [::std::mem::offset_of!(fq_zech_mpolyv_struct, coeffs) - 0usize];
    ["Offset of field: fq_zech_mpolyv_struct::alloc"]
        [::std::mem::offset_of!(fq_zech_mpolyv_struct, alloc) - 8usize];
    ["Offset of field: fq_zech_mpolyv_struct::length"]
        [::std::mem::offset_of!(fq_zech_mpolyv_struct, length) - 16usize];
};
impl Default for fq_zech_mpolyv_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_zech_mpolyv_t = [fq_zech_mpolyv_struct; 1usize];
#[repr(C)]
pub struct fq_zech_mpoly_pfrac_struct {
    pub bits: mp_limb_t,
    pub w: mp_limb_signed_t,
    pub r: mp_limb_signed_t,
    pub inv_prod_dbetas: *mut fq_zech_poly_struct,
    pub inv_prod_dbetas_mvar: *mut fq_zech_mpoly_struct,
    pub dbetas: *mut fq_zech_poly_struct,
    pub dbetas_mvar: *mut fq_zech_mpoly_struct,
    pub prod_mbetas: *mut fq_zech_mpoly_struct,
    pub prod_mbetas_coeffs: *mut fq_zech_mpolyv_struct,
    pub mbetas: *mut fq_zech_mpoly_struct,
    pub deltas: *mut fq_zech_mpoly_struct,
    pub xalpha: *mut fq_zech_mpoly_struct,
    pub q: *mut fq_zech_mpoly_struct,
    pub qt: *mut fq_zech_mpoly_struct,
    pub newt: *mut fq_zech_mpoly_struct,
    pub delta_coeffs: *mut fq_zech_mpolyv_struct,
    pub T: fq_zech_mpoly_t,
    pub Q: fq_zech_mpoly_t,
    pub R: fq_zech_mpoly_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_zech_mpoly_pfrac_struct"]
        [::std::mem::size_of::<fq_zech_mpoly_pfrac_struct>() - 248usize];
    ["Alignment of fq_zech_mpoly_pfrac_struct"]
        [::std::mem::align_of::<fq_zech_mpoly_pfrac_struct>() - 8usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::bits"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, bits) - 0usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::w"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, w) - 8usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::r"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, r) - 16usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::inv_prod_dbetas"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, inv_prod_dbetas) - 24usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::inv_prod_dbetas_mvar"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, inv_prod_dbetas_mvar) - 32usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::dbetas"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, dbetas) - 40usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::dbetas_mvar"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, dbetas_mvar) - 48usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::prod_mbetas"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, prod_mbetas) - 56usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::prod_mbetas_coeffs"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, prod_mbetas_coeffs) - 64usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::mbetas"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, mbetas) - 72usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::deltas"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, deltas) - 80usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::xalpha"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, xalpha) - 88usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::q"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, q) - 96usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::qt"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, qt) - 104usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::newt"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, newt) - 112usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::delta_coeffs"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, delta_coeffs) - 120usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::T"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, T) - 128usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::Q"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, Q) - 168usize];
    ["Offset of field: fq_zech_mpoly_pfrac_struct::R"]
        [::std::mem::offset_of!(fq_zech_mpoly_pfrac_struct, R) - 208usize];
};
impl Default for fq_zech_mpoly_pfrac_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_zech_mpoly_pfrac_t = [fq_zech_mpoly_pfrac_struct; 1usize];
extern "C" {
    #[link_name = "fq_zech_bpoly_init__extern"]
    pub fn fq_zech_bpoly_init(A: *mut fq_zech_bpoly_struct, ctx: *const fq_zech_ctx_struct);
    pub fn fq_zech_bpoly_clear(A: *mut fq_zech_bpoly_struct, ctx: *const fq_zech_ctx_struct);
    #[link_name = "fq_zech_bpoly_swap__extern"]
    pub fn fq_zech_bpoly_swap(
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_bpoly_normalise(A: *mut fq_zech_bpoly_struct, ctx: *const fq_zech_ctx_struct);
    pub fn fq_zech_bpoly_realloc(
        A: *mut fq_zech_bpoly_struct,
        len: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    );
    #[link_name = "fq_zech_bpoly_fit_length__extern"]
    pub fn fq_zech_bpoly_fit_length(
        A: *mut fq_zech_bpoly_struct,
        len: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    );
    #[link_name = "fq_zech_bpoly_zero__extern"]
    pub fn fq_zech_bpoly_zero(A: *mut fq_zech_bpoly_struct, ctx: *const fq_zech_ctx_struct);
    #[link_name = "fq_zech_bpoly_is_zero__extern"]
    pub fn fq_zech_bpoly_is_zero(
        A: *const fq_zech_bpoly_struct,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_bpoly_equal(
        A: *const fq_zech_bpoly_struct,
        B: *const fq_zech_bpoly_struct,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_bpoly_get_coeff(
        c: *mut fq_zech_struct,
        A: *const fq_zech_bpoly_struct,
        e0: mp_limb_signed_t,
        e1: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    );
    #[link_name = "fq_zech_bpoly_degree0__extern"]
    pub fn fq_zech_bpoly_degree0(
        A: *const fq_zech_bpoly_struct,
        ctx: *const fq_zech_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_zech_bpoly_degree1(
        A: *const fq_zech_bpoly_struct,
        ctx: *const fq_zech_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn fq_zech_bpoly_set_poly_var1(
        A: *mut fq_zech_bpoly_struct,
        B: *const fq_zech_poly_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_bpoly_set_poly_var0(
        A: *mut fq_zech_bpoly_struct,
        B: *const fq_zech_poly_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_bpoly_print_pretty(
        A: *const fq_zech_bpoly_struct,
        var0: *const libc::c_char,
        var1: *const libc::c_char,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_bpoly_is_canonical(
        A: *const fq_zech_bpoly_struct,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_bpoly_fq_equal(
        A: *const fq_zech_bpoly_struct,
        B: *const fq_zech_bpoly_struct,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_bpoly_set_coeff_fq_zech(
        A: *mut fq_zech_bpoly_struct,
        xi: mp_limb_signed_t,
        yi: mp_limb_signed_t,
        c: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_bpoly_set_fq_zech_poly_var0(
        A: *mut fq_zech_bpoly_struct,
        B: *const fq_zech_poly_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_bpoly_set_fq_zech_poly_var1(
        A: *mut fq_zech_bpoly_struct,
        B: *const fq_zech_poly_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_bpoly_make_monic(
        A: *mut fq_zech_bpoly_struct,
        order: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_bpoly_mul(
        A: *mut fq_zech_bpoly_struct,
        B: *const fq_zech_bpoly_struct,
        C: *const fq_zech_bpoly_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_bpoly_mul_series(
        A: *mut fq_zech_bpoly_struct,
        B: *const fq_zech_bpoly_struct,
        C: *const fq_zech_bpoly_struct,
        order: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_bpoly_add(
        A: *mut fq_zech_bpoly_struct,
        B: *const fq_zech_bpoly_struct,
        C: *const fq_zech_bpoly_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_bpoly_one(A: *mut fq_zech_bpoly_struct, ctx: *const fq_zech_ctx_struct);
    pub fn fq_zech_bpoly_sub(
        A: *mut fq_zech_bpoly_struct,
        B: *const fq_zech_bpoly_struct,
        C: *const fq_zech_bpoly_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_bpoly_derivative(
        A: *mut fq_zech_bpoly_struct,
        B: *const fq_zech_bpoly_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_bpoly_divrem_series(
        Q: *mut fq_zech_bpoly_struct,
        R: *mut fq_zech_bpoly_struct,
        A: *const fq_zech_bpoly_struct,
        B: *const fq_zech_bpoly_struct,
        order: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_bpoly_divides(
        Q: *mut fq_zech_bpoly_struct,
        A: *const fq_zech_bpoly_struct,
        B: *const fq_zech_bpoly_struct,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_bpoly_set(
        A: *mut fq_zech_bpoly_struct,
        B: *const fq_zech_bpoly_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_bpoly_make_primitive(
        g: *mut fq_zech_poly_struct,
        A: *mut fq_zech_bpoly_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_bpoly_taylor_shift_var1(
        A: *mut fq_zech_bpoly_struct,
        B: *const fq_zech_bpoly_struct,
        c_: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_bpoly_taylor_shift_var0(
        A: *mut fq_zech_bpoly_struct,
        alpha: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_mpoly_get_fq_zech_bpoly(
        A: *mut fq_zech_bpoly_struct,
        B: *const fq_zech_mpoly_struct,
        varx: mp_limb_signed_t,
        vary: mp_limb_signed_t,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpoly_set_fq_zech_bpoly(
        A: *mut fq_zech_mpoly_struct,
        Abits: mp_limb_t,
        B: *const fq_zech_bpoly_struct,
        varx: mp_limb_signed_t,
        vary: mp_limb_signed_t,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_bpoly_factor_smprime(
        c: *mut fq_zech_poly_struct,
        F: *mut fq_zech_tpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        allow_shift: libc::c_int,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_bpoly_factor_lgprime(
        c: *mut fq_zech_poly_struct,
        F: *mut fq_zech_tpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        ctx: *const fq_zech_ctx_struct,
        state: *mut flint_rand_s,
    ) -> libc::c_int;
    #[link_name = "fq_zech_tpoly_init__extern"]
    pub fn fq_zech_tpoly_init(A: *mut fq_zech_tpoly_struct, ctx: *const fq_zech_ctx_struct);
    #[link_name = "fq_zech_tpoly_swap__extern"]
    pub fn fq_zech_tpoly_swap(
        A: *mut fq_zech_tpoly_struct,
        B: *mut fq_zech_tpoly_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_tpoly_fit_length(
        A: *mut fq_zech_tpoly_struct,
        len: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_tpoly_clear(A: *mut fq_zech_tpoly_struct, ctx: *const fq_zech_ctx_struct);
    #[link_name = "fq_zech_polyu_init__extern"]
    pub fn fq_zech_polyu_init(A: *mut fq_zech_polyu_struct, ctx: *const fq_zech_ctx_struct);
    pub fn fq_zech_polyu_clear(A: *mut fq_zech_polyu_struct, ctx: *const fq_zech_ctx_struct);
    pub fn fq_zech_polyu_realloc(
        A: *mut fq_zech_polyu_struct,
        len: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    );
    #[link_name = "fq_zech_polyu_fit_length__extern"]
    pub fn fq_zech_polyu_fit_length(
        A: *mut fq_zech_polyu_struct,
        len: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    );
    #[link_name = "fq_zech_polyu_swap__extern"]
    pub fn fq_zech_polyu_swap(
        A: *mut fq_zech_polyu_struct,
        B: *mut fq_zech_polyu_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_polyu3_print_pretty(
        A: *const fq_zech_polyu_struct,
        var0: *const libc::c_char,
        var1: *const libc::c_char,
        var2: *const libc::c_char,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_polyu3_degrees(
        deg0: *mut mp_limb_signed_t,
        deg1: *mut mp_limb_signed_t,
        deg2: *mut mp_limb_signed_t,
        A: *const fq_zech_polyu_struct,
    );
    pub fn fq_zech_polyu_is_canonical(
        A: *const fq_zech_polyu_struct,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_zech_polyun_init__extern"]
    pub fn fq_zech_polyun_init(A: *mut fq_zech_polyun_struct, ctx: *const fq_zech_ctx_struct);
    pub fn fq_zech_polyun_clear(A: *mut fq_zech_polyun_struct, ctx: *const fq_zech_ctx_struct);
    pub fn fq_zech_polyun_realloc(
        A: *mut fq_zech_polyun_struct,
        len: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    );
    #[link_name = "fq_zech_polyun_fit_length__extern"]
    pub fn fq_zech_polyun_fit_length(
        A: *mut fq_zech_polyun_struct,
        len: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    );
    #[link_name = "fq_zech_polyun_swap__extern"]
    pub fn fq_zech_polyun_swap(
        A: *mut fq_zech_polyun_struct,
        B: *mut fq_zech_polyun_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_polyu2n_print_pretty(
        A: *const fq_zech_polyun_struct,
        var0: *const libc::c_char,
        var1: *const libc::c_char,
        varlast: *const libc::c_char,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_polyu3n_print_pretty(
        A: *const fq_zech_polyun_struct,
        var0: *const libc::c_char,
        var1: *const libc::c_char,
        var2: *const libc::c_char,
        varlast: *const libc::c_char,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_polyun_is_canonical(
        A: *const fq_zech_polyun_struct,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_mpoly_is_fq_zech_poly(
        A: *const fq_zech_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *const fq_zech_mpoly_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_mpoly_get_fq_zech_poly(
        A: *mut fq_zech_poly_struct,
        B: *const fq_zech_mpoly_struct,
        var: mp_limb_signed_t,
        ctx: *const fq_zech_mpoly_ctx_struct,
    ) -> libc::c_int;
    pub fn _fq_zech_mpoly_set_fq_zech_poly(
        A: *mut fq_zech_mpoly_struct,
        Abits: mp_limb_t,
        Bcoeffs: *const fq_zech_struct,
        Blen: mp_limb_signed_t,
        var: mp_limb_signed_t,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpoly_set_fq_zech_poly(
        A: *mut fq_zech_mpoly_struct,
        B: *const fq_zech_poly_struct,
        var: mp_limb_signed_t,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpoly_factor_init(
        f: *mut fq_zech_mpoly_factor_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpoly_factor_realloc(
        f: *mut fq_zech_mpoly_factor_struct,
        alloc: mp_limb_signed_t,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpoly_factor_fit_length(
        f: *mut fq_zech_mpoly_factor_struct,
        len: mp_limb_signed_t,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpoly_factor_clear(
        f: *mut fq_zech_mpoly_factor_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpoly_factor_set(
        a: *mut fq_zech_mpoly_factor_struct,
        b: *const fq_zech_mpoly_factor_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpoly_factor_print_pretty(
        f: *const fq_zech_mpoly_factor_struct,
        vars: *mut *const libc::c_char,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpoly_factor_append_ui(
        f: *mut fq_zech_mpoly_factor_struct,
        A: *const fq_zech_mpoly_struct,
        e: mp_limb_t,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpoly_factor_append_fmpz(
        f: *mut fq_zech_mpoly_factor_struct,
        A: *const fq_zech_mpoly_struct,
        e: *const fmpz,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpoly_factor_squarefree(
        f: *mut fq_zech_mpoly_factor_struct,
        A: *const fq_zech_mpoly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_mpoly_factor(
        f: *mut fq_zech_mpoly_factor_struct,
        A: *const fq_zech_mpoly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_zech_mpoly_factor_swap__extern"]
    pub fn fq_zech_mpoly_factor_swap(
        A: *mut fq_zech_mpoly_factor_struct,
        B: *mut fq_zech_mpoly_factor_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpoly_factor_one(
        a: *mut fq_zech_mpoly_factor_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpoly_factor_expand(
        A: *mut fq_zech_mpoly_struct,
        f: *const fq_zech_mpoly_factor_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_zech_mpoly_factor_matches__extern"]
    pub fn fq_zech_mpoly_factor_matches(
        a: *const fq_zech_mpoly_struct,
        f: *const fq_zech_mpoly_factor_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    ) -> libc::c_int;
    pub fn _fq_zech_mpoly_get_lead0(
        c: *mut fq_zech_mpoly_struct,
        A: *const fq_zech_mpoly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn _fq_zech_mpoly_set_lead0(
        A: *mut fq_zech_mpoly_struct,
        B: *const fq_zech_mpoly_struct,
        c: *const fq_zech_mpoly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    #[link_name = "fq_zech_mpolyv_init__extern"]
    pub fn fq_zech_mpolyv_init(A: *mut fq_zech_mpolyv_struct, ctx: *const fq_zech_mpoly_ctx_struct);
    #[link_name = "fq_zech_mpolyv_swap__extern"]
    pub fn fq_zech_mpolyv_swap(
        A: *mut fq_zech_mpolyv_struct,
        B: *mut fq_zech_mpolyv_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpolyv_clear(
        A: *mut fq_zech_mpolyv_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpolyv_print_pretty(
        poly: *const fq_zech_mpolyv_struct,
        x: *mut *const libc::c_char,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpolyv_fit_length(
        A: *mut fq_zech_mpolyv_struct,
        length: mp_limb_signed_t,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpolyv_set_coeff(
        A: *mut fq_zech_mpolyv_struct,
        i: mp_limb_signed_t,
        c: *mut fq_zech_mpoly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpoly_to_mpolyv(
        A: *mut fq_zech_mpolyv_struct,
        B: *const fq_zech_mpoly_struct,
        xalpha: *const fq_zech_mpoly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpoly_from_mpolyv(
        A: *mut fq_zech_mpoly_struct,
        B: *const fq_zech_mpolyv_struct,
        xalpha: *const fq_zech_mpoly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpoly_univar_content_mpoly(
        g: *mut fq_zech_mpoly_struct,
        A: *const fq_zech_mpoly_univar_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_mpoly_univar_divexact_mpoly(
        A: *mut fq_zech_mpoly_univar_struct,
        b: *const fq_zech_mpoly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpoly_factor_lcc_wang(
        lc_divs: *mut fq_zech_mpoly_struct,
        lcAfac: *const fq_zech_mpoly_factor_struct,
        Auc: *const fq_zech_poly_struct,
        Auf: *const fq_zech_bpoly_struct,
        r: mp_limb_signed_t,
        alpha: *const fq_zech_poly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_mpoly_factor_irred_smprime_zassenhaus(
        fac: *mut fq_zech_mpolyv_struct,
        A: *const fq_zech_mpoly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> libc::c_int;
    pub fn fq_zech_mpoly_factor_irred_lgprime_zassenhaus(
        fac: *mut fq_zech_mpolyv_struct,
        A: *const fq_zech_mpoly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> libc::c_int;
    pub fn fq_zech_mpoly_factor_irred_smprime_wang(
        fac: *mut fq_zech_mpolyv_struct,
        A: *const fq_zech_mpoly_struct,
        lcAfac: *const fq_zech_mpoly_factor_struct,
        lcA: *const fq_zech_mpoly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> libc::c_int;
    pub fn fq_zech_mpoly_factor_irred_lgprime_wang(
        Af: *mut fq_zech_mpolyv_struct,
        A: *const fq_zech_mpoly_struct,
        lcAfac: *const fq_zech_mpoly_factor_struct,
        lcA: *const fq_zech_mpoly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> libc::c_int;
    pub fn fq_zech_mpoly_factor_irred_smprime_zippel(
        fac: *mut fq_zech_mpolyv_struct,
        A: *const fq_zech_mpoly_struct,
        lcAfac: *const fq_zech_mpoly_factor_struct,
        lcA: *const fq_zech_mpoly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> libc::c_int;
    pub fn fq_zech_mpoly_factor_irred_lgprime_zippel(
        Af: *mut fq_zech_mpolyv_struct,
        A: *const fq_zech_mpoly_struct,
        lcAfac: *const fq_zech_mpoly_factor_struct,
        lcA: *const fq_zech_mpoly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
        state: *mut flint_rand_s,
    ) -> libc::c_int;
    pub fn fq_zech_mpoly_pfrac_init(
        I: *mut fq_zech_mpoly_pfrac_struct,
        bits: mp_limb_t,
        l: mp_limb_signed_t,
        r: mp_limb_signed_t,
        betas: *const fq_zech_mpoly_struct,
        alpha: *const fq_zech_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_mpoly_pfrac_clear(
        I: *mut fq_zech_mpoly_pfrac_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn fq_zech_mpoly_pfrac(
        r: mp_limb_signed_t,
        t: *mut fq_zech_mpoly_struct,
        deg: *const mp_limb_signed_t,
        I: *mut fq_zech_mpoly_pfrac_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_mpoly_hlift(
        m: mp_limb_signed_t,
        f: *mut fq_zech_mpoly_struct,
        r: mp_limb_signed_t,
        alpha: *const fq_zech_struct,
        A: *const fq_zech_mpoly_struct,
        degs: *const mp_limb_signed_t,
        ctx: *const fq_zech_mpoly_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_bpoly_hlift2(
        A: *mut fq_zech_bpoly_struct,
        B0: *mut fq_zech_bpoly_struct,
        B1: *mut fq_zech_bpoly_struct,
        alpha: *const fq_zech_struct,
        degree_inner: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_bpoly_hlift(
        r: mp_limb_signed_t,
        A: *mut fq_zech_bpoly_struct,
        B: *mut fq_zech_bpoly_struct,
        alpha: *const fq_zech_struct,
        degree_inner: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_polyu3_hlift(
        r: mp_limb_signed_t,
        BB: *mut fq_zech_polyun_struct,
        A: *mut fq_zech_polyu_struct,
        B: *mut fq_zech_polyu_struct,
        beta: *const fq_zech_struct,
        degree_inner: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_mpoly_factor_algo(
        f: *mut fq_zech_mpoly_factor_struct,
        A: *const fq_zech_mpoly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
        algo: libc::c_uint,
    ) -> libc::c_int;
    pub fn fq_zech_mpoly_factor_zassenhaus(
        f: *mut fq_zech_mpoly_factor_struct,
        A: *const fq_zech_mpoly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_mpoly_factor_wang(
        f: *mut fq_zech_mpoly_factor_struct,
        A: *const fq_zech_mpoly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_mpoly_factor_zippel(
        f: *mut fq_zech_mpoly_factor_struct,
        A: *const fq_zech_mpoly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_poly_product_roots_fq_zech(
        master: *mut fq_zech_poly_struct,
        monomials: *const fq_zech_struct,
        mlength: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn _fq_zech_mpoly_monomial_evals(
        E: *mut fq_zech_struct,
        Aexps: *const mp_limb_t,
        Abits: mp_limb_t,
        Alen: mp_limb_signed_t,
        alpha: *const fq_zech_struct,
        vstart: mp_limb_signed_t,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn _fq_zech_mpoly_eval_rest_fq_zech_poly(
        E: *mut fq_zech_poly_struct,
        starts: *mut mp_limb_signed_t,
        ends: *mut mp_limb_signed_t,
        stops: *mut mp_limb_signed_t,
        es: *mut mp_limb_t,
        Acoeffs: *const fq_zech_struct,
        Aexps: *const mp_limb_t,
        Alen: mp_limb_signed_t,
        var: mp_limb_signed_t,
        alphas: *const fq_zech_poly_struct,
        offsets: *const mp_limb_signed_t,
        shifts: *const mp_limb_signed_t,
        N: mp_limb_signed_t,
        mask: mp_limb_t,
        nvars: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    pub fn _fq_zech_mpoly_eval_to_bpoly(
        E: *mut fq_zech_bpoly_struct,
        A: *const fq_zech_mpoly_struct,
        alphabetas: *const fq_zech_poly_struct,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
    pub fn _fq_zech_mpoly_set_fq_zech_bpoly_var1_zero(
        A: *mut fq_zech_mpoly_struct,
        Abits: mp_limb_t,
        B: *const fq_zech_bpoly_struct,
        var: mp_limb_signed_t,
        ctx: *const fq_zech_mpoly_ctx_struct,
    );
}
