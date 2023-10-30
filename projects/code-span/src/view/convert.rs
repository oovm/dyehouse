use super::*;

impl<T> From<&str> for CodeRender2<T>
where
    T: Clone,
{
    fn from(s: &str) -> Self {
        CodeRender2::blank(s)
    }
}

impl<T> From<String> for CodeRender2<T>
where
    T: Clone,
{
    fn from(s: String) -> Self {
        CodeRender2::blank(s)
    }
}
