/* automatically generated by rust-bindgen 0.70.1 */

use libc::*;
use crate::deps::*;
use crate::bindgen::*;
use crate::acb::*;
use crate::acb_calc::*;
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
pub const CRT_MAX: u32 = 15;
pub const DFT_VERB: u32 = 0;
#[repr(C)]
pub struct crt_struct {
    pub num: libc::c_int,
    pub n: nmod_t,
    pub m: [mp_limb_signed_t; 15usize],
    pub M: [mp_limb_t; 15usize],
    pub vM: [mp_limb_t; 15usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of crt_struct"][::std::mem::size_of::<crt_struct>() - 392usize];
    ["Alignment of crt_struct"][::std::mem::align_of::<crt_struct>() - 8usize];
    ["Offset of field: crt_struct::num"][::std::mem::offset_of!(crt_struct, num) - 0usize];
    ["Offset of field: crt_struct::n"][::std::mem::offset_of!(crt_struct, n) - 8usize];
    ["Offset of field: crt_struct::m"][::std::mem::offset_of!(crt_struct, m) - 32usize];
    ["Offset of field: crt_struct::M"][::std::mem::offset_of!(crt_struct, M) - 152usize];
    ["Offset of field: crt_struct::vM"][::std::mem::offset_of!(crt_struct, vM) - 272usize];
};
impl Default for crt_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type crt_t = [crt_struct; 1usize];
pub type acb_dft_step_ptr = *mut acb_dft_step_struct;
#[repr(C)]
pub struct acb_dft_cyc_struct {
    pub n: mp_limb_signed_t,
    pub z: acb_ptr,
    pub zclear: libc::c_int,
    pub num: mp_limb_signed_t,
    pub cyc: acb_dft_step_ptr,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of acb_dft_cyc_struct"][::std::mem::size_of::<acb_dft_cyc_struct>() - 40usize];
    ["Alignment of acb_dft_cyc_struct"][::std::mem::align_of::<acb_dft_cyc_struct>() - 8usize];
    ["Offset of field: acb_dft_cyc_struct::n"]
        [::std::mem::offset_of!(acb_dft_cyc_struct, n) - 0usize];
    ["Offset of field: acb_dft_cyc_struct::z"]
        [::std::mem::offset_of!(acb_dft_cyc_struct, z) - 8usize];
    ["Offset of field: acb_dft_cyc_struct::zclear"]
        [::std::mem::offset_of!(acb_dft_cyc_struct, zclear) - 16usize];
    ["Offset of field: acb_dft_cyc_struct::num"]
        [::std::mem::offset_of!(acb_dft_cyc_struct, num) - 24usize];
    ["Offset of field: acb_dft_cyc_struct::cyc"]
        [::std::mem::offset_of!(acb_dft_cyc_struct, cyc) - 32usize];
};
impl Default for acb_dft_cyc_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type acb_dft_cyc_t = [acb_dft_cyc_struct; 1usize];
#[repr(C)]
pub struct acb_dft_rad2_struct {
    pub e: libc::c_int,
    pub n: mp_limb_signed_t,
    pub dv: mp_limb_signed_t,
    pub nz: mp_limb_signed_t,
    pub z: acb_ptr,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of acb_dft_rad2_struct"][::std::mem::size_of::<acb_dft_rad2_struct>() - 40usize];
    ["Alignment of acb_dft_rad2_struct"][::std::mem::align_of::<acb_dft_rad2_struct>() - 8usize];
    ["Offset of field: acb_dft_rad2_struct::e"]
        [::std::mem::offset_of!(acb_dft_rad2_struct, e) - 0usize];
    ["Offset of field: acb_dft_rad2_struct::n"]
        [::std::mem::offset_of!(acb_dft_rad2_struct, n) - 8usize];
    ["Offset of field: acb_dft_rad2_struct::dv"]
        [::std::mem::offset_of!(acb_dft_rad2_struct, dv) - 16usize];
    ["Offset of field: acb_dft_rad2_struct::nz"]
        [::std::mem::offset_of!(acb_dft_rad2_struct, nz) - 24usize];
    ["Offset of field: acb_dft_rad2_struct::z"]
        [::std::mem::offset_of!(acb_dft_rad2_struct, z) - 32usize];
};
impl Default for acb_dft_rad2_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type acb_dft_rad2_t = [acb_dft_rad2_struct; 1usize];
#[repr(C)]
pub struct acb_dft_bluestein_struct {
    pub n: mp_limb_signed_t,
    pub dv: mp_limb_signed_t,
    pub z: acb_ptr,
    pub g: acb_ptr,
    pub rad2: acb_dft_rad2_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of acb_dft_bluestein_struct"]
        [::std::mem::size_of::<acb_dft_bluestein_struct>() - 72usize];
    ["Alignment of acb_dft_bluestein_struct"]
        [::std::mem::align_of::<acb_dft_bluestein_struct>() - 8usize];
    ["Offset of field: acb_dft_bluestein_struct::n"]
        [::std::mem::offset_of!(acb_dft_bluestein_struct, n) - 0usize];
    ["Offset of field: acb_dft_bluestein_struct::dv"]
        [::std::mem::offset_of!(acb_dft_bluestein_struct, dv) - 8usize];
    ["Offset of field: acb_dft_bluestein_struct::z"]
        [::std::mem::offset_of!(acb_dft_bluestein_struct, z) - 16usize];
    ["Offset of field: acb_dft_bluestein_struct::g"]
        [::std::mem::offset_of!(acb_dft_bluestein_struct, g) - 24usize];
    ["Offset of field: acb_dft_bluestein_struct::rad2"]
        [::std::mem::offset_of!(acb_dft_bluestein_struct, rad2) - 32usize];
};
impl Default for acb_dft_bluestein_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type acb_dft_bluestein_t = [acb_dft_bluestein_struct; 1usize];
#[repr(C)]
pub struct acb_dft_prod_struct {
    pub n: mp_limb_signed_t,
    pub num: mp_limb_signed_t,
    pub cyc: acb_dft_step_ptr,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of acb_dft_prod_struct"][::std::mem::size_of::<acb_dft_prod_struct>() - 24usize];
    ["Alignment of acb_dft_prod_struct"][::std::mem::align_of::<acb_dft_prod_struct>() - 8usize];
    ["Offset of field: acb_dft_prod_struct::n"]
        [::std::mem::offset_of!(acb_dft_prod_struct, n) - 0usize];
    ["Offset of field: acb_dft_prod_struct::num"]
        [::std::mem::offset_of!(acb_dft_prod_struct, num) - 8usize];
    ["Offset of field: acb_dft_prod_struct::cyc"]
        [::std::mem::offset_of!(acb_dft_prod_struct, cyc) - 16usize];
};
impl Default for acb_dft_prod_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type acb_dft_prod_t = [acb_dft_prod_struct; 1usize];
#[repr(C)]
pub struct acb_dft_crt_struct {
    pub n: mp_limb_signed_t,
    pub c: crt_t,
    pub dv: mp_limb_signed_t,
    pub cyc: acb_dft_step_ptr,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of acb_dft_crt_struct"][::std::mem::size_of::<acb_dft_crt_struct>() - 416usize];
    ["Alignment of acb_dft_crt_struct"][::std::mem::align_of::<acb_dft_crt_struct>() - 8usize];
    ["Offset of field: acb_dft_crt_struct::n"]
        [::std::mem::offset_of!(acb_dft_crt_struct, n) - 0usize];
    ["Offset of field: acb_dft_crt_struct::c"]
        [::std::mem::offset_of!(acb_dft_crt_struct, c) - 8usize];
    ["Offset of field: acb_dft_crt_struct::dv"]
        [::std::mem::offset_of!(acb_dft_crt_struct, dv) - 400usize];
    ["Offset of field: acb_dft_crt_struct::cyc"]
        [::std::mem::offset_of!(acb_dft_crt_struct, cyc) - 408usize];
};
impl Default for acb_dft_crt_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type acb_dft_crt_t = [acb_dft_crt_struct; 1usize];
#[repr(C)]
pub struct acb_dft_naive_struct {
    pub n: mp_limb_signed_t,
    pub dv: mp_limb_signed_t,
    pub zclear: libc::c_int,
    pub z: acb_ptr,
    pub dz: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of acb_dft_naive_struct"][::std::mem::size_of::<acb_dft_naive_struct>() - 40usize];
    ["Alignment of acb_dft_naive_struct"][::std::mem::align_of::<acb_dft_naive_struct>() - 8usize];
    ["Offset of field: acb_dft_naive_struct::n"]
        [::std::mem::offset_of!(acb_dft_naive_struct, n) - 0usize];
    ["Offset of field: acb_dft_naive_struct::dv"]
        [::std::mem::offset_of!(acb_dft_naive_struct, dv) - 8usize];
    ["Offset of field: acb_dft_naive_struct::zclear"]
        [::std::mem::offset_of!(acb_dft_naive_struct, zclear) - 16usize];
    ["Offset of field: acb_dft_naive_struct::z"]
        [::std::mem::offset_of!(acb_dft_naive_struct, z) - 24usize];
    ["Offset of field: acb_dft_naive_struct::dz"]
        [::std::mem::offset_of!(acb_dft_naive_struct, dz) - 32usize];
};
impl Default for acb_dft_naive_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type acb_dft_naive_t = [acb_dft_naive_struct; 1usize];
#[repr(C)]
pub struct acb_dft_pre_struct {
    pub n: mp_limb_signed_t,
    pub type_: libc::c_int,
    pub t: acb_dft_pre_struct__bindgen_ty_1,
}
#[repr(C)]
pub struct acb_dft_pre_struct__bindgen_ty_1 {
    pub rad2: __BindgenUnionField<acb_dft_rad2_t>,
    pub cyc: __BindgenUnionField<acb_dft_cyc_t>,
    pub prod: __BindgenUnionField<acb_dft_prod_t>,
    pub crt: __BindgenUnionField<acb_dft_crt_t>,
    pub naive: __BindgenUnionField<acb_dft_naive_t>,
    pub bluestein: __BindgenUnionField<acb_dft_bluestein_t>,
    pub bindgen_union_field: [u64; 52usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of acb_dft_pre_struct__bindgen_ty_1"]
        [::std::mem::size_of::<acb_dft_pre_struct__bindgen_ty_1>() - 416usize];
    ["Alignment of acb_dft_pre_struct__bindgen_ty_1"]
        [::std::mem::align_of::<acb_dft_pre_struct__bindgen_ty_1>() - 8usize];
    ["Offset of field: acb_dft_pre_struct__bindgen_ty_1::rad2"]
        [::std::mem::offset_of!(acb_dft_pre_struct__bindgen_ty_1, rad2) - 0usize];
    ["Offset of field: acb_dft_pre_struct__bindgen_ty_1::cyc"]
        [::std::mem::offset_of!(acb_dft_pre_struct__bindgen_ty_1, cyc) - 0usize];
    ["Offset of field: acb_dft_pre_struct__bindgen_ty_1::prod"]
        [::std::mem::offset_of!(acb_dft_pre_struct__bindgen_ty_1, prod) - 0usize];
    ["Offset of field: acb_dft_pre_struct__bindgen_ty_1::crt"]
        [::std::mem::offset_of!(acb_dft_pre_struct__bindgen_ty_1, crt) - 0usize];
    ["Offset of field: acb_dft_pre_struct__bindgen_ty_1::naive"]
        [::std::mem::offset_of!(acb_dft_pre_struct__bindgen_ty_1, naive) - 0usize];
    ["Offset of field: acb_dft_pre_struct__bindgen_ty_1::bluestein"]
        [::std::mem::offset_of!(acb_dft_pre_struct__bindgen_ty_1, bluestein) - 0usize];
};
impl Default for acb_dft_pre_struct__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of acb_dft_pre_struct"][::std::mem::size_of::<acb_dft_pre_struct>() - 432usize];
    ["Alignment of acb_dft_pre_struct"][::std::mem::align_of::<acb_dft_pre_struct>() - 8usize];
    ["Offset of field: acb_dft_pre_struct::n"]
        [::std::mem::offset_of!(acb_dft_pre_struct, n) - 0usize];
    ["Offset of field: acb_dft_pre_struct::type_"]
        [::std::mem::offset_of!(acb_dft_pre_struct, type_) - 8usize];
    ["Offset of field: acb_dft_pre_struct::t"]
        [::std::mem::offset_of!(acb_dft_pre_struct, t) - 16usize];
};
impl Default for acb_dft_pre_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type acb_dft_pre_t = [acb_dft_pre_struct; 1usize];
#[repr(C)]
pub struct acb_dft_step_struct {
    pub m: mp_limb_signed_t,
    pub M: mp_limb_signed_t,
    pub dv: mp_limb_signed_t,
    pub z: acb_srcptr,
    pub dz: mp_limb_signed_t,
    pub pre: acb_dft_pre_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of acb_dft_step_struct"][::std::mem::size_of::<acb_dft_step_struct>() - 472usize];
    ["Alignment of acb_dft_step_struct"][::std::mem::align_of::<acb_dft_step_struct>() - 8usize];
    ["Offset of field: acb_dft_step_struct::m"]
        [::std::mem::offset_of!(acb_dft_step_struct, m) - 0usize];
    ["Offset of field: acb_dft_step_struct::M"]
        [::std::mem::offset_of!(acb_dft_step_struct, M) - 8usize];
    ["Offset of field: acb_dft_step_struct::dv"]
        [::std::mem::offset_of!(acb_dft_step_struct, dv) - 16usize];
    ["Offset of field: acb_dft_step_struct::z"]
        [::std::mem::offset_of!(acb_dft_step_struct, z) - 24usize];
    ["Offset of field: acb_dft_step_struct::dz"]
        [::std::mem::offset_of!(acb_dft_step_struct, dz) - 32usize];
    ["Offset of field: acb_dft_step_struct::pre"]
        [::std::mem::offset_of!(acb_dft_step_struct, pre) - 40usize];
};
impl Default for acb_dft_step_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const DFT_NAIVE: c_uint = 0;
pub const DFT_CYC: c_uint = 1;
pub const DFT_PROD: c_uint = 2;
pub const DFT_CRT: c_uint = 3;
pub const DFT_RAD2: c_uint = 4;
pub const DFT_CONV: c_uint = 5;
extern "C" {
    pub fn _acb_dft_naive(
        w: acb_ptr,
        v: acb_srcptr,
        dv: mp_limb_signed_t,
        z: acb_srcptr,
        dz: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_naive(w: acb_ptr, v: acb_srcptr, len: mp_limb_signed_t, prec: mp_limb_signed_t);
    pub fn acb_dft_crt(w: acb_ptr, v: acb_srcptr, len: mp_limb_signed_t, prec: mp_limb_signed_t);
    pub fn acb_dft_cyc(w: acb_ptr, v: acb_srcptr, len: mp_limb_signed_t, prec: mp_limb_signed_t);
    pub fn acb_dft_rad2_inplace(v: acb_ptr, e: libc::c_int, prec: mp_limb_signed_t);
    pub fn acb_dft_rad2(w: acb_ptr, v: acb_srcptr, e: libc::c_int, prec: mp_limb_signed_t);
    pub fn acb_dft_bluestein(
        w: acb_ptr,
        v: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_prod(
        w: acb_ptr,
        v: acb_srcptr,
        cyc: *mut mp_limb_signed_t,
        num: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_rad2_inplace_threaded(v: acb_ptr, e: libc::c_int, prec: mp_limb_signed_t);
    pub fn acb_dft_convol_naive(
        w: acb_ptr,
        f: acb_srcptr,
        g: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_convol_dft(
        w: acb_ptr,
        f: acb_srcptr,
        g: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_convol_rad2(
        w: acb_ptr,
        f: acb_srcptr,
        g: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_convol_mullow(
        w: acb_ptr,
        f: acb_srcptr,
        g: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_convol(
        w: acb_ptr,
        f: acb_srcptr,
        g: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn crt_init(c: *mut crt_struct, n: mp_limb_t);
    pub fn crt_decomp(
        y: acb_ptr,
        x: acb_srcptr,
        dx: mp_limb_signed_t,
        c: *const crt_struct,
        len: mp_limb_t,
    );
    pub fn crt_recomp(y: acb_ptr, x: acb_srcptr, c: *const crt_struct, len: mp_limb_t);
    pub fn acb_dft_step(
        w: acb_ptr,
        v: acb_srcptr,
        cyc: acb_dft_step_ptr,
        num: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_precomp(
        w: acb_ptr,
        v: acb_srcptr,
        pre: *const acb_dft_pre_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_inverse_precomp(
        w: acb_ptr,
        v: acb_srcptr,
        pre: *const acb_dft_pre_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_naive_precomp(
        w: acb_ptr,
        v: acb_srcptr,
        pol: *const acb_dft_naive_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_cyc_precomp(
        w: acb_ptr,
        v: acb_srcptr,
        cyc: *const acb_dft_cyc_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_rad2_precomp_inplace(
        v: acb_ptr,
        rad2: *const acb_dft_rad2_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_rad2_precomp(
        w: acb_ptr,
        v: acb_srcptr,
        rad2: *const acb_dft_rad2_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_crt_precomp(
        w: acb_ptr,
        v: acb_srcptr,
        crt: *const acb_dft_crt_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_prod_precomp(
        w: acb_ptr,
        v: acb_srcptr,
        prod: *const acb_dft_prod_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_bluestein_precomp(
        w: acb_ptr,
        v: acb_srcptr,
        t: *const acb_dft_bluestein_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_rad2_precomp_inplace_threaded(
        v: acb_ptr,
        rad2: *const acb_dft_rad2_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_inverse_rad2_precomp_inplace(
        v: acb_ptr,
        rad2: *const acb_dft_rad2_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_inverse_rad2_precomp(
        w: acb_ptr,
        v: acb_srcptr,
        rad2: *const acb_dft_rad2_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_convol_rad2_precomp(
        w: acb_ptr,
        f: acb_srcptr,
        g: acb_srcptr,
        len: mp_limb_signed_t,
        arg1: *const acb_dft_rad2_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_dft_precomp_init(
        pre: *mut acb_dft_pre_struct,
        dv: mp_limb_signed_t,
        z: acb_ptr,
        dz: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_precomp_init(
        pre: *mut acb_dft_pre_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_precomp_clear(pre: *mut acb_dft_pre_struct);
    pub fn acb_dft(w: acb_ptr, v: acb_srcptr, len: mp_limb_signed_t, prec: mp_limb_signed_t);
    pub fn acb_dft_inverse(
        w: acb_ptr,
        v: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_dft_steps_prod(
        m: *mut mp_limb_signed_t,
        num: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> acb_dft_step_ptr;
    #[link_name = "acb_dft_prod_init__extern"]
    pub fn acb_dft_prod_init(
        t: *mut acb_dft_prod_struct,
        cyc: *mut mp_limb_signed_t,
        num: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_prod_clear(t: *mut acb_dft_prod_struct);
    pub fn _acb_dft_cyc_init_z_fac(
        t: *mut acb_dft_cyc_struct,
        fac: n_factor_t,
        dv: mp_limb_signed_t,
        z: acb_ptr,
        dz: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_dft_cyc_init(
        t: *mut acb_dft_cyc_struct,
        dv: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    #[link_name = "acb_dft_cyc_init__extern"]
    pub fn acb_dft_cyc_init(
        t: *mut acb_dft_cyc_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_cyc_clear(t: *mut acb_dft_cyc_struct);
    pub fn _acb_dft_naive_init(
        pol: *mut acb_dft_naive_struct,
        dv: mp_limb_signed_t,
        z: acb_ptr,
        dz: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    #[link_name = "acb_dft_naive_init__extern"]
    pub fn acb_dft_naive_init(
        pol: *mut acb_dft_naive_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    #[link_name = "acb_dft_naive_clear__extern"]
    pub fn acb_dft_naive_clear(pol: *mut acb_dft_naive_struct);
    pub fn _acb_dft_rad2_init(
        t: *mut acb_dft_rad2_struct,
        dv: mp_limb_signed_t,
        e: libc::c_int,
        prec: mp_limb_signed_t,
    );
    #[link_name = "acb_dft_rad2_init__extern"]
    pub fn acb_dft_rad2_init(t: *mut acb_dft_rad2_struct, e: libc::c_int, prec: mp_limb_signed_t);
    #[link_name = "acb_dft_rad2_clear__extern"]
    pub fn acb_dft_rad2_clear(t: *mut acb_dft_rad2_struct);
    pub fn _acb_dft_bluestein_init(
        t: *mut acb_dft_bluestein_struct,
        dv: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    #[link_name = "acb_dft_bluestein_init__extern"]
    pub fn acb_dft_bluestein_init(
        t: *mut acb_dft_bluestein_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    #[link_name = "acb_dft_bluestein_clear__extern"]
    pub fn acb_dft_bluestein_clear(t: *mut acb_dft_bluestein_struct);
    pub fn _acb_dft_crt_init(
        crt: *mut acb_dft_crt_struct,
        dv: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_crt_init(
        crt: *mut acb_dft_crt_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_crt_clear(crt: *mut acb_dft_crt_struct);
    #[link_name = "acb_swap_ri__extern"]
    pub fn acb_swap_ri(x: *mut acb_struct);
    #[link_name = "acb_vec_swap_ri__extern"]
    pub fn acb_vec_swap_ri(v: acb_ptr, len: mp_limb_signed_t);
    #[link_name = "_acb_vec_kronecker_mul__extern"]
    pub fn _acb_vec_kronecker_mul(
        z: acb_ptr,
        x: acb_srcptr,
        y: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    #[link_name = "_acb_vec_kronecker_mul_step__extern"]
    pub fn _acb_vec_kronecker_mul_step(
        z: acb_ptr,
        x: acb_srcptr,
        y: acb_srcptr,
        step: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    #[link_name = "acb_vec_printd_index__extern"]
    pub fn acb_vec_printd_index(vec: acb_srcptr, len: mp_limb_signed_t, digits: mp_limb_signed_t);
}
