#![crate_name = "ruruby"]

extern crate libc;

pub mod raw;

pub mod array;
pub mod to_ruby;
pub mod value;

pub fn intern(s: &str) -> uint {
    unsafe {
        s.with_c_str(|s| raw::rb_intern(s))
    }
}

pub fn const_get(a: value::Value, id: uint) -> value::Value {
    unsafe {
        raw::rb_const_get(a, id)
    }
}

pub fn define_method(a: value::Value,
                     name: &str,
                     f: fn(int) -> value::Value,
                     arity: int)
                     -> value::Value {
    unsafe {
        name.with_c_str(|name| raw::rb_define_method(a, name, f, arity))
    }
}
