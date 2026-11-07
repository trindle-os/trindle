pub use xkbcommon::xkb;

#[derive(Clone, Debug, Default)]
pub struct XkbConfig<'a> {
    pub rules: &'a str,
    pub model: &'a str,
    pub layout: &'a str,
    pub variant: &'a str,
    pub options: Option<String>,
}

impl XkbConfig<'_> {
    pub(crate) fn compile_keymap(&self, context: &xkb::Context) -> Result<xkb::Keymap, ()> {
        xkb::Keymap::new_from_names(
            context,
            self.rules,
            self.model,
            self.layout,
            self.variant,
            self.options.clone(),
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        )
        .ok_or(())
    }
}
