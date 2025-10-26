use comfy_table::{presets::ASCII_MARKDOWN, Cell, CellAlignment, Cells, Row, Table};
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
            .set_header(Self::labeled_row("", self.header()))
            .add_row(Self::labeled_row("impl", self.impl_row()))
            .add_row(Self::labeled_row("where", self.where_row()));
        table
            .column_iter_mut() // comfy table maybe not support markdown centering?
            .for_each(|c| c.set_cell_alignment(CellAlignment::Center));
        table
    }

    pub fn labeled_row<T: Into<Cells>, U: Into<Cell>>(label: U, row: T) -> Row {
        Cells(vec![label.into()].into_iter().chain(row.into().0).collect()).into()
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
    pub fn impl_row(&self) -> [&str; 3] {
        let Self { attr, .. } = self;
        [attr.is_annotated(), attr.is_monoid(), attr.is_commutative()].map(Self::impl_cell)
    }
    pub fn impl_cell(is: bool) -> &'a str {
        if is {
            "✅"
        } else {
            "❌"
        }
    }

    pub fn where_row(&self) -> [String; 3] {
        let Self { attr, .. } = self;
        [
            attr.annotation_where(),
            attr.unit_where(),
            attr.commutative_where(),
        ]
        .map(|w| match w {
            Some(w) => format!("`{w}`"),
            None => String::new(),
        })
    }
}
