use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

mod iter;

pub struct CodeRender<'i, 's> {
    text: &'i str,
    view: BTreeMap<usize, InnerSpan<'s>>,
}

struct InnerSpan<'s> {
    style: BTreeSet<&'s str>,
    end: usize,
}
