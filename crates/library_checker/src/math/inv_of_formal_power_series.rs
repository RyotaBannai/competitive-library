use competitive::prelude::*;
#[doc(no_inline)]
pub use competitive::{math::Fps998244353, num::mint_basic::MInt998244353};

#[cfg_attr(nightly, verify::verify("https://judge.yosupo.jp/problem/inv_of_formal_power_series"))]
pub fn inv_of_formal_power_series(reader: impl Read, mut writer: impl Write) {
    let s = read_all_unchecked(reader);
    let mut scanner = Scanner::new(&s);
    scan!(scanner, n, a: [MInt998244353; n]);
    let f = Fps998244353::from_vec(a);
    let g = f.inv(n);
    iter_print!(writer, @iter g.data);
}
