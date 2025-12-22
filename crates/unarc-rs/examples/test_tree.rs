fn main() {
    // Test frequency tree with 4 symbols, all freq=1
    let tl = 4;
    let mut tree = vec![0u16; tl * 2];

    // Init leaves
    for i in tl..2 * tl {
        tree[i] = 1;
    }

    // Build internal nodes
    let mut j = tl * 2 - 2;
    for i in (1..tl).rev() {
        tree[i] = tree[j] + tree[j + 1];
        j = j.saturating_sub(2);
    }

    println!("Tree: {:?}", tree);
    println!("Total at [1]: {}", tree[1]);

    // Test decode for threshold=0,1,2,3
    for threshold in 0..4 {
        let mut l = 2;
        let mut lt = 0;

        while l < tl {
            println!(
                "  l={}, lt={}, tree[l]={}, threshold={}",
                l, lt, tree[l], threshold
            );
            if lt + tree[l] <= threshold {
                println!("    MATCH");
                lt += tree[l];
                l += 1;
            } else {
                println!("    NO MATCH");
            }
            l *= 2;
        }

        let symbol = l - tl;
        println!(
            "threshold={}: symbol={}, cumulative={}",
            threshold, symbol, lt
        );
        println!();
    }
}
