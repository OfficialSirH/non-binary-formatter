# [MS-NRBF]:</br>.NET Remoting: Binary Format Data Structure

# Table of Contents

- **[1 Introduction](#1-introduction)**
  - **[1.1 Glossary](#11-glossary)**
  - **[1.2 References](#12-references)**
    - **[1.2.1 Normative References](#121-normative-references)**
    - **[1.2.2 Informative References](#122-informative-references)**
  - **[1.3 Overview](#13-overview)**
  - **[1.4 Relationship to Protocols and Other Structures](#14-relationship-to-protocols-and-other-structures)**
  - **[1.5 Applicability Statement](#15-applicability-statement)**
  - **[1.6 Versioning and Localization](#16-versioning-and-localization)**
  - **[1.7 Vendor-Extensible Fields](#17-vendor-extensible-fields)**
- **[2 Structures](#2-structures)**
  - **[2.1 Common Definitions](#21-common-definitions)**
    - **[2.1.1 Common Data Types](#211-common-data-types)**
      - **[2.1.1.1 Char](#2111-char)**
      - **[2.1.1.2 Double](#2112-double)**
      - **[2.1.1.3 Single](#2113-single)**
      - **[2.1.1.4 TimeSpan](#2114-timespan)**
      - **[2.1.1.5 DateTime](#2115-datetime)**
      - **[2.1.1.6 LengthPrefixedString](#2116-lengthprefixedstring)**
      - **[2.1.1.7 Decimal](#2117-decimal)**
      - **[2.1.1.8 ClassTypeInfo](#2118-classtypeinfo)**
    - **[2.1.2 Enumerations](#212-enumerations)**
      - **[2.1.2.1 RecordTypeEnumeration](#2121-recordtypeenumeration)**
      - **[2.1.2.2 BinaryTypeEnumeration](#2122-binarytypeenumeration)**
      - **[2.1.2.3 PrimitiveTypeEnumeration](#2123-primitivetypeenumeration)**
  - **[2.2 Method Invocation Records](#22-method-invocation-records)**
    - **[2.2.1 Enumerations](#221-enumerations)**
      - **[2.2.1.1 MessageFlags](#2211-messageflags)**
    - **[2.2.2 Common Structures](#222-common-structures)**
      - **[2.2.2.1 ValueWithCode](#2221-valuewithcode)**
      - **[2.2.2.2 StringValueWithCode](#2222-stringvaluewithcode)**
      - **[2.2.2.3 ArrayOfValueWithCode](#2223-arrayofvaluewithcode)**
    - **[2.2.3 Record Definitions](#223-record-definitions)**
      - **[2.2.3.1 BinaryMethodCall](#2231-binarymethodcall)**
      - **[2.2.3.2 MethodCallArray](#2232-methodcallarray)**
      - **[2.2.3.3 BinaryMethodReturn](#2233-binarymethodreturn)**
      - **[2.2.3.4 MethodReturnCallArray](#2234-methodreturncallarray)**
  - **[2.3 Class Records](#23-class-records)**
    - **[2.3.1 Common Structures](#231-common-structures)**
      - **[2.3.1.1 ClassInfo](#2311-classinfo)**
      - **[2.3.1.2 MemberTypeInfo](#2312-membertypeinfo)**
    - **[2.3.2 Record Definitions](#232-record-definitions)**
      - **[2.3.2.1 ClassWithMembersAndTypes](#2321-classwithmembersandtypes)**
      - **[2.3.2.2 ClassWithMembers](#2322-classwithmembers)**
      - **[2.3.2.3 SystemClassWithMembersAndTypes](#2323-systemclasswithmembersandtypes)**
      - **[2.3.2.4 SystemClassWithMembers](#2324-systemclasswithmembers)**
      - **[2.3.2.5 ClassWithId](#2325-classwithid)**
  - **[2.4 Array Records](#24-array-records)**
    - **[2.4.1 Enumerations](#241-enumerations)**
      - **[2.4.1.1 BinaryArrayTypeEnumeration](#2411-binaryarraytypeenumeration)**
    - **[2.4.2 Common Definitions](#242-common-definitions)**
      - **[2.4.2.1 ArrayInfo](#2421-arrayinfo)**
    - **[2.4.3 Record Definitions](#243-record-definitions)**
      - **[2.4.3.1 BinaryArray](#2431-binaryarray)**
      - **[2.4.3.2 ArraySingleObject](#2432-arraysingleobject)**
      - **[2.4.3.3 ArraySinglePrimitive](#2433-arraysingleprimitive)**
      - **[2.4.3.4 ArraySingleString](#2434-arraysinglestring)**
  - **[2.5 Member Reference Records](#25-member-reference-records)**
    - **[2.5.1 MemberPrimitiveTyped](#251-memberprimitivetyped)**
    - **[2.5.2 MemberPrimitiveUnTyped](#252-memberprimitiveuntyped)**
    - **[2.5.3 MemberReference](#253-memberreference)**
    - **[2.5.4 ObjectNull](#254-objectnull)**
    - **[2.5.5 ObjectNullMultiple](#255-objectnullmultiple)**
    - **[2.5.6 ObjectNullMultiple256](#256-objectnullmultiple256)**
    - **[2.5.7 BinaryObjectString](#257-binaryobjectstring)**
  - **[2.6 Other Records](#26-other-records)**
    - **[2.6.1 SerializationHeaderRecord](#261-serializationheaderrecord)**
    - **[2.6.2 BinaryLibrary](#262-binarylibrary)**
    - **[2.6.3 MessageEnd](#263-messageend)**
  - **[2.7 Binary Record Grammar](#27-binary-record-grammar)**
- **[3 Structure Examples](#3-structure-examples)**
- **[4 Security Considerations](#4-security-considerations)**
- **[5 Appendix A: Product Behavior](#5-appendix-a-product-behavior)**
- **[6 Change Tracking](#6-change-tracking)**
- **[7 Index](#7-index)**

# 1 Introduction

The .NET Remoting: Binary Format Data Structure defines a set of structures that represent **[object graph](#object-graph)** or method invocation information as an octet stream. One possible application of the structure is as the **[serialization format](#serialization-format)** for the data model as specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.1. Sections 1.7 and 2 of this specification are normative. All other sections and examples in this specification are informative.

## 1.1 Glossary

This document uses the following terms:

### **argument**:
> A named **[Data Value](#data-value)** that is passed as part of a **[Remote Method](#remote-method)** invocation or returned as part of the results of a **[Remote Method](#remote-method)** invocation. For more information about Remote Method invocation, see [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.1.

### **array**:
> A **[Remoting Type](#remoting-type)** that is an ordered collection of values. The values are identified by their position and position is determined by a set of integer indices. The number of indices required to represent the position is called the Rank of the **[Array](#array)**. An **[Array](#array)** is part of the **[Remoting Data Model](#remoting-data-model)** and also specifies the **[Remoting Type](#remoting-type)** of its items. For more information, [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.1.

### **Call Context**:
> A mechanism to pass data that is not part of the method **[Arguments](#argument)** between client and server. It is a collection of name-value pairs that is carried with the execution of a **[Remote Method](#remote-method)**. This collection is sent along with other method **[Arguments](#argument)** from client to server, and is transmitted back, along with the **[Return Values](#return-value)** and output **[Arguments](#argument)**, from the server to the client. For more information, see [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 1.3.

### **class**:
> (1) A **[Remoting Type](#remoting-type)** that encapsulates a set of named values and a set of methods that operate on those values. The named values are called Members of the Class. A Class is part of the **[Remoting Data Model](#remoting-data-model)**. For more information, see [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.1.

> (2) See **[System.Object](#systemobject)**.

### **Class Metadata**:
> Information about a Class that includes the Class name, its Library name, and the names and Remoting Types of its Members.

### **Coordinated Universal Time (UTC)**:
> A high-precision atomic time standard that approximately tracks Universal Time (UT). It is the basis for legal, civil time all over the Earth. Time zones around the world are expressed as positive and negative offsets from UTC. In this role, it is also referred to as Zulu time (Z) and Greenwich Mean Time (GMT). In these specifications, all references to UTC refer to the time at UTC-0 (or GMT).

### **data value**:
> An instance of a **[Remoting Type](#remoting-type)**, which may be a **[Class](#class)**, **[Array](#array)**, **[Enum](#enum)**, or Primitive. A **[Data Value](#data-value)** is part of the **[Remoting Data Model](#remoting-data-model)**. For more information, see [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.1.

### **deserialize**:
> See unmarshal.

### **Enum**:
> A Primitive type whose members are constrained to a set of values. The Primitive type is considered to be an underlying **[Remoting Type](#remoting-type)** of the **[Enum](#enum)**. Each value has a name associated with it. An **[Enum](#enum)** is part of the **[Remoting Data Model](#remoting-data-model)**, and an abbreviation for "enumeration." For more information, see [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.1.

### **Exception**:
> A **[Class](#class)** that indicates an error in the execution of a **[Remote Method](#remote-method)**. It is sent as part of the return message from a server to a client. An **[Exception](#exception)** contains a human-readable message that indicates what the error is, and can also have additional data to identify the error. An **[Exception](#exception)** is part of the **[Remoting Data Model](#remoting-data-model)**. For more information, see [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.1.

### **Generic Argument**:
> A formal argument used in a **[Generic Type](#generic-type)** or a **[Generic Remote Method](#generic-remote-method)** to represent a parameterized **[Remoting Type](#remoting-type)**. **[Generic Arguments](#generic-argument)** can be referenced in the **[Class](#class)** or the method as opaque **[Remoting Types](#remoting-type)**. They are replaced by the actual types when the **[Class](#class)** or the method is used. For more information, see Generic Type and Methods in [[ECMA-335]](https://ecma-international.org/publications-and-standards/standards/ecma-335/).

### **Generic Remote Method**:
> A **[Remote Method](#remote-method)** that is parameterized by one or more **[Remoting Types](#remoting-type)**. The method caller must provide the actual **[Remoting Types](#remoting-type)** (in addition to the **[Input Arguments](#input-argument)**). For more information, see [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.1.

### **Generic Type**:
> A **[Class](#class)**, **[Server Type](#server-type)**, or Server Interface that is parameterized by one or more **[Remoting Types](#remoting-type)**. A **[Generic Type](#generic-type)** contains GenericArguments as a placeholder for the parameterized **[Remoting Types](#remoting-type)**. A **[Generic Type](#generic-type)** cannot have any instances. For more information, see Generic Types and Methods in [[ECMA-335]](https://ecma-international.org/publications-and-standards/standards/ecma-335/).

### **Input Argument**:
> A named **[Data Value](#data-value)** that is passed as part of a **[Remote Method](#remote-method)** invocation from the client to the server. For more information, see **[Remote Method](#remote-method)** in the Abstract Data Model (section 3.1.1).

### **Library**:
> Part of the **[Remoting Data Model](#remoting-data-model)**. A **[Library](#library)** is a named unit that contains a collection of **[Remoting Types](#remoting-type)**. For more information, see Library in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.1.

### **little-endian**:
> Multiple-byte values that are byte-ordered with the least significant byte stored in the memory location with the lowest address.

### **local time zone**:
> The time zone in which the computer running the implementation is configured.

### **logical call ID**:
> An optional string value that identifies the logical thread of execution. This value is passed as part of the **[Call Context](#call-context)** and can be used in implementation-specific local threading models on the server.

### **member**:
> See **[Class](#class)**.

### **message content**:
> The **[serialized](#serialize)** body of a message.

### **Message Properties**:
> A collection of implementation-specific, name-value pairs that are transmitted as part of a **[Remote Method](#remote-method)** invocation. **[Message Properties](#message-properties)** are used to exchange implementation-specific data between clients and servers.

### **method signature**:
> A list of the remoting types of the arguments of a remote method.

### **Null Object**:
> Part of the **[Remoting Data Model](#remoting-data-model)**. **[Null Object](#null-object)** is a special value that can be used in place of an instance of a Class, **[Array](#array)**, or String. It indicates that no instance is being specified. For more information, see [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.1.

### **object graph**:
> In object-oriented programming, groups of interrelated objects that form a network through often complex relationships with each other are known as an object graph. In an object graph, objects can be linked to each other by a specific object, by owning or containing another object, or by holding a reference to another object. Such an abstract structure can be used to represent the state of an application.

### **Output Argument**:
> A named **[Data Value](#data-value)** that is returned as part of the results of a **[Remote Method](#remote-method)** invocation. For more information, see **[Remote Method](#remote-method)** in Abstract Data Model (section 3.1.1).

### **Primitive Type**:
> Part of the **[Remoting Data Model](#remoting-data-model)**. **[Primitive Types](#primitive-type)** are predefined **[Remoting Types](#remoting-type)** such as Byte, Int16, Int32, Int64, and so on. For more information, see [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.1

### **Primitive Value**:
> Part of the **[Remoting Data Model](#remoting-data-model)**. A **[Primitive Value](#primitive-value)** is an instance of a **[Primitive Type](#primitive-type)**.

### **record**:
> A variable-length sequence of bytes with a predefined structure.

### **Remote Method**:
> Part of the **[Remoting Data Model](#remoting-data-model)**. A **[Remote Method](#remote-method)** is a remotely callable operation. A **[Remote Method](#remote-method)** can either be One-Way or Two-Way. In the case of a One-Way Method, there is no reply from the implementation. For more information, see [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.1

### **Remoting Data Model**:
> A model that is used to represent higher-layer–defined data structures and values, and to represent a **[Remote Method](#remote-method)** invocation and the **[Return Value](#return-value)** or error information from that invocation. A protocol, such as [[MS-NRLS]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrls), that is built on top of this protocol can be defined by using the **[Remoting Data Model](#remoting-data-model)**, and can be agnostic to the **[serialization format](#serialization-format)**. For more information, see Abstract Data Model (section 3.1.1).

### **Remoting Type**:
> Part of the **[Remoting Data Model](#remoting-data-model)**. **[Class](#class)**, **[Array](#array)**, **[Enum](#enum)**, and Primitive are different kinds of **[Remoting Types](#remoting-type)**. All **[Remoting Types](#remoting-type)** are identified by a name that is case sensitive. For more information, see [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.1 

### **Return Value**:
> A **[Data Value](#data-value)** that is returned as part of the results of a **[Remote Method](#remote-method)** invocation. For more information, see **[Remote Method](#remote-method)** in Abstract Data Model (section 3.1.1).

### **serialization**:
> A mechanism by which an application converts an object into an XML representation.

### **Serialization Format**:
> The structure of the serialized message content, which can be either binary or SOAP. Binary serialization format is specified in [[MS-NRBF]](#ms-nrbfnet-remoting-binary-format-data-structure). SOAP serialization format is specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp).

### **Serialization Stream**:
> An octet stream that contains a sequence of records defined in this document.

### **serialize**:
> The process of taking an in-memory data structure, flat or otherwise, and turning it into a flat stream of bytes. See also marshal.

### **Server Type**:
> Part of the **[Remoting Data Model](#remoting-data-model)**. A **[Server Type](#server-type)** contains **[Remote Methods](#remote-method)**.

### **System Library**:
> A specially designated **[library](#library)** that can be used to reduce the wire size for commonly used data types. The name of the **[library](#library)** is agreed to by both the server and the client.

### **System.Object**:
> Part of the **[Remoting Data Model](#remoting-data-model)**. **[System.Object](#systemobject)** is a **[Class](#class)** that has no **[Members](#member)**. A **[Class](#class)** that does not extend another **[Class](#class)** is considered to extend **[System.Object](#systemobject)**.

### **Ultimate Array Item Type**:
> The Item Type of the innermost Array in a recursive construction of Array of Arrays. For instance, an "Array of TypeA" has an Ultimate Array Item Type of TypeA. An "Array of Array of TypeA" also has an Ultimate Array Item Type of TypeA, as does an "Array of Array of Array of TypeA".

### **Unicode**:
> A character encoding standard developed by the Unicode Consortium that represents almost all of the written languages of the world. The **[Unicode](#unicode)** standard [[UNICODE5.0.0/2007]](https://www.unicode.org/versions/Unicode5.0.0/) provides three forms (UTF-8, UTF-16, and UTF-32) and seven schemes (UTF-8, UTF-16, UTF-16 BE, UTF-16 LE, UTF-32, UTF-32 LE, and UTF-32 BE).

### **UTF-8**:
> A byte-oriented standard for encoding Unicode characters, defined in the Unicode standard. Unless specified otherwise, this term refers to the UTF-8 encoding form specified in [[UNICODE5.0.0/2007]](https://www.unicode.org/versions/Unicode5.0.0/) section 3.9.

MAY, SHOULD, MUST, SHOULD NOT, MUST NOT: These terms (in all caps) are used as defined in [[RFC2119]](#121-normative-references). All statements of optional behavior use either MAY, SHOULD, or SHOULD NOT.

## 1.2 References

Links to a document in the Microsoft Open Specifications library point to the correct section in the most recently published version of the referenced document. However, because individual documents in the library are not updated at the same time, the section numbers in the documents may not match. You can confirm the correct section numbering by checking the [Errata](https://learn.microsoft.com/en-us/openspecs/main/ms-openspeclp/). 

## 1.2.1 Normative References

- [IEEE754] IEEE, "IEEE Standard for Binary Floating-Point Arithmetic", IEEE 754-1985, October 1985, http://ieeexplore.ieee.org/servlet/opac?punumber=2355
- [MS-DTYP] Microsoft Corporation, "[Windows Data Types](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp)". 
- [MS-NRTP] Microsoft Corporation, "[.NET Remoting: Core Protocol](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp)". 
- [RFC2119] Bradner, S., "Key words for use in RFCs to Indicate Requirement Levels", BCP 14, RFC 2119, March 1997, http://www.rfc-editor.org/rfc/rfc2119.txt 
- [RFC4234] Crocker, D., Ed., and Overell, P., "Augmented BNF for Syntax Specifications: ABNF", RFC 4234, October 2005, http://www.rfc-editor.org/rfc/rfc4234.txt

## 1.2.2 Informative References

- [MS-NETOD] Microsoft Corporation, "[Microsoft .NET Framework Protocols Overview](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-netod/)". 
- [MSDN-.NET-FRAMEWORK] Microsoft Corporation, "Overview of the .NET Framework", http://msdn.microsoft.com/en-us/library/zw4w595w.aspx

## 1.3 Overview

The [.NET Remoting: Binary Format Data Structure](#1-introduction) defines a set of structures that represent **[object graph](#object-graph)** or method invocation information as an octet stream. One possible application of the structure is as the serialization format for the data model as specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.1. 

This specification defines the **[records](#record)** used by this format, and the grammar for writing the records to the **[serialization stream](#serialization-stream)**. 

The format provides structures for mapping instances of data that conform to the **[Remoting Data Model](#remoting-data-model)** into octets. The Remoting Data Model is specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.1. 

The format consists of a sequence of variable-length records. The records are used to hold the **[serialized](#serialize)** instances of **[Classes (2)](#class)**, **[Arrays](#array)**, **[Primitive Types](#primitive-type)**, and method invocations. There are multiple record types to represent each of these instances. The various record types optimize the wire size of the serialized instance. This section specifies the structure of each record in detail. For clarity, the records are grouped as follows:

- Class (2) records contain Class (2) instances. The format allows serialization of **[Class Metadata](#class-metadata)**, in addition to the actual data. Richness of metadata directly contributes to the wire size. The amount of metadata can be reduced by conveying implicit information through special record types and by sharing metadata across records.

- Array records contain Array instances. There is a general record type for Array that can represent multiple dimensions and nonzero lower bound. There are more compact Array records for frequently used Array types such as single-dimensional Array of String, Object, and **[Primitive Values](#primitive-value)**.

- **[Members](#member)** reference records contain **[Data Value](#data-value)** of Class (2) Members or Array items. There are different record types for **[Null Object](#null-object)**, string values, Primitive Type values, and instances of Classes (2) and Arrays.

- Method invocation records contain information about **[Remote Method](#remote-method)**, **[Server Type](#server-type)**, **[Arguments](#argument)**, **[Return Value](#return-value)**, **[Exception](#exception)**, **[Message Properties](#message-properties)**, and **[Call Context](#call-context)**.

- Other records include records that are used to mark the beginning and end of the format.

## 1.4 Relationship To Protocols And Other Structures

This format is part of the .NET Remoting protocols. The .NET Remoting Protocol (as specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp)) uses this format to encode **[message content](#message-content)** before transmission, as specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3. 

The **[serialized](#serialize)** content is transmitted over either HTTP or TCP, by using headers and framing as specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3. The following block diagram illustrates the relationship.

| Figure 1: The .NET Remoting protocols   |
|-----------------------------------------|
![NET_Remote_Protocols](https://github.com/OfficialSirH/non-binary-formatter/blob/main/documentation/NET_Remoting_Protocols.png?raw=true)

## 1.5 Applicability Statement

The .NET Remoting: Binary Format Data Structure can be used as part of a **[Remote Method](#remote-method)** invocation protocol or to persist an **[object graph](#object-graph)**. It has a compact octet stream representation that makes it applicable to wire protocols. Because the format is binary, it is not suitable for cases where the output has to be human readable. The format does not include additional information to aid in error detection or to prevent corruption.

## 1.6 Versioning And Localization

This document covers versioning issues in the following areas:

- Protocol Versions: The Serialization Header record has fields called **MajorVersion** and **MinorVersion** that denote the version of the .NET Remoting: Binary Format Data Structure in use. Because only one version of the .NET Remoting: Binary Format Data Structure has been defined to date, the value of **MajorVersion** is always set to 1 and **MinorVersion** to 0. Future revisions of the format would increment this value. The Serialization Header record is specified in section [2.6.1](#261-serializationheaderrecord).

- Message Versions: MessageFlags (section [2.2.1.1](#2211-messageflags)) defines a flag named "Generic Method". The flag indicates that the method being invoked is a **[Generic Remote Method](#generic-remote-method)**. The flag is valid only in Microsoft .NET Framework 2.0, Microsoft .NET Framework 3.0, Microsoft .NET Framework 3.5, and Microsoft .NET Framework 4.0. For more information, see [[MSDN-.NET-FRAMEWORK]](#122-informative-references). 

There are no localization-dependent structures described in this document.

## 1.7 Vendor-Extensible Fields

This format allows implementation-specific name-value pairs called **[Message Properties](#message-properties)** to be added to the [MethodCallArray (section 2.2.3.2)](#2232-methodcallarray) and [MethodReturnCallArray (section 2.2.3.4)](#2234-methodreturncallarray) records.

___
# 2 Structures

## 2.1 Common Definitions

The following sections specify the common structures and enumerations that are used by all records.

## 2.1.1 Common Data Types

This section specifies the structures of the common **[Remoting Types](#remoting-type)** that are supported by this format. The format supports the following **[Primitive Types](#primitive-type)** as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp).

- BOOLEAN
- BYTE 
- INT8 
- INT16 
- INT32 
- INT64 
- UINT16 
- UINT32 
- UINT64 

The byte-ordering of the multibyte data types is **little-endian**. The signed data types use two's complement to represent the negative numbers.

In addition, this format defines the following common types.

## 2.1.1.1 Char

The Char represents a **[Unicode](#unicode)** character value.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <tr>
        <td align="center" colspan="32">Value (variable)</td>
    </tr>
    <td align="center" colspan="32">...</td>
  </tbody>
</table>

**Value (variable):** UTF-8-encoded bytes.

## 2.1.1.2 Double

The Double represents a 64-bit double-precision floating-point value.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <tr>
        <td align="center" colspan="32">Value</td>
    </tr>
    <td align="center" colspan="32">...</td>
  </tbody>
</table>

**Value (8 bytes):** A 64-bit double-precision floating-point value, as specified in [IEEE754](#121-normative-references).

## 2.1.1.3 Single

The Single represents a 32-bit single-precision floating-point value.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="32">Value</td>
  </tbody>
</table>

**Value (4 bytes):** A 32-bit single-precision floating-point value, as specified in [IEEE754](#121-normative-references).

## 2.1.1.4 Timespan

The TimeSpan represents time duration.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <tr>
        <td align="center" colspan="32">Value</td>
    </tr>
    <td align="center" colspan="32">...</td>
  </tbody>
</table>

**Value (8 bytes):** A 64-bit signed-integer value that specifies duration as the number of 100 nanoseconds. The values range from -10675199 days, 2 hours, 48 minutes, and 05.4775808 seconds to 10675199 days, 2 hours, 48 minutes, and 05.4775807 seconds inclusive.

## 2.1.1.5 Datetime

The DateTime represents an instant of time.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <tr>
        <td align="center" colspan="32">Ticks</td>
    </tr>
    <td align="center" colspan="30">...</td>
    <td align="center" colspan="2">Kind</td>
  </tbody>
</table>

**Ticks (62 bits):** A 62-bit signed-integer value that specifies the number of 100 nanoseconds that have elapsed since 12:00:00, January 1, 0001. The value can represent time instants in a granularity of 100 nanoseconds until 23:59:59.9999999, December 31, 9999.

**Kind (2 bits):** Provides the time-zone information as follows. The value can range from 0 to 2, inclusive[<1>](#5-appendix-a-product-behavior). The following table maps values with the meaning of the **Ticks** field.

| Value   | Meaning                                                                                                     |
|---------|-------------------------------------------------------------------------------------------------------------|
| 0       | Time-zone information is not specified.                                                                     |
| 1       | The time specified is in the [Coordinated Universal Time (UTC)](#coordinated-universal-time-utc) time zone. |
| 2       | The time specified is in the [local time zone](#local-time-zone).                                                               |

## 2.1.1.6 LengthPrefixedString

The LengthPrefixedString represents a string value. The string is prefixed by the length of the **UTF-8** encoded string in bytes. The length is encoded in a variable-length field with a minimum of 1 byte and a maximum of 5 bytes. To minimize the wire size, length is encoded as a variable-length field.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <tr>
        <td align="center" colspan="32">Length (variable)</td>
    </tr>
    <td align="center" colspan="32">...</td>
    <tr>
        <td align="center" colspan="32">String (variable)</td>
    </tr>
    <td align="center" colspan="32">...</td>
  </tbody>
</table>

**Length (variable):** A numerical value that can range from 0 to 2147483647 (2^31) inclusive.

To minimize the wire size, the encoding of the length MUST be encoded as follows:

- The **Length** field MUST be at least 1 byte and MUST NOT be more than 5 bytes.
 
- Each byte MUST hold the **Length** value in its lower 7 bits.
 
- The high bit MUST be used to indicate that the length continues in the next byte.

- In the case that all 5 bytes are used, the high 5 bits in the fifth byte MUST be 0.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="7">Length_0-6</td>
    <td align="center" colspan="1">A</td>
  </tbody>
</table>

**Length_0-6 (7 bits):** Length values range from 0 to 127 (7 bits).

**A - Reserved_7 (1 bit):** The value MUST be 0.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="7">Length_0-6</td>
    <td align="center" colspan="1">A</td>
    <td align="center" colspan="7">Length_8-14</td>
    <td align="center" colspan="1">B</td>
  </tbody>
</table>

**Length_0-6 (7 bits)**: Length values range from 128 to 16383 (14 bits).

**A - Reserved_7 (1 bit):** The value MUST be 1.

**Length_8-14 (7 bits):** Length values range from 128 to 16383 (14 bits).

**B - Reserved_15 (1 bit):** The value MUST be 0.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="7">Length_0-6</td>
    <td align="center" colspan="1">A</td>
    <td align="center" colspan="7">Length_8-14</td>
    <td align="center" colspan="1">B</td>
    <td align="center" colspan="7">Length_16-22</td>
    <td align="center" colspan="1">C</td>
  </tbody>
</table>

**Length_0-6 (7 bits):** Length values range from 16384 to 2097151 (21 bits).

**A - Reserved_7 (1 bit):** The value MUST be 1.

**Length_8-14 (7 bits):** Length values range from 16384 to 2097151 (21 bits).

**B - Reserved_15 (1 bit):** The value MUST be 1.

**Length_16-22 (7 bits):** Length values range from 16384 to 2097151 (21 bits).

**C - Reserved_23 (1 bit):** The value MUST be 0.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="7">Length_0-6</td>
    <td align="center" colspan="1">A</td>
    <td align="center" colspan="7">Length_8-14</td>
    <td align="center" colspan="1">B</td>
    <td align="center" colspan="7">Length_16-22</td>
    <td align="center" colspan="1">C</td>
    <td align="center" colspan="7">Length_24-30</td>
    <td align="center" colspan="1">D</td>
  </tbody>
</table>

**Length_0-6 (7 bits):** Length values range from 2097152 to 268435445 (28 bits).

**A - Reserved_7 (1 bit):** The value MUST be 1.

**Length_8-14 (7 bits):** Length values range from 2097152 to 268435445 (28 bits).

**B - Reserved_15 (1 bit):** The value MUST be 1.

**Length_16-22 (7 bits):** Length values range from 2097152 to 268435445 (28 bits).

**C - Reserved_23 (1 bit):** The value MUST be 1.

**Length_24-30 (7 bits):** Length values range from 2097152 to 268435445 (28 bits).

**D - Reserved_31 (1 bit):** The value MUST be 0.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="7">Length_0-6</td>
    <td align="center" colspan="1">A</td>
    <td align="center" colspan="7">Length_8-14</td>
    <td align="center" colspan="1">B</td>
    <td align="center" colspan="7">Length_16-22</td>
    <td align="center" colspan="1">C</td>
    <td align="center" colspan="7">Length_24-30</td>
    <td align="center" colspan="1">D</td>
    <tr>
        <td align="center" colspan="7">Length_32-38</td>
        <td align="center" colspan="1">E</td>
    </tr>
  </tbody>
</table>

**Length_0-6 (7 bits):** Length values range from 268435456 to 2147483647 (31 bits).

**A - Reserved_7 (1 bit):** The value MUST be 1.

**Length_8-14 (7 bits):** Length values range from 268435456 to 2147483647 (31 bits).

**B - Reserved_15 (1 bit):** The value MUST be 1.

**Length_16-22 (7 bits):** Length values range from 268435456 to 2147483647 (31 bits).

**C - Reserved_23 (1 bit):** The value MUST be 1.

**Length_24-30 (7 bits):** Length values range from 268435456 to 2147483647 (31 bits).

**D - Reserved_31 (1 bit):** The value MUST be 1.

**Length_32-38 (7 bits):** Length values range from 268435456 to 2147483647 (31 bits).

**E - Reserved_39 (1 bit):** The value MUST be 0.

**String (variable):** A UTF-8 encoded string value. The number of bytes of the encoded string MUST be equal to the value specified in the **Length** field.

## 2.1.1.7 Decimal

The Decimal represents a decimal value. It has the following format.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="32">Value (variable)</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
  </tbody>
</table>

**Value (variable):** A [LengthPrefixedString](#2116-lengthprefixedstring) value that is the string representation of the decimal value.

The string MUST be of the following format.

<table border="1">
  <thead>
    <tr>
      <th colspan="3">Formats for decimal value</th>
    </tr>
  </thead>
  <tbody>
    <tr>
        <td>Value</td>
        <td>=</td>
        <td>0*1(MINUS)IntegralPart 0*1(FractionalPart)</td>
    </tr>
    <tr>
        <td>IntegralPart</td>
        <td>=</td>
        <td>1*(DIGIT)</td>
    </tr>
    <tr>
        <td>FractionalPart</td>
        <td>=</td>
        <td>DECIMALPOINT 1*(DIGIT)</td>
    </tr>
    <tr>
        <td>MINUS</td>
        <td>=</td>
        <td>'-'</td>
    </tr>
    <tr>
        <td>DECIMALPOINT</td>
        <td>=</td>
        <td>'.'</td>
    </tr>
  </tbody>
</table>

The decimal value ranges from positive 79,228,162,514,264,337,593,543,950,335 to negative 79,228,162,514,264,337,593,543,950,335 inclusive.

When reading this value, if all of the following are true:

- The string has more than 29 digits, including both the IntegralPart and the FractionalPart.

- The net value is within the decimal value range. The number of digits in the Integral part is less than or equal to 29.

then the decimal value MUST be rounded to the nearest value such that the total number of digits is 29.

## 2.1.1.8 ClassTypeInfo

The ClassTypeInfo identifies a **[Class (2)](#class)** by its name and reference to [BinaryLibrary](#262-binarylibrary) record.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="32">TypeName (variable)</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
    <td align="center" colspan="32">LibraryId</td>
  </tbody>
</table>

**TypeName (variable):** A [LengthPrefixedString](#2116-lengthprefixedstring) value that contains the name of the Class (2). The format of the string is specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 2.2.1.2.

**LibraryId (4 bytes):** An INT32 (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) value that represents the ID that identifies the **[Library](#library)** name. The record that contains this field in a **[serialization stream](#serialization-stream)** MUST be preceded by a BinaryLibrary record that defines the Library name for the ID.

## 2.1.2 Enumerations

## 2.1.2.1 RecordTypeEnumeration

This enumeration identifies the type of the **record**. Each record (except for [MemberPrimitiveUnTyped](#252-memberprimitiveuntyped)) starts with a record type enumeration. The size of the enumeration is one BYTE.

| Constant/value                       | Description                                                                                 |
|--------------------------------------|---------------------------------------------------------------------------------------------|
| SerializationStreamHeader</br>0      | Identifies the [SerializationHeaderRecord](#261-serializationheaderrecord).                 |
| ClassWithId</br>1                    | Identifies a [ClassWithId](#2325-classwithid) record.                                       |
| SystemClassWithMembers</br>2         | Identifies a [SystemClassWithMembers](#2324-systemclasswithmembers) record.                 |
| ClassWithMembers</br>3               | Identifies a [ClassWithMembers](#2322-classwithmembers) record.                             |
| SystemClassWithMembersAndTypes</br>4 | Identifies a [SystemClassWithMembersAndTypes](#2323-systemclasswithmembersandtypes) record. |
| ClassWithMembersAndTypes</br>5       | Identifies a [ClassWithMembersAndTypes](#2321-classwithmembersandtypes) record.             |
| BinaryObjectString</br>6             | Identifies a [BinaryObjectString](#257-binaryobjectstring) record.                          |
| BinaryArray</br>7                    | Identifies a [BinaryArray](#2431-binaryarray) record.                                       |
| MemberPrimitiveTyped</br>8           | Identifies a [MemberPrimitiveTyped](#251-memberprimitivetyped) record.                      |
| MemberReference</br>9                | Identifies a [MemberReference](#253-memberreference) record.                                |
| ObjectNull</br>10                    | Identifies a [ObjectNull](#254-objectnull) record.                                          |
| MessageEnd</br>11                    | Identifies a [MessageEnd](#263-messageend) record.                                          |
| BinaryLibrary</br>12                 | Identifies a [BinaryLibrary](#262-binarylibrary) record.                                    |
| ObjectNullMultiple256</br>13         | Identifies an [ObjectNullMultiple256](#256-objectnullmultiple256) record.                   |
| ObjectNullMultiple</br>14            | Identifies an [ObjectNullMultiple](#255-objectnullmultiple) record.                         |
| ArraySinglePrimitive</br>15          | Identifies an [ArraySinglePrimitive](#2433-arraysingleprimitive).                           |
| ArraySingleObject</br>16             | Identifies an [ArraySingleObject](#2432-arraysingleobject) record.                          |
| ArraySingleString</br>17             | Identifies an [ArraySingleString](#2434-arraysinglestring) record.                          |
| MethodCall</br>21                    | Identifies a [BinaryMethodCall](#2231-binarymethodcall) record.                             |
| MethodReturn</br>22                  | Identifies a [BinaryMethodReturn](#2233-binarymethodreturn) record.                         |

## 2.1.2.2 BinaryTypeEnumeration

The BinaryTypeEnumeration identifies the **[Remoting Type](#remoting-type)** of a **[Class (2) Member](#class)** or an **[Array](#array)** item. The size of the enumeration is a BYTE.

<table border="1">
    <thead>
        <th align="center">Constant/value</th>
        <th align="center">Description</th>
    </thead>
    <tbody>
        <tr>
            <td>Primitive</br>0</td>
            <td>The Remoting Type is defined in [PrimitiveTypeEnumeration](#2123-primitivetypeenumeration) and the Remoting Type is not a string.</td>
        </tr>
        <tr>
            <td>String</br>1</td>
            <td>The Remoting Type is a [LengthPrefixedString](#2116-lengthprefixedstring).</td>
        </tr>
        <tr>
            <td>Object</br>2</td>
            <td>The Remoting Type is [System.Object](#systemobject).</td>
        </tr>
        <tr>
            <td>SystemClass</br>3</td>
            <td>The Remoting Type is one of the following:
                <ul>
                    <li>A Class (2) in the <a href="#system-library"><b>System Library</b></a></li>
                    <li>An Array whose <a href="#ultimate-array-item-type"><b>Ultimate Array Item Type</b></a> is a Class (2) in the                          System Library</li>
                    <li>An Array whose Ultimate Array Item Type is System.Object, String, or a <a href="#primitive-type"><b>Primitive Type</b></a> but does not meet the definition of ObjectArray, StringArray, or PrimitiveArray.</li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>Class</br>4</td>
            <td>The Remoting Type is a Class (2) or an Array whose Ultimate Array Item Type is a              Class (2) that is not in the System Library.</td>
        </tr>
        <tr>
            <td>ObjectArray</br>5</td>
            <td>The Remoting Type is a single-dimensional Array of System.Object with a lower                 bound of 0.</td>
        </tr>
        <tr>
            <td>StringArray</br>6</td>
            <td>The Remoting Type is a single-dimensional Array of String with a lower bound of               0.</td>
        </tr>
        <tr>
            <td>PrimitiveArray</br>7</td>
            <td>The Remoting Type is a single-dimensional Array of a Primitive Type with a lower              bound of 0.</td>
        </tr>
    </tbody>
</table>

## 2.1.2.3 PrimitiveTypeEnumeration

The PrimitiveTypeEnumeration identifies a **[Primitive Type](#primitive-type)** value. The size of the enumeration is a BYTE.

| Constant/value  | Description                                                   |
|-----------------|---------------------------------------------------------------|
| Boolean</br>1   | Identifies a BOOLEAN as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.4. |
| Byte</br>2      | Identifies a BYTE as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.6.    |
| Char</br>3      | Identifies a [Char (section 2.1.1.1)](#2111-char) type.                     |
| </br>4          | The value is not used in the protocol.                        |
| Decimal</br>5   | Identifies a [Decimal (section 2.1.1.7)](#2117-decimal).                       |
| Double</br>6    | Identifies a [Double (section 2.1.1.2)](#2112-double).                        |
| Int16</br>7     | Identifies an INT16 as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.21. |
| Int32</br>8     | Identifies an INT32 as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22. |
| Int64</br>9     | Identifies an INT64 as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.23. |
| SByte</br>10    | Identifies an INT8 as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.20.  |
| Single</br>11   | Identifies a [Single (section 2.1.1.3)](#2113-single).                        |
| TimeSpan</br>12 | Identifies a [TimeSpan (section 2.1.1.4)](#2114-timespan).                      |
| DateTime</br>13 | Identifies a [DateTime (section 2.1.1.5)](#2115-datetime).                      |
| UInt16</br>14   | Identifies a UINT16 as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.48. |
| UInt32</br>15   | Identifies a UINT32 as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.49. |
| UInt64</br>16   | Identifies a UINT64 as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.50. |
| Null</br>17     | Identifies a [Null Object](#null-object).                                     |
| String</br>18   | Identifies a [LengthPrefixedString (section 2.1.1.6)](#2116-lengthprefixedstring) value.    |

## 2.2 Method Invocation Records

This section specifies **records** that define the format for information required for a **[Remote Method](#remote-method)** invocation. [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) sections 3.1.5.1.1 and 3.1.5.1.2 describe the mechanism to map a method invocation to the records defined in this section.

## 2.2.1 Enumerations

## 2.2.1.1 Messageflags

The MessageFlags enumeration is used by the [BinaryMethodCall (section 2.2.3.1)](#2231-binarymethodcall) or [BinaryMethodReturn (section 2.2.3.3)](#2233-binarymethodreturn) **records** to provide information about the structure of the record. The type of the enumeration is INT32, as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22.

The following table is common for both the BinaryMethodCall and BinaryMethodReturn records. The term "Method record" is used in the description when it is applicable to both the records. The term "Call Array record" is used in the description when it is applicable to both [MethodCallArray (section 2.2.3.2)](#2232-methodcallarray) and [MethodReturnCallArray (section 2.2.3.4)](#2234-methodreturncallarray).

<table border="1">
    <thead>
        <th align="center">Constant/value</th>
        <th align="center">Description</th>
    </thead>
    <tbody>
        <tr>
            <td>NoArgs</br>0x00000001</td>
            <td>The record contains no arguments. It is in the Arg category.</td>
        </tr>
        <tr>
            <td>ArgsInline</br>0x00000002</td>
            <td>The Arguments <a href="#array"><b>Array</a></b> is in the <b>Args</b> field of the Method record. It is in the Arg category.</td>
        </tr>
        <tr>
            <td>ArgsIsArray</br>0x00000004</td>
            <td>Each argument is an item in a separate Call Array record. It is in the Arg category</td>
        </tr>
        <tr>
            <td>ArgsInArray</br>0x00000008</td>
            <td>The Arguments Array is an item in a separate Call Array record. It is in the Arg category.</td>
        </tr>
        <tr>
            <td>NoContext</br>0x00000010</td>
            <td>The record does not contain a <a href="#call-context"><b>Call Context</b></a> value. It is in the Context category.</td>
        </tr>
        <tr>
            <td>ContextInline</br>0x00000020</td>
            <td>Call Context contains only a <a href="#logical-call-id"><b>Logical Call ID</b></a> value and is in the <b>CallContext</b> field of the Method record. It is in the Context category.</td>
        </tr>
        <tr>
            <td>ContextInArray</br>0x00000040</td>
            <td>CallContext values are contained in an array that is contained in the Call Array record. It is in the Context category.</td>
        </tr>
        <tr>
            <td>MethodSignatureInArray</br>0x00000080</td>
            <td>The <a href="#method-signature"><b>Method Signature</b></a> is contained in the Call Array record. It is in the Signature category.</td>
        </tr>
        <tr>
            <td>PropertiesInArray</br>0x00000100</td>
            <td><a href="#message-properties"><b>Message Properties</b></a> is contained in the Call Array record. It is in the Property category.</td>
        </tr>
        <tr>
            <td>NoReturnValue</br>0x00000200</td>
            <td>The <a href="#return-value"><strong>Return Value</strong></a> is a <a href="#null-object"><strong>Null object</strong></a>. It is in the Return category.</td>
        </tr>
        <tr>
            <td>ReturnValueVoid</br>0x00000400</td>
            <td>The method has no Return Value. It is in the Return category.</td>
        </tr>
        <tr>
            <td>ReturnValueInline</br>0x00000800</td>
            <td>The Return Value is in the <strong>ReturnValue</strong> field of the MethodReturnCallArray record. It is in the Return category.</td>
        </tr>
        <tr>
            <td>ReturnValueInArray</br>0x00001000</td>
            <td>The Return Value is contained in the MethodReturnCallArray record. It is in the Return category.</td>
        </tr>
        <tr>
            <td>ExceptionInArray</br>0x00002000</td>
            <td>An <a href="#exception"><strong>Exception</strong></a> is contained in the MethodReturnCallArray record. It is in the Exception category.</td>
        </tr>
        <tr>
            <td>GenericMethod</br>0x00008000</td>
            <td>The <a href="#remote-method"><strong>Remote Method</strong></a> is generic and the actual <a href="#remoting-type"><strong>Remoting Types</strong></a> for the <a href="#generic-argument"><strong>Generic Arguments</strong></a> are contained in the Call Array. It is in the Generic category.<a href="#5-appendix-a-product-behavior"><2></a></td>
        </tr>
    </tbody>
</table>

The preceding table lists the possible values of the enumeration. The category designation for each value provides the grouping of these values. It is a flags enumeration. However, not all combinations are valid. 

To be valid, a **MessageFlags** value is required to conform to the following:

- For each flags category given in the preceding table (Arg, Context, Signature, Return, Exception, Property, and Generic), the value MUST NOT have more than one flag from the Category set.

- The Args and Exception flag categories are exclusive: if a flag from the Args category is set, the value MUST NOT have any flag from the Exception category set, and vice versa.

- The Return and Exception flag categories are exclusive: if a flag from the Return category is set, the value MUST NOT have any flag from the Exception category set, and vice versa.

- The Return and Signature categories are exclusive: if a flag from the Return category is set, the value MUST NOT have any flag from the Signature category set, and vice versa.

- The Exception and Signature categories are exclusive: if a flag from the Signature category is set, the value MUST NOT have any flag from the Exception category set, and vice versa.

The following table summarizes the preceding rules.

|           | Arg     | Context   | Signature | Return   | Exception  | Property   | Generic |
|-----------|---------|-----------|-----------|----------|------------|------------|---------|
| Arg       | Invalid | Valid     | Valid     | Valid    | Invalid    | Valid      | Valid   |
| Context   | Valid   | Invalid   | Valid     | Valid    | Valid      | Valid      | Valid   |
| Signature | Valid   | Valid     | N/A       | Invalid  | Invalid    | Valid      | Valid   |
| Return    | Valid   | Valid     | Invalid   | Invalid  | Invalid    | Valid      | Valid   |
| Exception | Invalid | Valid     | Invalid   | Invalid  | N/A        | Valid      | Valid   |
| Property  | Valid   | Valid     | Valid     | Valid    | Valid      | N/A        | Valid   |
| Generic   | Valid   | Valid     | Valid     | Valid    | Valid      | Valid      | N/A     |

The combination of Signature and Signature, Property and Property, Generic and Generic, or Exception and Exception is not applicable because there is only one bit in the **[Enum](#enum)** for each of these categories. 

## 2.2.2 Common Structures

## 2.2.2.1 ValueWithCode

The ValueWithCode structure is used to associate a **[Primitive Type](#primitive-type)** with an **[Enum](#enum)** that identifies the **[Primitive Type](#primitive-type)** of the Primitive Value. 

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">PrimitiveTypeEnum</td>
    <td align="center" colspan="24">Value (variable)</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
  </tbody>
</table>

**PrimitiveTypeEnum (1 byte):** A [PrimitiveTypeEnumeration](#2123-primitivetypeenumeration) value that specifies the type of the data.

**Value (variable):** A Primitive Value whose Primitive Type is identified by the **PrimitiveTypeEnum** field. For example, if the value of the **PrimitiveTypeEnum** field is the PrimitiveTypeEnumeration value INT32, the **Value** field MUST contain a valid INT32 (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) instance. The length of the field is determined by the Primitive Type of the **Value**. This field MUST NOT be present if the value of **PrimitiveTypeEnum** is Null (17).

## 2.2.2.2 StringValueWithCode

The StringValueWithCode structure is a [ValueWithCode](#2221-valuewithcode) where [PrimitiveTypeEnumeration](#primitive-type) is String (18).

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">PrimitiveTypeEnum</td>
    <td align="center" colspan="24">StringValue (variable)</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
  </tbody>
</table>

**PrimitiveTypeEnum (1 byte):** A PrimitiveTypeEnumeration value that specifies the **[Primitive Type](#primitive-type)** of the data. The value MUST be 18 (String).

**StringValue (variable):** A [LengthPrefixedString](#2116-lengthprefixedstring) that contains the string value.

## 2.2.2.3 ArrayOfValueWithCode

The ArrayOfValueWithCode structure contains a list of [ValueWithCode](#2221-valuewithcode) records. The list is prefixed with the length of the **[Array](#array)**.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="32">Length</td>
    <tr>
        <td align="center" colspan="32">ListOfValueWithCode (variable)</td>
    </tr>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
  </tbody>
</table>

**Length (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that indicates the number of items in the Array. The value can range from 0 to 2147483647 (2^31) inclusive.

**ListOfValueWithCode (variable):** A sequence of ValueWithCode records. The number of items in the sequence MUST be equal to the value specified in the **Length** field.

## 2.2.3 Record Definitions

## 2.2.3.1 BinaryMethodCall

The BinaryMethodCall record contains information that is required to perform a **[Remote Method](#remote-method)** invocation.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">BinaryTypeEnumeration</td>
    <td align="center" colspan="24">MessageEnum</td>
    <tr>
        <td align="center" colspan="8">...</td>
        <td align="center" colspan="24">MethodName (variable)</td>
    </tr>
    <td align="center" colspan="32">...</td>
    <tr>
        <td align="center" colspan="32">TypeName (variable)</td>
    </tr>
    <td align="center" colspan="32">...</td>
    <tr>
        <td align="center" colspan="32">CallContext (variable)</td>
    </tr>
    <td align="center" colspan="32">...</td>
    <tr>
        <td align="center" colspan="32">Args (variable)</td>
    </tr>
    <td align="center" colspan="32">...</td>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. The value MUST be 21.

**MessageEnum (4 bytes):** A [MessageFlags](#2211-messageflags) value that indicates whether the arguments and **[Call Context](#call-context)**, **[Message Properties](#message-properties)**, **[Generic Arguments](#generic-argument)**, and **[Method Signature](#method-signature)** are present. It also specifies whether the arguments and Call Context are present in this record or in the following [MethodCallArray](#2232-methodcallarray) record. For this record type, the field MUST NOT contain the values from the Return and the **[Exception](#exception)** categories.

**MethodName (variable):** A [StringValueWithCode](#2222-stringvaluewithcode) that represents the Remote Method name. The format of the string is as specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 2.2.1.1.

**TypeName (variable):** A StringValueWithCode that represents the **[Server Type](#server-type)** name. The format of the string is specified as QualifiedTypeName, as specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 2.2.1.2.

**CallContext (variable):** A StringValueWithCode that represents the **[Logical Call ID](#logical-call-id)**. This field is conditional. If the **MessageEnum** field has the ContextInline bit set, the field MUST be present; otherwise, the field MUST NOT be present. The presence of this field indicates that the Call Context contains a single entry with the Name as "__RemotingData" and the value is an instance of the **[Remoting Type](#remoting-type)** CallContextRemotingData, as specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 2.2.2.16. The value of this field MUST be interpreted as the value of the **logicalCallID** field in the CallContextRemotingData **[Class (2)](#class)**.

**Args (variable):** An [ArrayOfValueWithCode](#2223-arrayofvaluewithcode) where each item of the **[Array](#array)** corresponds to an input argument of the method. The items of the Array MUST be in the same order as the input arguments. This field is conditional. If the **MessageEnum** field has the ArgsInline bit set, the field MUST be present; otherwise, the field MUST NOT be present.

## 2.2.3.2 MethodCallArray

The MethodCallArray is a special use of the [ArraySingleObject](#2432-arraysingleobject) record. The record represents a **[serialized Array](#serialize)** that can contain instances of any **[Remoting Type](#remoting-type)**. The items of the Array include **[Input Arguments](#input-argument)**, **[Generic Type](#generic-type)** Arguments, **[Method Signature](#method-signature)**, **[Call Context](#call-context)**, and **[Message Properties](#message-properties)**. Each item is conditional. The conditions for presence of the item are given with the definition of each item. The items, if present, MUST be in the following order:

1. **[Input Arguments](#input-argument)**: An Array that contains the Input Arguments for the method. This item is conditional. If the **MessageEnum** field of the preceding [BinaryMethodCall](#2231-binarymethodcall) record has the ArgsInArray bit set, the item MUST be present; otherwise, the item MUST NOT be present.

2. **Generic Type Arguments**: An Array of UnitySerializationHolder **[classes (1)](#class)**, as specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 2.2.2.12. The presence of this field indicates that the method represented by the BinaryMethodCall record is a Generic Method. Each item of the array contains a Remoting Type that MUST be used as **[Generic Argument](#generic-argument)** for the Generic Method. This field is conditional. If the **MessageEnum** field of the preceding BinaryMethodCall record has the GenericMethod bit set, the field MUST be present; otherwise, the field MUST NOT be present.[<3>](#5-appendix-a-product-behavior)

3. **[Method Signature](#method-signature)**: An Array of UnitySerializationHolder classes (1) as specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 2.2.2.12. Each item of the Array contains the Remoting Type of an argument of the **[Remote Method](#remote-method)**. If the **MessageEnum** field of the preceding BinaryMethodCall record has the MethodSignatureInArray bit set, the field MUST be present; otherwise, the field MUST NOT be present. If present, the number of items in the Array MUST match the number of items in the Input Argument item.

4. **[Call Context](#call-context)**: An instance of the Class (2) "System.Runtime.Remoting.Messaging.LogicalCallContext". The **[Library](#library)** name of the Class (2) is "mscorlib". Each name-value pair of the Call Context MUST be mapped to a **[Member](#member)** name and Member value of the Class (2). If the **MessageEnum** field of the preceding BinaryMethodCall record has the ContextInArray bit set, the field MUST be present; otherwise, the field MUST NOT be present.

5. **[Message Properties](#message-properties)**: An Array that can contain instances of any Remoting Type. Each instance is a DictionaryEntry, as specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 2.2.2.6. If the **MessageEnum** field of the preceding BinaryMethodCall record has the PropertiesInArray bit set, the field MUST be present; otherwise, the field MUST NOT be present.

## 2.2.3.3 BinaryMethodReturn

The BinaryMethodReturn record contains the information returned by a **[Remote Method](#remote-method)**.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">BinaryTypeEnumeration</td>
    <td align="center" colspan="24">MessageEnum</td>
    <tr>
        <td align="center" colspan="8">...</td>
        <td align="center" colspan="24">ReturnValue (variable)</td>
    </tr>
    <td align="center" colspan="32">...</td>
    <tr>
        <td align="center" colspan="32">CallContext (variable)</td>
    </tr>
    <td align="center" colspan="32">...</td>
    <tr>
        <td align="center" colspan="32">Args (variable)</td>
    </tr>
    <td align="center" colspan="32">...</td>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. The value MUST be 22.

**MessageEnum (4 bytes):** A [MessageFlags](#2211-messageflags) value that indicates whether the **[Return Value](#return-value)**, **[Arguments](#argument)**, **[Message Properties](#message-properties)**, and **[Call Context](#call-context)** are present. The value also specifies whether the Return Value, Arguments, and Call Context are present in this record or the following [MethodReturnCallArray](#2234-methodreturncallarray) record. For this record, the field MUST NOT have the MethodSignatureInArray or GenericMethod bits set.

**ReturnValue (variable):** A [ValueWithCode](#2221-valuewithcode) that contains the Return Value of a Remote Method. If the **MessageEnum** field has the ReturnValueInline bit set, this field MUST be present; otherwise, this field MUST NOT be present.

**CallContext (variable):** A [StringValueWithCode](#2222-stringvaluewithcode) that represents the **[Logical Call ID](#logical-call-id)**. This field is conditional. If the **MessageEnum** field has the ContextInline bit set, the field MUST be present; otherwise, the field MUST NOT be present.

**Args (variable):** An [ArrayOfValueWithCode](#2223-arrayofvaluewithcode) that contains the **[Output Arguments](#output-argument)** of the method. This field is conditional. If the **MessageEnum** field has the ArgsInline bit set, the field MUST be present; otherwise, the field MUST NOT be present.

## 2.2.3.4 MethodReturnCallArray

The MethodReturnCallArray is a special use of the [ArraySingleObject](#2432-arraysingleobject) record. The record represents a **[serialized Array](#serialize)** that can contain instances of any **[Remoting Type](#remoting-type)**. The items of the Array include **[Return Value](#return-value)**, **[Output Arguments](#output-argument)**, **[Exception](#exception)**, **[Call Context](#call-context)**, and **[Message Properties](#message-properties)**. Each item is conditional. The conditions for presence of the item are given with the definition of the item in the following list. The items, if present, MUST be in the following order:

1. **[Return Value](#return-value)**: The Return Value of the method. This item is conditional. If the **MessageEnum** field of the preceding [BinaryMethodReturn](#2233-binarymethodreturn) record has the ReturnValueInArray bit set, the item MUST be present; otherwise, the item MUST NOT be present.

2. **Output Arguments**: An Array that contains the Output Arguments for the method. This item is conditional. If the **MessageEnum** field of the preceding BinaryMethodReturn record has the ArgsInArray bit set, the item MUST be present; otherwise, the item MUST NOT be present.

3. **[Exception](#exception)**: A **[Data Value](#data-value)** assignable to System.Exception **[Class (2)](#class)** as specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 2.2.2.7. This item is conditional. If the **MessageEnum** field of the preceding BinaryMethodReturn record has the ExceptionInArray bit set, the item MUST be present; otherwise, the item MUST NOT be present.

4. **[Call Context](#call-context)**: An instance of the Class (2) called "System.Runtime.Remoting.Messaging.LogicalCallContext". The **[Library](#library)** name of the Class (2) is "mscorlib". Each name-value pair of the Call Context MUST be mapped to a **[Member](#member)** name and a Member value of the Class (2). If the **MessageEnum** field of the preceding BinaryMethodReturn record has the ContextInArray bit set, the field MUST be present; otherwise, the field MUST NOT be present.

5. **[Message Properties](#message-properties)**: An Array that can contain instances of any Remoting Type. Each instance is a DictionaryEntry, as specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 2.2.2.6. If the **MessageEnum** field of the preceding BinaryMethodReturn record has the PropertiesInArray bit set, the field MUST be present; otherwise, the field MUST NOT be present.

## 2.3 Class Records

This section defines **[Class (1)](#class)** records. A Class (1) record represents an instance of a Class (1). [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.5.1.6 describes the mechanism to map a Class (1) instance to a record defined in this section. [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.5.1.9 describes the mechanism to map an **[Enum](#enum)** value to a record defined in this section.

The values of the **[Members](#member)** of the Class (1) MUST be **[serialized](#serialize)** as records that follow this record, as specified in section 2.7. The order of the records MUST match the order of MemberNames as specified in the [ClassInfo (section 2.3.1.1)](#2311-classinfo) structure.

## 2.3.1 Common Structures

## 2.3.1.1 ClassInfo

ClassInfo is a common structure used by all the **[Class (2)](#class)** records. It has the following structure.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="32">ObjectId</td>
    <tr>
        <td align="center" colspan="32">Name (variable)</td>
    </tr>
    <td align="center" colspan="32">...</td>
    <tr>
        <td align="center" colspan="32">MemberCount</td>
    </tr>
    <tr>
        <td align="center" colspan="32">MemberNames (variable)</td>
    </tr>
    <td align="center" colspan="32">...</td>
  </tbody>
</table>

**ObjectId (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that uniquely identifies the object in the **[serialization stream](#serialization-stream)**. An implementation MAY use any algorithm to generate the unique IDs. If the ObjectId is referenced by a [MemberReference](#253-memberreference) record elsewhere in the serialization stream, the ObjectId MUST be positive. If the ObjectId is not referenced by any MemberReference in the serialization stream, then the ObjectId SHOULD be positive, but MAY be negative.[<4>](#5-appendix-a-product-behavior)

**Name (variable):** A [LengthPrefixedString](#2116-lengthprefixedstring) value that contains the name of the Class (1). The format of the string MUST be as specified in the RemotingTypeName, as specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 2.2.1.2.

**MemberCount (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that contains the number of **[Members](#member)** in the Class (2). The value MUST be 0 or a positive integer.

**MemberNames (variable):** A sequence of LengthPrefixedString values that represents the names of the Members in the class (2). The number of items in the sequence MUST be equal to the value specified in the **MemberCount** field. The MemberNames MAY be in any order.[<5>](#5-appendix-a-product-behavior)

## 2.3.1.2 MemberTypeInfo

The MemberTypeInfo is a common structure that contains type information for **[Class (2) Members](#class)**. It has the following structure.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="32">BinaryTypeEnums (variable)</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
    <tr>
        <td align="center" colspan="32">AdditionalInfos (variable)</td>
    </tr>
    <td align="center" colspan="32">...</td>
  </tbody>
</table>

**BinaryTypeEnums (variable):** A sequence of [BinaryTypeEnumeration](#2122-binarytypeenumeration) values that represents the Member Types that are being transferred. The **[Array](#array)** MUST:

- Have the same number of items as the **MemberCount** field of the [ClassInfo](#2311-classinfo) structure.

- Be ordered such that the BinaryTypeEnumeration corresponds to the Member name in the MemberNames field of the ClassInfo structure. 

**AdditionalInfos (variable):** A sequence of additional information about a **[Remoting Type](#remoting-type)**. For every value of the BinaryTypeEnum in the **BinaryTypeEnums** field that is a Primitive, SystemClass, Class (2), or PrimitiveArray, the **AdditionalInfos** field contains additional information about the Remoting Type. For the BinaryTypeEnum value of Primitive and PrimitiveArray, this field specifies the actual **[Primitive Type](#primitive-type)** that uses the PrimitiveTypeEnum. For the BinaryTypeEnum value of SystemClass, this field specifies the name of the class (2). For the BinaryTypeEnum value of Class (2), this field specifies the name of the Class (2) and the **[Library](#library)** ID. The following table enumerates additional information required for each BinaryType enumeration.

| BinaryTypeEnum   | AdditionalInfos                                                   |
|------------------|-------------------------------------------------------------------|
| Primitive        | PrimitiveTypeEnumeration                                          |
| String           | None                                                              |
| Object           | None                                                              |
| SystemClass      | String (Class (1) name as specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 2.2.1.2) |
| Class            | ClassTypeInfo                                                     |
| ObjectArray      | None                                                              |
| StringArray      | None                                                              |
| PrimitiveArray   | PrimitiveTypeEnumeration                                          |

- The AdditionalInfos sequence MUST NOT contain any item for the BinaryTypeEnum values of String, Object, ObjectArray, or StringArray.

- The AdditionalInfos items MUST be in the same order as the corresponding BinaryTypeEnum items in the **BinaryTypeEnums** field. 

- When the BinaryTypeEnum value is Primitive, the PrimitiveTypeEnumeration value in AdditionalInfo MUST NOT be Null (17) or String (18).

## 2.3.2 Record Definitions

## 2.3.2.1 ClassWithMembersAndTypes

The ClassWithMembersAndTypes record is the most verbose of the Class records. It contains metadata about **[Members](#member)**, including the names and **[Remoting Types](#remoting-type)** of the Members. It also contains a **[Library](#library)** ID that references the Library Name of the Class.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">RecordTypeEnum</td>
    <td align="center" colspan="24">ClassInfo (variable)</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
    <td align="center" colspan="32">MemberTypeInfo (variable)</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
    <td align="center" colspan="32">LibraryId</td>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. Its value MUST be 5.

**ClassInfo (variable):** A [ClassInfo](#2311-classinfo) structure that provides information about the name and Members of the Class.

**MemberTypeInfo (variable):** A [MemberTypeInfo](#2312-membertypeinfo) structure that provides information about the Remoting Types of the Members.

**LibraryId (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that references a [BinaryLibrary](#262-binarylibrary) record by its Library ID. A BinaryLibrary record with the LibraryId MUST appear earlier in the **[serialization stream](#serialization-stream)**.

## 2.3.2.2 ClassWithMembers

The ClassWithMembers record is less verbose than [ClassWithMembersAndTypes](#2321-classwithmembersandtypes). It does not contain information about the **[Remoting Type](#remoting-type)** information of the **[Members](#member)**. This record can be used when the information is deemed unnecessary because it is known out of band or can be inferred from context.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">RecordTypeEnum</td>
    <td align="center" colspan="24">ClassInfo (variable)</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
    <td align="center" colspan="32">LibraryId</td>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. Its value MUST be 3.

**ClassInfo (variable):** A [ClassInfo](#2311-classinfo) structure that provides information about the name and Members of the Class.

**LibraryId (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that references a [BinaryLibrary](#262-binarylibrary) record by its Library ID. The ID MUST be a positive integer. A BinaryLibrary record with the LibraryId MUST appear earlier in the **[serialization stream](#serialization-stream)**.

## 2.3.2.3 SystemClassWithMembersAndTypes

The SystemClassWithMembersAndTypes record is less verbose than [ClassWithMembersAndTypes](#2321-classwithmembersandtypes). It does not contain a LibraryId. This record implicitly specifies that the Class is in the **[System Library](#system-library)**.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">RecordTypeEnum</td>
    <td align="center" colspan="24">ClassInfo (variable)</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
    <td align="center" colspan="32">MemberTypeInfo (variable)</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. Its value MUST be 4.

**ClassInfo (variable):** A [ClassInfo](#2311-classinfo) structure that provides information about the name and **[Members](#member)** of the Class.

**MemberTypeInfo (variable):** A [MemberTypeInfo](#2312-membertypeinfo) structure that provides information about the **[Remoting Type](#remoting-type)** of the Members.

## 2.3.2.4 SystemClassWithMembers

The SystemClassWithMembers record is less verbose than [ClassWithMembersAndTypes](#2321-classwithmembersandtypes). It does not contain a LibraryId or the information about the **[Remoting Types](#remoting-type)** of the **[Members](#member)**. This record implicitly specifies that the Class is in the **[System Library](#system-library)**. This record can be used when the information is deemed unnecessary because it is known out of band or can be inferred from context.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">RecordTypeEnum</td>
    <td align="center" colspan="24">ClassInfo (variable)</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. Its value MUST be 2.

**ClassInfo (variable):** A [ClassInfo](#2311-classinfo) structure that provides information about the name and Members of the Class.

## 2.3.2.5 ClassWithId

The ClassWithId record is the most compact. It has no metadata. It refers to metadata defined in [SystemClassWithMembers](#2324-systemclasswithmembers), [SystemClassWithMembersAndTypes](#2323-systemclasswithmembersandtypes), [ClassWithMembers](#2322-classwithmembers), or [ClassWithMembersAndTypes](#2321-classwithmembersandtypes) record. 

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">RecordTypeEnum</td>
    <td align="center" colspan="24">ObjectId</td>
    <tr>
        <td align="center" colspan="8">...</td>
        <td align="center" colspan="24">MetadataId</td>
    </tr>
    <td align="center" colspan="8">...</td>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. The value MUST be 1.

**ObjectId (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that uniquely identifies the object in the **[serialization stream](#serialization-stream)**.

**MetadataId (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that references one of the other Class records by its ObjectId. A SystemClassWithMembers, SystemClassWithMembersAndTypes, ClassWithMembers, or ClassWithMembersAndTypes record with the value of this field in its **ObjectId** field MUST appear earlier in the serialization stream.

## 2.4 Array Records

This section defines **[Array](#array)** records that represent Array instances. [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.5.1.7, describes the mechanism to map an Array instance to a record defined in this section.

Items of an Array MUST be **[serialized](#serialize)** as records following the Array record, as specified in section [2.7](#27-binary-record-grammar). The number of records that contain the Array items depends on the type of Array record. For the [ArraySingleObject](#2432-arraysingleobject), [ArraySinglePrimitive](#2433-arraysingleprimitive), and [ArraySingleString](#2434-arraysinglestring) records, the number of records containing Array items MUST be equal to the value of the **Length** field of the **ArrayInfo** field. For [BinaryArray](#2431-binaryarray) records, the number of records containing Array items MUST be equal to the product of the values contained in the **Lengths** field of the BinaryArray record. In the cases where an item of an Array can contain a **[Null Object](#null-object)**, multiple ObjectNull records in sequence MAY be represented by a single [ObjectNullMultiple (section 2.5.5)](#255-objectnullmultiple) or [ObjectNullMultiple256 (section 2.5.6)](#256-objectnullmultiple256) record. Each of these records contains a **NullCount** field that states how many [ObjectNull](#254-objectnull) records that the record represents. For the purpose of calculating the number of records, a single ObjectNullMultiple or ObjectNullMultiple256 record is counted as many times as the value specified in the **NullCount** field.[<6>](#5-appendix-a-product-behavior)

## 2.4.1 Enumerations

## 2.4.1.1 BinaryArrayTypeEnumeration

The BinaryArrayTypeEnumeration is used to denote the type of an **[Array](#array)**. The size of the enumeration is 1 byte. It is used by the Array records.

| Constant/value          | Description                                                                                                 |
|-------------------------|-------------------------------------------------------------------------------------------------------------|
| Single</br>0            | A single-dimensional Array.                                                                                 |
| Jagged</br>1            | An Array whose elements are Arrays. The elements of a jagged Array can be of different dimensions and sizes |
| Rectangular</br>2       | A multi-dimensional rectangular Array.                                                                      |
| SingleOffset</br>3      | A single-dimensional offset.                                                                                |
| JaggedOffset</br>4      | A jagged Array where the lower bound index is greater than 0.                                               |
| RectangularOffset</br>5 | Multi-dimensional Arrays where the lower bound index of at least one of the dimensions is  greater than 0.  |

## 2.4.2 Common Definitions

## 2.4.2.1 Arrayinfo

The ArrayInfo is a common structure that is used by **[Array](#array)** records.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="32">ObjectId</td>
    <tr>
        <td align="center" colspan="32">Length</td>
    </tr>
  </tbody>
</table>

**ObjectId (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that uniquely identifies the Array instance in the **[serialization stream](#serialization-stream)**. The ID MUST be a positive integer. An implementation MAY use any algorithm to generate the unique IDs.[<7>](#5-appendix-a-product-behavior)

**Length (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that specifies the number of items in the Array. The value MUST be 0 or a positive integer.

## 2.4.3 Record Definitions

## 2.4.3.1 BinaryArray

BinaryArray is the most general form of **[Array](#array)** records. The record is more verbose than the other Array records. 

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">RecordTypeEnum</td>
    <td align="center" colspan="24">ObjectId</td>
    <tr>
        <td align="center" colspan="8">...</td>
        <td align="center" colspan="8">BinaryArrayTypeEnum</td>
        <td align="center" colspan="16">Rank</td>
    </tr>
    <td align="center" colspan="16">...</td>
    <td align="center" colspan="16">Lengths (variable)</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
    <td align="center" colspan="32">LowerBounds (variable)</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
    <td align="center" colspan="8">TypeEnum</td>
    <td align="center" colspan="24">AdditionalTypeInfo (variable)</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. Its value MUST be 7.

**ObjectId (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that uniquely identifies the Array in the **[serialization stream](#serialization-stream)**. The value MUST be a positive integer. An implementation MAY use any algorithm to generate the unique IDs.[<8>](#5-appendix-a-product-behavior)

**BinaryArrayTypeEnum (1 byte):** A [BinaryArrayTypeEnumeration](#2411-binaryarraytypeenumeration) value that identifies the type of the Array.

**Rank (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that specifies the rank (number of dimensions) of the Array. The value MUST be 0 or a positive integer.

**Lengths (variable):** A sequence of INT32 values (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that specifies the length of each of the dimensions of the Array. The number of values MUST be equal to the value specified in the **Rank** field. Each value of the sequence MUST be 0 or a positive integer.

**LowerBounds (variable):** A sequence of INT32 values (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that specifies the lower bound (first index) of each of the dimensions of the Array. The number of values MUST be equal to the value specified in the **Rank** field. If the value of the **BinaryArrayTypeEnum** field is SingleOffset, JaggedOffset, or RectangularOffset, this field MUST be present in the serialization stream; otherwise, this field MUST NOT be present in the serialization stream.

**TypeEnum (1 byte):** A BinaryTypeEnum value that identifies the **[Remoting Type](#remoting-type)** of the Array item.

**AdditionalTypeInfo (variable):** Information about the Remoting Type of the Array item in addition to the information provided in the **TypeEnum** field. For the BinaryTypeEnum values of Primitive, SystemClass, Class, or PrimitiveArray, this field contains additional information about the Remoting Type. For the BinaryTypeEnum value of Primitive and PrimitiveArray, this field specifies the actual **[Primitive Type](#primitive-type)** that uses the PrimitiveTypeEnum. For the BinaryTypeEnum value of SystemClass, this field specifies the name of the Class. For the BinaryTypeEnum value of Class, this field specifies the name of the Class and the **[Library](#library)** ID. The following table enumerates additional information that is required for each BinaryType enumeration.

| BinaryTypeEnum   | AdditionalTypeInfo                                            |
|------------------|---------------------------------------------------------------|
| Primitive        | PrimitiveTypeEnum                                             |
| Object           | None                                                          |
| String           | None                                                          |
| SystemClass      | String (Class name as specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 2.2.1.2) |
| Class            | [ClassTypeInfo](#2118-classtypeinfo)                                                 |
| ObjectArray      | None                                                          |
| StringArray      | None                                                          |
| PrimitiveArray   | PrimitiveTypeEnum                                             |

If the BinaryTypeEnum value of the **TypeEnum** field is Object, String, ObjectArray, or StringArray, this field MUST NOT be present in the serialization stream.

If the BinaryTypeEnum value is Primitive, the [PrimitiveTypeEnumeration](#2123-primitivetypeenumeration) value in AdditionalTypeInfo MUST NOT be Null (17) or String (18).

## 2.4.3.2 ArraySingleObject

The ArraySingleObject record contains a single-dimensional **[Array](#array)** in which each **[Member](#member)** record MAY contain any **[Data Value](#data-value)**.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">RecordTypeEnum</td>
    <td align="center" colspan="24">ArrayInfo</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
    <td align="center" colspan="8">...</td>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. The value MUST be 16.

**ArrayInfo (8 bytes):** An [ArrayInfo](#2421-arrayinfo) structure that specifies the ID and the length of the Array instance.

## 2.4.3.3 ArraySinglePrimitive

The ArraySinglePrimitive record contains a single-dimensional **[Array](#array)** in which all **[Members](#member)** are **[Primitive Value](#primitive-value)**.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">RecordTypeEnum</td>
    <td align="center" colspan="24">ArrayInfo</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
    <td align="center" colspan="8">...</td>
    <td align="center" colspan="8">PrimitiveTypeEnum</td>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. The value MUST be 15.

**ArrayInfo (8 bytes):** An [ArrayInfo](#2421-arrayinfo) structure that specifies the ID and the length of the Array instance.

**PrimitiveTypeEnum (1 byte):** A [PrimitiveTypeEnumeration](#2123-primitivetypeenumeration) value that identifies the **[Primitive Type](#primitive-type)** of the items of the Array. The value MUST NOT be 17 (Null) or 18 (String).

This record MUST be followed by a sequence of MemberPrimitiveUnTyped records that contain values whose Primitive Type is specified by the **PrimitiveTypeEnum** field. The number of records in the sequence MUST match the value specified in the **Length** field of ArrayInfo.

## 2.4.3.4 ArraySingleString

The ArraySingleString record contains a single-dimensional **[Array](#array)** whose items are String values.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">RecordTypeEnum</td>
    <td align="center" colspan="24">ArrayInfo</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
    <td align="center" colspan="8">...</td>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. The value MUST be 17.

**ArrayInfo (8 bytes):** An [ArrayInfo](#2421-arrayinfo) structure that specifies the ID and the length of the Array instance.

## 2.5 Member Reference Records

**[Arrays](#array)** and classes are containers of **[Member](#member)** values; that is, graph nodes that represent instances of Arrays and Classes that have outbound edges. The Member values are the graph nodes that are destinations for the outbound edges. In the **[serialization stream](#serialization-stream)**, the Member values follow the Array and the Class records. The Member values are **[serialized](#serialize)** by using the Member Reference records.

## 2.5.1 MemberPrimitiveTyped

The MemberPrimitiveTyped record contains a **[Primitive Type](#primitive-type)** value other than String. The mechanism to serialize a **[Primitive Value](#primitive-value)** is described in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.5.1.8. 

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">RecordTypeEnum</td>
    <td align="center" colspan="8">PrimitiveTypeEnum</td>
    <td align="center" colspan="16">Value (variable)</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. The value MUST be 8.

**PrimitiveTypeEnum (1 byte):** A [PrimitiveTypeEnumeration](#2123-primitivetypeenumeration) value that specifies the Primitive Type of data that is being transmitted. This field MUST NOT contain a value of 17 (Null) or 18 (String).

**Value (variable):** The value whose type is inferred from the **PrimitiveTypeEnum** field as specified in the table in section 2.1.2.3.

## 2.5.2 MemberPrimitiveUntyped

The MemberPrimitiveUnTyped record is the most compact record to represent a **[Primitive Type](#primitive-type)** value. This type of record does not have a RecordTypeEnum to indicate the record type. The record MUST be used when a Class Member or **[Array](#array)** item is a Primitive Type. Because the containing Class or Array record specifies the Primitive Type of each Member, the Primitive Type is not respecified along with the value. Also, the **[Primitive Values](#primitive-value)** cannot be referenced by any other record; therefore it does not require an ObjectId. This record has no field besides the value. The mechanism to **[serialize](#serialize)** a Primitive Value is described in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.5.1.8.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="32">Value (variable)</td>
    <tr>
        <td align="center" colspan="32">...</td>
    </tr>
  </tbody>
</table>

**Value (variable):** A Primitive Type value other than String.

## 2.5.3 MemberReference

The MemberReference record contains a reference to another record that contains the actual value. The record is used to **[serialize](#serialize)** values of a Class Member and **[Array](#array)** items. The mechanism to serialize a Class instance is described in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.5.1.6. The mechanism to serialize an Array instance is described in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.5.1.7.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">RecordTypeEnum</td>
    <td align="center" colspan="24">IdRef</td>
    <tr>
        <td align="center" colspan="8">...</td>
    </tr>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. The value MUST be 9.

**IdRef (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that is an ID of an object defined in another record.

- The value MUST be a positive integer.

- A Class, Array, or [BinaryObjectString](#257-binaryobjectstring) record MUST exist in the **[serialization stream](#serialization-stream)** with the value as its ObjectId. Unlike other ID references, there is no restriction on where the record that defines the ID appears in the serialization stream; that is, it MAY appear after the referencing record.[<9>](#5-appendix-a-product-behavior)

## 2.5.4 ObjectNull

The ObjectNull record contains a **[Null Object](#null-object)**. The mechanism to **[serialize](#serialize)** a Null Object is described in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.5.1.12.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">RecordTypeEnum</td>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. The value MUST be 10.

## 2.5.5 ObjectNullMultiple

The ObjectNullMultiple record provides a more compact form for multiple consecutive Null records than using individual ObjectNull records. The mechanism to **[serialize](#serialize)** a **[Null Object](#null-object)** is described in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.5.1.12.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">RecordTypeEnum</td>
    <td align="center" colspan="24">NullCount</td>
    <tr>
        <td align="center" colspan="8">...</td>
    </tr>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. The value MUST be 14.

**NullCount (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that is the count of the number of consecutive Null Objects. The value MUST be a positive integer.

## 2.5.6 ObjectNullMultiple256

The ObjectNullMultiple256 record provides the most compact form for multiple, consecutive Null records when the count of Null records is less than 256. The mechanism to **[serialize](#serialize)** a **[Null Object](#null-object)** is described in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.5.1.12.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">RecordTypeEnum</td>
    <td align="center" colspan="8">NullCount</td>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. The value MUST be 13.

**NullCount (1 byte):** A BYTE value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.6) that is the count of the number of consecutive Null objects. The value MUST be in the range of 0 to 255, inclusive.

## 2.5.7 BinaryObjectString

The BinaryObjectString record identifies an object as a String object, and contains information about it. The mechanism to **[serialize](#serialize)** a string is described in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.5.1.11.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">RecordTypeEnum</td>
    <td align="center" colspan="24">ObjectId</td>
    <tr>
      <td align="center" colspan="8">...</td>
      <td align="center" colspan="24">Value (variable)</td>
    </tr>
    <td align="center" colspan="32">...</td>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. The value MUST be 6.

**ObjectId (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that uniquely identifies the string instance in the **[serialization stream](#serialization-stream)**. The value MUST be a positive integer. An implementation MAY use any algorithm to generate the unique IDs.[<10>](#5-appendix-a-product-behavior)

**Value (variable):** A [LengthPrefixedString](#2116-lengthprefixedstring) value.

## 2.6 Other Records

The following sections define the records that are not part of any of the previous categories.

## 2.6.1 SerializationHeaderRecord

The SerializationHeaderRecord record MUST be the first record in a binary **[serialization](#serialization)**. This record has the major and minor version of the format and the IDs of the top object and the headers.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">RecordTypeEnum</td>
    <td align="center" colspan="24">RootId</td>
    <tr>
      <td align="center" colspan="8">...</td>
      <td align="center" colspan="24">HeaderId</td>
    </tr>
    <td align="center" colspan="8">...</td>
    <td align="center" colspan="24">MajorVersion</td>
    <tr>
      <td align="center" colspan="8">...</td>
      <td align="center" colspan="24">MinorVersion</td>
    </tr>
    <td align="center" colspan="8">...</td>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. The value MUST be 0.

**RootId (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that identifies the root of the graph of nodes. The value of the field is set as follows:
- If a [BinaryMethodCall](#2231-binarymethodcall) record is present in the **[serialization stream](#serialization-stream)** and if there is no [MethodCallArray](#2232-methodcallarray) record following it, the value of this field MUST be 0; if a MethodCallArray record follows the BinaryMethodCall record, the value of this field MUST contain the ObjectId of the MethodCallArray.

- If a [BinaryMethodReturn](#2233-binarymethodreturn) record is present in the serialization stream and if there is no [MethodReturnCallArray](#2234-methodreturncallarray) record following it, the value of this field MUST be 0; if a MethodReturnCallArray record follows the BinaryMethodReturn record, the value of this field MUST contain the ObjectId of the MethodReturnCallArray.

- If neither the BinaryMethodCall nor BinaryMethodReturn record is present in the serialization stream, the value of this field MUST contain the ObjectId of a Class, **[Array](#array)**, or [BinaryObjectString](#257-binaryobjectstring) record contained in the serialization stream.

**HeaderId (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that identifies the Array that contains the header objects. The value of the field is set as follows:
- If a BinaryMethodCall record is present in the serialization stream and if there is no MethodCallArray record following it, the value of this field MUST be 0; if a MethodCallArray record follows the BinaryMethodCall record, the value of this field MUST be -1.

- If a BinaryMethodReturn record is present in the serialization stream and if there is no MethodReturnCallArray record following it, the value of this field MUST be 0; if a MethodReturnCallArray record follows the BinaryMethodReturn record, the value of this field MUST be -1.

- If neither the BinaryMethodCall nor BinaryMethodReturn record is present in the serialization stream, the value of this field MUST contain the ObjectId of a Class, Array , or BinaryObjectString record that is contained in the serialization stream.

The field MUST be ignored on read.

**MajorVersion (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that identifies the major version of the format. The value of this field MUST be 1.

**MinorVersion (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that identifies the minor version of the protocol. The value of this field MUST be 0.

## 2.6.2 BinaryLibrary

The BinaryLibrary record associates an INT32 ID (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) with a **[Library](#library)** name. This allows other records to reference the Library name by using the ID. This approach reduces the wire size when there are multiple records that reference the same Library name.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">RecordTypeEnum</td>
    <td align="center" colspan="24">LibraryId</td>
    <tr>
      <td align="center" colspan="8">...</td>
      <td align="center" colspan="24">LibraryName (variable)</td>
    </tr>
    <td align="center" colspan="32">...</td>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. The value MUST be 12.

**LibraryId (4 bytes):** An INT32 value (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22) that uniquely identifies the Library name in the **[serialization stream](#serialization-stream)**. The value MUST be a positive integer. An implementation MAY use any algorithm to generate the unique IDs.[<11>](#5-appendix-a-product-behavior)

**LibraryName (variable):** A [LengthPrefixedString](#2116-lengthprefixedstring) value that represents the Library name. The format of the string is specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 2.2.1.3.

## 2.6.3 MessageEnd

The MessageEnd record marks the end of the **[serialization stream](#serialization-stream)**.

<table border="1">
  <thead>
    <tr>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>1<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>2<br>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
      <th>8</th>
      <th>9</th>
      <th>3<br>0</th>
      <th>1</th>
    </tr>
  </thead>
  <tbody>
    <td align="center" colspan="8">RecordTypeEnum</td>
  </tbody>
</table>

**RecordTypeEnum (1 byte):** A [RecordTypeEnumeration](#2121-recordtypeenumeration) value that identifies the record type. The value MUST be 11.

## 2.7 Binary Record Grammar

This section specifies the grammar using the Augmented Backus-Naur Form (ABNF) syntax specified in [[RFC4234]](#121-normative-references) that defines how the records can appear in the **[serialization stream](#serialization-stream)**.

<table>
  <thead>
    <th>ABNF productions</th>
    <th></th>
    <th>Meaning</th>
  </thead>
  <tbody>
    <tr>
      <td>ABNF productions</br>remotingMessage</td>
      <td>=</td>
      <td>
        SerializationHeader</br>
        *(referenceable)</br>
        (methodCall/methodReturn)</br>
        *(referenceable)</br>
        MessageEnd
      </td>
    </tr>
    <tr>
      <td>methodCall</td>
      <td>=</td>
      <td>
        0*1(BinaryLibrary)</br>
        BinaryMethodCall</br>
        0*1(callArray)
      </td>
    </tr>
    <tr>
      <td>methodReturn</td>
      <td>=</td>
      <td>
        0*1(BinaryLibrary)</br>
        BinaryMethodReturn</br>
        0*1(callArray)
      </td>
    </tr>
    <tr>
      <td>callArray</td>
      <td>=</td>
      <td>
        0*1(BinaryLibrary)</br>
        ArraySingleObject</br>
        *(memberReference)
      </td>
    </tr>
    <tr>
      <td>memberReference</td>
      <td>=</td>
      <td>
        0*1(BinaryLibrary)</br>
        (MemberPrimitiveUnTyped / MemberPrimitiveTyped / MemberReference /  BinaryObjectString / nullObject / Classes)
      </td>
    </tr>
    <tr>
      <td>nullObject</td>
      <td>=</td>
      <td>
        ObjectNull / ObjectNullMultiple / ObjectNullMultiple256
      </td>
    </tr>
    <tr>
      <td>referenceable</td>
      <td>=</td>
      <td>
        Classes/<b>Arrays</b>/BinaryObjectString
      </td>
    </tr>
    <tr>
      <td>Classes</td>
      <td>=</td>
      <td>
        0*1(BinaryLibrary)</br>
        (ClassWithId / ClassWithMembers/ ClassWithMembersAndTypes /  SystemClassWithMembers / SystemClassWithMembersAndTypes)</br>
        *(memberReference)
      </td>
    </tr>
    <tr>
      <td>Arrays</td>
      <td>=</td>
      <td>
        0*1(BinaryLibrary)</br>
        ((ArraySingleObject *(memberReference)) / (ArraySinglePrimitive  *(MemberPrimitiveUnTyped)) /  (ArraySingleString *(BinaryObjectString/MemberReference/nullObject)) / (BinaryArray *(memberReference)))
      </td>
    </tr>
  </tbody>
</table>

___
# 3 Structure Examples

This sample illustrates the message exchanged when a **[Remote Method](#remote-method)** is invoked as specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.3.4.2. The data model is used to describe the information to perform the Remote Method invocation and the results of the invocation, as specified in [[MS-NRTP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-nrtp) section 3.1.1.

The client invokes a method "SendAddress" on a remote **[Server Type](#server-type)** "DOJRemotingMetadata.MyServer" and passes the following Address object (Street = "One Microsoft Way", City = "Redmond", State = "WA" and Zip = "98054") as an argument. The remote Server Type is accessible at a relative URI "MyServer.Rem" hosted on a server named "maheshdev2" and listening on port 8080. The server receives the request message, reads the argument passed in the message, and then invokes the method with the **[de-serialized](#deserialize)** argument. The server then embeds the **[Return Value](#return-value)** of "Address received" in the response message to the client.

The following is a sequence diagram for the preceding message exchange pattern.

![Sequence Diagram](https://github.com/OfficialSirH/non-binary-formatter/blob/main/documentation/Sequence_Diagram.png?raw=true)

<table>
  <thead>
    <th>Figure 2: Sequence diagram of the message exchanged when a Remote Method is invoked</th>
  </thead>
  <tbody>
    <td>
      The MessageContent data sent on the network is as follows.
      <pre><code>0000 00 01 00 00 00 FF FF FF FF 01 00 00 00 00 00 00 .....ÿÿÿÿ.......
0010 00 15 14 00 00 00 12 0B 53 65 6E 64 41 64 64 72 ........SendAddr
0020 65 73 73 12 6F 44 4F 4A 52 65 6D 6F 74 69 6E 67 ess.oDOJRemoting
0030 4D 65 74 61 64 61 74 61 2E 4D 79 53 65 72 76 65 Metadata.MyServe
0040 72 2C 20 44 4F 4A 52 65 6D 6F 74 69 6E 67 4D 65 r, DOJRemotingMe
0050 74 61 64 61 74 61 2C 20 56 65 72 73 69 6F 6E 3D tadata, Version=
0060 31 2E 30 2E 32 36 32 32 2E 33 31 33 32 36 2C 20 1.0.2622.31326,
0070 43 75 6C 74 75 72 65 3D 6E 65 75 74 72 61 6C 2C Culture=neutral,
0080 20 50 75 62 6C 69 63 4B 65 79 54 6F 6B 65 6E 3D PublicKeyToken=
0090 6E 75 6C 6C 10 01 00 00 00 01 00 00 00 09 02 00 null............
00A0 00 00 0C 03 00 00 00 51 44 4F 4A 52 65 6D 6F 74 .......QDOJRemot
00B0 69 6E 67 4D 65 74 61 64 61 74 61 2C 20 56 65 72 ingMetadata, Ver
00C0 73 69 6F 6E 3D 31 2E 30 2E 32 36 32 32 2E 33 31 sion=1.0.2622.31
00D0 33 32 36 2C 20 43 75 6C 74 75 72 65 3D 6E 65 75 326, Culture=neu
00E0 74 72 61 6C 2C 20 50 75 62 6C 69 63 4B 65 79 54 tral, PublicKeyT
00F0 6F 6B 65 6E 3D 6E 75 6C 6C 05 02 00 00 00 1B 44 oken=null......D
0100 4F 4A 52 65 6D 6F 74 69 6E 67 4D 65 74 61 64 61 OJRemotingMetada
0110 74 61 2E 41 64 64 72 65 73 73 04 00 00 00 06 53 ta.Address.....S
0120 74 72 65 65 74 04 43 69 74 79 05 53 74 61 74 65 treet.City.State
0130 03 5A 69 70 01 01 01 01 03 00 00 00 06 04 00 00 .Zip............
0140 00 11 4F 6E 65 20 4D 69 63 72 6F 73 6F 66 74 20 ..One Microsoft
0150 57 61 79 06 05 00 00 00 07 52 65 64 6D 6F 6E 64 Way......Redmond
0160 06 06 00 00 00 02 57 41 06 07 00 00 00 05 39 38 ......WA......98
0170 30 35 34 0B                                     054.</code></pre>
    </td>
  </tbody>
</table>

Referencing section 2 for various message structures, the bytes listed in the preceding sample can be mapped to the logical Request message structure that is used by .NET Remoting to service the request. The logical Request message for Microsoft .NET Framework 1.1 is as follows.

```
Binary Serialization Format
  SerializationHeaderRecord:
    RecordTypeEnum: SerializedStreamHeader (0x00)
    TopId: 1 (0x1)
    HeaderId: -1 (0xFFFFFFFF)
    MajorVersion: 1 (0x1)
    MinorVersion: 0 (0x0)
  BinaryMethodCall: 
    RecordTypeEnum: BinaryMethodCall (0x21)
    MessageEnum: 00000014
      NoArgs:                   (...............................0) 
      ArgsInline:               (..............................0.)
      ArgsIsArray:              (.............................1..) 
      ArgsInArray:              (............................0...)
      NoContext:                (...........................1....) 
      ContextInline:            (..........................0.....)
      ContextInArray:           (.........................0......)
      MethodSignatureInArray:   (........................0.......)
      PropertyInArray:          (.......................0........)
      NoReturnValue:            (......................0.........)
      ReturnValueVoid:          (.....................0..........) 
      ReturnValueInline:        (....................0...........)
      ReturnValueInArray:       (...................0............) 
      ExceptionInArray:         (..................0.............)
      Reserved:                 (000000000000000000..............)
    MethodName:
      PrimitiveTypeEnum: String (0x12)
      Data: SendAddress
    TypeName:
      PrimitiveTypeEnum: String (0x12)
      Data: DOJRemotingMetadata.MyServer, DOJRemotingMetadata, Version=1.0.2616.21414, Culture=neutral, PublicKeyToken=null
  CallArray:
    ArraySingleObject:
      RecordTypeEnum: ArraySingleObject (0x10)
      ObjectId: 1 (0x01)
    Length: 1 (0x1)
    MemberReference:
      RecordTypeEnum: MemberReference (0x09)
      IdRef: 2 (0x02)
    BinaryLibrary:
      RecordTypeEnum: BinaryLibrary (0x0C)
      LibraryId: 3 (0x03)
      LibraryString: LibrayString:DOJRemotingMetadata, Version=1.0.2621.26113, Culture=neutral, PublicKeyToken=null
    ClassWithMembersAndTypes:
      RecordTypeEnum: ClassWithMembersAndTypes (0x05)
      ObjectId: 2 (0x02)
      Name: DOJRemotingMetadata.MyData
      NumMembers: 4 (0x04)
        MemberNames:
          Data: Street
        MemberNames:
          Data: City
        MemberNames:
          Data: State
        MemberNames:
          Data: Zip
      BinaryTypeEnumA:
        String (0x01)
        String (0x01)
        String (0x01)
        String (0x01)
      LibraryId: 3 (0x03)
    BinaryObjectString:
      RecordTypeEnum: BinaryObjectString (0x06)
      ObjectId: 04 (0x04)
      Length: 17 (0x11)
      Value: One Microsoft Way
    BinaryObjectString:
      RecordTypeEnum: BinaryObjectString (0x06)
      ObjectId: 05 (0x04)
      Length: 7 (0x07)
      Value: Redmond
    BinaryObjectString:
      RecordTypeEnum: BinaryObjectString (0x06)
      ObjectId: 06 (0x04)
      Length: 2 (0x02)
      Value: WA
    BinaryObjectString:
      RecordTypeEnum: BinaryObjectString (0x06)
      ObjectId: 07 (0x04)
      Length: 5 (0x05)
      Value: 98054
  MessageEnd:
    RecordTypeEnum: MessageEnd (0x11)
```

The Server Type name, method name, and arguments are passed in a [BinaryMethodCall](#2231-binarymethodcall) structure. The MessageEnum record in BinaryMethodCall is used by the server to determine how to read the needed values. The ArgsInArray flag in this record is set to 1 because the argument passed to the method is not a **[Primitive Type](#primitive-type)**. Because the client is not passing any extra data in the CallContext of the request, the NoContext flag in the MessageEnum record is also set to 1. This information, coupled with the fact that the operation is of type Request, is used by the server to infer that the MethodName, Server Type, and Argument are embedded in the BinaryMethodCall record itself. Because the argument Address is passed in the callArray, CallArray contains an [ArraySingleObject](#2432-arraysingleobject) as the root element, and the first entry in the **[Array](#array)** is a MemberReference to the [ClassWithMembersAndTypes](#2321-classwithmembersandtypes) record that contains the input argument passed. The **[Library](#library)**, to which the ClassWithMembersAndTypes refers, appears next, and then the ClassWithMembersAndTypes record follows. All Members of Address are strings; therefore, the ClassWithMembersAndTypes record is followed by [BinaryObjectString](#257-binaryobjectstring) records for all of its Members.

After it invokes the method and is ready to return the result of that invocation, the server crafts a Response message and sends the Return Value ("Address received") in that message. The network capture of the response message is as follows.

```
0000  00 00 00 00 00 00 00 00 00 01 00 00 00 00 00 00 ................
0010  00 16 11 08 00 00 12 10 41 64 64 72 65 73 73 20 ........Address
0020  72 65 63 65 69 76 65 64 0B                      received.


Binary Serialization Format
  SerializationHeaderRecord:
    RecordTypeEnum: SerializedStreamHeader (0x00)
    TopId: 0 (0x0)
    HeaderId: 0 (0x0)
    MajorVersion: 1 (0x1)
    MinorVersion: 0 (0x0)
  BinaryMethodReturn:
    RecordTypeEnum: BinaryMethodReturn (0x16)
    MessageEnum: 00000811
      NoArgs:                   (...............................1) 
      ArgsInline:               (..............................0.)
      ArgsIsArray:              (.............................0..) 
      ArgsInArray:              (............................0...)
      NoContext:                (...........................1....) 
      ContextInline:            (..........................0.....)
      ContextInArray:           (.........................0......)
      MethodSignatureInArray:   (........................0.......)
      PropertyInArray:          (.......................0........)
      NoReturnValue:            (......................0.........)
      ReturnValueVoid:          (.....................0..........) 
      ReturnValueInline:        (....................1...........)
      ReturnValueInArray:       (...................0............) 
      ExceptionInArray:         (..................0.............)
      Reserved:                 (000000000000000000..............)
    ReturnValue:
      PrimitiveTypeEnum: String (0x12)
      Data: Address received
  MessageEnd:
    RecordTypeEnum: MessageEnd (0x11)
```

Because it is a response, the server sends back a message with the operation flag set to "Response". The return argument is enclosed in a "BinaryMethodResponse" enclosure. The following flags in the MessageEnum record of **BinaryMethodResponse** field are set to 1.

NoArgs: There are no output arguments.

NoContext: Similar to the client, the server is not sending any additional data in CallContext.

ReturnValueInline: Because the Return Value is a Primitive Type, it is contained in the [BinaryMethodReturn](#2233-binarymethodreturn) record.

___
# 4 Security Considerations

Some of the structures contain fields that specify size information of the data in the **[serialization stream](#serialization-stream)**. The type of the size that specifies fields is INT32 (as specified in [[MS-DTYP]](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp) section 2.2.22). The maximum value of these values can be as high as 0x7FFFFFFF. An implementation that consumes the stream either does not allocate memory based on the size information specified in the serialization stream, or ensures that the data in the serialization stream can be trusted.

The following table lists the structures with fields that specify size information.

| Type                 | Field       | Description                                                        |
|----------------------|-------------|--------------------------------------------------------------------|
| LengthPrefixedString | Length      | Size of the string                                                 |
| ArrayOfValueWithCode | Length      | Size of the **[Array](#array)**                                    |
| ClassInfo            | MemberCount | Number of Members                                                  |
| ArrayInfo            | Length      | Size of the Array                                                  |
| BinaryArray          | Rank        | Size of the Lengths and LowerBounds Arrays                         |
| BinaryArray          | Lengths     | Size of each dimension that would affect the net size of the Array |
| ObjectNullMultiple   | NullCount   | Number of **[Null Objects](#null-object)**                         |

De-serialization of the serialization stream results in creating instances of **[Remoting Types](#remoting-type)** whose information is provided in the serialization stream. It might be unsafe to create an instance of Remoting Types. An implementation protects against attacks where the serialization stream includes the unsafe Remoting Types. Such attacks can be mitigated by allowing the higher layer to configure a list of Remoting Types in an implementation-specific way and disallow **[de-serialization](#deserialize)** of any Remoting Type that is not in the list.

___
# 5 Appendix A: Product Behavior

The information in this specification is applicable to the following Microsoft products or supplemental software. References to product versions include updates to those products.

This document specifies version-specific details in the Microsoft .NET Framework. For information about which versions of .NET Framework are available in each released Windows product or as supplemental software, see [[MS-NETOD]](#122-informative-references) section 4.

The terms "earlier" and "later", when used with a product version, refer to either all preceding versions or all subsequent versions, respectively. The term "through" refers to the inclusive range of versions. Applicable Microsoft products are listed chronologically in this section.

- Microsoft .NET Framework 1.0
- Microsoft .NET Framework 2.0
- Microsoft .NET Framework 3.0
- Microsoft .NET Framework 3.5
- Microsoft .NET Framework 4.0
- Microsoft .NET Framework 4.5
- Microsoft .NET Framework 4.6
- Microsoft .NET Framework 4.7
- Microsoft .NET Framework 4.8

Exceptions, if any, are noted in this section. If an update version, service pack or Knowledge Base (KB) number appears with a product name, the behavior changed in that update. The new behavior also applies to subsequent updates unless otherwise specified. If a product edition appears with the product version, behavior is different in that product edition.

Unless otherwise specified, any statement of optional behavior in this specification that is prescribed using the terms "SHOULD" or "SHOULD NOT" implies product behavior in accordance with the SHOULD or SHOULD NOT prescription. Unless otherwise specified, the term "MAY" implies that the product does not follow the prescription.

[<1> Section 2.1.1.5](#2115-datetime): In .NET Framework 1.0 and .NET Framework 1.1, the value of **Kind** is always set to 0 when writing. On reading, the value is ignored and assumed to be 0.

[<2> Section 2.2.1.1](#2211-messageflags): The bit value GenericMethod is valid only with .NET Framework 2.0 and later versions.

[<3> Section 2.2.3.2](#2232-methodcallarray): This is present only in .NET Framework 2.0 and later versions.

[<4> Section 2.3.1.1](#2311-classinfo): Windows uses a single counter that counts from 1 to generate the [ObjectId](#2311-classinfo) in the ClassInfo, [ArrayInfo](#2421-arrayinfo), [BinaryObjectString](#257-binaryobjectstring), and [BinaryArray](#2431-binaryarray) records, and the [LibraryId](#262-binarylibrary) in the BinaryLibrary record. The maximum value is 2,147,483,647. If the object is of a **[Remoting Type](#remoting-type)** that cannot be referenced in Windows, the negative of the counter value is used.

[<5> Section 2.3.1.1](#2311-classinfo): In Windows, the order of the Members can vary for each occurrence of the record for a given class.

[<6> Section 2.4](#24-array-records): Windows uses [ObjectNullMultiple256](#256-objectnullmultiple256) if the number of sequential **[Null Objects](#null-object)** is 255 or fewer. Windows uses [ObjectNullMultiple](#255-objectnullmultiple) if the number of sequential Null Objects is greater than 255.

[<7> Section 2.4.2.1](#2421-arrayinfo): Windows uses a single counter that counts from 1 to generate the ObjectId in the ClassInfo, ArrayInfo, BinaryObjectString, and BinaryArray records, and the LibraryId in the BinaryLibrary record. The maximum value is 2,147,483,647.

[<8> Section 2.4.3.1](#2431-binaryarray): Windows uses a single counter that counts from 1 to generate the ObjectId in the ClassInfo, ArrayInfo, BinaryObjectString, and BinaryArray records, and the LibraryId in the BinaryLibrary record. The maximum value is 2,147,483,647.

[<9> Section 2.5.3](#253-memberreference): Windows places the record that defines the ID before or after the referencing record.

[<10> Section 2.5.7](#257-binaryobjectstring): Windows uses a single counter that counts from 1 to generate the ObjectId in the ClassInfo, ArrayInfo, BinaryObjectString, and BinaryArray records, and the LibraryId in the BinaryLibrary record. The maximum value is 2,147,483,647.

[<11> Section 2.6.2](#262-binarylibrary): Windows uses a single counter that counts from 1 to generate the ObjectId in the ClassInfo, ArrayInfo, BinaryObjectString, and BinaryArray records, and the LibraryId in the BinaryLibrary record. The maximum value is 2,147,483,647. 

___
# 6 Change Tracking

This section identifies changes that were made to this document since the last release. Changes are classified as Major, Minor, or None.

The revision class **Major** means that the technical content in the document was significantly revised. Major changes affect protocol interoperability or implementation. Examples of major changes are:

- A document revision that incorporates changes to interoperability requirements.

- A document revision that captures changes to protocol functionality.

The revision class **Minor** means that the meaning of the technical content was clarified. Minor changes do not affect protocol interoperability or implementation. Examples of minor changes are updates to clarify ambiguity at the sentence, paragraph, or table level.

The revision class **None** means that no new technical changes were introduced. Minor editorial and formatting changes may have been made, but the relevant technical content is identical to the last released version. The changes made to this document are listed in the following table.

<table>
  <thead>
    <th>Section</th>
    <th>Description</th>
    <th>Revision</br>class</th>
  </thead>
  <tbody>
    <tr>
      <td>2.1.2.2 BinaryTypeEnumeration</td>
      <td>9220 : Updated the term "string" to "String" in the Description column for the SystemClass and StringArray rows of the table.</td>
      <td>Major</td>
    </tr>
    <tr>
      <td>2.3.1.2 MemberTypeInfo</td>
      <td>9220 : Changed the description of the number of items in the  BinaryTypeEnums structure.</td>
      <td>Major</td>
    </tr>
    <tr>
      <td>5 Appendix A: Product Behavior</td>
      <td>Added .NET Framework v4.8 to the applicability list.</td>
      <td>Major</td>
    </tr>
  </tbody>
</table>

___
# 7 Index

## A
- **[Applicability](#15-applicability-statement)**
- **[ArgsInArray](#2211-messageflags)**
- **[ArgsInline](#2211-messageflags)**
- **[ArgsIsArray](#2211-messageflags)**
- **Array records**
  - **[Common definitions](#242-common-definitions)**
  - **[Data types](#211-common-data-types)**
  - **Enumerations** ([section 2.1.2](#212-enumerations), [section 2.4.1](#241-enumerations))
  - **[Overview](#24-array-records)**
  - **[Record definitions](#243-record-definitions)**
- **[ArrayInfo packet](#2421-arrayinfo)**
- **[ArrayOfValueWithCode packet](#2223-arrayofvaluewithcode)**
- **[ArraySingleObject](#2121-recordtypeenumeration)**
- **[ArraySingleObject packet](#2432-arraysingleobject)**
- **[ArraySinglePrimitive](#2121-recordtypeenumeration)**
- **[ArraySinglePrimitive packet](#2433-arraysingleprimitive)**
- **[ArraySingleString](#2121-recordtypeenumeration)**
- **[ArraySingleString packet](#2434-arraysinglestring)**

## B
- **[Binary records grammar](#27-binary-record-grammar)**
- **[BinaryArray](#2121-recordtypeenumeration)**
- **[BinaryArray packet](#2431-binaryarray)**
- **[BinaryLibrary](#2121-recordtypeenumeration)**
- **[BinaryLibrary packet](#262-binarylibrary)**
- **[BinaryMethodCall packet](#2231-binarymethodcall)**
- **[BinaryMethodReturn packet](#2233-binarymethodreturn)**
- **[BinaryObjectString](#2121-recordtypeenumeration)**
- **[BinaryObjectString packet](#257-binaryobjectstring)**
- **[Boolean](#2123-primitivetypeenumeration)**
- **[Byte](#2123-primitivetypeenumeration)**

## C
- **[Change tracking](#6-change-tracking)**
- **[Char](#2123-primitivetypeenumeration)**
- **[Char packet](#2111-char)**
- **[Class](#2122-binarytypeenumeration)**
- **Class records**
  - **[Data types](#211-common-data-types)**
  - **[Enumerations](#2121-recordtypeenumeration)**
  - **[Overview](#23-class-records)**
  - **[Record definitions](#232-record-definitions)**
  - **[Structures](#231-common-structures)**
- **[ClassInfo packet](#2311-classinfo)**
- **[ClassTypeInfo packet](#2118-classtypeinfo)**
- **[ClassWithId](#2121-recordtypeenumeration)**
- **[ClassWithId packet](#2325-classwithid)**
- **[ClassWithMembers](#2121-recordtypeenumeration)**
- **[ClassWithMembers packet](#2322-classwithmembers)**
- **[ClassWithMembersAndTypes](#2121-recordtypeenumeration)**
- **[ClassWithMembersAndTypes packet](#2321-classwithmembersandtypes)**
- **[Common data types](#211-common-data-types)**
- **[Common definitions](#21-common-definitions)**
- **[Common enumerations](#2121-recordtypeenumeration)**
- **[ContextInArray](#2211-messageflags)**
- **[ContextInline](#2211-messageflags)**

## D
- **[DateTime](#2123-primitivetypeenumeration)**
- **[DateTime packet](#2115-datetime)**
- **[Decimal](#2123-primitivetypeenumeration)**
- **[Decimal packet](#2117-decimal)**
- **[Double](#2123-primitivetypeenumeration)**
- **[Double packet](#2112-double)**

## E
- **[Examples](#3-structure-examples)**
- **[Examples - structure](#3-structure-examples)**
- **[ExceptionInArray](#2211-messageflags)**

## F
- **[Fields - vendor-extensible](#17-vendor-extensible-fields)**

## G
- **[GenericMethod](#2211-messageflags)**
- **[Glossary](#11-glossary)**
- **[Grammar - binary records](#27-binary-record-grammar)**

## I
- **[Implementer - security considerations](#4-security-considerations)**
- **[Informative references](#122-informative-references)**
- **[Int16](#2123-primitivetypeenumeration)**
- **[Int32](#2123-primitivetypeenumeration)**
- **[Int64](#2123-primitivetypeenumeration)**
- **[Introduction](#1-introduction)**

## J
- **[Jagged](#2411-binaryarraytypeenumeration)**
- **[JaggedOffset](#2411-binaryarraytypeenumeration)**

## L
- **[LengthPrefixedString packet](#2116-lengthprefixedstring)**
- **[Localization](#16-versioning-and-localization)**

## M
- **[Member reference records](#25-member-reference-records)**
  - **[Data types](#211-common-data-types)**
  - **[Enumerations](#2121-recordtypeenumeration)**
- **[MemberPrimitiveTyped](#2121-recordtypeenumeration)**
- **[MemberPrimitiveTyped packet](#251-memberprimitivetyped)**
- **[MemberPrimitiveUntyped packet](#252-memberprimitiveuntyped)**
- **[MemberReference](#2121-recordtypeenumeration)**
- **[MemberReference packet](#253-memberreference)**
- **[MemberTypeInfo packet](#2312-membertypeinfo)**
- **[MessageEnd](#2121-recordtypeenumeration)**
- **[MessageEnd packet](#263-messageend)**
- **Method invocation records**
  - **[Data types](#211-common-data-types)**
  - **Enumerations** ([section 2.1.2](#212-enumerations), [section 2.2.1](#221-enumerations))
  - **[Overview](#22-method-invocation-records)**
  - **[Record definitions](#223-record-definitions)**
  - **[Structures](#222-common-structures)**
- **[MethodCall](#2231-binarymethodcall)**
- **[MethodReturn](#2233-binarymethodreturn)**
- **[MethodSignatureInArray](#2211-messageflags)**

## N
- **[NoArgs](#2211-messageflags)**
- **[NoContext](#2211-messageflags)**
- **[NoReturnValue](#2211-messageflags)**
- **[Normative references](#121-normative-references)**
- **[Null](#2123-primitivetypeenumeration)**

## O
- **[Object](#2122-binarytypeenumeration)**
- **[ObjectArray](#2122-binarytypeenumeration)**
- **[ObjectNull](#2121-recordtypeenumeration)**
- **[ObjectNull packet](#254-objectnull)**
- **[ObjectNullMultiple](#2121-recordtypeenumeration)**
- **[ObjectNullMultiple packet](#255-objectnullmultiple)**
- **[ObjectNullMultiple256](#2121-recordtypeenumeration)**
- **[ObjectNullMultiple256 packet](#256-objectnullmultiple256)**
- **Other records** ([section 2.1.1](#211-common-data-types), [section 2.6](#26-other-records))
- **[Overview (synopsis)](#13-overview)**

## P
- **[Primitive](#2122-binarytypeenumeration)**
- **[PrimitiveArray](#2122-binarytypeenumeration)**
- **[Product behavior](#5-appendix-a-product-behavior)**
- **[PropertiesInArray](#2211-messageflags)**

## R
- **[Rectangular](#2411-binaryarraytypeenumeration)**
- **[RectangularOffset](#2411-binaryarraytypeenumeration)**
- **[References](#12-references)**
  - **[Informative](#122-informative-references)**
  - **[Normative](#121-normative-references)**
- **[Relationship to other protocols](#14-relationship-to-protocols-and-other-structures)**
- **[Relationship to protocols and other structures](#14-relationship-to-protocols-and-other-structures)**
- **[ReturnValueInArray](#2211-messageflags)**
- **[ReturnValueInline](#2211-messageflags)**
- **[ReturnValueVoid](#2211-messageflags)**

## S
- **[SByte](#2123-primitivetypeenumeration)**
- **[Security](#4-security-considerations)**
  - **[Implementer considerations](#4-security-considerations)**
- **[SerializationHeaderRecord packet](#261-serializationheaderrecord)**
- **[SerializedStreamHeader](#2121-recordtypeenumeration)**
- **Single** ([section 2.1.2.3](#2123-primitivetypeenumeration), [section 2.4.1.1](#2411-binaryarraytypeenumeration))
- **[Single packet](#2113-single)**
- **[SingleOffset](#2411-binaryarraytypeenumeration)**
- **String** ([section 2.1.2.2](#2122-binarytypeenumeration), [section 2.1.2.3](#2123-primitivetypeenumeration))
- **[StringArray](#2122-binarytypeenumeration)**
- **[StringValueWithCode packet](#2222-stringvaluewithcode)**
- **[Structure examples](#3-structure-examples)**
- **[Structures](#2-structures)**
- **[SystemClass](#2122-binarytypeenumeration)**
- **[SystemClassWithMembers](#2121-recordtypeenumeration)**
- **[SystemClassWithMembers packet](#2324-systemclasswithmembers)**
- **[SystemClassWithMembersAndTypes](#2121-recordtypeenumeration)**
- **[SystemClassWithMembersAndTypes packet](#2323-systemclasswithmembersandtypes)**

## T
- **[TimeSpan](#2123-primitivetypeenumeration)**
- **[TimeSpan packet](#2114-timespan)**
- **[Tracking changes](#6-change-tracking)**

## U
- **[UInt16](#2123-primitivetypeenumeration)**
- **[UInt32](#2123-primitivetypeenumeration)**
- **[UInt64](#2123-primitivetypeenumeration)**

## V
- **[ValueWithCode packet](#2221-valuewithcode)**
- **[Vendor-extensible fields](#17-vendor-extensible-fields)**
- **[Versioning](#16-versioning-and-localization)**

___
# Credits

- ## MS-NRBF documentation originally converted to Markdown with [marker](https://github.com/vikparuchuri/marker)
- ## Rewritten completely by hand by [Micah Benac](https://github.com/OfficialSirH)