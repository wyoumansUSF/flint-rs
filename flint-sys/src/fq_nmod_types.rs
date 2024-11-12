/* automatically generated by rust-bindgen 0.70.1 */

#![allow(non_camel_case_types)]
use crate::deps::*;
use crate::flint::*;
use crate::nmod_types::*;
use libc::{c_char, c_int, c_uint, c_void, size_t, FILE};


pub type fq_nmod_t = nmod_poly_t;
pub type fq_nmod_struct = nmod_poly_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_nmod_ctx_struct {
    pub mod_: nmod_t,
    pub sparse_modulus: ::std::os::raw::c_int,
    pub is_conway: ::std::os::raw::c_int,
    pub a: *mut mp_limb_t,
    pub j: *mut mp_limb_signed_t,
    pub len: mp_limb_signed_t,
    pub modulus: nmod_poly_t,
    pub inv: nmod_poly_t,
    pub var: *mut ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_nmod_ctx_struct"][::std::mem::size_of::<fq_nmod_ctx_struct>() - 160usize];
    ["Alignment of fq_nmod_ctx_struct"][::std::mem::align_of::<fq_nmod_ctx_struct>() - 8usize];
    ["Offset of field: fq_nmod_ctx_struct::mod_"]
        [::std::mem::offset_of!(fq_nmod_ctx_struct, mod_) - 0usize];
    ["Offset of field: fq_nmod_ctx_struct::sparse_modulus"]
        [::std::mem::offset_of!(fq_nmod_ctx_struct, sparse_modulus) - 24usize];
    ["Offset of field: fq_nmod_ctx_struct::is_conway"]
        [::std::mem::offset_of!(fq_nmod_ctx_struct, is_conway) - 28usize];
    ["Offset of field: fq_nmod_ctx_struct::a"]
        [::std::mem::offset_of!(fq_nmod_ctx_struct, a) - 32usize];
    ["Offset of field: fq_nmod_ctx_struct::j"]
        [::std::mem::offset_of!(fq_nmod_ctx_struct, j) - 40usize];
    ["Offset of field: fq_nmod_ctx_struct::len"]
        [::std::mem::offset_of!(fq_nmod_ctx_struct, len) - 48usize];
    ["Offset of field: fq_nmod_ctx_struct::modulus"]
        [::std::mem::offset_of!(fq_nmod_ctx_struct, modulus) - 56usize];
    ["Offset of field: fq_nmod_ctx_struct::inv"]
        [::std::mem::offset_of!(fq_nmod_ctx_struct, inv) - 104usize];
    ["Offset of field: fq_nmod_ctx_struct::var"]
        [::std::mem::offset_of!(fq_nmod_ctx_struct, var) - 152usize];
};
impl Default for fq_nmod_ctx_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_nmod_ctx_t = [fq_nmod_ctx_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_nmod_mat_struct {
    pub entries: *mut fq_nmod_struct,
    pub r: mp_limb_signed_t,
    pub c: mp_limb_signed_t,
    pub rows: *mut *mut fq_nmod_struct,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_nmod_mat_struct"][::std::mem::size_of::<fq_nmod_mat_struct>() - 32usize];
    ["Alignment of fq_nmod_mat_struct"][::std::mem::align_of::<fq_nmod_mat_struct>() - 8usize];
    ["Offset of field: fq_nmod_mat_struct::entries"]
        [::std::mem::offset_of!(fq_nmod_mat_struct, entries) - 0usize];
    ["Offset of field: fq_nmod_mat_struct::r"]
        [::std::mem::offset_of!(fq_nmod_mat_struct, r) - 8usize];
    ["Offset of field: fq_nmod_mat_struct::c"]
        [::std::mem::offset_of!(fq_nmod_mat_struct, c) - 16usize];
    ["Offset of field: fq_nmod_mat_struct::rows"]
        [::std::mem::offset_of!(fq_nmod_mat_struct, rows) - 24usize];
};
impl Default for fq_nmod_mat_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_nmod_mat_t = [fq_nmod_mat_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_nmod_poly_struct {
    pub coeffs: *mut fq_nmod_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_nmod_poly_struct"][::std::mem::size_of::<fq_nmod_poly_struct>() - 24usize];
    ["Alignment of fq_nmod_poly_struct"][::std::mem::align_of::<fq_nmod_poly_struct>() - 8usize];
    ["Offset of field: fq_nmod_poly_struct::coeffs"]
        [::std::mem::offset_of!(fq_nmod_poly_struct, coeffs) - 0usize];
    ["Offset of field: fq_nmod_poly_struct::alloc"]
        [::std::mem::offset_of!(fq_nmod_poly_struct, alloc) - 8usize];
    ["Offset of field: fq_nmod_poly_struct::length"]
        [::std::mem::offset_of!(fq_nmod_poly_struct, length) - 16usize];
};
impl Default for fq_nmod_poly_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_nmod_poly_t = [fq_nmod_poly_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_nmod_poly_factor_struct {
    pub poly: *mut fq_nmod_poly_struct,
    pub exp: *mut mp_limb_signed_t,
    pub num: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_nmod_poly_factor_struct"]
        [::std::mem::size_of::<fq_nmod_poly_factor_struct>() - 32usize];
    ["Alignment of fq_nmod_poly_factor_struct"]
        [::std::mem::align_of::<fq_nmod_poly_factor_struct>() - 8usize];
    ["Offset of field: fq_nmod_poly_factor_struct::poly"]
        [::std::mem::offset_of!(fq_nmod_poly_factor_struct, poly) - 0usize];
    ["Offset of field: fq_nmod_poly_factor_struct::exp"]
        [::std::mem::offset_of!(fq_nmod_poly_factor_struct, exp) - 8usize];
    ["Offset of field: fq_nmod_poly_factor_struct::num"]
        [::std::mem::offset_of!(fq_nmod_poly_factor_struct, num) - 16usize];
    ["Offset of field: fq_nmod_poly_factor_struct::alloc"]
        [::std::mem::offset_of!(fq_nmod_poly_factor_struct, alloc) - 24usize];
};
impl Default for fq_nmod_poly_factor_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_nmod_poly_factor_t = [fq_nmod_poly_factor_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_nmod_mpoly_struct {
    pub coeffs: *mut mp_limb_t,
    pub exps: *mut mp_limb_t,
    pub length: mp_limb_signed_t,
    pub bits: mp_limb_t,
    pub coeffs_alloc: mp_limb_signed_t,
    pub exps_alloc: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_nmod_mpoly_struct"][::std::mem::size_of::<fq_nmod_mpoly_struct>() - 48usize];
    ["Alignment of fq_nmod_mpoly_struct"][::std::mem::align_of::<fq_nmod_mpoly_struct>() - 8usize];
    ["Offset of field: fq_nmod_mpoly_struct::coeffs"]
        [::std::mem::offset_of!(fq_nmod_mpoly_struct, coeffs) - 0usize];
    ["Offset of field: fq_nmod_mpoly_struct::exps"]
        [::std::mem::offset_of!(fq_nmod_mpoly_struct, exps) - 8usize];
    ["Offset of field: fq_nmod_mpoly_struct::length"]
        [::std::mem::offset_of!(fq_nmod_mpoly_struct, length) - 16usize];
    ["Offset of field: fq_nmod_mpoly_struct::bits"]
        [::std::mem::offset_of!(fq_nmod_mpoly_struct, bits) - 24usize];
    ["Offset of field: fq_nmod_mpoly_struct::coeffs_alloc"]
        [::std::mem::offset_of!(fq_nmod_mpoly_struct, coeffs_alloc) - 32usize];
    ["Offset of field: fq_nmod_mpoly_struct::exps_alloc"]
        [::std::mem::offset_of!(fq_nmod_mpoly_struct, exps_alloc) - 40usize];
};
impl Default for fq_nmod_mpoly_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_nmod_mpoly_t = [fq_nmod_mpoly_struct; 1usize];
