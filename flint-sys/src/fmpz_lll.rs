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
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub const fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
pub const CPU_SIZE_1: u32 = 53;
pub const SIZE_RED_FAILURE_THRESH: u32 = 5;
pub const rep_type_GRAM: rep_type = 0;
pub const rep_type_Z_BASIS: rep_type = 1;
pub type rep_type = libc::c_uint;
pub const gram_type_APPROX: gram_type = 0;
pub const gram_type_EXACT: gram_type = 1;
pub type gram_type = libc::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_lll_struct {
    pub delta: f64,
    pub eta: f64,
    pub rt: rep_type,
    pub gt: gram_type,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fmpz_lll_struct"][::std::mem::size_of::<fmpz_lll_struct>() - 24usize];
    ["Alignment of fmpz_lll_struct"][::std::mem::align_of::<fmpz_lll_struct>() - 8usize];
    ["Offset of field: fmpz_lll_struct::delta"]
        [::std::mem::offset_of!(fmpz_lll_struct, delta) - 0usize];
    ["Offset of field: fmpz_lll_struct::eta"]
        [::std::mem::offset_of!(fmpz_lll_struct, eta) - 8usize];
    ["Offset of field: fmpz_lll_struct::rt"][::std::mem::offset_of!(fmpz_lll_struct, rt) - 16usize];
    ["Offset of field: fmpz_lll_struct::gt"][::std::mem::offset_of!(fmpz_lll_struct, gt) - 20usize];
};
impl Default for fmpz_lll_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fmpz_lll_t = [fmpz_lll_struct; 1usize];
#[repr(C)]
pub struct fmpz_gram_union {
    pub appSP: __BindgenUnionField<d_mat_t>,
    pub appSP2: __BindgenUnionField<mpf_mat_t>,
    pub exactSP: __BindgenUnionField<fmpz_mat_t>,
    pub bindgen_union_field: [u64; 5usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fmpz_gram_union"][::std::mem::size_of::<fmpz_gram_union>() - 40usize];
    ["Alignment of fmpz_gram_union"][::std::mem::align_of::<fmpz_gram_union>() - 8usize];
    ["Offset of field: fmpz_gram_union::appSP"]
        [::std::mem::offset_of!(fmpz_gram_union, appSP) - 0usize];
    ["Offset of field: fmpz_gram_union::appSP2"]
        [::std::mem::offset_of!(fmpz_gram_union, appSP2) - 0usize];
    ["Offset of field: fmpz_gram_union::exactSP"]
        [::std::mem::offset_of!(fmpz_gram_union, exactSP) - 0usize];
};
impl Default for fmpz_gram_union {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fmpz_gram_t = [fmpz_gram_union; 1usize];
extern "C" {
    pub fn fmpz_lll_context_init_default(fl: *mut fmpz_lll_struct);
    pub fn fmpz_lll_context_init(
        fl: *mut fmpz_lll_struct,
        delta: f64,
        eta: f64,
        rt: rep_type,
        gt: gram_type,
    );
    pub fn fmpz_lll_randtest(fl: *mut fmpz_lll_struct, state: *mut flint_rand_s);
    pub fn fmpz_lll_heuristic_dot(
        vec1: *const f64,
        vec2: *const f64,
        len2: mp_limb_signed_t,
        B: *const fmpz_mat_struct,
        k: mp_limb_signed_t,
        j: mp_limb_signed_t,
        exp_adj: mp_limb_signed_t,
    ) -> f64;
    pub fn fmpz_lll_check_babai(
        kappa: libc::c_int,
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        mu: *mut d_mat_struct,
        r: *mut d_mat_struct,
        s: *mut f64,
        appB: *mut d_mat_struct,
        expo: *mut libc::c_int,
        A: *mut fmpz_gram_union,
        a: libc::c_int,
        zeros: libc::c_int,
        kappamax: libc::c_int,
        n: libc::c_int,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_check_babai_heuristic_d(
        kappa: libc::c_int,
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        mu: *mut d_mat_struct,
        r: *mut d_mat_struct,
        s: *mut f64,
        appB: *mut d_mat_struct,
        expo: *mut libc::c_int,
        A: *mut fmpz_gram_union,
        a: libc::c_int,
        zeros: libc::c_int,
        kappamax: libc::c_int,
        n: libc::c_int,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_shift(B: *const fmpz_mat_struct) -> libc::c_int;
    pub fn fmpz_lll_d(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_d_heuristic(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_check_babai_heuristic(
        kappa: libc::c_int,
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        mu: *mut mpf_mat_struct,
        r: *mut mpf_mat_struct,
        s: *mut mpf,
        appB: *mut mpf_mat_struct,
        A: *mut fmpz_gram_union,
        a: libc::c_int,
        zeros: libc::c_int,
        kappamax: libc::c_int,
        n: libc::c_int,
        tmp: *mut __mpf_struct,
        rtmp: *mut __mpf_struct,
        prec: mp_limb_t,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_mpf2(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        prec: mp_limb_t,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_mpf(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_wrapper(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_advance_check_babai(
        cur_kappa: libc::c_int,
        kappa: libc::c_int,
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        mu: *mut d_mat_struct,
        r: *mut d_mat_struct,
        s: *mut f64,
        appB: *mut d_mat_struct,
        expo: *mut libc::c_int,
        A: *mut fmpz_gram_union,
        a: libc::c_int,
        zeros: libc::c_int,
        kappamax: libc::c_int,
        n: libc::c_int,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_advance_check_babai_heuristic_d(
        cur_kappa: libc::c_int,
        kappa: libc::c_int,
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        mu: *mut d_mat_struct,
        r: *mut d_mat_struct,
        s: *mut f64,
        appB: *mut d_mat_struct,
        expo: *mut libc::c_int,
        A: *mut fmpz_gram_union,
        a: libc::c_int,
        zeros: libc::c_int,
        kappamax: libc::c_int,
        n: libc::c_int,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_d_with_removal(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        gs_B: *const fmpz,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_d_heuristic_with_removal(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        gs_B: *const fmpz,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_mpf2_with_removal(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        prec: mp_limb_t,
        gs_B: *const fmpz,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_mpf_with_removal(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        gs_B: *const fmpz,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_wrapper_with_removal(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        gs_B: *const fmpz,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_d_with_removal_knapsack(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        gs_B: *const fmpz,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_wrapper_with_removal_knapsack(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        gs_B: *const fmpz,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_with_removal_ulll(
        FM: *mut fmpz_mat_struct,
        UM: *mut fmpz_mat_struct,
        new_size: mp_limb_signed_t,
        gs_B: *const fmpz,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_is_reduced_d(
        B: *const fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_is_reduced_mpfr(
        B: *const fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
        prec: mp_limb_t,
    ) -> libc::c_int;
    pub fn fmpz_lll_is_reduced(
        B: *const fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
        prec: mp_limb_t,
    ) -> libc::c_int;
    pub fn fmpz_lll_is_reduced_d_with_removal(
        B: *const fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
        gs_B: *const fmpz,
        newd: libc::c_int,
    ) -> libc::c_int;
    pub fn fmpz_lll_is_reduced_mpfr_with_removal(
        B: *const fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
        gs_B: *const fmpz,
        newd: libc::c_int,
        prec: mp_limb_t,
    ) -> libc::c_int;
    pub fn fmpz_lll_is_reduced_with_removal(
        B: *const fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
        gs_B: *const fmpz,
        newd: libc::c_int,
        prec: mp_limb_t,
    ) -> libc::c_int;
    pub fn fmpz_lll(B: *mut fmpz_mat_struct, U: *mut fmpz_mat_struct, fl: *const fmpz_lll_struct);
    pub fn fmpz_lll_with_removal(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        gs_B: *const fmpz,
        fl: *const fmpz_lll_struct,
    ) -> libc::c_int;
    pub fn fmpz_lll_storjohann_ulll(
        FM: *mut fmpz_mat_struct,
        new_size: mp_limb_signed_t,
        fl: *const fmpz_lll_struct,
    );
}
