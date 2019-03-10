use std::fmt;

pub struct AbbrevTree(Vec<(String, AbbrevTree)>);

impl AbbrevTree {
    pub fn new() -> Self {
        AbbrevTree(Vec::new())
    }

    pub fn add(&mut self, item: &str) {
        self._add(item, true)
    }

    // TODO: Recursion is probably bad but oh well.
    // TODO: is_root sucks.
    fn _add(&mut self, item: &str, is_root: bool) {
        // Find match.
        for (chunk, subtree) in &mut self.0 {
            let prefix_len = common_prefix_length(chunk, item);
            if prefix_len > 0 {
                if prefix_len == chunk.len() {
                    // Full match. Recurse. (Optimize for the case where item
                    // doesn't already exist.)
                    return subtree._add(&item[prefix_len..], false);
                } else {
                    // Partial match. Split and then add.
                    let chunk_suffix = chunk.split_off(prefix_len);
                    let v: Vec<_> = subtree.0.drain(..).collect();
                    subtree.0.push((
                        chunk_suffix,
                        AbbrevTree(v),
                    ));
                    return subtree._add(&item[prefix_len..], false);
                }
            }
        }

        // Else add new subtree.
        if self.0.len() == 0 && !is_root {
            self.0.push((
                "".to_string(),
                AbbrevTree::new(),
            ));
        }
        self.0.push((
            item.to_string(),
            AbbrevTree::new(),
        ));
    }

    pub fn complete(&self, item: &str) -> Vec<String> {
        let mut v = Vec::new();
        self._complete("", item, &mut v);
        v
    }

    // TODO: This mega-sucks.
    fn _complete(&self, left: &str, item: &str, v: &mut Vec<String>) {
        println!(
            "_complete({:?}, {:?}, v)",
            left,
            item,
        );
        println!("{:?}", self);

        if self.0.len() == 0 && item == "" {
            v.push(left.to_string());
        }

        for (chunk, subtree) in &self.0 {
            let prefix_len = common_prefix_length(chunk, item);
            if item == "" || item.len() == prefix_len
                    || chunk.len() == prefix_len {
                let mut s = left.to_string();
                s.push_str(chunk);
                subtree._complete(&s, &item[prefix_len..], v);
            }
        }
    }
}

impl fmt::Debug for AbbrevTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // FIXME: We should check for f.alternate().
        let mut stack = vec![self.0.iter()];
        let mut first = true;
        while stack.len() > 0 {
            match stack.last_mut().unwrap().next() {
                Some(x) => {
                    if !first {
                        write!(f, "\n")?;
                    }
                    write!(
                        f,
                        "{}{:?}",
                        " ".repeat(2 * (stack.len()-1)),
                        x.0,
                    )?;
                    stack.push((x.1).0.iter());
                },
                None => { stack.pop(); },
            }
            first = false;
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_abbrev_tree() {
    let mut t = AbbrevTree::new();
    println!("{:?}", t);
    assert_eq!(t.0.len(), 0);

    t.add("cat");
    println!("{:?}", t);
    assert_eq!(t.0.len(), 1);
    assert_eq!(t.0[0].0, "cat");
    assert_eq!((t.0[0].1).0.len(), 0);

    t.add("cargo");
    println!("{:?}", t);
    assert_eq!(t.0.len(), 1);
    assert_eq!(t.0[0].0, "ca");
    assert_eq!((t.0[0].1).0.len(), 2);
    assert_eq!((t.0[0].1).0[0].0, "t");
    assert_eq!(((t.0[0].1).0[0].1).0.len(), 0);
    assert_eq!((t.0[0].1).0[1].0, "rgo");
    assert_eq!(((t.0[0].1).0[1].1).0.len(), 0);

    t.add("chmod");
    println!("{:?}", t);
    assert_eq!(t.0.len(), 1);
    assert_eq!(t.0[0].0, "c");

    t.add("chown");
    println!("{:?}", t);
    assert_eq!(t.0.len(), 1);
    assert_eq!(t.0[0].0, "c");

    t.add("ls");
    println!("{:?}", t);
    assert_eq!(t.0.len(), 2);
    assert_eq!(t.0[0].0, "c");
    assert_eq!(t.0[1].0, "ls");

    t.add("lshw");
    println!("{:?}", t);

    assert_eq!(t.complete("c"), vec![
        "cat".to_string(),
        "cargo".to_string(),
        "chmod".to_string(),
        "chown".to_string(),
    ]);
    assert_eq!(t.complete("ca"), vec![
        "cat".to_string(),
        "cargo".to_string(),
    ]);
    assert_eq!(t.complete("cat"), vec!["cat".to_string()]);
    assert_eq!(t.complete("ch"), vec![
        "chmod".to_string(),
        "chown".to_string(),
    ]);
    assert_eq!(t.complete("cho"), vec!["chown".to_string()]);
    assert_eq!(t.complete("chow"), vec!["chown".to_string()]);
    assert_eq!(t.complete("chown"), vec!["chown".to_string()]);
    assert_eq!(t.complete("l"), vec![
        "ls".to_string(),
        "lshw".to_string(),
    ]);
    assert_eq!(t.complete("ls"), vec![
        "ls".to_string(),
        "lshw".to_string(),
    ]);
    assert_eq!(t.complete("lsh"), vec!["lshw".to_string()]);
    assert_eq!(t.complete("lshw"), vec!["lshw".to_string()]);
    assert_eq!(t.complete("x"), Vec::<String>::new());
    assert_eq!(t.complete("xyz"), Vec::<String>::new());
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
    assert_eq!(common_prefix_length("", "foo"), 0);
    assert_eq!(common_prefix_length("foo", "foo"), 3);
    assert_eq!(common_prefix_length("foo", "foobar"), 3);
    assert_eq!(common_prefix_length("foobar", "foo"), 3);
    assert_eq!(common_prefix_length("foobar", "bar"), 0);
    assert_eq!(common_prefix_length("foobar", "foofuzz"), 3);
}
