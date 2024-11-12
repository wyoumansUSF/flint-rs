/* automatically generated by rust-bindgen 0.70.1 */

#![allow(non_camel_case_types)]
use crate::deps::*;
use crate::flint::*;
use crate::gr::*;
use libc::{c_char, c_int, c_uint, c_void, size_t, FILE};


pub type gr_method_vec_normalise_op = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut mp_limb_signed_t,
        arg2: gr_srcptr,
        arg3: mp_limb_signed_t,
        arg4: gr_ctx_ptr,
    ) -> ::std::os::raw::c_int,
>;
pub type gr_method_vec_normalise_weak_op = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: gr_srcptr,
        arg2: mp_limb_signed_t,
        arg3: gr_ctx_ptr,
    ) -> mp_limb_signed_t,
>;
pub type gr_method_vec_reduce_op = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: gr_ptr,
        arg2: gr_srcptr,
        arg3: mp_limb_signed_t,
        arg4: gr_ctx_ptr,
    ) -> ::std::os::raw::c_int,
>;
pub type gr_method_vec_dot_op = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: gr_ptr,
        arg2: gr_srcptr,
        arg3: ::std::os::raw::c_int,
        arg4: gr_srcptr,
        arg5: gr_srcptr,
        arg6: mp_limb_signed_t,
        arg7: gr_ctx_ptr,
    ) -> ::std::os::raw::c_int,
>;
pub type gr_method_vec_dot_si_op = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: gr_ptr,
        arg2: gr_srcptr,
        arg3: ::std::os::raw::c_int,
        arg4: gr_srcptr,
        arg5: *const mp_limb_signed_t,
        arg6: mp_limb_signed_t,
        arg7: gr_ctx_ptr,
    ) -> ::std::os::raw::c_int,
>;
pub type gr_method_vec_dot_ui_op = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: gr_ptr,
        arg2: gr_srcptr,
        arg3: ::std::os::raw::c_int,
        arg4: gr_srcptr,
        arg5: *const mp_limb_t,
        arg6: mp_limb_signed_t,
        arg7: gr_ctx_ptr,
    ) -> ::std::os::raw::c_int,
>;
pub type gr_method_vec_dot_fmpz_op = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: gr_ptr,
        arg2: gr_srcptr,
        arg3: ::std::os::raw::c_int,
        arg4: gr_srcptr,
        arg5: *const fmpz,
        arg6: mp_limb_signed_t,
        arg7: gr_ctx_ptr,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn gr_vec_init(vec: *mut gr_vec_struct, len: mp_limb_signed_t, ctx: *mut gr_ctx_struct);
    pub fn gr_vec_clear(vec: *mut gr_vec_struct, ctx: *mut gr_ctx_struct);
    pub fn gr_vec_fit_length(
        vec: *mut gr_vec_struct,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    );
    pub fn gr_vec_set_length(
        vec: *mut gr_vec_struct,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    );
    pub fn gr_vec_set(
        res: *mut gr_vec_struct,
        src: *const gr_vec_struct,
        ctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_vec_append(
        vec: *mut gr_vec_struct,
        f: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _gr_vec_write(
        out: *mut gr_stream_struct,
        vec: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_vec_write(
        out: *mut gr_stream_struct,
        vec: *const gr_vec_struct,
        ctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _gr_vec_print(
        vec: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn gr_vec_print(
        vec: *const gr_vec_struct,
        ctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _gr_vec_randtest(
        res: gr_ptr,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _gr_vec_sum_bsplit_parallel(
        res: gr_ptr,
        vec: gr_srcptr,
        len: mp_limb_signed_t,
        basecase_cutoff: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _gr_vec_sum_bsplit(
        res: gr_ptr,
        vec: gr_srcptr,
        len: mp_limb_signed_t,
        basecase_cutoff: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _gr_vec_sum_parallel(
        res: gr_ptr,
        vec: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _gr_vec_sum_serial(
        res: gr_ptr,
        vec: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _gr_vec_sum_generic(
        res: gr_ptr,
        vec: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _gr_vec_product_bsplit_parallel(
        res: gr_ptr,
        vec: gr_srcptr,
        len: mp_limb_signed_t,
        basecase_cutoff: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _gr_vec_product_bsplit(
        res: gr_ptr,
        vec: gr_srcptr,
        len: mp_limb_signed_t,
        basecase_cutoff: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _gr_vec_product_parallel(
        res: gr_ptr,
        vec: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _gr_vec_product_serial(
        res: gr_ptr,
        vec: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _gr_vec_product_generic(
        res: gr_ptr,
        vec: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
    pub fn _gr_vec_step(
        vec: gr_ptr,
        start: gr_srcptr,
        step: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> ::std::os::raw::c_int;
}
