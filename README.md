# bitrev - bit reversion in rust

Implementation of a few different bit-reversion algorithms:

* Regular: split into bytes, reverse each byte in itself and the collection of bytes
* SIMD: same as above but using rusts portable SIMD
* Naive: convert value to a formatted binary String, reverse the string and convert back
* Gpt: whatever ChatGPT (GPT-3.5) replied to the prompt
    > Please write some idiomatic, high-performance Rust code that implements this trait
    > ```
    > pub trait ReverseBits {
    >     fn reverse_bits(self) -> Self;
    > }
    > ```
    > for `$type`.
 
    for `$type = u8, u16, ..., u128` (I renamed the trait and method name afterwards). These basically all ended up being algorithms that do some variation of "scanning through the binary using shifts and accumulate a result". The `u32` case is interesting in that it uses a lookup table for certain values.
* LUT: store all 8-bit reversions in a table and just index into that as necessary
