use libc::{
    intptr_t,
    uintptr_t,
    c_long,
    c_longlong,
    c_ulonglong
};

use value::{Value, SignedValue};

#[link(name = "ruby")]
extern {
    pub fn ruby_init();

    pub fn rb_uint2inum(n: Value) -> Value; // rb_uint_new(v)
    pub fn rb_int2inum(n: SignedValue) -> Value; // rb_int_new(v)
    pub fn rb_ll2inum(n: c_longlong) -> Value;
    pub fn rb_ull2inum(n: c_ulonglong) -> Value;

    // array.c
    pub fn rb_mem_clear(mem: *mut Value, size: c_long);
    pub fn rb_assoc_new(a: Value, b: Value) -> Value;
    pub fn rb_check_array_type(ary: Value) -> Value;
    pub fn rb_ary_new() -> Value;
    pub fn rb_ary_new_capa(capa: uintptr_t) -> Value;
    pub fn rb_ary_new_from_args(n: uintptr_t, ...) -> Value;
    pub fn rb_ary_new_from_Values(n: uintptr_t, elts: *const Value) -> Value;
    pub fn rb_ary_tmp_new(size: uintptr_t) -> Value;
    pub fn rb_ary_free(ary: Value);
    pub fn rb_ary_modify(ary: Value);
    pub fn rb_ary_freeze(ary: Value) -> Value;
    pub fn rb_ary_shared_with_p(a: Value, b: Value) -> Value;
    pub fn rb_ary_aref(a: intptr_t, b: *const Value, c: Value) -> Value;
    pub fn rb_ary_subseq(a: Value, b: c_long, c: c_long) -> Value;
    pub fn rb_ary_store(a: Value, i: c_long, v: Value);
    pub fn rb_ary_dup(ary: Value) -> Value;
    pub fn rb_ary_resurrect(ary: Value) -> Value;
    pub fn rb_ary_to_ary(ary: Value) -> Value;
    pub fn rb_ary_to_s(ary: Value) -> Value;
    pub fn rb_ary_cat(ary: Value, /*const*/v: *const Value, x: c_long) -> Value;
    pub fn rb_ary_push(ary: Value, v: Value) -> Value;
    pub fn rb_ary_pop(ary: Value) -> Value;
    pub fn rb_ary_shift(ary: Value) -> Value;
    pub fn rb_ary_unshift(ary: Value, v: Value) -> Value;
    pub fn rb_ary_entry(ary: Value, x: c_long) -> Value;
    pub fn rb_ary_each(ary: Value) -> Value;
    pub fn rb_ary_join(a: Value, b: Value) -> Value;
    pub fn rb_ary_reverse(ary: Value) -> Value;
    pub fn rb_ary_rotate(ary: Value, x: c_long) -> Value;
    pub fn rb_ary_sort(ary: Value) -> Value;
    pub fn rb_ary_sort_bang(ary: Value) -> Value;
    pub fn rb_ary_delete(ary: Value, v: Value) -> Value;
    pub fn rb_ary_delete_at(ary: Value, i: c_long) -> Value;
    pub fn rb_ary_clear(ary: Value) -> Value;
    pub fn rb_ary_plus(a: Value, b: Value) -> Value;
    pub fn rb_ary_concat(a: Value, b: Value) -> Value;
    pub fn rb_ary_assoc(a: Value, b: Value) -> Value;
    pub fn rb_ary_rassoc(a: Value, b: Value) -> Value;
    pub fn rb_ary_includes(ary: Value, v: Value) -> Value;
    pub fn rb_ary_cmp(a: Value, b: Value) -> Value;
    pub fn rb_ary_replace(cpy: Value, orig: Value) -> Value;
    pub fn rb_get_values_at(ary: Value, x: uintptr_t, y: intptr_t, p: *mut Value, f: fn(Value, uintptr_t) -> Value) -> Value;
    pub fn rb_ary_resize(ary: Value, len: c_long) -> Value;

    pub fn rb_define_method(a: Value, name: *const i8, f: fn(intptr_t) -> Value, x: intptr_t) -> Value;
    pub fn rb_const_get(a: Value, id: uintptr_t) -> Value;
    pub fn rb_intern(s: *const i8) -> uintptr_t;

    pub static rb_cObject: Value;
}
