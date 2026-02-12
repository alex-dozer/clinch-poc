pub struct LuopDescriptor {
    pub fn_name: &'static str,
    pub output: &'static str,
    pub module: &'static str,
}

inventory::collect!(LuopDescriptor);

impl IntoIterator for LuopDescriptor {
    type Item = (&'static str, &'static str, &'static str);
    type IntoIter = std::iter::Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once((self.fn_name, self.output, self.module))
    }
}
