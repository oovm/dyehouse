use std::ops::Range;

use crate::overlap::{CodeHighlighter, CodeView, IntervalRepr};

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
    iter: std::collections::btree_map::Iter<'r, usize, IntervalRepr<'s>>,
}

impl<'r, 'i, 's> IntoIterator for &'r CodeHighlighter<'i, 's> {
    type Item = CodeView<'r, 'i, 's>;
    type IntoIter = CodeRendered<'r, 'i, 's>;

    fn into_iter(self) -> Self::IntoIter {
        CodeRendered { text: self.text, iter: self.intervals.iter() }
    }
}

impl<'r, 'i, 's> Iterator for CodeRendered<'r, 'i, 's> {
    type Item = CodeView<'r, 'i, 's>;

    fn next(&mut self) -> Option<Self::Item> {
        let (start, span) = self.iter.next()?;
        // let safe_end = span.end.min(self.text.len());
        let range = Range { start: *start, end: span.end };
        let text = self.text.get(range.clone())?;
        Some(CodeView { text, kind: &span.styles })
    }
}
