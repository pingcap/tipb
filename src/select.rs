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
pub struct Row {
    // message fields
    handle: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Row {}

impl Row {
    pub fn new() -> Row {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Row {
        static mut instance: ::protobuf::lazy::Lazy<Row> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Row,
        };
        unsafe {
            instance.get(Row::new)
        }
    }

    // optional bytes handle = 1;

    pub fn clear_handle(&mut self) {
        self.handle.clear();
    }

    pub fn has_handle(&self) -> bool {
        self.handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_handle(&mut self, v: ::std::vec::Vec<u8>) {
        self.handle = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_handle(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.handle.is_none() {
            self.handle.set_default();
        }
        self.handle.as_mut().unwrap()
    }

    // Take field
    pub fn take_handle(&mut self) -> ::std::vec::Vec<u8> {
        self.handle.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_handle(&self) -> &[u8] {
        match self.handle.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_handle_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.handle
    }

    fn mut_handle_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.handle
    }

    // optional bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }
}

impl ::protobuf::Message for Row {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.handle)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
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
        if let Some(ref v) = self.handle.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.handle.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.data.as_ref() {
            os.write_bytes(2, &v)?;
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

impl ::protobuf::MessageStatic for Row {
    fn new() -> Row {
        Row::new()
    }

    fn descriptor_static(_: ::std::option::Option<Row>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "handle",
                    Row::get_handle_for_reflect,
                    Row::mut_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    Row::get_data_for_reflect,
                    Row::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Row>(
                    "Row",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Row {
    fn clear(&mut self) {
        self.clear_handle();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Row {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Row {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Error {
    // message fields
    code: ::std::option::Option<i32>,
    msg: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Error {}

impl Error {
    pub fn new() -> Error {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Error {
        static mut instance: ::protobuf::lazy::Lazy<Error> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Error,
        };
        unsafe {
            instance.get(Error::new)
        }
    }

    // optional int32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = ::std::option::Option::None;
    }

    pub fn has_code(&self) -> bool {
        self.code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: i32) {
        self.code = ::std::option::Option::Some(v);
    }

    pub fn get_code(&self) -> i32 {
        self.code.unwrap_or(0)
    }

    fn get_code_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.code
    }

    // optional string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        if self.msg.is_none() {
            self.msg.set_default();
        }
        self.msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        self.msg.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        match self.msg.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_msg_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.msg
    }
}

impl ::protobuf::Message for Error {
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
                    let tmp = is.read_int32()?;
                    self.code = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.msg)?;
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
        if let Some(v) = self.code {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.msg.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.code {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.msg.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for Error {
    fn new() -> Error {
        Error::new()
    }

    fn descriptor_static(_: ::std::option::Option<Error>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "code",
                    Error::get_code_for_reflect,
                    Error::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    Error::get_msg_for_reflect,
                    Error::mut_msg_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Error>(
                    "Error",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Error {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Error {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SelectResponse {
    // message fields
    error: ::protobuf::SingularPtrField<Error>,
    rows: ::protobuf::RepeatedField<Row>,
    chunks: ::protobuf::RepeatedField<Chunk>,
    warnings: ::protobuf::RepeatedField<Error>,
    output_counts: ::std::vec::Vec<i64>,
    warning_count: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SelectResponse {}

impl SelectResponse {
    pub fn new() -> SelectResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SelectResponse {
        static mut instance: ::protobuf::lazy::Lazy<SelectResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SelectResponse,
        };
        unsafe {
            instance.get(SelectResponse::new)
        }
    }

    // optional .tipb.Error error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: Error) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut Error {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> Error {
        self.error.take().unwrap_or_else(|| Error::new())
    }

    pub fn get_error(&self) -> &Error {
        self.error.as_ref().unwrap_or_else(|| Error::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<Error> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Error> {
        &mut self.error
    }

    // repeated .tipb.Row rows = 2;

    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    // Param is passed by value, moved
    pub fn set_rows(&mut self, v: ::protobuf::RepeatedField<Row>) {
        self.rows = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rows(&mut self) -> &mut ::protobuf::RepeatedField<Row> {
        &mut self.rows
    }

    // Take field
    pub fn take_rows(&mut self) -> ::protobuf::RepeatedField<Row> {
        ::std::mem::replace(&mut self.rows, ::protobuf::RepeatedField::new())
    }

    pub fn get_rows(&self) -> &[Row] {
        &self.rows
    }

    fn get_rows_for_reflect(&self) -> &::protobuf::RepeatedField<Row> {
        &self.rows
    }

    fn mut_rows_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Row> {
        &mut self.rows
    }

    // repeated .tipb.Chunk chunks = 3;

    pub fn clear_chunks(&mut self) {
        self.chunks.clear();
    }

    // Param is passed by value, moved
    pub fn set_chunks(&mut self, v: ::protobuf::RepeatedField<Chunk>) {
        self.chunks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chunks(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }

    // Take field
    pub fn take_chunks(&mut self) -> ::protobuf::RepeatedField<Chunk> {
        ::std::mem::replace(&mut self.chunks, ::protobuf::RepeatedField::new())
    }

    pub fn get_chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    fn get_chunks_for_reflect(&self) -> &::protobuf::RepeatedField<Chunk> {
        &self.chunks
    }

    fn mut_chunks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }

    // repeated .tipb.Error warnings = 4;

    pub fn clear_warnings(&mut self) {
        self.warnings.clear();
    }

    // Param is passed by value, moved
    pub fn set_warnings(&mut self, v: ::protobuf::RepeatedField<Error>) {
        self.warnings = v;
    }

    // Mutable pointer to the field.
    pub fn mut_warnings(&mut self) -> &mut ::protobuf::RepeatedField<Error> {
        &mut self.warnings
    }

    // Take field
    pub fn take_warnings(&mut self) -> ::protobuf::RepeatedField<Error> {
        ::std::mem::replace(&mut self.warnings, ::protobuf::RepeatedField::new())
    }

    pub fn get_warnings(&self) -> &[Error] {
        &self.warnings
    }

    fn get_warnings_for_reflect(&self) -> &::protobuf::RepeatedField<Error> {
        &self.warnings
    }

    fn mut_warnings_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Error> {
        &mut self.warnings
    }

    // repeated int64 output_counts = 5;

    pub fn clear_output_counts(&mut self) {
        self.output_counts.clear();
    }

    // Param is passed by value, moved
    pub fn set_output_counts(&mut self, v: ::std::vec::Vec<i64>) {
        self.output_counts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_output_counts(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.output_counts
    }

    // Take field
    pub fn take_output_counts(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.output_counts, ::std::vec::Vec::new())
    }

    pub fn get_output_counts(&self) -> &[i64] {
        &self.output_counts
    }

    fn get_output_counts_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.output_counts
    }

    fn mut_output_counts_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.output_counts
    }

    // optional int64 warning_count = 6;

    pub fn clear_warning_count(&mut self) {
        self.warning_count = ::std::option::Option::None;
    }

    pub fn has_warning_count(&self) -> bool {
        self.warning_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_warning_count(&mut self, v: i64) {
        self.warning_count = ::std::option::Option::Some(v);
    }

    pub fn get_warning_count(&self) -> i64 {
        self.warning_count.unwrap_or(0)
    }

    fn get_warning_count_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.warning_count
    }

    fn mut_warning_count_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.warning_count
    }
}

impl ::protobuf::Message for SelectResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.error {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.rows {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.chunks {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.warnings {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rows)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.chunks)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.warnings)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.output_counts)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.warning_count = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.rows {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.chunks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.warnings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.output_counts {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.warning_count {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.rows {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.chunks {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.warnings {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.output_counts {
            os.write_int64(5, *v)?;
        };
        if let Some(v) = self.warning_count {
            os.write_int64(6, v)?;
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

impl ::protobuf::MessageStatic for SelectResponse {
    fn new() -> SelectResponse {
        SelectResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SelectResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Error>>(
                    "error",
                    SelectResponse::get_error_for_reflect,
                    SelectResponse::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Row>>(
                    "rows",
                    SelectResponse::get_rows_for_reflect,
                    SelectResponse::mut_rows_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Chunk>>(
                    "chunks",
                    SelectResponse::get_chunks_for_reflect,
                    SelectResponse::mut_chunks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Error>>(
                    "warnings",
                    SelectResponse::get_warnings_for_reflect,
                    SelectResponse::mut_warnings_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "output_counts",
                    SelectResponse::get_output_counts_for_reflect,
                    SelectResponse::mut_output_counts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "warning_count",
                    SelectResponse::get_warning_count_for_reflect,
                    SelectResponse::mut_warning_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SelectResponse>(
                    "SelectResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SelectResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_rows();
        self.clear_chunks();
        self.clear_warnings();
        self.clear_output_counts();
        self.clear_warning_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SelectResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SelectResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Chunk {
    // message fields
    rows_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    rows_meta: ::protobuf::RepeatedField<RowMeta>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Chunk {}

impl Chunk {
    pub fn new() -> Chunk {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Chunk {
        static mut instance: ::protobuf::lazy::Lazy<Chunk> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Chunk,
        };
        unsafe {
            instance.get(Chunk::new)
        }
    }

    // optional bytes rows_data = 3;

    pub fn clear_rows_data(&mut self) {
        self.rows_data.clear();
    }

    pub fn has_rows_data(&self) -> bool {
        self.rows_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rows_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.rows_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rows_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.rows_data.is_none() {
            self.rows_data.set_default();
        }
        self.rows_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_rows_data(&mut self) -> ::std::vec::Vec<u8> {
        self.rows_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_rows_data(&self) -> &[u8] {
        match self.rows_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_rows_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.rows_data
    }

    fn mut_rows_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.rows_data
    }

    // repeated .tipb.RowMeta rows_meta = 4;

    pub fn clear_rows_meta(&mut self) {
        self.rows_meta.clear();
    }

    // Param is passed by value, moved
    pub fn set_rows_meta(&mut self, v: ::protobuf::RepeatedField<RowMeta>) {
        self.rows_meta = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rows_meta(&mut self) -> &mut ::protobuf::RepeatedField<RowMeta> {
        &mut self.rows_meta
    }

    // Take field
    pub fn take_rows_meta(&mut self) -> ::protobuf::RepeatedField<RowMeta> {
        ::std::mem::replace(&mut self.rows_meta, ::protobuf::RepeatedField::new())
    }

    pub fn get_rows_meta(&self) -> &[RowMeta] {
        &self.rows_meta
    }

    fn get_rows_meta_for_reflect(&self) -> &::protobuf::RepeatedField<RowMeta> {
        &self.rows_meta
    }

    fn mut_rows_meta_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RowMeta> {
        &mut self.rows_meta
    }
}

impl ::protobuf::Message for Chunk {
    fn is_initialized(&self) -> bool {
        for v in &self.rows_meta {
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
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.rows_data)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rows_meta)?;
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
        if let Some(ref v) = self.rows_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        for value in &self.rows_meta {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.rows_data.as_ref() {
            os.write_bytes(3, &v)?;
        }
        for v in &self.rows_meta {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for Chunk {
    fn new() -> Chunk {
        Chunk::new()
    }

    fn descriptor_static(_: ::std::option::Option<Chunk>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "rows_data",
                    Chunk::get_rows_data_for_reflect,
                    Chunk::mut_rows_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RowMeta>>(
                    "rows_meta",
                    Chunk::get_rows_meta_for_reflect,
                    Chunk::mut_rows_meta_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Chunk>(
                    "Chunk",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Chunk {
    fn clear(&mut self) {
        self.clear_rows_data();
        self.clear_rows_meta();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Chunk {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RowMeta {
    // message fields
    handle: ::std::option::Option<i64>,
    length: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RowMeta {}

impl RowMeta {
    pub fn new() -> RowMeta {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RowMeta {
        static mut instance: ::protobuf::lazy::Lazy<RowMeta> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RowMeta,
        };
        unsafe {
            instance.get(RowMeta::new)
        }
    }

    // optional int64 handle = 1;

    pub fn clear_handle(&mut self) {
        self.handle = ::std::option::Option::None;
    }

    pub fn has_handle(&self) -> bool {
        self.handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_handle(&mut self, v: i64) {
        self.handle = ::std::option::Option::Some(v);
    }

    pub fn get_handle(&self) -> i64 {
        self.handle.unwrap_or(0)
    }

    fn get_handle_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.handle
    }

    fn mut_handle_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.handle
    }

    // optional int64 length = 2;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: i64) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length(&self) -> i64 {
        self.length.unwrap_or(0)
    }

    fn get_length_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.length
    }
}

impl ::protobuf::Message for RowMeta {
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
                    let tmp = is.read_int64()?;
                    self.handle = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.length = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.handle {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.handle {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.length {
            os.write_int64(2, v)?;
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

impl ::protobuf::MessageStatic for RowMeta {
    fn new() -> RowMeta {
        RowMeta::new()
    }

    fn descriptor_static(_: ::std::option::Option<RowMeta>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "handle",
                    RowMeta::get_handle_for_reflect,
                    RowMeta::mut_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "length",
                    RowMeta::get_length_for_reflect,
                    RowMeta::mut_length_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RowMeta>(
                    "RowMeta",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RowMeta {
    fn clear(&mut self) {
        self.clear_handle();
        self.clear_length();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RowMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RowMeta {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DAGRequest {
    // message fields
    start_ts: ::std::option::Option<u64>,
    executors: ::protobuf::RepeatedField<super::executor::Executor>,
    time_zone_offset: ::std::option::Option<i64>,
    flags: ::std::option::Option<u64>,
    output_offsets: ::std::vec::Vec<u32>,
    collect_range_counts: ::std::option::Option<bool>,
    max_warning_count: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DAGRequest {}

impl DAGRequest {
    pub fn new() -> DAGRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DAGRequest {
        static mut instance: ::protobuf::lazy::Lazy<DAGRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DAGRequest,
        };
        unsafe {
            instance.get(DAGRequest::new)
        }
    }

    // optional uint64 start_ts = 1;

    pub fn clear_start_ts(&mut self) {
        self.start_ts = ::std::option::Option::None;
    }

    pub fn has_start_ts(&self) -> bool {
        self.start_ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = ::std::option::Option::Some(v);
    }

    pub fn get_start_ts(&self) -> u64 {
        self.start_ts.unwrap_or(0)
    }

    fn get_start_ts_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start_ts
    }

    fn mut_start_ts_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start_ts
    }

    // repeated .tipb.Executor executors = 2;

    pub fn clear_executors(&mut self) {
        self.executors.clear();
    }

    // Param is passed by value, moved
    pub fn set_executors(&mut self, v: ::protobuf::RepeatedField<super::executor::Executor>) {
        self.executors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_executors(&mut self) -> &mut ::protobuf::RepeatedField<super::executor::Executor> {
        &mut self.executors
    }

    // Take field
    pub fn take_executors(&mut self) -> ::protobuf::RepeatedField<super::executor::Executor> {
        ::std::mem::replace(&mut self.executors, ::protobuf::RepeatedField::new())
    }

    pub fn get_executors(&self) -> &[super::executor::Executor] {
        &self.executors
    }

    fn get_executors_for_reflect(&self) -> &::protobuf::RepeatedField<super::executor::Executor> {
        &self.executors
    }

    fn mut_executors_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::executor::Executor> {
        &mut self.executors
    }

    // optional int64 time_zone_offset = 3;

    pub fn clear_time_zone_offset(&mut self) {
        self.time_zone_offset = ::std::option::Option::None;
    }

    pub fn has_time_zone_offset(&self) -> bool {
        self.time_zone_offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_zone_offset(&mut self, v: i64) {
        self.time_zone_offset = ::std::option::Option::Some(v);
    }

    pub fn get_time_zone_offset(&self) -> i64 {
        self.time_zone_offset.unwrap_or(0)
    }

    fn get_time_zone_offset_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.time_zone_offset
    }

    fn mut_time_zone_offset_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.time_zone_offset
    }

    // optional uint64 flags = 4;

    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None;
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flags(&mut self, v: u64) {
        self.flags = ::std::option::Option::Some(v);
    }

    pub fn get_flags(&self) -> u64 {
        self.flags.unwrap_or(0)
    }

    fn get_flags_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.flags
    }

    fn mut_flags_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.flags
    }

    // repeated uint32 output_offsets = 5;

    pub fn clear_output_offsets(&mut self) {
        self.output_offsets.clear();
    }

    // Param is passed by value, moved
    pub fn set_output_offsets(&mut self, v: ::std::vec::Vec<u32>) {
        self.output_offsets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_output_offsets(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.output_offsets
    }

    // Take field
    pub fn take_output_offsets(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.output_offsets, ::std::vec::Vec::new())
    }

    pub fn get_output_offsets(&self) -> &[u32] {
        &self.output_offsets
    }

    fn get_output_offsets_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.output_offsets
    }

    fn mut_output_offsets_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.output_offsets
    }

    // optional bool collect_range_counts = 6;

    pub fn clear_collect_range_counts(&mut self) {
        self.collect_range_counts = ::std::option::Option::None;
    }

    pub fn has_collect_range_counts(&self) -> bool {
        self.collect_range_counts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collect_range_counts(&mut self, v: bool) {
        self.collect_range_counts = ::std::option::Option::Some(v);
    }

    pub fn get_collect_range_counts(&self) -> bool {
        self.collect_range_counts.unwrap_or(false)
    }

    fn get_collect_range_counts_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.collect_range_counts
    }

    fn mut_collect_range_counts_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.collect_range_counts
    }

    // optional uint64 max_warning_count = 7;

    pub fn clear_max_warning_count(&mut self) {
        self.max_warning_count = ::std::option::Option::None;
    }

    pub fn has_max_warning_count(&self) -> bool {
        self.max_warning_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_warning_count(&mut self, v: u64) {
        self.max_warning_count = ::std::option::Option::Some(v);
    }

    pub fn get_max_warning_count(&self) -> u64 {
        self.max_warning_count.unwrap_or(0)
    }

    fn get_max_warning_count_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.max_warning_count
    }

    fn mut_max_warning_count_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.max_warning_count
    }
}

impl ::protobuf::Message for DAGRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.executors {
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
                    let tmp = is.read_uint64()?;
                    self.start_ts = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.executors)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.time_zone_offset = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.output_offsets)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.collect_range_counts = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.max_warning_count = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.start_ts {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.executors {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.time_zone_offset {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.output_offsets {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.collect_range_counts {
            my_size += 2;
        }
        if let Some(v) = self.max_warning_count {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_ts {
            os.write_uint64(1, v)?;
        }
        for v in &self.executors {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.time_zone_offset {
            os.write_int64(3, v)?;
        }
        if let Some(v) = self.flags {
            os.write_uint64(4, v)?;
        }
        for v in &self.output_offsets {
            os.write_uint32(5, *v)?;
        };
        if let Some(v) = self.collect_range_counts {
            os.write_bool(6, v)?;
        }
        if let Some(v) = self.max_warning_count {
            os.write_uint64(7, v)?;
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

impl ::protobuf::MessageStatic for DAGRequest {
    fn new() -> DAGRequest {
        DAGRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DAGRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_ts",
                    DAGRequest::get_start_ts_for_reflect,
                    DAGRequest::mut_start_ts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::executor::Executor>>(
                    "executors",
                    DAGRequest::get_executors_for_reflect,
                    DAGRequest::mut_executors_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "time_zone_offset",
                    DAGRequest::get_time_zone_offset_for_reflect,
                    DAGRequest::mut_time_zone_offset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "flags",
                    DAGRequest::get_flags_for_reflect,
                    DAGRequest::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "output_offsets",
                    DAGRequest::get_output_offsets_for_reflect,
                    DAGRequest::mut_output_offsets_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "collect_range_counts",
                    DAGRequest::get_collect_range_counts_for_reflect,
                    DAGRequest::mut_collect_range_counts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "max_warning_count",
                    DAGRequest::get_max_warning_count_for_reflect,
                    DAGRequest::mut_max_warning_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DAGRequest>(
                    "DAGRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DAGRequest {
    fn clear(&mut self) {
        self.clear_start_ts();
        self.clear_executors();
        self.clear_time_zone_offset();
        self.clear_flags();
        self.clear_output_offsets();
        self.clear_collect_range_counts();
        self.clear_max_warning_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DAGRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DAGRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StreamResponse {
    // message fields
    error: ::protobuf::SingularPtrField<Error>,
    encode_type: ::std::option::Option<EncodeType>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    warnings: ::protobuf::RepeatedField<Error>,
    output_counts: ::std::vec::Vec<i64>,
    warning_count: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StreamResponse {}

impl StreamResponse {
    pub fn new() -> StreamResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StreamResponse {
        static mut instance: ::protobuf::lazy::Lazy<StreamResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StreamResponse,
        };
        unsafe {
            instance.get(StreamResponse::new)
        }
    }

    // optional .tipb.Error error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: Error) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut Error {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> Error {
        self.error.take().unwrap_or_else(|| Error::new())
    }

    pub fn get_error(&self) -> &Error {
        self.error.as_ref().unwrap_or_else(|| Error::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<Error> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Error> {
        &mut self.error
    }

    // optional .tipb.EncodeType encode_type = 2;

    pub fn clear_encode_type(&mut self) {
        self.encode_type = ::std::option::Option::None;
    }

    pub fn has_encode_type(&self) -> bool {
        self.encode_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encode_type(&mut self, v: EncodeType) {
        self.encode_type = ::std::option::Option::Some(v);
    }

    pub fn get_encode_type(&self) -> EncodeType {
        self.encode_type.unwrap_or(EncodeType::TypeDefault)
    }

    fn get_encode_type_for_reflect(&self) -> &::std::option::Option<EncodeType> {
        &self.encode_type
    }

    fn mut_encode_type_for_reflect(&mut self) -> &mut ::std::option::Option<EncodeType> {
        &mut self.encode_type
    }

    // optional bytes data = 3;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }

    // repeated .tipb.Error warnings = 4;

    pub fn clear_warnings(&mut self) {
        self.warnings.clear();
    }

    // Param is passed by value, moved
    pub fn set_warnings(&mut self, v: ::protobuf::RepeatedField<Error>) {
        self.warnings = v;
    }

    // Mutable pointer to the field.
    pub fn mut_warnings(&mut self) -> &mut ::protobuf::RepeatedField<Error> {
        &mut self.warnings
    }

    // Take field
    pub fn take_warnings(&mut self) -> ::protobuf::RepeatedField<Error> {
        ::std::mem::replace(&mut self.warnings, ::protobuf::RepeatedField::new())
    }

    pub fn get_warnings(&self) -> &[Error] {
        &self.warnings
    }

    fn get_warnings_for_reflect(&self) -> &::protobuf::RepeatedField<Error> {
        &self.warnings
    }

    fn mut_warnings_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Error> {
        &mut self.warnings
    }

    // repeated int64 output_counts = 5;

    pub fn clear_output_counts(&mut self) {
        self.output_counts.clear();
    }

    // Param is passed by value, moved
    pub fn set_output_counts(&mut self, v: ::std::vec::Vec<i64>) {
        self.output_counts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_output_counts(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.output_counts
    }

    // Take field
    pub fn take_output_counts(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.output_counts, ::std::vec::Vec::new())
    }

    pub fn get_output_counts(&self) -> &[i64] {
        &self.output_counts
    }

    fn get_output_counts_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.output_counts
    }

    fn mut_output_counts_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.output_counts
    }

    // optional int64 warning_count = 6;

    pub fn clear_warning_count(&mut self) {
        self.warning_count = ::std::option::Option::None;
    }

    pub fn has_warning_count(&self) -> bool {
        self.warning_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_warning_count(&mut self, v: i64) {
        self.warning_count = ::std::option::Option::Some(v);
    }

    pub fn get_warning_count(&self) -> i64 {
        self.warning_count.unwrap_or(0)
    }

    fn get_warning_count_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.warning_count
    }

    fn mut_warning_count_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.warning_count
    }
}

impl ::protobuf::Message for StreamResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.error {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.warnings {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.encode_type, 2, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.warnings)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.output_counts)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.warning_count = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.encode_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        for value in &self.warnings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.output_counts {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.warning_count {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.encode_type {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.data.as_ref() {
            os.write_bytes(3, &v)?;
        }
        for v in &self.warnings {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.output_counts {
            os.write_int64(5, *v)?;
        };
        if let Some(v) = self.warning_count {
            os.write_int64(6, v)?;
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

impl ::protobuf::MessageStatic for StreamResponse {
    fn new() -> StreamResponse {
        StreamResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<StreamResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Error>>(
                    "error",
                    StreamResponse::get_error_for_reflect,
                    StreamResponse::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EncodeType>>(
                    "encode_type",
                    StreamResponse::get_encode_type_for_reflect,
                    StreamResponse::mut_encode_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    StreamResponse::get_data_for_reflect,
                    StreamResponse::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Error>>(
                    "warnings",
                    StreamResponse::get_warnings_for_reflect,
                    StreamResponse::mut_warnings_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "output_counts",
                    StreamResponse::get_output_counts_for_reflect,
                    StreamResponse::mut_output_counts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "warning_count",
                    StreamResponse::get_warning_count_for_reflect,
                    StreamResponse::mut_warning_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StreamResponse>(
                    "StreamResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StreamResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_encode_type();
        self.clear_data();
        self.clear_warnings();
        self.clear_output_counts();
        self.clear_warning_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StreamResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StreamResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EncodeType {
    TypeDefault = 0,
    TypeArrow = 1,
}

impl ::protobuf::ProtobufEnum for EncodeType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EncodeType> {
        match value {
            0 => ::std::option::Option::Some(EncodeType::TypeDefault),
            1 => ::std::option::Option::Some(EncodeType::TypeArrow),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EncodeType] = &[
            EncodeType::TypeDefault,
            EncodeType::TypeArrow,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EncodeType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EncodeType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EncodeType {
}

impl ::protobuf::reflect::ProtobufValue for EncodeType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cselect.proto\x12\x04tipb\x1a\x0eexecutor.proto\x1a\x14gogoproto/go\
    go.proto\"1\n\x03Row\x12\x16\n\x06handle\x18\x01\x20\x01(\x0cR\x06handle\
    \x12\x12\n\x04data\x18\x02\x20\x01(\x0cR\x04data\"9\n\x05Error\x12\x18\n\
    \x04code\x18\x01\x20\x01(\x05R\x04codeB\x04\xc8\xde\x1f\0\x12\x16\n\x03m\
    sg\x18\x02\x20\x01(\tR\x03msgB\x04\xc8\xde\x1f\0\"\xf0\x01\n\x0eSelectRe\
    sponse\x12!\n\x05error\x18\x01\x20\x01(\x0b2\x0b.tipb.ErrorR\x05error\
    \x12\x1d\n\x04rows\x18\x02\x20\x03(\x0b2\t.tipb.RowR\x04rows\x12)\n\x06c\
    hunks\x18\x03\x20\x03(\x0b2\x0b.tipb.ChunkR\x06chunksB\x04\xc8\xde\x1f\0\
    \x12'\n\x08warnings\x18\x04\x20\x03(\x0b2\x0b.tipb.ErrorR\x08warnings\
    \x12#\n\routput_counts\x18\x05\x20\x03(\x03R\x0coutputCounts\x12#\n\rwar\
    ning_count\x18\x06\x20\x01(\x03R\x0cwarningCount\"\x8f\x01\n\x05Chunk\
    \x12T\n\trows_data\x18\x03\x20\x01(\x0cR\x08rowsDataB7\xc8\xde\x1f\0\xda\
    \xde\x1f/github.com/pingcap/tipb/sharedbytes.SharedBytes\x120\n\trows_me\
    ta\x18\x04\x20\x03(\x0b2\r.tipb.RowMetaR\x08rowsMetaB\x04\xc8\xde\x1f\0\
    \"E\n\x07RowMeta\x12\x1c\n\x06handle\x18\x01\x20\x01(\x03R\x06handleB\
    \x04\xc8\xde\x1f\0\x12\x1c\n\x06length\x18\x02\x20\x01(\x03R\x06lengthB\
    \x04\xc8\xde\x1f\0\"\xac\x02\n\nDAGRequest\x12\x1f\n\x08start_ts\x18\x01\
    \x20\x01(\x04R\x07startTsB\x04\xc8\xde\x1f\0\x12,\n\texecutors\x18\x02\
    \x20\x03(\x0b2\x0e.tipb.ExecutorR\texecutors\x12.\n\x10time_zone_offset\
    \x18\x03\x20\x01(\x03R\x0etimeZoneOffsetB\x04\xc8\xde\x1f\0\x12\x1a\n\
    \x05flags\x18\x04\x20\x01(\x04R\x05flagsB\x04\xc8\xde\x1f\0\x12%\n\x0eou\
    tput_offsets\x18\x05\x20\x03(\rR\routputOffsets\x120\n\x14collect_range_\
    counts\x18\x06\x20\x01(\x08R\x12collectRangeCounts\x12*\n\x11max_warning\
    _count\x18\x07\x20\x01(\x04R\x0fmaxWarningCount\"\xac\x02\n\x0eStreamRes\
    ponse\x12!\n\x05error\x18\x01\x20\x01(\x0b2\x0b.tipb.ErrorR\x05error\x12\
    7\n\x0bencode_type\x18\x02\x20\x01(\x0e2\x10.tipb.EncodeTypeR\nencodeTyp\
    eB\x04\xc8\xde\x1f\0\x12K\n\x04data\x18\x03\x20\x01(\x0cR\x04dataB7\xda\
    \xde\x1f/github.com/pingcap/tipb/sharedbytes.SharedBytes\xc8\xde\x1f\0\
    \x12'\n\x08warnings\x18\x04\x20\x03(\x0b2\x0b.tipb.ErrorR\x08warnings\
    \x12#\n\routput_counts\x18\x05\x20\x03(\x03R\x0coutputCounts\x12#\n\rwar\
    ning_count\x18\x06\x20\x01(\x03R\x0cwarningCount*,\n\nEncodeType\x12\x0f\
    \n\x0bTypeDefault\x10\0\x12\r\n\tTypeArrow\x10\x01B%\n\x15com.pingcap.ti\
    db.tipbP\x01\xe0\xe2\x1e\x01\xc8\xe2\x1e\x01\xd0\xe2\x1e\x01J\xb4(\n\x06\
    \x12\x04\0\0h\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\
    \x03\x02\x08\x0c\n\x08\n\x01\x08\x12\x03\x04\0\"\n\x0b\n\x04\x08\xe7\x07\
    \0\x12\x03\x04\0\"\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x04\x07\x1a\n\r\
    \n\x06\x08\xe7\x07\0\x02\0\x12\x03\x04\x07\x1a\n\x0e\n\x07\x08\xe7\x07\0\
    \x02\0\x01\x12\x03\x04\x07\x1a\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x04\
    \x1d!\n\x08\n\x01\x08\x12\x03\x05\0.\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\
    \x05\0.\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x05\x07\x13\n\r\n\x06\
    \x08\xe7\x07\x01\x02\0\x12\x03\x05\x07\x13\n\x0e\n\x07\x08\xe7\x07\x01\
    \x02\0\x01\x12\x03\x05\x07\x13\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\
    \x05\x16-\n\t\n\x02\x03\0\x12\x03\x07\x07\x17\n\t\n\x02\x03\x01\x12\x03\
    \x08\x07\x1d\n\x08\n\x01\x08\x12\x03\n\0(\n\x0b\n\x04\x08\xe7\x07\x02\
    \x12\x03\n\0(\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\n\x07\x20\n\r\n\
    \x06\x08\xe7\x07\x02\x02\0\x12\x03\n\x07\x20\n\x0e\n\x07\x08\xe7\x07\x02\
    \x02\0\x01\x12\x03\n\x08\x1f\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\n#'\
    \n\x08\n\x01\x08\x12\x03\x0b\0$\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x0b\
    \0$\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x0b\x07\x1c\n\r\n\x06\x08\
    \xe7\x07\x03\x02\0\x12\x03\x0b\x07\x1c\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\
    \x01\x12\x03\x0b\x08\x1b\n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\x03\x0b\x1f\
    #\n\x08\n\x01\x08\x12\x03\x0c\0*\n\x0b\n\x04\x08\xe7\x07\x04\x12\x03\x0c\
    \0*\n\x0c\n\x05\x08\xe7\x07\x04\x02\x12\x03\x0c\x07\"\n\r\n\x06\x08\xe7\
    \x07\x04\x02\0\x12\x03\x0c\x07\"\n\x0e\n\x07\x08\xe7\x07\x04\x02\0\x01\
    \x12\x03\x0c\x08!\n\x0c\n\x05\x08\xe7\x07\x04\x03\x12\x03\x0c%)\n,\n\x02\
    \x04\0\x12\x04\x0f\0\x12\x01\x1a\x20\x20values\x20are\x20all\x20in\x20te\
    xt\x20format.\n\n\n\n\x03\x04\0\x01\x12\x03\x0f\x08\x0b\n\x0b\n\x04\x04\
    \0\x02\0\x12\x03\x10\x08\"\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x10\x08\
    \x10\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x10\x11\x16\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\x10\x17\x1d\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x10\
    \x20!\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x11\x08\x20\n\x0c\n\x05\x04\0\
    \x02\x01\x04\x12\x03\x11\x08\x10\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\
    \x11\x11\x16\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x11\x17\x1b\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\x11\x1e\x1f\n\n\n\x02\x04\x01\x12\x04\x14\
    \0\x18\x01\n\n\n\x03\x04\x01\x01\x12\x03\x14\x08\r\n\x0b\n\x04\x04\x01\
    \x02\0\x12\x03\x15\x08?\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x15\x08\
    \x10\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x15\x11\x16\n\x0c\n\x05\x04\
    \x01\x02\0\x01\x12\x03\x15\x17\x1b\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\
    \x15\x1e\x1f\n\x0c\n\x05\x04\x01\x02\0\x08\x12\x03\x15\x20>\n\x0f\n\x08\
    \x04\x01\x02\0\x08\xe7\x07\0\x12\x03\x15!=\n\x10\n\t\x04\x01\x02\0\x08\
    \xe7\x07\0\x02\x12\x03\x15!5\n\x11\n\n\x04\x01\x02\0\x08\xe7\x07\0\x02\0\
    \x12\x03\x15!5\n\x12\n\x0b\x04\x01\x02\0\x08\xe7\x07\0\x02\0\x01\x12\x03\
    \x15\"4\n\x10\n\t\x04\x01\x02\0\x08\xe7\x07\0\x03\x12\x03\x158=\n\x0b\n\
    \x04\x04\x01\x02\x01\x12\x03\x17\x08?\n\x0c\n\x05\x04\x01\x02\x01\x04\
    \x12\x03\x17\x08\x10\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x17\x11\x17\
    \n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x17\x18\x1b\n\x0c\n\x05\x04\x01\
    \x02\x01\x03\x12\x03\x17\x1e\x1f\n\x0c\n\x05\x04\x01\x02\x01\x08\x12\x03\
    \x17\x20>\n\x0f\n\x08\x04\x01\x02\x01\x08\xe7\x07\0\x12\x03\x17!=\n\x10\
    \n\t\x04\x01\x02\x01\x08\xe7\x07\0\x02\x12\x03\x17!5\n\x11\n\n\x04\x01\
    \x02\x01\x08\xe7\x07\0\x02\0\x12\x03\x17!5\n\x12\n\x0b\x04\x01\x02\x01\
    \x08\xe7\x07\0\x02\0\x01\x12\x03\x17\"4\n\x10\n\t\x04\x01\x02\x01\x08\
    \xe7\x07\0\x03\x12\x03\x178=\n)\n\x02\x04\x02\x12\x04\x1b\0*\x01\x1a\x1d\
    \x20Response\x20for\x20SelectRequest.\n\n\n\n\x03\x04\x02\x01\x12\x03\
    \x1b\x08\x16\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x1c\x08!\n\x0c\n\x05\x04\
    \x02\x02\0\x04\x12\x03\x1c\x08\x10\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03\
    \x1c\x11\x16\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x1c\x17\x1c\n\x0c\n\
    \x05\x04\x02\x02\0\x03\x12\x03\x1c\x1f\x20\n\x1b\n\x04\x04\x02\x02\x01\
    \x12\x03\x1f\x08\x1e\x1a\x0e\x20Result\x20rows.\n\n\x0c\n\x05\x04\x02\
    \x02\x01\x04\x12\x03\x1f\x08\x10\n\x0c\n\x05\x04\x02\x02\x01\x06\x12\x03\
    \x1f\x11\x14\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x1f\x15\x19\n\x0c\n\
    \x05\x04\x02\x02\x01\x03\x12\x03\x1f\x1c\x1d\nm\n\x04\x04\x02\x02\x02\
    \x12\x03#\x08A\x1a`\x20Use\x20multiple\x20chunks\x20to\x20reduce\x20memo\
    ry\x20allocation\x20and\n\x20avoid\x20allocating\x20large\x20contiguous\
    \x20memory.\n\n\x0c\n\x05\x04\x02\x02\x02\x04\x12\x03#\x08\x10\n\x0c\n\
    \x05\x04\x02\x02\x02\x06\x12\x03#\x11\x16\n\x0c\n\x05\x04\x02\x02\x02\
    \x01\x12\x03#\x17\x1d\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03#\x20!\n\
    \x0c\n\x05\x04\x02\x02\x02\x08\x12\x03#\"@\n\x0f\n\x08\x04\x02\x02\x02\
    \x08\xe7\x07\0\x12\x03##?\n\x10\n\t\x04\x02\x02\x02\x08\xe7\x07\0\x02\
    \x12\x03##7\n\x11\n\n\x04\x02\x02\x02\x08\xe7\x07\0\x02\0\x12\x03##7\n\
    \x12\n\x0b\x04\x02\x02\x02\x08\xe7\x07\0\x02\0\x01\x12\x03#$6\n\x10\n\t\
    \x04\x02\x02\x02\x08\xe7\x07\0\x03\x12\x03#:?\n\x0b\n\x04\x04\x02\x02\
    \x03\x12\x03%\x08$\n\x0c\n\x05\x04\x02\x02\x03\x04\x12\x03%\x08\x10\n\
    \x0c\n\x05\x04\x02\x02\x03\x06\x12\x03%\x11\x16\n\x0c\n\x05\x04\x02\x02\
    \x03\x01\x12\x03%\x17\x1f\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\x03%\"#\n\
    \x0b\n\x04\x04\x02\x02\x04\x12\x03'\x08)\n\x0c\n\x05\x04\x02\x02\x04\x04\
    \x12\x03'\x08\x10\n\x0c\n\x05\x04\x02\x02\x04\x05\x12\x03'\x11\x16\n\x0c\
    \n\x05\x04\x02\x02\x04\x01\x12\x03'\x17$\n\x0c\n\x05\x04\x02\x02\x04\x03\
    \x12\x03''(\n\x0b\n\x04\x04\x02\x02\x05\x12\x03)\x08)\n\x0c\n\x05\x04\
    \x02\x02\x05\x04\x12\x03)\x08\x10\n\x0c\n\x05\x04\x02\x02\x05\x05\x12\
    \x03)\x11\x16\n\x0c\n\x05\x04\x02\x02\x05\x01\x12\x03)\x17$\n\x0c\n\x05\
    \x04\x02\x02\x05\x03\x12\x03)'(\n>\n\x02\x04\x03\x12\x04-\03\x01\x1a2\
    \x20Chunk\x20contains\x20multiple\x20rows\x20data\x20and\x20rows\x20meta\
    .\n\n\n\n\x03\x04\x03\x01\x12\x03-\x08\r\n/\n\x04\x04\x03\x02\0\x12\x04/\
    \x08\x90\x01\x1a!\x20Data\x20for\x20all\x20rows\x20in\x20the\x20chunk.\n\
    \n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03/\x08\x10\n\x0c\n\x05\x04\x03\x02\
    \0\x05\x12\x03/\x11\x16\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03/\x17\x20\n\
    \x0c\n\x05\x04\x03\x02\0\x03\x12\x03/#$\n\r\n\x05\x04\x03\x02\0\x08\x12\
    \x04/%\x8f\x01\n\x0f\n\x08\x04\x03\x02\0\x08\xe7\x07\0\x12\x03/&p\n\x10\
    \n\t\x04\x03\x02\0\x08\xe7\x07\0\x02\x12\x03/&<\n\x11\n\n\x04\x03\x02\0\
    \x08\xe7\x07\0\x02\0\x12\x03/&<\n\x12\n\x0b\x04\x03\x02\0\x08\xe7\x07\0\
    \x02\0\x01\x12\x03/';\n\x10\n\t\x04\x03\x02\0\x08\xe7\x07\0\x07\x12\x03/\
    ?p\n\x10\n\x08\x04\x03\x02\0\x08\xe7\x07\x01\x12\x04/r\x8e\x01\n\x11\n\t\
    \x04\x03\x02\0\x08\xe7\x07\x01\x02\x12\x04/r\x86\x01\n\x12\n\n\x04\x03\
    \x02\0\x08\xe7\x07\x01\x02\0\x12\x04/r\x86\x01\n\x13\n\x0b\x04\x03\x02\0\
    \x08\xe7\x07\x01\x02\0\x01\x12\x04/s\x85\x01\n\x12\n\t\x04\x03\x02\0\x08\
    \xe7\x07\x01\x03\x12\x05/\x89\x01\x8e\x01\n'\n\x04\x04\x03\x02\x01\x12\
    \x032\x08F\x1a\x1a\x20Meta\x20data\x20for\x20every\x20row.\n\n\x0c\n\x05\
    \x04\x03\x02\x01\x04\x12\x032\x08\x10\n\x0c\n\x05\x04\x03\x02\x01\x06\
    \x12\x032\x11\x18\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x032\x19\"\n\x0c\n\
    \x05\x04\x03\x02\x01\x03\x12\x032%&\n\x0c\n\x05\x04\x03\x02\x01\x08\x12\
    \x032'E\n\x0f\n\x08\x04\x03\x02\x01\x08\xe7\x07\0\x12\x032(D\n\x10\n\t\
    \x04\x03\x02\x01\x08\xe7\x07\0\x02\x12\x032(<\n\x11\n\n\x04\x03\x02\x01\
    \x08\xe7\x07\0\x02\0\x12\x032(<\n\x12\n\x0b\x04\x03\x02\x01\x08\xe7\x07\
    \0\x02\0\x01\x12\x032);\n\x10\n\t\x04\x03\x02\x01\x08\xe7\x07\0\x03\x12\
    \x032?D\n>\n\x02\x04\x04\x12\x046\09\x01\x1a2\x20RowMeta\x20contains\x20\
    row\x20handle\x20and\x20length\x20of\x20a\x20row.\n\n\n\n\x03\x04\x04\
    \x01\x12\x036\x08\x0f\n\x0b\n\x04\x04\x04\x02\0\x12\x037\x08A\n\x0c\n\
    \x05\x04\x04\x02\0\x04\x12\x037\x08\x10\n\x0c\n\x05\x04\x04\x02\0\x05\
    \x12\x037\x11\x16\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x037\x17\x1d\n\x0c\n\
    \x05\x04\x04\x02\0\x03\x12\x037\x20!\n\x0c\n\x05\x04\x04\x02\0\x08\x12\
    \x037\"@\n\x0f\n\x08\x04\x04\x02\0\x08\xe7\x07\0\x12\x037#?\n\x10\n\t\
    \x04\x04\x02\0\x08\xe7\x07\0\x02\x12\x037#7\n\x11\n\n\x04\x04\x02\0\x08\
    \xe7\x07\0\x02\0\x12\x037#7\n\x12\n\x0b\x04\x04\x02\0\x08\xe7\x07\0\x02\
    \0\x01\x12\x037$6\n\x10\n\t\x04\x04\x02\0\x08\xe7\x07\0\x03\x12\x037:?\n\
    \x0b\n\x04\x04\x04\x02\x01\x12\x038\x08A\n\x0c\n\x05\x04\x04\x02\x01\x04\
    \x12\x038\x08\x10\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x038\x11\x16\n\x0c\
    \n\x05\x04\x04\x02\x01\x01\x12\x038\x17\x1d\n\x0c\n\x05\x04\x04\x02\x01\
    \x03\x12\x038\x20!\n\x0c\n\x05\x04\x04\x02\x01\x08\x12\x038\"@\n\x0f\n\
    \x08\x04\x04\x02\x01\x08\xe7\x07\0\x12\x038#?\n\x10\n\t\x04\x04\x02\x01\
    \x08\xe7\x07\0\x02\x12\x038#7\n\x11\n\n\x04\x04\x02\x01\x08\xe7\x07\0\
    \x02\0\x12\x038#7\n\x12\n\x0b\x04\x04\x02\x01\x08\xe7\x07\0\x02\0\x01\
    \x12\x038$6\n\x10\n\t\x04\x04\x02\x01\x08\xe7\x07\0\x03\x12\x038:?\nS\n\
    \x02\x04\x05\x12\x04<\0X\x01\x1aG\x20DAGRequest\x20represents\x20the\x20\
    request\x20that\x20will\x20be\x20handled\x20with\x20DAG\x20mode.\n\n\n\n\
    \x03\x04\x05\x01\x12\x03<\x08\x12\n+\n\x04\x04\x05\x02\0\x12\x03>\x08D\
    \x1a\x1e\x20Transaction\x20start\x20timestamp.\n\n\x0c\n\x05\x04\x05\x02\
    \0\x04\x12\x03>\x08\x10\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03>\x11\x17\n\
    \x0c\n\x05\x04\x05\x02\0\x01\x12\x03>\x18\x20\n\x0c\n\x05\x04\x05\x02\0\
    \x03\x12\x03>#$\n\x0c\n\x05\x04\x05\x02\0\x08\x12\x03>%C\n\x0f\n\x08\x04\
    \x05\x02\0\x08\xe7\x07\0\x12\x03>&B\n\x10\n\t\x04\x05\x02\0\x08\xe7\x07\
    \0\x02\x12\x03>&:\n\x11\n\n\x04\x05\x02\0\x08\xe7\x07\0\x02\0\x12\x03>&:\
    \n\x12\n\x0b\x04\x05\x02\0\x08\xe7\x07\0\x02\0\x01\x12\x03>'9\n\x10\n\t\
    \x04\x05\x02\0\x08\xe7\x07\0\x03\x12\x03>=B\n1\n\x04\x04\x05\x02\x01\x12\
    \x03A\x08(\x1a$\x20It\x20represents\x20push\x20down\x20Executors.\n\n\
    \x0c\n\x05\x04\x05\x02\x01\x04\x12\x03A\x08\x10\n\x0c\n\x05\x04\x05\x02\
    \x01\x06\x12\x03A\x11\x19\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03A\x1a#\
    \n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x03A&'\n*\n\x04\x04\x05\x02\x02\x12\
    \x03D\x08K\x1a\x1d\x20time\x20zone\x20offset\x20in\x20seconds\n\n\x0c\n\
    \x05\x04\x05\x02\x02\x04\x12\x03D\x08\x10\n\x0c\n\x05\x04\x05\x02\x02\
    \x05\x12\x03D\x11\x16\n\x0c\n\x05\x04\x05\x02\x02\x01\x12\x03D\x17'\n\
    \x0c\n\x05\x04\x05\x02\x02\x03\x12\x03D*+\n\x0c\n\x05\x04\x05\x02\x02\
    \x08\x12\x03D,J\n\x0f\n\x08\x04\x05\x02\x02\x08\xe7\x07\0\x12\x03D-I\n\
    \x10\n\t\x04\x05\x02\x02\x08\xe7\x07\0\x02\x12\x03D-A\n\x11\n\n\x04\x05\
    \x02\x02\x08\xe7\x07\0\x02\0\x12\x03D-A\n\x12\n\x0b\x04\x05\x02\x02\x08\
    \xe7\x07\0\x02\0\x01\x12\x03D.@\n\x10\n\t\x04\x05\x02\x02\x08\xe7\x07\0\
    \x03\x12\x03DDI\n\xac\x02\n\x04\x04\x05\x02\x03\x12\x03M\x08A\x1a\x9e\
    \x02\x20flags\x20are\x20used\x20to\x20store\x20flags\x20that\x20change\
    \x20the\x20execution\x20mode,\x20it\x20contains:\n\tignore_truncate\x20=\
    \x201\n\t\ttruncate\x20error\x20should\x20be\x20ignore\x20if\x20set.\n\t\
    truncate_as_warning\x20=\x201\x20<<\x201\n\t\twhen\x20ignored_truncate\
    \x20is\x20not\x20set,\x20return\x20warning\x20instead\x20of\x20error\x20\
    if\x20this\x20flag\x20is\x20set.\n\t...\n\tadd\x20more\x20when\x20needed\
    .\n\n\x0c\n\x05\x04\x05\x02\x03\x04\x12\x03M\x08\x10\n\x0c\n\x05\x04\x05\
    \x02\x03\x05\x12\x03M\x11\x17\n\x0c\n\x05\x04\x05\x02\x03\x01\x12\x03M\
    \x18\x1d\n\x0c\n\x05\x04\x05\x02\x03\x03\x12\x03M\x20!\n\x0c\n\x05\x04\
    \x05\x02\x03\x08\x12\x03M\"@\n\x0f\n\x08\x04\x05\x02\x03\x08\xe7\x07\0\
    \x12\x03M#?\n\x10\n\t\x04\x05\x02\x03\x08\xe7\x07\0\x02\x12\x03M#7\n\x11\
    \n\n\x04\x05\x02\x03\x08\xe7\x07\0\x02\0\x12\x03M#7\n\x12\n\x0b\x04\x05\
    \x02\x03\x08\xe7\x07\0\x02\0\x01\x12\x03M$6\n\x10\n\t\x04\x05\x02\x03\
    \x08\xe7\x07\0\x03\x12\x03M:?\n<\n\x04\x04\x05\x02\x04\x12\x03P\x08+\x1a\
    /\x20It\x20represents\x20which\x20columns\x20we\x20should\x20output.\n\n\
    \x0c\n\x05\x04\x05\x02\x04\x04\x12\x03P\x08\x10\n\x0c\n\x05\x04\x05\x02\
    \x04\x05\x12\x03P\x11\x17\n\x0c\n\x05\x04\x05\x02\x04\x01\x12\x03P\x18&\
    \n\x0c\n\x05\x04\x05\x02\x04\x03\x12\x03P)*\nW\n\x04\x04\x05\x02\x05\x12\
    \x03S\x08/\x1aJ\x20It\x20represents\x20whether\x20we\x20collect\x20the\
    \x20detailed\x20scan\x20counts\x20in\x20each\x20range.\n\n\x0c\n\x05\x04\
    \x05\x02\x05\x04\x12\x03S\x08\x10\n\x0c\n\x05\x04\x05\x02\x05\x05\x12\
    \x03S\x11\x15\n\x0c\n\x05\x04\x05\x02\x05\x01\x12\x03S\x16*\n\x0c\n\x05\
    \x04\x05\x02\x05\x03\x12\x03S-.\nx\n\x04\x04\x05\x02\x06\x12\x03W\x08.\
    \x1ak\x20It\x20indicates\x20the\x20maximum\x20number\x20of\x20warning,\n\
    \x20which\x20is\x20the\x20number\x20of\x20messages\x20that\x20SHOW\x20WA\
    RNINGS\x20displays.\n\n\x0c\n\x05\x04\x05\x02\x06\x04\x12\x03W\x08\x10\n\
    \x0c\n\x05\x04\x05\x02\x06\x05\x12\x03W\x11\x17\n\x0c\n\x05\x04\x05\x02\
    \x06\x01\x12\x03W\x18)\n\x0c\n\x05\x04\x05\x02\x06\x03\x12\x03W,-\n\n\n\
    \x02\x05\0\x12\x04Z\0]\x01\n\n\n\x03\x05\0\x01\x12\x03Z\x05\x0f\n\x0b\n\
    \x04\x05\0\x02\0\x12\x03[\x08\x18\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03[\
    \x08\x13\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03[\x16\x17\n\x0b\n\x04\x05\0\
    \x02\x01\x12\x03\\\x08\x16\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\\\x08\
    \x11\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\\\x14\x15\n\n\n\x02\x04\x06\
    \x12\x04_\0h\x01\n\n\n\x03\x04\x06\x01\x12\x03_\x08\x16\n\x0b\n\x04\x04\
    \x06\x02\0\x12\x03`\x08!\n\x0c\n\x05\x04\x06\x02\0\x04\x12\x03`\x08\x10\
    \n\x0c\n\x05\x04\x06\x02\0\x06\x12\x03`\x11\x16\n\x0c\n\x05\x04\x06\x02\
    \0\x01\x12\x03`\x17\x1c\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03`\x1f\x20\n\
    \x0b\n\x04\x04\x06\x02\x01\x12\x03a\x08K\n\x0c\n\x05\x04\x06\x02\x01\x04\
    \x12\x03a\x08\x10\n\x0c\n\x05\x04\x06\x02\x01\x06\x12\x03a\x11\x1b\n\x0c\
    \n\x05\x04\x06\x02\x01\x01\x12\x03a\x1c'\n\x0c\n\x05\x04\x06\x02\x01\x03\
    \x12\x03a*+\n\x0c\n\x05\x04\x06\x02\x01\x08\x12\x03a,J\n\x0f\n\x08\x04\
    \x06\x02\x01\x08\xe7\x07\0\x12\x03a-I\n\x10\n\t\x04\x06\x02\x01\x08\xe7\
    \x07\0\x02\x12\x03a-A\n\x11\n\n\x04\x06\x02\x01\x08\xe7\x07\0\x02\0\x12\
    \x03a-A\n\x12\n\x0b\x04\x06\x02\x01\x08\xe7\x07\0\x02\0\x01\x12\x03a.@\n\
    \x10\n\t\x04\x06\x02\x01\x08\xe7\x07\0\x03\x12\x03aDI\n!\n\x04\x04\x06\
    \x02\x02\x12\x04c\x08\x8b\x01\x1a\x13\x20Data\x20for\x20all\x20rows\n\n\
    \x0c\n\x05\x04\x06\x02\x02\x04\x12\x03c\x08\x10\n\x0c\n\x05\x04\x06\x02\
    \x02\x05\x12\x03c\x11\x16\n\x0c\n\x05\x04\x06\x02\x02\x01\x12\x03c\x17\
    \x1b\n\x0c\n\x05\x04\x06\x02\x02\x03\x12\x03c\x1e\x1f\n\r\n\x05\x04\x06\
    \x02\x02\x08\x12\x04c\x20\x8a\x01\n\x0f\n\x08\x04\x06\x02\x02\x08\xe7\
    \x07\0\x12\x03c!k\n\x10\n\t\x04\x06\x02\x02\x08\xe7\x07\0\x02\x12\x03c!7\
    \n\x11\n\n\x04\x06\x02\x02\x08\xe7\x07\0\x02\0\x12\x03c!7\n\x12\n\x0b\
    \x04\x06\x02\x02\x08\xe7\x07\0\x02\0\x01\x12\x03c\"6\n\x10\n\t\x04\x06\
    \x02\x02\x08\xe7\x07\0\x07\x12\x03c:k\n\x10\n\x08\x04\x06\x02\x02\x08\
    \xe7\x07\x01\x12\x04cm\x89\x01\n\x11\n\t\x04\x06\x02\x02\x08\xe7\x07\x01\
    \x02\x12\x04cm\x81\x01\n\x12\n\n\x04\x06\x02\x02\x08\xe7\x07\x01\x02\0\
    \x12\x04cm\x81\x01\n\x13\n\x0b\x04\x06\x02\x02\x08\xe7\x07\x01\x02\0\x01\
    \x12\x04cn\x80\x01\n\x12\n\t\x04\x06\x02\x02\x08\xe7\x07\x01\x03\x12\x05\
    c\x84\x01\x89\x01\n\x0b\n\x04\x04\x06\x02\x03\x12\x03d\x08$\n\x0c\n\x05\
    \x04\x06\x02\x03\x04\x12\x03d\x08\x10\n\x0c\n\x05\x04\x06\x02\x03\x06\
    \x12\x03d\x11\x16\n\x0c\n\x05\x04\x06\x02\x03\x01\x12\x03d\x17\x1f\n\x0c\
    \n\x05\x04\x06\x02\x03\x03\x12\x03d\"#\n1\n\x04\x04\x06\x02\x04\x12\x03f\
    \x08)\x1a$\x20output\x20row\x20count\x20for\x20each\x20executor\n\n\x0c\
    \n\x05\x04\x06\x02\x04\x04\x12\x03f\x08\x10\n\x0c\n\x05\x04\x06\x02\x04\
    \x05\x12\x03f\x11\x16\n\x0c\n\x05\x04\x06\x02\x04\x01\x12\x03f\x17$\n\
    \x0c\n\x05\x04\x06\x02\x04\x03\x12\x03f'(\n\x0b\n\x04\x04\x06\x02\x05\
    \x12\x03g\x08)\n\x0c\n\x05\x04\x06\x02\x05\x04\x12\x03g\x08\x10\n\x0c\n\
    \x05\x04\x06\x02\x05\x05\x12\x03g\x11\x16\n\x0c\n\x05\x04\x06\x02\x05\
    \x01\x12\x03g\x17$\n\x0c\n\x05\x04\x06\x02\x05\x03\x12\x03g'(\
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
