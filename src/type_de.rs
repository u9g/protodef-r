use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

use crate::{
    Array, Buffer, ContainerField, EntityMetadataItem, EntityMetadataLoop, FieldOfBitField, Mapper,
    PString, ParticleData, Switch, TopBitSetTerminatedArray, Type,
};

struct TypeVisitor;

impl<'de> Visitor<'de> for TypeVisitor {
    type Value = Type;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a protodef type")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Type::TypeReference(value.to_string()))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let first_element: String = seq.next_element()?.ok_or_else(|| {
            de::Error::custom("no string as first argument in seq when looking for function name")
        })?;

        match first_element.as_str() {
            "pstring" => {
                let args: PString = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom(
                        "didn't get expected arguments for second list argument for pstring function"
                    )
                })?;

                Ok(Type::PString(args))
            }
            "array" => {
                let args: Array = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom(
                        "didn't get expected arguments for second list argument for array function",
                    )
                })?;

                Ok(Type::Array(args))
            }
            "container" => {
                let args: Vec<ContainerField> = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom(
                        "didn't get expected arguments for second list argument for container function",
                    )
                })?;

                Ok(Type::Container(crate::Container {
                    field_to_type: args,
                }))
            }
            "mapper" => {
                let args: Mapper = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom(
                        "didn't get expected arguments for second list argument for mapper function",
                    )
                })?;

                Ok(Type::Mapper(args))
            }
            "switch" => {
                let args: Switch = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom(
                        "didn't get expected arguments for second list argument for switch function",
                    )
                })?;

                Ok(Type::Switch(args))
            }
            "bitfield" => {
                let args: Vec<FieldOfBitField> = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom(
                        "didn't get expected arguments for second list argument for bitfield function",
                    )
                })?;

                Ok(Type::BitField(crate::BitField { fields: args }))
            }
            "option" => {
                let t: Type = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom(
                        "didn't get expected arguments for second list argument for option function",
                    )
                })?;

                Ok(Type::Option(crate::OptionType {
                    optional_type: Box::new(t),
                }))
            }
            "buffer" => {
                let buffer: Buffer = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom(
                        "didn't get expected arguments for second list argument for buffer function",
                    )
                })?;

                Ok(Type::Buffer(buffer))
            }

            "entityMetadataLoop" => {
                let metadata_loop: EntityMetadataLoop = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom(
                        "didn't get expected arguments for second list argument for entityMetadataLoop function",
                    )
                })?;

                Ok(Type::EntityMetadataLoop(metadata_loop))
            }
            "entityMetadataItem" => {
                let metadata_item: EntityMetadataItem = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom(
                        "didn't get expected arguments for second list argument for entityMetadataItem function",
                    )
                })?;

                Ok(Type::EntityMetadataItem(metadata_item))
            }
            "particleData" => {
                let particle_data: ParticleData = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom(
                        "didn't get expected arguments for second list argument for particleData function",
                    )
                })?;

                Ok(Type::ParticleData(particle_data))
            }
            "topBitSetTerminatedArray" => {
                let top_bit_set_terminated_array: TopBitSetTerminatedArray = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom(
                        "didn't get expected arguments for second list argument for topBitSetTerminatedArray function",
                    )
                })?;

                Ok(Type::TopBitSetTerminatedArray(top_bit_set_terminated_array))
            }

            _ => Err(de::Error::custom(format!(
                "unexpected function call: {first_element:?}"
            ))),
        }
    }
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Type, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(TypeVisitor)
    }
}
