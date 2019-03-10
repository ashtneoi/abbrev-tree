fn common_prefix_length(a: &str, b: &str) -> usize {
    let mut aa = a.char_indices();
    let mut bb = b.char_indices();

    loop {
        match (aa.next(), bb.next()) {
            (Some((ai, ac)), Some((_, bc))) =>
                if ac != bc {
                    return ai;
                },
            (None, Some((bi, _))) => return bi,
            (Some((ai, _)), None) => return ai,
            (None, None) => return a.len(),
        }
    }
}

#[test]
fn test_common_prefix_length() {
    assert_eq!(common_prefix_length("foo", "foo"), 3);
    assert_eq!(common_prefix_length("foo", "foobar"), 3);
    assert_eq!(common_prefix_length("foobar", "foo"), 3);
    assert_eq!(common_prefix_length("foobar", "bar"), 0);
    assert_eq!(common_prefix_length("foobar", "foofuzz"), 3);
}
