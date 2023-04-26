use crate::structures::*;
use std::convert::TryFrom;
#[cfg(test)]
use syn::{parse_str, ItemEnum, ItemStruct};
use Option;

use proc_macro2::Ident as SynIdent;
use syn::{
    AngleBracketedGenericArguments, Field as SynField, GenericArgument, Item, PathArguments,
    PathSegment, Type as SynType, TypePath, Variant as SynVariant,
};

#[derive(Debug, Clone)]
pub enum ConversionError {
    UnsupportedType,
}

impl TryFrom<Item> for Element {
    type Error = ConversionError;
    fn try_from(item: Item) -> Result<Self, Self::Error> {
        match item {
            Item::Struct(item_struct) => {
                let struct_element = StructElement::from(item_struct);
                Ok(Element::Struct(struct_element))
            }
            Item::Enum(item_enum) => {
                let enum_element = EnumElement::from(item_enum);
                Ok(Element::Enum(enum_element))
            }
            _ => Err(ConversionError::UnsupportedType),
        }
    }
}

impl From<syn::ItemStruct> for StructElement {
    fn from(item_struct: syn::ItemStruct) -> Self {
        let name = item_struct.ident.to_string();
        let mut fields: Vec<Field> = Vec::new();
        for item_struct_field in item_struct.fields {
            let field = Field::try_from(item_struct_field);
            match field {
                Ok(field) => fields.push(field),
                Err(_) => {}
            }
        }

        StructElement { name, fields }
    }
}

impl From<syn::ItemEnum> for EnumElement {
    fn from(item_enum: syn::ItemEnum) -> Self {
        let name = item_enum.ident.to_string();
        let variants: Vec<Variant> = item_enum.variants.into_iter().map(Variant::from).collect();

        EnumElement { name, variants }
    }
}

pub fn try_convert_option_ident_to_option_string(value: Option<SynIdent>) -> Option<String> {
    match value {
        Some(ident) => Some(ident.to_string()),
        None => None,
    }
}

pub fn try_convert_ident_to_string(value: SynIdent) -> Result<String, ConversionError> {
    Ok(value.to_string())
}

impl TryFrom<SynField> for Field {
    type Error = ConversionError;
    fn try_from(syn_field: SynField) -> Result<Self, Self::Error> {
        let name = try_convert_option_ident_to_option_string(syn_field.ident);

        match syn_field.ty {
            syn::Type::Path(type_path) => {
                let ty = Type::from(type_path);

                return Ok(Field { name, ty });
            }
            _ => return Err(ConversionError::UnsupportedType),
        }
    }
}

impl From<SynVariant> for Variant {
    fn from(syn_variant: SynVariant) -> Self {
        let name = syn_variant.ident.to_string();
        let mut fields: Vec<Field> = Vec::new();
        for field in syn_variant.fields {
            let try_from_field = Field::try_from(field);
            match try_from_field {
                Ok(field) => {
                    fields.push(field);
                }
                Err(_) => {}
            }
        }

        Variant { name, fields }
    }
}

impl From<SynType> for Type {
    fn from(syn_type: SynType) -> Self {
        match syn_type {
            SynType::Path(type_path) => {
                let path = type_path.path;
                let ident = path.segments.last().unwrap().ident.to_string();
                Type::Simple(ident)
            }
            SynType::Array(array) => {
                let inner_type = *array.elem;
                Type::Vec(Box::new(Type::from(inner_type)))
            }
            SynType::Tuple(tuple) => {
                let mut tuple_vec = Vec::new();
                for el in tuple.elems {
                    tuple_vec.push(Type::from(el))
                }
                Type::Tuple(tuple_vec)
            }
            _ => panic!("Unsupported type: {:?}", syn_type),
        }
    }
}

impl From<TypePath> for Type {
    fn from(type_path: TypePath) -> Self {
        let path = type_path.path;
        let path_sement = path.segments.last();
        match path_sement {
            Some(seg) => return Type::from(seg.clone()),
            None => {
                panic!("Error in from TypePath to Type")
            }
        }
        //Type::Simple(ident)
    }
}

impl From<PathSegment> for Type {
    fn from(path_segment: PathSegment) -> Self {
        let result = Type::try_from(path_segment.arguments);
        match result {
            Ok(res) => {
                let ident_string = path_segment.ident.to_string();
                if ident_string == "Vec".to_string() {
                    return Type::Vec(Box::new(res));
                }

                return res;
            }
            Err(_) => return Type::Simple(path_segment.ident.to_string()),
        }
    }
}

impl TryFrom<PathArguments> for Type {
    type Error = ConversionError;
    fn try_from(value: PathArguments) -> Result<Self, Self::Error> {
        match value {
            PathArguments::AngleBracketed(x) => {
                let result = match Type::try_from(x) {
                    Ok(x) => Ok(x),
                    Err(_) => return Err(ConversionError::UnsupportedType),
                };
                return result;
            }
            PathArguments::None => return Err(ConversionError::UnsupportedType),
            PathArguments::Parenthesized(_) => return Err(ConversionError::UnsupportedType),
        }
    }
}
impl TryFrom<AngleBracketedGenericArguments> for Type {
    type Error = ConversionError;
    fn try_from(value: AngleBracketedGenericArguments) -> Result<Self, Self::Error> {
        Ok(Type::from(value.args[0].clone()))
    }
}

