use algebraic_structure::magma::AddOperator;
use persistent_segment_tree::PersistentSegmentTree;

fn main() {
    let mut segs = vec![];
    segs.push(PersistentSegmentTree::<AddOperator<i32>>::new(100));
    for i in 0..100 {
        segs.push(segs[i].set(i, i as i32));
    }
    for i in 10..20 {
        segs.push(segs.last().unwrap().add(i, i as i32));
    }
    segs.push(segs[100].copy_range(10..15, &segs[110]));
    for i in 0..100 {
        eprintln!("{}", segs.last().unwrap().get(i));
    }
}
