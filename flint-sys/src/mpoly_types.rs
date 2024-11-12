/* automatically generated by rust-bindgen 0.70.1 */

#![allow(non_camel_case_types)]
use crate::deps::*;
use libc::{c_char, c_int, c_uint, c_void, size_t, FILE};


pub const MPOLY_NUM_ORDERINGS: u32 = 3;
pub const ordering_t_ORD_LEX: ordering_t = 0;
pub const ordering_t_ORD_DEGLEX: ordering_t = 1;
pub const ordering_t_ORD_DEGREVLEX: ordering_t = 2;
pub type ordering_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpoly_ctx_struct {
    pub nvars: mp_limb_signed_t,
    pub nfields: mp_limb_signed_t,
    pub ord: ordering_t,
    pub deg: ::std::os::raw::c_int,
    pub rev: ::std::os::raw::c_int,
    pub lut_words_per_exp: [mp_limb_signed_t; 64usize],
    pub lut_fix_bits: [::std::os::raw::c_uchar; 64usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of mpoly_ctx_struct"][::std::mem::size_of::<mpoly_ctx_struct>() - 608usize];
    ["Alignment of mpoly_ctx_struct"][::std::mem::align_of::<mpoly_ctx_struct>() - 8usize];
    ["Offset of field: mpoly_ctx_struct::nvars"]
        [::std::mem::offset_of!(mpoly_ctx_struct, nvars) - 0usize];
    ["Offset of field: mpoly_ctx_struct::nfields"]
        [::std::mem::offset_of!(mpoly_ctx_struct, nfields) - 8usize];
    ["Offset of field: mpoly_ctx_struct::ord"]
        [::std::mem::offset_of!(mpoly_ctx_struct, ord) - 16usize];
    ["Offset of field: mpoly_ctx_struct::deg"]
        [::std::mem::offset_of!(mpoly_ctx_struct, deg) - 20usize];
    ["Offset of field: mpoly_ctx_struct::rev"]
        [::std::mem::offset_of!(mpoly_ctx_struct, rev) - 24usize];
    ["Offset of field: mpoly_ctx_struct::lut_words_per_exp"]
        [::std::mem::offset_of!(mpoly_ctx_struct, lut_words_per_exp) - 32usize];
    ["Offset of field: mpoly_ctx_struct::lut_fix_bits"]
        [::std::mem::offset_of!(mpoly_ctx_struct, lut_fix_bits) - 544usize];
};
impl Default for mpoly_ctx_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type mpoly_ctx_t = [mpoly_ctx_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_mpoly_ctx_struct {
    pub minfo: mpoly_ctx_t,
    pub mod_: nmod_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of nmod_mpoly_ctx_struct"][::std::mem::size_of::<nmod_mpoly_ctx_struct>() - 632usize];
    ["Alignment of nmod_mpoly_ctx_struct"]
        [::std::mem::align_of::<nmod_mpoly_ctx_struct>() - 8usize];
    ["Offset of field: nmod_mpoly_ctx_struct::minfo"]
        [::std::mem::offset_of!(nmod_mpoly_ctx_struct, minfo) - 0usize];
    ["Offset of field: nmod_mpoly_ctx_struct::mod_"]
        [::std::mem::offset_of!(nmod_mpoly_ctx_struct, mod_) - 608usize];
};
impl Default for nmod_mpoly_ctx_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type nmod_mpoly_ctx_t = [nmod_mpoly_ctx_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mpoly_ctx_struct {
    pub minfo: mpoly_ctx_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fmpz_mpoly_ctx_struct"][::std::mem::size_of::<fmpz_mpoly_ctx_struct>() - 608usize];
    ["Alignment of fmpz_mpoly_ctx_struct"]
        [::std::mem::align_of::<fmpz_mpoly_ctx_struct>() - 8usize];
    ["Offset of field: fmpz_mpoly_ctx_struct::minfo"]
        [::std::mem::offset_of!(fmpz_mpoly_ctx_struct, minfo) - 0usize];
};
impl Default for fmpz_mpoly_ctx_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fmpz_mpoly_ctx_t = [fmpz_mpoly_ctx_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpq_mpoly_ctx_struct {
    pub zctx: fmpz_mpoly_ctx_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fmpq_mpoly_ctx_struct"][::std::mem::size_of::<fmpq_mpoly_ctx_struct>() - 608usize];
    ["Alignment of fmpq_mpoly_ctx_struct"]
        [::std::mem::align_of::<fmpq_mpoly_ctx_struct>() - 8usize];
    ["Offset of field: fmpq_mpoly_ctx_struct::zctx"]
        [::std::mem::offset_of!(fmpq_mpoly_ctx_struct, zctx) - 0usize];
};
impl Default for fmpq_mpoly_ctx_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fmpq_mpoly_ctx_t = [fmpq_mpoly_ctx_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_mpoly_ctx_struct {
    pub minfo: mpoly_ctx_t,
    pub ffinfo: fmpz_mod_ctx_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fmpz_mod_mpoly_ctx_struct"]
        [::std::mem::size_of::<fmpz_mod_mpoly_ctx_struct>() - 720usize];
    ["Alignment of fmpz_mod_mpoly_ctx_struct"]
        [::std::mem::align_of::<fmpz_mod_mpoly_ctx_struct>() - 8usize];
    ["Offset of field: fmpz_mod_mpoly_ctx_struct::minfo"]
        [::std::mem::offset_of!(fmpz_mod_mpoly_ctx_struct, minfo) - 0usize];
    ["Offset of field: fmpz_mod_mpoly_ctx_struct::ffinfo"]
        [::std::mem::offset_of!(fmpz_mod_mpoly_ctx_struct, ffinfo) - 608usize];
};
impl Default for fmpz_mod_mpoly_ctx_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fmpz_mod_mpoly_ctx_t = [fmpz_mod_mpoly_ctx_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_nmod_mpoly_ctx_struct {
    pub minfo: mpoly_ctx_t,
    pub fqctx: fq_nmod_ctx_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_nmod_mpoly_ctx_struct"]
        [::std::mem::size_of::<fq_nmod_mpoly_ctx_struct>() - 768usize];
    ["Alignment of fq_nmod_mpoly_ctx_struct"]
        [::std::mem::align_of::<fq_nmod_mpoly_ctx_struct>() - 8usize];
    ["Offset of field: fq_nmod_mpoly_ctx_struct::minfo"]
        [::std::mem::offset_of!(fq_nmod_mpoly_ctx_struct, minfo) - 0usize];
    ["Offset of field: fq_nmod_mpoly_ctx_struct::fqctx"]
        [::std::mem::offset_of!(fq_nmod_mpoly_ctx_struct, fqctx) - 608usize];
};
impl Default for fq_nmod_mpoly_ctx_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_nmod_mpoly_ctx_t = [fq_nmod_mpoly_ctx_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_2 {
    pub elem_size: mp_limb_signed_t,
    pub ctx: *const ::std::os::raw::c_void,
    pub init: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_void,
        ),
    >,
    pub clear: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_void,
        ),
    >,
    pub is_zero: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub zero: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_void,
        ),
    >,
    pub one: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_void,
        ),
    >,
    pub set_fmpz: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const fmpz,
            arg3: *const ::std::os::raw::c_void,
        ),
    >,
    pub set: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_void,
            arg3: *const ::std::os::raw::c_void,
        ),
    >,
    pub swap: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_void,
            arg3: *const ::std::os::raw::c_void,
        ),
    >,
    pub neg: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_void,
            arg3: *const ::std::os::raw::c_void,
        ),
    >,
    pub add: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_void,
            arg3: *const ::std::os::raw::c_void,
            arg4: *const ::std::os::raw::c_void,
        ),
    >,
    pub sub: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_void,
            arg3: *const ::std::os::raw::c_void,
            arg4: *const ::std::os::raw::c_void,
        ),
    >,
    pub mul_fmpz: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_void,
            arg3: *const fmpz,
            arg4: *const ::std::os::raw::c_void,
        ),
    >,
    pub mul: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_void,
            arg3: *const ::std::os::raw::c_void,
            arg4: *const ::std::os::raw::c_void,
        ),
    >,
    pub divexact: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_void,
            arg3: *const ::std::os::raw::c_void,
            arg4: *const ::std::os::raw::c_void,
        ),
    >,
    pub divides: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_void,
            arg3: *const ::std::os::raw::c_void,
            arg4: *const ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub pow_fmpz: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_void,
            arg3: *const fmpz,
            arg4: *const ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub length: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_void,
        ) -> mp_limb_signed_t,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _bindgen_ty_2"][::std::mem::size_of::<_bindgen_ty_2>() - 152usize];
    ["Alignment of _bindgen_ty_2"][::std::mem::align_of::<_bindgen_ty_2>() - 8usize];
    ["Offset of field: _bindgen_ty_2::elem_size"]
        [::std::mem::offset_of!(_bindgen_ty_2, elem_size) - 0usize];
    ["Offset of field: _bindgen_ty_2::ctx"][::std::mem::offset_of!(_bindgen_ty_2, ctx) - 8usize];
    ["Offset of field: _bindgen_ty_2::init"][::std::mem::offset_of!(_bindgen_ty_2, init) - 16usize];
    ["Offset of field: _bindgen_ty_2::clear"]
        [::std::mem::offset_of!(_bindgen_ty_2, clear) - 24usize];
    ["Offset of field: _bindgen_ty_2::is_zero"]
        [::std::mem::offset_of!(_bindgen_ty_2, is_zero) - 32usize];
    ["Offset of field: _bindgen_ty_2::zero"][::std::mem::offset_of!(_bindgen_ty_2, zero) - 40usize];
    ["Offset of field: _bindgen_ty_2::one"][::std::mem::offset_of!(_bindgen_ty_2, one) - 48usize];
    ["Offset of field: _bindgen_ty_2::set_fmpz"]
        [::std::mem::offset_of!(_bindgen_ty_2, set_fmpz) - 56usize];
    ["Offset of field: _bindgen_ty_2::set"][::std::mem::offset_of!(_bindgen_ty_2, set) - 64usize];
    ["Offset of field: _bindgen_ty_2::swap"][::std::mem::offset_of!(_bindgen_ty_2, swap) - 72usize];
    ["Offset of field: _bindgen_ty_2::neg"][::std::mem::offset_of!(_bindgen_ty_2, neg) - 80usize];
    ["Offset of field: _bindgen_ty_2::add"][::std::mem::offset_of!(_bindgen_ty_2, add) - 88usize];
    ["Offset of field: _bindgen_ty_2::sub"][::std::mem::offset_of!(_bindgen_ty_2, sub) - 96usize];
    ["Offset of field: _bindgen_ty_2::mul_fmpz"]
        [::std::mem::offset_of!(_bindgen_ty_2, mul_fmpz) - 104usize];
    ["Offset of field: _bindgen_ty_2::mul"][::std::mem::offset_of!(_bindgen_ty_2, mul) - 112usize];
    ["Offset of field: _bindgen_ty_2::divexact"]
        [::std::mem::offset_of!(_bindgen_ty_2, divexact) - 120usize];
    ["Offset of field: _bindgen_ty_2::divides"]
        [::std::mem::offset_of!(_bindgen_ty_2, divides) - 128usize];
    ["Offset of field: _bindgen_ty_2::pow_fmpz"]
        [::std::mem::offset_of!(_bindgen_ty_2, pow_fmpz) - 136usize];
    ["Offset of field: _bindgen_ty_2::length"]
        [::std::mem::offset_of!(_bindgen_ty_2, length) - 144usize];
};
impl Default for _bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type mpoly_void_ring_t = [_bindgen_ty_2; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpoly_gcd_info_struct {
    pub Amax_exp: *mut mp_limb_t,
    pub Amin_exp: *mut mp_limb_t,
    pub Astride: *mut mp_limb_t,
    pub Adeflate_deg: *mut mp_limb_signed_t,
    pub Alead_count: *mut mp_limb_signed_t,
    pub Atail_count: *mut mp_limb_signed_t,
    pub Bmax_exp: *mut mp_limb_t,
    pub Bmin_exp: *mut mp_limb_t,
    pub Bstride: *mut mp_limb_t,
    pub Bdeflate_deg: *mut mp_limb_signed_t,
    pub Blead_count: *mut mp_limb_signed_t,
    pub Btail_count: *mut mp_limb_signed_t,
    pub Gmin_exp: *mut mp_limb_t,
    pub Abarmin_exp: *mut mp_limb_t,
    pub Bbarmin_exp: *mut mp_limb_t,
    pub Gstride: *mut mp_limb_t,
    pub Gterm_count_est: *mut mp_limb_signed_t,
    pub Gdeflate_deg_bound: *mut mp_limb_signed_t,
    pub Gbits: mp_limb_t,
    pub Abarbits: mp_limb_t,
    pub Bbarbits: mp_limb_t,
    pub mvars: mp_limb_signed_t,
    pub Adeflate_tdeg: mp_limb_signed_t,
    pub Bdeflate_tdeg: mp_limb_signed_t,
    pub Adensity: f64,
    pub Bdensity: f64,
    pub hensel_time: f64,
    pub brown_time: f64,
    pub zippel_time: f64,
    pub zippel2_time: f64,
    pub hensel_perm: *mut mp_limb_signed_t,
    pub brown_perm: *mut mp_limb_signed_t,
    pub zippel_perm: *mut mp_limb_signed_t,
    pub zippel2_perm: *mut mp_limb_signed_t,
    pub can_use: ::std::os::raw::c_uint,
    pub Gdeflate_deg_bounds_are_nice: ::std::os::raw::c_int,
    pub data: *mut ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of mpoly_gcd_info_struct"][::std::mem::size_of::<mpoly_gcd_info_struct>() - 288usize];
    ["Alignment of mpoly_gcd_info_struct"]
        [::std::mem::align_of::<mpoly_gcd_info_struct>() - 8usize];
    ["Offset of field: mpoly_gcd_info_struct::Amax_exp"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Amax_exp) - 0usize];
    ["Offset of field: mpoly_gcd_info_struct::Amin_exp"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Amin_exp) - 8usize];
    ["Offset of field: mpoly_gcd_info_struct::Astride"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Astride) - 16usize];
    ["Offset of field: mpoly_gcd_info_struct::Adeflate_deg"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Adeflate_deg) - 24usize];
    ["Offset of field: mpoly_gcd_info_struct::Alead_count"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Alead_count) - 32usize];
    ["Offset of field: mpoly_gcd_info_struct::Atail_count"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Atail_count) - 40usize];
    ["Offset of field: mpoly_gcd_info_struct::Bmax_exp"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Bmax_exp) - 48usize];
    ["Offset of field: mpoly_gcd_info_struct::Bmin_exp"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Bmin_exp) - 56usize];
    ["Offset of field: mpoly_gcd_info_struct::Bstride"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Bstride) - 64usize];
    ["Offset of field: mpoly_gcd_info_struct::Bdeflate_deg"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Bdeflate_deg) - 72usize];
    ["Offset of field: mpoly_gcd_info_struct::Blead_count"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Blead_count) - 80usize];
    ["Offset of field: mpoly_gcd_info_struct::Btail_count"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Btail_count) - 88usize];
    ["Offset of field: mpoly_gcd_info_struct::Gmin_exp"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Gmin_exp) - 96usize];
    ["Offset of field: mpoly_gcd_info_struct::Abarmin_exp"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Abarmin_exp) - 104usize];
    ["Offset of field: mpoly_gcd_info_struct::Bbarmin_exp"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Bbarmin_exp) - 112usize];
    ["Offset of field: mpoly_gcd_info_struct::Gstride"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Gstride) - 120usize];
    ["Offset of field: mpoly_gcd_info_struct::Gterm_count_est"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Gterm_count_est) - 128usize];
    ["Offset of field: mpoly_gcd_info_struct::Gdeflate_deg_bound"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Gdeflate_deg_bound) - 136usize];
    ["Offset of field: mpoly_gcd_info_struct::Gbits"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Gbits) - 144usize];
    ["Offset of field: mpoly_gcd_info_struct::Abarbits"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Abarbits) - 152usize];
    ["Offset of field: mpoly_gcd_info_struct::Bbarbits"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Bbarbits) - 160usize];
    ["Offset of field: mpoly_gcd_info_struct::mvars"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, mvars) - 168usize];
    ["Offset of field: mpoly_gcd_info_struct::Adeflate_tdeg"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Adeflate_tdeg) - 176usize];
    ["Offset of field: mpoly_gcd_info_struct::Bdeflate_tdeg"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Bdeflate_tdeg) - 184usize];
    ["Offset of field: mpoly_gcd_info_struct::Adensity"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Adensity) - 192usize];
    ["Offset of field: mpoly_gcd_info_struct::Bdensity"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Bdensity) - 200usize];
    ["Offset of field: mpoly_gcd_info_struct::hensel_time"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, hensel_time) - 208usize];
    ["Offset of field: mpoly_gcd_info_struct::brown_time"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, brown_time) - 216usize];
    ["Offset of field: mpoly_gcd_info_struct::zippel_time"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, zippel_time) - 224usize];
    ["Offset of field: mpoly_gcd_info_struct::zippel2_time"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, zippel2_time) - 232usize];
    ["Offset of field: mpoly_gcd_info_struct::hensel_perm"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, hensel_perm) - 240usize];
    ["Offset of field: mpoly_gcd_info_struct::brown_perm"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, brown_perm) - 248usize];
    ["Offset of field: mpoly_gcd_info_struct::zippel_perm"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, zippel_perm) - 256usize];
    ["Offset of field: mpoly_gcd_info_struct::zippel2_perm"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, zippel2_perm) - 264usize];
    ["Offset of field: mpoly_gcd_info_struct::can_use"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, can_use) - 272usize];
    ["Offset of field: mpoly_gcd_info_struct::Gdeflate_deg_bounds_are_nice"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, Gdeflate_deg_bounds_are_nice) - 276usize];
    ["Offset of field: mpoly_gcd_info_struct::data"]
        [::std::mem::offset_of!(mpoly_gcd_info_struct, data) - 280usize];
};
impl Default for mpoly_gcd_info_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type mpoly_gcd_info_t = [mpoly_gcd_info_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpoly_compression_struct {
    pub mvars: mp_limb_signed_t,
    pub nvars: mp_limb_signed_t,
    pub exps: *mut mp_limb_signed_t,
    pub exps_alloc: mp_limb_signed_t,
    pub rest: *mut mp_limb_signed_t,
    pub rest_alloc: mp_limb_signed_t,
    pub umat: *mut mp_limb_signed_t,
    pub deltas: *mut mp_limb_signed_t,
    pub degs: *mut mp_limb_signed_t,
    pub is_trivial: ::std::os::raw::c_int,
    pub is_perm: ::std::os::raw::c_int,
    pub is_irred: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of mpoly_compression_struct"]
        [::std::mem::size_of::<mpoly_compression_struct>() - 88usize];
    ["Alignment of mpoly_compression_struct"]
        [::std::mem::align_of::<mpoly_compression_struct>() - 8usize];
    ["Offset of field: mpoly_compression_struct::mvars"]
        [::std::mem::offset_of!(mpoly_compression_struct, mvars) - 0usize];
    ["Offset of field: mpoly_compression_struct::nvars"]
        [::std::mem::offset_of!(mpoly_compression_struct, nvars) - 8usize];
    ["Offset of field: mpoly_compression_struct::exps"]
        [::std::mem::offset_of!(mpoly_compression_struct, exps) - 16usize];
    ["Offset of field: mpoly_compression_struct::exps_alloc"]
        [::std::mem::offset_of!(mpoly_compression_struct, exps_alloc) - 24usize];
    ["Offset of field: mpoly_compression_struct::rest"]
        [::std::mem::offset_of!(mpoly_compression_struct, rest) - 32usize];
    ["Offset of field: mpoly_compression_struct::rest_alloc"]
        [::std::mem::offset_of!(mpoly_compression_struct, rest_alloc) - 40usize];
    ["Offset of field: mpoly_compression_struct::umat"]
        [::std::mem::offset_of!(mpoly_compression_struct, umat) - 48usize];
    ["Offset of field: mpoly_compression_struct::deltas"]
        [::std::mem::offset_of!(mpoly_compression_struct, deltas) - 56usize];
    ["Offset of field: mpoly_compression_struct::degs"]
        [::std::mem::offset_of!(mpoly_compression_struct, degs) - 64usize];
    ["Offset of field: mpoly_compression_struct::is_trivial"]
        [::std::mem::offset_of!(mpoly_compression_struct, is_trivial) - 72usize];
    ["Offset of field: mpoly_compression_struct::is_perm"]
        [::std::mem::offset_of!(mpoly_compression_struct, is_perm) - 76usize];
    ["Offset of field: mpoly_compression_struct::is_irred"]
        [::std::mem::offset_of!(mpoly_compression_struct, is_irred) - 80usize];
};
impl Default for mpoly_compression_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type mpoly_compression_t = [mpoly_compression_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_mpolyn_struct {
    pub coeffs: *mut n_poly_struct,
    pub exps: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of nmod_mpolyn_struct"][::std::mem::size_of::<nmod_mpolyn_struct>() - 40usize];
    ["Alignment of nmod_mpolyn_struct"][::std::mem::align_of::<nmod_mpolyn_struct>() - 8usize];
    ["Offset of field: nmod_mpolyn_struct::coeffs"]
        [::std::mem::offset_of!(nmod_mpolyn_struct, coeffs) - 0usize];
    ["Offset of field: nmod_mpolyn_struct::exps"]
        [::std::mem::offset_of!(nmod_mpolyn_struct, exps) - 8usize];
    ["Offset of field: nmod_mpolyn_struct::alloc"]
        [::std::mem::offset_of!(nmod_mpolyn_struct, alloc) - 16usize];
    ["Offset of field: nmod_mpolyn_struct::length"]
        [::std::mem::offset_of!(nmod_mpolyn_struct, length) - 24usize];
    ["Offset of field: nmod_mpolyn_struct::bits"]
        [::std::mem::offset_of!(nmod_mpolyn_struct, bits) - 32usize];
};
impl Default for nmod_mpolyn_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type nmod_mpolyn_t = [nmod_mpolyn_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_mpolyun_struct {
    pub coeffs: *mut nmod_mpolyn_struct,
    pub exps: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of nmod_mpolyun_struct"][::std::mem::size_of::<nmod_mpolyun_struct>() - 40usize];
    ["Alignment of nmod_mpolyun_struct"][::std::mem::align_of::<nmod_mpolyun_struct>() - 8usize];
    ["Offset of field: nmod_mpolyun_struct::coeffs"]
        [::std::mem::offset_of!(nmod_mpolyun_struct, coeffs) - 0usize];
    ["Offset of field: nmod_mpolyun_struct::exps"]
        [::std::mem::offset_of!(nmod_mpolyun_struct, exps) - 8usize];
    ["Offset of field: nmod_mpolyun_struct::alloc"]
        [::std::mem::offset_of!(nmod_mpolyun_struct, alloc) - 16usize];
    ["Offset of field: nmod_mpolyun_struct::length"]
        [::std::mem::offset_of!(nmod_mpolyun_struct, length) - 24usize];
    ["Offset of field: nmod_mpolyun_struct::bits"]
        [::std::mem::offset_of!(nmod_mpolyun_struct, bits) - 32usize];
};
impl Default for nmod_mpolyun_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type nmod_mpolyun_t = [nmod_mpolyun_struct; 1usize];
pub const nmod_gcds_ret_t_nmod_gcds_success: nmod_gcds_ret_t = 0;
pub const nmod_gcds_ret_t_nmod_gcds_form_main_degree_too_high: nmod_gcds_ret_t = 1;
pub const nmod_gcds_ret_t_nmod_gcds_form_wrong: nmod_gcds_ret_t = 2;
pub const nmod_gcds_ret_t_nmod_gcds_no_solution: nmod_gcds_ret_t = 3;
pub const nmod_gcds_ret_t_nmod_gcds_scales_not_found: nmod_gcds_ret_t = 4;
pub const nmod_gcds_ret_t_nmod_gcds_eval_point_not_found: nmod_gcds_ret_t = 5;
pub const nmod_gcds_ret_t_nmod_gcds_eval_gcd_deg_too_high: nmod_gcds_ret_t = 6;
pub type nmod_gcds_ret_t = ::std::os::raw::c_uint;
