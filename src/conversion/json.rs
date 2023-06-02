use crate::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;
pub type JsonDict = serde_json::Map<String, JsonValue>;

pub use serde_json::Value as JsonValue;

/// Convert an object to json string.
pub fn obj_to_json<T>(obj: &T) -> Outcome<String>
where
    T: Serialize,
{
    let json: String = serde_json::to_string(obj).catch(
        EXN::SerializationException,
        &format!(
            "Failed to convert object of type `{}` to String",
            std::any::type_name::<T>()
        ),
    )?;
    Ok(json)
}

/// Convert an object to formatted json string.
pub fn obj_to_json_pretty<T>(obj: &T) -> Outcome<String>
where
    T: Serialize,
{
    let json: String = serde_json::to_string_pretty(obj).catch(
        EXN::SerializationException,
        &format!(
            "Failed to convert object of type `{}` to String",
            std::any::type_name::<T>()
        ),
    )?;
    Ok(json)
}

/// Convert a json string to an object.
pub fn json_to_obj<'a, T>(text: &'a str) -> Outcome<T>
where
    T: Deserialize<'a>,
{
    let obj: T = serde_json::from_str(text).catch(
        EXN::DeserializationException,
        &format!(
            "Failed to convert string to object of type `{}`",
            std::any::type_name::<T>()
        ),
    )?;
    Ok(obj)
}

/// Convert a `serde_json::Value` to an object.
pub fn jval_to_obj<T>(val: serde_json::value::Value) -> Outcome<T>
where
    T: DeserializeOwned,
{
    let obj: T = serde_json::from_value(val).catch(
        EXN::DeserializationException,
        &format!(
            "jval_to_obj failed to convert serde_json::Value to object of type `{}`",
            std::any::type_name::<T>()
        ),
    )?;
    Ok(obj)
}

pub trait JsonDictGet {
    fn get_must_provide<V>(&self, field: &str) -> Outcome<V>
    where
        V: DeserializeOwned;
    fn get_with_default<V>(&self, field: &str, default: V) -> Outcome<V>
    where
        V: DeserializeOwned;
    fn jcatch_(&self) -> Outcome<JsonValue>;
}

impl JsonDictGet for JsonDict {
    fn get_must_provide<V>(&self, field: &str) -> Outcome<V>
    where
        V: DeserializeOwned,
    {
        match self.get(field) {
            Some(jval) => {
                let val: V = jval_to_obj(jval.clone()).catch(
                    "JsonInvalidFieldException",
                    &format!(
                        "The provided JSON field \"{}\" cannot be parsed into type `{}`",
                        field,
                        std::any::type_name::<V>()
                    ),
                )?;
                Ok(val)
            }
            None => {
                throw!(
                    "JsonNoRequiredFieldException",
                    &format!("The required JSON field \"{}\" is absent", field)
                );
            }
        }
    }

    fn get_with_default<V>(&self, field: &str, default: V) -> Outcome<V>
    where
        V: DeserializeOwned,
    {
        match self.get(field) {
            Some(jval) => {
                let val: V = jval_to_obj(jval.clone()).catch(
                    "JsonInvalidFieldException",
                    &format!(
                        "The provided JSON field \"{}\" cannot be parsed into type `{}`",
                        field,
                        std::any::type_name::<V>()
                    ),
                )?;
                Ok(val)
            }
            None => Ok(default),
        }
    }

    fn jcatch_(&self) -> Outcome<JsonValue> {
        let status: String = self.get_must_provide("status")
            .catch(
                "DataFormatException", 
                "If call `catch(...)` or `catch_()` on a JsonDict object, the `status` field must exist."
            )?;
        if status == "ok" {
            let mut ret: JsonValue = JsonValue::Null;
            let special_keys = ["__array", "__bool", "__string", "__number"];
            let mut has_special_key = false;
            for special_key in special_keys {
                if self.contains_key(special_key) {
                    if !has_special_key {
                        has_special_key = true;
                        ret = self.get(special_key).unwrap().clone();
                    } else {
                        throw!(
                            "DataFormatException",
                            &format!(
                                r#"If call `catch(...)` or `catch_()` on a JsonDict object,
                                the object should have at most 1 field within the following list:
                                ["__array", "__bool", "__string", "__number"]."#
                            )
                        );
                    }
                }
            }
            // if self is a dictionary with 0 or more key-value pairs (except "status").
            if !has_special_key {
                let mut dict = self.clone();
                dict.remove("status");
                if dict.len() != 0 {
                    ret = JsonValue::Object(dict);
                }
            }
            return Ok(ret);
        } else if status == "err" {
            let trace: String = self.get_must_provide("trace").catch_()?;
            return Err(trace).catch_()?;
        } else {
            throw!(
                "DataFormatException",
                r#"If call `catch(...)` or `catch_()` on a JsonDict object,
                the `status` field must be either "ok" or "err"."#
            );
        }
    }
}

pub trait StringToJsonDict {
    fn try_into_json_dict(&self) -> Outcome<JsonDict>;
}

impl<T: AsRef<str>> StringToJsonDict for T {
    fn try_into_json_dict(&self) -> Outcome<JsonDict> {
        let text = self.as_ref();
        let jd: JsonDict = json_to_obj(text).catch_()?;
        Ok(jd)
    }
}
