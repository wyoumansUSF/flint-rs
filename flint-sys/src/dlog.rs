/* automatically generated by rust-bindgen 0.70.1 */

use libc::*;
use crate::deps::*;
use crate::bindgen::*;
use crate::flint::*;
use crate::limb_types::*;


pub const DLOG_SMALL_LIM: u32 = 50;
pub const DLOG_TABLE_LIM: u32 = 50;
pub const DLOG_TABLE_P_LIM: u32 = 50;
pub const DLOG_TABLE_MODPE_LIM: u32 = 50;
pub const DLOG_TABLE_PE_LIM: u32 = 50;
pub const DLOG_TABLE_N_LIM: u32 = 50;
pub const DLOG_BSGS_LIM: u32 = 500;
pub const DLOG_LOOP_MAX_FACTOR: u32 = 6;
pub const DLOG_G_SMALL: u32 = 0;
pub const DLOG_G_BIG: u32 = 1;
pub const DLOG_MODPE: c_uint = 0;
pub const DLOG_CRT: c_uint = 1;
pub const DLOG_POWER: c_uint = 2;
pub const DLOG_BSGS: c_uint = 3;
pub const DLOG_TABLE: c_uint = 4;
pub const DLOG_23: c_uint = 5;
pub type dlog_precomp_ptr = *mut dlog_precomp_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dlog_1modpe_struct {
    pub inv1p: mp_limb_t,
    pub invloga1: mp_limb_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of dlog_1modpe_struct"][::std::mem::size_of::<dlog_1modpe_struct>() - 16usize];
    ["Alignment of dlog_1modpe_struct"][::std::mem::align_of::<dlog_1modpe_struct>() - 8usize];
    ["Offset of field: dlog_1modpe_struct::inv1p"]
        [::std::mem::offset_of!(dlog_1modpe_struct, inv1p) - 0usize];
    ["Offset of field: dlog_1modpe_struct::invloga1"]
        [::std::mem::offset_of!(dlog_1modpe_struct, invloga1) - 8usize];
};
impl Default for dlog_1modpe_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type dlog_1modpe_t = [dlog_1modpe_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dlog_modpe_struct {
    pub p: mp_limb_t,
    pub e: mp_limb_t,
    pub pe1: mp_limb_t,
    pub inva: mp_limb_t,
    pub pe: nmod_t,
    pub modp: *mut dlog_precomp_struct,
    pub modpe: dlog_1modpe_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of dlog_modpe_struct"][::std::mem::size_of::<dlog_modpe_struct>() - 80usize];
    ["Alignment of dlog_modpe_struct"][::std::mem::align_of::<dlog_modpe_struct>() - 8usize];
    ["Offset of field: dlog_modpe_struct::p"]
        [::std::mem::offset_of!(dlog_modpe_struct, p) - 0usize];
    ["Offset of field: dlog_modpe_struct::e"]
        [::std::mem::offset_of!(dlog_modpe_struct, e) - 8usize];
    ["Offset of field: dlog_modpe_struct::pe1"]
        [::std::mem::offset_of!(dlog_modpe_struct, pe1) - 16usize];
    ["Offset of field: dlog_modpe_struct::inva"]
        [::std::mem::offset_of!(dlog_modpe_struct, inva) - 24usize];
    ["Offset of field: dlog_modpe_struct::pe"]
        [::std::mem::offset_of!(dlog_modpe_struct, pe) - 32usize];
    ["Offset of field: dlog_modpe_struct::modp"]
        [::std::mem::offset_of!(dlog_modpe_struct, modp) - 56usize];
    ["Offset of field: dlog_modpe_struct::modpe"]
        [::std::mem::offset_of!(dlog_modpe_struct, modpe) - 64usize];
};
impl Default for dlog_modpe_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type dlog_modpe_t = [dlog_modpe_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dlog_table_struct {
    pub mod_: mp_limb_t,
    pub table: *mut mp_limb_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of dlog_table_struct"][::std::mem::size_of::<dlog_table_struct>() - 16usize];
    ["Alignment of dlog_table_struct"][::std::mem::align_of::<dlog_table_struct>() - 8usize];
    ["Offset of field: dlog_table_struct::mod_"]
        [::std::mem::offset_of!(dlog_table_struct, mod_) - 0usize];
    ["Offset of field: dlog_table_struct::table"]
        [::std::mem::offset_of!(dlog_table_struct, table) - 8usize];
};
impl Default for dlog_table_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type dlog_table_t = [dlog_table_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct apow {
    pub k: mp_limb_t,
    pub ak: mp_limb_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of apow"][::std::mem::size_of::<apow>() - 16usize];
    ["Alignment of apow"][::std::mem::align_of::<apow>() - 8usize];
    ["Offset of field: apow::k"][::std::mem::offset_of!(apow, k) - 0usize];
    ["Offset of field: apow::ak"][::std::mem::offset_of!(apow, ak) - 8usize];
};
impl Default for apow {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type apow_t = apow;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dlog_bsgs_struct {
    pub mod_: nmod_t,
    pub m: mp_limb_t,
    pub am: mp_limb_t,
    pub g: mp_limb_t,
    pub table: *mut apow_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of dlog_bsgs_struct"][::std::mem::size_of::<dlog_bsgs_struct>() - 56usize];
    ["Alignment of dlog_bsgs_struct"][::std::mem::align_of::<dlog_bsgs_struct>() - 8usize];
    ["Offset of field: dlog_bsgs_struct::mod_"]
        [::std::mem::offset_of!(dlog_bsgs_struct, mod_) - 0usize];
    ["Offset of field: dlog_bsgs_struct::m"][::std::mem::offset_of!(dlog_bsgs_struct, m) - 24usize];
    ["Offset of field: dlog_bsgs_struct::am"]
        [::std::mem::offset_of!(dlog_bsgs_struct, am) - 32usize];
    ["Offset of field: dlog_bsgs_struct::g"][::std::mem::offset_of!(dlog_bsgs_struct, g) - 40usize];
    ["Offset of field: dlog_bsgs_struct::table"]
        [::std::mem::offset_of!(dlog_bsgs_struct, table) - 48usize];
};
impl Default for dlog_bsgs_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type dlog_bsgs_t = [dlog_bsgs_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dlog_rho_struct {
    pub a: mp_limb_t,
    pub n: nmod_t,
    pub mod_: nmod_t,
    pub nisprime: libc::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of dlog_rho_struct"][::std::mem::size_of::<dlog_rho_struct>() - 64usize];
    ["Alignment of dlog_rho_struct"][::std::mem::align_of::<dlog_rho_struct>() - 8usize];
    ["Offset of field: dlog_rho_struct::a"][::std::mem::offset_of!(dlog_rho_struct, a) - 0usize];
    ["Offset of field: dlog_rho_struct::n"][::std::mem::offset_of!(dlog_rho_struct, n) - 8usize];
    ["Offset of field: dlog_rho_struct::mod_"]
        [::std::mem::offset_of!(dlog_rho_struct, mod_) - 32usize];
    ["Offset of field: dlog_rho_struct::nisprime"]
        [::std::mem::offset_of!(dlog_rho_struct, nisprime) - 56usize];
};
impl Default for dlog_rho_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type dlog_rho_t = [dlog_rho_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dlog_crt_struct {
    pub mod_: nmod_t,
    pub n: nmod_t,
    pub num: mp_limb_t,
    pub expo: *mut mp_limb_t,
    pub crt_coeffs: *mut mp_limb_t,
    pub pre: dlog_precomp_ptr,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of dlog_crt_struct"][::std::mem::size_of::<dlog_crt_struct>() - 80usize];
    ["Alignment of dlog_crt_struct"][::std::mem::align_of::<dlog_crt_struct>() - 8usize];
    ["Offset of field: dlog_crt_struct::mod_"]
        [::std::mem::offset_of!(dlog_crt_struct, mod_) - 0usize];
    ["Offset of field: dlog_crt_struct::n"][::std::mem::offset_of!(dlog_crt_struct, n) - 24usize];
    ["Offset of field: dlog_crt_struct::num"]
        [::std::mem::offset_of!(dlog_crt_struct, num) - 48usize];
    ["Offset of field: dlog_crt_struct::expo"]
        [::std::mem::offset_of!(dlog_crt_struct, expo) - 56usize];
    ["Offset of field: dlog_crt_struct::crt_coeffs"]
        [::std::mem::offset_of!(dlog_crt_struct, crt_coeffs) - 64usize];
    ["Offset of field: dlog_crt_struct::pre"]
        [::std::mem::offset_of!(dlog_crt_struct, pre) - 72usize];
};
impl Default for dlog_crt_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type dlog_crt_t = [dlog_crt_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dlog_power_struct {
    pub mod_: nmod_t,
    pub p: mp_limb_t,
    pub e: mp_limb_t,
    pub apk: *mut mp_limb_t,
    pub pre: *mut dlog_precomp_struct,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of dlog_power_struct"][::std::mem::size_of::<dlog_power_struct>() - 56usize];
    ["Alignment of dlog_power_struct"][::std::mem::align_of::<dlog_power_struct>() - 8usize];
    ["Offset of field: dlog_power_struct::mod_"]
        [::std::mem::offset_of!(dlog_power_struct, mod_) - 0usize];
    ["Offset of field: dlog_power_struct::p"]
        [::std::mem::offset_of!(dlog_power_struct, p) - 24usize];
    ["Offset of field: dlog_power_struct::e"]
        [::std::mem::offset_of!(dlog_power_struct, e) - 32usize];
    ["Offset of field: dlog_power_struct::apk"]
        [::std::mem::offset_of!(dlog_power_struct, apk) - 40usize];
    ["Offset of field: dlog_power_struct::pre"]
        [::std::mem::offset_of!(dlog_power_struct, pre) - 48usize];
};
impl Default for dlog_power_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type dlog_power_t = [dlog_power_struct; 1usize];
pub type dlog_order23_t = [mp_limb_t; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dlog_precomp_struct {
    pub type_: libc::c_int,
    pub cost: mp_limb_t,
    pub t: dlog_precomp_struct__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dlog_precomp_struct__bindgen_ty_1 {
    pub table: __BindgenUnionField<dlog_table_t>,
    pub bsgs: __BindgenUnionField<dlog_bsgs_t>,
    pub crt: __BindgenUnionField<dlog_crt_t>,
    pub power: __BindgenUnionField<dlog_power_t>,
    pub modpe: __BindgenUnionField<dlog_modpe_t>,
    pub order23: __BindgenUnionField<dlog_order23_t>,
    pub bindgen_union_field: [u64; 10usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of dlog_precomp_struct__bindgen_ty_1"]
        [::std::mem::size_of::<dlog_precomp_struct__bindgen_ty_1>() - 80usize];
    ["Alignment of dlog_precomp_struct__bindgen_ty_1"]
        [::std::mem::align_of::<dlog_precomp_struct__bindgen_ty_1>() - 8usize];
    ["Offset of field: dlog_precomp_struct__bindgen_ty_1::table"]
        [::std::mem::offset_of!(dlog_precomp_struct__bindgen_ty_1, table) - 0usize];
    ["Offset of field: dlog_precomp_struct__bindgen_ty_1::bsgs"]
        [::std::mem::offset_of!(dlog_precomp_struct__bindgen_ty_1, bsgs) - 0usize];
    ["Offset of field: dlog_precomp_struct__bindgen_ty_1::crt"]
        [::std::mem::offset_of!(dlog_precomp_struct__bindgen_ty_1, crt) - 0usize];
    ["Offset of field: dlog_precomp_struct__bindgen_ty_1::power"]
        [::std::mem::offset_of!(dlog_precomp_struct__bindgen_ty_1, power) - 0usize];
    ["Offset of field: dlog_precomp_struct__bindgen_ty_1::modpe"]
        [::std::mem::offset_of!(dlog_precomp_struct__bindgen_ty_1, modpe) - 0usize];
    ["Offset of field: dlog_precomp_struct__bindgen_ty_1::order23"]
        [::std::mem::offset_of!(dlog_precomp_struct__bindgen_ty_1, order23) - 0usize];
};
impl Default for dlog_precomp_struct__bindgen_ty_1 {
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
    ["Size of dlog_precomp_struct"][::std::mem::size_of::<dlog_precomp_struct>() - 96usize];
    ["Alignment of dlog_precomp_struct"][::std::mem::align_of::<dlog_precomp_struct>() - 8usize];
    ["Offset of field: dlog_precomp_struct::type_"]
        [::std::mem::offset_of!(dlog_precomp_struct, type_) - 0usize];
    ["Offset of field: dlog_precomp_struct::cost"]
        [::std::mem::offset_of!(dlog_precomp_struct, cost) - 8usize];
    ["Offset of field: dlog_precomp_struct::t"]
        [::std::mem::offset_of!(dlog_precomp_struct, t) - 16usize];
};
impl Default for dlog_precomp_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type dlog_precomp_t = [dlog_precomp_struct; 1usize];
extern "C" {
    pub fn apow_cmp(x: *const apow_t, y: *const apow_t) -> libc::c_int;
    pub fn dlog_precomp_modpe_init(
        pre: *mut dlog_precomp_struct,
        a: mp_limb_t,
        p: mp_limb_t,
        e: mp_limb_t,
        pe: mp_limb_t,
        num: mp_limb_t,
    );
    pub fn dlog_precomp_small_init(
        pre: *mut dlog_precomp_struct,
        a: mp_limb_t,
        mod_: mp_limb_t,
        n: mp_limb_t,
        num: mp_limb_t,
    );
    pub fn dlog_precomp_n_init(
        pre: *mut dlog_precomp_struct,
        a: mp_limb_t,
        mod_: mp_limb_t,
        n: mp_limb_t,
        num: mp_limb_t,
    );
    pub fn dlog_precomp_p_init(
        pre: *mut dlog_precomp_struct,
        a: mp_limb_t,
        mod_: mp_limb_t,
        p: mp_limb_t,
        num: mp_limb_t,
    );
    pub fn dlog_precomp_pe_init(
        pre: *mut dlog_precomp_struct,
        a: mp_limb_t,
        mod_: mp_limb_t,
        p: mp_limb_t,
        e: mp_limb_t,
        pe: mp_limb_t,
        num: mp_limb_t,
    );
    pub fn dlog_precomp_clear(pre: *mut dlog_precomp_struct);
    pub fn dlog_precomp(pre: *const dlog_precomp_struct, b: mp_limb_t) -> mp_limb_t;
    pub fn dlog_order23_init(t: *mut mp_limb_t, a: mp_limb_t) -> mp_limb_t;
    pub fn dlog_table_init(t: *mut dlog_table_struct, a: mp_limb_t, mod_: mp_limb_t) -> mp_limb_t;
    pub fn dlog_crt_init(
        t: *mut dlog_crt_struct,
        a: mp_limb_t,
        mod_: mp_limb_t,
        n: mp_limb_t,
        num: mp_limb_t,
    ) -> mp_limb_t;
    pub fn dlog_power_init(
        t: *mut dlog_power_struct,
        a: mp_limb_t,
        mod_: mp_limb_t,
        p: mp_limb_t,
        e: mp_limb_t,
        num: mp_limb_t,
    ) -> mp_limb_t;
    pub fn dlog_modpe_init(
        t: *mut dlog_modpe_struct,
        a: mp_limb_t,
        p: mp_limb_t,
        e: mp_limb_t,
        pe: mp_limb_t,
        num: mp_limb_t,
    ) -> mp_limb_t;
    pub fn dlog_bsgs_init(
        t: *mut dlog_bsgs_struct,
        a: mp_limb_t,
        mod_: mp_limb_t,
        n: mp_limb_t,
        m: mp_limb_t,
    ) -> mp_limb_t;
    pub fn dlog_1modpe_init(
        t: *mut dlog_1modpe_struct,
        a1: mp_limb_t,
        p: mp_limb_t,
        e: mp_limb_t,
        pe: nmod_t,
    );
    pub fn dlog_rho_init(t: *mut dlog_rho_struct, a: mp_limb_t, mod_: mp_limb_t, n: mp_limb_t);
    pub fn dlog_once(b: mp_limb_t, a: mp_limb_t, mod_: nmod_t, n: mp_limb_t) -> mp_limb_t;
    #[link_name = "dlog_order23_clear__extern"]
    pub fn dlog_order23_clear(t: *mut mp_limb_t);
    #[link_name = "dlog_table_clear__extern"]
    pub fn dlog_table_clear(t: *mut dlog_table_struct);
    pub fn dlog_crt_clear(t: *mut dlog_crt_struct);
    #[link_name = "dlog_power_clear__extern"]
    pub fn dlog_power_clear(t: *mut dlog_power_struct);
    #[link_name = "dlog_modpe_clear__extern"]
    pub fn dlog_modpe_clear(t: *mut dlog_modpe_struct);
    #[link_name = "dlog_bsgs_clear__extern"]
    pub fn dlog_bsgs_clear(t: *mut dlog_bsgs_struct);
    #[link_name = "dlog_rho_clear__extern"]
    pub fn dlog_rho_clear(t: *mut dlog_rho_struct);
    #[link_name = "dlog_bsgs_size__extern"]
    pub fn dlog_bsgs_size(n: mp_limb_t, num: mp_limb_t) -> mp_limb_t;
    pub fn dlog_order23(t: *const mp_limb_t, b: mp_limb_t) -> mp_limb_t;
    pub fn dlog_table(t: *const dlog_table_struct, b: mp_limb_t) -> mp_limb_t;
    pub fn dlog_crt(t: *const dlog_crt_struct, b: mp_limb_t) -> mp_limb_t;
    pub fn dlog_power(t: *const dlog_power_struct, b: mp_limb_t) -> mp_limb_t;
    pub fn dlog_modpe(t: *const dlog_modpe_struct, b: mp_limb_t) -> mp_limb_t;
    pub fn dlog_bsgs(t: *const dlog_bsgs_struct, b: mp_limb_t) -> mp_limb_t;
    pub fn dlog_rho(t: *const dlog_rho_struct, b: mp_limb_t) -> mp_limb_t;
    pub fn dlog_1modpe_1modp(
        b1: mp_limb_t,
        p: mp_limb_t,
        e: mp_limb_t,
        inv1p: mp_limb_t,
        pe: nmod_t,
    ) -> mp_limb_t;
    pub fn dlog_1modpe(
        t: *const dlog_1modpe_struct,
        b1: mp_limb_t,
        p: mp_limb_t,
        e: mp_limb_t,
        pe: nmod_t,
    ) -> mp_limb_t;
    pub fn dlog_mod2e_1mod4(b1: mp_limb_t, e: mp_limb_t, inva: mp_limb_t, pe: nmod_t) -> mp_limb_t;
    pub fn dlog_mod2e(t: *const dlog_modpe_struct, b: mp_limb_t) -> mp_limb_t;
    pub fn dlog_n_factor_group(fac: *mut n_factor_t, bound: mp_limb_t);
    pub fn dlog_vec_pindex_factorgcd(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        p: mp_limb_t,
        mod_: nmod_t,
        a: mp_limb_t,
        na: mp_limb_t,
        loga: mp_limb_t,
        logm1: mp_limb_t,
        order: nmod_t,
        maxtry: libc::c_int,
    ) -> mp_limb_t;
    pub fn dlog_vec_fill(v: *mut mp_limb_t, nv: mp_limb_t, x: mp_limb_t);
    pub fn dlog_vec_set_not_found(v: *mut mp_limb_t, nv: mp_limb_t, mod_: nmod_t);
    pub fn dlog_vec_loop(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec_loop_add(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec_eratos_add(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec_eratos(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec_sieve_add(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec_sieve(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec_add(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec_sieve_precomp(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        pre: *mut dlog_precomp_struct,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec_sieve_add_precomp(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        pre: *mut dlog_precomp_struct,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec_add_precomp(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        pre: *mut dlog_precomp_struct,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
}
