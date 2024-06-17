Use rust to write a code highlighting plug-in. Note that spans may overlap. This is boilerplate code.

```rust
#[derive(Debug)]
pub struct CodeRender<'i, 's> {
    /// raw text
    text: &'i str,
    /// start -> (styles , end)
    interval: BTreeMap<usize, InnerSpan<'s>>,
}

#[derive(Clone, Debug)]
struct InnerSpan<'s> {
    styles: BTreeSet<&'s str>,
    end: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct CodeView<'r, 'i, 's> {
    pub text: &'i str,
    pub kind: &'r BTreeSet<&'s str>,
}

impl<'i, 's> CodeRender<'i, 's> {
    pub fn new(text: &'i str) -> Self {
        let mut interval = BTreeMap::new();
        interval.insert(0, InnerSpan { styles: Default::default(), end: text.len() });
        Self { text, interval }
    }
    pub fn get_span<'r>(&'r mut self, offset: usize) -> Option<CodeView<'r, 'i, 's>> {
        let (start, span) = self.interval.range(offset..).next()?;
        let range = Range { start: *start, end: span.end };
        let text = self.text.get(range.clone())?;
        Some(CodeView { text, kind: &span.styles })
    }
    pub fn mark_span(&mut self, span: Range<usize>, style: &'s str) {
        match self.interval.get(&span.start) {
            Some(_) => {
                match self.interval.get(&span.end) {
                    // easy case, no need modifier old interval
                    Some(_) => self.interval.range_mut(span).for_each(|(_, inner)| {
                        inner.styles.insert(style);
                    }),
                    None => {
                        todo!()
                    }
                }
            }
            None => {
                // 修改前一个区间的结束位置
                self.interval.range_mut(..span.start).next_back().map(|(_, v)| {
                    v.end = span.start;
                });
                todo!()
            }
        }
    }
}

#[test]
fn main() {
    let mut code_render = CodeRender::new("public class Main {}");
    code_render.mark_span(0..17, "class");
    code_render.mark_span(0..3, "keyword");
    code_render.mark_span(7..12, "keyword");
    println!("{:#?}", code_render);
}

```