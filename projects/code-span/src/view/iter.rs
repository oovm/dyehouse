use std::collections::BTreeSet;
use std::ops::Range;

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

pub struct CodeView<'i, 's> {
    pub text: &'i str,
    pub kind: BTreeSet<&'s str>,
}

impl<'r, 'i, 's> IntoIterator for &'r CodeRender<'i, 's>
{
    type Item = CodeView<'i, 's>;
    type IntoIter = CodeRendered<'r, 'i, 's>;

    fn into_iter(self) -> Self::IntoIter {
        CodeRendered {
            text: self.text,
            iter: self.view.iter(),
        }
    }
}

impl<'r, 'i, 's> Iterator for CodeRendered<'r, 'i, 's>
{
    type Item = CodeView<'i, 's>;

    fn next(&mut self) -> Option<Self::Item> {
        let (start, span) = self.iter.next()?;
        let range = Range { start: *start, end: *span.end };



    }
}

