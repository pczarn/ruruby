use array::Array;
use raw;
use value::Value;

trait ToRuby {
    fn to_ruby(&self) -> Option<Value>;

    fn to_ary(&self) -> Option<Array> {
        // self.to_ruby().map(|val| raw::rb_ary_to_ary(val))
        unimplemented!()
    }

    fn to_fixnum(&self) -> Value {
        unimplemented!()
    }
}

impl ToRuby for u64 {
    fn to_ruby(&self) -> Option<Value> {
        unimplemented!()
    }
}
