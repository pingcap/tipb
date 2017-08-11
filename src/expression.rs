// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct FieldType {
    // message fields
    tp: ::std::option::Option<DataType>,
    flag: ::std::option::Option<u32>,
    flen: ::std::option::Option<i32>,
    decimal: ::std::option::Option<i32>,
    charset: ::protobuf::SingularField<::std::string::String>,
    collate: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FieldType {}

impl FieldType {
    pub fn new() -> FieldType {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FieldType {
        static mut instance: ::protobuf::lazy::Lazy<FieldType> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FieldType,
        };
        unsafe {
            instance.get(FieldType::new)
        }
    }

    // optional .tipb.DataType tp = 1;

    pub fn clear_tp(&mut self) {
        self.tp = ::std::option::Option::None;
    }

    pub fn has_tp(&self) -> bool {
        self.tp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tp(&mut self, v: DataType) {
        self.tp = ::std::option::Option::Some(v);
    }

    pub fn get_tp(&self) -> DataType {
        self.tp.unwrap_or(DataType::TypeDecimal)
    }

    fn get_tp_for_reflect(&self) -> &::std::option::Option<DataType> {
        &self.tp
    }

    fn mut_tp_for_reflect(&mut self) -> &mut ::std::option::Option<DataType> {
        &mut self.tp
    }

    // optional uint32 flag = 2;

    pub fn clear_flag(&mut self) {
        self.flag = ::std::option::Option::None;
    }

    pub fn has_flag(&self) -> bool {
        self.flag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flag(&mut self, v: u32) {
        self.flag = ::std::option::Option::Some(v);
    }

    pub fn get_flag(&self) -> u32 {
        self.flag.unwrap_or(0)
    }

    fn get_flag_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.flag
    }

    fn mut_flag_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.flag
    }

    // optional int32 flen = 3;

    pub fn clear_flen(&mut self) {
        self.flen = ::std::option::Option::None;
    }

    pub fn has_flen(&self) -> bool {
        self.flen.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flen(&mut self, v: i32) {
        self.flen = ::std::option::Option::Some(v);
    }

    pub fn get_flen(&self) -> i32 {
        self.flen.unwrap_or(0)
    }

    fn get_flen_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.flen
    }

