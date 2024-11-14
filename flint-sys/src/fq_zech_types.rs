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
use crate::fq_zech_mpoly_factor::*;
use crate::fq_zech_poly::*;
use crate::fq_zech_poly_factor::*;
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
pub struct fq_zech_struct {
    pub value: mp_limb_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_zech_struct"][::std::mem::size_of::<fq_zech_struct>() - 8usize];
    ["Alignment of fq_zech_struct"][::std::mem::align_of::<fq_zech_struct>() - 8usize];
    ["Offset of field: fq_zech_struct::value"]
        [::std::mem::offset_of!(fq_zech_struct, value) - 0usize];
};
impl Default for fq_zech_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_zech_t = [fq_zech_struct; 1usize];
#[repr(C)]
pub struct fq_zech_ctx_struct {
    pub qm1: mp_limb_t,
    pub qm1o2: mp_limb_t,
    pub qm1opm1: mp_limb_t,
    pub p: mp_limb_t,
    pub ppre: f64,
    pub prime_root: mp_limb_t,
    pub zech_log_table: *mut mp_limb_t,
    pub prime_field_table: *mut mp_limb_t,
    pub eval_table: *mut mp_limb_t,
    pub fq_nmod_ctx: *mut fq_nmod_ctx_struct,
    pub owns_fq_nmod_ctx: libc::c_int,
    pub is_conway: libc::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_zech_ctx_struct"][::std::mem::size_of::<fq_zech_ctx_struct>() - 88usize];
    ["Alignment of fq_zech_ctx_struct"][::std::mem::align_of::<fq_zech_ctx_struct>() - 8usize];
    ["Offset of field: fq_zech_ctx_struct::qm1"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, qm1) - 0usize];
    ["Offset of field: fq_zech_ctx_struct::qm1o2"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, qm1o2) - 8usize];
    ["Offset of field: fq_zech_ctx_struct::qm1opm1"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, qm1opm1) - 16usize];
    ["Offset of field: fq_zech_ctx_struct::p"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, p) - 24usize];
    ["Offset of field: fq_zech_ctx_struct::ppre"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, ppre) - 32usize];
    ["Offset of field: fq_zech_ctx_struct::prime_root"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, prime_root) - 40usize];
    ["Offset of field: fq_zech_ctx_struct::zech_log_table"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, zech_log_table) - 48usize];
    ["Offset of field: fq_zech_ctx_struct::prime_field_table"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, prime_field_table) - 56usize];
    ["Offset of field: fq_zech_ctx_struct::eval_table"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, eval_table) - 64usize];
    ["Offset of field: fq_zech_ctx_struct::fq_nmod_ctx"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, fq_nmod_ctx) - 72usize];
    ["Offset of field: fq_zech_ctx_struct::owns_fq_nmod_ctx"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, owns_fq_nmod_ctx) - 80usize];
    ["Offset of field: fq_zech_ctx_struct::is_conway"]
        [::std::mem::offset_of!(fq_zech_ctx_struct, is_conway) - 84usize];
};
impl Default for fq_zech_ctx_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_zech_ctx_t = [fq_zech_ctx_struct; 1usize];
#[repr(C)]
pub struct fq_zech_mat_struct {
    pub entries: *mut fq_zech_struct,
    pub r: mp_limb_signed_t,
    pub c: mp_limb_signed_t,
    pub rows: *mut *mut fq_zech_struct,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_zech_mat_struct"][::std::mem::size_of::<fq_zech_mat_struct>() - 32usize];
    ["Alignment of fq_zech_mat_struct"][::std::mem::align_of::<fq_zech_mat_struct>() - 8usize];
    ["Offset of field: fq_zech_mat_struct::entries"]
        [::std::mem::offset_of!(fq_zech_mat_struct, entries) - 0usize];
    ["Offset of field: fq_zech_mat_struct::r"]
        [::std::mem::offset_of!(fq_zech_mat_struct, r) - 8usize];
    ["Offset of field: fq_zech_mat_struct::c"]
        [::std::mem::offset_of!(fq_zech_mat_struct, c) - 16usize];
    ["Offset of field: fq_zech_mat_struct::rows"]
        [::std::mem::offset_of!(fq_zech_mat_struct, rows) - 24usize];
};
impl Default for fq_zech_mat_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_zech_mat_t = [fq_zech_mat_struct; 1usize];
#[repr(C)]
pub struct fq_zech_poly_struct {
    pub coeffs: *mut fq_zech_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_zech_poly_struct"][::std::mem::size_of::<fq_zech_poly_struct>() - 24usize];
    ["Alignment of fq_zech_poly_struct"][::std::mem::align_of::<fq_zech_poly_struct>() - 8usize];
    ["Offset of field: fq_zech_poly_struct::coeffs"]
        [::std::mem::offset_of!(fq_zech_poly_struct, coeffs) - 0usize];
    ["Offset of field: fq_zech_poly_struct::alloc"]
        [::std::mem::offset_of!(fq_zech_poly_struct, alloc) - 8usize];
    ["Offset of field: fq_zech_poly_struct::length"]
        [::std::mem::offset_of!(fq_zech_poly_struct, length) - 16usize];
};
impl Default for fq_zech_poly_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_zech_poly_t = [fq_zech_poly_struct; 1usize];
#[repr(C)]
pub struct fq_zech_poly_factor_struct {
    pub poly: *mut fq_zech_poly_struct,
    pub exp: *mut mp_limb_signed_t,
    pub num: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_zech_poly_factor_struct"]
        [::std::mem::size_of::<fq_zech_poly_factor_struct>() - 32usize];
    ["Alignment of fq_zech_poly_factor_struct"]
        [::std::mem::align_of::<fq_zech_poly_factor_struct>() - 8usize];
    ["Offset of field: fq_zech_poly_factor_struct::poly"]
        [::std::mem::offset_of!(fq_zech_poly_factor_struct, poly) - 0usize];
    ["Offset of field: fq_zech_poly_factor_struct::exp"]
        [::std::mem::offset_of!(fq_zech_poly_factor_struct, exp) - 8usize];
    ["Offset of field: fq_zech_poly_factor_struct::num"]
        [::std::mem::offset_of!(fq_zech_poly_factor_struct, num) - 16usize];
    ["Offset of field: fq_zech_poly_factor_struct::alloc"]
        [::std::mem::offset_of!(fq_zech_poly_factor_struct, alloc) - 24usize];
};
impl Default for fq_zech_poly_factor_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_zech_poly_factor_t = [fq_zech_poly_factor_struct; 1usize];
