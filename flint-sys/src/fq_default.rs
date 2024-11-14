/* automatically generated by rust-bindgen 0.70.1 */

use libc::*;
use crate::deps::*;
use crate::bindgen::*;
use crate::flint::*;
use crate::fmpz_mod_types::*;
use crate::fmpz_types::*;
use crate::fq_nmod_types::*;
use crate::fq_types::*;
use crate::fq_zech_types::*;
use crate::gr::*;
use crate::nmod_types::*;


pub const FQ_DEFAULT_FQ_ZECH: u32 = 1;
pub const FQ_DEFAULT_FQ_NMOD: u32 = 2;
pub const FQ_DEFAULT_FQ: u32 = 3;
pub const FQ_DEFAULT_NMOD: u32 = 4;
pub const FQ_DEFAULT_FMPZ_MOD: u32 = 5;
#[repr(C)]
pub struct fq_default_struct {
    pub fq: __BindgenUnionField<fq_t>,
    pub fq_nmod: __BindgenUnionField<fq_nmod_t>,
    pub fq_zech: __BindgenUnionField<fq_zech_t>,
    pub nmod: __BindgenUnionField<mp_limb_t>,
    pub fmpz_mod: __BindgenUnionField<fmpz_t>,
    pub bindgen_union_field: [u64; 6usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_default_struct"][::std::mem::size_of::<fq_default_struct>() - 48usize];
    ["Alignment of fq_default_struct"][::std::mem::align_of::<fq_default_struct>() - 8usize];
    ["Offset of field: fq_default_struct::fq"]
        [::std::mem::offset_of!(fq_default_struct, fq) - 0usize];
    ["Offset of field: fq_default_struct::fq_nmod"]
        [::std::mem::offset_of!(fq_default_struct, fq_nmod) - 0usize];
    ["Offset of field: fq_default_struct::fq_zech"]
        [::std::mem::offset_of!(fq_default_struct, fq_zech) - 0usize];
    ["Offset of field: fq_default_struct::nmod"]
        [::std::mem::offset_of!(fq_default_struct, nmod) - 0usize];
    ["Offset of field: fq_default_struct::fmpz_mod"]
        [::std::mem::offset_of!(fq_default_struct, fmpz_mod) - 0usize];
};
impl Default for fq_default_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_default_t = [fq_default_struct; 1usize];
pub type fq_default_ctx_struct = gr_ctx_struct;
pub type fq_default_ctx_t = [fq_default_ctx_struct; 1usize];
#[repr(C)]
pub struct _gr_fmpz_mod_ctx_struct {
    pub ctx: *mut fmpz_mod_ctx_struct,
    pub is_prime: truth_t,
    pub a: fmpz,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _gr_fmpz_mod_ctx_struct"][::std::mem::size_of::<_gr_fmpz_mod_ctx_struct>() - 24usize];
    ["Alignment of _gr_fmpz_mod_ctx_struct"]
        [::std::mem::align_of::<_gr_fmpz_mod_ctx_struct>() - 8usize];
    ["Offset of field: _gr_fmpz_mod_ctx_struct::ctx"]
        [::std::mem::offset_of!(_gr_fmpz_mod_ctx_struct, ctx) - 0usize];
    ["Offset of field: _gr_fmpz_mod_ctx_struct::is_prime"]
        [::std::mem::offset_of!(_gr_fmpz_mod_ctx_struct, is_prime) - 8usize];
    ["Offset of field: _gr_fmpz_mod_ctx_struct::a"]
        [::std::mem::offset_of!(_gr_fmpz_mod_ctx_struct, a) - 16usize];
};
impl Default for _gr_fmpz_mod_ctx_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct _gr_nmod_ctx_struct {
    pub nmod: nmod_t,
    pub a: mp_limb_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _gr_nmod_ctx_struct"][::std::mem::size_of::<_gr_nmod_ctx_struct>() - 32usize];
    ["Alignment of _gr_nmod_ctx_struct"][::std::mem::align_of::<_gr_nmod_ctx_struct>() - 8usize];
    ["Offset of field: _gr_nmod_ctx_struct::nmod"]
        [::std::mem::offset_of!(_gr_nmod_ctx_struct, nmod) - 0usize];
    ["Offset of field: _gr_nmod_ctx_struct::a"]
        [::std::mem::offset_of!(_gr_nmod_ctx_struct, a) - 24usize];
};
impl Default for _gr_nmod_ctx_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub fn fq_default_ctx_init_type(
        ctx: *mut fq_default_ctx_struct,
        p: *const fmpz,
        d: mp_limb_signed_t,
        var: *const libc::c_char,
        type_: libc::c_int,
    );
    #[link_name = "fq_default_ctx_init__extern"]
    pub fn fq_default_ctx_init(
        ctx: *mut fq_default_ctx_struct,
        p: *const fmpz,
        d: mp_limb_signed_t,
        var: *const libc::c_char,
    );
    #[link_name = "fq_default_ctx_inner__extern"]
    pub fn fq_default_ctx_inner(ctx: *const fq_default_ctx_struct) -> *mut libc::c_void;
    pub fn fq_default_ctx_init_modulus_type(
        ctx: *mut fq_default_ctx_struct,
        modulus: *const fmpz_mod_poly_struct,
        mod_ctx: *mut fmpz_mod_ctx_struct,
        var: *const libc::c_char,
        type_: libc::c_int,
    );
    pub fn fq_default_ctx_init_modulus(
        ctx: *mut fq_default_ctx_struct,
        modulus: *const fmpz_mod_poly_struct,
        mod_ctx: *mut fmpz_mod_ctx_struct,
        var: *const libc::c_char,
    );
    pub fn fq_default_ctx_init_modulus_nmod_type(
        ctx: *mut fq_default_ctx_struct,
        modulus: *const nmod_poly_struct,
        var: *const libc::c_char,
        type_: libc::c_int,
    );
    pub fn fq_default_ctx_init_modulus_nmod(
        ctx: *mut fq_default_ctx_struct,
        modulus: *const nmod_poly_struct,
        var: *const libc::c_char,
    );
    #[link_name = "fq_default_ctx_clear__extern"]
    pub fn fq_default_ctx_clear(ctx: *mut fq_default_ctx_struct);
    #[link_name = "fq_default_ctx_type__extern"]
    pub fn fq_default_ctx_type(ctx: *const fq_default_ctx_struct) -> libc::c_int;
    #[link_name = "fq_default_ctx_degree__extern"]
    pub fn fq_default_ctx_degree(ctx: *const fq_default_ctx_struct) -> mp_limb_signed_t;
    #[link_name = "fq_default_ctx_prime__extern"]
    pub fn fq_default_ctx_prime(prime: *mut fmpz, ctx: *const fq_default_ctx_struct);
    pub fn fq_default_ctx_modulus(p: *mut fmpz_mod_poly_struct, ctx: *const fq_default_ctx_struct);
    #[link_name = "fq_default_ctx_order__extern"]
    pub fn fq_default_ctx_order(f: *mut fmpz, ctx: *const fq_default_ctx_struct);
    pub fn fq_default_ctx_fprint(file: *mut FILE, ctx: *const fq_default_ctx_struct)
        -> libc::c_int;
    pub fn fq_default_ctx_print(ctx: *const fq_default_ctx_struct);
    #[link_name = "fq_default_init__extern"]
    pub fn fq_default_init(rop: *mut fq_default_struct, ctx: *const fq_default_ctx_struct);
    #[link_name = "fq_default_init2__extern"]
    pub fn fq_default_init2(rop: *mut fq_default_struct, ctx: *const fq_default_ctx_struct);
    #[link_name = "fq_default_clear__extern"]
    pub fn fq_default_clear(rop: *mut fq_default_struct, ctx: *const fq_default_ctx_struct);
    #[link_name = "fq_default_add__extern"]
    pub fn fq_default_add(
        rop: *mut fq_default_struct,
        op1: *const fq_default_struct,
        op2: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_sub__extern"]
    pub fn fq_default_sub(
        rop: *mut fq_default_struct,
        op1: *const fq_default_struct,
        op2: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_sub_one__extern"]
    pub fn fq_default_sub_one(
        rop: *mut fq_default_struct,
        op1: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_neg__extern"]
    pub fn fq_default_neg(
        rop: *mut fq_default_struct,
        op1: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_mul__extern"]
    pub fn fq_default_mul(
        rop: *mut fq_default_struct,
        op1: *const fq_default_struct,
        op2: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_mul_fmpz__extern"]
    pub fn fq_default_mul_fmpz(
        rop: *mut fq_default_struct,
        op: *const fq_default_struct,
        x: *const fmpz,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_mul_si__extern"]
    pub fn fq_default_mul_si(
        rop: *mut fq_default_struct,
        op: *const fq_default_struct,
        x: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_mul_ui__extern"]
    pub fn fq_default_mul_ui(
        rop: *mut fq_default_struct,
        op: *const fq_default_struct,
        x: mp_limb_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_sqr__extern"]
    pub fn fq_default_sqr(
        rop: *mut fq_default_struct,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_is_invertible__extern"]
    pub fn fq_default_is_invertible(
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_default_inv__extern"]
    pub fn fq_default_inv(
        rop: *mut fq_default_struct,
        op1: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_div__extern"]
    pub fn fq_default_div(
        rop: *mut fq_default_struct,
        op1: *mut fq_default_struct,
        op2: *mut fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_pow__extern"]
    pub fn fq_default_pow(
        rop: *mut fq_default_struct,
        op1: *const fq_default_struct,
        e: *const fmpz,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_pow_ui__extern"]
    pub fn fq_default_pow_ui(
        rop: *mut fq_default_struct,
        op: *const fq_default_struct,
        e: mp_limb_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_is_square__extern"]
    pub fn fq_default_is_square(
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_default_sqrt__extern"]
    pub fn fq_default_sqrt(
        rop: *mut fq_default_struct,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_default_pth_root__extern"]
    pub fn fq_default_pth_root(
        rop: *mut fq_default_struct,
        op1: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_randtest__extern"]
    pub fn fq_default_randtest(
        rop: *mut fq_default_struct,
        state: *mut flint_rand_s,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_randtest_not_zero__extern"]
    pub fn fq_default_randtest_not_zero(
        rop: *mut fq_default_struct,
        state: *mut flint_rand_s,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_rand__extern"]
    pub fn fq_default_rand(
        rop: *mut fq_default_struct,
        state: *mut flint_rand_s,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_rand_not_zero__extern"]
    pub fn fq_default_rand_not_zero(
        rop: *mut fq_default_struct,
        state: *mut flint_rand_s,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_equal__extern"]
    pub fn fq_default_equal(
        op1: *const fq_default_struct,
        op2: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_default_is_zero__extern"]
    pub fn fq_default_is_zero(
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_default_is_one__extern"]
    pub fn fq_default_is_one(
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_default_set__extern"]
    pub fn fq_default_set(
        rop: *mut fq_default_struct,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_set_fmpz__extern"]
    pub fn fq_default_set_fmpz(
        rop: *mut fq_default_struct,
        x: *const fmpz,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_set_ui__extern"]
    pub fn fq_default_set_ui(
        rop: *mut fq_default_struct,
        x: mp_limb_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_set_si__extern"]
    pub fn fq_default_set_si(
        rop: *mut fq_default_struct,
        x: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_zero__extern"]
    pub fn fq_default_zero(rop: *mut fq_default_struct, ctx: *const fq_default_ctx_struct);
    #[link_name = "fq_default_one__extern"]
    pub fn fq_default_one(rop: *mut fq_default_struct, ctx: *const fq_default_ctx_struct);
    #[link_name = "fq_default_swap__extern"]
    pub fn fq_default_swap(
        op1: *mut fq_default_struct,
        op2: *mut fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_gen__extern"]
    pub fn fq_default_gen(rop: *mut fq_default_struct, ctx: *const fq_default_ctx_struct);
    #[link_name = "fq_default_get_nmod_poly__extern"]
    pub fn fq_default_get_nmod_poly(
        poly: *mut nmod_poly_struct,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_set_nmod_poly__extern"]
    pub fn fq_default_set_nmod_poly(
        op: *mut fq_default_struct,
        poly: *const nmod_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_get_fmpz__extern"]
    pub fn fq_default_get_fmpz(
        z: *mut fmpz,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_default_get_fmpz_mod_poly(
        poly: *mut fmpz_mod_poly_struct,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_set_fmpz_mod_poly(
        op: *mut fq_default_struct,
        poly: *const fmpz_mod_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_get_fmpz_poly(
        poly: *mut fmpz_poly_struct,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_set_fmpz_poly(
        op: *mut fq_default_struct,
        poly: *const fmpz_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_get_coeff_fmpz__extern"]
    pub fn fq_default_get_coeff_fmpz(
        c: *mut fmpz,
        op: *mut fq_default_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_fprint(
        file: *mut FILE,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_default_fprint_pretty(
        file: *mut FILE,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_default_print(op: *const fq_default_struct, ctx: *const fq_default_ctx_struct);
    pub fn fq_default_print_pretty(op: *const fq_default_struct, ctx: *const fq_default_ctx_struct);
    #[link_name = "fq_default_get_str__extern"]
    pub fn fq_default_get_str(
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> *mut libc::c_char;
    #[link_name = "fq_default_get_str_pretty__extern"]
    pub fn fq_default_get_str_pretty(
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> *mut libc::c_char;
    #[link_name = "fq_default_trace__extern"]
    pub fn fq_default_trace(
        rop: *mut fmpz,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_frobenius__extern"]
    pub fn fq_default_frobenius(
        rop: *mut fq_default_struct,
        op: *const fq_default_struct,
        e: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_norm__extern"]
    pub fn fq_default_norm(
        rop: *mut fmpz,
        op: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
}
