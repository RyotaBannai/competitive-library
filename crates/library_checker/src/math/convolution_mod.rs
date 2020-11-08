use competitive::prelude::*;
#[doc(no_inline)]
pub use competitive::{
    math::NTT998244353,
    num::{mint_base::MInt998244353, MInt},
};

#[verify::verify("https://judge.yosupo.jp/problem/convolution_mod")]
pub fn convolution_mod(reader: &mut impl Read, writer: &mut impl Write) {
    let s = read_all(reader);
    let mut scanner = Scanner::new(&s);
    scan!(scanner, n, m, a: [MInt998244353; n], b: [MInt998244353; m]);
    let c = NTT998244353::convolve(a, b);
    echo(writer, c, ' ').ok();
}