    fn mut_flen_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.flen
    }

    // optional int32 decimal = 4;

    pub fn clear_decimal(&mut self) {
        self.decimal = ::std::option::Option::None;
    }

    pub fn has_decimal(&self) -> bool {
        self.decimal.is_some()
    }

    // Param is passed by value, moved
    pub fn set_decimal(&mut self, v: i32) {
        self.decimal = ::std::option::Option::Some(v);
    }

    pub fn get_decimal(&self) -> i32 {
        self.decimal.unwrap_or(0)
    }

    fn get_decimal_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.decimal
    }

    fn mut_decimal_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.decimal
    }

    // optional string charset = 5;

    pub fn clear_charset(&mut self) {
        self.charset.clear();
    }

    pub fn has_charset(&self) -> bool {
        self.charset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_charset(&mut self, v: ::std::string::String) {
        self.charset = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_charset(&mut self) -> &mut ::std::string::String {
        if self.charset.is_none() {
            self.charset.set_default();
        }
        self.charset.as_mut().unwrap()
    }

    // Take field
    pub fn take_charset(&mut self) -> ::std::string::String {
        self.charset.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_charset(&self) -> &str {
        match self.charset.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_charset_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.charset
    }

    fn mut_charset_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.charset
    }

    // optional string collate = 6;

    pub fn clear_collate(&mut self) {
        self.collate.clear();
    }

    pub fn has_collate(&self) -> bool {
        self.collate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collate(&mut self, v: ::std::string::String) {
        self.collate = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_collate(&mut self) -> &mut ::std::string::String {
        if self.collate.is_none() {
            self.collate.set_default();
        }
        self.collate.as_mut().unwrap()
    }

    // Take field
    pub fn take_collate(&mut self) -> ::std::string::String {
        self.collate.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_collate(&self) -> &str {
        match self.collate.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_collate_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.collate
    }

    fn mut_collate_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.collate
    }
}

impl ::protobuf::Message for FieldType {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.tp = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.flag = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.flen = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.decimal = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.charset)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.collate)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.tp {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.flag {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.flen {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.decimal {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.charset.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.collate.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tp {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.flag {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.flen {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.decimal {
            os.write_int32(4, v)?;
        }
        if let Some(ref v) = self.charset.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.collate.as_ref() {
            os.write_string(6, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FieldType {
    fn new() -> FieldType {
        FieldType::new()
    }

    fn descriptor_static(_: ::std::option::Option<FieldType>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DataType>>(
                    "tp",
                    FieldType::get_tp_for_reflect,
                    FieldType::mut_tp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flag",
                    FieldType::get_flag_for_reflect,
                    FieldType::mut_flag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "flen",
                    FieldType::get_flen_for_reflect,
                    FieldType::mut_flen_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "decimal",
                    FieldType::get_decimal_for_reflect,
                    FieldType::mut_decimal_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "charset",
                    FieldType::get_charset_for_reflect,
                    FieldType::mut_charset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "collate",
                    FieldType::get_collate_for_reflect,
                    FieldType::mut_collate_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FieldType>(
                    "FieldType",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FieldType {
    fn clear(&mut self) {
        self.clear_tp();
        self.clear_flag();
        self.clear_flen();
        self.clear_decimal();
        self.clear_charset();
        self.clear_collate();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FieldType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FieldType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Expr {
    // message fields
    tp: ::std::option::Option<ExprType>,
    val: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    children: ::protobuf::RepeatedField<Expr>,
    sig: ::std::option::Option<ScalarFuncSig>,
    field_type: ::protobuf::SingularPtrField<FieldType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Expr {}

impl Expr {
    pub fn new() -> Expr {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Expr {
        static mut instance: ::protobuf::lazy::Lazy<Expr> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Expr,
        };
        unsafe {
            instance.get(Expr::new)
        }
    }

    // optional .tipb.ExprType tp = 1;

    pub fn clear_tp(&mut self) {
        self.tp = ::std::option::Option::None;
    }

    pub fn has_tp(&self) -> bool {
        self.tp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tp(&mut self, v: ExprType) {
        self.tp = ::std::option::Option::Some(v);
    }

    pub fn get_tp(&self) -> ExprType {
        self.tp.unwrap_or(ExprType::Null)
    }

    fn get_tp_for_reflect(&self) -> &::std::option::Option<ExprType> {
        &self.tp
    }

    fn mut_tp_for_reflect(&mut self) -> &mut ::std::option::Option<ExprType> {
        &mut self.tp
    }

    // optional bytes val = 2;

    pub fn clear_val(&mut self) {
        self.val.clear();
    }

    pub fn has_val(&self) -> bool {
        self.val.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val(&mut self, v: ::std::vec::Vec<u8>) {
        self.val = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_val(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.val.is_none() {
            self.val.set_default();
        }
        self.val.as_mut().unwrap()
    }

    // Take field
    pub fn take_val(&mut self) -> ::std::vec::Vec<u8> {
        self.val.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_val(&self) -> &[u8] {
        match self.val.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_val_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.val
    }

    fn mut_val_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.val
    }

    // repeated .tipb.Expr children = 3;

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    // Param is passed by value, moved
    pub fn set_children(&mut self, v: ::protobuf::RepeatedField<Expr>) {
        self.children = v;
    }

    // Mutable pointer to the field.
    pub fn mut_children(&mut self) -> &mut ::protobuf::RepeatedField<Expr> {
        &mut self.children
    }

    // Take field
    pub fn take_children(&mut self) -> ::protobuf::RepeatedField<Expr> {
        ::std::mem::replace(&mut self.children, ::protobuf::RepeatedField::new())
    }

    pub fn get_children(&self) -> &[Expr] {
        &self.children
    }

    fn get_children_for_reflect(&self) -> &::protobuf::RepeatedField<Expr> {
        &self.children
    }

    fn mut_children_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Expr> {
        &mut self.children
    }

    // optional .tipb.ScalarFuncSig sig = 4;

    pub fn clear_sig(&mut self) {
        self.sig = ::std::option::Option::None;
    }

    pub fn has_sig(&self) -> bool {
        self.sig.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sig(&mut self, v: ScalarFuncSig) {
        self.sig = ::std::option::Option::Some(v);
    }

    pub fn get_sig(&self) -> ScalarFuncSig {
        self.sig.unwrap_or(ScalarFuncSig::CastIntAsInt)
    }

    fn get_sig_for_reflect(&self) -> &::std::option::Option<ScalarFuncSig> {
        &self.sig
    }

    fn mut_sig_for_reflect(&mut self) -> &mut ::std::option::Option<ScalarFuncSig> {
        &mut self.sig
    }

    // optional .tipb.FieldType field_type = 5;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: FieldType) {
        self.field_type = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut FieldType {
        if self.field_type.is_none() {
            self.field_type.set_default();
        }
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> FieldType {
        self.field_type.take().unwrap_or_else(|| FieldType::new())
    }

    pub fn get_field_type(&self) -> &FieldType {
        self.field_type.as_ref().unwrap_or_else(|| FieldType::default_instance())
    }

    fn get_field_type_for_reflect(&self) -> &::protobuf::SingularPtrField<FieldType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FieldType> {
        &mut self.field_type
    }
}

impl ::protobuf::Message for Expr {
    fn is_initialized(&self) -> bool {
        for v in &self.children {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.field_type {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.tp = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.val)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.children)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.sig = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.field_type)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.tp {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.val.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        for value in &self.children {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.sig {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        if let Some(ref v) = self.field_type.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tp {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.val.as_ref() {
            os.write_bytes(2, &v)?;
        }
        for v in &self.children {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.sig {
            os.write_enum(4, v.value())?;
        }
        if let Some(ref v) = self.field_type.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Expr {
    fn new() -> Expr {
        Expr::new()
    }

    fn descriptor_static(_: ::std::option::Option<Expr>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ExprType>>(
                    "tp",
                    Expr::get_tp_for_reflect,
                    Expr::mut_tp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "val",
                    Expr::get_val_for_reflect,
                    Expr::mut_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Expr>>(
                    "children",
                    Expr::get_children_for_reflect,
                    Expr::mut_children_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ScalarFuncSig>>(
                    "sig",
                    Expr::get_sig_for_reflect,
                    Expr::mut_sig_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FieldType>>(
                    "field_type",
                    Expr::get_field_type_for_reflect,
                    Expr::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Expr>(
                    "Expr",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Expr {
    fn clear(&mut self) {
        self.clear_tp();
        self.clear_val();
        self.clear_children();
        self.clear_sig();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Expr {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ByItem {
    // message fields
    expr: ::protobuf::SingularPtrField<Expr>,
    desc: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ByItem {}

impl ByItem {
    pub fn new() -> ByItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ByItem {
        static mut instance: ::protobuf::lazy::Lazy<ByItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ByItem,
        };
        unsafe {
            instance.get(ByItem::new)
        }
    }

    // optional .tipb.Expr expr = 1;

    pub fn clear_expr(&mut self) {
        self.expr.clear();
    }

    pub fn has_expr(&self) -> bool {
        self.expr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expr(&mut self, v: Expr) {
        self.expr = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_expr(&mut self) -> &mut Expr {
        if self.expr.is_none() {
            self.expr.set_default();
        }
        self.expr.as_mut().unwrap()
    }

    // Take field
    pub fn take_expr(&mut self) -> Expr {
        self.expr.take().unwrap_or_else(|| Expr::new())
    }

    pub fn get_expr(&self) -> &Expr {
        self.expr.as_ref().unwrap_or_else(|| Expr::default_instance())
    }

    fn get_expr_for_reflect(&self) -> &::protobuf::SingularPtrField<Expr> {
        &self.expr
    }

    fn mut_expr_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Expr> {
        &mut self.expr
    }

    // optional bool desc = 2;

    pub fn clear_desc(&mut self) {
        self.desc = ::std::option::Option::None;
    }

    pub fn has_desc(&self) -> bool {
        self.desc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_desc(&mut self, v: bool) {
        self.desc = ::std::option::Option::Some(v);
    }

    pub fn get_desc(&self) -> bool {
        self.desc.unwrap_or(false)
    }

    fn get_desc_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.desc
    }

    fn mut_desc_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.desc
    }
}

impl ::protobuf::Message for ByItem {
    fn is_initialized(&self) -> bool {
        for v in &self.expr {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.expr)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.desc = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.expr.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.desc {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.expr.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.desc {
            os.write_bool(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ByItem {
    fn new() -> ByItem {
        ByItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<ByItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Expr>>(
                    "expr",
                    ByItem::get_expr_for_reflect,
                    ByItem::mut_expr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "desc",
                    ByItem::get_desc_for_reflect,
                    ByItem::mut_desc_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ByItem>(
                    "ByItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ByItem {
    fn clear(&mut self) {
        self.clear_expr();
        self.clear_desc();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ByItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ByItem {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DataType {
    TypeDecimal = 0,
    TypeTiny = 1,
    TypeShort = 2,
    TypeLong = 3,
    TypeFloat = 4,
    TypeDouble = 5,
    TypeNull = 6,
    TypeTimestamp = 7,
    TypeLongLong = 8,
    TypeInt24 = 9,
    TypeDate = 10,
    TypeDuration = 11,
    TypeDatetime = 12,
    TypeYear = 13,
    TypeNewDate = 14,
    TypeVarchar = 15,
    TypeBit = 16,
    TypeJSON = 245,
    TypeNewDecimal = 246,
    TypeEnum = 247,
    TypeSet = 248,
    TypeTinyBlob = 249,
    TypeMediumBlob = 250,
    TypeLongBlob = 251,
    TypeBlob = 252,
    TypeVarString = 253,
    TypeString = 254,
    TypeGeometry = 255,
}

impl ::protobuf::ProtobufEnum for DataType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DataType> {
        match value {
            0 => ::std::option::Option::Some(DataType::TypeDecimal),
            1 => ::std::option::Option::Some(DataType::TypeTiny),
            2 => ::std::option::Option::Some(DataType::TypeShort),
            3 => ::std::option::Option::Some(DataType::TypeLong),
            4 => ::std::option::Option::Some(DataType::TypeFloat),
            5 => ::std::option::Option::Some(DataType::TypeDouble),
            6 => ::std::option::Option::Some(DataType::TypeNull),
            7 => ::std::option::Option::Some(DataType::TypeTimestamp),
            8 => ::std::option::Option::Some(DataType::TypeLongLong),
            9 => ::std::option::Option::Some(DataType::TypeInt24),
            10 => ::std::option::Option::Some(DataType::TypeDate),
            11 => ::std::option::Option::Some(DataType::TypeDuration),
            12 => ::std::option::Option::Some(DataType::TypeDatetime),
            13 => ::std::option::Option::Some(DataType::TypeYear),
            14 => ::std::option::Option::Some(DataType::TypeNewDate),
            15 => ::std::option::Option::Some(DataType::TypeVarchar),
            16 => ::std::option::Option::Some(DataType::TypeBit),
            245 => ::std::option::Option::Some(DataType::TypeJSON),
            246 => ::std::option::Option::Some(DataType::TypeNewDecimal),
            247 => ::std::option::Option::Some(DataType::TypeEnum),
            248 => ::std::option::Option::Some(DataType::TypeSet),
            249 => ::std::option::Option::Some(DataType::TypeTinyBlob),
            250 => ::std::option::Option::Some(DataType::TypeMediumBlob),
            251 => ::std::option::Option::Some(DataType::TypeLongBlob),
            252 => ::std::option::Option::Some(DataType::TypeBlob),
            253 => ::std::option::Option::Some(DataType::TypeVarString),
            254 => ::std::option::Option::Some(DataType::TypeString),
            255 => ::std::option::Option::Some(DataType::TypeGeometry),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DataType] = &[
            DataType::TypeDecimal,
            DataType::TypeTiny,
            DataType::TypeShort,
            DataType::TypeLong,
            DataType::TypeFloat,
            DataType::TypeDouble,
            DataType::TypeNull,
            DataType::TypeTimestamp,
            DataType::TypeLongLong,
            DataType::TypeInt24,
            DataType::TypeDate,
            DataType::TypeDuration,
            DataType::TypeDatetime,
            DataType::TypeYear,
            DataType::TypeNewDate,
            DataType::TypeVarchar,
            DataType::TypeBit,
            DataType::TypeJSON,
            DataType::TypeNewDecimal,
            DataType::TypeEnum,
            DataType::TypeSet,
            DataType::TypeTinyBlob,
            DataType::TypeMediumBlob,
            DataType::TypeLongBlob,
            DataType::TypeBlob,
            DataType::TypeVarString,
            DataType::TypeString,
            DataType::TypeGeometry,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DataType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DataType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DataType {
}

impl ::protobuf::reflect::ProtobufValue for DataType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ExprType {
    Null = 0,
    Int64 = 1,
    Uint64 = 2,
    Float32 = 3,
    Float64 = 4,
    String = 5,
    Bytes = 6,
    MysqlBit = 101,
    MysqlDecimal = 102,
    MysqlDuration = 103,
    MysqlEnum = 104,
    MysqlHex = 105,
    MysqlSet = 106,
    MysqlTime = 107,
    ValueList = 151,
    ColumnRef = 201,
    Not = 1001,
    Neg = 1002,
    BitNeg = 1003,
    LT = 2001,
    LE = 2002,
    EQ = 2003,
    NE = 2004,
    GE = 2005,
    GT = 2006,
    NullEQ = 2007,
    BitAnd = 2101,
    BitOr = 2102,
    BitXor = 2103,
    LeftShift = 2104,
    RighShift = 2105,
    Plus = 2201,
    Minus = 2202,
    Mul = 2203,
    Div = 2204,
    IntDiv = 2205,
    Mod = 2206,
    And = 2301,
    Or = 2302,
    Xor = 2303,
    Count = 3001,
    Sum = 3002,
    Avg = 3003,
    Min = 3004,
    Max = 3005,
    First = 3006,
    GroupConcat = 3007,
    Abs = 3101,
    Pow = 3102,
    Round = 3103,
    Concat = 3201,
    ConcatWS = 3202,
    Left = 3203,
    Length = 3204,
    Lower = 3205,
    Repeat = 3206,
    Replace = 3207,
    Upper = 3208,
    Strcmp = 3209,
    Convert = 3210,
    Cast = 3211,
    Substring = 3212,
    SubstringIndex = 3213,
    Locate = 3214,
    Trim = 3215,
    If = 3301,
    NullIf = 3302,
    IfNull = 3303,
    Date = 3401,
    DateAdd = 3402,
    DateSub = 3403,
    Year = 3411,
    YearWeek = 3412,
    Month = 3421,
    Week = 3431,
    Weekday = 3432,
    WeekOfYear = 3433,
    Day = 3441,
    DayName = 3442,
    DayOfYear = 3443,
    DayOfMonth = 3444,
    DayOfWeek = 3445,
    Hour = 3451,
    Minute = 3452,
    Second = 3453,
    Microsecond = 3454,
    Extract = 3461,
    Coalesce = 3501,
    Greatest = 3502,
    Least = 3503,
    JsonExtract = 3601,
    JsonType = 3602,
    JsonArray = 3603,
    JsonObject = 3604,
    JsonMerge = 3605,
    JsonValid = 3606,
    JsonSet = 3607,
    JsonInsert = 3608,
    JsonReplace = 3609,
    JsonRemove = 3610,
    JsonContains = 3611,
    JsonUnquote = 3612,
    JsonContainsPath = 3613,
    In = 4001,
    IsTruth = 4002,
    IsNull = 4003,
    ExprRow = 4004,
    Like = 4005,
    RLike = 4006,
    Case = 4007,
    ScalarFunc = 10000,
}

impl ::protobuf::ProtobufEnum for ExprType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ExprType> {
        match value {
            0 => ::std::option::Option::Some(ExprType::Null),
            1 => ::std::option::Option::Some(ExprType::Int64),
            2 => ::std::option::Option::Some(ExprType::Uint64),
            3 => ::std::option::Option::Some(ExprType::Float32),
            4 => ::std::option::Option::Some(ExprType::Float64),
            5 => ::std::option::Option::Some(ExprType::String),
            6 => ::std::option::Option::Some(ExprType::Bytes),
            101 => ::std::option::Option::Some(ExprType::MysqlBit),
            102 => ::std::option::Option::Some(ExprType::MysqlDecimal),
            103 => ::std::option::Option::Some(ExprType::MysqlDuration),
            104 => ::std::option::Option::Some(ExprType::MysqlEnum),
            105 => ::std::option::Option::Some(ExprType::MysqlHex),
            106 => ::std::option::Option::Some(ExprType::MysqlSet),
            107 => ::std::option::Option::Some(ExprType::MysqlTime),
            151 => ::std::option::Option::Some(ExprType::ValueList),
            201 => ::std::option::Option::Some(ExprType::ColumnRef),
            1001 => ::std::option::Option::Some(ExprType::Not),
            1002 => ::std::option::Option::Some(ExprType::Neg),
            1003 => ::std::option::Option::Some(ExprType::BitNeg),
            2001 => ::std::option::Option::Some(ExprType::LT),
            2002 => ::std::option::Option::Some(ExprType::LE),
            2003 => ::std::option::Option::Some(ExprType::EQ),
            2004 => ::std::option::Option::Some(ExprType::NE),
            2005 => ::std::option::Option::Some(ExprType::GE),
            2006 => ::std::option::Option::Some(ExprType::GT),
            2007 => ::std::option::Option::Some(ExprType::NullEQ),
            2101 => ::std::option::Option::Some(ExprType::BitAnd),
            2102 => ::std::option::Option::Some(ExprType::BitOr),
            2103 => ::std::option::Option::Some(ExprType::BitXor),
            2104 => ::std::option::Option::Some(ExprType::LeftShift),
            2105 => ::std::option::Option::Some(ExprType::RighShift),
            2201 => ::std::option::Option::Some(ExprType::Plus),
            2202 => ::std::option::Option::Some(ExprType::Minus),
            2203 => ::std::option::Option::Some(ExprType::Mul),
            2204 => ::std::option::Option::Some(ExprType::Div),
            2205 => ::std::option::Option::Some(ExprType::IntDiv),
            2206 => ::std::option::Option::Some(ExprType::Mod),
            2301 => ::std::option::Option::Some(ExprType::And),
            2302 => ::std::option::Option::Some(ExprType::Or),
            2303 => ::std::option::Option::Some(ExprType::Xor),
            3001 => ::std::option::Option::Some(ExprType::Count),
            3002 => ::std::option::Option::Some(ExprType::Sum),
            3003 => ::std::option::Option::Some(ExprType::Avg),
            3004 => ::std::option::Option::Some(ExprType::Min),
            3005 => ::std::option::Option::Some(ExprType::Max),
            3006 => ::std::option::Option::Some(ExprType::First),
            3007 => ::std::option::Option::Some(ExprType::GroupConcat),
            3101 => ::std::option::Option::Some(ExprType::Abs),
            3102 => ::std::option::Option::Some(ExprType::Pow),
            3103 => ::std::option::Option::Some(ExprType::Round),
            3201 => ::std::option::Option::Some(ExprType::Concat),
            3202 => ::std::option::Option::Some(ExprType::ConcatWS),
            3203 => ::std::option::Option::Some(ExprType::Left),
            3204 => ::std::option::Option::Some(ExprType::Length),
            3205 => ::std::option::Option::Some(ExprType::Lower),
            3206 => ::std::option::Option::Some(ExprType::Repeat),
            3207 => ::std::option::Option::Some(ExprType::Replace),
            3208 => ::std::option::Option::Some(ExprType::Upper),
            3209 => ::std::option::Option::Some(ExprType::Strcmp),
            3210 => ::std::option::Option::Some(ExprType::Convert),
            3211 => ::std::option::Option::Some(ExprType::Cast),
            3212 => ::std::option::Option::Some(ExprType::Substring),
            3213 => ::std::option::Option::Some(ExprType::SubstringIndex),
            3214 => ::std::option::Option::Some(ExprType::Locate),
            3215 => ::std::option::Option::Some(ExprType::Trim),
            3301 => ::std::option::Option::Some(ExprType::If),
            3302 => ::std::option::Option::Some(ExprType::NullIf),
            3303 => ::std::option::Option::Some(ExprType::IfNull),
            3401 => ::std::option::Option::Some(ExprType::Date),
            3402 => ::std::option::Option::Some(ExprType::DateAdd),
            3403 => ::std::option::Option::Some(ExprType::DateSub),
            3411 => ::std::option::Option::Some(ExprType::Year),
            3412 => ::std::option::Option::Some(ExprType::YearWeek),
            3421 => ::std::option::Option::Some(ExprType::Month),
            3431 => ::std::option::Option::Some(ExprType::Week),
            3432 => ::std::option::Option::Some(ExprType::Weekday),
            3433 => ::std::option::Option::Some(ExprType::WeekOfYear),
            3441 => ::std::option::Option::Some(ExprType::Day),
            3442 => ::std::option::Option::Some(ExprType::DayName),
            3443 => ::std::option::Option::Some(ExprType::DayOfYear),
            3444 => ::std::option::Option::Some(ExprType::DayOfMonth),
            3445 => ::std::option::Option::Some(ExprType::DayOfWeek),
            3451 => ::std::option::Option::Some(ExprType::Hour),
            3452 => ::std::option::Option::Some(ExprType::Minute),
            3453 => ::std::option::Option::Some(ExprType::Second),
            3454 => ::std::option::Option::Some(ExprType::Microsecond),
            3461 => ::std::option::Option::Some(ExprType::Extract),
            3501 => ::std::option::Option::Some(ExprType::Coalesce),
            3502 => ::std::option::Option::Some(ExprType::Greatest),
            3503 => ::std::option::Option::Some(ExprType::Least),
            3601 => ::std::option::Option::Some(ExprType::JsonExtract),
            3602 => ::std::option::Option::Some(ExprType::JsonType),
            3603 => ::std::option::Option::Some(ExprType::JsonArray),
            3604 => ::std::option::Option::Some(ExprType::JsonObject),
            3605 => ::std::option::Option::Some(ExprType::JsonMerge),
            3606 => ::std::option::Option::Some(ExprType::JsonValid),
            3607 => ::std::option::Option::Some(ExprType::JsonSet),
            3608 => ::std::option::Option::Some(ExprType::JsonInsert),
            3609 => ::std::option::Option::Some(ExprType::JsonReplace),
            3610 => ::std::option::Option::Some(ExprType::JsonRemove),
            3611 => ::std::option::Option::Some(ExprType::JsonContains),
            3612 => ::std::option::Option::Some(ExprType::JsonUnquote),
            3613 => ::std::option::Option::Some(ExprType::JsonContainsPath),
            4001 => ::std::option::Option::Some(ExprType::In),
            4002 => ::std::option::Option::Some(ExprType::IsTruth),
            4003 => ::std::option::Option::Some(ExprType::IsNull),
            4004 => ::std::option::Option::Some(ExprType::ExprRow),
            4005 => ::std::option::Option::Some(ExprType::Like),
            4006 => ::std::option::Option::Some(ExprType::RLike),
            4007 => ::std::option::Option::Some(ExprType::Case),
            10000 => ::std::option::Option::Some(ExprType::ScalarFunc),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ExprType] = &[
            ExprType::Null,
            ExprType::Int64,
            ExprType::Uint64,
            ExprType::Float32,
            ExprType::Float64,
            ExprType::String,
            ExprType::Bytes,
            ExprType::MysqlBit,
            ExprType::MysqlDecimal,
            ExprType::MysqlDuration,
            ExprType::MysqlEnum,
            ExprType::MysqlHex,
            ExprType::MysqlSet,
            ExprType::MysqlTime,
            ExprType::ValueList,
            ExprType::ColumnRef,
            ExprType::Not,
            ExprType::Neg,
            ExprType::BitNeg,
            ExprType::LT,
            ExprType::LE,
            ExprType::EQ,
            ExprType::NE,
            ExprType::GE,
            ExprType::GT,
            ExprType::NullEQ,
            ExprType::BitAnd,
            ExprType::BitOr,
            ExprType::BitXor,
            ExprType::LeftShift,
            ExprType::RighShift,
            ExprType::Plus,
            ExprType::Minus,
            ExprType::Mul,
            ExprType::Div,
            ExprType::IntDiv,
            ExprType::Mod,
            ExprType::And,
            ExprType::Or,
            ExprType::Xor,
            ExprType::Count,
            ExprType::Sum,
            ExprType::Avg,
            ExprType::Min,
            ExprType::Max,
            ExprType::First,
            ExprType::GroupConcat,
            ExprType::Abs,
            ExprType::Pow,
            ExprType::Round,
            ExprType::Concat,
            ExprType::ConcatWS,
            ExprType::Left,
            ExprType::Length,
            ExprType::Lower,
            ExprType::Repeat,
            ExprType::Replace,
            ExprType::Upper,
            ExprType::Strcmp,
            ExprType::Convert,
            ExprType::Cast,
            ExprType::Substring,
            ExprType::SubstringIndex,
            ExprType::Locate,
            ExprType::Trim,
            ExprType::If,
            ExprType::NullIf,
            ExprType::IfNull,
            ExprType::Date,
            ExprType::DateAdd,
            ExprType::DateSub,
            ExprType::Year,
            ExprType::YearWeek,
            ExprType::Month,
            ExprType::Week,
            ExprType::Weekday,
            ExprType::WeekOfYear,
            ExprType::Day,
            ExprType::DayName,
            ExprType::DayOfYear,
            ExprType::DayOfMonth,
            ExprType::DayOfWeek,
            ExprType::Hour,
            ExprType::Minute,
            ExprType::Second,
            ExprType::Microsecond,
            ExprType::Extract,
            ExprType::Coalesce,
            ExprType::Greatest,
            ExprType::Least,
            ExprType::JsonExtract,
            ExprType::JsonType,
            ExprType::JsonArray,
            ExprType::JsonObject,
            ExprType::JsonMerge,
            ExprType::JsonValid,
            ExprType::JsonSet,
            ExprType::JsonInsert,
            ExprType::JsonReplace,
            ExprType::JsonRemove,
            ExprType::JsonContains,
            ExprType::JsonUnquote,
            ExprType::JsonContainsPath,
            ExprType::In,
            ExprType::IsTruth,
            ExprType::IsNull,
            ExprType::ExprRow,
            ExprType::Like,
            ExprType::RLike,
            ExprType::Case,
            ExprType::ScalarFunc,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ExprType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ExprType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ExprType {
}

impl ::protobuf::reflect::ProtobufValue for ExprType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ScalarFuncSig {
    CastIntAsInt = 0,
    CastIntAsReal = 1,
    CastIntAsString = 2,
    CastIntAsDecimal = 3,
    CastIntAsTime = 4,
    CastIntAsDuration = 5,
    CastRealAsInt = 10,
    CastRealAsReal = 11,
    CastRealAsString = 12,
    CastRealAsDecimal = 13,
    CastRealAsTime = 14,
    CastRealAsDuration = 15,
    CastDecimalAsInt = 20,
    CastDecimalAsReal = 21,
    CastDecimalAsString = 22,
    CastDecimalAsDecimal = 23,
    CastDecimalAsTime = 24,
    CastDecimalAsDuration = 25,
    CastStringAsInt = 30,
    CastStringAsReal = 31,
    CastStringAsString = 32,
    CastStringAsDecimal = 33,
    CastStringAsTime = 34,
    CastStringAsDuration = 35,
    CastTimeAsInt = 40,
    CastTimeAsReal = 41,
    CastTimeAsString = 42,
    CastTimeAsDecimal = 43,
    CastTimeAsTime = 44,
    CastTimeAsDuration = 45,
    CastDurationAsInt = 50,
    CastDurationAsReal = 51,
    CastDurationAsString = 52,
    CastDurationAsDecimal = 53,
    CastDurationAsTime = 54,
    CastDurationAsDuration = 55,
    LTInt = 100,
    LTReal = 101,
    LTDecimal = 102,
    LTString = 103,
    LTTime = 104,
    LTDuration = 105,
    LTJson = 106,
    LEInt = 110,
    LEReal = 111,
    LEDecimal = 112,
    LEString = 113,
    LETime = 114,
    LEDuration = 115,
    LEJson = 116,
    GTInt = 120,
    GTReal = 121,
    GTDecimal = 122,
    GTString = 123,
    GTTime = 124,
    GTDuration = 125,
    GTJson = 126,
    GEInt = 130,
    GEReal = 131,
    GEDecimal = 132,
    GEString = 133,
    GETime = 134,
    GEDuration = 135,
    GEJson = 136,
    EQInt = 140,
    EQReal = 141,
    EQDecimal = 142,
    EQString = 143,
    EQTime = 144,
    EQDuration = 145,
    EQJson = 146,
    NEInt = 150,
    NEReal = 151,
    NEDecimal = 152,
    NEString = 153,
    NETime = 154,
    NEDuration = 155,
    NEJson = 156,
    NullEQInt = 160,
    NullEQReal = 161,
    NullEQDecimal = 162,
    NullEQString = 163,
    NullEQTime = 164,
    NullEQDuration = 165,
    NullEQJson = 166,
    AbsInt = 2101,
    AbsReal = 2102,
    AbsDecimal = 2103,
    CeilInt = 2104,
    CeilReal = 2105,
    FloorInt = 2106,
    FloorReal = 2107,
}

impl ::protobuf::ProtobufEnum for ScalarFuncSig {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ScalarFuncSig> {
        match value {
            0 => ::std::option::Option::Some(ScalarFuncSig::CastIntAsInt),
            1 => ::std::option::Option::Some(ScalarFuncSig::CastIntAsReal),
            2 => ::std::option::Option::Some(ScalarFuncSig::CastIntAsString),
            3 => ::std::option::Option::Some(ScalarFuncSig::CastIntAsDecimal),
            4 => ::std::option::Option::Some(ScalarFuncSig::CastIntAsTime),
            5 => ::std::option::Option::Some(ScalarFuncSig::CastIntAsDuration),
            10 => ::std::option::Option::Some(ScalarFuncSig::CastRealAsInt),
            11 => ::std::option::Option::Some(ScalarFuncSig::CastRealAsReal),
            12 => ::std::option::Option::Some(ScalarFuncSig::CastRealAsString),
            13 => ::std::option::Option::Some(ScalarFuncSig::CastRealAsDecimal),
            14 => ::std::option::Option::Some(ScalarFuncSig::CastRealAsTime),
            15 => ::std::option::Option::Some(ScalarFuncSig::CastRealAsDuration),
            20 => ::std::option::Option::Some(ScalarFuncSig::CastDecimalAsInt),
            21 => ::std::option::Option::Some(ScalarFuncSig::CastDecimalAsReal),
            22 => ::std::option::Option::Some(ScalarFuncSig::CastDecimalAsString),
            23 => ::std::option::Option::Some(ScalarFuncSig::CastDecimalAsDecimal),
            24 => ::std::option::Option::Some(ScalarFuncSig::CastDecimalAsTime),
            25 => ::std::option::Option::Some(ScalarFuncSig::CastDecimalAsDuration),
            30 => ::std::option::Option::Some(ScalarFuncSig::CastStringAsInt),
            31 => ::std::option::Option::Some(ScalarFuncSig::CastStringAsReal),
            32 => ::std::option::Option::Some(ScalarFuncSig::CastStringAsString),
            33 => ::std::option::Option::Some(ScalarFuncSig::CastStringAsDecimal),
            34 => ::std::option::Option::Some(ScalarFuncSig::CastStringAsTime),
            35 => ::std::option::Option::Some(ScalarFuncSig::CastStringAsDuration),
            40 => ::std::option::Option::Some(ScalarFuncSig::CastTimeAsInt),
            41 => ::std::option::Option::Some(ScalarFuncSig::CastTimeAsReal),
            42 => ::std::option::Option::Some(ScalarFuncSig::CastTimeAsString),
            43 => ::std::option::Option::Some(ScalarFuncSig::CastTimeAsDecimal),
            44 => ::std::option::Option::Some(ScalarFuncSig::CastTimeAsTime),
            45 => ::std::option::Option::Some(ScalarFuncSig::CastTimeAsDuration),
            50 => ::std::option::Option::Some(ScalarFuncSig::CastDurationAsInt),
            51 => ::std::option::Option::Some(ScalarFuncSig::CastDurationAsReal),
            52 => ::std::option::Option::Some(ScalarFuncSig::CastDurationAsString),
            53 => ::std::option::Option::Some(ScalarFuncSig::CastDurationAsDecimal),
            54 => ::std::option::Option::Some(ScalarFuncSig::CastDurationAsTime),
            55 => ::std::option::Option::Some(ScalarFuncSig::CastDurationAsDuration),
            100 => ::std::option::Option::Some(ScalarFuncSig::LTInt),
            101 => ::std::option::Option::Some(ScalarFuncSig::LTReal),
            102 => ::std::option::Option::Some(ScalarFuncSig::LTDecimal),
            103 => ::std::option::Option::Some(ScalarFuncSig::LTString),
            104 => ::std::option::Option::Some(ScalarFuncSig::LTTime),
            105 => ::std::option::Option::Some(ScalarFuncSig::LTDuration),
            106 => ::std::option::Option::Some(ScalarFuncSig::LTJson),
            110 => ::std::option::Option::Some(ScalarFuncSig::LEInt),
            111 => ::std::option::Option::Some(ScalarFuncSig::LEReal),
            112 => ::std::option::Option::Some(ScalarFuncSig::LEDecimal),
            113 => ::std::option::Option::Some(ScalarFuncSig::LEString),
            114 => ::std::option::Option::Some(ScalarFuncSig::LETime),
            115 => ::std::option::Option::Some(ScalarFuncSig::LEDuration),
            116 => ::std::option::Option::Some(ScalarFuncSig::LEJson),
            120 => ::std::option::Option::Some(ScalarFuncSig::GTInt),
            121 => ::std::option::Option::Some(ScalarFuncSig::GTReal),
            122 => ::std::option::Option::Some(ScalarFuncSig::GTDecimal),
            123 => ::std::option::Option::Some(ScalarFuncSig::GTString),
            124 => ::std::option::Option::Some(ScalarFuncSig::GTTime),
            125 => ::std::option::Option::Some(ScalarFuncSig::GTDuration),
            126 => ::std::option::Option::Some(ScalarFuncSig::GTJson),
            130 => ::std::option::Option::Some(ScalarFuncSig::GEInt),
            131 => ::std::option::Option::Some(ScalarFuncSig::GEReal),
            132 => ::std::option::Option::Some(ScalarFuncSig::GEDecimal),
            133 => ::std::option::Option::Some(ScalarFuncSig::GEString),
            134 => ::std::option::Option::Some(ScalarFuncSig::GETime),
            135 => ::std::option::Option::Some(ScalarFuncSig::GEDuration),
            136 => ::std::option::Option::Some(ScalarFuncSig::GEJson),
            140 => ::std::option::Option::Some(ScalarFuncSig::EQInt),
            141 => ::std::option::Option::Some(ScalarFuncSig::EQReal),
            142 => ::std::option::Option::Some(ScalarFuncSig::EQDecimal),
            143 => ::std::option::Option::Some(ScalarFuncSig::EQString),
            144 => ::std::option::Option::Some(ScalarFuncSig::EQTime),
            145 => ::std::option::Option::Some(ScalarFuncSig::EQDuration),
            146 => ::std::option::Option::Some(ScalarFuncSig::EQJson),
            150 => ::std::option::Option::Some(ScalarFuncSig::NEInt),
            151 => ::std::option::Option::Some(ScalarFuncSig::NEReal),
            152 => ::std::option::Option::Some(ScalarFuncSig::NEDecimal),
            153 => ::std::option::Option::Some(ScalarFuncSig::NEString),
            154 => ::std::option::Option::Some(ScalarFuncSig::NETime),
            155 => ::std::option::Option::Some(ScalarFuncSig::NEDuration),
            156 => ::std::option::Option::Some(ScalarFuncSig::NEJson),
            160 => ::std::option::Option::Some(ScalarFuncSig::NullEQInt),
            161 => ::std::option::Option::Some(ScalarFuncSig::NullEQReal),
            162 => ::std::option::Option::Some(ScalarFuncSig::NullEQDecimal),
            163 => ::std::option::Option::Some(ScalarFuncSig::NullEQString),
            164 => ::std::option::Option::Some(ScalarFuncSig::NullEQTime),
            165 => ::std::option::Option::Some(ScalarFuncSig::NullEQDuration),
            166 => ::std::option::Option::Some(ScalarFuncSig::NullEQJson),
            2101 => ::std::option::Option::Some(ScalarFuncSig::AbsInt),
            2102 => ::std::option::Option::Some(ScalarFuncSig::AbsReal),
            2103 => ::std::option::Option::Some(ScalarFuncSig::AbsDecimal),
            2104 => ::std::option::Option::Some(ScalarFuncSig::CeilInt),
            2105 => ::std::option::Option::Some(ScalarFuncSig::CeilReal),
            2106 => ::std::option::Option::Some(ScalarFuncSig::FloorInt),
            2107 => ::std::option::Option::Some(ScalarFuncSig::FloorReal),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ScalarFuncSig] = &[
            ScalarFuncSig::CastIntAsInt,
            ScalarFuncSig::CastIntAsReal,
            ScalarFuncSig::CastIntAsString,
            ScalarFuncSig::CastIntAsDecimal,
            ScalarFuncSig::CastIntAsTime,
            ScalarFuncSig::CastIntAsDuration,
            ScalarFuncSig::CastRealAsInt,
            ScalarFuncSig::CastRealAsReal,
            ScalarFuncSig::CastRealAsString,
            ScalarFuncSig::CastRealAsDecimal,
            ScalarFuncSig::CastRealAsTime,
            ScalarFuncSig::CastRealAsDuration,
            ScalarFuncSig::CastDecimalAsInt,
            ScalarFuncSig::CastDecimalAsReal,
            ScalarFuncSig::CastDecimalAsString,
            ScalarFuncSig::CastDecimalAsDecimal,
            ScalarFuncSig::CastDecimalAsTime,
            ScalarFuncSig::CastDecimalAsDuration,
            ScalarFuncSig::CastStringAsInt,
            ScalarFuncSig::CastStringAsReal,
            ScalarFuncSig::CastStringAsString,
            ScalarFuncSig::CastStringAsDecimal,
            ScalarFuncSig::CastStringAsTime,
            ScalarFuncSig::CastStringAsDuration,
            ScalarFuncSig::CastTimeAsInt,
            ScalarFuncSig::CastTimeAsReal,
            ScalarFuncSig::CastTimeAsString,
            ScalarFuncSig::CastTimeAsDecimal,
            ScalarFuncSig::CastTimeAsTime,
            ScalarFuncSig::CastTimeAsDuration,
            ScalarFuncSig::CastDurationAsInt,
            ScalarFuncSig::CastDurationAsReal,
            ScalarFuncSig::CastDurationAsString,
            ScalarFuncSig::CastDurationAsDecimal,
            ScalarFuncSig::CastDurationAsTime,
            ScalarFuncSig::CastDurationAsDuration,
            ScalarFuncSig::LTInt,
            ScalarFuncSig::LTReal,
            ScalarFuncSig::LTDecimal,
            ScalarFuncSig::LTString,
            ScalarFuncSig::LTTime,
            ScalarFuncSig::LTDuration,
            ScalarFuncSig::LTJson,
            ScalarFuncSig::LEInt,
            ScalarFuncSig::LEReal,
            ScalarFuncSig::LEDecimal,
            ScalarFuncSig::LEString,
            ScalarFuncSig::LETime,
            ScalarFuncSig::LEDuration,
            ScalarFuncSig::LEJson,
            ScalarFuncSig::GTInt,
            ScalarFuncSig::GTReal,
            ScalarFuncSig::GTDecimal,
            ScalarFuncSig::GTString,
            ScalarFuncSig::GTTime,
            ScalarFuncSig::GTDuration,
            ScalarFuncSig::GTJson,
            ScalarFuncSig::GEInt,
            ScalarFuncSig::GEReal,
            ScalarFuncSig::GEDecimal,
            ScalarFuncSig::GEString,
            ScalarFuncSig::GETime,
            ScalarFuncSig::GEDuration,
            ScalarFuncSig::GEJson,
            ScalarFuncSig::EQInt,
            ScalarFuncSig::EQReal,
            ScalarFuncSig::EQDecimal,
            ScalarFuncSig::EQString,
            ScalarFuncSig::EQTime,
            ScalarFuncSig::EQDuration,
            ScalarFuncSig::EQJson,
            ScalarFuncSig::NEInt,
            ScalarFuncSig::NEReal,
            ScalarFuncSig::NEDecimal,
            ScalarFuncSig::NEString,
            ScalarFuncSig::NETime,
            ScalarFuncSig::NEDuration,
            ScalarFuncSig::NEJson,
            ScalarFuncSig::NullEQInt,
            ScalarFuncSig::NullEQReal,
            ScalarFuncSig::NullEQDecimal,
            ScalarFuncSig::NullEQString,
            ScalarFuncSig::NullEQTime,
            ScalarFuncSig::NullEQDuration,
            ScalarFuncSig::NullEQJson,
            ScalarFuncSig::AbsInt,
            ScalarFuncSig::AbsReal,
            ScalarFuncSig::AbsDecimal,
            ScalarFuncSig::CeilInt,
            ScalarFuncSig::CeilReal,
            ScalarFuncSig::FloorInt,
            ScalarFuncSig::FloorReal,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ScalarFuncSig>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ScalarFuncSig", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ScalarFuncSig {
}

impl ::protobuf::reflect::ProtobufValue for ScalarFuncSig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10expression.proto\x12\x04tipb\x1a\x14gogoproto/gogo.proto\"\xa7\x01\
    \n\tFieldType\x12$\n\x02tp\x18\x01\x20\x01(\x0e2\x0e.tipb.DataTypeR\x02t\
    pB\x04\xc8\xde\x1f\0\x12\x12\n\x04flag\x18\x02\x20\x01(\rR\x04flag\x12\
    \x12\n\x04flen\x18\x03\x20\x01(\x05R\x04flen\x12\x18\n\x07decimal\x18\
    \x04\x20\x01(\x05R\x07decimal\x12\x18\n\x07charset\x18\x05\x20\x01(\tR\
    \x07charset\x12\x18\n\x07collate\x18\x06\x20\x01(\tR\x07collate\"\xbd\
    \x01\n\x04Expr\x12$\n\x02tp\x18\x01\x20\x01(\x0e2\x0e.tipb.ExprTypeR\x02\
    tpB\x04\xc8\xde\x1f\0\x12\x10\n\x03val\x18\x02\x20\x01(\x0cR\x03val\x12&\
    \n\x08children\x18\x03\x20\x03(\x0b2\n.tipb.ExprR\x08children\x12%\n\x03\
    sig\x18\x04\x20\x01(\x0e2\x13.tipb.ScalarFuncSigR\x03sig\x12.\n\nfield_t\
    ype\x18\x05\x20\x01(\x0b2\x0f.tipb.FieldTypeR\tfieldType\"B\n\x06ByItem\
    \x12\x1e\n\x04expr\x18\x01\x20\x01(\x0b2\n.tipb.ExprR\x04expr\x12\x18\n\
    \x04desc\x18\x02\x20\x01(\x08R\x04descB\x04\xc8\xde\x1f\0*\xd9\x03\n\x08\
    DataType\x12\x0f\n\x0bTypeDecimal\x10\0\x12\x0c\n\x08TypeTiny\x10\x01\
    \x12\r\n\tTypeShort\x10\x02\x12\x0c\n\x08TypeLong\x10\x03\x12\r\n\tTypeF\
    loat\x10\x04\x12\x0e\n\nTypeDouble\x10\x05\x12\x0c\n\x08TypeNull\x10\x06\
    \x12\x11\n\rTypeTimestamp\x10\x07\x12\x10\n\x0cTypeLongLong\x10\x08\x12\
    \r\n\tTypeInt24\x10\t\x12\x0c\n\x08TypeDate\x10\n\x12\x10\n\x0cTypeDurat\
    ion\x10\x0b\x12\x10\n\x0cTypeDatetime\x10\x0c\x12\x0c\n\x08TypeYear\x10\
    \r\x12\x0f\n\x0bTypeNewDate\x10\x0e\x12\x0f\n\x0bTypeVarchar\x10\x0f\x12\
    \x0b\n\x07TypeBit\x10\x10\x12\r\n\x08TypeJSON\x10\xf5\x01\x12\x13\n\x0eT\
    ypeNewDecimal\x10\xf6\x01\x12\r\n\x08TypeEnum\x10\xf7\x01\x12\x0c\n\x07T\
    ypeSet\x10\xf8\x01\x12\x11\n\x0cTypeTinyBlob\x10\xf9\x01\x12\x13\n\x0eTy\
    peMediumBlob\x10\xfa\x01\x12\x11\n\x0cTypeLongBlob\x10\xfb\x01\x12\r\n\
    \x08TypeBlob\x10\xfc\x01\x12\x12\n\rTypeVarString\x10\xfd\x01\x12\x0f\n\
    \nTypeString\x10\xfe\x01\x12\x11\n\x0cTypeGeometry\x10\xff\x01*\xc5\x0b\
    \n\x08ExprType\x12\x08\n\x04Null\x10\0\x12\t\n\x05Int64\x10\x01\x12\n\n\
    \x06Uint64\x10\x02\x12\x0b\n\x07Float32\x10\x03\x12\x0b\n\x07Float64\x10\
    \x04\x12\n\n\x06String\x10\x05\x12\t\n\x05Bytes\x10\x06\x12\x0c\n\x08Mys\
    qlBit\x10e\x12\x10\n\x0cMysqlDecimal\x10f\x12\x11\n\rMysqlDuration\x10g\
    \x12\r\n\tMysqlEnum\x10h\x12\x0c\n\x08MysqlHex\x10i\x12\x0c\n\x08MysqlSe\
    t\x10j\x12\r\n\tMysqlTime\x10k\x12\x0e\n\tValueList\x10\x97\x01\x12\x0e\
    \n\tColumnRef\x10\xc9\x01\x12\x08\n\x03Not\x10\xe9\x07\x12\x08\n\x03Neg\
    \x10\xea\x07\x12\x0b\n\x06BitNeg\x10\xeb\x07\x12\x07\n\x02LT\x10\xd1\x0f\
    \x12\x07\n\x02LE\x10\xd2\x0f\x12\x07\n\x02EQ\x10\xd3\x0f\x12\x07\n\x02NE\
    \x10\xd4\x0f\x12\x07\n\x02GE\x10\xd5\x0f\x12\x07\n\x02GT\x10\xd6\x0f\x12\
    \x0b\n\x06NullEQ\x10\xd7\x0f\x12\x0b\n\x06BitAnd\x10\xb5\x10\x12\n\n\x05\
    BitOr\x10\xb6\x10\x12\x0b\n\x06BitXor\x10\xb7\x10\x12\x0e\n\tLeftShift\
    \x10\xb8\x10\x12\x0e\n\tRighShift\x10\xb9\x10\x12\t\n\x04Plus\x10\x99\
    \x11\x12\n\n\x05Minus\x10\x9a\x11\x12\x08\n\x03Mul\x10\x9b\x11\x12\x08\n\
    \x03Div\x10\x9c\x11\x12\x0b\n\x06IntDiv\x10\x9d\x11\x12\x08\n\x03Mod\x10\
    \x9e\x11\x12\x08\n\x03And\x10\xfd\x11\x12\x07\n\x02Or\x10\xfe\x11\x12\
    \x08\n\x03Xor\x10\xff\x11\x12\n\n\x05Count\x10\xb9\x17\x12\x08\n\x03Sum\
    \x10\xba\x17\x12\x08\n\x03Avg\x10\xbb\x17\x12\x08\n\x03Min\x10\xbc\x17\
    \x12\x08\n\x03Max\x10\xbd\x17\x12\n\n\x05First\x10\xbe\x17\x12\x10\n\x0b\
    GroupConcat\x10\xbf\x17\x12\x08\n\x03Abs\x10\x9d\x18\x12\x08\n\x03Pow\
    \x10\x9e\x18\x12\n\n\x05Round\x10\x9f\x18\x12\x0b\n\x06Concat\x10\x81\
    \x19\x12\r\n\x08ConcatWS\x10\x82\x19\x12\t\n\x04Left\x10\x83\x19\x12\x0b\
    \n\x06Length\x10\x84\x19\x12\n\n\x05Lower\x10\x85\x19\x12\x0b\n\x06Repea\
    t\x10\x86\x19\x12\x0c\n\x07Replace\x10\x87\x19\x12\n\n\x05Upper\x10\x88\
    \x19\x12\x0b\n\x06Strcmp\x10\x89\x19\x12\x0c\n\x07Convert\x10\x8a\x19\
    \x12\t\n\x04Cast\x10\x8b\x19\x12\x0e\n\tSubstring\x10\x8c\x19\x12\x13\n\
    \x0eSubstringIndex\x10\x8d\x19\x12\x0b\n\x06Locate\x10\x8e\x19\x12\t\n\
    \x04Trim\x10\x8f\x19\x12\x07\n\x02If\x10\xe5\x19\x12\x0b\n\x06NullIf\x10\
    \xe6\x19\x12\x0b\n\x06IfNull\x10\xe7\x19\x12\t\n\x04Date\x10\xc9\x1a\x12\
    \x0c\n\x07DateAdd\x10\xca\x1a\x12\x0c\n\x07DateSub\x10\xcb\x1a\x12\t\n\
    \x04Year\x10\xd3\x1a\x12\r\n\x08YearWeek\x10\xd4\x1a\x12\n\n\x05Month\
    \x10\xdd\x1a\x12\t\n\x04Week\x10\xe7\x1a\x12\x0c\n\x07Weekday\x10\xe8\
    \x1a\x12\x0f\n\nWeekOfYear\x10\xe9\x1a\x12\x08\n\x03Day\x10\xf1\x1a\x12\
    \x0c\n\x07DayName\x10\xf2\x1a\x12\x0e\n\tDayOfYear\x10\xf3\x1a\x12\x0f\n\
    \nDayOfMonth\x10\xf4\x1a\x12\x0e\n\tDayOfWeek\x10\xf5\x1a\x12\t\n\x04Hou\
    r\x10\xfb\x1a\x12\x0b\n\x06Minute\x10\xfc\x1a\x12\x0b\n\x06Second\x10\
    \xfd\x1a\x12\x10\n\x0bMicrosecond\x10\xfe\x1a\x12\x0c\n\x07Extract\x10\
    \x85\x1b\x12\r\n\x08Coalesce\x10\xad\x1b\x12\r\n\x08Greatest\x10\xae\x1b\
    \x12\n\n\x05Least\x10\xaf\x1b\x12\x10\n\x0bJsonExtract\x10\x91\x1c\x12\r\
    \n\x08JsonType\x10\x92\x1c\x12\x0e\n\tJsonArray\x10\x93\x1c\x12\x0f\n\nJ\
    sonObject\x10\x94\x1c\x12\x0e\n\tJsonMerge\x10\x95\x1c\x12\x0e\n\tJsonVa\
    lid\x10\x96\x1c\x12\x0c\n\x07JsonSet\x10\x97\x1c\x12\x0f\n\nJsonInsert\
    \x10\x98\x1c\x12\x10\n\x0bJsonReplace\x10\x99\x1c\x12\x0f\n\nJsonRemove\
    \x10\x9a\x1c\x12\x11\n\x0cJsonContains\x10\x9b\x1c\x12\x10\n\x0bJsonUnqu\
    ote\x10\x9c\x1c\x12\x15\n\x10JsonContainsPath\x10\x9d\x1c\x12\x07\n\x02I\
    n\x10\xa1\x1f\x12\x0c\n\x07IsTruth\x10\xa2\x1f\x12\x0b\n\x06IsNull\x10\
    \xa3\x1f\x12\x0c\n\x07ExprRow\x10\xa4\x1f\x12\t\n\x04Like\x10\xa5\x1f\
    \x12\n\n\x05RLike\x10\xa6\x1f\x12\t\n\x04Case\x10\xa7\x1f\x12\x0f\n\nSca\
    larFunc\x10\x90N*\xe3\x0c\n\rScalarFuncSig\x12\x10\n\x0cCastIntAsInt\x10\
    \0\x12\x11\n\rCastIntAsReal\x10\x01\x12\x13\n\x0fCastIntAsString\x10\x02\
    \x12\x14\n\x10CastIntAsDecimal\x10\x03\x12\x11\n\rCastIntAsTime\x10\x04\
    \x12\x15\n\x11CastIntAsDuration\x10\x05\x12\x11\n\rCastRealAsInt\x10\n\
    \x12\x12\n\x0eCastRealAsReal\x10\x0b\x12\x14\n\x10CastRealAsString\x10\
    \x0c\x12\x15\n\x11CastRealAsDecimal\x10\r\x12\x12\n\x0eCastRealAsTime\
    \x10\x0e\x12\x16\n\x12CastRealAsDuration\x10\x0f\x12\x14\n\x10CastDecima\
    lAsInt\x10\x14\x12\x15\n\x11CastDecimalAsReal\x10\x15\x12\x17\n\x13CastD\
    ecimalAsString\x10\x16\x12\x18\n\x14CastDecimalAsDecimal\x10\x17\x12\x15\
    \n\x11CastDecimalAsTime\x10\x18\x12\x19\n\x15CastDecimalAsDuration\x10\
    \x19\x12\x13\n\x0fCastStringAsInt\x10\x1e\x12\x14\n\x10CastStringAsReal\
    \x10\x1f\x12\x16\n\x12CastStringAsString\x10\x20\x12\x17\n\x13CastString\
    AsDecimal\x10!\x12\x14\n\x10CastStringAsTime\x10\"\x12\x18\n\x14CastStri\
    ngAsDuration\x10#\x12\x11\n\rCastTimeAsInt\x10(\x12\x12\n\x0eCastTimeAsR\
    eal\x10)\x12\x14\n\x10CastTimeAsString\x10*\x12\x15\n\x11CastTimeAsDecim\
    al\x10+\x12\x12\n\x0eCastTimeAsTime\x10,\x12\x16\n\x12CastTimeAsDuration\
    \x10-\x12\x15\n\x11CastDurationAsInt\x102\x12\x16\n\x12CastDurationAsRea\
    l\x103\x12\x18\n\x14CastDurationAsString\x104\x12\x19\n\x15CastDurationA\
    sDecimal\x105\x12\x16\n\x12CastDurationAsTime\x106\x12\x1a\n\x16CastDura\
    tionAsDuration\x107\x12\t\n\x05LTInt\x10d\x12\n\n\x06LTReal\x10e\x12\r\n\
    \tLTDecimal\x10f\x12\x0c\n\x08LTString\x10g\x12\n\n\x06LTTime\x10h\x12\
    \x0e\n\nLTDuration\x10i\x12\n\n\x06LTJson\x10j\x12\t\n\x05LEInt\x10n\x12\
    \n\n\x06LEReal\x10o\x12\r\n\tLEDecimal\x10p\x12\x0c\n\x08LEString\x10q\
    \x12\n\n\x06LETime\x10r\x12\x0e\n\nLEDuration\x10s\x12\n\n\x06LEJson\x10\
    t\x12\t\n\x05GTInt\x10x\x12\n\n\x06GTReal\x10y\x12\r\n\tGTDecimal\x10z\
    \x12\x0c\n\x08GTString\x10{\x12\n\n\x06GTTime\x10|\x12\x0e\n\nGTDuration\
    \x10}\x12\n\n\x06GTJson\x10~\x12\n\n\x05GEInt\x10\x82\x01\x12\x0b\n\x06G\
    EReal\x10\x83\x01\x12\x0e\n\tGEDecimal\x10\x84\x01\x12\r\n\x08GEString\
    \x10\x85\x01\x12\x0b\n\x06GETime\x10\x86\x01\x12\x0f\n\nGEDuration\x10\
    \x87\x01\x12\x0b\n\x06GEJson\x10\x88\x01\x12\n\n\x05EQInt\x10\x8c\x01\
    \x12\x0b\n\x06EQReal\x10\x8d\x01\x12\x0e\n\tEQDecimal\x10\x8e\x01\x12\r\
    \n\x08EQString\x10\x8f\x01\x12\x0b\n\x06EQTime\x10\x90\x01\x12\x0f\n\nEQ\
    Duration\x10\x91\x01\x12\x0b\n\x06EQJson\x10\x92\x01\x12\n\n\x05NEInt\
    \x10\x96\x01\x12\x0b\n\x06NEReal\x10\x97\x01\x12\x0e\n\tNEDecimal\x10\
    \x98\x01\x12\r\n\x08NEString\x10\x99\x01\x12\x0b\n\x06NETime\x10\x9a\x01\
    \x12\x0f\n\nNEDuration\x10\x9b\x01\x12\x0b\n\x06NEJson\x10\x9c\x01\x12\
    \x0e\n\tNullEQInt\x10\xa0\x01\x12\x0f\n\nNullEQReal\x10\xa1\x01\x12\x12\
    \n\rNullEQDecimal\x10\xa2\x01\x12\x11\n\x0cNullEQString\x10\xa3\x01\x12\
    \x0f\n\nNullEQTime\x10\xa4\x01\x12\x13\n\x0eNullEQDuration\x10\xa5\x01\
    \x12\x0f\n\nNullEQJson\x10\xa6\x01\x12\x0b\n\x06AbsInt\x10\xb5\x10\x12\
    \x0c\n\x07AbsReal\x10\xb6\x10\x12\x0f\n\nAbsDecimal\x10\xb7\x10\x12\x0c\
    \n\x07CeilInt\x10\xb8\x10\x12\r\n\x08CeilReal\x10\xb9\x10\x12\r\n\x08Flo\
    orInt\x10\xba\x10\x12\x0e\n\tFloorReal\x10\xbb\x10B%\n\x15com.pingcap.ti\
    db.tipbP\x01\xd0\xe2\x1e\x01\xe0\xe2\x1e\x01\xc8\xe2\x1e\x01J\xdba\n\x07\
    \x12\x05\0\0\xcf\x02\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\
    \x12\x03\x02\x08\x0c\n\x08\n\x01\x08\x12\x03\x04\0\"\n\x0b\n\x04\x08\xe7\
    \x07\0\x12\x03\x04\0\"\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x04\x07\x1a\
    \n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x04\x07\x1a\n\x0e\n\x07\x08\xe7\
    \x07\0\x02\0\x01\x12\x03\x04\x07\x1a\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\
    \x03\x04\x1d!\n\x08\n\x01\x08\x12\x03\x05\0.\n\x0b\n\x04\x08\xe7\x07\x01\
    \x12\x03\x05\0.\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x05\x07\x13\n\r\
    \n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x05\x07\x13\n\x0e\n\x07\x08\xe7\x07\
    \x01\x02\0\x01\x12\x03\x05\x07\x13\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\
    \x03\x05\x16-\n\t\n\x02\x03\0\x12\x03\x07\x07\x1d\n\x08\n\x01\x08\x12\
    \x03\t\0(\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\t\0(\n\x0c\n\x05\x08\xe7\
    \x07\x02\x02\x12\x03\t\x07\x20\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\t\
    \x07\x20\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\t\x08\x1f\n\x0c\n\
    \x05\x08\xe7\x07\x02\x03\x12\x03\t#'\n\x08\n\x01\x08\x12\x03\n\0$\n\x0b\
    \n\x04\x08\xe7\x07\x03\x12\x03\n\0$\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\
    \x03\n\x07\x1c\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\n\x07\x1c\n\x0e\n\
    \x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\n\x08\x1b\n\x0c\n\x05\x08\xe7\x07\
    \x03\x03\x12\x03\n\x1f#\n\x08\n\x01\x08\x12\x03\x0b\0*\n\x0b\n\x04\x08\
    \xe7\x07\x04\x12\x03\x0b\0*\n\x0c\n\x05\x08\xe7\x07\x04\x02\x12\x03\x0b\
    \x07\"\n\r\n\x06\x08\xe7\x07\x04\x02\0\x12\x03\x0b\x07\"\n\x0e\n\x07\x08\
    \xe7\x07\x04\x02\0\x01\x12\x03\x0b\x08!\n\x0c\n\x05\x08\xe7\x07\x04\x03\
    \x12\x03\x0b%)\n\x1f\n\x02\x05\0\x12\x04\x0e\0,\x01\x1a\x13\x20MySQL\x20\
    data\x20types.\n\n\n\n\x03\x05\0\x01\x12\x03\x0e\x05\r\n\x0b\n\x04\x05\0\
    \x02\0\x12\x03\x0f\x08\x18\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x0f\x08\
    \x13\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x0f\x16\x17\n\x0b\n\x04\x05\0\
    \x02\x01\x12\x03\x10\x08\x15\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x10\
    \x08\x10\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x10\x13\x14\n\x0b\n\x04\
    \x05\0\x02\x02\x12\x03\x11\x08\x16\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\
    \x11\x08\x11\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x11\x14\x15\n\x0b\n\
    \x04\x05\0\x02\x03\x12\x03\x12\x08\x15\n\x0c\n\x05\x05\0\x02\x03\x01\x12\
    \x03\x12\x08\x10\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\x12\x13\x14\n\x0b\
    \n\x04\x05\0\x02\x04\x12\x03\x13\x08\x16\n\x0c\n\x05\x05\0\x02\x04\x01\
    \x12\x03\x13\x08\x11\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x13\x14\x15\n\
    \x0b\n\x04\x05\0\x02\x05\x12\x03\x14\x08\x17\n\x0c\n\x05\x05\0\x02\x05\
    \x01\x12\x03\x14\x08\x12\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03\x14\x15\
    \x16\n\x0b\n\x04\x05\0\x02\x06\x12\x03\x15\x08\x15\n\x0c\n\x05\x05\0\x02\
    \x06\x01\x12\x03\x15\x08\x10\n\x0c\n\x05\x05\0\x02\x06\x02\x12\x03\x15\
    \x13\x14\n\x0b\n\x04\x05\0\x02\x07\x12\x03\x16\x08\x1a\n\x0c\n\x05\x05\0\
    \x02\x07\x01\x12\x03\x16\x08\x15\n\x0c\n\x05\x05\0\x02\x07\x02\x12\x03\
    \x16\x18\x19\n\x0b\n\x04\x05\0\x02\x08\x12\x03\x17\x08\x19\n\x0c\n\x05\
    \x05\0\x02\x08\x01\x12\x03\x17\x08\x14\n\x0c\n\x05\x05\0\x02\x08\x02\x12\
    \x03\x17\x17\x18\n\x0b\n\x04\x05\0\x02\t\x12\x03\x18\x08\x16\n\x0c\n\x05\
    \x05\0\x02\t\x01\x12\x03\x18\x08\x11\n\x0c\n\x05\x05\0\x02\t\x02\x12\x03\
    \x18\x14\x15\n\x0b\n\x04\x05\0\x02\n\x12\x03\x19\x08\x16\n\x0c\n\x05\x05\
    \0\x02\n\x01\x12\x03\x19\x08\x10\n\x0c\n\x05\x05\0\x02\n\x02\x12\x03\x19\
    \x13\x15\n\x0b\n\x04\x05\0\x02\x0b\x12\x03\x1a\x08\x1a\n\x0c\n\x05\x05\0\
    \x02\x0b\x01\x12\x03\x1a\x08\x14\n\x0c\n\x05\x05\0\x02\x0b\x02\x12\x03\
    \x1a\x17\x19\n\x0b\n\x04\x05\0\x02\x0c\x12\x03\x1b\x08\x1a\n\x0c\n\x05\
    \x05\0\x02\x0c\x01\x12\x03\x1b\x08\x14\n\x0c\n\x05\x05\0\x02\x0c\x02\x12\
    \x03\x1b\x17\x19\n\x0b\n\x04\x05\0\x02\r\x12\x03\x1c\x08\x16\n\x0c\n\x05\
    \x05\0\x02\r\x01\x12\x03\x1c\x08\x10\n\x0c\n\x05\x05\0\x02\r\x02\x12\x03\
    \x1c\x13\x15\n\x0b\n\x04\x05\0\x02\x0e\x12\x03\x1d\x08\x19\n\x0c\n\x05\
    \x05\0\x02\x0e\x01\x12\x03\x1d\x08\x13\n\x0c\n\x05\x05\0\x02\x0e\x02\x12\
    \x03\x1d\x16\x18\n\x0b\n\x04\x05\0\x02\x0f\x12\x03\x1e\x08\x19\n\x0c\n\
    \x05\x05\0\x02\x0f\x01\x12\x03\x1e\x08\x13\n\x0c\n\x05\x05\0\x02\x0f\x02\
    \x12\x03\x1e\x16\x18\n\x0b\n\x04\x05\0\x02\x10\x12\x03\x1f\x08\x15\n\x0c\
    \n\x05\x05\0\x02\x10\x01\x12\x03\x1f\x08\x0f\n\x0c\n\x05\x05\0\x02\x10\
    \x02\x12\x03\x1f\x12\x14\n\x0b\n\x04\x05\0\x02\x11\x12\x03!\x08\x18\n\
    \x0c\n\x05\x05\0\x02\x11\x01\x12\x03!\x08\x10\n\x0c\n\x05\x05\0\x02\x11\
    \x02\x12\x03!\x13\x17\n\x0b\n\x04\x05\0\x02\x12\x12\x03\"\x08\x1e\n\x0c\
    \n\x05\x05\0\x02\x12\x01\x12\x03\"\x08\x16\n\x0c\n\x05\x05\0\x02\x12\x02\
    \x12\x03\"\x19\x1d\n\x0b\n\x04\x05\0\x02\x13\x12\x03#\x08\x18\n\x0c\n\
    \x05\x05\0\x02\x13\x01\x12\x03#\x08\x10\n\x0c\n\x05\x05\0\x02\x13\x02\
    \x12\x03#\x13\x17\n\x0b\n\x04\x05\0\x02\x14\x12\x03$\x08\x17\n\x0c\n\x05\
    \x05\0\x02\x14\x01\x12\x03$\x08\x0f\n\x0c\n\x05\x05\0\x02\x14\x02\x12\
    \x03$\x12\x16\n\x0b\n\x04\x05\0\x02\x15\x12\x03%\x08\x1c\n\x0c\n\x05\x05\
    \0\x02\x15\x01\x12\x03%\x08\x14\n\x0c\n\x05\x05\0\x02\x15\x02\x12\x03%\
    \x17\x1b\n\x0b\n\x04\x05\0\x02\x16\x12\x03&\x08\x1e\n\x0c\n\x05\x05\0\
    \x02\x16\x01\x12\x03&\x08\x16\n\x0c\n\x05\x05\0\x02\x16\x02\x12\x03&\x19\
    \x1d\n\x0b\n\x04\x05\0\x02\x17\x12\x03'\x08\x1c\n\x0c\n\x05\x05\0\x02\
    \x17\x01\x12\x03'\x08\x14\n\x0c\n\x05\x05\0\x02\x17\x02\x12\x03'\x17\x1b\
    \n\x0b\n\x04\x05\0\x02\x18\x12\x03(\x08\x18\n\x0c\n\x05\x05\0\x02\x18\
    \x01\x12\x03(\x08\x10\n\x0c\n\x05\x05\0\x02\x18\x02\x12\x03(\x13\x17\n\
    \x0b\n\x04\x05\0\x02\x19\x12\x03)\x08\x1d\n\x0c\n\x05\x05\0\x02\x19\x01\
    \x12\x03)\x08\x15\n\x0c\n\x05\x05\0\x02\x19\x02\x12\x03)\x18\x1c\n\x0b\n\
    \x04\x05\0\x02\x1a\x12\x03*\x08\x1a\n\x0c\n\x05\x05\0\x02\x1a\x01\x12\
    \x03*\x08\x12\n\x0c\n\x05\x05\0\x02\x1a\x02\x12\x03*\x15\x19\n\x0b\n\x04\
    \x05\0\x02\x1b\x12\x03+\x08\x1c\n\x0c\n\x05\x05\0\x02\x1b\x01\x12\x03+\
    \x08\x14\n\x0c\n\x05\x05\0\x02\x1b\x02\x12\x03+\x17\x1b\n\n\n\x02\x04\0\
    \x12\x04.\05\x01\n\n\n\x03\x04\0\x01\x12\x03.\x08\x11\n\x0b\n\x04\x04\0\
    \x02\0\x12\x03/\x08@\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03/\x08\x10\n\x0c\
    \n\x05\x04\0\x02\0\x06\x12\x03/\x11\x19\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03/\x1a\x1c\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03/\x1f\x20\n\x0c\n\x05\
    \x04\0\x02\0\x08\x12\x03/!?\n\x0f\n\x08\x04\0\x02\0\x08\xe7\x07\0\x12\
    \x03/\">\n\x10\n\t\x04\0\x02\0\x08\xe7\x07\0\x02\x12\x03/\"6\n\x11\n\n\
    \x04\0\x02\0\x08\xe7\x07\0\x02\0\x12\x03/\"6\n\x12\n\x0b\x04\0\x02\0\x08\
    \xe7\x07\0\x02\0\x01\x12\x03/#5\n\x10\n\t\x04\0\x02\0\x08\xe7\x07\0\x03\
    \x12\x03/9>\n\x0b\n\x04\x04\0\x02\x01\x12\x030\x08!\n\x0c\n\x05\x04\0\
    \x02\x01\x04\x12\x030\x08\x10\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x030\x11\
    \x17\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x030\x18\x1c\n\x0c\n\x05\x04\0\
    \x02\x01\x03\x12\x030\x1f\x20\n\x0b\n\x04\x04\0\x02\x02\x12\x031\x08\x20\
    \n\x0c\n\x05\x04\0\x02\x02\x04\x12\x031\x08\x10\n\x0c\n\x05\x04\0\x02\
    \x02\x05\x12\x031\x11\x16\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x031\x17\x1b\
    \n\x0c\n\x05\x04\0\x02\x02\x03\x12\x031\x1e\x1f\n\x0b\n\x04\x04\0\x02\
    \x03\x12\x032\x08#\n\x0c\n\x05\x04\0\x02\x03\x04\x12\x032\x08\x10\n\x0c\
    \n\x05\x04\0\x02\x03\x05\x12\x032\x11\x16\n\x0c\n\x05\x04\0\x02\x03\x01\
    \x12\x032\x17\x1e\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x032!\"\n\x0b\n\x04\
    \x04\0\x02\x04\x12\x033\x08$\n\x0c\n\x05\x04\0\x02\x04\x04\x12\x033\x08\
    \x10\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x033\x11\x17\n\x0c\n\x05\x04\0\
    \x02\x04\x01\x12\x033\x18\x1f\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x033\"#\
    \n\x0b\n\x04\x04\0\x02\x05\x12\x034\x08$\n\x0c\n\x05\x04\0\x02\x05\x04\
    \x12\x034\x08\x10\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x034\x11\x17\n\x0c\n\
    \x05\x04\0\x02\x05\x01\x12\x034\x18\x1f\n\x0c\n\x05\x04\0\x02\x05\x03\
    \x12\x034\"#\n\x20\n\x02\x05\x01\x12\x057\0\xd3\x01\x01\"\x13\x20Childre\
    n\x20count\x200.\x20\n\n\n\x03\x05\x01\x01\x12\x037\x05\r\n(\n\x04\x05\
    \x01\x02\0\x12\x03:\x08\x11\x1a\x1b\x20Values\x20are\x20encoded\x20bytes\
    .\n\n\x0c\n\x05\x05\x01\x02\0\x01\x12\x03:\x08\x0c\n\x0c\n\x05\x05\x01\
    \x02\0\x02\x12\x03:\x0f\x10\n\x0b\n\x04\x05\x01\x02\x01\x12\x03;\x08\x12\
    \n\x0c\n\x05\x05\x01\x02\x01\x01\x12\x03;\x08\r\n\x0c\n\x05\x05\x01\x02\
    \x01\x02\x12\x03;\x10\x11\n\x0b\n\x04\x05\x01\x02\x02\x12\x03<\x08\x13\n\
    \x0c\n\x05\x05\x01\x02\x02\x01\x12\x03<\x08\x0e\n\x0c\n\x05\x05\x01\x02\
    \x02\x02\x12\x03<\x11\x12\n\x0b\n\x04\x05\x01\x02\x03\x12\x03=\x08\x14\n\
    \x0c\n\x05\x05\x01\x02\x03\x01\x12\x03=\x08\x0f\n\x0c\n\x05\x05\x01\x02\
    \x03\x02\x12\x03=\x12\x13\n\x0b\n\x04\x05\x01\x02\x04\x12\x03>\x08\x14\n\
    \x0c\n\x05\x05\x01\x02\x04\x01\x12\x03>\x08\x0f\n\x0c\n\x05\x05\x01\x02\
    \x04\x02\x12\x03>\x12\x13\n\x0b\n\x04\x05\x01\x02\x05\x12\x03?\x08\x13\n\
    \x0c\n\x05\x05\x01\x02\x05\x01\x12\x03?\x08\x0e\n\x0c\n\x05\x05\x01\x02\
    \x05\x02\x12\x03?\x11\x12\n\x0b\n\x04\x05\x01\x02\x06\x12\x03@\x08\x12\n\
    \x0c\n\x05\x05\x01\x02\x06\x01\x12\x03@\x08\r\n\x0c\n\x05\x05\x01\x02\
    \x06\x02\x12\x03@\x10\x11\n$\n\x04\x05\x01\x02\x07\x12\x03C\x08\x17\x1a\
    \x17\x20Mysql\x20specific\x20types.\n\n\x0c\n\x05\x05\x01\x02\x07\x01\
    \x12\x03C\x08\x10\n\x0c\n\x05\x05\x01\x02\x07\x02\x12\x03C\x13\x16\n\x0b\
    \n\x04\x05\x01\x02\x08\x12\x03D\x08\x1b\n\x0c\n\x05\x05\x01\x02\x08\x01\
    \x12\x03D\x08\x14\n\x0c\n\x05\x05\x01\x02\x08\x02\x12\x03D\x17\x1a\n\x0b\
    \n\x04\x05\x01\x02\t\x12\x03E\x08\x1c\n\x0c\n\x05\x05\x01\x02\t\x01\x12\
    \x03E\x08\x15\n\x0c\n\x05\x05\x01\x02\t\x02\x12\x03E\x18\x1b\n\x0b\n\x04\
    \x05\x01\x02\n\x12\x03F\x08\x18\n\x0c\n\x05\x05\x01\x02\n\x01\x12\x03F\
    \x08\x11\n\x0c\n\x05\x05\x01\x02\n\x02\x12\x03F\x14\x17\n\x0b\n\x04\x05\
    \x01\x02\x0b\x12\x03G\x08\x17\n\x0c\n\x05\x05\x01\x02\x0b\x01\x12\x03G\
    \x08\x10\n\x0c\n\x05\x05\x01\x02\x0b\x02\x12\x03G\x13\x16\n\x0b\n\x04\
    \x05\x01\x02\x0c\x12\x03H\x08\x17\n\x0c\n\x05\x05\x01\x02\x0c\x01\x12\
    \x03H\x08\x10\n\x0c\n\x05\x05\x01\x02\x0c\x02\x12\x03H\x13\x16\n\x0b\n\
    \x04\x05\x01\x02\r\x12\x03I\x08\x18\n\x0c\n\x05\x05\x01\x02\r\x01\x12\
    \x03I\x08\x11\n\x0c\n\x05\x05\x01\x02\r\x02\x12\x03I\x14\x17\n\"\n\x04\
    \x05\x01\x02\x0e\x12\x03L\x08\x18\x1a\x15\x20Encoded\x20value\x20list.\n\
    \n\x0c\n\x05\x05\x01\x02\x0e\x01\x12\x03L\x08\x11\n\x0c\n\x05\x05\x01\
    \x02\x0e\x02\x12\x03L\x14\x17\n:\n\x04\x05\x01\x02\x0f\x12\x03O\x08\x18\
    \x1a-\x20Column\x20reference.\x20value\x20is\x20int64\x20column\x20ID.\n\
    \n\x0c\n\x05\x05\x01\x02\x0f\x01\x12\x03O\x08\x11\n\x0c\n\x05\x05\x01\
    \x02\x0f\x02\x12\x03O\x14\x17\n2\n\x04\x05\x01\x02\x10\x12\x03R\x08\x13\
    \x1a%\x20Unary\x20operations,\x20children\x20count\x201.\x20\n\x0c\n\x05\
    \x05\x01\x02\x10\x01\x12\x03R\x08\x0b\n\x0c\n\x05\x05\x01\x02\x10\x02\
    \x12\x03R\x0e\x12\n\x0b\n\x04\x05\x01\x02\x11\x12\x03S\x08\x13\n\x0c\n\
    \x05\x05\x01\x02\x11\x01\x12\x03S\x08\x0b\n\x0c\n\x05\x05\x01\x02\x11\
    \x02\x12\x03S\x0e\x12\n\x0b\n\x04\x05\x01\x02\x12\x12\x03T\x08\x16\n\x0c\
    \n\x05\x05\x01\x02\x12\x01\x12\x03T\x08\x0e\n\x0c\n\x05\x05\x01\x02\x12\
    \x02\x12\x03T\x11\x15\nM\n\x04\x05\x01\x02\x13\x12\x03X\x08\x12\x1a\x18\
    \x20Comparison\x20operations.\n2&\x20Binary\x20operations,\x20children\
    \x20count\x202.\x20\n\x0c\n\x05\x05\x01\x02\x13\x01\x12\x03X\x08\n\n\x0c\
    \n\x05\x05\x01\x02\x13\x02\x12\x03X\r\x11\n\x0b\n\x04\x05\x01\x02\x14\
    \x12\x03Y\x08\x12\n\x0c\n\x05\x05\x01\x02\x14\x01\x12\x03Y\x08\n\n\x0c\n\
    \x05\x05\x01\x02\x14\x02\x12\x03Y\r\x11\n\x0b\n\x04\x05\x01\x02\x15\x12\
    \x03Z\x08\x12\n\x0c\n\x05\x05\x01\x02\x15\x01\x12\x03Z\x08\n\n\x0c\n\x05\
    \x05\x01\x02\x15\x02\x12\x03Z\r\x11\n\x0b\n\x04\x05\x01\x02\x16\x12\x03[\
    \x08\x12\n\x0c\n\x05\x05\x01\x02\x16\x01\x12\x03[\x08\n\n\x0c\n\x05\x05\
    \x01\x02\x16\x02\x12\x03[\r\x11\n\x0b\n\x04\x05\x01\x02\x17\x12\x03\\\
    \x08\x12\n\x0c\n\x05\x05\x01\x02\x17\x01\x12\x03\\\x08\n\n\x0c\n\x05\x05\
    \x01\x02\x17\x02\x12\x03\\\r\x11\n\x0b\n\x04\x05\x01\x02\x18\x12\x03]\
    \x08\x12\n\x0c\n\x05\x05\x01\x02\x18\x01\x12\x03]\x08\n\n\x0c\n\x05\x05\
    \x01\x02\x18\x02\x12\x03]\r\x11\n\x0b\n\x04\x05\x01\x02\x19\x12\x03^\x08\
    \x16\n\x0c\n\x05\x05\x01\x02\x19\x01\x12\x03^\x08\x0e\n\x0c\n\x05\x05\
    \x01\x02\x19\x02\x12\x03^\x11\x15\n\x1e\n\x04\x05\x01\x02\x1a\x12\x03a\
    \x08\x16\x1a\x11\x20Bit\x20operations.\n\n\x0c\n\x05\x05\x01\x02\x1a\x01\
    \x12\x03a\x08\x0e\n\x0c\n\x05\x05\x01\x02\x1a\x02\x12\x03a\x11\x15\n\x0b\
    \n\x04\x05\x01\x02\x1b\x12\x03b\x08\x15\n\x0c\n\x05\x05\x01\x02\x1b\x01\
    \x12\x03b\x08\r\n\x0c\n\x05\x05\x01\x02\x1b\x02\x12\x03b\x10\x14\n\x0b\n\
    \x04\x05\x01\x02\x1c\x12\x03c\x08\x16\n\x0c\n\x05\x05\x01\x02\x1c\x01\
    \x12\x03c\x08\x0e\n\x0c\n\x05\x05\x01\x02\x1c\x02\x12\x03c\x11\x15\n\x0b\
    \n\x04\x05\x01\x02\x1d\x12\x03d\x08\x19\n\x0c\n\x05\x05\x01\x02\x1d\x01\
    \x12\x03d\x08\x11\n\x0c\n\x05\x05\x01\x02\x1d\x02\x12\x03d\x14\x18\n\x0b\
    \n\x04\x05\x01\x02\x1e\x12\x03e\x08\x19\n\x0c\n\x05\x05\x01\x02\x1e\x01\
    \x12\x03e\x08\x11\n\x0c\n\x05\x05\x01\x02\x1e\x02\x12\x03e\x14\x18\n\x1a\
    \n\x04\x05\x01\x02\x1f\x12\x03h\x08\x14\x1a\r\x20Arithmatic.\n\n\x0c\n\
    \x05\x05\x01\x02\x1f\x01\x12\x03h\x08\x0c\n\x0c\n\x05\x05\x01\x02\x1f\
    \x02\x12\x03h\x0f\x13\n\x0b\n\x04\x05\x01\x02\x20\x12\x03i\x08\x15\n\x0c\
    \n\x05\x05\x01\x02\x20\x01\x12\x03i\x08\r\n\x0c\n\x05\x05\x01\x02\x20\
    \x02\x12\x03i\x10\x14\n\x0b\n\x04\x05\x01\x02!\x12\x03j\x08\x13\n\x0c\n\
    \x05\x05\x01\x02!\x01\x12\x03j\x08\x0b\n\x0c\n\x05\x05\x01\x02!\x02\x12\
    \x03j\x0e\x12\n\x0b\n\x04\x05\x01\x02\"\x12\x03k\x08\x13\n\x0c\n\x05\x05\
    \x01\x02\"\x01\x12\x03k\x08\x0b\n\x0c\n\x05\x05\x01\x02\"\x02\x12\x03k\
    \x0e\x12\n\x0b\n\x04\x05\x01\x02#\x12\x03l\x08\x16\n\x0c\n\x05\x05\x01\
    \x02#\x01\x12\x03l\x08\x0e\n\x0c\n\x05\x05\x01\x02#\x02\x12\x03l\x11\x15\
    \n\x0b\n\x04\x05\x01\x02$\x12\x03m\x08\x13\n\x0c\n\x05\x05\x01\x02$\x01\
    \x12\x03m\x08\x0b\n\x0c\n\x05\x05\x01\x02$\x02\x12\x03m\x0e\x12\n\x20\n\
    \x04\x05\x01\x02%\x12\x03p\x08\x13\x1a\x13\x20Logic\x20operations.\n\n\
    \x0c\n\x05\x05\x01\x02%\x01\x12\x03p\x08\x0b\n\x0c\n\x05\x05\x01\x02%\
    \x02\x12\x03p\x0e\x12\n\x0b\n\x04\x05\x01\x02&\x12\x03q\x08\x12\n\x0c\n\
    \x05\x05\x01\x02&\x01\x12\x03q\x08\n\n\x0c\n\x05\x05\x01\x02&\x02\x12\
    \x03q\r\x11\n\x0b\n\x04\x05\x01\x02'\x12\x03r\x08\x13\n\x0c\n\x05\x05\
    \x01\x02'\x01\x12\x03r\x08\x0b\n\x0c\n\x05\x05\x01\x02'\x02\x12\x03r\x0e\
    \x12\n\\\n\x04\x05\x01\x02(\x12\x03v\x08\x15\x1a\x16\x20Aggregate\x20fun\
    ctions.\n27\x20Mysql\x20functions,\x20children\x20count\x20is\x20functio\
    n\x20specific.\x20\n\x0c\n\x05\x05\x01\x02(\x01\x12\x03v\x08\r\n\x0c\n\
    \x05\x05\x01\x02(\x02\x12\x03v\x10\x14\n\x0b\n\x04\x05\x01\x02)\x12\x03w\
    \x08\x13\n\x0c\n\x05\x05\x01\x02)\x01\x12\x03w\x08\x0b\n\x0c\n\x05\x05\
    \x01\x02)\x02\x12\x03w\x0e\x12\n\x0b\n\x04\x05\x01\x02*\x12\x03x\x08\x13\
    \n\x0c\n\x05\x05\x01\x02*\x01\x12\x03x\x08\x0b\n\x0c\n\x05\x05\x01\x02*\
    \x02\x12\x03x\x0e\x12\n\x0b\n\x04\x05\x01\x02+\x12\x03y\x08\x13\n\x0c\n\
    \x05\x05\x01\x02+\x01\x12\x03y\x08\x0b\n\x0c\n\x05\x05\x01\x02+\x02\x12\
    \x03y\x0e\x12\n\x0b\n\x04\x05\x01\x02,\x12\x03z\x08\x13\n\x0c\n\x05\x05\
    \x01\x02,\x01\x12\x03z\x08\x0b\n\x0c\n\x05\x05\x01\x02,\x02\x12\x03z\x0e\
    \x12\n\x0b\n\x04\x05\x01\x02-\x12\x03{\x08\x15\n\x0c\n\x05\x05\x01\x02-\
    \x01\x12\x03{\x08\r\n\x0c\n\x05\x05\x01\x02-\x02\x12\x03{\x10\x14\n\x0b\
    \n\x04\x05\x01\x02.\x12\x03|\x08\x1b\n\x0c\n\x05\x05\x01\x02.\x01\x12\
    \x03|\x08\x13\n\x0c\n\x05\x05\x01\x02.\x02\x12\x03|\x16\x1a\n\x1e\n\x04\
    \x05\x01\x02/\x12\x03\x7f\x08\x13\x1a\x11\x20Math\x20functions.\n\n\x0c\
    \n\x05\x05\x01\x02/\x01\x12\x03\x7f\x08\x0b\n\x0c\n\x05\x05\x01\x02/\x02\
    \x12\x03\x7f\x0e\x12\n\x0c\n\x04\x05\x01\x020\x12\x04\x80\x01\x08\x13\n\
    \r\n\x05\x05\x01\x020\x01\x12\x04\x80\x01\x08\x0b\n\r\n\x05\x05\x01\x020\
    \x02\x12\x04\x80\x01\x0e\x12\n\x0c\n\x04\x05\x01\x021\x12\x04\x81\x01\
    \x08\x15\n\r\n\x05\x05\x01\x021\x01\x12\x04\x81\x01\x08\r\n\r\n\x05\x05\
    \x01\x021\x02\x12\x04\x81\x01\x10\x14\n!\n\x04\x05\x01\x022\x12\x04\x84\
    \x01\x08\x16\x1a\x13\x20String\x20functions.\n\n\r\n\x05\x05\x01\x022\
    \x01\x12\x04\x84\x01\x08\x0e\n\r\n\x05\x05\x01\x022\x02\x12\x04\x84\x01\
    \x11\x15\n\x0c\n\x04\x05\x01\x023\x12\x04\x85\x01\x08\x18\n\r\n\x05\x05\
    \x01\x023\x01\x12\x04\x85\x01\x08\x10\n\r\n\x05\x05\x01\x023\x02\x12\x04\
    \x85\x01\x13\x17\n\x0c\n\x04\x05\x01\x024\x12\x04\x86\x01\x08\x14\n\r\n\
    \x05\x05\x01\x024\x01\x12\x04\x86\x01\x08\x0c\n\r\n\x05\x05\x01\x024\x02\
    \x12\x04\x86\x01\x0f\x13\n\x0c\n\x04\x05\x01\x025\x12\x04\x87\x01\x08\
    \x16\n\r\n\x05\x05\x01\x025\x01\x12\x04\x87\x01\x08\x0e\n\r\n\x05\x05\
    \x01\x025\x02\x12\x04\x87\x01\x11\x15\n\x0c\n\x04\x05\x01\x026\x12\x04\
    \x88\x01\x08\x15\n\r\n\x05\x05\x01\x026\x01\x12\x04\x88\x01\x08\r\n\r\n\
    \x05\x05\x01\x026\x02\x12\x04\x88\x01\x10\x14\n\x0c\n\x04\x05\x01\x027\
    \x12\x04\x89\x01\x08\x16\n\r\n\x05\x05\x01\x027\x01\x12\x04\x89\x01\x08\
    \x0e\n\r\n\x05\x05\x01\x027\x02\x12\x04\x89\x01\x11\x15\n\x0c\n\x04\x05\
    \x01\x028\x12\x04\x8a\x01\x08\x17\n\r\n\x05\x05\x01\x028\x01\x12\x04\x8a\
    \x01\x08\x0f\n\r\n\x05\x05\x01\x028\x02\x12\x04\x8a\x01\x12\x16\n\x0c\n\
    \x04\x05\x01\x029\x12\x04\x8b\x01\x08\x15\n\r\n\x05\x05\x01\x029\x01\x12\
    \x04\x8b\x01\x08\r\n\r\n\x05\x05\x01\x029\x02\x12\x04\x8b\x01\x10\x14\n\
    \x0c\n\x04\x05\x01\x02:\x12\x04\x8c\x01\x08\x16\n\r\n\x05\x05\x01\x02:\
    \x01\x12\x04\x8c\x01\x08\x0e\n\r\n\x05\x05\x01\x02:\x02\x12\x04\x8c\x01\
    \x11\x15\n\x0c\n\x04\x05\x01\x02;\x12\x04\x8d\x01\x08\x17\n\r\n\x05\x05\
    \x01\x02;\x01\x12\x04\x8d\x01\x08\x0f\n\r\n\x05\x05\x01\x02;\x02\x12\x04\
    \x8d\x01\x12\x16\n\x0c\n\x04\x05\x01\x02<\x12\x04\x8e\x01\x08\x14\n\r\n\
    \x05\x05\x01\x02<\x01\x12\x04\x8e\x01\x08\x0c\n\r\n\x05\x05\x01\x02<\x02\
    \x12\x04\x8e\x01\x0f\x13\n\x0c\n\x04\x05\x01\x02=\x12\x04\x8f\x01\x08\
    \x19\n\r\n\x05\x05\x01\x02=\x01\x12\x04\x8f\x01\x08\x11\n\r\n\x05\x05\
    \x01\x02=\x02\x12\x04\x8f\x01\x14\x18\n\x0c\n\x04\x05\x01\x02>\x12\x04\
    \x90\x01\x08\x1e\n\r\n\x05\x05\x01\x02>\x01\x12\x04\x90\x01\x08\x16\n\r\
    \n\x05\x05\x01\x02>\x02\x12\x04\x90\x01\x19\x1d\n\x0c\n\x04\x05\x01\x02?\
    \x12\x04\x91\x01\x08\x16\n\r\n\x05\x05\x01\x02?\x01\x12\x04\x91\x01\x08\
    \x0e\n\r\n\x05\x05\x01\x02?\x02\x12\x04\x91\x01\x11\x15\n\x0c\n\x04\x05\
    \x01\x02@\x12\x04\x92\x01\x08\x14\n\r\n\x05\x05\x01\x02@\x01\x12\x04\x92\
    \x01\x08\x0c\n\r\n\x05\x05\x01\x02@\x02\x12\x04\x92\x01\x0f\x13\n'\n\x04\
    \x05\x01\x02A\x12\x04\x95\x01\x08\x12\x1a\x19\x20Control\x20flow\x20func\
    tions.\n\n\r\n\x05\x05\x01\x02A\x01\x12\x04\x95\x01\x08\n\n\r\n\x05\x05\
    \x01\x02A\x02\x12\x04\x95\x01\r\x11\n\x0c\n\x04\x05\x01\x02B\x12\x04\x96\
    \x01\x08\x16\n\r\n\x05\x05\x01\x02B\x01\x12\x04\x96\x01\x08\x0e\n\r\n\
    \x05\x05\x01\x02B\x02\x12\x04\x96\x01\x11\x15\n\x0c\n\x04\x05\x01\x02C\
    \x12\x04\x97\x01\x08\x16\n\r\n\x05\x05\x01\x02C\x01\x12\x04\x97\x01\x08\
    \x0e\n\r\n\x05\x05\x01\x02C\x02\x12\x04\x97\x01\x11\x15\n\x1f\n\x04\x05\
    \x01\x02D\x12\x04\x9a\x01\x08\x14\x1a\x11\x20Time\x20functions.\n\n\r\n\
    \x05\x05\x01\x02D\x01\x12\x04\x9a\x01\x08\x0c\n\r\n\x05\x05\x01\x02D\x02\
    \x12\x04\x9a\x01\x0f\x13\n\x0c\n\x04\x05\x01\x02E\x12\x04\x9b\x01\x08\
    \x17\n\r\n\x05\x05\x01\x02E\x01\x12\x04\x9b\x01\x08\x0f\n\r\n\x05\x05\
    \x01\x02E\x02\x12\x04\x9b\x01\x12\x16\n\x0c\n\x04\x05\x01\x02F\x12\x04\
    \x9c\x01\x08\x17\n\r\n\x05\x05\x01\x02F\x01\x12\x04\x9c\x01\x08\x0f\n\r\
    \n\x05\x05\x01\x02F\x02\x12\x04\x9c\x01\x12\x16\n\x0c\n\x04\x05\x01\x02G\
    \x12\x04\x9e\x01\x08\x14\n\r\n\x05\x05\x01\x02G\x01\x12\x04\x9e\x01\x08\
    \x0c\n\r\n\x05\x05\x01\x02G\x02\x12\x04\x9e\x01\x0f\x13\n\x0c\n\x04\x05\
    \x01\x02H\x12\x04\x9f\x01\x08\x18\n\r\n\x05\x05\x01\x02H\x01\x12\x04\x9f\
    \x01\x08\x10\n\r\n\x05\x05\x01\x02H\x02\x12\x04\x9f\x01\x13\x17\n\x0c\n\
    \x04\x05\x01\x02I\x12\x04\xa1\x01\x08\x15\n\r\n\x05\x05\x01\x02I\x01\x12\
    \x04\xa1\x01\x08\r\n\r\n\x05\x05\x01\x02I\x02\x12\x04\xa1\x01\x10\x14\n\
    \x0c\n\x04\x05\x01\x02J\x12\x04\xa3\x01\x08\x14\n\r\n\x05\x05\x01\x02J\
    \x01\x12\x04\xa3\x01\x08\x0c\n\r\n\x05\x05\x01\x02J\x02\x12\x04\xa3\x01\
    \x0f\x13\n\x0c\n\x04\x05\x01\x02K\x12\x04\xa4\x01\x08\x17\n\r\n\x05\x05\
    \x01\x02K\x01\x12\x04\xa4\x01\x08\x0f\n\r\n\x05\x05\x01\x02K\x02\x12\x04\
    \xa4\x01\x12\x16\n\x0c\n\x04\x05\x01\x02L\x12\x04\xa5\x01\x08\x1a\n\r\n\
    \x05\x05\x01\x02L\x01\x12\x04\xa5\x01\x08\x12\n\r\n\x05\x05\x01\x02L\x02\
    \x12\x04\xa5\x01\x15\x19\n\x0c\n\x04\x05\x01\x02M\x12\x04\xa7\x01\x08\
    \x13\n\r\n\x05\x05\x01\x02M\x01\x12\x04\xa7\x01\x08\x0b\n\r\n\x05\x05\
    \x01\x02M\x02\x12\x04\xa7\x01\x0e\x12\n\x0c\n\x04\x05\x01\x02N\x12\x04\
    \xa8\x01\x08\x17\n\r\n\x05\x05\x01\x02N\x01\x12\x04\xa8\x01\x08\x0f\n\r\
    \n\x05\x05\x01\x02N\x02\x12\x04\xa8\x01\x12\x16\n\x0c\n\x04\x05\x01\x02O\
    \x12\x04\xa9\x01\x08\x19\n\r\n\x05\x05\x01\x02O\x01\x12\x04\xa9\x01\x08\
    \x11\n\r\n\x05\x05\x01\x02O\x02\x12\x04\xa9\x01\x14\x18\n\x0c\n\x04\x05\
    \x01\x02P\x12\x04\xaa\x01\x08\x1a\n\r\n\x05\x05\x01\x02P\x01\x12\x04\xaa\
    \x01\x08\x12\n\r\n\x05\x05\x01\x02P\x02\x12\x04\xaa\x01\x15\x19\n\x0c\n\
    \x04\x05\x01\x02Q\x12\x04\xab\x01\x08\x19\n\r\n\x05\x05\x01\x02Q\x01\x12\
    \x04\xab\x01\x08\x11\n\r\n\x05\x05\x01\x02Q\x02\x12\x04\xab\x01\x14\x18\
    \n\x0c\n\x04\x05\x01\x02R\x12\x04\xad\x01\x08\x14\n\r\n\x05\x05\x01\x02R\
    \x01\x12\x04\xad\x01\x08\x0c\n\r\n\x05\x05\x01\x02R\x02\x12\x04\xad\x01\
    \x0f\x13\n\x0c\n\x04\x05\x01\x02S\x12\x04\xae\x01\x08\x16\n\r\n\x05\x05\
    \x01\x02S\x01\x12\x04\xae\x01\x08\x0e\n\r\n\x05\x05\x01\x02S\x02\x12\x04\
    \xae\x01\x11\x15\n\x0c\n\x04\x05\x01\x02T\x12\x04\xaf\x01\x08\x16\n\r\n\
    \x05\x05\x01\x02T\x01\x12\x04\xaf\x01\x08\x0e\n\r\n\x05\x05\x01\x02T\x02\
    \x12\x04\xaf\x01\x11\x15\n\x0c\n\x04\x05\x01\x02U\x12\x04\xb0\x01\x08\
    \x1b\n\r\n\x05\x05\x01\x02U\x01\x12\x04\xb0\x01\x08\x13\n\r\n\x05\x05\
    \x01\x02U\x02\x12\x04\xb0\x01\x16\x1a\n\x0c\n\x04\x05\x01\x02V\x12\x04\
    \xb2\x01\x08\x17\n\r\n\x05\x05\x01\x02V\x01\x12\x04\xb2\x01\x08\x0f\n\r\
    \n\x05\x05\x01\x02V\x02\x12\x04\xb2\x01\x12\x16\n\x20\n\x04\x05\x01\x02W\
    \x12\x04\xb5\x01\x08\x18\x1a\x12\x20Other\x20functions;\n\n\r\n\x05\x05\
    \x01\x02W\x01\x12\x04\xb5\x01\x08\x10\n\r\n\x05\x05\x01\x02W\x02\x12\x04\
    \xb5\x01\x13\x17\n\x0c\n\x04\x05\x01\x02X\x12\x04\xb6\x01\x08\x18\n\r\n\
    \x05\x05\x01\x02X\x01\x12\x04\xb6\x01\x08\x10\n\r\n\x05\x05\x01\x02X\x02\
    \x12\x04\xb6\x01\x13\x17\n\x0c\n\x04\x05\x01\x02Y\x12\x04\xb7\x01\x08\
    \x15\n\r\n\x05\x05\x01\x02Y\x01\x12\x04\xb7\x01\x08\r\n\r\n\x05\x05\x01\
    \x02Y\x02\x12\x04\xb7\x01\x10\x14\n\x1f\n\x04\x05\x01\x02Z\x12\x04\xba\
    \x01\x08\x1b\x1a\x11\x20Json\x20functions;\x20\n\r\n\x05\x05\x01\x02Z\
    \x01\x12\x04\xba\x01\x08\x13\n\r\n\x05\x05\x01\x02Z\x02\x12\x04\xba\x01\
    \x16\x1a\n\x0c\n\x04\x05\x01\x02[\x12\x04\xbb\x01\x08\x18\n\r\n\x05\x05\
    \x01\x02[\x01\x12\x04\xbb\x01\x08\x10\n\r\n\x05\x05\x01\x02[\x02\x12\x04\
    \xbb\x01\x13\x17\n\x0c\n\x04\x05\x01\x02\\\x12\x04\xbc\x01\x08\x19\n\r\n\
    \x05\x05\x01\x02\\\x01\x12\x04\xbc\x01\x08\x11\n\r\n\x05\x05\x01\x02\\\
    \x02\x12\x04\xbc\x01\x14\x18\n\x0c\n\x04\x05\x01\x02]\x12\x04\xbd\x01\
    \x08\x1a\n\r\n\x05\x05\x01\x02]\x01\x12\x04\xbd\x01\x08\x12\n\r\n\x05\
    \x05\x01\x02]\x02\x12\x04\xbd\x01\x15\x19\n\x0c\n\x04\x05\x01\x02^\x12\
    \x04\xbe\x01\x08\x19\n\r\n\x05\x05\x01\x02^\x01\x12\x04\xbe\x01\x08\x11\
    \n\r\n\x05\x05\x01\x02^\x02\x12\x04\xbe\x01\x14\x18\n\x0c\n\x04\x05\x01\
    \x02_\x12\x04\xbf\x01\x08\x19\n\r\n\x05\x05\x01\x02_\x01\x12\x04\xbf\x01\
    \x08\x11\n\r\n\x05\x05\x01\x02_\x02\x12\x04\xbf\x01\x14\x18\n\x0c\n\x04\
    \x05\x01\x02`\x12\x04\xc0\x01\x08\x17\n\r\n\x05\x05\x01\x02`\x01\x12\x04\
    \xc0\x01\x08\x0f\n\r\n\x05\x05\x01\x02`\x02\x12\x04\xc0\x01\x12\x16\n\
    \x0c\n\x04\x05\x01\x02a\x12\x04\xc1\x01\x08\x1a\n\r\n\x05\x05\x01\x02a\
    \x01\x12\x04\xc1\x01\x08\x12\n\r\n\x05\x05\x01\x02a\x02\x12\x04\xc1\x01\
    \x15\x19\n\x0c\n\x04\x05\x01\x02b\x12\x04\xc2\x01\x08\x1b\n\r\n\x05\x05\
    \x01\x02b\x01\x12\x04\xc2\x01\x08\x13\n\r\n\x05\x05\x01\x02b\x02\x12\x04\
    \xc2\x01\x16\x1a\n\x0c\n\x04\x05\x01\x02c\x12\x04\xc3\x01\x08\x1a\n\r\n\
    \x05\x05\x01\x02c\x01\x12\x04\xc3\x01\x08\x12\n\r\n\x05\x05\x01\x02c\x02\
    \x12\x04\xc3\x01\x15\x19\n\x0c\n\x04\x05\x01\x02d\x12\x04\xc4\x01\x08\
    \x1c\n\r\n\x05\x05\x01\x02d\x01\x12\x04\xc4\x01\x08\x14\n\r\n\x05\x05\
    \x01\x02d\x02\x12\x04\xc4\x01\x17\x1b\n\x0c\n\x04\x05\x01\x02e\x12\x04\
    \xc5\x01\x08\x1b\n\r\n\x05\x05\x01\x02e\x01\x12\x04\xc5\x01\x08\x13\n\r\
    \n\x05\x05\x01\x02e\x02\x12\x04\xc5\x01\x16\x1a\n\x0c\n\x04\x05\x01\x02f\
    \x12\x04\xc6\x01\x08\x20\n\r\n\x05\x05\x01\x02f\x01\x12\x04\xc6\x01\x08\
    \x18\n\r\n\x05\x05\x01\x02f\x02\x12\x04\xc6\x01\x1b\x1f\n\"\n\x04\x05\
    \x01\x02g\x12\x04\xc9\x01\x08\x12\x1a\x14\x20Other\x20expressions.\x20\n\
    \r\n\x05\x05\x01\x02g\x01\x12\x04\xc9\x01\x08\n\n\r\n\x05\x05\x01\x02g\
    \x02\x12\x04\xc9\x01\r\x11\n\x0c\n\x04\x05\x01\x02h\x12\x04\xca\x01\x08\
    \x17\n\r\n\x05\x05\x01\x02h\x01\x12\x04\xca\x01\x08\x0f\n\r\n\x05\x05\
    \x01\x02h\x02\x12\x04\xca\x01\x12\x16\n\x0c\n\x04\x05\x01\x02i\x12\x04\
    \xcb\x01\x08\x16\n\r\n\x05\x05\x01\x02i\x01\x12\x04\xcb\x01\x08\x0e\n\r\
    \n\x05\x05\x01\x02i\x02\x12\x04\xcb\x01\x11\x15\n\x0c\n\x04\x05\x01\x02j\
    \x12\x04\xcc\x01\x08\x17\n\r\n\x05\x05\x01\x02j\x01\x12\x04\xcc\x01\x08\
    \x0f\n\r\n\x05\x05\x01\x02j\x02\x12\x04\xcc\x01\x12\x16\n\x0c\n\x04\x05\
    \x01\x02k\x12\x04\xcd\x01\x08\x14\n\r\n\x05\x05\x01\x02k\x01\x12\x04\xcd\
    \x01\x08\x0c\n\r\n\x05\x05\x01\x02k\x02\x12\x04\xcd\x01\x0f\x13\n\x0c\n\
    \x04\x05\x01\x02l\x12\x04\xce\x01\x08\x15\n\r\n\x05\x05\x01\x02l\x01\x12\
    \x04\xce\x01\x08\r\n\r\n\x05\x05\x01\x02l\x02\x12\x04\xce\x01\x10\x14\n\
    \x0c\n\x04\x05\x01\x02m\x12\x04\xcf\x01\x08\x14\n\r\n\x05\x05\x01\x02m\
    \x01\x12\x04\xcf\x01\x08\x0c\n\r\n\x05\x05\x01\x02m\x02\x12\x04\xcf\x01\
    \x0f\x13\n\x1f\n\x04\x05\x01\x02n\x12\x04\xd2\x01\x08\x1b\x1a\x11\x20Sca\
    lar\x20Function\x20\n\r\n\x05\x05\x01\x02n\x01\x12\x04\xd2\x01\x08\x12\n\
    \r\n\x05\x05\x01\x02n\x02\x12\x04\xd2\x01\x15\x1a\n\x0c\n\x02\x05\x02\
    \x12\x06\xd5\x01\0\xc0\x02\x01\n\x0b\n\x03\x05\x02\x01\x12\x04\xd5\x01\
    \x05\x12\n\x17\n\x04\x05\x02\x02\0\x12\x04\xd7\x01\x08\x19\x1a\t\x20Cast\
    ing\x20\n\r\n\x05\x05\x02\x02\0\x01\x12\x04\xd7\x01\x08\x14\n\r\n\x05\
    \x05\x02\x02\0\x02\x12\x04\xd7\x01\x17\x18\n\x0c\n\x04\x05\x02\x02\x01\
    \x12\x04\xd8\x01\x08\x1a\n\r\n\x05\x05\x02\x02\x01\x01\x12\x04\xd8\x01\
    \x08\x15\n\r\n\x05\x05\x02\x02\x01\x02\x12\x04\xd8\x01\x18\x19\n\x0c\n\
    \x04\x05\x02\x02\x02\x12\x04\xd9\x01\x08\x1c\n\r\n\x05\x05\x02\x02\x02\
    \x01\x12\x04\xd9\x01\x08\x17\n\r\n\x05\x05\x02\x02\x02\x02\x12\x04\xd9\
    \x01\x1a\x1b\n\x0c\n\x04\x05\x02\x02\x03\x12\x04\xda\x01\x08\x1d\n\r\n\
    \x05\x05\x02\x02\x03\x01\x12\x04\xda\x01\x08\x18\n\r\n\x05\x05\x02\x02\
    \x03\x02\x12\x04\xda\x01\x1b\x1c\n\x0c\n\x04\x05\x02\x02\x04\x12\x04\xdb\
    \x01\x08\x1a\n\r\n\x05\x05\x02\x02\x04\x01\x12\x04\xdb\x01\x08\x15\n\r\n\
    \x05\x05\x02\x02\x04\x02\x12\x04\xdb\x01\x18\x19\n\x0c\n\x04\x05\x02\x02\
    \x05\x12\x04\xdc\x01\x08\x1e\n\r\n\x05\x05\x02\x02\x05\x01\x12\x04\xdc\
    \x01\x08\x19\n\r\n\x05\x05\x02\x02\x05\x02\x12\x04\xdc\x01\x1c\x1d\n\x0c\
    \n\x04\x05\x02\x02\x06\x12\x04\xde\x01\x08\x1b\n\r\n\x05\x05\x02\x02\x06\
    \x01\x12\x04\xde\x01\x08\x15\n\r\n\x05\x05\x02\x02\x06\x02\x12\x04\xde\
    \x01\x18\x1a\n\x0c\n\x04\x05\x02\x02\x07\x12\x04\xdf\x01\x08\x1c\n\r\n\
    \x05\x05\x02\x02\x07\x01\x12\x04\xdf\x01\x08\x16\n\r\n\x05\x05\x02\x02\
    \x07\x02\x12\x04\xdf\x01\x19\x1b\n\x0c\n\x04\x05\x02\x02\x08\x12\x04\xe0\
    \x01\x08\x1e\n\r\n\x05\x05\x02\x02\x08\x01\x12\x04\xe0\x01\x08\x18\n\r\n\
    \x05\x05\x02\x02\x08\x02\x12\x04\xe0\x01\x1b\x1d\n\x0c\n\x04\x05\x02\x02\
    \t\x12\x04\xe1\x01\x08\x1f\n\r\n\x05\x05\x02\x02\t\x01\x12\x04\xe1\x01\
    \x08\x19\n\r\n\x05\x05\x02\x02\t\x02\x12\x04\xe1\x01\x1c\x1e\n\x0c\n\x04\
    \x05\x02\x02\n\x12\x04\xe2\x01\x08\x1c\n\r\n\x05\x05\x02\x02\n\x01\x12\
    \x04\xe2\x01\x08\x16\n\r\n\x05\x05\x02\x02\n\x02\x12\x04\xe2\x01\x19\x1b\
    \n\x0c\n\x04\x05\x02\x02\x0b\x12\x04\xe3\x01\x08\x20\n\r\n\x05\x05\x02\
    \x02\x0b\x01\x12\x04\xe3\x01\x08\x1a\n\r\n\x05\x05\x02\x02\x0b\x02\x12\
    \x04\xe3\x01\x1d\x1f\n\x0c\n\x04\x05\x02\x02\x0c\x12\x04\xe5\x01\x08\x1e\
    \n\r\n\x05\x05\x02\x02\x0c\x01\x12\x04\xe5\x01\x08\x18\n\r\n\x05\x05\x02\
    \x02\x0c\x02\x12\x04\xe5\x01\x1b\x1d\n\x0c\n\x04\x05\x02\x02\r\x12\x04\
    \xe6\x01\x08\x1f\n\r\n\x05\x05\x02\x02\r\x01\x12\x04\xe6\x01\x08\x19\n\r\
    \n\x05\x05\x02\x02\r\x02\x12\x04\xe6\x01\x1c\x1e\n\x0c\n\x04\x05\x02\x02\
    \x0e\x12\x04\xe7\x01\x08!\n\r\n\x05\x05\x02\x02\x0e\x01\x12\x04\xe7\x01\
    \x08\x1b\n\r\n\x05\x05\x02\x02\x0e\x02\x12\x04\xe7\x01\x1e\x20\n\x0c\n\
    \x04\x05\x02\x02\x0f\x12\x04\xe8\x01\x08\"\n\r\n\x05\x05\x02\x02\x0f\x01\
    \x12\x04\xe8\x01\x08\x1c\n\r\n\x05\x05\x02\x02\x0f\x02\x12\x04\xe8\x01\
    \x1f!\n\x0c\n\x04\x05\x02\x02\x10\x12\x04\xe9\x01\x08\x1f\n\r\n\x05\x05\
    \x02\x02\x10\x01\x12\x04\xe9\x01\x08\x19\n\r\n\x05\x05\x02\x02\x10\x02\
    \x12\x04\xe9\x01\x1c\x1e\n\x0c\n\x04\x05\x02\x02\x11\x12\x04\xea\x01\x08\
    #\n\r\n\x05\x05\x02\x02\x11\x01\x12\x04\xea\x01\x08\x1d\n\r\n\x05\x05\
    \x02\x02\x11\x02\x12\x04\xea\x01\x20\"\n\x0c\n\x04\x05\x02\x02\x12\x12\
    \x04\xec\x01\x08\x1d\n\r\n\x05\x05\x02\x02\x12\x01\x12\x04\xec\x01\x08\
    \x17\n\r\n\x05\x05\x02\x02\x12\x02\x12\x04\xec\x01\x1a\x1c\n\x0c\n\x04\
    \x05\x02\x02\x13\x12\x04\xed\x01\x08\x1e\n\r\n\x05\x05\x02\x02\x13\x01\
    \x12\x04\xed\x01\x08\x18\n\r\n\x05\x05\x02\x02\x13\x02\x12\x04\xed\x01\
    \x1b\x1d\n\x0c\n\x04\x05\x02\x02\x14\x12\x04\xee\x01\x08\x20\n\r\n\x05\
    \x05\x02\x02\x14\x01\x12\x04\xee\x01\x08\x1a\n\r\n\x05\x05\x02\x02\x14\
    \x02\x12\x04\xee\x01\x1d\x1f\n\x0c\n\x04\x05\x02\x02\x15\x12\x04\xef\x01\
    \x08!\n\r\n\x05\x05\x02\x02\x15\x01\x12\x04\xef\x01\x08\x1b\n\r\n\x05\
    \x05\x02\x02\x15\x02\x12\x04\xef\x01\x1e\x20\n\x0c\n\x04\x05\x02\x02\x16\
    \x12\x04\xf0\x01\x08\x1e\n\r\n\x05\x05\x02\x02\x16\x01\x12\x04\xf0\x01\
    \x08\x18\n\r\n\x05\x05\x02\x02\x16\x02\x12\x04\xf0\x01\x1b\x1d\n\x0c\n\
    \x04\x05\x02\x02\x17\x12\x04\xf1\x01\x08\"\n\r\n\x05\x05\x02\x02\x17\x01\
    \x12\x04\xf1\x01\x08\x1c\n\r\n\x05\x05\x02\x02\x17\x02\x12\x04\xf1\x01\
    \x1f!\n\x0c\n\x04\x05\x02\x02\x18\x12\x04\xf3\x01\x08\x1b\n\r\n\x05\x05\
    \x02\x02\x18\x01\x12\x04\xf3\x01\x08\x15\n\r\n\x05\x05\x02\x02\x18\x02\
    \x12\x04\xf3\x01\x18\x1a\n\x0c\n\x04\x05\x02\x02\x19\x12\x04\xf4\x01\x08\
    \x1c\n\r\n\x05\x05\x02\x02\x19\x01\x12\x04\xf4\x01\x08\x16\n\r\n\x05\x05\
    \x02\x02\x19\x02\x12\x04\xf4\x01\x19\x1b\n\x0c\n\x04\x05\x02\x02\x1a\x12\
    \x04\xf5\x01\x08\x1e\n\r\n\x05\x05\x02\x02\x1a\x01\x12\x04\xf5\x01\x08\
    \x18\n\r\n\x05\x05\x02\x02\x1a\x02\x12\x04\xf5\x01\x1b\x1d\n\x0c\n\x04\
    \x05\x02\x02\x1b\x12\x04\xf6\x01\x08\x1f\n\r\n\x05\x05\x02\x02\x1b\x01\
    \x12\x04\xf6\x01\x08\x19\n\r\n\x05\x05\x02\x02\x1b\x02\x12\x04\xf6\x01\
    \x1c\x1e\n\x0c\n\x04\x05\x02\x02\x1c\x12\x04\xf7\x01\x08\x1c\n\r\n\x05\
    \x05\x02\x02\x1c\x01\x12\x04\xf7\x01\x08\x16\n\r\n\x05\x05\x02\x02\x1c\
    \x02\x12\x04\xf7\x01\x19\x1b\n\x0c\n\x04\x05\x02\x02\x1d\x12\x04\xf8\x01\
    \x08\x20\n\r\n\x05\x05\x02\x02\x1d\x01\x12\x04\xf8\x01\x08\x1a\n\r\n\x05\
    \x05\x02\x02\x1d\x02\x12\x04\xf8\x01\x1d\x1f\n\x0c\n\x04\x05\x02\x02\x1e\
    \x12\x04\xfa\x01\x08\x1f\n\r\n\x05\x05\x02\x02\x1e\x01\x12\x04\xfa\x01\
    \x08\x19\n\r\n\x05\x05\x02\x02\x1e\x02\x12\x04\xfa\x01\x1c\x1e\n\x0c\n\
    \x04\x05\x02\x02\x1f\x12\x04\xfb\x01\x08\x20\n\r\n\x05\x05\x02\x02\x1f\
    \x01\x12\x04\xfb\x01\x08\x1a\n\r\n\x05\x05\x02\x02\x1f\x02\x12\x04\xfb\
    \x01\x1d\x1f\n\x0c\n\x04\x05\x02\x02\x20\x12\x04\xfc\x01\x08\"\n\r\n\x05\
    \x05\x02\x02\x20\x01\x12\x04\xfc\x01\x08\x1c\n\r\n\x05\x05\x02\x02\x20\
    \x02\x12\x04\xfc\x01\x1f!\n\x0c\n\x04\x05\x02\x02!\x12\x04\xfd\x01\x08#\
    \n\r\n\x05\x05\x02\x02!\x01\x12\x04\xfd\x01\x08\x1d\n\r\n\x05\x05\x02\
    \x02!\x02\x12\x04\xfd\x01\x20\"\n\x0c\n\x04\x05\x02\x02\"\x12\x04\xfe\
    \x01\x08\x20\n\r\n\x05\x05\x02\x02\"\x01\x12\x04\xfe\x01\x08\x1a\n\r\n\
    \x05\x05\x02\x02\"\x02\x12\x04\xfe\x01\x1d\x1f\n\x0c\n\x04\x05\x02\x02#\
    \x12\x04\xff\x01\x08$\n\r\n\x05\x05\x02\x02#\x01\x12\x04\xff\x01\x08\x1e\
    \n\r\n\x05\x05\x02\x02#\x02\x12\x04\xff\x01!#\n\x0c\n\x04\x05\x02\x02$\
    \x12\x04\x81\x02\x08\x14\n\r\n\x05\x05\x02\x02$\x01\x12\x04\x81\x02\x08\
    \r\n\r\n\x05\x05\x02\x02$\x02\x12\x04\x81\x02\x10\x13\n\x0c\n\x04\x05\
    \x02\x02%\x12\x04\x82\x02\x08\x15\n\r\n\x05\x05\x02\x02%\x01\x12\x04\x82\
    \x02\x08\x0e\n\r\n\x05\x05\x02\x02%\x02\x12\x04\x82\x02\x11\x14\n\x0c\n\
    \x04\x05\x02\x02&\x12\x04\x83\x02\x08\x18\n\r\n\x05\x05\x02\x02&\x01\x12\
    \x04\x83\x02\x08\x11\n\r\n\x05\x05\x02\x02&\x02\x12\x04\x83\x02\x14\x17\
    \n\x0c\n\x04\x05\x02\x02'\x12\x04\x84\x02\x08\x17\n\r\n\x05\x05\x02\x02'\
    \x01\x12\x04\x84\x02\x08\x10\n\r\n\x05\x05\x02\x02'\x02\x12\x04\x84\x02\
    \x13\x16\n\x0c\n\x04\x05\x02\x02(\x12\x04\x85\x02\x08\x15\n\r\n\x05\x05\
    \x02\x02(\x01\x12\x04\x85\x02\x08\x0e\n\r\n\x05\x05\x02\x02(\x02\x12\x04\
    \x85\x02\x11\x14\n\x0c\n\x04\x05\x02\x02)\x12\x04\x86\x02\x08\x19\n\r\n\
    \x05\x05\x02\x02)\x01\x12\x04\x86\x02\x08\x12\n\r\n\x05\x05\x02\x02)\x02\
    \x12\x04\x86\x02\x15\x18\n\x0c\n\x04\x05\x02\x02*\x12\x04\x87\x02\x08\
    \x15\n\r\n\x05\x05\x02\x02*\x01\x12\x04\x87\x02\x08\x0e\n\r\n\x05\x05\
    \x02\x02*\x02\x12\x04\x87\x02\x11\x14\n\x0c\n\x04\x05\x02\x02+\x12\x04\
    \x89\x02\x08\x14\n\r\n\x05\x05\x02\x02+\x01\x12\x04\x89\x02\x08\r\n\r\n\
    \x05\x05\x02\x02+\x02\x12\x04\x89\x02\x10\x13\n\x0c\n\x04\x05\x02\x02,\
    \x12\x04\x8a\x02\x08\x15\n\r\n\x05\x05\x02\x02,\x01\x12\x04\x8a\x02\x08\
    \x0e\n\r\n\x05\x05\x02\x02,\x02\x12\x04\x8a\x02\x11\x14\n\x0c\n\x04\x05\
    \x02\x02-\x12\x04\x8b\x02\x08\x18\n\r\n\x05\x05\x02\x02-\x01\x12\x04\x8b\
    \x02\x08\x11\n\r\n\x05\x05\x02\x02-\x02\x12\x04\x8b\x02\x14\x17\n\x0c\n\
    \x04\x05\x02\x02.\x12\x04\x8c\x02\x08\x17\n\r\n\x05\x05\x02\x02.\x01\x12\
    \x04\x8c\x02\x08\x10\n\r\n\x05\x05\x02\x02.\x02\x12\x04\x8c\x02\x13\x16\
    \n\x0c\n\x04\x05\x02\x02/\x12\x04\x8d\x02\x08\x15\n\r\n\x05\x05\x02\x02/\
    \x01\x12\x04\x8d\x02\x08\x0e\n\r\n\x05\x05\x02\x02/\x02\x12\x04\x8d\x02\
    \x11\x14\n\x0c\n\x04\x05\x02\x020\x12\x04\x8e\x02\x08\x19\n\r\n\x05\x05\
    \x02\x020\x01\x12\x04\x8e\x02\x08\x12\n\r\n\x05\x05\x02\x020\x02\x12\x04\
    \x8e\x02\x15\x18\n\x0c\n\x04\x05\x02\x021\x12\x04\x8f\x02\x08\x15\n\r\n\
    \x05\x05\x02\x021\x01\x12\x04\x8f\x02\x08\x0e\n\r\n\x05\x05\x02\x021\x02\
    \x12\x04\x8f\x02\x11\x14\n\x0c\n\x04\x05\x02\x022\x12\x04\x91\x02\x08\
    \x14\n\r\n\x05\x05\x02\x022\x01\x12\x04\x91\x02\x08\r\n\r\n\x05\x05\x02\
    \x022\x02\x12\x04\x91\x02\x10\x13\n\x0c\n\x04\x05\x02\x023\x12\x04\x92\
    \x02\x08\x15\n\r\n\x05\x05\x02\x023\x01\x12\x04\x92\x02\x08\x0e\n\r\n\
    \x05\x05\x02\x023\x02\x12\x04\x92\x02\x11\x14\n\x0c\n\x04\x05\x02\x024\
    \x12\x04\x93\x02\x08\x18\n\r\n\x05\x05\x02\x024\x01\x12\x04\x93\x02\x08\
    \x11\n\r\n\x05\x05\x02\x024\x02\x12\x04\x93\x02\x14\x17\n\x0c\n\x04\x05\
    \x02\x025\x12\x04\x94\x02\x08\x17\n\r\n\x05\x05\x02\x025\x01\x12\x04\x94\
    \x02\x08\x10\n\r\n\x05\x05\x02\x025\x02\x12\x04\x94\x02\x13\x16\n\x0c\n\
    \x04\x05\x02\x026\x12\x04\x95\x02\x08\x15\n\r\n\x05\x05\x02\x026\x01\x12\
    \x04\x95\x02\x08\x0e\n\r\n\x05\x05\x02\x026\x02\x12\x04\x95\x02\x11\x14\
    \n\x0c\n\x04\x05\x02\x027\x12\x04\x96\x02\x08\x19\n\r\n\x05\x05\x02\x027\
    \x01\x12\x04\x96\x02\x08\x12\n\r\n\x05\x05\x02\x027\x02\x12\x04\x96\x02\
    \x15\x18\n\x0c\n\x04\x05\x02\x028\x12\x04\x97\x02\x08\x15\n\r\n\x05\x05\
    \x02\x028\x01\x12\x04\x97\x02\x08\x0e\n\r\n\x05\x05\x02\x028\x02\x12\x04\
    \x97\x02\x11\x14\n\x0c\n\x04\x05\x02\x029\x12\x04\x99\x02\x08\x14\n\r\n\
    \x05\x05\x02\x029\x01\x12\x04\x99\x02\x08\r\n\r\n\x05\x05\x02\x029\x02\
    \x12\x04\x99\x02\x10\x13\n\x0c\n\x04\x05\x02\x02:\x12\x04\x9a\x02\x08\
    \x15\n\r\n\x05\x05\x02\x02:\x01\x12\x04\x9a\x02\x08\x0e\n\r\n\x05\x05\
    \x02\x02:\x02\x12\x04\x9a\x02\x11\x14\n\x0c\n\x04\x05\x02\x02;\x12\x04\
    \x9b\x02\x08\x18\n\r\n\x05\x05\x02\x02;\x01\x12\x04\x9b\x02\x08\x11\n\r\
    \n\x05\x05\x02\x02;\x02\x12\x04\x9b\x02\x14\x17\n\x0c\n\x04\x05\x02\x02<\
    \x12\x04\x9c\x02\x08\x17\n\r\n\x05\x05\x02\x02<\x01\x12\x04\x9c\x02\x08\
    \x10\n\r\n\x05\x05\x02\x02<\x02\x12\x04\x9c\x02\x13\x16\n\x0c\n\x04\x05\
    \x02\x02=\x12\x04\x9d\x02\x08\x15\n\r\n\x05\x05\x02\x02=\x01\x12\x04\x9d\
    \x02\x08\x0e\n\r\n\x05\x05\x02\x02=\x02\x12\x04\x9d\x02\x11\x14\n\x0c\n\
    \x04\x05\x02\x02>\x12\x04\x9e\x02\x08\x19\n\r\n\x05\x05\x02\x02>\x01\x12\
    \x04\x9e\x02\x08\x12\n\r\n\x05\x05\x02\x02>\x02\x12\x04\x9e\x02\x15\x18\
    \n\x0c\n\x04\x05\x02\x02?\x12\x04\x9f\x02\x08\x15\n\r\n\x05\x05\x02\x02?\
    \x01\x12\x04\x9f\x02\x08\x0e\n\r\n\x05\x05\x02\x02?\x02\x12\x04\x9f\x02\
    \x11\x14\n\x0c\n\x04\x05\x02\x02@\x12\x04\xa1\x02\x08\x14\n\r\n\x05\x05\
    \x02\x02@\x01\x12\x04\xa1\x02\x08\r\n\r\n\x05\x05\x02\x02@\x02\x12\x04\
    \xa1\x02\x10\x13\n\x0c\n\x04\x05\x02\x02A\x12\x04\xa2\x02\x08\x15\n\r\n\
    \x05\x05\x02\x02A\x01\x12\x04\xa2\x02\x08\x0e\n\r\n\x05\x05\x02\x02A\x02\
    \x12\x04\xa2\x02\x11\x14\n\x0c\n\x04\x05\x02\x02B\x12\x04\xa3\x02\x08\
    \x18\n\r\n\x05\x05\x02\x02B\x01\x12\x04\xa3\x02\x08\x11\n\r\n\x05\x05\
    \x02\x02B\x02\x12\x04\xa3\x02\x14\x17\n\x0c\n\x04\x05\x02\x02C\x12\x04\
    \xa4\x02\x08\x17\n\r\n\x05\x05\x02\x02C\x01\x12\x04\xa4\x02\x08\x10\n\r\
    \n\x05\x05\x02\x02C\x02\x12\x04\xa4\x02\x13\x16\n\x0c\n\x04\x05\x02\x02D\
    \x12\x04\xa5\x02\x08\x15\n\r\n\x05\x05\x02\x02D\x01\x12\x04\xa5\x02\x08\
    \x0e\n\r\n\x05\x05\x02\x02D\x02\x12\x04\xa5\x02\x11\x14\n\x0c\n\x04\x05\
    \x02\x02E\x12\x04\xa6\x02\x08\x19\n\r\n\x05\x05\x02\x02E\x01\x12\x04\xa6\
    \x02\x08\x12\n\r\n\x05\x05\x02\x02E\x02\x12\x04\xa6\x02\x15\x18\n\x0c\n\
    \x04\x05\x02\x02F\x12\x04\xa7\x02\x08\x15\n\r\n\x05\x05\x02\x02F\x01\x12\
    \x04\xa7\x02\x08\x0e\n\r\n\x05\x05\x02\x02F\x02\x12\x04\xa7\x02\x11\x14\
    \n\x0c\n\x04\x05\x02\x02G\x12\x04\xa9\x02\x08\x14\n\r\n\x05\x05\x02\x02G\
    \x01\x12\x04\xa9\x02\x08\r\n\r\n\x05\x05\x02\x02G\x02\x12\x04\xa9\x02\
    \x10\x13\n\x0c\n\x04\x05\x02\x02H\x12\x04\xaa\x02\x08\x15\n\r\n\x05\x05\
    \x02\x02H\x01\x12\x04\xaa\x02\x08\x0e\n\r\n\x05\x05\x02\x02H\x02\x12\x04\
    \xaa\x02\x11\x14\n\x0c\n\x04\x05\x02\x02I\x12\x04\xab\x02\x08\x18\n\r\n\
    \x05\x05\x02\x02I\x01\x12\x04\xab\x02\x08\x11\n\r\n\x05\x05\x02\x02I\x02\
    \x12\x04\xab\x02\x14\x17\n\x0c\n\x04\x05\x02\x02J\x12\x04\xac\x02\x08\
    \x17\n\r\n\x05\x05\x02\x02J\x01\x12\x04\xac\x02\x08\x10\n\r\n\x05\x05\
    \x02\x02J\x02\x12\x04\xac\x02\x13\x16\n\x0c\n\x04\x05\x02\x02K\x12\x04\
    \xad\x02\x08\x15\n\r\n\x05\x05\x02\x02K\x01\x12\x04\xad\x02\x08\x0e\n\r\
    \n\x05\x05\x02\x02K\x02\x12\x04\xad\x02\x11\x14\n\x0c\n\x04\x05\x02\x02L\
    \x12\x04\xae\x02\x08\x19\n\r\n\x05\x05\x02\x02L\x01\x12\x04\xae\x02\x08\
    \x12\n\r\n\x05\x05\x02\x02L\x02\x12\x04\xae\x02\x15\x18\n\x0c\n\x04\x05\
    \x02\x02M\x12\x04\xaf\x02\x08\x15\n\r\n\x05\x05\x02\x02M\x01\x12\x04\xaf\
    \x02\x08\x0e\n\r\n\x05\x05\x02\x02M\x02\x12\x04\xaf\x02\x11\x14\n\x0c\n\
    \x04\x05\x02\x02N\x12\x04\xb1\x02\x08\x18\n\r\n\x05\x05\x02\x02N\x01\x12\
    \x04\xb1\x02\x08\x11\n\r\n\x05\x05\x02\x02N\x02\x12\x04\xb1\x02\x14\x17\
    \n\x0c\n\x04\x05\x02\x02O\x12\x04\xb2\x02\x08\x19\n\r\n\x05\x05\x02\x02O\
    \x01\x12\x04\xb2\x02\x08\x12\n\r\n\x05\x05\x02\x02O\x02\x12\x04\xb2\x02\
    \x15\x18\n\x0c\n\x04\x05\x02\x02P\x12\x04\xb3\x02\x08\x1c\n\r\n\x05\x05\
    \x02\x02P\x01\x12\x04\xb3\x02\x08\x15\n\r\n\x05\x05\x02\x02P\x02\x12\x04\
    \xb3\x02\x18\x1b\n\x0c\n\x04\x05\x02\x02Q\x12\x04\xb4\x02\x08\x1b\n\r\n\
    \x05\x05\x02\x02Q\x01\x12\x04\xb4\x02\x08\x14\n\r\n\x05\x05\x02\x02Q\x02\
    \x12\x04\xb4\x02\x17\x1a\n\x0c\n\x04\x05\x02\x02R\x12\x04\xb5\x02\x08\
    \x19\n\r\n\x05\x05\x02\x02R\x01\x12\x04\xb5\x02\x08\x12\n\r\n\x05\x05\
    \x02\x02R\x02\x12\x04\xb5\x02\x15\x18\n\x0c\n\x04\x05\x02\x02S\x12\x04\
    \xb6\x02\x08\x1d\n\r\n\x05\x05\x02\x02S\x01\x12\x04\xb6\x02\x08\x16\n\r\
    \n\x05\x05\x02\x02S\x02\x12\x04\xb6\x02\x19\x1c\n\x0c\n\x04\x05\x02\x02T\
    \x12\x04\xb7\x02\x08\x19\n\r\n\x05\x05\x02\x02T\x01\x12\x04\xb7\x02\x08\
    \x12\n\r\n\x05\x05\x02\x02T\x02\x12\x04\xb7\x02\x15\x18\n\x0c\n\x04\x05\
    \x02\x02U\x12\x04\xb9\x02\x08\x16\n\r\n\x05\x05\x02\x02U\x01\x12\x04\xb9\
    \x02\x08\x0e\n\r\n\x05\x05\x02\x02U\x02\x12\x04\xb9\x02\x11\x15\n\x0c\n\
    \x04\x05\x02\x02V\x12\x04\xba\x02\x08\x17\n\r\n\x05\x05\x02\x02V\x01\x12\
    \x04\xba\x02\x08\x0f\n\r\n\x05\x05\x02\x02V\x02\x12\x04\xba\x02\x12\x16\
    \n\x0c\n\x04\x05\x02\x02W\x12\x04\xbb\x02\x08\x1a\n\r\n\x05\x05\x02\x02W\
    \x01\x12\x04\xbb\x02\x08\x12\n\r\n\x05\x05\x02\x02W\x02\x12\x04\xbb\x02\
    \x15\x19\n\x0c\n\x04\x05\x02\x02X\x12\x04\xbc\x02\x08\x17\n\r\n\x05\x05\
    \x02\x02X\x01\x12\x04\xbc\x02\x08\x0f\n\r\n\x05\x05\x02\x02X\x02\x12\x04\
    \xbc\x02\x12\x16\n\x0c\n\x04\x05\x02\x02Y\x12\x04\xbd\x02\x08\x18\n\r\n\
    \x05\x05\x02\x02Y\x01\x12\x04\xbd\x02\x08\x10\n\r\n\x05\x05\x02\x02Y\x02\
    \x12\x04\xbd\x02\x13\x17\n\x0c\n\x04\x05\x02\x02Z\x12\x04\xbe\x02\x08\
    \x18\n\r\n\x05\x05\x02\x02Z\x01\x12\x04\xbe\x02\x08\x10\n\r\n\x05\x05\
    \x02\x02Z\x02\x12\x04\xbe\x02\x13\x17\n\x0c\n\x04\x05\x02\x02[\x12\x04\
    \xbf\x02\x08\x19\n\r\n\x05\x05\x02\x02[\x01\x12\x04\xbf\x02\x08\x11\n\r\
    \n\x05\x05\x02\x02[\x02\x12\x04\xbf\x02\x14\x18\n[\n\x02\x04\x01\x12\x06\
    \xc3\x02\0\xc9\x02\x01\x1aM\x20Evaluators\x20should\x20implement\x20eval\
    uation\x20functions\x20for\x20every\x20expression\x20type.\n\n\x0b\n\x03\
    \x04\x01\x01\x12\x04\xc3\x02\x08\x0c\n\x0c\n\x04\x04\x01\x02\0\x12\x04\
    \xc4\x02\x08@\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\xc4\x02\x08\x10\n\r\n\
    \x05\x04\x01\x02\0\x06\x12\x04\xc4\x02\x11\x19\n\r\n\x05\x04\x01\x02\0\
    \x01\x12\x04\xc4\x02\x1a\x1c\n\r\n\x05\x04\x01\x02\0\x03\x12\x04\xc4\x02\
    \x1f\x20\n\r\n\x05\x04\x01\x02\0\x08\x12\x04\xc4\x02!?\n\x10\n\x08\x04\
    \x01\x02\0\x08\xe7\x07\0\x12\x04\xc4\x02\">\n\x11\n\t\x04\x01\x02\0\x08\
    \xe7\x07\0\x02\x12\x04\xc4\x02\"6\n\x12\n\n\x04\x01\x02\0\x08\xe7\x07\0\
    \x02\0\x12\x04\xc4\x02\"6\n\x13\n\x0b\x04\x01\x02\0\x08\xe7\x07\0\x02\0\
    \x01\x12\x04\xc4\x02#5\n\x11\n\t\x04\x01\x02\0\x08\xe7\x07\0\x03\x12\x04\
    \xc4\x029>\n\x0c\n\x04\x04\x01\x02\x01\x12\x04\xc5\x02\x08\x1f\n\r\n\x05\
    \x04\x01\x02\x01\x04\x12\x04\xc5\x02\x08\x10\n\r\n\x05\x04\x01\x02\x01\
    \x05\x12\x04\xc5\x02\x11\x16\n\r\n\x05\x04\x01\x02\x01\x01\x12\x04\xc5\
    \x02\x17\x1a\n\r\n\x05\x04\x01\x02\x01\x03\x12\x04\xc5\x02\x1d\x1e\n\x0c\
    \n\x04\x04\x01\x02\x02\x12\x04\xc6\x02\x08#\n\r\n\x05\x04\x01\x02\x02\
    \x04\x12\x04\xc6\x02\x08\x10\n\r\n\x05\x04\x01\x02\x02\x06\x12\x04\xc6\
    \x02\x11\x15\n\r\n\x05\x04\x01\x02\x02\x01\x12\x04\xc6\x02\x16\x1e\n\r\n\
    \x05\x04\x01\x02\x02\x03\x12\x04\xc6\x02!\"\n\x0c\n\x04\x04\x01\x02\x03\
    \x12\x04\xc7\x02\x08'\n\r\n\x05\x04\x01\x02\x03\x04\x12\x04\xc7\x02\x08\
    \x10\n\r\n\x05\x04\x01\x02\x03\x06\x12\x04\xc7\x02\x11\x1e\n\r\n\x05\x04\
    \x01\x02\x03\x01\x12\x04\xc7\x02\x1f\"\n\r\n\x05\x04\x01\x02\x03\x03\x12\
    \x04\xc7\x02%&\n\x0c\n\x04\x04\x01\x02\x04\x12\x04\xc8\x02\x08*\n\r\n\
    \x05\x04\x01\x02\x04\x04\x12\x04\xc8\x02\x08\x10\n\r\n\x05\x04\x01\x02\
    \x04\x06\x12\x04\xc8\x02\x11\x1a\n\r\n\x05\x04\x01\x02\x04\x01\x12\x04\
    \xc8\x02\x1b%\n\r\n\x05\x04\x01\x02\x04\x03\x12\x04\xc8\x02()\n6\n\x02\
    \x04\x02\x12\x06\xcc\x02\0\xcf\x02\x01\x1a(\x20ByItem\x20type\x20for\x20\
    group\x20by\x20and\x20order\x20by.\n\n\x0b\n\x03\x04\x02\x01\x12\x04\xcc\
    \x02\x08\x0e\n\x0c\n\x04\x04\x02\x02\0\x12\x04\xcd\x02\x08\x1f\n\r\n\x05\
    \x04\x02\x02\0\x04\x12\x04\xcd\x02\x08\x10\n\r\n\x05\x04\x02\x02\0\x06\
    \x12\x04\xcd\x02\x11\x15\n\r\n\x05\x04\x02\x02\0\x01\x12\x04\xcd\x02\x16\
    \x1a\n\r\n\x05\x04\x02\x02\0\x03\x12\x04\xcd\x02\x1d\x1e\n\x0c\n\x04\x04\
    \x02\x02\x01\x12\x04\xce\x02\x08>\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\
    \xce\x02\x08\x10\n\r\n\x05\x04\x02\x02\x01\x05\x12\x04\xce\x02\x11\x15\n\
    \r\n\x05\x04\x02\x02\x01\x01\x12\x04\xce\x02\x16\x1a\n\r\n\x05\x04\x02\
    \x02\x01\x03\x12\x04\xce\x02\x1d\x1e\n\r\n\x05\x04\x02\x02\x01\x08\x12\
    \x04\xce\x02\x1f=\n\x10\n\x08\x04\x02\x02\x01\x08\xe7\x07\0\x12\x04\xce\
    \x02\x20<\n\x11\n\t\x04\x02\x02\x01\x08\xe7\x07\0\x02\x12\x04\xce\x02\
    \x204\n\x12\n\n\x04\x02\x02\x01\x08\xe7\x07\0\x02\0\x12\x04\xce\x02\x204\
    \n\x13\n\x0b\x04\x02\x02\x01\x08\xe7\x07\0\x02\0\x01\x12\x04\xce\x02!3\n\
    \x11\n\t\x04\x02\x02\x01\x08\xe7\x07\0\x03\x12\x04\xce\x027<\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
