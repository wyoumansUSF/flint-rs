/* automatically generated by rust-bindgen 0.70.1 */

#![allow(non_camel_case_types)]
use crate::deps::*;
use libc::{c_char, c_int, c_uint, c_void, size_t, FILE};


pub type acb_calc_func_t = ::std::option::Option<
    unsafe extern "C" fn(
        out: acb_ptr,
        inp: *const acb_struct,
        param: *mut ::std::os::raw::c_void,
        order: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
pub struct acb_calc_integrate_opt_struct {
    pub deg_limit: mp_limb_signed_t,
    pub eval_limit: mp_limb_signed_t,
    pub depth_limit: mp_limb_signed_t,
    pub use_heap: ::std::os::raw::c_int,
    pub verbose: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of acb_calc_integrate_opt_struct"]
        [::std::mem::size_of::<acb_calc_integrate_opt_struct>() - 32usize];
    ["Alignment of acb_calc_integrate_opt_struct"]
        [::std::mem::align_of::<acb_calc_integrate_opt_struct>() - 8usize];
    ["Offset of field: acb_calc_integrate_opt_struct::deg_limit"]
        [::std::mem::offset_of!(acb_calc_integrate_opt_struct, deg_limit) - 0usize];
    ["Offset of field: acb_calc_integrate_opt_struct::eval_limit"]
        [::std::mem::offset_of!(acb_calc_integrate_opt_struct, eval_limit) - 8usize];
    ["Offset of field: acb_calc_integrate_opt_struct::depth_limit"]
        [::std::mem::offset_of!(acb_calc_integrate_opt_struct, depth_limit) - 16usize];
    ["Offset of field: acb_calc_integrate_opt_struct::use_heap"]
        [::std::mem::offset_of!(acb_calc_integrate_opt_struct, use_heap) - 24usize];
    ["Offset of field: acb_calc_integrate_opt_struct::verbose"]
        [::std::mem::offset_of!(acb_calc_integrate_opt_struct, verbose) - 28usize];
};
impl Default for acb_calc_integrate_opt_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type acb_calc_integrate_opt_t = [acb_calc_integrate_opt_struct; 1usize];
extern "C" {
    pub fn acb_calc_cauchy_bound(
        bound: *mut arb_struct,
        func: acb_calc_func_t,
        param: *mut ::std::os::raw::c_void,
        x: *const acb_struct,
        radius: *const arb_struct,
        maxdepth: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_calc_integrate_taylor(
        res: *mut acb_struct,
        func: acb_calc_func_t,
        param: *mut ::std::os::raw::c_void,
        a: *const acb_struct,
        b: *const acb_struct,
        inner_radius: *const arf_struct,
        outer_radius: *const arf_struct,
        accuracy_goal: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn acb_calc_integrate_opt_init(options: *mut acb_calc_integrate_opt_struct);
    pub fn acb_calc_integrate(
        res: *mut acb_struct,
        f: acb_calc_func_t,
        param: *mut ::std::os::raw::c_void,
        a: *const acb_struct,
        b: *const acb_struct,
        goal: mp_limb_signed_t,
        tol: *const mag_struct,
        options: *const acb_calc_integrate_opt_struct,
        prec: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn acb_calc_integrate_gl_auto_deg(
        res: *mut acb_struct,
        eval_count: *mut mp_limb_signed_t,
        f: acb_calc_func_t,
        param: *mut ::std::os::raw::c_void,
        a: *const acb_struct,
        b: *const acb_struct,
        tol: *const mag_struct,
        deg_limit: mp_limb_signed_t,
        verbose: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
}