impl From<GenericArgument> for Type {
    /// TODO: match all types and correct error-hadling
    fn from(value: GenericArgument) -> Self {
        match value {
            GenericArgument::Type(x) => Type::from(x),
            _ => panic!("Unknown type!\n"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_struct_element_from() {
        let code = "pub struct TestStruct { field1: u32, field2: String }";
        let item_struct: ItemStruct = parse_str(code).unwrap();
        let struct_element = StructElement::from(item_struct);

        assert_eq!(struct_element.name, "TestStruct");
        assert_eq!(struct_element.fields.len(), 2);
        assert_eq!(struct_element.fields[0].name, Some("field1".to_string()));
        assert_eq!(struct_element.fields[0].ty, Type::Simple("u32".to_string()));
        assert_eq!(struct_element.fields[1].name, Some("field2".to_string()));
        assert_eq!(
            struct_element.fields[1].ty,
            Type::Simple("String".to_string())
        );
    }

    #[test]
    fn test_enum_element_from() {
        let code = "
        pub enum TestEnum {
            Variant1(u32),
            Variant2 { field: String },
        }
    ";
        let item_enum: ItemEnum = parse_str(code).unwrap();
        let enum_element = EnumElement::from(item_enum);

        assert_eq!(enum_element.name, "TestEnum");
        assert_eq!(enum_element.variants.len(), 2);
        assert_eq!(enum_element.variants[0].name, "Variant1");
        assert_eq!(enum_element.variants[0].fields.len(), 1);
        assert_eq!(enum_element.variants[1].name, "Variant2");
        assert_eq!(enum_element.variants[1].fields.len(), 1);
    }

    #[test]
    fn test_field_from() {
        let code = "pub struct TestStruct { field1: u32 }";
        let item_struct: ItemStruct = parse_str(code).unwrap();
        let syn_field = item_struct.fields.into_iter().next().unwrap();
        let field = Field::try_from(syn_field);

        match field {
            Ok(field) => {
                assert_eq!(field.name, Some("field1".to_string()));
                assert_eq!(field.ty, Type::Simple("u32".to_string()));
            }
            Err(_) => {
                panic!("Error field conversion")
            }
        }
    }

    #[test]
    fn test_field_vec() {
        let code = r#"
        pub struct C {
            pub f: F,
            pub g: Vec<G>,
        }
    "#;

        let parsed: syn::Item = parse_str(code).unwrap();
        let element = Element::try_from(parsed).unwrap();

        let expected_element = Element::Struct(StructElement {
            name: "C".to_string(),
            fields: vec![
                Field {
                    name: Some("f".to_string()),
                    ty: Type::Simple("F".to_string()),
                },
                Field {
                    name: Some("g".to_string()),
                    ty: Type::Vec(Box::new(Type::Simple("G".to_string()))),
                },
            ],
        });

        assert_eq!(element, expected_element);
    }

    #[test]
    fn test_variant_from() {
        let code = "
        pub enum TestEnum {
            Variant1(u32),
        }
    ";
        let item_enum: ItemEnum = parse_str(code).unwrap();
        let syn_variant = item_enum.variants.into_iter().next().unwrap();
        let variant = Variant::from(syn_variant);

        assert_eq!(variant.name, "Variant1");
        assert_eq!(variant.fields.len(), 1);
        assert_eq!(variant.fields[0].name, None);
        assert_eq!(variant.fields[0].ty, Type::Simple("u32".to_string()));
    }

    #[test]
    fn test_element_from() {
        let code = "pub struct TestStruct { field1: u32 }";
        let item: Item = parse_str(code).unwrap();
        let element = Element::try_from(item);

        match element {
            Ok(element) => match element {
                Element::Struct(struct_element) => {
                    assert_eq!(struct_element.name, "TestStruct");
                    assert_eq!(struct_element.fields.len(), 1);
                }
                _ => panic!("Expected Element::Struct"),
            },
            _ => panic!("Expected Element::Struct"),
        }
    }

    #[test]
    fn test_try_convert_option_ident_to_option_string() {
        let opt_ident: Option<SynIdent> = Some(parse_str("test_ident").unwrap());
        let opt_string = try_convert_option_ident_to_option_string(opt_ident);

        assert_eq!(opt_string, Some("test_ident".to_string()));
    }

    #[test]
    fn test_try_convert_ident_to_string() {
        let ident: SynIdent = parse_str("test_ident").unwrap();
        let result = try_convert_ident_to_string(ident);

        match result {
            Ok(s) => assert_eq!(s, "test_ident".to_string()),
            Err(_) => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_type_try_from_angle_bracketed_generic_arguments() {
        let generic_args_str = "<u32>";
        let angle_bracketed_generic_args: AngleBracketedGenericArguments =
            parse_str(generic_args_str).unwrap();
        let ty = Type::try_from(angle_bracketed_generic_args);

        match ty {
            Ok(t) => assert_eq!(t, Type::Simple("u32".to_string())),
            Err(_) => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_type_from_generic_argument() {
        let generic_arg_str = "u32";
        let generic_arg: GenericArgument = parse_str(generic_arg_str).unwrap();
        let ty = Type::from(generic_arg);

        assert_eq!(ty, Type::Simple("u32".to_string()));
    }
}
