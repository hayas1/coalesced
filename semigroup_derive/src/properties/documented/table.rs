use comfy_table::{presets::ASCII_MARKDOWN, Table};
use syn::ItemStruct;

use crate::{constant::Constant, properties::attr::ContainerAttr};

#[derive(Debug, Clone)]
pub struct PropertiesTable<'a> {
    constant: &'a Constant,
    attr: &'a ContainerAttr,
    item: &'a ItemStruct,
}
impl<'a> PropertiesTable<'a> {
    pub fn new(constant: &'a Constant, attr: &'a ContainerAttr, item: &'a ItemStruct) -> Self {
        Self {
            constant,
            attr,
            item,
        }
    }
    pub fn table(&self) -> Table {
        let mut table = Table::new();
        table
            .load_preset(ASCII_MARKDOWN)
            .set_header(self.header())
            .add_row(self.row());
        table
    }
    pub fn header(&self) -> Vec<&str> {
        vec!["annotated", "monoid", "commutative"]
    }
    pub fn row(&self) -> Vec<&str> {
        vec![
            Self::cell(self.attr.is_annotated()),
            Self::cell(self.attr.is_monoid()),
            Self::cell(self.attr.is_commutative()),
        ]
    }
    pub fn cell(is: bool) -> &'a str {
        if is {
            "✅"
        } else {
            "❌"
        }
    }
}
