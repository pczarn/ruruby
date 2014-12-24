require_relative "rust"

def test_sig
   rust = Inline::Rust.new(Module.new {})
   puts rust.parse_signature "fn foo(one: uint, two: bool) -> int { return bar; }"
end

test_sig
