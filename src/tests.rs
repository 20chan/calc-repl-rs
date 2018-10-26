#[test]
pub fn test_interpreter() {
    execute("1+2");
    assert_eq!(execute("1+2"), 3);
}
