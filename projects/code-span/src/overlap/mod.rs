use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    ops::Range,
};

pub use self::iter::CodeRendered;

mod iter;

#[derive(Debug)]
pub struct CodeHighlighter<'i, 's> {
    /// raw text
    text: &'i str,
    /// start -> (styles , end)
    intervals: BTreeMap<usize, IntervalRepr<'s>>,
}

#[derive(Copy, Clone, Debug)]
pub struct CodeView<'r, 'i, 's> {
    pub text: &'i str,
    pub kind: &'r BTreeSet<&'s str>,
}

#[derive(Clone, Debug)]
struct IntervalRepr<'s> {
    styles: BTreeSet<&'s str>,
    end: usize,
}

impl<'i, 's> CodeHighlighter<'i, 's> {
    pub fn new(text: &'i str) -> Self {
        let mut interval = BTreeMap::new();
        // Mark all code as no style
        interval.insert(0, IntervalRepr { styles: Default::default(), end: text.len() });
        Self { text, intervals: interval }
    }
    pub fn mark_span(&mut self, span: Range<usize>, style: &'s str) {
        match self.intervals.get(&span.start) {
            Some(_) => {
                match self.intervals.get(&span.end) {
                    // simple case, no need change old interval
                    Some(_) => self.mark_span_unchecked(span, style),
                    // remark end position
                    None => {
                        let (_, last_span) = self.intervals.range(span.end..).next().unwrap();
                        let new_end = last_span.end;
                        let new_styles = last_span.styles.clone();
                        self.intervals.remove(&span.end);
                        self.intervals.entry(span.end).or_insert(IntervalRepr { styles: new_styles, end: new_end });
                        self.mark_span_unchecked(span, style)
                    }
                }
            }
            None => {
                // truncate the previous span
                let prev_span = self.intervals.range_mut(..span.start).next_back();
                if let Some((_, prev_inner)) = prev_span {
                    prev_inner.end = span.start;
                }
                // insert start position
                let (_, next_span) = self.intervals.range(span.start..).next().unwrap();
                let new_end = next_span.end;
                let new_styles = next_span.styles.clone();
                self.intervals.remove(&span.start);
                self.intervals.entry(span.start).or_insert(IntervalRepr { styles: new_styles, end: new_end });
                self.mark_span_unchecked(span, style)
            }
        }
    }
    fn mark_span_unchecked(&mut self, span: Range<usize>, style: &'s str) {
        for (_, inner) in self.intervals.range_mut(span) {
            inner.styles.insert(style);
        }
    }
}

#[test]
fn colorize() {
    let mut code_render = CodeHighlighter::new("public class Main {}");
    code_render.mark_span(0..6, "keyword");
    code_render.mark_span(7..12, "keyword");
    code_render.mark_span(13..17, "class");
    println!("{:#?}", code_render);
}
