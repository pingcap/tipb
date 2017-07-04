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
pub struct Executor {
    // message fields
    tp: ::std::option::Option<ExecType>,
    tbl_scan: ::protobuf::SingularPtrField<TableScan>,
    idx_scan: ::protobuf::SingularPtrField<IndexScan>,
    selection: ::protobuf::SingularPtrField<Selection>,
    aggregation: ::protobuf::SingularPtrField<Aggregation>,
    topN: ::protobuf::SingularPtrField<TopN>,
    limit: ::protobuf::SingularPtrField<Limit>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Executor {}

impl Executor {
    pub fn new() -> Executor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Executor {
        static mut instance: ::protobuf::lazy::Lazy<Executor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Executor,
        };
        unsafe {
            instance.get(|| {
                Executor {
                    tp: ::std::option::Option::None,
                    tbl_scan: ::protobuf::SingularPtrField::none(),
                    idx_scan: ::protobuf::SingularPtrField::none(),
                    selection: ::protobuf::SingularPtrField::none(),
                    aggregation: ::protobuf::SingularPtrField::none(),
                    topN: ::protobuf::SingularPtrField::none(),
                    limit: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .tipb.ExecType tp = 1;

    pub fn clear_tp(&mut self) {
        self.tp = ::std::option::Option::None;
    }

    pub fn has_tp(&self) -> bool {
        self.tp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tp(&mut self, v: ExecType) {
        self.tp = ::std::option::Option::Some(v);
    }

    pub fn get_tp(&self) -> ExecType {
        self.tp.unwrap_or(ExecType::TypeTableScan)
    }

    // optional .tipb.TableScan tbl_scan = 2;

    pub fn clear_tbl_scan(&mut self) {
        self.tbl_scan.clear();
    }

    pub fn has_tbl_scan(&self) -> bool {
        self.tbl_scan.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tbl_scan(&mut self, v: TableScan) {
        self.tbl_scan = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tbl_scan(&mut self) -> &mut TableScan {
        if self.tbl_scan.is_none() {
            self.tbl_scan.set_default();
        };
        self.tbl_scan.as_mut().unwrap()
    }

    // Take field
    pub fn take_tbl_scan(&mut self) -> TableScan {
        self.tbl_scan.take().unwrap_or_else(|| TableScan::new())
    }

    pub fn get_tbl_scan(&self) -> &TableScan {
        self.tbl_scan.as_ref().unwrap_or_else(|| TableScan::default_instance())
    }

    // optional .tipb.IndexScan idx_scan = 3;

    pub fn clear_idx_scan(&mut self) {
        self.idx_scan.clear();
    }

    pub fn has_idx_scan(&self) -> bool {
        self.idx_scan.is_some()
    }

    // Param is passed by value, moved
    pub fn set_idx_scan(&mut self, v: IndexScan) {
        self.idx_scan = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_idx_scan(&mut self) -> &mut IndexScan {
        if self.idx_scan.is_none() {
            self.idx_scan.set_default();
        };
        self.idx_scan.as_mut().unwrap()
    }

    // Take field
    pub fn take_idx_scan(&mut self) -> IndexScan {
        self.idx_scan.take().unwrap_or_else(|| IndexScan::new())
    }

    pub fn get_idx_scan(&self) -> &IndexScan {
        self.idx_scan.as_ref().unwrap_or_else(|| IndexScan::default_instance())
    }

    // optional .tipb.Selection selection = 4;

    pub fn clear_selection(&mut self) {
        self.selection.clear();
    }

    pub fn has_selection(&self) -> bool {
        self.selection.is_some()
    }

    // Param is passed by value, moved
    pub fn set_selection(&mut self, v: Selection) {
        self.selection = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_selection(&mut self) -> &mut Selection {
        if self.selection.is_none() {
            self.selection.set_default();
        };
        self.selection.as_mut().unwrap()
    }

    // Take field
    pub fn take_selection(&mut self) -> Selection {
        self.selection.take().unwrap_or_else(|| Selection::new())
    }

    pub fn get_selection(&self) -> &Selection {
        self.selection.as_ref().unwrap_or_else(|| Selection::default_instance())
    }

    // optional .tipb.Aggregation aggregation = 5;

    pub fn clear_aggregation(&mut self) {
        self.aggregation.clear();
    }

    pub fn has_aggregation(&self) -> bool {
        self.aggregation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_aggregation(&mut self, v: Aggregation) {
        self.aggregation = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_aggregation(&mut self) -> &mut Aggregation {
        if self.aggregation.is_none() {
            self.aggregation.set_default();
        };
        self.aggregation.as_mut().unwrap()
    }

    // Take field
    pub fn take_aggregation(&mut self) -> Aggregation {
        self.aggregation.take().unwrap_or_else(|| Aggregation::new())
    }

    pub fn get_aggregation(&self) -> &Aggregation {
        self.aggregation.as_ref().unwrap_or_else(|| Aggregation::default_instance())
    }

    // optional .tipb.TopN topN = 6;

    pub fn clear_topN(&mut self) {
        self.topN.clear();
    }

    pub fn has_topN(&self) -> bool {
        self.topN.is_some()
    }

    // Param is passed by value, moved
    pub fn set_topN(&mut self, v: TopN) {
        self.topN = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_topN(&mut self) -> &mut TopN {
        if self.topN.is_none() {
            self.topN.set_default();
        };
        self.topN.as_mut().unwrap()
    }

    // Take field
    pub fn take_topN(&mut self) -> TopN {
        self.topN.take().unwrap_or_else(|| TopN::new())
    }

    pub fn get_topN(&self) -> &TopN {
        self.topN.as_ref().unwrap_or_else(|| TopN::default_instance())
    }

    // optional .tipb.Limit limit = 7;

    pub fn clear_limit(&mut self) {
        self.limit.clear();
    }

    pub fn has_limit(&self) -> bool {
        self.limit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: Limit) {
        self.limit = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_limit(&mut self) -> &mut Limit {
        if self.limit.is_none() {
            self.limit.set_default();
        };
        self.limit.as_mut().unwrap()
    }

    // Take field
    pub fn take_limit(&mut self) -> Limit {
        self.limit.take().unwrap_or_else(|| Limit::new())
    }

    pub fn get_limit(&self) -> &Limit {
        self.limit.as_ref().unwrap_or_else(|| Limit::default_instance())
    }
}

impl ::protobuf::Message for Executor {
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tbl_scan));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.idx_scan));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.selection));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.aggregation));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.topN));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.limit));
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
        for value in &self.tbl_scan {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.idx_scan {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.selection {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.aggregation {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.topN {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.limit {
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
        if let Some(v) = self.tbl_scan.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.idx_scan.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.selection.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.aggregation.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.topN.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.limit.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<Executor>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Executor {
    fn new() -> Executor {
        Executor::new()
    }

    fn descriptor_static(_: ::std::option::Option<Executor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "tp",
                    Executor::has_tp,
                    Executor::get_tp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "tbl_scan",
                    Executor::has_tbl_scan,
                    Executor::get_tbl_scan,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "idx_scan",
                    Executor::has_idx_scan,
                    Executor::get_idx_scan,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "selection",
                    Executor::has_selection,
                    Executor::get_selection,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "aggregation",
                    Executor::has_aggregation,
                    Executor::get_aggregation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "topN",
                    Executor::has_topN,
                    Executor::get_topN,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "limit",
                    Executor::has_limit,
                    Executor::get_limit,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Executor>(
                    "Executor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Executor {
    fn clear(&mut self) {
        self.clear_tp();
        self.clear_tbl_scan();
        self.clear_idx_scan();
        self.clear_selection();
        self.clear_aggregation();
        self.clear_topN();
        self.clear_limit();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Executor {
    fn eq(&self, other: &Executor) -> bool {
        self.tp == other.tp &&
        self.tbl_scan == other.tbl_scan &&
        self.idx_scan == other.idx_scan &&
        self.selection == other.selection &&
        self.aggregation == other.aggregation &&
        self.topN == other.topN &&
        self.limit == other.limit &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Executor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TableScan {
    // message fields
    table_id: ::std::option::Option<i64>,
    columns: ::protobuf::RepeatedField<super::schema::ColumnInfo>,
    desc: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TableScan {}

impl TableScan {
    pub fn new() -> TableScan {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TableScan {
        static mut instance: ::protobuf::lazy::Lazy<TableScan> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TableScan,
        };
        unsafe {
            instance.get(|| {
                TableScan {
                    table_id: ::std::option::Option::None,
                    columns: ::protobuf::RepeatedField::new(),
                    desc: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 table_id = 1;

    pub fn clear_table_id(&mut self) {
        self.table_id = ::std::option::Option::None;
    }

    pub fn has_table_id(&self) -> bool {
        self.table_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_id(&mut self, v: i64) {
        self.table_id = ::std::option::Option::Some(v);
    }

    pub fn get_table_id(&self) -> i64 {
        self.table_id.unwrap_or(0)
    }

    // repeated .tipb.ColumnInfo columns = 2;

    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }

    // Param is passed by value, moved
    pub fn set_columns(&mut self, v: ::protobuf::RepeatedField<super::schema::ColumnInfo>) {
        self.columns = v;
    }

    // Mutable pointer to the field.
    pub fn mut_columns(&mut self) -> &mut ::protobuf::RepeatedField<super::schema::ColumnInfo> {
        &mut self.columns
    }

    // Take field
    pub fn take_columns(&mut self) -> ::protobuf::RepeatedField<super::schema::ColumnInfo> {
        ::std::mem::replace(&mut self.columns, ::protobuf::RepeatedField::new())
    }

    pub fn get_columns(&self) -> &[super::schema::ColumnInfo] {
        &self.columns
    }

    // optional bool desc = 3;

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
}

impl ::protobuf::Message for TableScan {
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
                    self.table_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.columns));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.desc = ::std::option::Option::Some(tmp);
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
        for value in &self.table_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.columns {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.desc.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table_id {
            try!(os.write_int64(1, v));
        };
        for v in &self.columns {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.desc {
            try!(os.write_bool(3, v));
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
        ::std::any::TypeId::of::<TableScan>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TableScan {
    fn new() -> TableScan {
        TableScan::new()
    }

    fn descriptor_static(_: ::std::option::Option<TableScan>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "table_id",
                    TableScan::has_table_id,
                    TableScan::get_table_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "columns",
                    TableScan::get_columns,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "desc",
                    TableScan::has_desc,
                    TableScan::get_desc,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TableScan>(
                    "TableScan",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TableScan {
    fn clear(&mut self) {
        self.clear_table_id();
        self.clear_columns();
        self.clear_desc();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TableScan {
    fn eq(&self, other: &TableScan) -> bool {
        self.table_id == other.table_id &&
        self.columns == other.columns &&
        self.desc == other.desc &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TableScan {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct IndexScan {
    // message fields
    table_id: ::std::option::Option<i64>,
    index_id: ::std::option::Option<i64>,
    columns: ::protobuf::RepeatedField<super::schema::ColumnInfo>,
    desc: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IndexScan {}

impl IndexScan {
    pub fn new() -> IndexScan {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IndexScan {
        static mut instance: ::protobuf::lazy::Lazy<IndexScan> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IndexScan,
        };
        unsafe {
            instance.get(|| {
                IndexScan {
                    table_id: ::std::option::Option::None,
                    index_id: ::std::option::Option::None,
                    columns: ::protobuf::RepeatedField::new(),
                    desc: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 table_id = 1;

    pub fn clear_table_id(&mut self) {
        self.table_id = ::std::option::Option::None;
    }

    pub fn has_table_id(&self) -> bool {
        self.table_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_id(&mut self, v: i64) {
        self.table_id = ::std::option::Option::Some(v);
    }

    pub fn get_table_id(&self) -> i64 {
        self.table_id.unwrap_or(0)
    }

    // optional int64 index_id = 2;

    pub fn clear_index_id(&mut self) {
        self.index_id = ::std::option::Option::None;
    }

    pub fn has_index_id(&self) -> bool {
        self.index_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index_id(&mut self, v: i64) {
        self.index_id = ::std::option::Option::Some(v);
    }

    pub fn get_index_id(&self) -> i64 {
        self.index_id.unwrap_or(0)
    }

    // repeated .tipb.ColumnInfo columns = 3;

    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }

    // Param is passed by value, moved
    pub fn set_columns(&mut self, v: ::protobuf::RepeatedField<super::schema::ColumnInfo>) {
        self.columns = v;
    }

    // Mutable pointer to the field.
    pub fn mut_columns(&mut self) -> &mut ::protobuf::RepeatedField<super::schema::ColumnInfo> {
        &mut self.columns
    }

    // Take field
    pub fn take_columns(&mut self) -> ::protobuf::RepeatedField<super::schema::ColumnInfo> {
        ::std::mem::replace(&mut self.columns, ::protobuf::RepeatedField::new())
    }

    pub fn get_columns(&self) -> &[super::schema::ColumnInfo] {
        &self.columns
    }

    // optional bool desc = 4;

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
}

impl ::protobuf::Message for IndexScan {
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
                    self.table_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.index_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.columns));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.desc = ::std::option::Option::Some(tmp);
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
        for value in &self.table_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.index_id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.columns {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.desc.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table_id {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.index_id {
            try!(os.write_int64(2, v));
        };
        for v in &self.columns {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.desc {
            try!(os.write_bool(4, v));
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
        ::std::any::TypeId::of::<IndexScan>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IndexScan {
    fn new() -> IndexScan {
        IndexScan::new()
    }

    fn descriptor_static(_: ::std::option::Option<IndexScan>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "table_id",
                    IndexScan::has_table_id,
                    IndexScan::get_table_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "index_id",
                    IndexScan::has_index_id,
                    IndexScan::get_index_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "columns",
                    IndexScan::get_columns,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "desc",
                    IndexScan::has_desc,
                    IndexScan::get_desc,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IndexScan>(
                    "IndexScan",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IndexScan {
    fn clear(&mut self) {
        self.clear_table_id();
        self.clear_index_id();
        self.clear_columns();
        self.clear_desc();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for IndexScan {
    fn eq(&self, other: &IndexScan) -> bool {
        self.table_id == other.table_id &&
        self.index_id == other.index_id &&
        self.columns == other.columns &&
        self.desc == other.desc &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for IndexScan {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Selection {
    // message fields
    conditions: ::protobuf::RepeatedField<super::expression::Expr>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Selection {}

impl Selection {
    pub fn new() -> Selection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Selection {
        static mut instance: ::protobuf::lazy::Lazy<Selection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Selection,
        };
        unsafe {
            instance.get(|| {
                Selection {
                    conditions: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .tipb.Expr conditions = 1;

    pub fn clear_conditions(&mut self) {
        self.conditions.clear();
    }

    // Param is passed by value, moved
    pub fn set_conditions(&mut self, v: ::protobuf::RepeatedField<super::expression::Expr>) {
        self.conditions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_conditions(&mut self) -> &mut ::protobuf::RepeatedField<super::expression::Expr> {
        &mut self.conditions
    }

    // Take field
    pub fn take_conditions(&mut self) -> ::protobuf::RepeatedField<super::expression::Expr> {
        ::std::mem::replace(&mut self.conditions, ::protobuf::RepeatedField::new())
    }

    pub fn get_conditions(&self) -> &[super::expression::Expr] {
        &self.conditions
    }
}

impl ::protobuf::Message for Selection {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.conditions));
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
        for value in &self.conditions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.conditions {
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
        ::std::any::TypeId::of::<Selection>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Selection {
    fn new() -> Selection {
        Selection::new()
    }

    fn descriptor_static(_: ::std::option::Option<Selection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "conditions",
                    Selection::get_conditions,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Selection>(
                    "Selection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Selection {
    fn clear(&mut self) {
        self.clear_conditions();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Selection {
    fn eq(&self, other: &Selection) -> bool {
        self.conditions == other.conditions &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Selection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Projection {
    // message fields
    exprs: ::protobuf::RepeatedField<super::expression::Expr>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Projection {}

impl Projection {
    pub fn new() -> Projection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Projection {
        static mut instance: ::protobuf::lazy::Lazy<Projection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Projection,
        };
        unsafe {
            instance.get(|| {
                Projection {
                    exprs: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .tipb.Expr exprs = 1;

    pub fn clear_exprs(&mut self) {
        self.exprs.clear();
    }

    // Param is passed by value, moved
    pub fn set_exprs(&mut self, v: ::protobuf::RepeatedField<super::expression::Expr>) {
        self.exprs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_exprs(&mut self) -> &mut ::protobuf::RepeatedField<super::expression::Expr> {
        &mut self.exprs
    }

    // Take field
    pub fn take_exprs(&mut self) -> ::protobuf::RepeatedField<super::expression::Expr> {
        ::std::mem::replace(&mut self.exprs, ::protobuf::RepeatedField::new())
    }

    pub fn get_exprs(&self) -> &[super::expression::Expr] {
        &self.exprs
    }
}

impl ::protobuf::Message for Projection {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.exprs));
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
        for value in &self.exprs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.exprs {
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
        ::std::any::TypeId::of::<Projection>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Projection {
    fn new() -> Projection {
        Projection::new()
    }

    fn descriptor_static(_: ::std::option::Option<Projection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "exprs",
                    Projection::get_exprs,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Projection>(
                    "Projection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Projection {
    fn clear(&mut self) {
        self.clear_exprs();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Projection {
    fn eq(&self, other: &Projection) -> bool {
        self.exprs == other.exprs &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Projection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Aggregation {
    // message fields
    group_by: ::protobuf::RepeatedField<super::expression::Expr>,
    agg_func: ::protobuf::RepeatedField<super::expression::Expr>,
    streamed: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Aggregation {}

impl Aggregation {
    pub fn new() -> Aggregation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Aggregation {
        static mut instance: ::protobuf::lazy::Lazy<Aggregation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Aggregation,
        };
        unsafe {
            instance.get(|| {
                Aggregation {
                    group_by: ::protobuf::RepeatedField::new(),
                    agg_func: ::protobuf::RepeatedField::new(),
                    streamed: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .tipb.Expr group_by = 1;

    pub fn clear_group_by(&mut self) {
        self.group_by.clear();
    }

    // Param is passed by value, moved
    pub fn set_group_by(&mut self, v: ::protobuf::RepeatedField<super::expression::Expr>) {
        self.group_by = v;
    }

    // Mutable pointer to the field.
    pub fn mut_group_by(&mut self) -> &mut ::protobuf::RepeatedField<super::expression::Expr> {
        &mut self.group_by
    }

    // Take field
    pub fn take_group_by(&mut self) -> ::protobuf::RepeatedField<super::expression::Expr> {
        ::std::mem::replace(&mut self.group_by, ::protobuf::RepeatedField::new())
    }

    pub fn get_group_by(&self) -> &[super::expression::Expr] {
        &self.group_by
    }

    // repeated .tipb.Expr agg_func = 2;

    pub fn clear_agg_func(&mut self) {
        self.agg_func.clear();
    }

    // Param is passed by value, moved
    pub fn set_agg_func(&mut self, v: ::protobuf::RepeatedField<super::expression::Expr>) {
        self.agg_func = v;
    }

    // Mutable pointer to the field.
    pub fn mut_agg_func(&mut self) -> &mut ::protobuf::RepeatedField<super::expression::Expr> {
        &mut self.agg_func
    }

    // Take field
    pub fn take_agg_func(&mut self) -> ::protobuf::RepeatedField<super::expression::Expr> {
        ::std::mem::replace(&mut self.agg_func, ::protobuf::RepeatedField::new())
    }

    pub fn get_agg_func(&self) -> &[super::expression::Expr] {
        &self.agg_func
    }

    // optional bool streamed = 3;

    pub fn clear_streamed(&mut self) {
        self.streamed = ::std::option::Option::None;
    }

    pub fn has_streamed(&self) -> bool {
        self.streamed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_streamed(&mut self, v: bool) {
        self.streamed = ::std::option::Option::Some(v);
    }

    pub fn get_streamed(&self) -> bool {
        self.streamed.unwrap_or(false)
    }
}

impl ::protobuf::Message for Aggregation {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.group_by));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.agg_func));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.streamed = ::std::option::Option::Some(tmp);
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
        for value in &self.group_by {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.agg_func {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.streamed.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.group_by {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.agg_func {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.streamed {
            try!(os.write_bool(3, v));
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
        ::std::any::TypeId::of::<Aggregation>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Aggregation {
    fn new() -> Aggregation {
        Aggregation::new()
    }

    fn descriptor_static(_: ::std::option::Option<Aggregation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "group_by",
                    Aggregation::get_group_by,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "agg_func",
                    Aggregation::get_agg_func,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "streamed",
                    Aggregation::has_streamed,
                    Aggregation::get_streamed,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Aggregation>(
                    "Aggregation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Aggregation {
    fn clear(&mut self) {
        self.clear_group_by();
        self.clear_agg_func();
        self.clear_streamed();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Aggregation {
    fn eq(&self, other: &Aggregation) -> bool {
        self.group_by == other.group_by &&
        self.agg_func == other.agg_func &&
        self.streamed == other.streamed &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Aggregation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TopN {
    // message fields
    order_by: ::protobuf::RepeatedField<super::expression::ByItem>,
    limit: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TopN {}

impl TopN {
    pub fn new() -> TopN {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TopN {
        static mut instance: ::protobuf::lazy::Lazy<TopN> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TopN,
        };
        unsafe {
            instance.get(|| {
                TopN {
                    order_by: ::protobuf::RepeatedField::new(),
                    limit: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .tipb.ByItem order_by = 1;

    pub fn clear_order_by(&mut self) {
        self.order_by.clear();
    }

    // Param is passed by value, moved
    pub fn set_order_by(&mut self, v: ::protobuf::RepeatedField<super::expression::ByItem>) {
        self.order_by = v;
    }

    // Mutable pointer to the field.
    pub fn mut_order_by(&mut self) -> &mut ::protobuf::RepeatedField<super::expression::ByItem> {
        &mut self.order_by
    }

    // Take field
    pub fn take_order_by(&mut self) -> ::protobuf::RepeatedField<super::expression::ByItem> {
        ::std::mem::replace(&mut self.order_by, ::protobuf::RepeatedField::new())
    }

    pub fn get_order_by(&self) -> &[super::expression::ByItem] {
        &self.order_by
    }

    // optional uint64 limit = 2;

    pub fn clear_limit(&mut self) {
        self.limit = ::std::option::Option::None;
    }

    pub fn has_limit(&self) -> bool {
        self.limit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: u64) {
        self.limit = ::std::option::Option::Some(v);
    }

    pub fn get_limit(&self) -> u64 {
        self.limit.unwrap_or(0)
    }
}

impl ::protobuf::Message for TopN {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.order_by));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.limit = ::std::option::Option::Some(tmp);
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
        for value in &self.order_by {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.limit {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.order_by {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.limit {
            try!(os.write_uint64(2, v));
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
        ::std::any::TypeId::of::<TopN>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TopN {
    fn new() -> TopN {
        TopN::new()
    }

    fn descriptor_static(_: ::std::option::Option<TopN>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "order_by",
                    TopN::get_order_by,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "limit",
                    TopN::has_limit,
                    TopN::get_limit,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TopN>(
                    "TopN",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TopN {
    fn clear(&mut self) {
        self.clear_order_by();
        self.clear_limit();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TopN {
    fn eq(&self, other: &TopN) -> bool {
        self.order_by == other.order_by &&
        self.limit == other.limit &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TopN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Limit {
    // message fields
    limit: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Limit {}

impl Limit {
    pub fn new() -> Limit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Limit {
        static mut instance: ::protobuf::lazy::Lazy<Limit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Limit,
        };
        unsafe {
            instance.get(|| {
                Limit {
                    limit: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 limit = 1;

    pub fn clear_limit(&mut self) {
        self.limit = ::std::option::Option::None;
    }

    pub fn has_limit(&self) -> bool {
        self.limit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: u64) {
        self.limit = ::std::option::Option::Some(v);
    }

    pub fn get_limit(&self) -> u64 {
        self.limit.unwrap_or(0)
    }
}

impl ::protobuf::Message for Limit {
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
                    self.limit = ::std::option::Option::Some(tmp);
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
        for value in &self.limit {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.limit {
            try!(os.write_uint64(1, v));
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
        ::std::any::TypeId::of::<Limit>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Limit {
    fn new() -> Limit {
        Limit::new()
    }

    fn descriptor_static(_: ::std::option::Option<Limit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "limit",
                    Limit::has_limit,
                    Limit::get_limit,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Limit>(
                    "Limit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Limit {
    fn clear(&mut self) {
        self.clear_limit();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Limit {
    fn eq(&self, other: &Limit) -> bool {
        self.limit == other.limit &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Limit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ExecType {
    TypeTableScan = 0,
    TypeIndexScan = 1,
    TypeSelection = 2,
    TypeAggregation = 3,
    TypeTopN = 4,
    TypeLimit = 5,
}

impl ::protobuf::ProtobufEnum for ExecType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ExecType> {
        match value {
            0 => ::std::option::Option::Some(ExecType::TypeTableScan),
            1 => ::std::option::Option::Some(ExecType::TypeIndexScan),
            2 => ::std::option::Option::Some(ExecType::TypeSelection),
            3 => ::std::option::Option::Some(ExecType::TypeAggregation),
            4 => ::std::option::Option::Some(ExecType::TypeTopN),
            5 => ::std::option::Option::Some(ExecType::TypeLimit),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ExecType] = &[
            ExecType::TypeTableScan,
            ExecType::TypeIndexScan,
            ExecType::TypeSelection,
            ExecType::TypeAggregation,
            ExecType::TypeTopN,
            ExecType::TypeLimit,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ExecType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ExecType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ExecType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0e, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x04, 0x74, 0x69, 0x70, 0x62, 0x1a, 0x10, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69,
    0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0c, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x14, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2f, 0x67, 0x6f, 0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xaf, 0x02, 0x0a,
    0x08, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x12, 0x24, 0x0a, 0x02, 0x74, 0x70, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0e, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x45, 0x78, 0x65,
    0x63, 0x54, 0x79, 0x70, 0x65, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x02, 0x74, 0x70, 0x12,
    0x2a, 0x0a, 0x08, 0x74, 0x62, 0x6c, 0x5f, 0x73, 0x63, 0x61, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0f, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x53, 0x63,
    0x61, 0x6e, 0x52, 0x07, 0x74, 0x62, 0x6c, 0x53, 0x63, 0x61, 0x6e, 0x12, 0x2a, 0x0a, 0x08, 0x69,
    0x64, 0x78, 0x5f, 0x73, 0x63, 0x61, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e,
    0x74, 0x69, 0x70, 0x62, 0x2e, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x07,
    0x69, 0x64, 0x78, 0x53, 0x63, 0x61, 0x6e, 0x12, 0x2d, 0x0a, 0x09, 0x73, 0x65, 0x6c, 0x65, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x74, 0x69, 0x70,
    0x62, 0x2e, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x09, 0x73, 0x65, 0x6c,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x33, 0x0a, 0x0b, 0x61, 0x67, 0x67, 0x72, 0x65, 0x67,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x74, 0x69,
    0x70, 0x62, 0x2e, 0x41, 0x67, 0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0b,
    0x61, 0x67, 0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1e, 0x0a, 0x04, 0x74,
    0x6f, 0x70, 0x4e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x74, 0x69, 0x70, 0x62,
    0x2e, 0x54, 0x6f, 0x70, 0x4e, 0x52, 0x04, 0x74, 0x6f, 0x70, 0x4e, 0x12, 0x21, 0x0a, 0x05, 0x6c,
    0x69, 0x6d, 0x69, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x74, 0x69, 0x70,
    0x62, 0x2e, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x52, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x22, 0x72,
    0x0a, 0x09, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x53, 0x63, 0x61, 0x6e, 0x12, 0x1f, 0x0a, 0x08, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x42, 0x04, 0xc8,
    0xde, 0x1f, 0x00, 0x52, 0x07, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x49, 0x64, 0x12, 0x2a, 0x0a, 0x07,
    0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e,
    0x74, 0x69, 0x70, 0x62, 0x2e, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x52,
    0x07, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x12, 0x18, 0x0a, 0x04, 0x64, 0x65, 0x73, 0x63,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x04, 0x64, 0x65,
    0x73, 0x63, 0x22, 0x93, 0x01, 0x0a, 0x09, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x53, 0x63, 0x61, 0x6e,
    0x12, 0x1f, 0x0a, 0x08, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x03, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x07, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x49,
    0x64, 0x12, 0x1f, 0x0a, 0x08, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x03, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x07, 0x69, 0x6e, 0x64, 0x65, 0x78,
    0x49, 0x64, 0x12, 0x2a, 0x0a, 0x07, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x18, 0x03, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x43, 0x6f, 0x6c, 0x75, 0x6d,
    0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x07, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x12, 0x18,
    0x0a, 0x04, 0x64, 0x65, 0x73, 0x63, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x42, 0x04, 0xc8, 0xde,
    0x1f, 0x00, 0x52, 0x04, 0x64, 0x65, 0x73, 0x63, 0x22, 0x37, 0x0a, 0x09, 0x53, 0x65, 0x6c, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x2a, 0x0a, 0x0a, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x74, 0x69, 0x70, 0x62,
    0x2e, 0x45, 0x78, 0x70, 0x72, 0x52, 0x0a, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x22, 0x2e, 0x0a, 0x0a, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12,
    0x20, 0x0a, 0x05, 0x65, 0x78, 0x70, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0a,
    0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x45, 0x78, 0x70, 0x72, 0x52, 0x05, 0x65, 0x78, 0x70, 0x72,
    0x73, 0x22, 0x7d, 0x0a, 0x0b, 0x41, 0x67, 0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x12, 0x25, 0x0a, 0x08, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x5f, 0x62, 0x79, 0x18, 0x01, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x74, 0x69, 0x70, 0x62, 0x2e, 0x45, 0x78, 0x70, 0x72, 0x52, 0x07,
    0x67, 0x72, 0x6f, 0x75, 0x70, 0x42, 0x79, 0x12, 0x25, 0x0a, 0x08, 0x61, 0x67, 0x67, 0x5f, 0x66,
    0x75, 0x6e, 0x63, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x74, 0x69, 0x70, 0x62,
    0x2e, 0x45, 0x78, 0x70, 0x72, 0x52, 0x07, 0x61, 0x67, 0x67, 0x46, 0x75, 0x6e, 0x63, 0x12, 0x20,
    0x0a, 0x08, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x65, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08,
    0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x08, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x65, 0x64,
    0x22, 0x4b, 0x0a, 0x04, 0x54, 0x6f, 0x70, 0x4e, 0x12, 0x27, 0x0a, 0x08, 0x6f, 0x72, 0x64, 0x65,
    0x72, 0x5f, 0x62, 0x79, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x74, 0x69, 0x70,
    0x62, 0x2e, 0x42, 0x79, 0x49, 0x74, 0x65, 0x6d, 0x52, 0x07, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x42,
    0x79, 0x12, 0x1a, 0x0a, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04,
    0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x22, 0x23, 0x0a,
    0x05, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x12, 0x1a, 0x0a, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x04, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x05, 0x6c, 0x69, 0x6d,
    0x69, 0x74, 0x2a, 0x75, 0x0a, 0x08, 0x45, 0x78, 0x65, 0x63, 0x54, 0x79, 0x70, 0x65, 0x12, 0x11,
    0x0a, 0x0d, 0x54, 0x79, 0x70, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x53, 0x63, 0x61, 0x6e, 0x10,
    0x00, 0x12, 0x11, 0x0a, 0x0d, 0x54, 0x79, 0x70, 0x65, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x53, 0x63,
    0x61, 0x6e, 0x10, 0x01, 0x12, 0x11, 0x0a, 0x0d, 0x54, 0x79, 0x70, 0x65, 0x53, 0x65, 0x6c, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x10, 0x02, 0x12, 0x13, 0x0a, 0x0f, 0x54, 0x79, 0x70, 0x65, 0x41,
    0x67, 0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x10, 0x03, 0x12, 0x0c, 0x0a, 0x08,
    0x54, 0x79, 0x70, 0x65, 0x54, 0x6f, 0x70, 0x4e, 0x10, 0x04, 0x12, 0x0d, 0x0a, 0x09, 0x54, 0x79,
    0x70, 0x65, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x10, 0x05, 0x42, 0x25, 0x0a, 0x15, 0x63, 0x6f, 0x6d,
    0x2e, 0x70, 0x69, 0x6e, 0x67, 0x63, 0x61, 0x70, 0x2e, 0x74, 0x69, 0x64, 0x62, 0x2e, 0x74, 0x69,
    0x70, 0x62, 0x50, 0x01, 0xd0, 0xe2, 0x1e, 0x01, 0xc8, 0xe2, 0x1e, 0x01, 0xe0, 0xe2, 0x1e, 0x01,
    0x4a, 0xa0, 0x1c, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x4c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c,
    0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x04, 0x00, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x04, 0x00, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x03, 0x04, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x04, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x04, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x04, 0x1d,
    0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x05, 0x00, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x08,
    0xe7, 0x07, 0x01, 0x12, 0x03, 0x05, 0x00, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01,
    0x02, 0x12, 0x03, 0x05, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x05, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x05, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x07, 0x12, 0x03,
    0x05, 0x16, 0x2d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x07, 0x19, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x08, 0x07, 0x15, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12,
    0x03, 0x09, 0x07, 0x1d, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x28, 0x0a, 0x0b,
    0x0a, 0x04, 0x08, 0xe7, 0x07, 0x02, 0x12, 0x03, 0x0b, 0x00, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x08,
    0xe7, 0x07, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x07, 0x20, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07,
    0x02, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x07, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x02,
    0x03, 0x12, 0x03, 0x0b, 0x23, 0x27, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x24,
    0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x03, 0x12, 0x03, 0x0c, 0x00, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x08, 0xe7, 0x07, 0x03, 0x02, 0x12, 0x03, 0x0c, 0x07, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x08,
    0xe7, 0x07, 0x03, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x07, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7,
    0x07, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7,
    0x07, 0x03, 0x03, 0x12, 0x03, 0x0c, 0x1f, 0x23, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d,
    0x00, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x04, 0x12, 0x03, 0x0d, 0x00, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x04, 0x02, 0x12, 0x03, 0x0d, 0x07, 0x22, 0x0a, 0x0d, 0x0a,
    0x06, 0x08, 0xe7, 0x07, 0x04, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x07, 0x22, 0x0a, 0x0e, 0x0a, 0x07,
    0x08, 0xe7, 0x07, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x04, 0x03, 0x12, 0x03, 0x0d, 0x25, 0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00,
    0x12, 0x04, 0x0f, 0x00, 0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x0f,
    0x05, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x10, 0x08, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x08, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x10, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x11, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x11, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x11, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x12, 0x08,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x12, 0x08, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x12, 0x18, 0x19, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x13, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x13, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x13, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03,
    0x14, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x14, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x14, 0x13, 0x14, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05, 0x12, 0x03, 0x15, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x15, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x05, 0x02, 0x12, 0x03, 0x15, 0x14, 0x15, 0x0a, 0x27, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x19, 0x00, 0x21, 0x01, 0x1a, 0x1b, 0x20, 0x49, 0x74, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73,
    0x65, 0x6e, 0x74, 0x73, 0x20, 0x61, 0x20, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x19, 0x08, 0x10, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1a, 0x08, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x1a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x1a, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x1a, 0x1a, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1a,
    0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x1a, 0x21, 0x3f,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1a, 0x22,
    0x3e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x1a, 0x22, 0x36, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x1a, 0x22, 0x36, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x23, 0x35, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00,
    0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x1a, 0x39, 0x3e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x1b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x1b, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x1b, 0x1b, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1b, 0x26,
    0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x1c, 0x08, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x1c, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x1b, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x1c, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x1d, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x1d, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x1d, 0x11, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1d, 0x1b, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1d, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x04, 0x12, 0x03, 0x1e, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x04, 0x12, 0x03, 0x1e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12,
    0x03, 0x1e, 0x11, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1e,
    0x1d, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1e, 0x2b, 0x2c,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x1f, 0x08, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x1f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x06, 0x12, 0x03, 0x1f, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x1f, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03,
    0x12, 0x03, 0x1f, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x20,
    0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x20, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06, 0x12, 0x03, 0x20, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x20, 0x17, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x20, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x23, 0x00, 0x27, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x23,
    0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x24, 0x08, 0x43, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x24, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x24, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x24, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x08,
    0x12, 0x03, 0x24, 0x24, 0x42, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x24, 0x25, 0x41, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x03, 0x24, 0x25, 0x39, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x24, 0x25, 0x39, 0x0a, 0x12, 0x0a, 0x0b, 0x04,
    0x01, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x26, 0x38, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x24, 0x3c,
    0x41, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x25, 0x08, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x25, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x25, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x25, 0x1c, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x25, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03,
    0x26, 0x08, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x26, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x26, 0x11, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x26, 0x16, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x26, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x08, 0x12, 0x03, 0x26, 0x1f, 0x3d, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02,
    0x02, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x26, 0x20, 0x3c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01,
    0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x26, 0x20, 0x34, 0x0a, 0x11, 0x0a, 0x0a,
    0x04, 0x01, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x26, 0x20, 0x34, 0x0a,
    0x12, 0x0a, 0x0b, 0x04, 0x01, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x26, 0x21, 0x33, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x03,
    0x12, 0x03, 0x26, 0x37, 0x3c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x29, 0x00, 0x2e,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x29, 0x08, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x2a, 0x08, 0x43, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x2a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x2a, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x2a, 0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2a,
    0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x08, 0x12, 0x03, 0x2a, 0x24, 0x42,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x2a, 0x25,
    0x41, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x2a, 0x25, 0x39, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x2a, 0x25, 0x39, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2a, 0x26, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02,
    0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x2a, 0x3c, 0x41, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x43, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x2b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x2b, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x2b, 0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2b, 0x22,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x08, 0x12, 0x03, 0x2b, 0x24, 0x42, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x2b, 0x25, 0x41,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x2b,
    0x25, 0x39, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x02, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x2b, 0x25, 0x39, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x02, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x26, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02,
    0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x3c, 0x41, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x2c, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x2c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12,
    0x03, 0x2c, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2c,
    0x1c, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2c, 0x26, 0x27,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2d, 0x08, 0x3e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x2d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x2d, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x2d, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x2d, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x08, 0x12, 0x03,
    0x2d, 0x1f, 0x3d, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x12,
    0x03, 0x2d, 0x20, 0x3c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x2d, 0x20, 0x34, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x02, 0x02, 0x03, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x2d, 0x20, 0x34, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x02, 0x02,
    0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x21, 0x33, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x02, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x37, 0x3c, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x30, 0x00, 0x33, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x03, 0x01, 0x12, 0x03, 0x30, 0x08, 0x11, 0x0a, 0x20, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12,
    0x03, 0x32, 0x08, 0x25, 0x1a, 0x13, 0x20, 0x57, 0x68, 0x65, 0x72, 0x65, 0x20, 0x63, 0x6f, 0x6e,
    0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x32, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x32, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x32, 0x16, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x32, 0x23,
    0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x35, 0x00, 0x38, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x35, 0x08, 0x12, 0x0a, 0x26, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x00, 0x12, 0x03, 0x37, 0x08, 0x20, 0x1a, 0x19, 0x20, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x37, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x37, 0x11, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x37, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x37, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x3a, 0x00, 0x41, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x3a, 0x08,
    0x13, 0x0a, 0x1f, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x3c, 0x08, 0x23, 0x1a, 0x12,
    0x20, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x20, 0x62, 0x79, 0x20, 0x63, 0x6c, 0x61, 0x75, 0x73, 0x65,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3c, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3c, 0x11, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3c, 0x16, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3c, 0x21, 0x22, 0x0a, 0x23, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x23, 0x1a, 0x16, 0x20, 0x41, 0x67, 0x67, 0x72, 0x65, 0x67,
    0x61, 0x74, 0x65, 0x20, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3e, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03, 0x3e, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3e, 0x16, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x3e, 0x21, 0x22, 0x0a, 0x2d, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12,
    0x03, 0x40, 0x08, 0x42, 0x1a, 0x20, 0x20, 0x49, 0x66, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20,
    0x61, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x61, 0x67, 0x67, 0x72, 0x65, 0x67, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x40, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x40,
    0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x40, 0x16, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x40, 0x21, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x08, 0x12, 0x03, 0x40, 0x23, 0x41, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x05, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x40, 0x24, 0x40, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x05, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x40, 0x24, 0x38, 0x0a,
    0x11, 0x0a, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x40,
    0x24, 0x38, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x05, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x40, 0x25, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x05, 0x02, 0x02, 0x08, 0xe7,
    0x07, 0x00, 0x03, 0x12, 0x03, 0x40, 0x3b, 0x40, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04,
    0x43, 0x00, 0x47, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x43, 0x08, 0x0c,
    0x0a, 0x1f, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x45, 0x08, 0x25, 0x1a, 0x12, 0x20,
    0x4f, 0x72, 0x64, 0x65, 0x72, 0x20, 0x62, 0x79, 0x20, 0x63, 0x6c, 0x61, 0x75, 0x73, 0x65, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x45, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x45, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x45, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x45, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x01, 0x12, 0x03, 0x46, 0x08, 0x41, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x46, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x46,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x46, 0x18, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x46, 0x20, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x08, 0x12, 0x03, 0x46, 0x22, 0x40, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x06, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x46, 0x23, 0x3f, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x06, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x46, 0x23, 0x37, 0x0a,
    0x11, 0x0a, 0x0a, 0x04, 0x06, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x46,
    0x23, 0x37, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x06, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x46, 0x24, 0x36, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x06, 0x02, 0x01, 0x08, 0xe7,
    0x07, 0x00, 0x03, 0x12, 0x03, 0x46, 0x3a, 0x3f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04,
    0x49, 0x00, 0x4c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x49, 0x08, 0x0d,
    0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x4b, 0x08, 0x41, 0x1a, 0x22, 0x20,
    0x4c, 0x69, 0x6d, 0x69, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4b, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x4b, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4b, 0x18, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4b, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x08, 0x12, 0x03, 0x4b, 0x22, 0x40, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x07, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x4b, 0x23, 0x3f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x07, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x4b, 0x23, 0x37, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x07,
    0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x4b, 0x23, 0x37, 0x0a, 0x12, 0x0a,
    0x0b, 0x04, 0x07, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4b, 0x24,
    0x36, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x07, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03,
    0x4b, 0x3a, 0x3f,
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
