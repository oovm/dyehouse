use std::{collections::BTreeSet, ops::Range};

use crate::view::{CodeRender, InnerSpan};

/// # Arguments
///
/// * `text`:
/// * `default`:
///
/// returns: TextView<T>
///
/// # Examples
///
/// ```
/// use code_span::CodeRender2;
/// ```
#[derive(Debug)]
pub struct CodeRendered<'r, 'i, 's> {
    text: &'i str,
    iter: std::collections::btree_map::Iter<'r, usize, InnerSpan<'s>>,
}

#[derive(Copy, Clone, Debug)]
pub struct CodeView<'r, 'i, 's> {
    pub text: &'i str,
    pub kind: &'r BTreeSet<&'s str>,
}

impl<'r, 'i, 's> IntoIterator for &'r CodeRender<'i, 's> {
    type Item = CodeView<'r, 'i, 's>;
    type IntoIter = CodeRendered<'r, 'i, 's>;

    fn into_iter(self) -> Self::IntoIter {
        CodeRendered { text: self.text, iter: self.interval.iter() }
    }
}

impl<'r, 'i, 's> Iterator for CodeRendered<'r, 'i, 's> {
    type Item = CodeView<'r, 'i, 's>;

    fn next(&mut self) -> Option<Self::Item> {
        let (start, span) = self.iter.next()?;
        let range = Range { start: *start, end: span.end };
        let text = self.text.get(range.clone())?;
        Some(CodeView { text, kind: &span.styles })
    }
}
