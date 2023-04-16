use testing::sploosh;
use testing::splish;

#[test]
fn integration_one() {
    assert_eq!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)), 4);
}