// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct AnalyzeReq {
    // message fields
    tp: ::std::option::Option<AnalyzeType>,
    start_ts: ::std::option::Option<u64>,
    flags: ::std::option::Option<u64>,
    time_zone_offset: ::std::option::Option<i64>,
    idx_req: ::protobuf::SingularPtrField<AnalyzeIndexReq>,
    col_req: ::protobuf::SingularPtrField<AnalyzeColumnsReq>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AnalyzeReq {}

impl AnalyzeReq {
    pub fn new() -> AnalyzeReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AnalyzeReq {
        static mut instance: ::protobuf::lazy::Lazy<AnalyzeReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AnalyzeReq,
        };
        unsafe {
            instance.get(|| {
                AnalyzeReq {
                    tp: ::std::option::Option::None,
                    start_ts: ::std::option::Option::None,
                    flags: ::std::option::Option::None,
                    time_zone_offset: ::std::option::Option::None,
                    idx_req: ::protobuf::SingularPtrField::none(),
                    col_req: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .tipb.AnalyzeType tp = 1;

    pub fn clear_tp(&mut self) {
        self.tp = ::std::option::Option::None;
    }

    pub fn has_tp(&self) -> bool {
        self.tp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tp(&mut self, v: AnalyzeType) {
        self.tp = ::std::option::Option::Some(v);
    }

    pub fn get_tp(&self) -> AnalyzeType {
        self.tp.unwrap_or(AnalyzeType::TypeIndex)
    }

    // optional uint64 start_ts = 2;

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

    // optional uint64 flags = 3;

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

    // optional int64 time_zone_offset = 4;

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

    // optional .tipb.AnalyzeIndexReq idx_req = 5;

    pub fn clear_idx_req(&mut self) {
        self.idx_req.clear();
    }

    pub fn has_idx_req(&self) -> bool {
        self.idx_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_idx_req(&mut self, v: AnalyzeIndexReq) {
        self.idx_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_idx_req(&mut self) -> &mut AnalyzeIndexReq {
        if self.idx_req.is_none() {
            self.idx_req.set_default();
        };
        self.idx_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_idx_req(&mut self) -> AnalyzeIndexReq {
        self.idx_req.take().unwrap_or_else(|| AnalyzeIndexReq::new())
    }

    pub fn get_idx_req(&self) -> &AnalyzeIndexReq {
        self.idx_req.as_ref().unwrap_or_else(|| AnalyzeIndexReq::default_instance())
    }

    // optional .tipb.AnalyzeColumnsReq col_req = 6;

    pub fn clear_col_req(&mut self) {
        self.col_req.clear();
    }

    pub fn has_col_req(&self) -> bool {
        self.col_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_col_req(&mut self, v: AnalyzeColumnsReq) {
        self.col_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_col_req(&mut self) -> &mut AnalyzeColumnsReq {
        if self.col_req.is_none() {
            self.col_req.set_default();
        };
        self.col_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_col_req(&mut self) -> AnalyzeColumnsReq {
        self.col_req.take().unwrap_or_else(|| AnalyzeColumnsReq::new())
    }

    pub fn get_col_req(&self) -> &AnalyzeColumnsReq {
        self.col_req.as_ref().unwrap_or_else(|| AnalyzeColumnsReq::default_instance())
    }
}

impl ::protobuf::Message for AnalyzeReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.tp = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.start_ts = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.flags = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.time_zone_offset = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.idx_req));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.col_req));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.tp {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.start_ts {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.flags {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.time_zone_offset {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.idx_req {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.col_req {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tp {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.start_ts {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.flags {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.time_zone_offset {
            try!(os.write_int64(4, v));
        };
        if let Some(v) = self.idx_req.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.col_req.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AnalyzeReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AnalyzeReq {
    fn new() -> AnalyzeReq {
        AnalyzeReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<AnalyzeReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "tp",
                    AnalyzeReq::has_tp,
                    AnalyzeReq::get_tp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "start_ts",
                    AnalyzeReq::has_start_ts,
                    AnalyzeReq::get_start_ts,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "flags",
                    AnalyzeReq::has_flags,
                    AnalyzeReq::get_flags,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "time_zone_offset",
                    AnalyzeReq::has_time_zone_offset,
                    AnalyzeReq::get_time_zone_offset,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "idx_req",
                    AnalyzeReq::has_idx_req,
                    AnalyzeReq::get_idx_req,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "col_req",
                    AnalyzeReq::has_col_req,
                    AnalyzeReq::get_col_req,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AnalyzeReq>(
                    "AnalyzeReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AnalyzeReq {
    fn clear(&mut self) {
        self.clear_tp();
        self.clear_start_ts();
        self.clear_flags();
        self.clear_time_zone_offset();
        self.clear_idx_req();
        self.clear_col_req();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AnalyzeReq {
    fn eq(&self, other: &AnalyzeReq) -> bool {
        self.tp == other.tp &&
        self.start_ts == other.start_ts &&
        self.flags == other.flags &&
        self.time_zone_offset == other.time_zone_offset &&
        self.idx_req == other.idx_req &&
        self.col_req == other.col_req &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AnalyzeReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AnalyzeIndexReq {
    // message fields
    bucket_size: ::std::option::Option<i64>,
    num_columns: ::std::option::Option<i32>,
    cmsketch_depth: ::std::option::Option<i32>,
    cmsketch_width: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AnalyzeIndexReq {}

impl AnalyzeIndexReq {
    pub fn new() -> AnalyzeIndexReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AnalyzeIndexReq {
        static mut instance: ::protobuf::lazy::Lazy<AnalyzeIndexReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AnalyzeIndexReq,
        };
        unsafe {
            instance.get(|| {
                AnalyzeIndexReq {
                    bucket_size: ::std::option::Option::None,
                    num_columns: ::std::option::Option::None,
                    cmsketch_depth: ::std::option::Option::None,
                    cmsketch_width: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 bucket_size = 1;

    pub fn clear_bucket_size(&mut self) {
        self.bucket_size = ::std::option::Option::None;
    }

    pub fn has_bucket_size(&self) -> bool {
        self.bucket_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucket_size(&mut self, v: i64) {
        self.bucket_size = ::std::option::Option::Some(v);
    }

    pub fn get_bucket_size(&self) -> i64 {
        self.bucket_size.unwrap_or(0)
    }

    // optional int32 num_columns = 2;

    pub fn clear_num_columns(&mut self) {
        self.num_columns = ::std::option::Option::None;
    }

    pub fn has_num_columns(&self) -> bool {
        self.num_columns.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_columns(&mut self, v: i32) {
        self.num_columns = ::std::option::Option::Some(v);
    }

    pub fn get_num_columns(&self) -> i32 {
        self.num_columns.unwrap_or(0)
    }

    // optional int32 cmsketch_depth = 3;

    pub fn clear_cmsketch_depth(&mut self) {
        self.cmsketch_depth = ::std::option::Option::None;
    }

    pub fn has_cmsketch_depth(&self) -> bool {
        self.cmsketch_depth.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmsketch_depth(&mut self, v: i32) {
        self.cmsketch_depth = ::std::option::Option::Some(v);
    }

    pub fn get_cmsketch_depth(&self) -> i32 {
        self.cmsketch_depth.unwrap_or(0)
    }

    // optional int32 cmsketch_width = 4;

    pub fn clear_cmsketch_width(&mut self) {
        self.cmsketch_width = ::std::option::Option::None;
    }

    pub fn has_cmsketch_width(&self) -> bool {
        self.cmsketch_width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmsketch_width(&mut self, v: i32) {
        self.cmsketch_width = ::std::option::Option::Some(v);
    }

    pub fn get_cmsketch_width(&self) -> i32 {
        self.cmsketch_width.unwrap_or(0)
    }
}

impl ::protobuf::Message for AnalyzeIndexReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.bucket_size = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.num_columns = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.cmsketch_depth = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.cmsketch_width = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.bucket_size {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.num_columns {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.cmsketch_depth {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.cmsketch_width {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket_size {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.num_columns {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.cmsketch_depth {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.cmsketch_width {
            try!(os.write_int32(4, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AnalyzeIndexReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AnalyzeIndexReq {
    fn new() -> AnalyzeIndexReq {
        AnalyzeIndexReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<AnalyzeIndexReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "bucket_size",
                    AnalyzeIndexReq::has_bucket_size,
                    AnalyzeIndexReq::get_bucket_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "num_columns",
                    AnalyzeIndexReq::has_num_columns,
                    AnalyzeIndexReq::get_num_columns,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "cmsketch_depth",
                    AnalyzeIndexReq::has_cmsketch_depth,
                    AnalyzeIndexReq::get_cmsketch_depth,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "cmsketch_width",
                    AnalyzeIndexReq::has_cmsketch_width,
                    AnalyzeIndexReq::get_cmsketch_width,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AnalyzeIndexReq>(
                    "AnalyzeIndexReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AnalyzeIndexReq {
    fn clear(&mut self) {
        self.clear_bucket_size();
        self.clear_num_columns();
        self.clear_cmsketch_depth();
        self.clear_cmsketch_width();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AnalyzeIndexReq {
    fn eq(&self, other: &AnalyzeIndexReq) -> bool {
        self.bucket_size == other.bucket_size &&
        self.num_columns == other.num_columns &&
        self.cmsketch_depth == other.cmsketch_depth &&
        self.cmsketch_width == other.cmsketch_width &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AnalyzeIndexReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AnalyzeColumnsReq {
    // message fields
    bucket_size: ::std::option::Option<i64>,
    sample_size: ::std::option::Option<i64>,
    sketch_size: ::std::option::Option<i64>,
    columns_info: ::protobuf::RepeatedField<super::schema::ColumnInfo>,
    cmsketch_depth: ::std::option::Option<i32>,
    cmsketch_width: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AnalyzeColumnsReq {}

impl AnalyzeColumnsReq {
    pub fn new() -> AnalyzeColumnsReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AnalyzeColumnsReq {
        static mut instance: ::protobuf::lazy::Lazy<AnalyzeColumnsReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AnalyzeColumnsReq,
        };
        unsafe {
            instance.get(|| {
                AnalyzeColumnsReq {
                    bucket_size: ::std::option::Option::None,
                    sample_size: ::std::option::Option::None,
                    sketch_size: ::std::option::Option::None,
                    columns_info: ::protobuf::RepeatedField::new(),
                    cmsketch_depth: ::std::option::Option::None,
                    cmsketch_width: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 bucket_size = 1;

    pub fn clear_bucket_size(&mut self) {
        self.bucket_size = ::std::option::Option::None;
    }

    pub fn has_bucket_size(&self) -> bool {
        self.bucket_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucket_size(&mut self, v: i64) {
        self.bucket_size = ::std::option::Option::Some(v);
    }

    pub fn get_bucket_size(&self) -> i64 {
        self.bucket_size.unwrap_or(0)
    }

    // optional int64 sample_size = 2;

    pub fn clear_sample_size(&mut self) {
        self.sample_size = ::std::option::Option::None;
    }

    pub fn has_sample_size(&self) -> bool {
        self.sample_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sample_size(&mut self, v: i64) {
        self.sample_size = ::std::option::Option::Some(v);
    }

    pub fn get_sample_size(&self) -> i64 {
        self.sample_size.unwrap_or(0)
    }

    // optional int64 sketch_size = 3;

    pub fn clear_sketch_size(&mut self) {
        self.sketch_size = ::std::option::Option::None;
    }

    pub fn has_sketch_size(&self) -> bool {
        self.sketch_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sketch_size(&mut self, v: i64) {
        self.sketch_size = ::std::option::Option::Some(v);
    }

    pub fn get_sketch_size(&self) -> i64 {
        self.sketch_size.unwrap_or(0)
    }

    // repeated .tipb.ColumnInfo columns_info = 4;

    pub fn clear_columns_info(&mut self) {
        self.columns_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_columns_info(&mut self, v: ::protobuf::RepeatedField<super::schema::ColumnInfo>) {
        self.columns_info = v;
    }

    // Mutable pointer to the field.
    pub fn mut_columns_info(&mut self) -> &mut ::protobuf::RepeatedField<super::schema::ColumnInfo> {
        &mut self.columns_info
    }

    // Take field
    pub fn take_columns_info(&mut self) -> ::protobuf::RepeatedField<super::schema::ColumnInfo> {
        ::std::mem::replace(&mut self.columns_info, ::protobuf::RepeatedField::new())
    }

    pub fn get_columns_info(&self) -> &[super::schema::ColumnInfo] {
        &self.columns_info
    }

    // optional int32 cmsketch_depth = 5;

    pub fn clear_cmsketch_depth(&mut self) {
        self.cmsketch_depth = ::std::option::Option::None;
    }

    pub fn has_cmsketch_depth(&self) -> bool {
        self.cmsketch_depth.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmsketch_depth(&mut self, v: i32) {
        self.cmsketch_depth = ::std::option::Option::Some(v);
    }

    pub fn get_cmsketch_depth(&self) -> i32 {
        self.cmsketch_depth.unwrap_or(0)
    }

    // optional int32 cmsketch_width = 6;

    pub fn clear_cmsketch_width(&mut self) {
        self.cmsketch_width = ::std::option::Option::None;
    }

    pub fn has_cmsketch_width(&self) -> bool {
        self.cmsketch_width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmsketch_width(&mut self, v: i32) {
        self.cmsketch_width = ::std::option::Option::Some(v);
    }

    pub fn get_cmsketch_width(&self) -> i32 {
        self.cmsketch_width.unwrap_or(0)
    }
}

impl ::protobuf::Message for AnalyzeColumnsReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.bucket_size = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.sample_size = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.sketch_size = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.columns_info));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.cmsketch_depth = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.cmsketch_width = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.bucket_size {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.sample_size {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.sketch_size {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.columns_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.cmsketch_depth {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.cmsketch_width {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket_size {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.sample_size {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.sketch_size {
            try!(os.write_int64(3, v));
        };
        for v in &self.columns_info {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmsketch_depth {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.cmsketch_width {
            try!(os.write_int32(6, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AnalyzeColumnsReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AnalyzeColumnsReq {
    fn new() -> AnalyzeColumnsReq {
        AnalyzeColumnsReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<AnalyzeColumnsReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "bucket_size",
                    AnalyzeColumnsReq::has_bucket_size,
                    AnalyzeColumnsReq::get_bucket_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "sample_size",
                    AnalyzeColumnsReq::has_sample_size,
                    AnalyzeColumnsReq::get_sample_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "sketch_size",
                    AnalyzeColumnsReq::has_sketch_size,
                    AnalyzeColumnsReq::get_sketch_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "columns_info",
                    AnalyzeColumnsReq::get_columns_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "cmsketch_depth",
                    AnalyzeColumnsReq::has_cmsketch_depth,
                    AnalyzeColumnsReq::get_cmsketch_depth,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "cmsketch_width",
                    AnalyzeColumnsReq::has_cmsketch_width,
                    AnalyzeColumnsReq::get_cmsketch_width,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AnalyzeColumnsReq>(
                    "AnalyzeColumnsReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AnalyzeColumnsReq {
    fn clear(&mut self) {
        self.clear_bucket_size();
        self.clear_sample_size();
        self.clear_sketch_size();
        self.clear_columns_info();
        self.clear_cmsketch_depth();
        self.clear_cmsketch_width();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AnalyzeColumnsReq {
    fn eq(&self, other: &AnalyzeColumnsReq) -> bool {
        self.bucket_size == other.bucket_size &&
        self.sample_size == other.sample_size &&
        self.sketch_size == other.sketch_size &&
        self.columns_info == other.columns_info &&
        self.cmsketch_depth == other.cmsketch_depth &&
        self.cmsketch_width == other.cmsketch_width &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AnalyzeColumnsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AnalyzeColumnsResp {
    // message fields
    collectors: ::protobuf::RepeatedField<SampleCollector>,
    pk_hist: ::protobuf::SingularPtrField<Histogram>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AnalyzeColumnsResp {}

impl AnalyzeColumnsResp {
    pub fn new() -> AnalyzeColumnsResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AnalyzeColumnsResp {
        static mut instance: ::protobuf::lazy::Lazy<AnalyzeColumnsResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AnalyzeColumnsResp,
        };
        unsafe {
            instance.get(|| {
                AnalyzeColumnsResp {
                    collectors: ::protobuf::RepeatedField::new(),
                    pk_hist: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .tipb.SampleCollector collectors = 1;

    pub fn clear_collectors(&mut self) {
        self.collectors.clear();
    }

    // Param is passed by value, moved
    pub fn set_collectors(&mut self, v: ::protobuf::RepeatedField<SampleCollector>) {
        self.collectors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_collectors(&mut self) -> &mut ::protobuf::RepeatedField<SampleCollector> {
        &mut self.collectors
    }

    // Take field
    pub fn take_collectors(&mut self) -> ::protobuf::RepeatedField<SampleCollector> {
        ::std::mem::replace(&mut self.collectors, ::protobuf::RepeatedField::new())
    }

    pub fn get_collectors(&self) -> &[SampleCollector] {
        &self.collectors
    }

    // optional .tipb.Histogram pk_hist = 2;

    pub fn clear_pk_hist(&mut self) {
        self.pk_hist.clear();
    }

    pub fn has_pk_hist(&self) -> bool {
        self.pk_hist.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pk_hist(&mut self, v: Histogram) {
        self.pk_hist = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pk_hist(&mut self) -> &mut Histogram {
        if self.pk_hist.is_none() {
            self.pk_hist.set_default();
        };
        self.pk_hist.as_mut().unwrap()
    }

    // Take field
    pub fn take_pk_hist(&mut self) -> Histogram {
        self.pk_hist.take().unwrap_or_else(|| Histogram::new())
    }

    pub fn get_pk_hist(&self) -> &Histogram {
        self.pk_hist.as_ref().unwrap_or_else(|| Histogram::default_instance())
    }
}

impl ::protobuf::Message for AnalyzeColumnsResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.collectors));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pk_hist));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.collectors {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.pk_hist {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.collectors {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.pk_hist.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AnalyzeColumnsResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AnalyzeColumnsResp {
    fn new() -> AnalyzeColumnsResp {
        AnalyzeColumnsResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<AnalyzeColumnsResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "collectors",
                    AnalyzeColumnsResp::get_collectors,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pk_hist",
                    AnalyzeColumnsResp::has_pk_hist,
                    AnalyzeColumnsResp::get_pk_hist,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AnalyzeColumnsResp>(
                    "AnalyzeColumnsResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AnalyzeColumnsResp {
    fn clear(&mut self) {
        self.clear_collectors();
        self.clear_pk_hist();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AnalyzeColumnsResp {
    fn eq(&self, other: &AnalyzeColumnsResp) -> bool {
        self.collectors == other.collectors &&
        self.pk_hist == other.pk_hist &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AnalyzeColumnsResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AnalyzeIndexResp {
    // message fields
    hist: ::protobuf::SingularPtrField<Histogram>,
    cms: ::protobuf::SingularPtrField<CMSketch>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AnalyzeIndexResp {}

impl AnalyzeIndexResp {
    pub fn new() -> AnalyzeIndexResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AnalyzeIndexResp {
        static mut instance: ::protobuf::lazy::Lazy<AnalyzeIndexResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AnalyzeIndexResp,
        };
        unsafe {
            instance.get(|| {
                AnalyzeIndexResp {
                    hist: ::protobuf::SingularPtrField::none(),
                    cms: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .tipb.Histogram hist = 1;

    pub fn clear_hist(&mut self) {
        self.hist.clear();
    }

    pub fn has_hist(&self) -> bool {
        self.hist.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hist(&mut self, v: Histogram) {
        self.hist = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hist(&mut self) -> &mut Histogram {
        if self.hist.is_none() {
            self.hist.set_default();
        };
        self.hist.as_mut().unwrap()
    }

    // Take field
    pub fn take_hist(&mut self) -> Histogram {
        self.hist.take().unwrap_or_else(|| Histogram::new())
    }

    pub fn get_hist(&self) -> &Histogram {
        self.hist.as_ref().unwrap_or_else(|| Histogram::default_instance())
    }

    // optional .tipb.CMSketch cms = 2;

    pub fn clear_cms(&mut self) {
        self.cms.clear();
    }

    pub fn has_cms(&self) -> bool {
        self.cms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cms(&mut self, v: CMSketch) {
        self.cms = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cms(&mut self) -> &mut CMSketch {
        if self.cms.is_none() {
            self.cms.set_default();
        };
        self.cms.as_mut().unwrap()
    }

    // Take field
    pub fn take_cms(&mut self) -> CMSketch {
        self.cms.take().unwrap_or_else(|| CMSketch::new())
    }

    pub fn get_cms(&self) -> &CMSketch {
        self.cms.as_ref().unwrap_or_else(|| CMSketch::default_instance())
    }
}

impl ::protobuf::Message for AnalyzeIndexResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.hist));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cms));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.hist {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.cms {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hist.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cms.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AnalyzeIndexResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AnalyzeIndexResp {
    fn new() -> AnalyzeIndexResp {
        AnalyzeIndexResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<AnalyzeIndexResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "hist",
                    AnalyzeIndexResp::has_hist,
                    AnalyzeIndexResp::get_hist,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cms",
                    AnalyzeIndexResp::has_cms,
                    AnalyzeIndexResp::get_cms,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AnalyzeIndexResp>(
                    "AnalyzeIndexResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AnalyzeIndexResp {
    fn clear(&mut self) {
        self.clear_hist();
        self.clear_cms();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AnalyzeIndexResp {
    fn eq(&self, other: &AnalyzeIndexResp) -> bool {
        self.hist == other.hist &&
        self.cms == other.cms &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AnalyzeIndexResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Bucket {
    // message fields
    count: ::std::option::Option<i64>,
    lower_bound: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    upper_bound: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    repeats: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Bucket {}

impl Bucket {
    pub fn new() -> Bucket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Bucket {
        static mut instance: ::protobuf::lazy::Lazy<Bucket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Bucket,
        };
        unsafe {
            instance.get(|| {
                Bucket {
                    count: ::std::option::Option::None,
                    lower_bound: ::protobuf::SingularField::none(),
                    upper_bound: ::protobuf::SingularField::none(),
                    repeats: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 count = 1;

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: i64) {
        self.count = ::std::option::Option::Some(v);
    }

    pub fn get_count(&self) -> i64 {
        self.count.unwrap_or(0)
    }

    // optional bytes lower_bound = 2;

    pub fn clear_lower_bound(&mut self) {
        self.lower_bound.clear();
    }

    pub fn has_lower_bound(&self) -> bool {
        self.lower_bound.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lower_bound(&mut self, v: ::std::vec::Vec<u8>) {
        self.lower_bound = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lower_bound(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.lower_bound.is_none() {
            self.lower_bound.set_default();
        };
        self.lower_bound.as_mut().unwrap()
    }

    // Take field
    pub fn take_lower_bound(&mut self) -> ::std::vec::Vec<u8> {
        self.lower_bound.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_lower_bound(&self) -> &[u8] {
        match self.lower_bound.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes upper_bound = 3;

    pub fn clear_upper_bound(&mut self) {
        self.upper_bound.clear();
    }

    pub fn has_upper_bound(&self) -> bool {
        self.upper_bound.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upper_bound(&mut self, v: ::std::vec::Vec<u8>) {
        self.upper_bound = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_upper_bound(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.upper_bound.is_none() {
            self.upper_bound.set_default();
        };
        self.upper_bound.as_mut().unwrap()
    }

    // Take field
    pub fn take_upper_bound(&mut self) -> ::std::vec::Vec<u8> {
        self.upper_bound.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_upper_bound(&self) -> &[u8] {
        match self.upper_bound.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional int64 repeats = 4;

    pub fn clear_repeats(&mut self) {
        self.repeats = ::std::option::Option::None;
    }

    pub fn has_repeats(&self) -> bool {
        self.repeats.is_some()
    }

    // Param is passed by value, moved
    pub fn set_repeats(&mut self, v: i64) {
        self.repeats = ::std::option::Option::Some(v);
    }

    pub fn get_repeats(&self) -> i64 {
        self.repeats.unwrap_or(0)
    }
}

impl ::protobuf::Message for Bucket {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.count = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.lower_bound));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.upper_bound));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.repeats = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.count {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.lower_bound {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.upper_bound {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in &self.repeats {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.count {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.lower_bound.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.upper_bound.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.repeats {
            try!(os.write_int64(4, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Bucket>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Bucket {
    fn new() -> Bucket {
        Bucket::new()
    }

    fn descriptor_static(_: ::std::option::Option<Bucket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "count",
                    Bucket::has_count,
                    Bucket::get_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "lower_bound",
                    Bucket::has_lower_bound,
                    Bucket::get_lower_bound,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "upper_bound",
                    Bucket::has_upper_bound,
                    Bucket::get_upper_bound,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "repeats",
                    Bucket::has_repeats,
                    Bucket::get_repeats,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Bucket>(
                    "Bucket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Bucket {
    fn clear(&mut self) {
        self.clear_count();
        self.clear_lower_bound();
        self.clear_upper_bound();
        self.clear_repeats();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Bucket {
    fn eq(&self, other: &Bucket) -> bool {
        self.count == other.count &&
        self.lower_bound == other.lower_bound &&
        self.upper_bound == other.upper_bound &&
        self.repeats == other.repeats &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Bucket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Histogram {
    // message fields
    ndv: ::std::option::Option<i64>,
    buckets: ::protobuf::RepeatedField<Bucket>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Histogram {}

impl Histogram {
    pub fn new() -> Histogram {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Histogram {
        static mut instance: ::protobuf::lazy::Lazy<Histogram> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Histogram,
        };
        unsafe {
            instance.get(|| {
                Histogram {
                    ndv: ::std::option::Option::None,
                    buckets: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 ndv = 1;

    pub fn clear_ndv(&mut self) {
        self.ndv = ::std::option::Option::None;
    }

    pub fn has_ndv(&self) -> bool {
        self.ndv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ndv(&mut self, v: i64) {
        self.ndv = ::std::option::Option::Some(v);
    }

    pub fn get_ndv(&self) -> i64 {
        self.ndv.unwrap_or(0)
    }

    // repeated .tipb.Bucket buckets = 2;

    pub fn clear_buckets(&mut self) {
        self.buckets.clear();
    }

    // Param is passed by value, moved
    pub fn set_buckets(&mut self, v: ::protobuf::RepeatedField<Bucket>) {
        self.buckets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_buckets(&mut self) -> &mut ::protobuf::RepeatedField<Bucket> {
        &mut self.buckets
    }

    // Take field
    pub fn take_buckets(&mut self) -> ::protobuf::RepeatedField<Bucket> {
        ::std::mem::replace(&mut self.buckets, ::protobuf::RepeatedField::new())
    }

    pub fn get_buckets(&self) -> &[Bucket] {
        &self.buckets
    }
}

impl ::protobuf::Message for Histogram {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.ndv = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.buckets));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.ndv {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.buckets {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ndv {
            try!(os.write_int64(1, v));
        };
        for v in &self.buckets {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Histogram>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Histogram {
    fn new() -> Histogram {
        Histogram::new()
    }

    fn descriptor_static(_: ::std::option::Option<Histogram>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "ndv",
                    Histogram::has_ndv,
                    Histogram::get_ndv,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "buckets",
                    Histogram::get_buckets,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Histogram>(
                    "Histogram",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Histogram {
    fn clear(&mut self) {
        self.clear_ndv();
        self.clear_buckets();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Histogram {
    fn eq(&self, other: &Histogram) -> bool {
        self.ndv == other.ndv &&
        self.buckets == other.buckets &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Histogram {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FMSketch {
    // message fields
    mask: ::std::option::Option<u64>,
    hashset: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FMSketch {}

impl FMSketch {
    pub fn new() -> FMSketch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FMSketch {
        static mut instance: ::protobuf::lazy::Lazy<FMSketch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FMSketch,
        };
        unsafe {
            instance.get(|| {
                FMSketch {
                    mask: ::std::option::Option::None,
                    hashset: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 mask = 1;

    pub fn clear_mask(&mut self) {
        self.mask = ::std::option::Option::None;
    }

    pub fn has_mask(&self) -> bool {
        self.mask.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mask(&mut self, v: u64) {
        self.mask = ::std::option::Option::Some(v);
    }

    pub fn get_mask(&self) -> u64 {
        self.mask.unwrap_or(0)
    }

    // repeated uint64 hashset = 2;

    pub fn clear_hashset(&mut self) {
        self.hashset.clear();
    }

    // Param is passed by value, moved
    pub fn set_hashset(&mut self, v: ::std::vec::Vec<u64>) {
        self.hashset = v;
    }

    // Mutable pointer to the field.
    pub fn mut_hashset(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.hashset
    }

    // Take field
    pub fn take_hashset(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.hashset, ::std::vec::Vec::new())
    }

    pub fn get_hashset(&self) -> &[u64] {
        &self.hashset
    }
}

impl ::protobuf::Message for FMSketch {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.mask = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.hashset));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.mask {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.hashset {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.mask {
            try!(os.write_uint64(1, v));
        };
        for v in &self.hashset {
            try!(os.write_uint64(2, *v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<FMSketch>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FMSketch {
    fn new() -> FMSketch {
        FMSketch::new()
    }

    fn descriptor_static(_: ::std::option::Option<FMSketch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "mask",
                    FMSketch::has_mask,
                    FMSketch::get_mask,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u64_accessor(
                    "hashset",
                    FMSketch::get_hashset,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FMSketch>(
                    "FMSketch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FMSketch {
    fn clear(&mut self) {
        self.clear_mask();
        self.clear_hashset();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FMSketch {
    fn eq(&self, other: &FMSketch) -> bool {
        self.mask == other.mask &&
        self.hashset == other.hashset &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FMSketch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SampleCollector {
    // message fields
    samples: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    null_count: ::std::option::Option<i64>,
    count: ::std::option::Option<i64>,
    fm_sketch: ::protobuf::SingularPtrField<FMSketch>,
    cm_sketch: ::protobuf::SingularPtrField<CMSketch>,
    total_size: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SampleCollector {}

impl SampleCollector {
    pub fn new() -> SampleCollector {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SampleCollector {
        static mut instance: ::protobuf::lazy::Lazy<SampleCollector> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SampleCollector,
        };
        unsafe {
            instance.get(|| {
                SampleCollector {
                    samples: ::protobuf::RepeatedField::new(),
                    null_count: ::std::option::Option::None,
                    count: ::std::option::Option::None,
                    fm_sketch: ::protobuf::SingularPtrField::none(),
                    cm_sketch: ::protobuf::SingularPtrField::none(),
                    total_size: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated bytes samples = 1;

    pub fn clear_samples(&mut self) {
        self.samples.clear();
    }

    // Param is passed by value, moved
    pub fn set_samples(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.samples = v;
    }

    // Mutable pointer to the field.
    pub fn mut_samples(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.samples
    }

    // Take field
    pub fn take_samples(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.samples, ::protobuf::RepeatedField::new())
    }

    pub fn get_samples(&self) -> &[::std::vec::Vec<u8>] {
        &self.samples
    }

    // optional int64 null_count = 2;

    pub fn clear_null_count(&mut self) {
        self.null_count = ::std::option::Option::None;
    }

    pub fn has_null_count(&self) -> bool {
        self.null_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_null_count(&mut self, v: i64) {
        self.null_count = ::std::option::Option::Some(v);
    }

    pub fn get_null_count(&self) -> i64 {
        self.null_count.unwrap_or(0)
    }

    // optional int64 count = 3;

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: i64) {
        self.count = ::std::option::Option::Some(v);
    }

    pub fn get_count(&self) -> i64 {
        self.count.unwrap_or(0)
    }

    // optional .tipb.FMSketch fm_sketch = 4;

    pub fn clear_fm_sketch(&mut self) {
        self.fm_sketch.clear();
    }

    pub fn has_fm_sketch(&self) -> bool {
        self.fm_sketch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fm_sketch(&mut self, v: FMSketch) {
        self.fm_sketch = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fm_sketch(&mut self) -> &mut FMSketch {
        if self.fm_sketch.is_none() {
            self.fm_sketch.set_default();
        };
        self.fm_sketch.as_mut().unwrap()
    }

    // Take field
    pub fn take_fm_sketch(&mut self) -> FMSketch {
        self.fm_sketch.take().unwrap_or_else(|| FMSketch::new())
    }

    pub fn get_fm_sketch(&self) -> &FMSketch {
        self.fm_sketch.as_ref().unwrap_or_else(|| FMSketch::default_instance())
    }

    // optional .tipb.CMSketch cm_sketch = 5;

    pub fn clear_cm_sketch(&mut self) {
        self.cm_sketch.clear();
    }

    pub fn has_cm_sketch(&self) -> bool {
        self.cm_sketch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cm_sketch(&mut self, v: CMSketch) {
        self.cm_sketch = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cm_sketch(&mut self) -> &mut CMSketch {
        if self.cm_sketch.is_none() {
            self.cm_sketch.set_default();
        };
        self.cm_sketch.as_mut().unwrap()
    }

    // Take field
    pub fn take_cm_sketch(&mut self) -> CMSketch {
        self.cm_sketch.take().unwrap_or_else(|| CMSketch::new())
    }

    pub fn get_cm_sketch(&self) -> &CMSketch {
        self.cm_sketch.as_ref().unwrap_or_else(|| CMSketch::default_instance())
    }

    // optional int64 total_size = 6;

    pub fn clear_total_size(&mut self) {
        self.total_size = ::std::option::Option::None;
    }

    pub fn has_total_size(&self) -> bool {
        self.total_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_size(&mut self, v: i64) {
        self.total_size = ::std::option::Option::Some(v);
    }

    pub fn get_total_size(&self) -> i64 {
        self.total_size.unwrap_or(0)
    }
}

impl ::protobuf::Message for SampleCollector {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.samples));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.null_count = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.count = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fm_sketch));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cm_sketch));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.total_size = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.samples {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.null_count {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.count {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.fm_sketch {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.cm_sketch {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.total_size {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.samples {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.null_count {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.count {
            try!(os.write_int64(3, v));
        };
        if let Some(v) = self.fm_sketch.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cm_sketch.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.total_size {
            try!(os.write_int64(6, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SampleCollector>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SampleCollector {
    fn new() -> SampleCollector {
        SampleCollector::new()
    }

    fn descriptor_static(_: ::std::option::Option<SampleCollector>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "samples",
                    SampleCollector::get_samples,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "null_count",
                    SampleCollector::has_null_count,
                    SampleCollector::get_null_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "count",
                    SampleCollector::has_count,
                    SampleCollector::get_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fm_sketch",
                    SampleCollector::has_fm_sketch,
                    SampleCollector::get_fm_sketch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cm_sketch",
                    SampleCollector::has_cm_sketch,
                    SampleCollector::get_cm_sketch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "total_size",
                    SampleCollector::has_total_size,
                    SampleCollector::get_total_size,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SampleCollector>(
                    "SampleCollector",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SampleCollector {
    fn clear(&mut self) {
        self.clear_samples();
        self.clear_null_count();
        self.clear_count();
        self.clear_fm_sketch();
        self.clear_cm_sketch();
        self.clear_total_size();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SampleCollector {
    fn eq(&self, other: &SampleCollector) -> bool {
        self.samples == other.samples &&
        self.null_count == other.null_count &&
        self.count == other.count &&
        self.fm_sketch == other.fm_sketch &&
        self.cm_sketch == other.cm_sketch &&
        self.total_size == other.total_size &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SampleCollector {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMSketchRow {
    // message fields
    counters: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMSketchRow {}

impl CMSketchRow {
    pub fn new() -> CMSketchRow {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMSketchRow {
        static mut instance: ::protobuf::lazy::Lazy<CMSketchRow> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMSketchRow,
        };
        unsafe {
            instance.get(|| {
                CMSketchRow {
                    counters: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated uint32 counters = 1;

    pub fn clear_counters(&mut self) {
        self.counters.clear();
    }

    // Param is passed by value, moved
    pub fn set_counters(&mut self, v: ::std::vec::Vec<u32>) {
        self.counters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_counters(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.counters
    }

    // Take field
    pub fn take_counters(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.counters, ::std::vec::Vec::new())
    }

    pub fn get_counters(&self) -> &[u32] {
        &self.counters
    }
}

impl ::protobuf::Message for CMSketchRow {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.counters));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.counters {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.counters {
            try!(os.write_uint32(1, *v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CMSketchRow>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMSketchRow {
    fn new() -> CMSketchRow {
        CMSketchRow::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMSketchRow>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_u32_accessor(
                    "counters",
                    CMSketchRow::get_counters,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMSketchRow>(
                    "CMSketchRow",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMSketchRow {
    fn clear(&mut self) {
        self.clear_counters();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMSketchRow {
    fn eq(&self, other: &CMSketchRow) -> bool {
        self.counters == other.counters &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CMSketchRow {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMSketch {
    // message fields
    rows: ::protobuf::RepeatedField<CMSketchRow>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMSketch {}

impl CMSketch {
    pub fn new() -> CMSketch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMSketch {
        static mut instance: ::protobuf::lazy::Lazy<CMSketch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMSketch,
        };
        unsafe {
            instance.get(|| {
                CMSketch {
                    rows: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .tipb.CMSketchRow rows = 1;

    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    // Param is passed by value, moved
    pub fn set_rows(&mut self, v: ::protobuf::RepeatedField<CMSketchRow>) {
        self.rows = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rows(&mut self) -> &mut ::protobuf::RepeatedField<CMSketchRow> {
        &mut self.rows
    }

    // Take field
    pub fn take_rows(&mut self) -> ::protobuf::RepeatedField<CMSketchRow> {
        ::std::mem::replace(&mut self.rows, ::protobuf::RepeatedField::new())
    }

    pub fn get_rows(&self) -> &[CMSketchRow] {
        &self.rows
    }
}

impl ::protobuf::Message for CMSketch {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rows));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.rows {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.rows {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CMSketch>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMSketch {
    fn new() -> CMSketch {
        CMSketch::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMSketch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "rows",
                    CMSketch::get_rows,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMSketch>(
                    "CMSketch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMSketch {
    fn clear(&mut self) {
        self.clear_rows();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMSketch {
    fn eq(&self, other: &CMSketch) -> bool {
        self.rows == other.rows &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CMSketch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AnalyzeType {
    TypeIndex = 0,
    TypeColumn = 1,
}

impl ::protobuf::ProtobufEnum for AnalyzeType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AnalyzeType> {
        match value {
            0 => ::std::option::Option::Some(AnalyzeType::TypeIndex),
            1 => ::std::option::Option::Some(AnalyzeType::TypeColumn),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AnalyzeType] = &[
            AnalyzeType::TypeIndex,
            AnalyzeType::TypeColumn,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<AnalyzeType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AnalyzeType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AnalyzeType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x61, 0x6e, 0x61, 0x6c, 0x79, 0x7a, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x04, 0x74, 0x69, 0x70, 0x62, 0x1a, 0x0c, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x14, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67,
    0x6f, 0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x84, 0x02, 0x0a, 0x0a, 0x41, 0x6e,
    0x61, 0x6c, 0x79, 0x7a, 0x65, 0x52, 0x65, 0x71, 0x12, 0x27, 0x0a, 0x02, 0x74, 0x70, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x11, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x41, 0x6e, 0x61, 0x6c,
    0x79, 0x7a, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x02, 0x74, 0x70, 0x42, 0x04, 0xc8, 0xde, 0x1f,
    0x00, 0x12, 0x1f, 0x0a, 0x08, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x74, 0x73, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x07, 0x73, 0x74, 0x61, 0x72, 0x74, 0x54, 0x73, 0x42, 0x04, 0xc8, 0xde,
    0x1f, 0x00, 0x12, 0x1a, 0x0a, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x2e,
    0x0a, 0x10, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x7a, 0x6f, 0x6e, 0x65, 0x5f, 0x6f, 0x66, 0x66, 0x73,
    0x65, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0e, 0x74, 0x69, 0x6d, 0x65, 0x5a, 0x6f,
    0x6e, 0x65, 0x4f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x2e,
    0x0a, 0x07, 0x69, 0x64, 0x78, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x15, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x41, 0x6e, 0x61, 0x6c, 0x79, 0x7a, 0x65, 0x49, 0x6e,
    0x64, 0x65, 0x78, 0x52, 0x65, 0x71, 0x52, 0x06, 0x69, 0x64, 0x78, 0x52, 0x65, 0x71, 0x12, 0x30,
    0x0a, 0x07, 0x63, 0x6f, 0x6c, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x17, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x41, 0x6e, 0x61, 0x6c, 0x79, 0x7a, 0x65, 0x43, 0x6f,
    0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x52, 0x06, 0x63, 0x6f, 0x6c, 0x52, 0x65, 0x71,
    0x22, 0xad, 0x01, 0x0a, 0x0f, 0x41, 0x6e, 0x61, 0x6c, 0x79, 0x7a, 0x65, 0x49, 0x6e, 0x64, 0x65,
    0x78, 0x52, 0x65, 0x71, 0x12, 0x25, 0x0a, 0x0b, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x5f, 0x73,
    0x69, 0x7a, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0a, 0x62, 0x75, 0x63, 0x6b, 0x65,
    0x74, 0x53, 0x69, 0x7a, 0x65, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x25, 0x0a, 0x0b, 0x6e,
    0x75, 0x6d, 0x5f, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x0a, 0x6e, 0x75, 0x6d, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x42, 0x04, 0xc8, 0xde,
    0x1f, 0x00, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x6d, 0x73, 0x6b, 0x65, 0x74, 0x63, 0x68, 0x5f, 0x64,
    0x65, 0x70, 0x74, 0x68, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x63, 0x6d, 0x73, 0x6b,
    0x65, 0x74, 0x63, 0x68, 0x44, 0x65, 0x70, 0x74, 0x68, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x6d, 0x73,
    0x6b, 0x65, 0x74, 0x63, 0x68, 0x5f, 0x77, 0x69, 0x64, 0x74, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x0d, 0x63, 0x6d, 0x73, 0x6b, 0x65, 0x74, 0x63, 0x68, 0x57, 0x69, 0x64, 0x74, 0x68,
    0x22, 0x8b, 0x02, 0x0a, 0x11, 0x41, 0x6e, 0x61, 0x6c, 0x79, 0x7a, 0x65, 0x43, 0x6f, 0x6c, 0x75,
    0x6d, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x12, 0x25, 0x0a, 0x0b, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0a, 0x62, 0x75, 0x63,
    0x6b, 0x65, 0x74, 0x53, 0x69, 0x7a, 0x65, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x25, 0x0a,
    0x0b, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x03, 0x52, 0x0a, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x53, 0x69, 0x7a, 0x65, 0x42, 0x04,
    0xc8, 0xde, 0x1f, 0x00, 0x12, 0x25, 0x0a, 0x0b, 0x73, 0x6b, 0x65, 0x74, 0x63, 0x68, 0x5f, 0x73,
    0x69, 0x7a, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0a, 0x73, 0x6b, 0x65, 0x74, 0x63,
    0x68, 0x53, 0x69, 0x7a, 0x65, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x33, 0x0a, 0x0c, 0x63,
    0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x04, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x10, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x49,
    0x6e, 0x66, 0x6f, 0x52, 0x0b, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x49, 0x6e, 0x66, 0x6f,
    0x12, 0x25, 0x0a, 0x0e, 0x63, 0x6d, 0x73, 0x6b, 0x65, 0x74, 0x63, 0x68, 0x5f, 0x64, 0x65, 0x70,
    0x74, 0x68, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x63, 0x6d, 0x73, 0x6b, 0x65, 0x74,
    0x63, 0x68, 0x44, 0x65, 0x70, 0x74, 0x68, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x6d, 0x73, 0x6b, 0x65,
    0x74, 0x63, 0x68, 0x5f, 0x77, 0x69, 0x64, 0x74, 0x68, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x0d, 0x63, 0x6d, 0x73, 0x6b, 0x65, 0x74, 0x63, 0x68, 0x57, 0x69, 0x64, 0x74, 0x68, 0x22, 0x75,
    0x0a, 0x12, 0x41, 0x6e, 0x61, 0x6c, 0x79, 0x7a, 0x65, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73,
    0x52, 0x65, 0x73, 0x70, 0x12, 0x35, 0x0a, 0x0a, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x6f,
    0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e,
    0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x43, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52,
    0x0a, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x73, 0x12, 0x28, 0x0a, 0x07, 0x70,
    0x6b, 0x5f, 0x68, 0x69, 0x73, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x74,
    0x69, 0x70, 0x62, 0x2e, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x52, 0x06, 0x70,
    0x6b, 0x48, 0x69, 0x73, 0x74, 0x22, 0x59, 0x0a, 0x10, 0x41, 0x6e, 0x61, 0x6c, 0x79, 0x7a, 0x65,
    0x49, 0x6e, 0x64, 0x65, 0x78, 0x52, 0x65, 0x73, 0x70, 0x12, 0x23, 0x0a, 0x04, 0x68, 0x69, 0x73,
    0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x48,
    0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x52, 0x04, 0x68, 0x69, 0x73, 0x74, 0x12, 0x20,
    0x0a, 0x03, 0x63, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x74, 0x69,
    0x70, 0x62, 0x2e, 0x43, 0x4d, 0x53, 0x6b, 0x65, 0x74, 0x63, 0x68, 0x52, 0x03, 0x63, 0x6d, 0x73,
    0x22, 0x86, 0x01, 0x0a, 0x06, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x1a, 0x0a, 0x05, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x52, 0x05, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x1f, 0x0a, 0x0b, 0x6c, 0x6f, 0x77, 0x65, 0x72,
    0x5f, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0a, 0x6c, 0x6f,
    0x77, 0x65, 0x72, 0x42, 0x6f, 0x75, 0x6e, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x75, 0x70, 0x70, 0x65,
    0x72, 0x5f, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0a, 0x75,
    0x70, 0x70, 0x65, 0x72, 0x42, 0x6f, 0x75, 0x6e, 0x64, 0x12, 0x1e, 0x0a, 0x07, 0x72, 0x65, 0x70,
    0x65, 0x61, 0x74, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x52, 0x07, 0x72, 0x65, 0x70, 0x65,
    0x61, 0x74, 0x73, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x22, 0x4b, 0x0a, 0x09, 0x48, 0x69, 0x73,
    0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x12, 0x16, 0x0a, 0x03, 0x6e, 0x64, 0x76, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x03, 0x52, 0x03, 0x6e, 0x64, 0x76, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x26,
    0x0a, 0x07, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x0c, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x07, 0x62,
    0x75, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x22, 0x3e, 0x0a, 0x08, 0x46, 0x4d, 0x53, 0x6b, 0x65, 0x74,
    0x63, 0x68, 0x12, 0x18, 0x0a, 0x04, 0x6d, 0x61, 0x73, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x04, 0x6d, 0x61, 0x73, 0x6b, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x18, 0x0a, 0x07,
    0x68, 0x61, 0x73, 0x68, 0x73, 0x65, 0x74, 0x18, 0x02, 0x20, 0x03, 0x28, 0x04, 0x52, 0x07, 0x68,
    0x61, 0x73, 0x68, 0x73, 0x65, 0x74, 0x22, 0xe5, 0x01, 0x0a, 0x0f, 0x53, 0x61, 0x6d, 0x70, 0x6c,
    0x65, 0x43, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x61,
    0x6d, 0x70, 0x6c, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x07, 0x73, 0x61, 0x6d,
    0x70, 0x6c, 0x65, 0x73, 0x12, 0x23, 0x0a, 0x0a, 0x6e, 0x75, 0x6c, 0x6c, 0x5f, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x09, 0x6e, 0x75, 0x6c, 0x6c, 0x43, 0x6f,
    0x75, 0x6e, 0x74, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x1a, 0x0a, 0x05, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x52, 0x05, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x42,
    0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x2b, 0x0a, 0x09, 0x66, 0x6d, 0x5f, 0x73, 0x6b, 0x65, 0x74,
    0x63, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e,
    0x46, 0x4d, 0x53, 0x6b, 0x65, 0x74, 0x63, 0x68, 0x52, 0x08, 0x66, 0x6d, 0x53, 0x6b, 0x65, 0x74,
    0x63, 0x68, 0x12, 0x2b, 0x0a, 0x09, 0x63, 0x6d, 0x5f, 0x73, 0x6b, 0x65, 0x74, 0x63, 0x68, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x43, 0x4d, 0x53,
    0x6b, 0x65, 0x74, 0x63, 0x68, 0x52, 0x08, 0x63, 0x6d, 0x53, 0x6b, 0x65, 0x74, 0x63, 0x68, 0x12,
    0x1d, 0x0a, 0x0a, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x03, 0x52, 0x09, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x53, 0x69, 0x7a, 0x65, 0x22, 0x29,
    0x0a, 0x0b, 0x43, 0x4d, 0x53, 0x6b, 0x65, 0x74, 0x63, 0x68, 0x52, 0x6f, 0x77, 0x12, 0x1a, 0x0a,
    0x08, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0d, 0x52,
    0x08, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x73, 0x22, 0x31, 0x0a, 0x08, 0x43, 0x4d, 0x53,
    0x6b, 0x65, 0x74, 0x63, 0x68, 0x12, 0x25, 0x0a, 0x04, 0x72, 0x6f, 0x77, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x43, 0x4d, 0x53, 0x6b, 0x65,
    0x74, 0x63, 0x68, 0x52, 0x6f, 0x77, 0x52, 0x04, 0x72, 0x6f, 0x77, 0x73, 0x2a, 0x2c, 0x0a, 0x0b,
    0x41, 0x6e, 0x61, 0x6c, 0x79, 0x7a, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0d, 0x0a, 0x09, 0x54,
    0x79, 0x70, 0x65, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x10, 0x00, 0x12, 0x0e, 0x0a, 0x0a, 0x54, 0x79,
    0x70, 0x65, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x10, 0x01, 0x42, 0x25, 0x0a, 0x15, 0x63, 0x6f,
    0x6d, 0x2e, 0x70, 0x69, 0x6e, 0x67, 0x63, 0x61, 0x70, 0x2e, 0x74, 0x69, 0x64, 0x62, 0x2e, 0x74,
    0x69, 0x70, 0x62, 0x50, 0x01, 0xe0, 0xe2, 0x1e, 0x01, 0xd0, 0xe2, 0x1e, 0x01, 0xc8, 0xe2, 0x1e,
    0x01, 0x4a, 0xd5, 0x2c, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x6e, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0c,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x04, 0x00, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7,
    0x07, 0x00, 0x12, 0x03, 0x04, 0x00, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x12, 0x03, 0x04, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x04, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x04, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x04,
    0x1d, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x05, 0x00, 0x2e, 0x0a, 0x0b, 0x0a, 0x04,
    0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x05, 0x00, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x01, 0x02, 0x12, 0x03, 0x05, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x05, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x05, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x07, 0x12,
    0x03, 0x05, 0x16, 0x2d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x15, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x08, 0x07, 0x1d, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x0a, 0x00, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x02, 0x12, 0x03, 0x0a, 0x00,
    0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x12, 0x03, 0x0a, 0x07, 0x20, 0x0a,
    0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x07, 0x20, 0x0a, 0x0e,
    0x0a, 0x07, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x08, 0xe7, 0x07, 0x02, 0x03, 0x12, 0x03, 0x0a, 0x23, 0x27, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x0b, 0x00, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x03, 0x12, 0x03,
    0x0b, 0x00, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x03, 0x02, 0x12, 0x03, 0x0b, 0x07,
    0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x03, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x07, 0x1c,
    0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x03, 0x03, 0x12, 0x03, 0x0b, 0x1f, 0x23, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x04,
    0x12, 0x03, 0x0c, 0x00, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x04, 0x02, 0x12, 0x03,
    0x0c, 0x07, 0x22, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x04, 0x02, 0x00, 0x12, 0x03, 0x0c,
    0x07, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c,
    0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x04, 0x03, 0x12, 0x03, 0x0c, 0x25, 0x29,
    0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x0e, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x05, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x05, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x0f, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x0f, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0f, 0x10,
    0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x10, 0x04, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x10, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x10, 0x11, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x13, 0x00, 0x1a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x13,
    0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x14, 0x04, 0x3f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x14, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x14, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x19, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x14, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08,
    0x12, 0x03, 0x14, 0x20, 0x3e, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x14, 0x21, 0x3d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x03, 0x14, 0x21, 0x35, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x14, 0x21, 0x35, 0x0a, 0x12, 0x0a, 0x0b, 0x04,
    0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x22, 0x34, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x14, 0x38,
    0x3d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x15, 0x04, 0x40, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x15, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x15, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x15, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x15, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12,
    0x03, 0x15, 0x21, 0x3f, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x15, 0x22, 0x3e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x15, 0x22, 0x36, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x01, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x15, 0x22, 0x36, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x23, 0x35, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x15, 0x39, 0x3e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x16, 0x04, 0x3d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x16, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x16, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x16, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x16, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x08, 0x12, 0x03,
    0x16, 0x1e, 0x3c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x12,
    0x03, 0x16, 0x1f, 0x3b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x16, 0x1f, 0x33, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x02, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x16, 0x1f, 0x33, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02,
    0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x20, 0x32, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x00, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x16, 0x36, 0x3b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x17, 0x04, 0x47, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x17, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x17, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x17, 0x13, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x17, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x08, 0x12, 0x03, 0x17,
    0x28, 0x46, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03,
    0x17, 0x29, 0x45, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x12, 0x03, 0x17, 0x29, 0x3d, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x03, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x17, 0x29, 0x3d, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x03,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x17, 0x2a, 0x3c, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x00, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x17, 0x40, 0x45, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x18, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x18, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x06, 0x12, 0x03, 0x18, 0x0d, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x18, 0x1d, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x18, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x19, 0x04, 0x2b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x19, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x06, 0x12, 0x03, 0x19, 0x0d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x19, 0x1f, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x05, 0x03, 0x12, 0x03, 0x19, 0x29, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x1c, 0x00, 0x26, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x17,
    0x0a, 0x3d, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x04, 0x42, 0x1a, 0x30, 0x20,
    0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6d, 0x61, 0x78, 0x20, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d,
    0x73, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x73, 0x69, 0x7a, 0x65, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x1e, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x08,
    0x12, 0x03, 0x1e, 0x23, 0x41, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x1e, 0x24, 0x40, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x03, 0x1e, 0x24, 0x38, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x24, 0x38, 0x0a, 0x12, 0x0a, 0x0b, 0x04,
    0x01, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x25, 0x37, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x1e, 0x3b,
    0x40, 0x0a, 0x41, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x21, 0x04, 0x42, 0x1a, 0x34,
    0x20, 0x6e, 0x75, 0x6d, 0x5f, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x20, 0x69, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x6f,
    0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x64,
    0x65, 0x78, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x21,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x21, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x21, 0x13, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x21, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x08, 0x12, 0x03, 0x21, 0x23, 0x41, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x21, 0x24, 0x40, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x01, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x21, 0x24, 0x38, 0x0a, 0x11, 0x0a,
    0x0a, 0x04, 0x01, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x21, 0x24, 0x38,
    0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x01, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x21, 0x25, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x21, 0x3b, 0x40, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03,
    0x23, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x23, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x23, 0x0d, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x23, 0x13, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x23, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x03, 0x12, 0x03, 0x25, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x25, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x25, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x25,
    0x13, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x25, 0x24, 0x25,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x28, 0x00, 0x39, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x28, 0x08, 0x19, 0x0a, 0x96, 0x01, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x00, 0x12, 0x03, 0x2b, 0x04, 0x42, 0x1a, 0x88, 0x01, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x5f, 0x73, 0x69, 0x7a, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x78,
    0x20, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x73, 0x20, 0x62, 0x75, 0x63, 0x6b,
    0x65, 0x74, 0x20, 0x73, 0x69, 0x7a, 0x65, 0x2c, 0x20, 0x77, 0x65, 0x20, 0x6e, 0x65, 0x65, 0x64,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x62, 0x65, 0x63, 0x61, 0x75, 0x73, 0x65, 0x20, 0x77, 0x68,
    0x65, 0x6e, 0x20, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x69,
    0x73, 0x20, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x2c, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x68,
    0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65,
    0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6c, 0x79, 0x20, 0x62, 0x75, 0x69, 0x6c, 0x74, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2b, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2b, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x08, 0x12, 0x03, 0x2b, 0x23, 0x41, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x2b, 0x24, 0x40, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x2b, 0x24, 0x38, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x02,
    0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x2b, 0x24, 0x38, 0x0a, 0x12, 0x0a,
    0x0b, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x25,
    0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03,
    0x2b, 0x3b, 0x40, 0x0a, 0x4f, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x04, 0x42,
    0x1a, 0x42, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x20, 0x69,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x78, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72,
    0x20, 0x6f, 0x66, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74,
    0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2e,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2e, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2e, 0x13, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2e, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x08, 0x12, 0x03, 0x2e, 0x23, 0x41, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x2e, 0x24, 0x40, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x02, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x2e, 0x24, 0x38, 0x0a, 0x11, 0x0a,
    0x0a, 0x04, 0x02, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x2e, 0x24, 0x38,
    0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x02, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x2e, 0x25, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x2e, 0x3b, 0x40, 0x0a, 0x32, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x31, 0x04, 0x42, 0x1a, 0x25, 0x20, 0x73, 0x6b, 0x65, 0x74, 0x63, 0x68, 0x5f, 0x73, 0x69, 0x7a,
    0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x78, 0x20, 0x73, 0x6b, 0x65,
    0x74, 0x63, 0x68, 0x20, 0x73, 0x69, 0x7a, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x31, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x31, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x31, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x31,
    0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x08, 0x12, 0x03, 0x31, 0x23, 0x41,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x31, 0x24,
    0x40, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x31, 0x24, 0x38, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x02, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x31, 0x24, 0x38, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x02, 0x02, 0x02, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x31, 0x25, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02,
    0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x31, 0x3b, 0x40, 0x0a, 0x55, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x34, 0x04, 0x29, 0x1a, 0x48, 0x20, 0x63, 0x6f, 0x6c, 0x75,
    0x6d, 0x6e, 0x73, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x69, 0x6e, 0x66, 0x6f, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x6e, 0x65, 0x65,
    0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x61, 0x6e, 0x61, 0x6c, 0x79, 0x7a, 0x65,
    0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x34, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x06, 0x12, 0x03, 0x34, 0x0d, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x34, 0x18, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x34, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x36, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04,
    0x04, 0x12, 0x03, 0x36, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12,
    0x03, 0x36, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x36,
    0x13, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x36, 0x24, 0x25,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x38, 0x04, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x05, 0x04, 0x12, 0x03, 0x38, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x05, 0x05, 0x12, 0x03, 0x38, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x38, 0x13, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03,
    0x12, 0x03, 0x38, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x3b, 0x00, 0x41,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x3b, 0x08, 0x1a, 0x0a, 0x3f, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x3d, 0x04, 0x2c, 0x1a, 0x32, 0x20, 0x63, 0x6f, 0x6c,
    0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73,
    0x61, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x73,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3d, 0x0d, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x3d, 0x1d, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x3d, 0x2a, 0x2b, 0x0a, 0x4e, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03,
    0x40, 0x04, 0x23, 0x1a, 0x41, 0x20, 0x70, 0x6b, 0x5f, 0x68, 0x69, 0x73, 0x74, 0x20, 0x69, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x77,
    0x68, 0x65, 0x6e, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x68, 0x61,
    0x6e, 0x64, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x40, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x40,
    0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x40, 0x17, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x40, 0x21, 0x22, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x43, 0x00, 0x46, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04,
    0x01, 0x12, 0x03, 0x43, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03,
    0x44, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x44, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x44, 0x0d, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x44, 0x17, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x44, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x45, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x45, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x45, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x45,
    0x16, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x45, 0x1c, 0x1d,
    0x0a, 0x30, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x49, 0x00, 0x4e, 0x01, 0x1a, 0x24, 0x20, 0x42,
    0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6c, 0x65, 0x6d,
    0x65, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d,
    0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x49, 0x08, 0x0e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x4a, 0x04, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x4a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x4a, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x4a, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x08, 0x12, 0x03, 0x4a, 0x1d,
    0x3b, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x05, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x4a,
    0x1e, 0x3a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x05, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x03, 0x4a, 0x1e, 0x32, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x05, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x4a, 0x1e, 0x32, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x05, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4a, 0x1f, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x05, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x4a, 0x35, 0x3a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x4b, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x4b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x4b, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x4b, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4b,
    0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x4c, 0x04, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x4c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4c, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4c, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x4c, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12,
    0x03, 0x4d, 0x04, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x03, 0x4d,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x05, 0x12, 0x03, 0x4d, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4d, 0x13, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x4d, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x03, 0x08, 0x12, 0x03, 0x4d, 0x1f, 0x3d, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x05,
    0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x4d, 0x20, 0x3c, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x05, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x4d, 0x20, 0x34, 0x0a, 0x11, 0x0a,
    0x0a, 0x04, 0x05, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x4d, 0x20, 0x34,
    0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x05, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x4d, 0x21, 0x33, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x05, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x4d, 0x37, 0x3c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x50, 0x00,
    0x56, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x50, 0x08, 0x11, 0x0a, 0x34,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x52, 0x04, 0x3a, 0x1a, 0x27, 0x20, 0x6e, 0x64,
    0x76, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20,
    0x6f, 0x66, 0x20, 0x64, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x63, 0x74, 0x20, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x52,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x52, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x52, 0x13, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x52, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x08, 0x12, 0x03, 0x52, 0x1b, 0x39, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x06,
    0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x52, 0x1c, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x06, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x52, 0x1c, 0x30, 0x0a, 0x11, 0x0a,
    0x0a, 0x04, 0x06, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x52, 0x1c, 0x30,
    0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x06, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x52, 0x1d, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x06, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x52, 0x33, 0x38, 0x0a, 0x32, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03,
    0x55, 0x04, 0x20, 0x1a, 0x25, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x20, 0x72, 0x65,
    0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x55, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x55, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x55, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x55,
    0x1e, 0x1f, 0x0a, 0x44, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x59, 0x00, 0x5c, 0x01, 0x1a, 0x38,
    0x20, 0x46, 0x4d, 0x53, 0x6b, 0x65, 0x74, 0x63, 0x68, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65,
    0x64, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x64, 0x69, 0x73, 0x74, 0x69,
    0x6e, 0x63, 0x74, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x63,
    0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12,
    0x03, 0x59, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x5a, 0x04,
    0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5a, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5a, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5a, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5a, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x08, 0x12, 0x03, 0x5a, 0x1d, 0x3b, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x07, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x5a, 0x1e, 0x3a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x07, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x5a, 0x1e, 0x32, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x07,
    0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x5a, 0x1e, 0x32, 0x0a, 0x12, 0x0a,
    0x0b, 0x04, 0x07, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5a, 0x1f,
    0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x07, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03,
    0x5a, 0x35, 0x3a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x5b, 0x04, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5b, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5b, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x5b, 0x1e, 0x1f, 0x0a, 0x67, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04,
    0x5f, 0x00, 0x66, 0x01, 0x1a, 0x5b, 0x20, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x43, 0x6f, 0x6c,
    0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c,
    0x65, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x63, 0x61, 0x6c, 0x63, 0x75, 0x6c, 0x61, 0x74, 0x65,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6e,
    0x64, 0x76, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x5f, 0x08, 0x17, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x60, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x60, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x60, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x60, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x60,
    0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x61, 0x04, 0x41, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x61, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x61, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x61, 0x13, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x61, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x08,
    0x12, 0x03, 0x61, 0x22, 0x40, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x08, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x61, 0x23, 0x3f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x08, 0x02, 0x01, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x03, 0x61, 0x23, 0x37, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x08, 0x02, 0x01,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x61, 0x23, 0x37, 0x0a, 0x12, 0x0a, 0x0b, 0x04,
    0x08, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x61, 0x24, 0x36, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x08, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x61, 0x3a,
    0x3f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x62, 0x04, 0x3c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x03, 0x62, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03, 0x62, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x62, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x62, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x08, 0x12,
    0x03, 0x62, 0x1d, 0x3b, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x08, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x62, 0x1e, 0x3a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x08, 0x02, 0x02, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x62, 0x1e, 0x32, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x08, 0x02, 0x02, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x62, 0x1e, 0x32, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x08,
    0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x62, 0x1f, 0x31, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x08, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x62, 0x35, 0x3a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x03, 0x63, 0x04, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x03, 0x04, 0x12, 0x03, 0x63, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x03, 0x06, 0x12, 0x03, 0x63, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x63, 0x16, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x63, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12, 0x03, 0x64,
    0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x04, 0x12, 0x03, 0x64, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x06, 0x12, 0x03, 0x64, 0x0d, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x03, 0x64, 0x16, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x03, 0x64, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08,
    0x02, 0x05, 0x12, 0x03, 0x65, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x04,
    0x12, 0x03, 0x65, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x05, 0x12, 0x03,
    0x65, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x01, 0x12, 0x03, 0x65, 0x13,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x03, 0x12, 0x03, 0x65, 0x20, 0x21, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x68, 0x00, 0x6a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x09, 0x01, 0x12, 0x03, 0x68, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12,
    0x03, 0x69, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x69,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x69, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x69, 0x14, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x69, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x0a, 0x12, 0x04, 0x6c, 0x00, 0x6e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12,
    0x03, 0x6c, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x6d, 0x04,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6d, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x03, 0x6d, 0x0d, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6d, 0x19, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6d, 0x20, 0x21,
];

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
