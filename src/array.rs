use raw;
use value::Value;
use libc::c_long;

pub struct Array {
    ary: Value
}

impl Array {
    pub fn new() -> Array {
        Array {
            ary: unsafe { raw::rb_ary_new() }
        }
    }

    pub fn with_capacity(capacity: uint) -> Array {
        Array {
            ary: unsafe { raw::rb_ary_new_capa(capacity as c_long) }
        }
    }

    pub fn push(&mut self, v: Value) {
        unsafe {
            raw::rb_ary_push(self.ary, v);
        }
    }

    pub fn shift(&mut self) -> Value {
        unsafe {
            raw::rb_ary_shift(self.ary)
        }
    }
}

impl Clone for Array {
    fn clone(&self) -> Array {
        Array { ary: unsafe { raw::rb_ary_dup(self.ary) } }
    }
}

#[test]
fn test_push_shift() {
    unsafe { raw::ruby_init() }
    let mut a = Array::new();
    a.push(FromPrimitive::from_u64(123).unwrap());
    assert_eq!(a.shift().to_int(), Some(123))
}
