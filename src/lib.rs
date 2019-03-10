enum Completion {
    Zero,
    One(String),
    Many,
}

pub struct AbbrevTree(Vec<(String, AbbrevTree)>);

impl AbbrevTree {
    pub fn new() -> Self {
        AbbrevTree(Vec::new())
    }

    // TODO: Recursion is probably bad but oh well.
    pub fn add(&mut self, item: &str) {
        if item.len() == 0 {
            return;
        }

        // Find match.
        for (chunk, subtree) in &mut self.0 {
            let prefix_len = common_prefix_length(chunk, item);
            if prefix_len == chunk.len() {
                // Full match. Recurse. (Optimize for the case where item
                // doesn't already exist.)
                return subtree.add(&item[prefix_len..]);
            } else if prefix_len > 0 {
                // Partial match. Split and then add.
                let chunk_suffix = chunk.split_off(prefix_len);
                let v: Vec<_> = subtree.0.drain(..).collect();
                subtree.0.push((
                    chunk_suffix,
                    AbbrevTree(v),
                ));
                return subtree.add(&item[prefix_len..]);
            }
        }

        // Else add new subtree.
        self.0.push((
            item.to_string(),
            AbbrevTree::new(),
        ));
    }

    /*
     *pub fn complete(&self, item: &self) -> Completion {
     *    for (chunk, subtree) in &self.0 {
     *        let prefix_len = common_prefix_length(chunk, item);
     *        if prefix_len == chunk.len() {
     *            // Full match. Recurse.
     *            match subtree.complete(&item[prefix_len..]) {
     *                Zero => return Zero,
     *                One(mut s) => {
     *                    s.insert_str(0, chunk);
     *                    return One(s);
     *                },
     *                Many => return Many,
     *            }
     *        } else if prefix_len > 0 {
     *            return Many;
     *        }
     *    }
     *}
     */
}

#[cfg(test)]
#[test]
fn test_abbrev_tree() {
    let mut t = AbbrevTree::new();
    assert_eq!(t.0.len(), 0);

    t.add("cat");
    assert_eq!(t.0.len(), 1);
    assert_eq!(t.0[0].0, "cat");
    assert_eq!((t.0[0].1).0.len(), 0);

    t.add("cargo");
    assert_eq!(t.0.len(), 1);
    assert_eq!(t.0[0].0, "ca");
    assert_eq!((t.0[0].1).0.len(), 2);
    assert_eq!((t.0[0].1).0[0].0, "t");
    assert_eq!((t.0[0].1).0[1].0, "rgo");

    t.add("chmod");
    assert_eq!(t.0.len(), 1);
    assert_eq!(t.0[0].0, "c");

    t.add("chown");
    assert_eq!(t.0.len(), 1);
    assert_eq!(t.0[0].0, "c");

    t.add("ls");
    assert_eq!(t.0.len(), 2);
    assert_eq!(t.0[0].0, "c");
    assert_eq!(t.0[1].0, "ls");
}

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

#[cfg(test)]
#[test]
fn test_common_prefix_length() {
    assert_eq!(common_prefix_length("foo", "foo"), 3);
    assert_eq!(common_prefix_length("foo", "foobar"), 3);
    assert_eq!(common_prefix_length("foobar", "foo"), 3);
    assert_eq!(common_prefix_length("foobar", "bar"), 0);
    assert_eq!(common_prefix_length("foobar", "foofuzz"), 3);
}