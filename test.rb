require_relative "rust"
class MyTest
  inline(:Rust) do |builder|
    builder.rust <<-RUST
      fn factorial(max: i64) -> i64 {
        let mut i = max;
        let mut result = 1;
        while i >= 2 { result *= i; i -= 1; }
        return result;
      }
      RUST
  end
end
t = MyTest.new()
factorial_5 = t.factorial()
