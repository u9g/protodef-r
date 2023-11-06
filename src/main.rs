use std::collections::HashMap;

use anyhow::Context;
use serde::Deserialize;

mod type_de;

#[derive(Debug)]
enum Type {
    #[allow(clippy::enum_variant_names)]
    TypeReference(String),
    PString(PString),
    Array(Array),
    Container(Container),
    Mapper(Mapper),
    Switch(Switch),
    BitField(BitField),
    Option(OptionType),
    Buffer(Buffer),

    EntityMetadataLoop(EntityMetadataLoop),
    EntityMetadataItem(EntityMetadataItem),
    ParticleData(ParticleData),
}

#[derive(Debug, Deserialize)]
struct PString {
    /// Field name: count or count_type
    #[serde(alias = "countType")]
    count: Count,
    encoding: Option<String>,
}

#[derive(Debug, Deserialize)]
struct EntityMetadataLoop {
    #[serde(alias = "endVal")]
    end_val: u32,
    #[serde(alias = "type")]
    _type: Box<Type>,
}

#[derive(Debug, Deserialize)]
struct EntityMetadataItem {
    #[serde(alias = "compareTo")]
    compare_to: String,
}

#[derive(Debug, Deserialize)]
struct Buffer {
    #[serde(alias = "countType")]
    count: Count,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Count {
    Number(u32),
    Type(Box<Type>),
}

#[derive(Debug, Deserialize)]
struct ParticleData {
    #[serde(alias = "compareTo")]
    compare_to: String,
}

#[derive(Debug, Deserialize)]
struct OptionType {
    optional_type: Box<Type>,
}

#[derive(Debug, Deserialize)]
struct Array {
    #[serde(alias = "countType")]
    count: Count,
    #[serde(alias = "type")]
    element_type: Box<Type>,
}

#[derive(Debug)]
struct BitField {
    fields: Vec<FieldOfBitField>,
}

#[derive(Debug, Deserialize)]
struct FieldOfBitField {
    name: String,
    size: u32,
    signed: bool,
}

#[derive(Debug, Deserialize)]
struct Switch {
    #[serde(alias = "compareTo")]
    compare_to: Box<Type>,
    fields: HashMap<String, Type>,
    default: Option<Box<Type>>,
}

#[derive(Default, Debug)]
struct Container {
    field_to_type: Vec<ContainerField>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ContainerField {
    Named {
        name: String,
        #[serde(alias = "type")]
        field_type: Type,
    },
    Anon {
        anon: bool,
        #[serde(alias = "type")]
        field_type: Type,
    },
}

#[derive(Deserialize, Default, Debug)]
struct Mapper {
    #[serde(alias = "type")]
    key_type: String,
    #[serde(alias = "mappings")]
    key_to_type: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
struct Protocol {
    types: HashMap<String, Type>,
    // handshaking: HashMap<String, TypesHolder>,
}

#[derive(Deserialize, Debug)]
struct TypesHolder {
    types: HashMap<String, Type>,
}

fn main() -> anyhow::Result<()> {
    let nbt_json = std::fs::read_to_string("./nbt.json").context("read nbt.json")?;
    let deserializer = &mut serde_json::Deserializer::from_str(&nbt_json);
    let v: Result<HashMap<String, Type>, _> = serde_path_to_error::deserialize(deserializer);

    // match v {
    //     Ok(types) => println!("{types:#?}"),
    //     Err(e) => {
    //         println!("{e}");
    //     }
    // }

    let protocol_json = std::fs::read_to_string("./protocol.json").context("read protocol.json")?;
    let deserializer = &mut serde_json::Deserializer::from_str(&protocol_json);
    let v: Result<Protocol, _> = serde_path_to_error::deserialize(deserializer);

    match v {
        Ok(types) => println!("{types:#?}"),
        Err(e) => {
            println!("{e}");
        }
    }

    Ok(())
}
