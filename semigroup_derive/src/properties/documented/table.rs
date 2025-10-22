use comfy_table::{CellAlignment, Table, presets::ASCII_MARKDOWN};
use syn::ItemStruct;

use crate::{constant::Constant, properties::attr::ContainerAttr};

#[derive(Debug, Clone)]
pub struct PropertiesTable<'a> {
    constant: &'a Constant,
    attr: &'a ContainerAttr,
}
impl<'a> PropertiesTable<'a> {
    pub fn new(constant: &'a Constant, attr: &'a ContainerAttr, item: &'a ItemStruct) -> Self {
        let _ = item;
        Self { constant, attr }
    }
    pub fn table(&self) -> Table {
        let mut table = Table::new();
        table
            .load_preset(ASCII_MARKDOWN)
            .set_header(self.header())
            .add_row(self.row());
        table
            .column_iter_mut() // comfy table maybe not support markdown centering?
            .for_each(|c| c.set_cell_alignment(CellAlignment::Center));
        table
    }
    pub fn header(&self) -> [String; 3] {
        let Constant {
            path_annotate,
            path_monoid,
            path_commutative,
            ..
        } = self.constant;
        [path_annotate, path_monoid, path_commutative].map(|p| {
            format!(
                "[`{}`]",
                p.segments
                    .iter()
                    .map(|seg| seg.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::")
            )
        })
    }
    pub fn row(&self) -> [&str; 3] {
        let Self { attr, .. } = self;
        [attr.is_annotated(), attr.is_monoid(), attr.is_commutative()].map(Self::cell)
    }
    pub fn cell(is: bool) -> &'a str {
        if is { "✅" } else { "❌" }
    }
}
