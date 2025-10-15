use comfy_table::{presets::ASCII_MARKDOWN, Table};
use syn::ItemStruct;

use crate::{constant::Constant, properties::attr::ContainerAttr};

#[derive(Debug, Clone)]
pub struct PropertiesTable<'a> {
    attr: &'a ContainerAttr,
}
impl<'a> PropertiesTable<'a> {
    pub fn new(constant: &'a Constant, attr: &'a ContainerAttr, item: &'a ItemStruct) -> Self {
        let _ = (constant, item);
        Self { attr }
    }
    pub fn table(&self) -> Table {
        let mut table = Table::new();
        table
            .load_preset(ASCII_MARKDOWN)
            .set_header(self.header())
            .add_row(self.row());
        table
    }
    pub fn header(&self) -> [&str; 3] {
        self.attr.attributes()
    }
    pub fn row(&self) -> [&str; 3] {
        self.attr.fields().map(Self::cell)
    }
    pub fn cell(is: bool) -> &'a str {
        if is {
            "✅"
        } else {
            "❌"
        }
    }
}
