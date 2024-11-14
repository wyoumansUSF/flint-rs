/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;
use crate::bindgen::*;
use crate::flint::*;
use crate::fmpq_poly::*;
use crate::fmpq_types::*;
use crate::fmpz_poly::*;
use crate::fmpz_types::*;


pub const NF_POWERS_CUTOFF: u32 = 30;
pub const NF_GENERIC: u32 = 0;
pub const NF_MONIC: u32 = 1;
pub const NF_LINEAR: u32 = 2;
pub const NF_QUADRATIC: u32 = 4;
pub const NF_GAUSSIAN: u32 = 8;
#[repr(C)]
pub struct nf_struct {
    pub pol: fmpq_poly_t,
    pub pinv: nf_struct__bindgen_ty_1,
    pub powers: nf_struct__bindgen_ty_2,
    pub traces: fmpq_poly_t,
    pub flag: mp_limb_t,
}
#[repr(C)]
pub struct nf_struct__bindgen_ty_1 {
    pub qq: __BindgenUnionField<fmpz_preinvn_t>,
    pub bindgen_union_field: [u64; 3usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of nf_struct__bindgen_ty_1"][::std::mem::size_of::<nf_struct__bindgen_ty_1>() - 24usize];
    ["Alignment of nf_struct__bindgen_ty_1"]
        [::std::mem::align_of::<nf_struct__bindgen_ty_1>() - 8usize];
    ["Offset of field: nf_struct__bindgen_ty_1::qq"]
        [::std::mem::offset_of!(nf_struct__bindgen_ty_1, qq) - 0usize];
};
impl Default for nf_struct__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct nf_struct__bindgen_ty_2 {
    pub qq: __BindgenUnionField<fmpq_poly_powers_precomp_t>,
    pub zz: __BindgenUnionField<fmpz_poly_powers_precomp_t>,
    pub bindgen_union_field: [u64; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of nf_struct__bindgen_ty_2"][::std::mem::size_of::<nf_struct__bindgen_ty_2>() - 16usize];
    ["Alignment of nf_struct__bindgen_ty_2"]
        [::std::mem::align_of::<nf_struct__bindgen_ty_2>() - 8usize];
    ["Offset of field: nf_struct__bindgen_ty_2::qq"]
        [::std::mem::offset_of!(nf_struct__bindgen_ty_2, qq) - 0usize];
    ["Offset of field: nf_struct__bindgen_ty_2::zz"]
        [::std::mem::offset_of!(nf_struct__bindgen_ty_2, zz) - 0usize];
};
impl Default for nf_struct__bindgen_ty_2 {
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
    ["Size of nf_struct"][::std::mem::size_of::<nf_struct>() - 112usize];
    ["Alignment of nf_struct"][::std::mem::align_of::<nf_struct>() - 8usize];
    ["Offset of field: nf_struct::pol"][::std::mem::offset_of!(nf_struct, pol) - 0usize];
    ["Offset of field: nf_struct::pinv"][::std::mem::offset_of!(nf_struct, pinv) - 32usize];
    ["Offset of field: nf_struct::powers"][::std::mem::offset_of!(nf_struct, powers) - 56usize];
    ["Offset of field: nf_struct::traces"][::std::mem::offset_of!(nf_struct, traces) - 72usize];
    ["Offset of field: nf_struct::flag"][::std::mem::offset_of!(nf_struct, flag) - 104usize];
};
impl Default for nf_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type nf_t = [nf_struct; 1usize];
extern "C" {
    #[doc = "Initialisation"]
    pub fn nf_init(nf: *mut nf_struct, pol: *const fmpq_poly_struct);
    pub fn nf_init_randtest(
        nf: *mut nf_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        bits_in: mp_bitcnt_t,
    );
    pub fn nf_clear(nf: *mut nf_struct);
    pub fn nf_print(nf: *const nf_struct);
}
