#[repr(C)]
pub struct Value(uint);

pub type SignedValue = int;

enum SpecialConsts {
    Qfalse = 0,
    Qtrue  = 2,
    Qnil   = 4,
    Qundef = 6,
}

static RUBY_IMMEDIATE_MASK : uint = 0x03;
static RUBY_FIXNUM_FLAG    : uint = 0x01;
static RUBY_FLONUM_MASK    : uint = 0x00; /* any values ANDed with FLONUM_MASK cannot be FLONUM_FLAG */
static RUBY_FLONUM_FLAG    : uint = 0x02;
static RUBY_SYMBOL_FLAG    : uint = 0x0e;
static RUBY_SPECIAL_SHIFT  : uint = 8;

impl FromPrimitive for Value {
    fn from_u64(n: u64) -> Option<Value> {
        Some(Value(n as uint << 1 | RUBY_FIXNUM_FLAG))
    }

    fn from_i64(n: i64) -> Option<Value> {
        Some(Value(n as uint << 1 | RUBY_FIXNUM_FLAG))
    }
}

impl ToPrimitive for Value {
    fn to_u64(&self) -> Option<u64> {
        let &Value(fix) = self;
        if fix & RUBY_FIXNUM_FLAG == RUBY_FIXNUM_FLAG {
            Some((fix >> 1) as u64)
        } else {
            None
        }
    }

    fn to_i64(&self) -> Option<i64> {
        let &Value(fix) = self;
        if fix & RUBY_FIXNUM_FLAG == RUBY_FIXNUM_FLAG {
            Some((fix >> 1) as i64)
        } else {
            None
        }
    }
}
