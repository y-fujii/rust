
fn main() {
    #macro[[#apply[f, [x, ...]], f(x, ...)]];

    macro_rules! apply_tt(
        ($f:expr, ($($x:expr),*)) => {$f($($x),*)}
    )

    fn add(a: int, b: int) -> int { return a + b; }

    assert(apply!(add, [1, 15]) == 16);
    assert(apply!(add, [1, 15]) == 16);
    assert(apply_tt!(add, (1, 15)) == 16);
}
