use serde::Deserialize;

use crate::{
    common::enumerations::BinaryTypeEnumeration, errors::Error, records::AdditionalTypeInfo,
};

use super::BinaryArrayTypeEnumeration;

/// [`BinaryArray`] is the most general form of Array records. The record is more verbose than the other
/// Array records.
#[derive(Debug)]
pub struct BinaryArray {
    pub object_id: i32,
    pub binary_array_type_enumeration: BinaryArrayTypeEnumeration,
    pub rank: i32,
    pub lengths: Vec<i32>,
    pub lower_bounds: Option<Vec<i32>>,
    pub type_enum: BinaryTypeEnumeration,
    pub additional_type_info: AdditionalTypeInfo,
}

// impl BinaryArray {
//     pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
//         let object_id = read_bytes(reader)?;

//         let binary_array_type_enumeration: BinaryArrayTypeEnumeration = from_reader(reader)?;

//         let rank = read_bytes(reader)?;

//         let lengths = vec![0i32; rank as usize]
//             .into_iter()
//             .map(|_| read_bytes(reader))
//             .collect::<Result<Vec<i32>, Error>>()?;

//         let lower_bounds = match binary_array_type_enumeration {
//             BinaryArrayTypeEnumeration::SingleOffset
//             | BinaryArrayTypeEnumeration::JaggedOffset
//             | BinaryArrayTypeEnumeration::RectangularOffset => {
//                 let lower_bounds = vec![0i32; rank as usize]
//                     .into_iter()
//                     .map(|_| read_bytes(reader))
//                     .collect::<Result<Vec<i32>, Error>>()?;
//                 Some(lower_bounds)
//             }
//             _ => None,
//         };

//         let type_enum =
//             BinaryTypeEnumeration::from_repr(read_bytes(reader)?).ok_or(Error::InvalidEnum)?;

//         let additional_type_info = AdditionalTypeInfo::try_from((reader, &type_enum))?;

//         Ok(BinaryArray {
//             object_id,
//             binary_array_type_enumeration,
//             rank,
//             lengths,
//             lower_bounds,
//             type_enum,
//             additional_type_info,
//         })
//     }
// }

// region: BinaryArray Deserialization
impl<'de> Deserialize<'de> for BinaryArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // struct BinaryArrayVisitor;

        // impl<'de> Visitor<'de> for BinaryArrayVisitor {
        //     type Value = BinaryArray;

        //     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        //         formatter.write_str("struct BinaryArray")
        //     }

        //     fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        //     where
        //         A: serde::de::SeqAccess<'de>,
        //     {
        //         let object_id = seq
        //             .next_element()?
        //             .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        //         let binary_array_type_enumeration = seq
        //             .next_element()?
        //             .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
        //         let rank = seq
        //             .next_element()?
        //             .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;

        //         let lengths = vec![0i32; rank as usize]
        //             .into_iter()
        //             .map(|_| {
        //                 seq.next_element()?
        //                     .ok_or_else(|| serde::de::Error::invalid_length(3, &self))
        //             })
        //             .collect()?;

        //         let lower_bounds = match binary_array_type_enumeration {
        //             BinaryArrayTypeEnumeration::SingleOffset
        //             | BinaryArrayTypeEnumeration::JaggedOffset
        //             | BinaryArrayTypeEnumeration::RectangularOffset => {
        //                 let lower_bounds = vec![0i32; rank as usize]
        //                     .into_iter()
        //                     .map(|_| {
        //                         seq.next_element()?
        //                             .ok_or_else(|| serde::de::Error::invalid_length(4, &self))
        //                     })
        //                     .collect()?;
        //                 Some(lower_bounds)
        //             }
        //             _ => None,
        //         };

        //         let type_enum = seq
        //             .next_element()?
        //             .ok_or_else(|| serde::de::Error::invalid_length(5, &self))?;

        //         let additional_type_info = AdditionalTypeInfo::try_from((reader, &type_enum))?;

        //         Ok(BinaryArray {
        //             object_id,
        //             binary_array_type_enumeration,
        //             rank,
        //             lengths,
        //             lower_bounds,
        //             type_enum,
        //             additional_type_info,
        //         })
        //     }
        // }

        // const FIELDS: &[&str] = &[
        //     "object_id",
        //     "binary_array_type_enumeration",
        //     "rank",
        //     "lengths",
        //     "lower_bounds",
        //     "type_enum",
        //     "additional_type_info",
        // ];
        // deserializer.deserialize_struct("BinaryArray", FIELDS, BinaryArrayVisitor)

        let object_id = i32::deserialize(deserializer)?;

        let binary_array_type_enumeration = BinaryArrayTypeEnumeration::deserialize(deserializer)?;

        let rank = i32::deserialize(deserializer)?;

        let lengths = vec![0i32; rank as usize]
            .into_iter()
            .map(|_| i32::deserialize(deserializer))
            .collect()
            .map_err(|_| serde::de::Error::custom("size of lengths doesn't match rank"));

        let lower_bounds = match binary_array_type_enumeration {
            BinaryArrayTypeEnumeration::SingleOffset
            | BinaryArrayTypeEnumeration::JaggedOffset
            | BinaryArrayTypeEnumeration::RectangularOffset => {
                let lower_bounds = vec![0i32; rank as usize]
                    .into_iter()
                    .map(|_| i32::deserialize(deserializer))
                    .collect()?;
                Some(lower_bounds)
            }
            _ => None,
        };

        let type_enum = BinaryTypeEnumeration::deserialize(deserializer)?;

        let additional_type_info = AdditionalTypeInfo::try_from((deserializer, &type_enum))?;

        Ok(BinaryArray {
            object_id,
            binary_array_type_enumeration,
            rank,
            lengths,
            lower_bounds,
            type_enum,
            additional_type_info,
        })
    }
}
// endregion
