use competitive::prelude::*;
#[doc(no_inline)]
pub use competitive::{
    algebra::{LinearOperation, ReverseOperation},
    data_structure::SegmentTree,
    graph::TreeGraphScanner,
    num::{mint_basic::MInt998244353, MInt},
    tree::HeavyLightDecomposition,
};

#[verify::verify("https://judge.yosupo.jp/problem/vertex_set_path_composite")]
pub fn vertex_set_path_composite(reader: impl Read, mut writer: impl Write) {
    let s = read_all(reader);
    let mut scanner = Scanner::new(&s);
    scan!(scanner, n, q, ab: [(MInt998244353, MInt998244353); n], (mut graph, _): { TreeGraphScanner::<usize, ()>::new(n) });
    let hld = HeavyLightDecomposition::new(0, &mut graph);
    let monoid = LinearOperation::new();
    let mut nab = vec![(MInt998244353::zero(), MInt998244353::zero()); n];
    for i in 0..n {
        nab[hld.vidx[i]] = ab[i];
    }
    let mut seg1 = SegmentTree::from_vec(nab.clone(), monoid);
    let mut seg2 = SegmentTree::from_vec(nab, ReverseOperation::new(monoid));
    for _ in 0..q {
        scan!(scanner, ty);
        if ty == 0 {
            scan!(scanner, p, cd: (MInt998244353, MInt998244353));
            seg1.set(hld.vidx[p], cd);
            seg2.set(hld.vidx[p], cd);
        } else {
            scan!(scanner, u, v, x: MInt998244353);
            let (a, b) = hld.query_noncom(
                u,
                v,
                false,
                |l, r| seg1.fold(l, r),
                |l, r| seg2.fold(l, r),
                &monoid,
            );
            writeln!(writer, "{}", a * x + b).ok();
        }
    }
}
