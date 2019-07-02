// Generated file, please don't edit manually.

impl TableInfo {
    pub fn new_() -> TableInfo {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_table_id(&self) -> bool {
        self.table_id.is_some()
    }
    #[inline]
    pub fn clear_table_id(&mut self) {
        self.table_id = ::std::option::Option::None
    }
    #[inline]
    pub fn set_table_id(&mut self, v: i64) {
        self.table_id = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_table_id(&self) -> i64 {
        match self.table_id {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }
    #[inline]
    pub fn set_columns(&mut self, v: ::std::vec::Vec<ColumnInfo>) {
        self.columns = v;
    }
    #[inline]
    pub fn get_columns(&self) -> &::std::vec::Vec<ColumnInfo> {
        &self.columns
    }
    #[inline]
    pub fn mut_columns(&mut self) -> &mut ::std::vec::Vec<ColumnInfo> {
        &mut self.columns
    }
    #[inline]
    pub fn take_columns(&mut self) -> ::std::vec::Vec<ColumnInfo> {
        ::std::mem::replace(&mut self.columns, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for TableInfo {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for TableInfo {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static TableInfo {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: TableInfo = TableInfo::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ColumnInfo {
    pub fn new_() -> ColumnInfo {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_column_id(&self) -> bool {
        self.column_id.is_some()
    }
    #[inline]
    pub fn clear_column_id(&mut self) {
        self.column_id = ::std::option::Option::None
    }
    #[inline]
    pub fn set_column_id(&mut self, v: i64) {
        self.column_id = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_column_id(&self) -> i64 {
        match self.column_id {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_tp(&self) -> bool {
        self.tp.is_some()
    }
    #[inline]
    pub fn clear_tp(&mut self) {
        self.tp = ::std::option::Option::None
    }
    #[inline]
    pub fn set_tp(&mut self, v: i32) {
        self.tp = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_tp(&self) -> i32 {
        match self.tp {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_collation(&self) -> bool {
        self.collation.is_some()
    }
    #[inline]
    pub fn clear_collation(&mut self) {
        self.collation = ::std::option::Option::None
    }
    #[inline]
    pub fn set_collation(&mut self, v: i32) {
        self.collation = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_collation(&self) -> i32 {
        match self.collation {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_column_len(&self) -> bool {
        self.column_len.is_some()
    }
    #[inline]
    pub fn clear_column_len(&mut self) {
        self.column_len = ::std::option::Option::None
    }
    #[inline]
    pub fn set_column_len(&mut self, v: i32) {
        self.column_len = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_column_len(&self) -> i32 {
        match self.column_len {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_decimal(&self) -> bool {
        self.decimal.is_some()
    }
    #[inline]
    pub fn clear_decimal(&mut self) {
        self.decimal = ::std::option::Option::None
    }
    #[inline]
    pub fn set_decimal(&mut self, v: i32) {
        self.decimal = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_decimal(&self) -> i32 {
        match self.decimal {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_flag(&self) -> bool {
        self.flag.is_some()
    }
    #[inline]
    pub fn clear_flag(&mut self) {
        self.flag = ::std::option::Option::None
    }
    #[inline]
    pub fn set_flag(&mut self, v: i32) {
        self.flag = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_flag(&self) -> i32 {
        match self.flag {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn clear_elems(&mut self) {
        self.elems.clear();
    }
    #[inline]
    pub fn set_elems(&mut self, v: ::std::vec::Vec<std::string::String>) {
        self.elems = v;
    }
    #[inline]
    pub fn get_elems(&self) -> &::std::vec::Vec<std::string::String> {
        &self.elems
    }
    #[inline]
    pub fn mut_elems(&mut self) -> &mut ::std::vec::Vec<std::string::String> {
        &mut self.elems
    }
    #[inline]
    pub fn take_elems(&mut self) -> ::std::vec::Vec<std::string::String> {
        ::std::mem::replace(&mut self.elems, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_default_val(&self) -> bool {
        self.default_val.is_some()
    }
    #[inline]
    pub fn clear_default_val(&mut self) {
        self.default_val = ::std::option::Option::None
    }
    #[inline]
    pub fn set_default_val(&mut self, v: std::vec::Vec<u8>) {
        self.default_val = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_default_val(&self) -> &[u8] {
        match self.default_val.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }
    #[inline]
    pub fn mut_default_val(&mut self) -> &mut std::vec::Vec<u8> {
        if self.default_val.is_none() {
            self.default_val = ::std::option::Option::Some(::std::vec::Vec::default());
        }
        self.default_val.as_mut().unwrap()
    }
    #[inline]
    pub fn take_default_val(&mut self) -> std::vec::Vec<u8> {
        self.default_val.take().unwrap_or_else(::std::vec::Vec::new)
    }
    #[inline]
    pub fn has_pk_handle(&self) -> bool {
        self.pk_handle.is_some()
    }
    #[inline]
    pub fn clear_pk_handle(&mut self) {
        self.pk_handle = ::std::option::Option::None
    }
    #[inline]
    pub fn set_pk_handle(&mut self, v: bool) {
        self.pk_handle = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_pk_handle(&self) -> bool {
        match self.pk_handle {
            Some(v) => v,
            None => false,
        }
    }
}
impl ::protobuf::Clear for ColumnInfo {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ColumnInfo {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static ColumnInfo {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ColumnInfo = ColumnInfo::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl IndexInfo {
    pub fn new_() -> IndexInfo {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_table_id(&self) -> bool {
        self.table_id.is_some()
    }
    #[inline]
    pub fn clear_table_id(&mut self) {
        self.table_id = ::std::option::Option::None
    }
    #[inline]
    pub fn set_table_id(&mut self, v: i64) {
        self.table_id = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_table_id(&self) -> i64 {
        match self.table_id {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_index_id(&self) -> bool {
        self.index_id.is_some()
    }
    #[inline]
    pub fn clear_index_id(&mut self) {
        self.index_id = ::std::option::Option::None
    }
    #[inline]
    pub fn set_index_id(&mut self, v: i64) {
        self.index_id = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_index_id(&self) -> i64 {
        match self.index_id {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }
    #[inline]
    pub fn set_columns(&mut self, v: ::std::vec::Vec<ColumnInfo>) {
        self.columns = v;
    }
    #[inline]
    pub fn get_columns(&self) -> &::std::vec::Vec<ColumnInfo> {
        &self.columns
    }
    #[inline]
    pub fn mut_columns(&mut self) -> &mut ::std::vec::Vec<ColumnInfo> {
        &mut self.columns
    }
    #[inline]
    pub fn take_columns(&mut self) -> ::std::vec::Vec<ColumnInfo> {
        ::std::mem::replace(&mut self.columns, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_unique(&self) -> bool {
        self.unique.is_some()
    }
    #[inline]
    pub fn clear_unique(&mut self) {
        self.unique = ::std::option::Option::None
    }
    #[inline]
    pub fn set_unique(&mut self, v: bool) {
        self.unique = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_unique(&self) -> bool {
        match self.unique {
            Some(v) => v,
            None => false,
        }
    }
}
impl ::protobuf::Clear for IndexInfo {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for IndexInfo {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static IndexInfo {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: IndexInfo = IndexInfo::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl KeyRange {
    pub fn new_() -> KeyRange {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_low(&self) -> bool {
        self.low.is_some()
    }
    #[inline]
    pub fn clear_low(&mut self) {
        self.low = ::std::option::Option::None
    }
    #[inline]
    pub fn set_low(&mut self, v: std::vec::Vec<u8>) {
        self.low = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_low(&self) -> &[u8] {
        match self.low.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }
    #[inline]
    pub fn mut_low(&mut self) -> &mut std::vec::Vec<u8> {
        if self.low.is_none() {
            self.low = ::std::option::Option::Some(::std::vec::Vec::default());
        }
        self.low.as_mut().unwrap()
    }
    #[inline]
    pub fn take_low(&mut self) -> std::vec::Vec<u8> {
        self.low.take().unwrap_or_else(::std::vec::Vec::new)
    }
    #[inline]
    pub fn has_high(&self) -> bool {
        self.high.is_some()
    }
    #[inline]
    pub fn clear_high(&mut self) {
        self.high = ::std::option::Option::None
    }
    #[inline]
    pub fn set_high(&mut self, v: std::vec::Vec<u8>) {
        self.high = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_high(&self) -> &[u8] {
        match self.high.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }
    #[inline]
    pub fn mut_high(&mut self) -> &mut std::vec::Vec<u8> {
        if self.high.is_none() {
            self.high = ::std::option::Option::Some(::std::vec::Vec::default());
        }
        self.high.as_mut().unwrap()
    }
    #[inline]
    pub fn take_high(&mut self) -> std::vec::Vec<u8> {
        self.high.take().unwrap_or_else(::std::vec::Vec::new)
    }
}
impl ::protobuf::Clear for KeyRange {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for KeyRange {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static KeyRange {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: KeyRange = KeyRange::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl AnalyzeReq {
    pub fn new_() -> AnalyzeReq {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_tp(&self) -> bool {
        self.tp.is_some()
    }
    #[inline]
    pub fn clear_tp(&mut self) {
        self.tp = ::std::option::Option::None
    }
    #[inline]
    pub fn set_tp_(&mut self, v: AnalyzeType) {
        self.tp =
            ::std::option::Option::Some(unsafe { ::std::mem::transmute::<AnalyzeType, i32>(v) });
    }
    #[inline]
    pub fn get_tp(&self) -> AnalyzeType {
        unsafe {
            ::std::mem::transmute::<i32, AnalyzeType>(match self.tp {
                Some(v) => v,
                None => 0,
            })
        }
    }
    #[inline]
    pub fn has_start_ts(&self) -> bool {
        self.start_ts.is_some()
    }
    #[inline]
    pub fn clear_start_ts(&mut self) {
        self.start_ts = ::std::option::Option::None
    }
    #[inline]
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_start_ts(&self) -> u64 {
        match self.start_ts {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }
    #[inline]
    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None
    }
    #[inline]
    pub fn set_flags(&mut self, v: u64) {
        self.flags = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_flags(&self) -> u64 {
        match self.flags {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_time_zone_offset(&self) -> bool {
        self.time_zone_offset.is_some()
    }
    #[inline]
    pub fn clear_time_zone_offset(&mut self) {
        self.time_zone_offset = ::std::option::Option::None
    }
    #[inline]
    pub fn set_time_zone_offset(&mut self, v: i64) {
        self.time_zone_offset = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_time_zone_offset(&self) -> i64 {
        match self.time_zone_offset {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_idx_req(&self) -> bool {
        self.idx_req.is_some()
    }
    #[inline]
    pub fn clear_idx_req(&mut self) {
        self.idx_req = ::std::option::Option::None
    }
    #[inline]
    pub fn set_idx_req(&mut self, v: AnalyzeIndexReq) {
        self.idx_req = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_idx_req(&self) -> &AnalyzeIndexReq {
        match self.idx_req.as_ref() {
            Some(v) => v,
            None => AnalyzeIndexReq::default_ref(),
        }
    }
    #[inline]
    pub fn mut_idx_req(&mut self) -> &mut AnalyzeIndexReq {
        if self.idx_req.is_none() {
            self.idx_req = ::std::option::Option::Some(AnalyzeIndexReq::default());
        }
        self.idx_req.as_mut().unwrap()
    }
    #[inline]
    pub fn take_idx_req(&mut self) -> AnalyzeIndexReq {
        self.idx_req.take().unwrap_or_else(AnalyzeIndexReq::default)
    }
    #[inline]
    pub fn has_col_req(&self) -> bool {
        self.col_req.is_some()
    }
    #[inline]
    pub fn clear_col_req(&mut self) {
        self.col_req = ::std::option::Option::None
    }
    #[inline]
    pub fn set_col_req(&mut self, v: AnalyzeColumnsReq) {
        self.col_req = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_col_req(&self) -> &AnalyzeColumnsReq {
        match self.col_req.as_ref() {
            Some(v) => v,
            None => AnalyzeColumnsReq::default_ref(),
        }
    }
    #[inline]
    pub fn mut_col_req(&mut self) -> &mut AnalyzeColumnsReq {
        if self.col_req.is_none() {
            self.col_req = ::std::option::Option::Some(AnalyzeColumnsReq::default());
        }
        self.col_req.as_mut().unwrap()
    }
    #[inline]
    pub fn take_col_req(&mut self) -> AnalyzeColumnsReq {
        self.col_req
            .take()
            .unwrap_or_else(AnalyzeColumnsReq::default)
    }
}
impl ::protobuf::Clear for AnalyzeReq {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for AnalyzeReq {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static AnalyzeReq {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: AnalyzeReq = AnalyzeReq::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl AnalyzeIndexReq {
    pub fn new_() -> AnalyzeIndexReq {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_bucket_size(&self) -> bool {
        self.bucket_size.is_some()
    }
    #[inline]
    pub fn clear_bucket_size(&mut self) {
        self.bucket_size = ::std::option::Option::None
    }
    #[inline]
    pub fn set_bucket_size(&mut self, v: i64) {
        self.bucket_size = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_bucket_size(&self) -> i64 {
        match self.bucket_size {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_num_columns(&self) -> bool {
        self.num_columns.is_some()
    }
    #[inline]
    pub fn clear_num_columns(&mut self) {
        self.num_columns = ::std::option::Option::None
    }
    #[inline]
    pub fn set_num_columns(&mut self, v: i32) {
        self.num_columns = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_num_columns(&self) -> i32 {
        match self.num_columns {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_cmsketch_depth(&self) -> bool {
        self.cmsketch_depth.is_some()
    }
    #[inline]
    pub fn clear_cmsketch_depth(&mut self) {
        self.cmsketch_depth = ::std::option::Option::None
    }
    #[inline]
    pub fn set_cmsketch_depth(&mut self, v: i32) {
        self.cmsketch_depth = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_cmsketch_depth(&self) -> i32 {
        match self.cmsketch_depth {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_cmsketch_width(&self) -> bool {
        self.cmsketch_width.is_some()
    }
    #[inline]
    pub fn clear_cmsketch_width(&mut self) {
        self.cmsketch_width = ::std::option::Option::None
    }
    #[inline]
    pub fn set_cmsketch_width(&mut self, v: i32) {
        self.cmsketch_width = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_cmsketch_width(&self) -> i32 {
        match self.cmsketch_width {
            Some(v) => v,
            None => 0,
        }
    }
}
impl ::protobuf::Clear for AnalyzeIndexReq {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for AnalyzeIndexReq {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static AnalyzeIndexReq {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: AnalyzeIndexReq = AnalyzeIndexReq::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl AnalyzeColumnsReq {
    pub fn new_() -> AnalyzeColumnsReq {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_bucket_size(&self) -> bool {
        self.bucket_size.is_some()
    }
    #[inline]
    pub fn clear_bucket_size(&mut self) {
        self.bucket_size = ::std::option::Option::None
    }
    #[inline]
    pub fn set_bucket_size(&mut self, v: i64) {
        self.bucket_size = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_bucket_size(&self) -> i64 {
        match self.bucket_size {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_sample_size(&self) -> bool {
        self.sample_size.is_some()
    }
    #[inline]
    pub fn clear_sample_size(&mut self) {
        self.sample_size = ::std::option::Option::None
    }
    #[inline]
    pub fn set_sample_size(&mut self, v: i64) {
        self.sample_size = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_sample_size(&self) -> i64 {
        match self.sample_size {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_sketch_size(&self) -> bool {
        self.sketch_size.is_some()
    }
    #[inline]
    pub fn clear_sketch_size(&mut self) {
        self.sketch_size = ::std::option::Option::None
    }
    #[inline]
    pub fn set_sketch_size(&mut self, v: i64) {
        self.sketch_size = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_sketch_size(&self) -> i64 {
        match self.sketch_size {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn clear_columns_info(&mut self) {
        self.columns_info.clear();
    }
    #[inline]
    pub fn set_columns_info(&mut self, v: ::std::vec::Vec<ColumnInfo>) {
        self.columns_info = v;
    }
    #[inline]
    pub fn get_columns_info(&self) -> &::std::vec::Vec<ColumnInfo> {
        &self.columns_info
    }
    #[inline]
    pub fn mut_columns_info(&mut self) -> &mut ::std::vec::Vec<ColumnInfo> {
        &mut self.columns_info
    }
    #[inline]
    pub fn take_columns_info(&mut self) -> ::std::vec::Vec<ColumnInfo> {
        ::std::mem::replace(&mut self.columns_info, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_cmsketch_depth(&self) -> bool {
        self.cmsketch_depth.is_some()
    }
    #[inline]
    pub fn clear_cmsketch_depth(&mut self) {
        self.cmsketch_depth = ::std::option::Option::None
    }
    #[inline]
    pub fn set_cmsketch_depth(&mut self, v: i32) {
        self.cmsketch_depth = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_cmsketch_depth(&self) -> i32 {
        match self.cmsketch_depth {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_cmsketch_width(&self) -> bool {
        self.cmsketch_width.is_some()
    }
    #[inline]
    pub fn clear_cmsketch_width(&mut self) {
        self.cmsketch_width = ::std::option::Option::None
    }
    #[inline]
    pub fn set_cmsketch_width(&mut self, v: i32) {
        self.cmsketch_width = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_cmsketch_width(&self) -> i32 {
        match self.cmsketch_width {
            Some(v) => v,
            None => 0,
        }
    }
}
impl ::protobuf::Clear for AnalyzeColumnsReq {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for AnalyzeColumnsReq {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static AnalyzeColumnsReq {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: AnalyzeColumnsReq = AnalyzeColumnsReq::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl AnalyzeColumnsResp {
    pub fn new_() -> AnalyzeColumnsResp {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_collectors(&mut self) {
        self.collectors.clear();
    }
    #[inline]
    pub fn set_collectors(&mut self, v: ::std::vec::Vec<SampleCollector>) {
        self.collectors = v;
    }
    #[inline]
    pub fn get_collectors(&self) -> &::std::vec::Vec<SampleCollector> {
        &self.collectors
    }
    #[inline]
    pub fn mut_collectors(&mut self) -> &mut ::std::vec::Vec<SampleCollector> {
        &mut self.collectors
    }
    #[inline]
    pub fn take_collectors(&mut self) -> ::std::vec::Vec<SampleCollector> {
        ::std::mem::replace(&mut self.collectors, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_pk_hist(&self) -> bool {
        self.pk_hist.is_some()
    }
    #[inline]
    pub fn clear_pk_hist(&mut self) {
        self.pk_hist = ::std::option::Option::None
    }
    #[inline]
    pub fn set_pk_hist(&mut self, v: Histogram) {
        self.pk_hist = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_pk_hist(&self) -> &Histogram {
        match self.pk_hist.as_ref() {
            Some(v) => v,
            None => Histogram::default_ref(),
        }
    }
    #[inline]
    pub fn mut_pk_hist(&mut self) -> &mut Histogram {
        if self.pk_hist.is_none() {
            self.pk_hist = ::std::option::Option::Some(Histogram::default());
        }
        self.pk_hist.as_mut().unwrap()
    }
    #[inline]
    pub fn take_pk_hist(&mut self) -> Histogram {
        self.pk_hist.take().unwrap_or_else(Histogram::default)
    }
}
impl ::protobuf::Clear for AnalyzeColumnsResp {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for AnalyzeColumnsResp {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static AnalyzeColumnsResp {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: AnalyzeColumnsResp = AnalyzeColumnsResp::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl AnalyzeIndexResp {
    pub fn new_() -> AnalyzeIndexResp {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_hist(&self) -> bool {
        self.hist.is_some()
    }
    #[inline]
    pub fn clear_hist(&mut self) {
        self.hist = ::std::option::Option::None
    }
    #[inline]
    pub fn set_hist(&mut self, v: Histogram) {
        self.hist = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_hist(&self) -> &Histogram {
        match self.hist.as_ref() {
            Some(v) => v,
            None => Histogram::default_ref(),
        }
    }
    #[inline]
    pub fn mut_hist(&mut self) -> &mut Histogram {
        if self.hist.is_none() {
            self.hist = ::std::option::Option::Some(Histogram::default());
        }
        self.hist.as_mut().unwrap()
    }
    #[inline]
    pub fn take_hist(&mut self) -> Histogram {
        self.hist.take().unwrap_or_else(Histogram::default)
    }
    #[inline]
    pub fn has_cms(&self) -> bool {
        self.cms.is_some()
    }
    #[inline]
    pub fn clear_cms(&mut self) {
        self.cms = ::std::option::Option::None
    }
    #[inline]
    pub fn set_cms(&mut self, v: CmSketch) {
        self.cms = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_cms(&self) -> &CmSketch {
        match self.cms.as_ref() {
            Some(v) => v,
            None => CmSketch::default_ref(),
        }
    }
    #[inline]
    pub fn mut_cms(&mut self) -> &mut CmSketch {
        if self.cms.is_none() {
            self.cms = ::std::option::Option::Some(CmSketch::default());
        }
        self.cms.as_mut().unwrap()
    }
    #[inline]
    pub fn take_cms(&mut self) -> CmSketch {
        self.cms.take().unwrap_or_else(CmSketch::default)
    }
}
impl ::protobuf::Clear for AnalyzeIndexResp {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for AnalyzeIndexResp {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static AnalyzeIndexResp {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: AnalyzeIndexResp = AnalyzeIndexResp::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Bucket {
    pub fn new_() -> Bucket {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }
    #[inline]
    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None
    }
    #[inline]
    pub fn set_count(&mut self, v: i64) {
        self.count = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_count(&self) -> i64 {
        match self.count {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_lower_bound(&self) -> bool {
        self.lower_bound.is_some()
    }
    #[inline]
    pub fn clear_lower_bound(&mut self) {
        self.lower_bound = ::std::option::Option::None
    }
    #[inline]
    pub fn set_lower_bound(&mut self, v: std::vec::Vec<u8>) {
        self.lower_bound = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_lower_bound(&self) -> &[u8] {
        match self.lower_bound.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }
    #[inline]
    pub fn mut_lower_bound(&mut self) -> &mut std::vec::Vec<u8> {
        if self.lower_bound.is_none() {
            self.lower_bound = ::std::option::Option::Some(::std::vec::Vec::default());
        }
        self.lower_bound.as_mut().unwrap()
    }
    #[inline]
    pub fn take_lower_bound(&mut self) -> std::vec::Vec<u8> {
        self.lower_bound.take().unwrap_or_else(::std::vec::Vec::new)
    }
    #[inline]
    pub fn has_upper_bound(&self) -> bool {
        self.upper_bound.is_some()
    }
    #[inline]
    pub fn clear_upper_bound(&mut self) {
        self.upper_bound = ::std::option::Option::None
    }
    #[inline]
    pub fn set_upper_bound(&mut self, v: std::vec::Vec<u8>) {
        self.upper_bound = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_upper_bound(&self) -> &[u8] {
        match self.upper_bound.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }
    #[inline]
    pub fn mut_upper_bound(&mut self) -> &mut std::vec::Vec<u8> {
        if self.upper_bound.is_none() {
            self.upper_bound = ::std::option::Option::Some(::std::vec::Vec::default());
        }
        self.upper_bound.as_mut().unwrap()
    }
    #[inline]
    pub fn take_upper_bound(&mut self) -> std::vec::Vec<u8> {
        self.upper_bound.take().unwrap_or_else(::std::vec::Vec::new)
    }
    #[inline]
    pub fn has_repeats(&self) -> bool {
        self.repeats.is_some()
    }
    #[inline]
    pub fn clear_repeats(&mut self) {
        self.repeats = ::std::option::Option::None
    }
    #[inline]
    pub fn set_repeats(&mut self, v: i64) {
        self.repeats = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_repeats(&self) -> i64 {
        match self.repeats {
            Some(v) => v,
            None => 0,
        }
    }
}
impl ::protobuf::Clear for Bucket {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Bucket {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static Bucket {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Bucket = Bucket::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Histogram {
    pub fn new_() -> Histogram {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_ndv(&self) -> bool {
        self.ndv.is_some()
    }
    #[inline]
    pub fn clear_ndv(&mut self) {
        self.ndv = ::std::option::Option::None
    }
    #[inline]
    pub fn set_ndv(&mut self, v: i64) {
        self.ndv = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_ndv(&self) -> i64 {
        match self.ndv {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn clear_buckets(&mut self) {
        self.buckets.clear();
    }
    #[inline]
    pub fn set_buckets(&mut self, v: ::std::vec::Vec<Bucket>) {
        self.buckets = v;
    }
    #[inline]
    pub fn get_buckets(&self) -> &::std::vec::Vec<Bucket> {
        &self.buckets
    }
    #[inline]
    pub fn mut_buckets(&mut self) -> &mut ::std::vec::Vec<Bucket> {
        &mut self.buckets
    }
    #[inline]
    pub fn take_buckets(&mut self) -> ::std::vec::Vec<Bucket> {
        ::std::mem::replace(&mut self.buckets, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for Histogram {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Histogram {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static Histogram {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Histogram = Histogram::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl FmSketch {
    pub fn new_() -> FmSketch {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_mask(&self) -> bool {
        self.mask.is_some()
    }
    #[inline]
    pub fn clear_mask(&mut self) {
        self.mask = ::std::option::Option::None
    }
    #[inline]
    pub fn set_mask(&mut self, v: u64) {
        self.mask = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_mask(&self) -> u64 {
        match self.mask {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn clear_hashset(&mut self) {
        self.hashset.clear();
    }
    #[inline]
    pub fn set_hashset(&mut self, v: ::std::vec::Vec<u64>) {
        self.hashset = v;
    }
    #[inline]
    pub fn get_hashset(&self) -> &::std::vec::Vec<u64> {
        &self.hashset
    }
    #[inline]
    pub fn mut_hashset(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.hashset
    }
    #[inline]
    pub fn take_hashset(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.hashset, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for FmSketch {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for FmSketch {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static FmSketch {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: FmSketch = FmSketch::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl SampleCollector {
    pub fn new_() -> SampleCollector {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_samples(&mut self) {
        self.samples.clear();
    }
    #[inline]
    pub fn set_samples(&mut self, v: ::std::vec::Vec<std::vec::Vec<u8>>) {
        self.samples = v;
    }
    #[inline]
    pub fn get_samples(&self) -> &::std::vec::Vec<std::vec::Vec<u8>> {
        &self.samples
    }
    #[inline]
    pub fn mut_samples(&mut self) -> &mut ::std::vec::Vec<std::vec::Vec<u8>> {
        &mut self.samples
    }
    #[inline]
    pub fn take_samples(&mut self) -> ::std::vec::Vec<std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.samples, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_null_count(&self) -> bool {
        self.null_count.is_some()
    }
    #[inline]
    pub fn clear_null_count(&mut self) {
        self.null_count = ::std::option::Option::None
    }
    #[inline]
    pub fn set_null_count(&mut self, v: i64) {
        self.null_count = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_null_count(&self) -> i64 {
        match self.null_count {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }
    #[inline]
    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None
    }
    #[inline]
    pub fn set_count(&mut self, v: i64) {
        self.count = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_count(&self) -> i64 {
        match self.count {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_fm_sketch(&self) -> bool {
        self.fm_sketch.is_some()
    }
    #[inline]
    pub fn clear_fm_sketch(&mut self) {
        self.fm_sketch = ::std::option::Option::None
    }
    #[inline]
    pub fn set_fm_sketch(&mut self, v: FmSketch) {
        self.fm_sketch = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_fm_sketch(&self) -> &FmSketch {
        match self.fm_sketch.as_ref() {
            Some(v) => v,
            None => FmSketch::default_ref(),
        }
    }
    #[inline]
    pub fn mut_fm_sketch(&mut self) -> &mut FmSketch {
        if self.fm_sketch.is_none() {
            self.fm_sketch = ::std::option::Option::Some(FmSketch::default());
        }
        self.fm_sketch.as_mut().unwrap()
    }
    #[inline]
    pub fn take_fm_sketch(&mut self) -> FmSketch {
        self.fm_sketch.take().unwrap_or_else(FmSketch::default)
    }
    #[inline]
    pub fn has_cm_sketch(&self) -> bool {
        self.cm_sketch.is_some()
    }
    #[inline]
    pub fn clear_cm_sketch(&mut self) {
        self.cm_sketch = ::std::option::Option::None
    }
    #[inline]
    pub fn set_cm_sketch(&mut self, v: CmSketch) {
        self.cm_sketch = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_cm_sketch(&self) -> &CmSketch {
        match self.cm_sketch.as_ref() {
            Some(v) => v,
            None => CmSketch::default_ref(),
        }
    }
    #[inline]
    pub fn mut_cm_sketch(&mut self) -> &mut CmSketch {
        if self.cm_sketch.is_none() {
            self.cm_sketch = ::std::option::Option::Some(CmSketch::default());
        }
        self.cm_sketch.as_mut().unwrap()
    }
    #[inline]
    pub fn take_cm_sketch(&mut self) -> CmSketch {
        self.cm_sketch.take().unwrap_or_else(CmSketch::default)
    }
    #[inline]
    pub fn has_total_size(&self) -> bool {
        self.total_size.is_some()
    }
    #[inline]
    pub fn clear_total_size(&mut self) {
        self.total_size = ::std::option::Option::None
    }
    #[inline]
    pub fn set_total_size(&mut self, v: i64) {
        self.total_size = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_total_size(&self) -> i64 {
        match self.total_size {
            Some(v) => v,
            None => 0,
        }
    }
}
impl ::protobuf::Clear for SampleCollector {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SampleCollector {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static SampleCollector {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SampleCollector = SampleCollector::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl CmSketchRow {
    pub fn new_() -> CmSketchRow {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_counters(&mut self) {
        self.counters.clear();
    }
    #[inline]
    pub fn set_counters(&mut self, v: ::std::vec::Vec<u32>) {
        self.counters = v;
    }
    #[inline]
    pub fn get_counters(&self) -> &::std::vec::Vec<u32> {
        &self.counters
    }
    #[inline]
    pub fn mut_counters(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.counters
    }
    #[inline]
    pub fn take_counters(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.counters, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for CmSketchRow {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CmSketchRow {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static CmSketchRow {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CmSketchRow = CmSketchRow::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl CmSketchTopN {
    pub fn new_() -> CmSketchTopN {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }
    #[inline]
    pub fn clear_data(&mut self) {
        self.data = ::std::option::Option::None
    }
    #[inline]
    pub fn set_data(&mut self, v: std::vec::Vec<u8>) {
        self.data = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }
    #[inline]
    pub fn mut_data(&mut self) -> &mut std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data = ::std::option::Option::Some(::std::vec::Vec::default());
        }
        self.data.as_mut().unwrap()
    }
    #[inline]
    pub fn take_data(&mut self) -> std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(::std::vec::Vec::new)
    }
    #[inline]
    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }
    #[inline]
    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None
    }
    #[inline]
    pub fn set_count(&mut self, v: u64) {
        self.count = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_count(&self) -> u64 {
        match self.count {
            Some(v) => v,
            None => 0,
        }
    }
}
impl ::protobuf::Clear for CmSketchTopN {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CmSketchTopN {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static CmSketchTopN {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CmSketchTopN = CmSketchTopN::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl CmSketch {
    pub fn new_() -> CmSketch {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }
    #[inline]
    pub fn set_rows(&mut self, v: ::std::vec::Vec<CmSketchRow>) {
        self.rows = v;
    }
    #[inline]
    pub fn get_rows(&self) -> &::std::vec::Vec<CmSketchRow> {
        &self.rows
    }
    #[inline]
    pub fn mut_rows(&mut self) -> &mut ::std::vec::Vec<CmSketchRow> {
        &mut self.rows
    }
    #[inline]
    pub fn take_rows(&mut self) -> ::std::vec::Vec<CmSketchRow> {
        ::std::mem::replace(&mut self.rows, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_top_n(&mut self) {
        self.top_n.clear();
    }
    #[inline]
    pub fn set_top_n(&mut self, v: ::std::vec::Vec<CmSketchTopN>) {
        self.top_n = v;
    }
    #[inline]
    pub fn get_top_n(&self) -> &::std::vec::Vec<CmSketchTopN> {
        &self.top_n
    }
    #[inline]
    pub fn mut_top_n(&mut self) -> &mut ::std::vec::Vec<CmSketchTopN> {
        &mut self.top_n
    }
    #[inline]
    pub fn take_top_n(&mut self) -> ::std::vec::Vec<CmSketchTopN> {
        ::std::mem::replace(&mut self.top_n, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_default_value(&self) -> bool {
        self.default_value.is_some()
    }
    #[inline]
    pub fn clear_default_value(&mut self) {
        self.default_value = ::std::option::Option::None
    }
    #[inline]
    pub fn set_default_value(&mut self, v: u64) {
        self.default_value = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_default_value(&self) -> u64 {
        match self.default_value {
            Some(v) => v,
            None => 0,
        }
    }
}
impl ::protobuf::Clear for CmSketch {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CmSketch {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static CmSketch {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CmSketch = CmSketch::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl AnalyzeType {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [AnalyzeType] = &[AnalyzeType::TypeIndex, AnalyzeType::TypeColumn];
        VALUES
    }
}
impl ChecksumRequest {
    pub fn new_() -> ChecksumRequest {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_start_ts(&self) -> bool {
        self.start_ts.is_some()
    }
    #[inline]
    pub fn clear_start_ts(&mut self) {
        self.start_ts = ::std::option::Option::None
    }
    #[inline]
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_start_ts(&self) -> u64 {
        match self.start_ts {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_scan_on(&self) -> bool {
        self.scan_on.is_some()
    }
    #[inline]
    pub fn clear_scan_on(&mut self) {
        self.scan_on = ::std::option::Option::None
    }
    #[inline]
    pub fn set_scan_on_(&mut self, v: ChecksumScanOn) {
        self.scan_on =
            ::std::option::Option::Some(unsafe { ::std::mem::transmute::<ChecksumScanOn, i32>(v) });
    }
    #[inline]
    pub fn get_scan_on(&self) -> ChecksumScanOn {
        unsafe {
            ::std::mem::transmute::<i32, ChecksumScanOn>(match self.scan_on {
                Some(v) => v,
                None => 0,
            })
        }
    }
    #[inline]
    pub fn has_algorithm(&self) -> bool {
        self.algorithm.is_some()
    }
    #[inline]
    pub fn clear_algorithm(&mut self) {
        self.algorithm = ::std::option::Option::None
    }
    #[inline]
    pub fn set_algorithm_(&mut self, v: ChecksumAlgorithm) {
        self.algorithm = ::std::option::Option::Some(unsafe {
            ::std::mem::transmute::<ChecksumAlgorithm, i32>(v)
        });
    }
    #[inline]
    pub fn get_algorithm(&self) -> ChecksumAlgorithm {
        unsafe {
            ::std::mem::transmute::<i32, ChecksumAlgorithm>(match self.algorithm {
                Some(v) => v,
                None => 0,
            })
        }
    }
}
impl ::protobuf::Clear for ChecksumRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ChecksumRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static ChecksumRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ChecksumRequest = ChecksumRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ChecksumResponse {
    pub fn new_() -> ChecksumResponse {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_checksum(&self) -> bool {
        self.checksum.is_some()
    }
    #[inline]
    pub fn clear_checksum(&mut self) {
        self.checksum = ::std::option::Option::None
    }
    #[inline]
    pub fn set_checksum(&mut self, v: u64) {
        self.checksum = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_checksum(&self) -> u64 {
        match self.checksum {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_total_kvs(&self) -> bool {
        self.total_kvs.is_some()
    }
    #[inline]
    pub fn clear_total_kvs(&mut self) {
        self.total_kvs = ::std::option::Option::None
    }
    #[inline]
    pub fn set_total_kvs(&mut self, v: u64) {
        self.total_kvs = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_total_kvs(&self) -> u64 {
        match self.total_kvs {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_total_bytes(&self) -> bool {
        self.total_bytes.is_some()
    }
    #[inline]
    pub fn clear_total_bytes(&mut self) {
        self.total_bytes = ::std::option::Option::None
    }
    #[inline]
    pub fn set_total_bytes(&mut self, v: u64) {
        self.total_bytes = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_total_bytes(&self) -> u64 {
        match self.total_bytes {
            Some(v) => v,
            None => 0,
        }
    }
}
impl ::protobuf::Clear for ChecksumResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ChecksumResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static ChecksumResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ChecksumResponse = ChecksumResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ChecksumScanOn {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [ChecksumScanOn] = &[ChecksumScanOn::Table, ChecksumScanOn::Index];
        VALUES
    }
}
impl ChecksumAlgorithm {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [ChecksumAlgorithm] = &[ChecksumAlgorithm::Crc64Xor];
        VALUES
    }
}
impl FieldType {
    pub fn new_() -> FieldType {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_tp(&self) -> bool {
        self.tp.is_some()
    }
    #[inline]
    pub fn clear_tp(&mut self) {
        self.tp = ::std::option::Option::None
    }
    #[inline]
    pub fn set_tp(&mut self, v: i32) {
        self.tp = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_tp(&self) -> i32 {
        match self.tp {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_flag(&self) -> bool {
        self.flag.is_some()
    }
    #[inline]
    pub fn clear_flag(&mut self) {
        self.flag = ::std::option::Option::None
    }
    #[inline]
    pub fn set_flag(&mut self, v: u32) {
        self.flag = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_flag(&self) -> u32 {
        match self.flag {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_flen(&self) -> bool {
        self.flen.is_some()
    }
    #[inline]
    pub fn clear_flen(&mut self) {
        self.flen = ::std::option::Option::None
    }
    #[inline]
    pub fn set_flen(&mut self, v: i32) {
        self.flen = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_flen(&self) -> i32 {
        match self.flen {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_decimal(&self) -> bool {
        self.decimal.is_some()
    }
    #[inline]
    pub fn clear_decimal(&mut self) {
        self.decimal = ::std::option::Option::None
    }
    #[inline]
    pub fn set_decimal(&mut self, v: i32) {
        self.decimal = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_decimal(&self) -> i32 {
        match self.decimal {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_collate(&self) -> bool {
        self.collate.is_some()
    }
    #[inline]
    pub fn clear_collate(&mut self) {
        self.collate = ::std::option::Option::None
    }
    #[inline]
    pub fn set_collate(&mut self, v: i32) {
        self.collate = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_collate(&self) -> i32 {
        match self.collate {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_charset(&self) -> bool {
        self.charset.is_some()
    }
    #[inline]
    pub fn clear_charset(&mut self) {
        self.charset = ::std::option::Option::None
    }
    #[inline]
    pub fn set_charset(&mut self, v: std::string::String) {
        self.charset = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_charset(&self) -> &str {
        match self.charset.as_ref() {
            Some(v) => v,
            None => "",
        }
    }
    #[inline]
    pub fn mut_charset(&mut self) -> &mut std::string::String {
        if self.charset.is_none() {
            self.charset = ::std::option::Option::Some(std::string::String::default());
        }
        self.charset.as_mut().unwrap()
    }
    #[inline]
    pub fn take_charset(&mut self) -> std::string::String {
        self.charset
            .take()
            .unwrap_or_else(::std::string::String::new)
    }
}
impl ::protobuf::Clear for FieldType {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for FieldType {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static FieldType {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: FieldType = FieldType::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Expr {
    pub fn new_() -> Expr {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_tp(&self) -> bool {
        self.tp.is_some()
    }
    #[inline]
    pub fn clear_tp(&mut self) {
        self.tp = ::std::option::Option::None
    }
    #[inline]
    pub fn set_tp_(&mut self, v: ExprType) {
        self.tp = ::std::option::Option::Some(unsafe { ::std::mem::transmute::<ExprType, i32>(v) });
    }
    #[inline]
    pub fn get_tp(&self) -> ExprType {
        unsafe {
            ::std::mem::transmute::<i32, ExprType>(match self.tp {
                Some(v) => v,
                None => 0,
            })
        }
    }
    #[inline]
    pub fn has_val(&self) -> bool {
        self.val.is_some()
    }
    #[inline]
    pub fn clear_val(&mut self) {
        self.val = ::std::option::Option::None
    }
    #[inline]
    pub fn set_val(&mut self, v: std::vec::Vec<u8>) {
        self.val = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_val(&self) -> &[u8] {
        match self.val.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }
    #[inline]
    pub fn mut_val(&mut self) -> &mut std::vec::Vec<u8> {
        if self.val.is_none() {
            self.val = ::std::option::Option::Some(::std::vec::Vec::default());
        }
        self.val.as_mut().unwrap()
    }
    #[inline]
    pub fn take_val(&mut self) -> std::vec::Vec<u8> {
        self.val.take().unwrap_or_else(::std::vec::Vec::new)
    }
    #[inline]
    pub fn clear_children(&mut self) {
        self.children.clear();
    }
    #[inline]
    pub fn set_children(&mut self, v: ::std::vec::Vec<Expr>) {
        self.children = v;
    }
    #[inline]
    pub fn get_children(&self) -> &::std::vec::Vec<Expr> {
        &self.children
    }
    #[inline]
    pub fn mut_children(&mut self) -> &mut ::std::vec::Vec<Expr> {
        &mut self.children
    }
    #[inline]
    pub fn take_children(&mut self) -> ::std::vec::Vec<Expr> {
        ::std::mem::replace(&mut self.children, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_sig(&self) -> bool {
        self.sig.is_some()
    }
    #[inline]
    pub fn clear_sig(&mut self) {
        self.sig = ::std::option::Option::None
    }
    #[inline]
    pub fn set_sig_(&mut self, v: ScalarFuncSig) {
        self.sig =
            ::std::option::Option::Some(unsafe { ::std::mem::transmute::<ScalarFuncSig, i32>(v) });
    }
    #[inline]
    pub fn get_sig(&self) -> ScalarFuncSig {
        unsafe {
            ::std::mem::transmute::<i32, ScalarFuncSig>(match self.sig {
                Some(v) => v,
                None => 0,
            })
        }
    }
    #[inline]
    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }
    #[inline]
    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None
    }
    #[inline]
    pub fn set_field_type(&mut self, v: FieldType) {
        self.field_type = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_field_type(&self) -> &FieldType {
        match self.field_type.as_ref() {
            Some(v) => v,
            None => FieldType::default_ref(),
        }
    }
    #[inline]
    pub fn mut_field_type(&mut self) -> &mut FieldType {
        if self.field_type.is_none() {
            self.field_type = ::std::option::Option::Some(FieldType::default());
        }
        self.field_type.as_mut().unwrap()
    }
    #[inline]
    pub fn take_field_type(&mut self) -> FieldType {
        self.field_type.take().unwrap_or_else(FieldType::default)
    }
}
impl ::protobuf::Clear for Expr {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Expr {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static Expr {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Expr = Expr::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ByItem {
    pub fn new_() -> ByItem {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_expr(&self) -> bool {
        self.expr.is_some()
    }
    #[inline]
    pub fn clear_expr(&mut self) {
        self.expr = ::std::option::Option::None
    }
    #[inline]
    pub fn set_expr(&mut self, v: Expr) {
        self.expr = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_expr(&self) -> &Expr {
        match self.expr.as_ref() {
            Some(v) => v,
            None => Expr::default_ref(),
        }
    }
    #[inline]
    pub fn mut_expr(&mut self) -> &mut Expr {
        if self.expr.is_none() {
            self.expr = ::std::option::Option::Some(Expr::default());
        }
        self.expr.as_mut().unwrap()
    }
    #[inline]
    pub fn take_expr(&mut self) -> Expr {
        self.expr.take().unwrap_or_else(Expr::default)
    }
    #[inline]
    pub fn has_desc(&self) -> bool {
        self.desc.is_some()
    }
    #[inline]
    pub fn clear_desc(&mut self) {
        self.desc = ::std::option::Option::None
    }
    #[inline]
    pub fn set_desc(&mut self, v: bool) {
        self.desc = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_desc(&self) -> bool {
        match self.desc {
            Some(v) => v,
            None => false,
        }
    }
}
impl ::protobuf::Clear for ByItem {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ByItem {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static ByItem {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ByItem = ByItem::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ExprType {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [ExprType] = &[
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
            ExprType::MysqlJson,
            ExprType::ValueList,
            ExprType::ColumnRef,
            ExprType::Count,
            ExprType::Sum,
            ExprType::Avg,
            ExprType::Min,
            ExprType::Max,
            ExprType::First,
            ExprType::GroupConcat,
            ExprType::AggBitAnd,
            ExprType::AggBitOr,
            ExprType::AggBitXor,
            ExprType::Std,
            ExprType::Stddev,
            ExprType::StddevPop,
            ExprType::StddevSamp,
            ExprType::VarPop,
            ExprType::VarSamp,
            ExprType::Variance,
            ExprType::JsonArrayAgg,
            ExprType::JsonObjectAgg,
            ExprType::ScalarFunc,
        ];
        VALUES
    }
}
impl ScalarFuncSig {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [ScalarFuncSig] = &[
            ScalarFuncSig::CastIntAsInt,
            ScalarFuncSig::CastIntAsReal,
            ScalarFuncSig::CastIntAsString,
            ScalarFuncSig::CastIntAsDecimal,
            ScalarFuncSig::CastIntAsTime,
            ScalarFuncSig::CastIntAsDuration,
            ScalarFuncSig::CastIntAsJson,
            ScalarFuncSig::CastRealAsInt,
            ScalarFuncSig::CastRealAsReal,
            ScalarFuncSig::CastRealAsString,
            ScalarFuncSig::CastRealAsDecimal,
            ScalarFuncSig::CastRealAsTime,
            ScalarFuncSig::CastRealAsDuration,
            ScalarFuncSig::CastRealAsJson,
            ScalarFuncSig::CastDecimalAsInt,
            ScalarFuncSig::CastDecimalAsReal,
            ScalarFuncSig::CastDecimalAsString,
            ScalarFuncSig::CastDecimalAsDecimal,
            ScalarFuncSig::CastDecimalAsTime,
            ScalarFuncSig::CastDecimalAsDuration,
            ScalarFuncSig::CastDecimalAsJson,
            ScalarFuncSig::CastStringAsInt,
            ScalarFuncSig::CastStringAsReal,
            ScalarFuncSig::CastStringAsString,
            ScalarFuncSig::CastStringAsDecimal,
            ScalarFuncSig::CastStringAsTime,
            ScalarFuncSig::CastStringAsDuration,
            ScalarFuncSig::CastStringAsJson,
            ScalarFuncSig::CastTimeAsInt,
            ScalarFuncSig::CastTimeAsReal,
            ScalarFuncSig::CastTimeAsString,
            ScalarFuncSig::CastTimeAsDecimal,
            ScalarFuncSig::CastTimeAsTime,
            ScalarFuncSig::CastTimeAsDuration,
            ScalarFuncSig::CastTimeAsJson,
            ScalarFuncSig::CastDurationAsInt,
            ScalarFuncSig::CastDurationAsReal,
            ScalarFuncSig::CastDurationAsString,
            ScalarFuncSig::CastDurationAsDecimal,
            ScalarFuncSig::CastDurationAsTime,
            ScalarFuncSig::CastDurationAsDuration,
            ScalarFuncSig::CastDurationAsJson,
            ScalarFuncSig::CastJsonAsInt,
            ScalarFuncSig::CastJsonAsReal,
            ScalarFuncSig::CastJsonAsString,
            ScalarFuncSig::CastJsonAsDecimal,
            ScalarFuncSig::CastJsonAsTime,
            ScalarFuncSig::CastJsonAsDuration,
            ScalarFuncSig::CastJsonAsJson,
            ScalarFuncSig::CoalesceInt,
            ScalarFuncSig::CoalesceReal,
            ScalarFuncSig::CoalesceDecimal,
            ScalarFuncSig::CoalesceString,
            ScalarFuncSig::CoalesceTime,
            ScalarFuncSig::CoalesceDuration,
            ScalarFuncSig::CoalesceJson,
            ScalarFuncSig::LtInt,
            ScalarFuncSig::LtReal,
            ScalarFuncSig::LtDecimal,
            ScalarFuncSig::LtString,
            ScalarFuncSig::LtTime,
            ScalarFuncSig::LtDuration,
            ScalarFuncSig::LtJson,
            ScalarFuncSig::LeInt,
            ScalarFuncSig::LeReal,
            ScalarFuncSig::LeDecimal,
            ScalarFuncSig::LeString,
            ScalarFuncSig::LeTime,
            ScalarFuncSig::LeDuration,
            ScalarFuncSig::LeJson,
            ScalarFuncSig::GtInt,
            ScalarFuncSig::GtReal,
            ScalarFuncSig::GtDecimal,
            ScalarFuncSig::GtString,
            ScalarFuncSig::GtTime,
            ScalarFuncSig::GtDuration,
            ScalarFuncSig::GtJson,
            ScalarFuncSig::GreatestInt,
            ScalarFuncSig::GreatestReal,
            ScalarFuncSig::GreatestDecimal,
            ScalarFuncSig::GreatestString,
            ScalarFuncSig::GreatestTime,
            ScalarFuncSig::LeastInt,
            ScalarFuncSig::LeastReal,
            ScalarFuncSig::LeastDecimal,
            ScalarFuncSig::LeastString,
            ScalarFuncSig::LeastTime,
            ScalarFuncSig::IntervalInt,
            ScalarFuncSig::IntervalReal,
            ScalarFuncSig::GeInt,
            ScalarFuncSig::GeReal,
            ScalarFuncSig::GeDecimal,
            ScalarFuncSig::GeString,
            ScalarFuncSig::GeTime,
            ScalarFuncSig::GeDuration,
            ScalarFuncSig::GeJson,
            ScalarFuncSig::EqInt,
            ScalarFuncSig::EqReal,
            ScalarFuncSig::EqDecimal,
            ScalarFuncSig::EqString,
            ScalarFuncSig::EqTime,
            ScalarFuncSig::EqDuration,
            ScalarFuncSig::EqJson,
            ScalarFuncSig::NeInt,
            ScalarFuncSig::NeReal,
            ScalarFuncSig::NeDecimal,
            ScalarFuncSig::NeString,
            ScalarFuncSig::NeTime,
            ScalarFuncSig::NeDuration,
            ScalarFuncSig::NeJson,
            ScalarFuncSig::NullEqInt,
            ScalarFuncSig::NullEqReal,
            ScalarFuncSig::NullEqDecimal,
            ScalarFuncSig::NullEqString,
            ScalarFuncSig::NullEqTime,
            ScalarFuncSig::NullEqDuration,
            ScalarFuncSig::NullEqJson,
            ScalarFuncSig::PlusReal,
            ScalarFuncSig::PlusDecimal,
            ScalarFuncSig::PlusInt,
            ScalarFuncSig::MinusReal,
            ScalarFuncSig::MinusDecimal,
            ScalarFuncSig::MinusInt,
            ScalarFuncSig::MultiplyReal,
            ScalarFuncSig::MultiplyDecimal,
            ScalarFuncSig::MultiplyInt,
            ScalarFuncSig::DivideReal,
            ScalarFuncSig::DivideDecimal,
            ScalarFuncSig::IntDivideInt,
            ScalarFuncSig::IntDivideDecimal,
            ScalarFuncSig::ModReal,
            ScalarFuncSig::ModDecimal,
            ScalarFuncSig::ModInt,
            ScalarFuncSig::MultiplyIntUnsigned,
            ScalarFuncSig::AbsInt,
            ScalarFuncSig::AbsUInt,
            ScalarFuncSig::AbsReal,
            ScalarFuncSig::AbsDecimal,
            ScalarFuncSig::CeilIntToDec,
            ScalarFuncSig::CeilIntToInt,
            ScalarFuncSig::CeilDecToInt,
            ScalarFuncSig::CeilDecToDec,
            ScalarFuncSig::CeilReal,
            ScalarFuncSig::FloorIntToDec,
            ScalarFuncSig::FloorIntToInt,
            ScalarFuncSig::FloorDecToInt,
            ScalarFuncSig::FloorDecToDec,
            ScalarFuncSig::FloorReal,
            ScalarFuncSig::RoundReal,
            ScalarFuncSig::RoundInt,
            ScalarFuncSig::RoundDec,
            ScalarFuncSig::RoundWithFracReal,
            ScalarFuncSig::RoundWithFracInt,
            ScalarFuncSig::RoundWithFracDec,
            ScalarFuncSig::Log1Arg,
            ScalarFuncSig::Log2Args,
            ScalarFuncSig::Log2,
            ScalarFuncSig::Log10,
            ScalarFuncSig::Rand,
            ScalarFuncSig::RandWithSeed,
            ScalarFuncSig::Pow,
            ScalarFuncSig::Conv,
            ScalarFuncSig::Crc32,
            ScalarFuncSig::Sign,
            ScalarFuncSig::Sqrt,
            ScalarFuncSig::Acos,
            ScalarFuncSig::Asin,
            ScalarFuncSig::Atan1Arg,
            ScalarFuncSig::Atan2Args,
            ScalarFuncSig::Cos,
            ScalarFuncSig::Cot,
            ScalarFuncSig::Degrees,
            ScalarFuncSig::Exp,
            ScalarFuncSig::Pi,
            ScalarFuncSig::Radians,
            ScalarFuncSig::Sin,
            ScalarFuncSig::Tan,
            ScalarFuncSig::TruncateInt,
            ScalarFuncSig::TruncateReal,
            ScalarFuncSig::TruncateDecimal,
            ScalarFuncSig::LogicalAnd,
            ScalarFuncSig::LogicalOr,
            ScalarFuncSig::LogicalXor,
            ScalarFuncSig::UnaryNot,
            ScalarFuncSig::UnaryMinusInt,
            ScalarFuncSig::UnaryMinusReal,
            ScalarFuncSig::UnaryMinusDecimal,
            ScalarFuncSig::DecimalIsNull,
            ScalarFuncSig::DurationIsNull,
            ScalarFuncSig::RealIsNull,
            ScalarFuncSig::StringIsNull,
            ScalarFuncSig::TimeIsNull,
            ScalarFuncSig::IntIsNull,
            ScalarFuncSig::JsonIsNull,
            ScalarFuncSig::BitAndSig,
            ScalarFuncSig::BitOrSig,
            ScalarFuncSig::BitXorSig,
            ScalarFuncSig::BitNegSig,
            ScalarFuncSig::IntIsTrue,
            ScalarFuncSig::RealIsTrue,
            ScalarFuncSig::DecimalIsTrue,
            ScalarFuncSig::IntIsFalse,
            ScalarFuncSig::RealIsFalse,
            ScalarFuncSig::DecimalIsFalse,
            ScalarFuncSig::LeftShift,
            ScalarFuncSig::RightShift,
            ScalarFuncSig::BitCount,
            ScalarFuncSig::GetParamString,
            ScalarFuncSig::GetVar,
            ScalarFuncSig::RowSig,
            ScalarFuncSig::SetVar,
            ScalarFuncSig::ValuesDecimal,
            ScalarFuncSig::ValuesDuration,
            ScalarFuncSig::ValuesInt,
            ScalarFuncSig::ValuesJson,
            ScalarFuncSig::ValuesReal,
            ScalarFuncSig::ValuesString,
            ScalarFuncSig::ValuesTime,
            ScalarFuncSig::InInt,
            ScalarFuncSig::InReal,
            ScalarFuncSig::InDecimal,
            ScalarFuncSig::InString,
            ScalarFuncSig::InTime,
            ScalarFuncSig::InDuration,
            ScalarFuncSig::InJson,
            ScalarFuncSig::IfNullInt,
            ScalarFuncSig::IfNullReal,
            ScalarFuncSig::IfNullDecimal,
            ScalarFuncSig::IfNullString,
            ScalarFuncSig::IfNullTime,
            ScalarFuncSig::IfNullDuration,
            ScalarFuncSig::IfInt,
            ScalarFuncSig::IfReal,
            ScalarFuncSig::IfDecimal,
            ScalarFuncSig::IfString,
            ScalarFuncSig::IfTime,
            ScalarFuncSig::IfDuration,
            ScalarFuncSig::IfNullJson,
            ScalarFuncSig::IfJson,
            ScalarFuncSig::CaseWhenInt,
            ScalarFuncSig::CaseWhenReal,
            ScalarFuncSig::CaseWhenDecimal,
            ScalarFuncSig::CaseWhenString,
            ScalarFuncSig::CaseWhenTime,
            ScalarFuncSig::CaseWhenDuration,
            ScalarFuncSig::CaseWhenJson,
            ScalarFuncSig::AesDecrypt,
            ScalarFuncSig::AesEncrypt,
            ScalarFuncSig::Compress,
            ScalarFuncSig::Md5,
            ScalarFuncSig::Password,
            ScalarFuncSig::RandomBytes,
            ScalarFuncSig::Sha1,
            ScalarFuncSig::Sha2,
            ScalarFuncSig::Uncompress,
            ScalarFuncSig::UncompressedLength,
            ScalarFuncSig::Database,
            ScalarFuncSig::FoundRows,
            ScalarFuncSig::CurrentUser,
            ScalarFuncSig::User,
            ScalarFuncSig::ConnectionId,
            ScalarFuncSig::LastInsertId,
            ScalarFuncSig::LastInsertIdWithId,
            ScalarFuncSig::Version,
            ScalarFuncSig::TiDbVersion,
            ScalarFuncSig::RowCount,
            ScalarFuncSig::Sleep,
            ScalarFuncSig::Lock,
            ScalarFuncSig::ReleaseLock,
            ScalarFuncSig::DecimalAnyValue,
            ScalarFuncSig::DurationAnyValue,
            ScalarFuncSig::IntAnyValue,
            ScalarFuncSig::JsonAnyValue,
            ScalarFuncSig::RealAnyValue,
            ScalarFuncSig::StringAnyValue,
            ScalarFuncSig::TimeAnyValue,
            ScalarFuncSig::InetAton,
            ScalarFuncSig::InetNtoa,
            ScalarFuncSig::Inet6Aton,
            ScalarFuncSig::Inet6Ntoa,
            ScalarFuncSig::IsIPv4,
            ScalarFuncSig::IsIPv4Compat,
            ScalarFuncSig::IsIPv4Mapped,
            ScalarFuncSig::IsIPv6,
            ScalarFuncSig::Uuid,
            ScalarFuncSig::LikeSig,
            ScalarFuncSig::RegexpBinarySig,
            ScalarFuncSig::RegexpSig,
            ScalarFuncSig::JsonExtractSig,
            ScalarFuncSig::JsonUnquoteSig,
            ScalarFuncSig::JsonTypeSig,
            ScalarFuncSig::JsonSetSig,
            ScalarFuncSig::JsonInsertSig,
            ScalarFuncSig::JsonReplaceSig,
            ScalarFuncSig::JsonRemoveSig,
            ScalarFuncSig::JsonMergeSig,
            ScalarFuncSig::JsonObjectSig,
            ScalarFuncSig::JsonArraySig,
            ScalarFuncSig::JsonValidJsonSig,
            ScalarFuncSig::JsonContainsSig,
            ScalarFuncSig::JsonArrayAppendSig,
            ScalarFuncSig::JsonArrayInsertSig,
            ScalarFuncSig::JsonMergePatchSig,
            ScalarFuncSig::JsonMergePreserveSig,
            ScalarFuncSig::JsonContainsPathSig,
            ScalarFuncSig::JsonPrettySig,
            ScalarFuncSig::JsonQuoteSig,
            ScalarFuncSig::JsonSearchSig,
            ScalarFuncSig::JsonStorageSizeSig,
            ScalarFuncSig::JsonDepthSig,
            ScalarFuncSig::JsonKeysSig,
            ScalarFuncSig::JsonLengthSig,
            ScalarFuncSig::JsonKeys2ArgsSig,
            ScalarFuncSig::JsonValidStringSig,
            ScalarFuncSig::DateFormatSig,
            ScalarFuncSig::DateLiteral,
            ScalarFuncSig::DateDiff,
            ScalarFuncSig::NullTimeDiff,
            ScalarFuncSig::TimeStringTimeDiff,
            ScalarFuncSig::DurationStringTimeDiff,
            ScalarFuncSig::DurationDurationTimeDiff,
            ScalarFuncSig::StringTimeTimeDiff,
            ScalarFuncSig::StringDurationTimeDiff,
            ScalarFuncSig::StringStringTimeDiff,
            ScalarFuncSig::TimeTimeTimeDiff,
            ScalarFuncSig::Date,
            ScalarFuncSig::Hour,
            ScalarFuncSig::Minute,
            ScalarFuncSig::Second,
            ScalarFuncSig::MicroSecond,
            ScalarFuncSig::Month,
            ScalarFuncSig::MonthName,
            ScalarFuncSig::NowWithArg,
            ScalarFuncSig::NowWithoutArg,
            ScalarFuncSig::DayName,
            ScalarFuncSig::DayOfMonth,
            ScalarFuncSig::DayOfWeek,
            ScalarFuncSig::DayOfYear,
            ScalarFuncSig::WeekWithMode,
            ScalarFuncSig::WeekWithoutMode,
            ScalarFuncSig::WeekDay,
            ScalarFuncSig::WeekOfYear,
            ScalarFuncSig::Year,
            ScalarFuncSig::YearWeekWithMode,
            ScalarFuncSig::YearWeekWithoutMode,
            ScalarFuncSig::GetFormat,
            ScalarFuncSig::SysDateWithFsp,
            ScalarFuncSig::SysDateWithoutFsp,
            ScalarFuncSig::CurrentDate,
            ScalarFuncSig::CurrentTime0Arg,
            ScalarFuncSig::CurrentTime1Arg,
            ScalarFuncSig::Time,
            ScalarFuncSig::TimeLiteral,
            ScalarFuncSig::UtcDate,
            ScalarFuncSig::UtcTimestampWithArg,
            ScalarFuncSig::UtcTimestampWithoutArg,
            ScalarFuncSig::AddDatetimeAndDuration,
            ScalarFuncSig::AddDatetimeAndString,
            ScalarFuncSig::AddTimeDateTimeNull,
            ScalarFuncSig::AddStringAndDuration,
            ScalarFuncSig::AddStringAndString,
            ScalarFuncSig::AddTimeStringNull,
            ScalarFuncSig::AddDurationAndDuration,
            ScalarFuncSig::AddDurationAndString,
            ScalarFuncSig::AddTimeDurationNull,
            ScalarFuncSig::AddDateAndDuration,
            ScalarFuncSig::AddDateAndString,
            ScalarFuncSig::SubDatetimeAndDuration,
            ScalarFuncSig::SubDatetimeAndString,
            ScalarFuncSig::SubTimeDateTimeNull,
            ScalarFuncSig::SubStringAndDuration,
            ScalarFuncSig::SubStringAndString,
            ScalarFuncSig::SubTimeStringNull,
            ScalarFuncSig::SubDurationAndDuration,
            ScalarFuncSig::SubDurationAndString,
            ScalarFuncSig::SubTimeDurationNull,
            ScalarFuncSig::SubDateAndDuration,
            ScalarFuncSig::SubDateAndString,
            ScalarFuncSig::UnixTimestampCurrent,
            ScalarFuncSig::UnixTimestampInt,
            ScalarFuncSig::UnixTimestampDec,
            ScalarFuncSig::ConvertTz,
            ScalarFuncSig::MakeDate,
            ScalarFuncSig::MakeTime,
            ScalarFuncSig::PeriodAdd,
            ScalarFuncSig::PeriodDiff,
            ScalarFuncSig::Quarter,
            ScalarFuncSig::SecToTime,
            ScalarFuncSig::TimeToSec,
            ScalarFuncSig::TimestampAdd,
            ScalarFuncSig::ToDays,
            ScalarFuncSig::ToSeconds,
            ScalarFuncSig::UtcTimeWithArg,
            ScalarFuncSig::UtcTimeWithoutArg,
            ScalarFuncSig::Timestamp1Arg,
            ScalarFuncSig::Timestamp2Args,
            ScalarFuncSig::TimestampLiteral,
            ScalarFuncSig::LastDay,
            ScalarFuncSig::StrToDateDate,
            ScalarFuncSig::StrToDateDatetime,
            ScalarFuncSig::StrToDateDuration,
            ScalarFuncSig::FromUnixTime1Arg,
            ScalarFuncSig::FromUnixTime2Arg,
            ScalarFuncSig::ExtractDatetime,
            ScalarFuncSig::ExtractDuration,
            ScalarFuncSig::AddDateStringString,
            ScalarFuncSig::AddDateStringInt,
            ScalarFuncSig::AddDateStringDecimal,
            ScalarFuncSig::AddDateIntString,
            ScalarFuncSig::AddDateIntInt,
            ScalarFuncSig::AddDateDatetimeString,
            ScalarFuncSig::AddDateDatetimeInt,
            ScalarFuncSig::SubDateStringString,
            ScalarFuncSig::SubDateStringInt,
            ScalarFuncSig::SubDateStringDecimal,
            ScalarFuncSig::SubDateIntString,
            ScalarFuncSig::SubDateIntInt,
            ScalarFuncSig::SubDateDatetimeString,
            ScalarFuncSig::SubDateDatetimeInt,
            ScalarFuncSig::FromDays,
            ScalarFuncSig::TimeFormat,
            ScalarFuncSig::TimestampDiff,
            ScalarFuncSig::BitLength,
            ScalarFuncSig::Bin,
            ScalarFuncSig::Ascii,
            ScalarFuncSig::Char,
            ScalarFuncSig::CharLength,
            ScalarFuncSig::Concat,
            ScalarFuncSig::ConcatWs,
            ScalarFuncSig::Convert,
            ScalarFuncSig::Elt,
            ScalarFuncSig::ExportSet3Arg,
            ScalarFuncSig::ExportSet4Arg,
            ScalarFuncSig::ExportSet5Arg,
            ScalarFuncSig::FieldInt,
            ScalarFuncSig::FieldReal,
            ScalarFuncSig::FieldString,
            ScalarFuncSig::FindInSet,
            ScalarFuncSig::Format,
            ScalarFuncSig::FormatWithLocale,
            ScalarFuncSig::FromBase64,
            ScalarFuncSig::HexIntArg,
            ScalarFuncSig::HexStrArg,
            ScalarFuncSig::Insert,
            ScalarFuncSig::InsertBinary,
            ScalarFuncSig::Instr,
            ScalarFuncSig::InstrBinary,
            ScalarFuncSig::LTrim,
            ScalarFuncSig::Left,
            ScalarFuncSig::LeftBinary,
            ScalarFuncSig::Length,
            ScalarFuncSig::Locate2Args,
            ScalarFuncSig::Locate3Args,
            ScalarFuncSig::LocateBinary2Args,
            ScalarFuncSig::LocateBinary3Args,
            ScalarFuncSig::Lower,
            ScalarFuncSig::Lpad,
            ScalarFuncSig::LpadBinary,
            ScalarFuncSig::MakeSet,
            ScalarFuncSig::OctInt,
            ScalarFuncSig::OctString,
            ScalarFuncSig::Ord,
            ScalarFuncSig::Quote,
            ScalarFuncSig::RTrim,
            ScalarFuncSig::Repeat,
            ScalarFuncSig::Replace,
            ScalarFuncSig::Reverse,
            ScalarFuncSig::ReverseBinary,
            ScalarFuncSig::Right,
            ScalarFuncSig::RightBinary,
            ScalarFuncSig::Rpad,
            ScalarFuncSig::RpadBinary,
            ScalarFuncSig::Space,
            ScalarFuncSig::Strcmp,
            ScalarFuncSig::Substring2Args,
            ScalarFuncSig::Substring3Args,
            ScalarFuncSig::SubstringBinary2Args,
            ScalarFuncSig::SubstringBinary3Args,
            ScalarFuncSig::SubstringIndex,
            ScalarFuncSig::ToBase64,
            ScalarFuncSig::Trim1Arg,
            ScalarFuncSig::Trim2Args,
            ScalarFuncSig::Trim3Args,
            ScalarFuncSig::UnHex,
            ScalarFuncSig::Upper,
        ];
        VALUES
    }
}
impl Executor {
    pub fn new_() -> Executor {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_tp(&self) -> bool {
        self.tp.is_some()
    }
    #[inline]
    pub fn clear_tp(&mut self) {
        self.tp = ::std::option::Option::None
    }
    #[inline]
    pub fn set_tp_(&mut self, v: ExecType) {
        self.tp = ::std::option::Option::Some(unsafe { ::std::mem::transmute::<ExecType, i32>(v) });
    }
    #[inline]
    pub fn get_tp(&self) -> ExecType {
        unsafe {
            ::std::mem::transmute::<i32, ExecType>(match self.tp {
                Some(v) => v,
                None => 0,
            })
        }
    }
    #[inline]
    pub fn has_tbl_scan(&self) -> bool {
        self.tbl_scan.is_some()
    }
    #[inline]
    pub fn clear_tbl_scan(&mut self) {
        self.tbl_scan = ::std::option::Option::None
    }
    #[inline]
    pub fn set_tbl_scan(&mut self, v: TableScan) {
        self.tbl_scan = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_tbl_scan(&self) -> &TableScan {
        match self.tbl_scan.as_ref() {
            Some(v) => v,
            None => TableScan::default_ref(),
        }
    }
    #[inline]
    pub fn mut_tbl_scan(&mut self) -> &mut TableScan {
        if self.tbl_scan.is_none() {
            self.tbl_scan = ::std::option::Option::Some(TableScan::default());
        }
        self.tbl_scan.as_mut().unwrap()
    }
    #[inline]
    pub fn take_tbl_scan(&mut self) -> TableScan {
        self.tbl_scan.take().unwrap_or_else(TableScan::default)
    }
    #[inline]
    pub fn has_idx_scan(&self) -> bool {
        self.idx_scan.is_some()
    }
    #[inline]
    pub fn clear_idx_scan(&mut self) {
        self.idx_scan = ::std::option::Option::None
    }
    #[inline]
    pub fn set_idx_scan(&mut self, v: IndexScan) {
        self.idx_scan = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_idx_scan(&self) -> &IndexScan {
        match self.idx_scan.as_ref() {
            Some(v) => v,
            None => IndexScan::default_ref(),
        }
    }
    #[inline]
    pub fn mut_idx_scan(&mut self) -> &mut IndexScan {
        if self.idx_scan.is_none() {
            self.idx_scan = ::std::option::Option::Some(IndexScan::default());
        }
        self.idx_scan.as_mut().unwrap()
    }
    #[inline]
    pub fn take_idx_scan(&mut self) -> IndexScan {
        self.idx_scan.take().unwrap_or_else(IndexScan::default)
    }
    #[inline]
    pub fn has_selection(&self) -> bool {
        self.selection.is_some()
    }
    #[inline]
    pub fn clear_selection(&mut self) {
        self.selection = ::std::option::Option::None
    }
    #[inline]
    pub fn set_selection(&mut self, v: Selection) {
        self.selection = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_selection(&self) -> &Selection {
        match self.selection.as_ref() {
            Some(v) => v,
            None => Selection::default_ref(),
        }
    }
    #[inline]
    pub fn mut_selection(&mut self) -> &mut Selection {
        if self.selection.is_none() {
            self.selection = ::std::option::Option::Some(Selection::default());
        }
        self.selection.as_mut().unwrap()
    }
    #[inline]
    pub fn take_selection(&mut self) -> Selection {
        self.selection.take().unwrap_or_else(Selection::default)
    }
    #[inline]
    pub fn has_aggregation(&self) -> bool {
        self.aggregation.is_some()
    }
    #[inline]
    pub fn clear_aggregation(&mut self) {
        self.aggregation = ::std::option::Option::None
    }
    #[inline]
    pub fn set_aggregation(&mut self, v: Aggregation) {
        self.aggregation = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_aggregation(&self) -> &Aggregation {
        match self.aggregation.as_ref() {
            Some(v) => v,
            None => Aggregation::default_ref(),
        }
    }
    #[inline]
    pub fn mut_aggregation(&mut self) -> &mut Aggregation {
        if self.aggregation.is_none() {
            self.aggregation = ::std::option::Option::Some(Aggregation::default());
        }
        self.aggregation.as_mut().unwrap()
    }
    #[inline]
    pub fn take_aggregation(&mut self) -> Aggregation {
        self.aggregation.take().unwrap_or_else(Aggregation::default)
    }
    #[inline]
    pub fn has_top_n(&self) -> bool {
        self.top_n.is_some()
    }
    #[inline]
    pub fn clear_top_n(&mut self) {
        self.top_n = ::std::option::Option::None
    }
    #[inline]
    pub fn set_top_n(&mut self, v: TopN) {
        self.top_n = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_top_n(&self) -> &TopN {
        match self.top_n.as_ref() {
            Some(v) => v,
            None => TopN::default_ref(),
        }
    }
    #[inline]
    pub fn mut_top_n(&mut self) -> &mut TopN {
        if self.top_n.is_none() {
            self.top_n = ::std::option::Option::Some(TopN::default());
        }
        self.top_n.as_mut().unwrap()
    }
    #[inline]
    pub fn take_top_n(&mut self) -> TopN {
        self.top_n.take().unwrap_or_else(TopN::default)
    }
    #[inline]
    pub fn has_limit(&self) -> bool {
        self.limit.is_some()
    }
    #[inline]
    pub fn clear_limit(&mut self) {
        self.limit = ::std::option::Option::None
    }
    #[inline]
    pub fn set_limit(&mut self, v: Limit) {
        self.limit = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_limit(&self) -> &Limit {
        match self.limit.as_ref() {
            Some(v) => v,
            None => Limit::default_ref(),
        }
    }
    #[inline]
    pub fn mut_limit(&mut self) -> &mut Limit {
        if self.limit.is_none() {
            self.limit = ::std::option::Option::Some(Limit::default());
        }
        self.limit.as_mut().unwrap()
    }
    #[inline]
    pub fn take_limit(&mut self) -> Limit {
        self.limit.take().unwrap_or_else(Limit::default)
    }
    #[inline]
    pub fn has_stream_agg(&self) -> bool {
        self.stream_agg.is_some()
    }
    #[inline]
    pub fn clear_stream_agg(&mut self) {
        self.stream_agg = ::std::option::Option::None
    }
    #[inline]
    pub fn set_stream_agg(&mut self, v: Aggregation) {
        self.stream_agg = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_stream_agg(&self) -> &Aggregation {
        match self.stream_agg.as_ref() {
            Some(v) => v,
            None => Aggregation::default_ref(),
        }
    }
    #[inline]
    pub fn mut_stream_agg(&mut self) -> &mut Aggregation {
        if self.stream_agg.is_none() {
            self.stream_agg = ::std::option::Option::Some(Aggregation::default());
        }
        self.stream_agg.as_mut().unwrap()
    }
    #[inline]
    pub fn take_stream_agg(&mut self) -> Aggregation {
        self.stream_agg.take().unwrap_or_else(Aggregation::default)
    }
}
impl ::protobuf::Clear for Executor {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Executor {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static Executor {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Executor = Executor::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl TableScan {
    pub fn new_() -> TableScan {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_table_id(&self) -> bool {
        self.table_id.is_some()
    }
    #[inline]
    pub fn clear_table_id(&mut self) {
        self.table_id = ::std::option::Option::None
    }
    #[inline]
    pub fn set_table_id(&mut self, v: i64) {
        self.table_id = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_table_id(&self) -> i64 {
        match self.table_id {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }
    #[inline]
    pub fn set_columns(&mut self, v: ::std::vec::Vec<ColumnInfo>) {
        self.columns = v;
    }
    #[inline]
    pub fn get_columns(&self) -> &::std::vec::Vec<ColumnInfo> {
        &self.columns
    }
    #[inline]
    pub fn mut_columns(&mut self) -> &mut ::std::vec::Vec<ColumnInfo> {
        &mut self.columns
    }
    #[inline]
    pub fn take_columns(&mut self) -> ::std::vec::Vec<ColumnInfo> {
        ::std::mem::replace(&mut self.columns, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_desc(&self) -> bool {
        self.desc.is_some()
    }
    #[inline]
    pub fn clear_desc(&mut self) {
        self.desc = ::std::option::Option::None
    }
    #[inline]
    pub fn set_desc(&mut self, v: bool) {
        self.desc = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_desc(&self) -> bool {
        match self.desc {
            Some(v) => v,
            None => false,
        }
    }
}
impl ::protobuf::Clear for TableScan {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for TableScan {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static TableScan {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: TableScan = TableScan::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl IndexScan {
    pub fn new_() -> IndexScan {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_table_id(&self) -> bool {
        self.table_id.is_some()
    }
    #[inline]
    pub fn clear_table_id(&mut self) {
        self.table_id = ::std::option::Option::None
    }
    #[inline]
    pub fn set_table_id(&mut self, v: i64) {
        self.table_id = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_table_id(&self) -> i64 {
        match self.table_id {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_index_id(&self) -> bool {
        self.index_id.is_some()
    }
    #[inline]
    pub fn clear_index_id(&mut self) {
        self.index_id = ::std::option::Option::None
    }
    #[inline]
    pub fn set_index_id(&mut self, v: i64) {
        self.index_id = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_index_id(&self) -> i64 {
        match self.index_id {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }
    #[inline]
    pub fn set_columns(&mut self, v: ::std::vec::Vec<ColumnInfo>) {
        self.columns = v;
    }
    #[inline]
    pub fn get_columns(&self) -> &::std::vec::Vec<ColumnInfo> {
        &self.columns
    }
    #[inline]
    pub fn mut_columns(&mut self) -> &mut ::std::vec::Vec<ColumnInfo> {
        &mut self.columns
    }
    #[inline]
    pub fn take_columns(&mut self) -> ::std::vec::Vec<ColumnInfo> {
        ::std::mem::replace(&mut self.columns, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_desc(&self) -> bool {
        self.desc.is_some()
    }
    #[inline]
    pub fn clear_desc(&mut self) {
        self.desc = ::std::option::Option::None
    }
    #[inline]
    pub fn set_desc(&mut self, v: bool) {
        self.desc = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_desc(&self) -> bool {
        match self.desc {
            Some(v) => v,
            None => false,
        }
    }
    #[inline]
    pub fn has_unique(&self) -> bool {
        self.unique.is_some()
    }
    #[inline]
    pub fn clear_unique(&mut self) {
        self.unique = ::std::option::Option::None
    }
    #[inline]
    pub fn set_unique(&mut self, v: bool) {
        self.unique = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_unique(&self) -> bool {
        match self.unique {
            Some(v) => v,
            None => false,
        }
    }
}
impl ::protobuf::Clear for IndexScan {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for IndexScan {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static IndexScan {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: IndexScan = IndexScan::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Selection {
    pub fn new_() -> Selection {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_conditions(&mut self) {
        self.conditions.clear();
    }
    #[inline]
    pub fn set_conditions(&mut self, v: ::std::vec::Vec<Expr>) {
        self.conditions = v;
    }
    #[inline]
    pub fn get_conditions(&self) -> &::std::vec::Vec<Expr> {
        &self.conditions
    }
    #[inline]
    pub fn mut_conditions(&mut self) -> &mut ::std::vec::Vec<Expr> {
        &mut self.conditions
    }
    #[inline]
    pub fn take_conditions(&mut self) -> ::std::vec::Vec<Expr> {
        ::std::mem::replace(&mut self.conditions, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for Selection {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Selection {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static Selection {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Selection = Selection::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Projection {
    pub fn new_() -> Projection {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_exprs(&mut self) {
        self.exprs.clear();
    }
    #[inline]
    pub fn set_exprs(&mut self, v: ::std::vec::Vec<Expr>) {
        self.exprs = v;
    }
    #[inline]
    pub fn get_exprs(&self) -> &::std::vec::Vec<Expr> {
        &self.exprs
    }
    #[inline]
    pub fn mut_exprs(&mut self) -> &mut ::std::vec::Vec<Expr> {
        &mut self.exprs
    }
    #[inline]
    pub fn take_exprs(&mut self) -> ::std::vec::Vec<Expr> {
        ::std::mem::replace(&mut self.exprs, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for Projection {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Projection {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static Projection {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Projection = Projection::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Aggregation {
    pub fn new_() -> Aggregation {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_group_by(&mut self) {
        self.group_by.clear();
    }
    #[inline]
    pub fn set_group_by(&mut self, v: ::std::vec::Vec<Expr>) {
        self.group_by = v;
    }
    #[inline]
    pub fn get_group_by(&self) -> &::std::vec::Vec<Expr> {
        &self.group_by
    }
    #[inline]
    pub fn mut_group_by(&mut self) -> &mut ::std::vec::Vec<Expr> {
        &mut self.group_by
    }
    #[inline]
    pub fn take_group_by(&mut self) -> ::std::vec::Vec<Expr> {
        ::std::mem::replace(&mut self.group_by, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_agg_func(&mut self) {
        self.agg_func.clear();
    }
    #[inline]
    pub fn set_agg_func(&mut self, v: ::std::vec::Vec<Expr>) {
        self.agg_func = v;
    }
    #[inline]
    pub fn get_agg_func(&self) -> &::std::vec::Vec<Expr> {
        &self.agg_func
    }
    #[inline]
    pub fn mut_agg_func(&mut self) -> &mut ::std::vec::Vec<Expr> {
        &mut self.agg_func
    }
    #[inline]
    pub fn take_agg_func(&mut self) -> ::std::vec::Vec<Expr> {
        ::std::mem::replace(&mut self.agg_func, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_streamed(&self) -> bool {
        self.streamed.is_some()
    }
    #[inline]
    pub fn clear_streamed(&mut self) {
        self.streamed = ::std::option::Option::None
    }
    #[inline]
    pub fn set_streamed(&mut self, v: bool) {
        self.streamed = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_streamed(&self) -> bool {
        match self.streamed {
            Some(v) => v,
            None => false,
        }
    }
}
impl ::protobuf::Clear for Aggregation {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Aggregation {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static Aggregation {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Aggregation = Aggregation::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl TopN {
    pub fn new_() -> TopN {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_order_by(&mut self) {
        self.order_by.clear();
    }
    #[inline]
    pub fn set_order_by(&mut self, v: ::std::vec::Vec<ByItem>) {
        self.order_by = v;
    }
    #[inline]
    pub fn get_order_by(&self) -> &::std::vec::Vec<ByItem> {
        &self.order_by
    }
    #[inline]
    pub fn mut_order_by(&mut self) -> &mut ::std::vec::Vec<ByItem> {
        &mut self.order_by
    }
    #[inline]
    pub fn take_order_by(&mut self) -> ::std::vec::Vec<ByItem> {
        ::std::mem::replace(&mut self.order_by, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_limit(&self) -> bool {
        self.limit.is_some()
    }
    #[inline]
    pub fn clear_limit(&mut self) {
        self.limit = ::std::option::Option::None
    }
    #[inline]
    pub fn set_limit(&mut self, v: u64) {
        self.limit = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_limit(&self) -> u64 {
        match self.limit {
            Some(v) => v,
            None => 0,
        }
    }
}
impl ::protobuf::Clear for TopN {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for TopN {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static TopN {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: TopN = TopN::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Limit {
    pub fn new_() -> Limit {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_limit(&self) -> bool {
        self.limit.is_some()
    }
    #[inline]
    pub fn clear_limit(&mut self) {
        self.limit = ::std::option::Option::None
    }
    #[inline]
    pub fn set_limit(&mut self, v: u64) {
        self.limit = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_limit(&self) -> u64 {
        match self.limit {
            Some(v) => v,
            None => 0,
        }
    }
}
impl ::protobuf::Clear for Limit {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Limit {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static Limit {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Limit = Limit::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ExecutorExecutionSummary {
    pub fn new_() -> ExecutorExecutionSummary {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_time_processed_ns(&self) -> bool {
        self.time_processed_ns.is_some()
    }
    #[inline]
    pub fn clear_time_processed_ns(&mut self) {
        self.time_processed_ns = ::std::option::Option::None
    }
    #[inline]
    pub fn set_time_processed_ns(&mut self, v: u64) {
        self.time_processed_ns = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_time_processed_ns(&self) -> u64 {
        match self.time_processed_ns {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_num_produced_rows(&self) -> bool {
        self.num_produced_rows.is_some()
    }
    #[inline]
    pub fn clear_num_produced_rows(&mut self) {
        self.num_produced_rows = ::std::option::Option::None
    }
    #[inline]
    pub fn set_num_produced_rows(&mut self, v: u64) {
        self.num_produced_rows = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_num_produced_rows(&self) -> u64 {
        match self.num_produced_rows {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_num_iterations(&self) -> bool {
        self.num_iterations.is_some()
    }
    #[inline]
    pub fn clear_num_iterations(&mut self) {
        self.num_iterations = ::std::option::Option::None
    }
    #[inline]
    pub fn set_num_iterations(&mut self, v: u64) {
        self.num_iterations = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_num_iterations(&self) -> u64 {
        match self.num_iterations {
            Some(v) => v,
            None => 0,
        }
    }
}
impl ::protobuf::Clear for ExecutorExecutionSummary {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ExecutorExecutionSummary {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static ExecutorExecutionSummary {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ExecutorExecutionSummary = ExecutorExecutionSummary::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ExecType {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [ExecType] = &[
            ExecType::TypeTableScan,
            ExecType::TypeIndexScan,
            ExecType::TypeSelection,
            ExecType::TypeAggregation,
            ExecType::TypeTopN,
            ExecType::TypeLimit,
            ExecType::TypeStreamAgg,
        ];
        VALUES
    }
}
impl Row {
    pub fn new_() -> Row {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_handle(&self) -> bool {
        self.handle.is_some()
    }
    #[inline]
    pub fn clear_handle(&mut self) {
        self.handle = ::std::option::Option::None
    }
    #[inline]
    pub fn set_handle(&mut self, v: std::vec::Vec<u8>) {
        self.handle = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_handle(&self) -> &[u8] {
        match self.handle.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }
    #[inline]
    pub fn mut_handle(&mut self) -> &mut std::vec::Vec<u8> {
        if self.handle.is_none() {
            self.handle = ::std::option::Option::Some(::std::vec::Vec::default());
        }
        self.handle.as_mut().unwrap()
    }
    #[inline]
    pub fn take_handle(&mut self) -> std::vec::Vec<u8> {
        self.handle.take().unwrap_or_else(::std::vec::Vec::new)
    }
    #[inline]
    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }
    #[inline]
    pub fn clear_data(&mut self) {
        self.data = ::std::option::Option::None
    }
    #[inline]
    pub fn set_data(&mut self, v: std::vec::Vec<u8>) {
        self.data = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }
    #[inline]
    pub fn mut_data(&mut self) -> &mut std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data = ::std::option::Option::Some(::std::vec::Vec::default());
        }
        self.data.as_mut().unwrap()
    }
    #[inline]
    pub fn take_data(&mut self) -> std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(::std::vec::Vec::new)
    }
}
impl ::protobuf::Clear for Row {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Row {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static Row {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Row = Row::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Error {
    pub fn new_() -> Error {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_code(&self) -> bool {
        self.code.is_some()
    }
    #[inline]
    pub fn clear_code(&mut self) {
        self.code = ::std::option::Option::None
    }
    #[inline]
    pub fn set_code(&mut self, v: i32) {
        self.code = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_code(&self) -> i32 {
        match self.code {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }
    #[inline]
    pub fn clear_msg(&mut self) {
        self.msg = ::std::option::Option::None
    }
    #[inline]
    pub fn set_msg(&mut self, v: std::string::String) {
        self.msg = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_msg(&self) -> &str {
        match self.msg.as_ref() {
            Some(v) => v,
            None => "",
        }
    }
    #[inline]
    pub fn mut_msg(&mut self) -> &mut std::string::String {
        if self.msg.is_none() {
            self.msg = ::std::option::Option::Some(std::string::String::default());
        }
        self.msg.as_mut().unwrap()
    }
    #[inline]
    pub fn take_msg(&mut self) -> std::string::String {
        self.msg.take().unwrap_or_else(::std::string::String::new)
    }
}
impl ::protobuf::Clear for Error {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Error {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static Error {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Error = Error::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl SelectResponse {
    pub fn new_() -> SelectResponse {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_error(&mut self, v: Error) {
        self.error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_error(&self) -> &Error {
        match self.error.as_ref() {
            Some(v) => v,
            None => Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut Error {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(Error::default());
        }
        self.error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_error(&mut self) -> Error {
        self.error.take().unwrap_or_else(Error::default)
    }
    #[inline]
    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }
    #[inline]
    pub fn set_rows(&mut self, v: ::std::vec::Vec<Row>) {
        self.rows = v;
    }
    #[inline]
    pub fn get_rows(&self) -> &::std::vec::Vec<Row> {
        &self.rows
    }
    #[inline]
    pub fn mut_rows(&mut self) -> &mut ::std::vec::Vec<Row> {
        &mut self.rows
    }
    #[inline]
    pub fn take_rows(&mut self) -> ::std::vec::Vec<Row> {
        ::std::mem::replace(&mut self.rows, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_chunks(&mut self) {
        self.chunks.clear();
    }
    #[inline]
    pub fn set_chunks(&mut self, v: ::std::vec::Vec<Chunk>) {
        self.chunks = v;
    }
    #[inline]
    pub fn get_chunks(&self) -> &::std::vec::Vec<Chunk> {
        &self.chunks
    }
    #[inline]
    pub fn mut_chunks(&mut self) -> &mut ::std::vec::Vec<Chunk> {
        &mut self.chunks
    }
    #[inline]
    pub fn take_chunks(&mut self) -> ::std::vec::Vec<Chunk> {
        ::std::mem::replace(&mut self.chunks, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_warnings(&mut self) {
        self.warnings.clear();
    }
    #[inline]
    pub fn set_warnings(&mut self, v: ::std::vec::Vec<Error>) {
        self.warnings = v;
    }
    #[inline]
    pub fn get_warnings(&self) -> &::std::vec::Vec<Error> {
        &self.warnings
    }
    #[inline]
    pub fn mut_warnings(&mut self) -> &mut ::std::vec::Vec<Error> {
        &mut self.warnings
    }
    #[inline]
    pub fn take_warnings(&mut self) -> ::std::vec::Vec<Error> {
        ::std::mem::replace(&mut self.warnings, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_output_counts(&mut self) {
        self.output_counts.clear();
    }
    #[inline]
    pub fn set_output_counts(&mut self, v: ::std::vec::Vec<i64>) {
        self.output_counts = v;
    }
    #[inline]
    pub fn get_output_counts(&self) -> &::std::vec::Vec<i64> {
        &self.output_counts
    }
    #[inline]
    pub fn mut_output_counts(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.output_counts
    }
    #[inline]
    pub fn take_output_counts(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.output_counts, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_warning_count(&self) -> bool {
        self.warning_count.is_some()
    }
    #[inline]
    pub fn clear_warning_count(&mut self) {
        self.warning_count = ::std::option::Option::None
    }
    #[inline]
    pub fn set_warning_count(&mut self, v: i64) {
        self.warning_count = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_warning_count(&self) -> i64 {
        match self.warning_count {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_row_batch_data(&self) -> bool {
        self.row_batch_data.is_some()
    }
    #[inline]
    pub fn clear_row_batch_data(&mut self) {
        self.row_batch_data = ::std::option::Option::None
    }
    #[inline]
    pub fn set_row_batch_data(&mut self, v: std::vec::Vec<u8>) {
        self.row_batch_data = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_row_batch_data(&self) -> &[u8] {
        match self.row_batch_data.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }
    #[inline]
    pub fn mut_row_batch_data(&mut self) -> &mut std::vec::Vec<u8> {
        if self.row_batch_data.is_none() {
            self.row_batch_data = ::std::option::Option::Some(::std::vec::Vec::default());
        }
        self.row_batch_data.as_mut().unwrap()
    }
    #[inline]
    pub fn take_row_batch_data(&mut self) -> std::vec::Vec<u8> {
        self.row_batch_data
            .take()
            .unwrap_or_else(::std::vec::Vec::new)
    }
    #[inline]
    pub fn clear_execution_summaries(&mut self) {
        self.execution_summaries.clear();
    }
    #[inline]
    pub fn set_execution_summaries(&mut self, v: ::std::vec::Vec<ExecutorExecutionSummary>) {
        self.execution_summaries = v;
    }
    #[inline]
    pub fn get_execution_summaries(&self) -> &::std::vec::Vec<ExecutorExecutionSummary> {
        &self.execution_summaries
    }
    #[inline]
    pub fn mut_execution_summaries(&mut self) -> &mut ::std::vec::Vec<ExecutorExecutionSummary> {
        &mut self.execution_summaries
    }
    #[inline]
    pub fn take_execution_summaries(&mut self) -> ::std::vec::Vec<ExecutorExecutionSummary> {
        ::std::mem::replace(&mut self.execution_summaries, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for SelectResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SelectResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static SelectResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SelectResponse = SelectResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Chunk {
    pub fn new_() -> Chunk {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_rows_data(&self) -> bool {
        self.rows_data.is_some()
    }
    #[inline]
    pub fn clear_rows_data(&mut self) {
        self.rows_data = ::std::option::Option::None
    }
    #[inline]
    pub fn set_rows_data(&mut self, v: std::vec::Vec<u8>) {
        self.rows_data = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_rows_data(&self) -> &[u8] {
        match self.rows_data.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }
    #[inline]
    pub fn mut_rows_data(&mut self) -> &mut std::vec::Vec<u8> {
        if self.rows_data.is_none() {
            self.rows_data = ::std::option::Option::Some(::std::vec::Vec::default());
        }
        self.rows_data.as_mut().unwrap()
    }
    #[inline]
    pub fn take_rows_data(&mut self) -> std::vec::Vec<u8> {
        self.rows_data.take().unwrap_or_else(::std::vec::Vec::new)
    }
    #[inline]
    pub fn clear_rows_meta(&mut self) {
        self.rows_meta.clear();
    }
    #[inline]
    pub fn set_rows_meta(&mut self, v: ::std::vec::Vec<RowMeta>) {
        self.rows_meta = v;
    }
    #[inline]
    pub fn get_rows_meta(&self) -> &::std::vec::Vec<RowMeta> {
        &self.rows_meta
    }
    #[inline]
    pub fn mut_rows_meta(&mut self) -> &mut ::std::vec::Vec<RowMeta> {
        &mut self.rows_meta
    }
    #[inline]
    pub fn take_rows_meta(&mut self) -> ::std::vec::Vec<RowMeta> {
        ::std::mem::replace(&mut self.rows_meta, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for Chunk {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Chunk {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static Chunk {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Chunk = Chunk::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RowMeta {
    pub fn new_() -> RowMeta {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_handle(&self) -> bool {
        self.handle.is_some()
    }
    #[inline]
    pub fn clear_handle(&mut self) {
        self.handle = ::std::option::Option::None
    }
    #[inline]
    pub fn set_handle(&mut self, v: i64) {
        self.handle = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_handle(&self) -> i64 {
        match self.handle {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }
    #[inline]
    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None
    }
    #[inline]
    pub fn set_length(&mut self, v: i64) {
        self.length = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_length(&self) -> i64 {
        match self.length {
            Some(v) => v,
            None => 0,
        }
    }
}
impl ::protobuf::Clear for RowMeta {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RowMeta {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RowMeta {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RowMeta = RowMeta::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl DagRequest {
    pub fn new_() -> DagRequest {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_start_ts(&self) -> bool {
        self.start_ts.is_some()
    }
    #[inline]
    pub fn clear_start_ts(&mut self) {
        self.start_ts = ::std::option::Option::None
    }
    #[inline]
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_start_ts(&self) -> u64 {
        match self.start_ts {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn clear_executors(&mut self) {
        self.executors.clear();
    }
    #[inline]
    pub fn set_executors(&mut self, v: ::std::vec::Vec<Executor>) {
        self.executors = v;
    }
    #[inline]
    pub fn get_executors(&self) -> &::std::vec::Vec<Executor> {
        &self.executors
    }
    #[inline]
    pub fn mut_executors(&mut self) -> &mut ::std::vec::Vec<Executor> {
        &mut self.executors
    }
    #[inline]
    pub fn take_executors(&mut self) -> ::std::vec::Vec<Executor> {
        ::std::mem::replace(&mut self.executors, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_time_zone_offset(&self) -> bool {
        self.time_zone_offset.is_some()
    }
    #[inline]
    pub fn clear_time_zone_offset(&mut self) {
        self.time_zone_offset = ::std::option::Option::None
    }
    #[inline]
    pub fn set_time_zone_offset(&mut self, v: i64) {
        self.time_zone_offset = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_time_zone_offset(&self) -> i64 {
        match self.time_zone_offset {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }
    #[inline]
    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None
    }
    #[inline]
    pub fn set_flags(&mut self, v: u64) {
        self.flags = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_flags(&self) -> u64 {
        match self.flags {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn clear_output_offsets(&mut self) {
        self.output_offsets.clear();
    }
    #[inline]
    pub fn set_output_offsets(&mut self, v: ::std::vec::Vec<u32>) {
        self.output_offsets = v;
    }
    #[inline]
    pub fn get_output_offsets(&self) -> &::std::vec::Vec<u32> {
        &self.output_offsets
    }
    #[inline]
    pub fn mut_output_offsets(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.output_offsets
    }
    #[inline]
    pub fn take_output_offsets(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.output_offsets, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_collect_range_counts(&self) -> bool {
        self.collect_range_counts.is_some()
    }
    #[inline]
    pub fn clear_collect_range_counts(&mut self) {
        self.collect_range_counts = ::std::option::Option::None
    }
    #[inline]
    pub fn set_collect_range_counts(&mut self, v: bool) {
        self.collect_range_counts = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_collect_range_counts(&self) -> bool {
        match self.collect_range_counts {
            Some(v) => v,
            None => false,
        }
    }
    #[inline]
    pub fn has_max_warning_count(&self) -> bool {
        self.max_warning_count.is_some()
    }
    #[inline]
    pub fn clear_max_warning_count(&mut self) {
        self.max_warning_count = ::std::option::Option::None
    }
    #[inline]
    pub fn set_max_warning_count(&mut self, v: u64) {
        self.max_warning_count = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_max_warning_count(&self) -> u64 {
        match self.max_warning_count {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_encode_type(&self) -> bool {
        self.encode_type.is_some()
    }
    #[inline]
    pub fn clear_encode_type(&mut self) {
        self.encode_type = ::std::option::Option::None
    }
    #[inline]
    pub fn set_encode_type_(&mut self, v: EncodeType) {
        self.encode_type =
            ::std::option::Option::Some(unsafe { ::std::mem::transmute::<EncodeType, i32>(v) });
    }
    #[inline]
    pub fn get_encode_type(&self) -> EncodeType {
        unsafe {
            ::std::mem::transmute::<i32, EncodeType>(match self.encode_type {
                Some(v) => v,
                None => 0,
            })
        }
    }
    #[inline]
    pub fn has_sql_mode(&self) -> bool {
        self.sql_mode.is_some()
    }
    #[inline]
    pub fn clear_sql_mode(&mut self) {
        self.sql_mode = ::std::option::Option::None
    }
    #[inline]
    pub fn set_sql_mode(&mut self, v: u64) {
        self.sql_mode = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_sql_mode(&self) -> u64 {
        match self.sql_mode {
            Some(v) => v,
            None => 0,
        }
    }
    #[inline]
    pub fn has_time_zone_name(&self) -> bool {
        self.time_zone_name.is_some()
    }
    #[inline]
    pub fn clear_time_zone_name(&mut self) {
        self.time_zone_name = ::std::option::Option::None
    }
    #[inline]
    pub fn set_time_zone_name(&mut self, v: std::string::String) {
        self.time_zone_name = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_time_zone_name(&self) -> &str {
        match self.time_zone_name.as_ref() {
            Some(v) => v,
            None => "",
        }
    }
    #[inline]
    pub fn mut_time_zone_name(&mut self) -> &mut std::string::String {
        if self.time_zone_name.is_none() {
            self.time_zone_name = ::std::option::Option::Some(std::string::String::default());
        }
        self.time_zone_name.as_mut().unwrap()
    }
    #[inline]
    pub fn take_time_zone_name(&mut self) -> std::string::String {
        self.time_zone_name
            .take()
            .unwrap_or_else(::std::string::String::new)
    }
    #[inline]
    pub fn has_collect_execution_summaries(&self) -> bool {
        self.collect_execution_summaries.is_some()
    }
    #[inline]
    pub fn clear_collect_execution_summaries(&mut self) {
        self.collect_execution_summaries = ::std::option::Option::None
    }
    #[inline]
    pub fn set_collect_execution_summaries(&mut self, v: bool) {
        self.collect_execution_summaries = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_collect_execution_summaries(&self) -> bool {
        match self.collect_execution_summaries {
            Some(v) => v,
            None => false,
        }
    }
    #[inline]
    pub fn has_max_allowed_packet(&self) -> bool {
        self.max_allowed_packet.is_some()
    }
    #[inline]
    pub fn clear_max_allowed_packet(&mut self) {
        self.max_allowed_packet = ::std::option::Option::None
    }
    #[inline]
    pub fn set_max_allowed_packet(&mut self, v: u64) {
        self.max_allowed_packet = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_max_allowed_packet(&self) -> u64 {
        match self.max_allowed_packet {
            Some(v) => v,
            None => 0,
        }
    }
}
impl ::protobuf::Clear for DagRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for DagRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static DagRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: DagRequest = DagRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl StreamResponse {
    pub fn new_() -> StreamResponse {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_error(&mut self, v: Error) {
        self.error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_error(&self) -> &Error {
        match self.error.as_ref() {
            Some(v) => v,
            None => Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut Error {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(Error::default());
        }
        self.error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_error(&mut self) -> Error {
        self.error.take().unwrap_or_else(Error::default)
    }
    #[inline]
    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }
    #[inline]
    pub fn clear_data(&mut self) {
        self.data = ::std::option::Option::None
    }
    #[inline]
    pub fn set_data(&mut self, v: std::vec::Vec<u8>) {
        self.data = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }
    #[inline]
    pub fn mut_data(&mut self) -> &mut std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data = ::std::option::Option::Some(::std::vec::Vec::default());
        }
        self.data.as_mut().unwrap()
    }
    #[inline]
    pub fn take_data(&mut self) -> std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(::std::vec::Vec::new)
    }
    #[inline]
    pub fn clear_warnings(&mut self) {
        self.warnings.clear();
    }
    #[inline]
    pub fn set_warnings(&mut self, v: ::std::vec::Vec<Error>) {
        self.warnings = v;
    }
    #[inline]
    pub fn get_warnings(&self) -> &::std::vec::Vec<Error> {
        &self.warnings
    }
    #[inline]
    pub fn mut_warnings(&mut self) -> &mut ::std::vec::Vec<Error> {
        &mut self.warnings
    }
    #[inline]
    pub fn take_warnings(&mut self) -> ::std::vec::Vec<Error> {
        ::std::mem::replace(&mut self.warnings, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_output_counts(&mut self) {
        self.output_counts.clear();
    }
    #[inline]
    pub fn set_output_counts(&mut self, v: ::std::vec::Vec<i64>) {
        self.output_counts = v;
    }
    #[inline]
    pub fn get_output_counts(&self) -> &::std::vec::Vec<i64> {
        &self.output_counts
    }
    #[inline]
    pub fn mut_output_counts(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.output_counts
    }
    #[inline]
    pub fn take_output_counts(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.output_counts, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_warning_count(&self) -> bool {
        self.warning_count.is_some()
    }
    #[inline]
    pub fn clear_warning_count(&mut self) {
        self.warning_count = ::std::option::Option::None
    }
    #[inline]
    pub fn set_warning_count(&mut self, v: i64) {
        self.warning_count = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_warning_count(&self) -> i64 {
        match self.warning_count {
            Some(v) => v,
            None => 0,
        }
    }
}
impl ::protobuf::Clear for StreamResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for StreamResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static StreamResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: StreamResponse = StreamResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl EncodeType {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [EncodeType] = &[EncodeType::TypeDefault, EncodeType::TypeArrow];
        VALUES
    }
}
