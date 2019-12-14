use exercises::int_stats::calc_stats;

fn main() {
    let vals = vec![10, 5, 3]; // mean=6, median=5, mode=None
    let stats = calc_stats(&vals);
    println!("Stats for {:?}: {:#?}", vals, stats);

    let vals = vec![3, 1, 2, 3, 4]; // mean=2.6, median=3, mode=3
    let stats = calc_stats(&vals);
    println!("Stats for {:?}: {:#?}", vals, stats);

    let vals = vec![3, 4, 2, 10, 4, 0]; // mean=3.83, median=3.5, mode=4
    let stats = calc_stats(&vals);
    println!("Stats for {:?}: {:#?}", vals, stats);

    let vals = vec![2, 0, 2, 3, 4]; // mean=2.2, median=2, mode=2
    let stats = calc_stats(&vals);
    println!("Stats for {:?}: {:#?}", vals, stats);
}
