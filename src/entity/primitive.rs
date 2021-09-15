use std::collections::HashMap;

use super::*;
use serde::{Serialize, Serializer};

#[derive(Serialize, Clone)]
pub struct PrimitiveReg
{
    // #[serde(rename = "type")] 
    // #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde(skip)] pub init: JsValue,
    #[serde(rename = "defaultComponents")] 
    default_components: HashMap<Cow<'static, str>, ComponentDefaults>
}

#[derive(Serialize, Clone)]
pub struct ComponentDefaults
{
    //
}