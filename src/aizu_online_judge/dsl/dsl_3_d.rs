pub use crate::algebra::MinOperation;
pub use crate::data_structure::QueueAggregation;
use crate::prelude::*;

#[verify_attr::verify("https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/3/DSL_3_D")]
pub fn dsl_3_d(reader: &mut impl Read, writer: &mut impl Write) {
    let s = read_all(reader);
    let mut scanner = Scanner::new(&s);
    scan!(scanner, n, l, a: [u64; n]);
    let mut que = QueueAggregation::new(MinOperation::new());
    let mut ans = Vec::with_capacity(n - l + 1);
    for a in a {
        que.push(a);
        if que.len() == l {
            ans.push(que.fold_all());
            que.pop();
        }
    }
    echo!(writer, ans, " ");
}