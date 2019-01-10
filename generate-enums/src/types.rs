#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Root {
    pub typedefs: Vec<Typedef>,
    pub enums: Vec<Enum>,
    pub consts: Vec<Const>,
    pub structs: Vec<Struct>,
    pub methods: Vec<Method>,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Typedef {
    #[serde(rename = "typedef")]
    pub id: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Enum {
    #[serde(rename = "enumname")]
    pub id: String,
    #[serde(rename = "values")]
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnumVariant {
    #[serde(rename = "name")]
    pub id: String,
    #[serde(rename = "value")]
    pub lit: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Const {
    #[serde(rename = "constname")]
    pub id: String,
    #[serde(rename = "consttype")]
    pub ty: String,
    #[serde(rename = "constval")]
    pub lit: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Struct {
    #[serde(rename = "struct")]
    pub id: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Field {
    #[serde(rename = "fieldname")]
    pub id: String,
    #[serde(rename = "fieldtype")]
    pub ty: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Method {
    #[serde(rename = "classname")]
    pub struct_id: String,
    #[serde(rename = "methodname")]
    pub fn_id: String,
    #[serde(rename = "returntype")]
    pub ret_ty: String,
    pub params: Option<Vec<Param>>,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Param {
    #[serde(rename = "paramname")]
    pub id: String,
    #[serde(rename = "paramtype")]
    pub ty: String,
}
