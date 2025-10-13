use syn::ItemStruct;

use crate::{constant::Constant, property::attr::ContainerAttr};

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
    pub fn table(&self) -> String {
        format!("TODO {:?}", self.attr)
    }
}
