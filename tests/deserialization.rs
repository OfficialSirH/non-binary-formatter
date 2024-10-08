use nonbinary_formatter::{
    common::enumerations::RecordTypeEnumeration, deserializer::from_reader, errors::Error,
    records::class::ClassWithMembersAndTypes,
};

#[test]
// TODO: fix deserialization errors within this
#[ignore = "This is a very old test that I'll need to adjust later to work with current refactor"]
fn test_classwithmembertypes_deserialization() -> Result<(), Error> {
    let data = [
        0x05, // RecordTypeEnum: ClassWithMembersAndTypes
        0x02, 0x00, 0x00, 0x00,       // ObjectId: 2
        0b00011010, // Name length: 26
        b'D', b'O', b'J', b'R', b'e', b'm', b'o', b't', b'i', b'n', b'g', b'M', b'e', b't', b'a',
        b'd', b'a', b't', b'a', b'.', b'M', b'y', b'D', b'a', b't',
        b'a', // Name: "DOJRemotingMetadata.MyData"
        0x04, 0x00, 0x00, 0x00, // NumMembers: 4
        0b0110, b'S', b't', b'r', b'e', b'e', b't', // MemberNames[0]: "Street"
        0b0100, b'C', b'i', b't', b'y', // MemberNames[1]: "City"
        0b0101, b'S', b't', b'a', b't', b'e', // MemberNames[2]: "State"
        0b0011, b'Z', b'i', b'p', // MemberNames[3]: "Zip"
        0x01, 0x01, 0x01, 0x01, // BinaryTypeEnums: all strings
        0x03, 0x00, 0x00, 0x00, // LibraryId: 3
        0x06, // RecordTypeEnum: BinaryObjectString
        0x04, 0x00, 0x00, 0x00, // ObjectId: 4
        0x11, 0x00, 0x00, 0x00, // Value length: 17
        b'O', b'n', b'e', b' ', b'M', b'i', b'c', b'r', b'o', b's', b'o', b'f', b't', b' ', b'W',
        b'a', b'y', // Value: "One Microsoft Way"
        0x06, // RecordTypeEnum: BinaryObjectString
        0x05, 0x00, 0x00, 0x00, // ObjectId: 5
        0x07, 0x00, 0x00, 0x00, // Value length: 7
        b'R', b'e', b'd', b'm', b'o', b'n', b'd', // Value: "Remond"
        0x06, // RecordTypeEnum: BinaryObjectString
        0x06, 0x00, 0x00, 0x00, // ObjectId: 6
        0x02, 0x00, 0x00, 0x00, // Value length: 2
        b'W', b'A', // Value: "WA"
        0x06, // RecordTypeEnum: BinaryObjectString
        0x07, 0x00, 0x00, 0x00, // ObjectId: 7
        0x05, 0x00, 0x00, 0x00, // Value length: 5
        b'9', b'8', b'0', b'5', b'4', // Value: "98054"
    ];

    let mut cursor = std::io::Cursor::new(data);
    let record_type: RecordTypeEnumeration = from_reader(&mut cursor)?;
    assert_eq!(RecordTypeEnumeration::ClassWithMembersAndTypes, record_type);
    let class = ClassWithMembersAndTypes::deserialize(&mut cursor)?;

    println!("{:?}", class);

    assert_eq!("ClassWithMembersAndTypes { object_id: 2, name: \"DOJRemotingMetadata.MyData\", member_names: [\"Street\", \"City\", \"State\", \"Zip\"], member_types: [String, String, String, String], library_id: 3, member_values: [String(BinaryObjectString { object_id: 4, value: \"One Microsoft Way\" }), String(BinaryObjectString { object_id: 5, value: \"Redmond\" }), String(BinaryObjectString { object_id: 6, value: \"WA\" }), String(BinaryObjectString { object_id: 7, value: \"98054\" })] }", 
        format!("{:?}", class));

    Ok(())
}
