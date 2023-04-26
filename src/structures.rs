#[derive(Debug, Eq, PartialEq)]
pub enum Element {
    Struct(StructElement),
    Enum(EnumElement),
}

#[derive(Debug, Eq, PartialEq)]
pub struct StructElement {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct EnumElement {
    pub name: String,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Variant {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Simple(String),
    Vec(Box<Type>),
    Tuple(Vec<Type>),
    Other(Box<Type>),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Field {
    /// Can be option because of enums:
    /// ```no_run
    /// enum TestEnum {
    ///     Variant1(u32)
    /// }
    /// ```
    /// In that case name will placed in  Variant struct and Field.name will be empty
    pub name: Option<String>,
    pub ty: Type,
}
