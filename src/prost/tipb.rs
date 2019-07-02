#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableInfo {
    #[prost(int64, optional, tag = "1")]
    pub table_id: ::std::option::Option<i64>,
    #[prost(message, repeated, tag = "2")]
    pub columns: ::std::vec::Vec<ColumnInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnInfo {
    #[prost(int64, optional, tag = "1")]
    pub column_id: ::std::option::Option<i64>,
    /// MySQL type.
    #[prost(int32, optional, tag = "2")]
    pub tp: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub collation: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub column_len: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub decimal: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub flag: ::std::option::Option<i32>,
    #[prost(string, repeated, tag = "7")]
    pub elems: ::std::vec::Vec<std::string::String>,
    /// Encoded datum.
    #[prost(bytes, optional, tag = "8")]
    pub default_val: ::std::option::Option<std::vec::Vec<u8>>,
    /// PK handle column value is row handle.
    #[prost(bool, optional, tag = "21")]
    pub pk_handle: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexInfo {
    #[prost(int64, optional, tag = "1")]
    pub table_id: ::std::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub index_id: ::std::option::Option<i64>,
    #[prost(message, repeated, tag = "3")]
    pub columns: ::std::vec::Vec<ColumnInfo>,
    #[prost(bool, optional, tag = "4")]
    pub unique: ::std::option::Option<bool>,
}
/// KeyRange is the encoded index key range, low is closed, high is open. (low <= x < high)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyRange {
    #[prost(bytes, optional, tag = "1")]
    pub low: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bytes, optional, tag = "2")]
    pub high: ::std::option::Option<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeReq {
    #[prost(enumeration = "AnalyzeType", optional, tag = "1")]
    pub tp: ::std::option::Option<i32>,
    #[prost(uint64, optional, tag = "2")]
    pub start_ts: ::std::option::Option<u64>,
    #[prost(uint64, optional, tag = "3")]
    pub flags: ::std::option::Option<u64>,
    #[prost(int64, optional, tag = "4")]
    pub time_zone_offset: ::std::option::Option<i64>,
    #[prost(message, optional, tag = "5")]
    pub idx_req: ::std::option::Option<AnalyzeIndexReq>,
    #[prost(message, optional, tag = "6")]
    pub col_req: ::std::option::Option<AnalyzeColumnsReq>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeIndexReq {
    /// bucket_size is the max histograms bucket size.
    #[prost(int64, optional, tag = "1")]
    pub bucket_size: ::std::option::Option<i64>,
    /// num_columns is the number of columns in the index.
    #[prost(int32, optional, tag = "2")]
    pub num_columns: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub cmsketch_depth: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub cmsketch_width: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeColumnsReq {
    /// bucket_size is the max histograms bucket size, we need this because when primary key is handle,
    /// the histogram will be directly built.
    #[prost(int64, optional, tag = "1")]
    pub bucket_size: ::std::option::Option<i64>,
    /// sample_size is the max number of samples that will be collected.
    #[prost(int64, optional, tag = "2")]
    pub sample_size: ::std::option::Option<i64>,
    /// sketch_size is the max sketch size.
    #[prost(int64, optional, tag = "3")]
    pub sketch_size: ::std::option::Option<i64>,
    /// columns_info is the info of all the columns that needs to be analyzed.
    #[prost(message, repeated, tag = "4")]
    pub columns_info: ::std::vec::Vec<ColumnInfo>,
    #[prost(int32, optional, tag = "5")]
    pub cmsketch_depth: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub cmsketch_width: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeColumnsResp {
    /// collectors is the sample collectors for columns.
    #[prost(message, repeated, tag = "1")]
    pub collectors: ::std::vec::Vec<SampleCollector>,
    /// pk_hist is the histogram for primary key when it is the handle.
    #[prost(message, optional, tag = "2")]
    pub pk_hist: ::std::option::Option<Histogram>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeIndexResp {
    #[prost(message, optional, tag = "1")]
    pub hist: ::std::option::Option<Histogram>,
    #[prost(message, optional, tag = "2")]
    pub cms: ::std::option::Option<CmSketch>,
}
/// Bucket is an element of histogram.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bucket {
    #[prost(int64, optional, tag = "1")]
    pub count: ::std::option::Option<i64>,
    #[prost(bytes, optional, tag = "2")]
    pub lower_bound: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bytes, optional, tag = "3")]
    pub upper_bound: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(int64, optional, tag = "4")]
    pub repeats: ::std::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Histogram {
    /// ndv is the number of distinct values.
    #[prost(int64, optional, tag = "1")]
    pub ndv: ::std::option::Option<i64>,
    /// buckets represents all the buckets.
    #[prost(message, repeated, tag = "2")]
    pub buckets: ::std::vec::Vec<Bucket>,
}
/// FMSketch is used to count distinct values for columns.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FmSketch {
    #[prost(uint64, optional, tag = "1")]
    pub mask: ::std::option::Option<u64>,
    #[prost(uint64, repeated, packed = "false", tag = "2")]
    pub hashset: ::std::vec::Vec<u64>,
}
/// SampleCollector is used for collect samples and calculate the count and ndv of an column.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SampleCollector {
    #[prost(bytes, repeated, tag = "1")]
    pub samples: ::std::vec::Vec<std::vec::Vec<u8>>,
    #[prost(int64, optional, tag = "2")]
    pub null_count: ::std::option::Option<i64>,
    #[prost(int64, optional, tag = "3")]
    pub count: ::std::option::Option<i64>,
    #[prost(message, optional, tag = "4")]
    pub fm_sketch: ::std::option::Option<FmSketch>,
    #[prost(message, optional, tag = "5")]
    pub cm_sketch: ::std::option::Option<CmSketch>,
    #[prost(int64, optional, tag = "6")]
    pub total_size: ::std::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CmSketchRow {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub counters: ::std::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CmSketchTopN {
    #[prost(bytes, optional, tag = "1")]
    pub data: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(uint64, optional, tag = "2")]
    pub count: ::std::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CmSketch {
    #[prost(message, repeated, tag = "1")]
    pub rows: ::std::vec::Vec<CmSketchRow>,
    #[prost(message, repeated, tag = "2")]
    pub top_n: ::std::vec::Vec<CmSketchTopN>,
    #[prost(uint64, optional, tag = "3")]
    pub default_value: ::std::option::Option<u64>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AnalyzeType {
    TypeIndex = 0,
    TypeColumn = 1,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChecksumRequest {
    #[prost(uint64, optional, tag = "1")]
    pub start_ts: ::std::option::Option<u64>,
    #[prost(enumeration = "ChecksumScanOn", optional, tag = "2")]
    pub scan_on: ::std::option::Option<i32>,
    #[prost(enumeration = "ChecksumAlgorithm", optional, tag = "3")]
    pub algorithm: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChecksumResponse {
    #[prost(uint64, optional, tag = "1")]
    pub checksum: ::std::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub total_kvs: ::std::option::Option<u64>,
    #[prost(uint64, optional, tag = "3")]
    pub total_bytes: ::std::option::Option<u64>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChecksumScanOn {
    Table = 0,
    Index = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChecksumAlgorithm {
    Crc64Xor = 0,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldType {
    #[prost(int32, optional, tag = "1")]
    pub tp: ::std::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub flag: ::std::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub flen: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub decimal: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub collate: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "6")]
    pub charset: ::std::option::Option<std::string::String>,
}
/// Evaluators should implement evaluation functions for every expression type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Expr {
    #[prost(enumeration = "ExprType", optional, tag = "1")]
    pub tp: ::std::option::Option<i32>,
    #[prost(bytes, optional, tag = "2")]
    pub val: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "3")]
    pub children: ::std::vec::Vec<Expr>,
    #[prost(enumeration = "ScalarFuncSig", optional, tag = "4")]
    pub sig: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "5")]
    pub field_type: ::std::option::Option<FieldType>,
}
/// ByItem type for group by and order by.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ByItem {
    #[prost(message, optional, tag = "1")]
    pub expr: ::std::option::Option<Expr>,
    #[prost(bool, optional, tag = "2")]
    pub desc: ::std::option::Option<bool>,
}
/// Children count 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExprType {
    /// Values are encoded bytes.
    Null = 0,
    Int64 = 1,
    Uint64 = 2,
    Float32 = 3,
    Float64 = 4,
    String = 5,
    Bytes = 6,
    /// Mysql specific types.
    MysqlBit = 101,
    MysqlDecimal = 102,
    MysqlDuration = 103,
    MysqlEnum = 104,
    MysqlHex = 105,
    MysqlSet = 106,
    MysqlTime = 107,
    MysqlJson = 108,
    /// Encoded value list.
    ValueList = 151,
    /// Column reference. value is int64 column ID.
    ColumnRef = 201,
    // Mysql functions, children count is function specific.
    /// Aggregate functions.
    Count = 3001,
    Sum = 3002,
    Avg = 3003,
    Min = 3004,
    Max = 3005,
    First = 3006,
    GroupConcat = 3007,
    AggBitAnd = 3008,
    AggBitOr = 3009,
    AggBitXor = 3010,
    Std = 3011,
    Stddev = 3012,
    StddevPop = 3013,
    StddevSamp = 3014,
    VarPop = 3015,
    VarSamp = 3016,
    Variance = 3017,
    JsonArrayAgg = 3018,
    JsonObjectAgg = 3019,
    /// Scalar Function
    ScalarFunc = 10000,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ScalarFuncSig {
    /// Casting
    CastIntAsInt = 0,
    CastIntAsReal = 1,
    CastIntAsString = 2,
    CastIntAsDecimal = 3,
    CastIntAsTime = 4,
    CastIntAsDuration = 5,
    CastIntAsJson = 6,
    CastRealAsInt = 10,
    CastRealAsReal = 11,
    CastRealAsString = 12,
    CastRealAsDecimal = 13,
    CastRealAsTime = 14,
    CastRealAsDuration = 15,
    CastRealAsJson = 16,
    CastDecimalAsInt = 20,
    CastDecimalAsReal = 21,
    CastDecimalAsString = 22,
    CastDecimalAsDecimal = 23,
    CastDecimalAsTime = 24,
    CastDecimalAsDuration = 25,
    CastDecimalAsJson = 26,
    CastStringAsInt = 30,
    CastStringAsReal = 31,
    CastStringAsString = 32,
    CastStringAsDecimal = 33,
    CastStringAsTime = 34,
    CastStringAsDuration = 35,
    CastStringAsJson = 36,
    CastTimeAsInt = 40,
    CastTimeAsReal = 41,
    CastTimeAsString = 42,
    CastTimeAsDecimal = 43,
    CastTimeAsTime = 44,
    CastTimeAsDuration = 45,
    CastTimeAsJson = 46,
    CastDurationAsInt = 50,
    CastDurationAsReal = 51,
    CastDurationAsString = 52,
    CastDurationAsDecimal = 53,
    CastDurationAsTime = 54,
    CastDurationAsDuration = 55,
    CastDurationAsJson = 56,
    CastJsonAsInt = 60,
    CastJsonAsReal = 61,
    CastJsonAsString = 62,
    CastJsonAsDecimal = 63,
    CastJsonAsTime = 64,
    CastJsonAsDuration = 65,
    CastJsonAsJson = 66,
    ///compare
    CoalesceInt = 4201,
    CoalesceReal = 4202,
    CoalesceDecimal = 4203,
    CoalesceString = 4204,
    CoalesceTime = 4205,
    CoalesceDuration = 4206,
    /// unimplemented in tidb
    CoalesceJson = 4207,
    LtInt = 100,
    LtReal = 101,
    LtDecimal = 102,
    LtString = 103,
    LtTime = 104,
    LtDuration = 105,
    LtJson = 106,
    LeInt = 110,
    LeReal = 111,
    LeDecimal = 112,
    LeString = 113,
    LeTime = 114,
    LeDuration = 115,
    LeJson = 116,
    GtInt = 120,
    GtReal = 121,
    GtDecimal = 122,
    GtString = 123,
    GtTime = 124,
    GtDuration = 125,
    GtJson = 126,
    GreatestInt = 4215,
    GreatestReal = 4216,
    GreatestDecimal = 4217,
    GreatestString = 4218,
    GreatestTime = 4219,
    LeastInt = 4220,
    LeastReal = 4221,
    LeastDecimal = 4222,
    LeastString = 4223,
    LeastTime = 4224,
    IntervalInt = 4225,
    IntervalReal = 4226,
    GeInt = 130,
    GeReal = 131,
    GeDecimal = 132,
    GeString = 133,
    GeTime = 134,
    GeDuration = 135,
    GeJson = 136,
    EqInt = 140,
    EqReal = 141,
    EqDecimal = 142,
    EqString = 143,
    EqTime = 144,
    EqDuration = 145,
    EqJson = 146,
    NeInt = 150,
    NeReal = 151,
    NeDecimal = 152,
    NeString = 153,
    NeTime = 154,
    NeDuration = 155,
    NeJson = 156,
    NullEqInt = 160,
    NullEqReal = 161,
    NullEqDecimal = 162,
    NullEqString = 163,
    NullEqTime = 164,
    NullEqDuration = 165,
    NullEqJson = 166,
    ///arithmetic
    PlusReal = 200,
    PlusDecimal = 201,
    PlusInt = 203,
    MinusReal = 204,
    MinusDecimal = 205,
    MinusInt = 207,
    MultiplyReal = 208,
    MultiplyDecimal = 209,
    MultiplyInt = 210,
    DivideReal = 211,
    DivideDecimal = 212,
    IntDivideInt = 213,
    IntDivideDecimal = 214,
    ModReal = 215,
    ModDecimal = 216,
    ModInt = 217,
    MultiplyIntUnsigned = 218,
    ///math
    AbsInt = 2101,
    AbsUInt = 2102,
    AbsReal = 2103,
    AbsDecimal = 2104,
    CeilIntToDec = 2105,
    CeilIntToInt = 2106,
    CeilDecToInt = 2107,
    CeilDecToDec = 2108,
    CeilReal = 2109,
    FloorIntToDec = 2110,
    FloorIntToInt = 2111,
    FloorDecToInt = 2112,
    FloorDecToDec = 2113,
    FloorReal = 2114,
    RoundReal = 2121,
    RoundInt = 2122,
    RoundDec = 2123,
    RoundWithFracReal = 2124,
    RoundWithFracInt = 2125,
    RoundWithFracDec = 2126,
    Log1Arg = 2131,
    Log2Args = 2132,
    Log2 = 2133,
    Log10 = 2134,
    Rand = 2135,
    RandWithSeed = 2136,
    Pow = 2137,
    Conv = 2138,
    Crc32 = 2139,
    Sign = 2140,
    Sqrt = 2141,
    Acos = 2142,
    Asin = 2143,
    Atan1Arg = 2144,
    Atan2Args = 2145,
    Cos = 2146,
    Cot = 2147,
    Degrees = 2148,
    Exp = 2149,
    Pi = 2150,
    Radians = 2151,
    Sin = 2152,
    Tan = 2153,
    TruncateInt = 2154,
    TruncateReal = 2155,
    TruncateDecimal = 2156,
    ///op
    LogicalAnd = 3101,
    LogicalOr = 3102,
    LogicalXor = 3103,
    UnaryNot = 3104,
    UnaryMinusInt = 3108,
    UnaryMinusReal = 3109,
    UnaryMinusDecimal = 3110,
    DecimalIsNull = 3111,
    DurationIsNull = 3112,
    RealIsNull = 3113,
    StringIsNull = 3114,
    TimeIsNull = 3115,
    IntIsNull = 3116,
    /// unimplemented in tidb
    JsonIsNull = 3117,
    BitAndSig = 3118,
    BitOrSig = 3119,
    BitXorSig = 3120,
    BitNegSig = 3121,
    IntIsTrue = 3122,
    RealIsTrue = 3123,
    DecimalIsTrue = 3124,
    IntIsFalse = 3125,
    RealIsFalse = 3126,
    DecimalIsFalse = 3127,
    LeftShift = 3129,
    RightShift = 3130,
    ///other
    BitCount = 3128,
    GetParamString = 3131,
    GetVar = 3132,
    RowSig = 3133,
    SetVar = 3134,
    ValuesDecimal = 3135,
    ValuesDuration = 3136,
    ValuesInt = 3137,
    ValuesJson = 3138,
    ValuesReal = 3139,
    ValuesString = 3140,
    ValuesTime = 3141,
    InInt = 4001,
    InReal = 4002,
    InDecimal = 4003,
    InString = 4004,
    InTime = 4005,
    InDuration = 4006,
    InJson = 4007,
    ///control
    IfNullInt = 4101,
    IfNullReal = 4102,
    IfNullDecimal = 4103,
    IfNullString = 4104,
    IfNullTime = 4105,
    IfNullDuration = 4106,
    IfInt = 4107,
    IfReal = 4108,
    IfDecimal = 4109,
    IfString = 4110,
    IfTime = 4111,
    IfDuration = 4112,
    IfNullJson = 4113,
    IfJson = 4114,
    CaseWhenInt = 4208,
    CaseWhenReal = 4209,
    CaseWhenDecimal = 4210,
    CaseWhenString = 4211,
    CaseWhenTime = 4212,
    CaseWhenDuration = 4213,
    /// unimplemented in tidb
    CaseWhenJson = 4214,
    /// encryption
    AesDecrypt = 4501,
    AesEncrypt = 4502,
    Compress = 4503,
    Md5 = 4504,
    Password = 4505,
    RandomBytes = 4506,
    Sha1 = 4507,
    Sha2 = 4508,
    Uncompress = 4509,
    UncompressedLength = 4510,
    ///info
    Database = 4521,
    FoundRows = 4522,
    CurrentUser = 4523,
    User = 4524,
    ConnectionId = 4525,
    LastInsertId = 4526,
    LastInsertIdWithId = 4527,
    Version = 4528,
    TiDbVersion = 4529,
    RowCount = 4530,
    ///miscellaneous
    Sleep = 4551,
    Lock = 4552,
    ReleaseLock = 4553,
    DecimalAnyValue = 4554,
    DurationAnyValue = 4555,
    IntAnyValue = 4556,
    JsonAnyValue = 4557,
    RealAnyValue = 4558,
    StringAnyValue = 4559,
    TimeAnyValue = 4560,
    InetAton = 4561,
    InetNtoa = 4562,
    Inet6Aton = 4563,
    Inet6Ntoa = 4564,
    IsIPv4 = 4565,
    IsIPv4Compat = 4566,
    IsIPv4Mapped = 4567,
    IsIPv6 = 4568,
    Uuid = 4569,
    ///like
    LikeSig = 4310,
    RegexpBinarySig = 4311,
    RegexpSig = 4312,
    ///json
    JsonExtractSig = 5001,
    JsonUnquoteSig = 5002,
    JsonTypeSig = 5003,
    JsonSetSig = 5004,
    JsonInsertSig = 5005,
    JsonReplaceSig = 5006,
    JsonRemoveSig = 5007,
    JsonMergeSig = 5008,
    JsonObjectSig = 5009,
    JsonArraySig = 5010,
    JsonValidJsonSig = 5011,
    JsonContainsSig = 5012,
    JsonArrayAppendSig = 5013,
    JsonArrayInsertSig = 5014,
    JsonMergePatchSig = 5015,
    JsonMergePreserveSig = 5016,
    JsonContainsPathSig = 5017,
    JsonPrettySig = 5018,
    JsonQuoteSig = 5019,
    JsonSearchSig = 5020,
    JsonStorageSizeSig = 5021,
    JsonDepthSig = 5022,
    JsonKeysSig = 5023,
    JsonLengthSig = 5024,
    JsonKeys2ArgsSig = 5025,
    JsonValidStringSig = 5026,
    ///time
    DateFormatSig = 6001,
    DateLiteral = 6002,
    DateDiff = 6003,
    NullTimeDiff = 6004,
    TimeStringTimeDiff = 6005,
    DurationStringTimeDiff = 6006,
    DurationDurationTimeDiff = 6007,
    StringTimeTimeDiff = 6008,
    StringDurationTimeDiff = 6009,
    StringStringTimeDiff = 6010,
    TimeTimeTimeDiff = 6011,
    Date = 6012,
    Hour = 6013,
    Minute = 6014,
    Second = 6015,
    MicroSecond = 6016,
    Month = 6017,
    MonthName = 6018,
    NowWithArg = 6019,
    NowWithoutArg = 6020,
    DayName = 6021,
    DayOfMonth = 6022,
    DayOfWeek = 6023,
    DayOfYear = 6024,
    WeekWithMode = 6025,
    WeekWithoutMode = 6026,
    WeekDay = 6027,
    WeekOfYear = 6028,
    Year = 6029,
    YearWeekWithMode = 6030,
    YearWeekWithoutMode = 6031,
    GetFormat = 6032,
    SysDateWithFsp = 6033,
    SysDateWithoutFsp = 6034,
    CurrentDate = 6035,
    CurrentTime0Arg = 6036,
    CurrentTime1Arg = 6037,
    Time = 6038,
    TimeLiteral = 6039,
    UtcDate = 6040,
    UtcTimestampWithArg = 6041,
    UtcTimestampWithoutArg = 6042,
    AddDatetimeAndDuration = 6043,
    AddDatetimeAndString = 6044,
    AddTimeDateTimeNull = 6045,
    AddStringAndDuration = 6046,
    AddStringAndString = 6047,
    AddTimeStringNull = 6048,
    AddDurationAndDuration = 6049,
    AddDurationAndString = 6050,
    AddTimeDurationNull = 6051,
    AddDateAndDuration = 6052,
    AddDateAndString = 6053,
    SubDatetimeAndDuration = 6054,
    SubDatetimeAndString = 6055,
    SubTimeDateTimeNull = 6056,
    SubStringAndDuration = 6057,
    SubStringAndString = 6058,
    SubTimeStringNull = 6059,
    SubDurationAndDuration = 6060,
    SubDurationAndString = 6061,
    SubTimeDurationNull = 6062,
    SubDateAndDuration = 6063,
    SubDateAndString = 6064,
    UnixTimestampCurrent = 6065,
    UnixTimestampInt = 6066,
    UnixTimestampDec = 6067,
    ConvertTz = 6068,
    MakeDate = 6069,
    MakeTime = 6070,
    PeriodAdd = 6071,
    PeriodDiff = 6072,
    Quarter = 6073,
    SecToTime = 6074,
    TimeToSec = 6075,
    TimestampAdd = 6076,
    ToDays = 6077,
    ToSeconds = 6078,
    UtcTimeWithArg = 6079,
    UtcTimeWithoutArg = 6080,
    Timestamp1Arg = 6081,
    Timestamp2Args = 6082,
    TimestampLiteral = 6083,
    LastDay = 6084,
    StrToDateDate = 6085,
    StrToDateDatetime = 6086,
    StrToDateDuration = 6087,
    FromUnixTime1Arg = 6088,
    FromUnixTime2Arg = 6089,
    ExtractDatetime = 6090,
    ExtractDuration = 6091,
    AddDateStringString = 6092,
    AddDateStringInt = 6093,
    AddDateStringDecimal = 6094,
    AddDateIntString = 6095,
    AddDateIntInt = 6096,
    AddDateDatetimeString = 6097,
    AddDateDatetimeInt = 6098,
    SubDateStringString = 6099,
    SubDateStringInt = 6100,
    SubDateStringDecimal = 6101,
    SubDateIntString = 6102,
    SubDateIntInt = 6103,
    SubDateDatetimeString = 6104,
    SubDateDatetimeInt = 6105,
    FromDays = 6106,
    TimeFormat = 6107,
    TimestampDiff = 6108,
    /// String functions
    BitLength = 7001,
    Bin = 7002,
    Ascii = 7003,
    Char = 7004,
    CharLength = 7005,
    Concat = 7006,
    ConcatWs = 7007,
    Convert = 7008,
    Elt = 7009,
    ExportSet3Arg = 7010,
    ExportSet4Arg = 7011,
    ExportSet5Arg = 7012,
    FieldInt = 7013,
    FieldReal = 7014,
    FieldString = 7015,
    FindInSet = 7016,
    Format = 7017,
    FormatWithLocale = 7018,
    FromBase64 = 7019,
    HexIntArg = 7020,
    HexStrArg = 7021,
    Insert = 7022,
    InsertBinary = 7023,
    Instr = 7024,
    InstrBinary = 7025,
    LTrim = 7026,
    Left = 7027,
    LeftBinary = 7028,
    Length = 7029,
    Locate2Args = 7030,
    Locate3Args = 7031,
    LocateBinary2Args = 7032,
    LocateBinary3Args = 7033,
    Lower = 7034,
    Lpad = 7035,
    LpadBinary = 7036,
    MakeSet = 7037,
    OctInt = 7038,
    OctString = 7039,
    Ord = 7040,
    Quote = 7041,
    RTrim = 7042,
    Repeat = 7043,
    Replace = 7044,
    Reverse = 7045,
    ReverseBinary = 7046,
    Right = 7047,
    RightBinary = 7048,
    Rpad = 7049,
    RpadBinary = 7050,
    Space = 7051,
    Strcmp = 7052,
    Substring2Args = 7053,
    Substring3Args = 7054,
    SubstringBinary2Args = 7055,
    SubstringBinary3Args = 7056,
    SubstringIndex = 7057,
    ToBase64 = 7058,
    Trim1Arg = 7059,
    Trim2Args = 7060,
    Trim3Args = 7061,
    UnHex = 7062,
    Upper = 7063,
}
/// It represents a Executor.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Executor {
    #[prost(enumeration = "ExecType", optional, tag = "1")]
    pub tp: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub tbl_scan: ::std::option::Option<TableScan>,
    #[prost(message, optional, tag = "3")]
    pub idx_scan: ::std::option::Option<IndexScan>,
    #[prost(message, optional, tag = "4")]
    pub selection: ::std::option::Option<Selection>,
    #[prost(message, optional, tag = "5")]
    pub aggregation: ::std::option::Option<Aggregation>,
    #[prost(message, optional, tag = "6")]
    pub top_n: ::std::option::Option<TopN>,
    #[prost(message, optional, tag = "7")]
    pub limit: ::std::option::Option<Limit>,
    #[prost(message, optional, tag = "8")]
    pub stream_agg: ::std::option::Option<Aggregation>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableScan {
    #[prost(int64, optional, tag = "1")]
    pub table_id: ::std::option::Option<i64>,
    #[prost(message, repeated, tag = "2")]
    pub columns: ::std::vec::Vec<ColumnInfo>,
    #[prost(bool, optional, tag = "3")]
    pub desc: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexScan {
    #[prost(int64, optional, tag = "1")]
    pub table_id: ::std::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub index_id: ::std::option::Option<i64>,
    #[prost(message, repeated, tag = "3")]
    pub columns: ::std::vec::Vec<ColumnInfo>,
    #[prost(bool, optional, tag = "4")]
    pub desc: ::std::option::Option<bool>,
    /// check whether it is a unique index.
    #[prost(bool, optional, tag = "5")]
    pub unique: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Selection {
    /// Where conditions.
    #[prost(message, repeated, tag = "1")]
    pub conditions: ::std::vec::Vec<Expr>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Projection {
    /// Projection expressions.
    #[prost(message, repeated, tag = "1")]
    pub exprs: ::std::vec::Vec<Expr>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Aggregation {
    /// Group by clause.
    #[prost(message, repeated, tag = "1")]
    pub group_by: ::std::vec::Vec<Expr>,
    /// Aggregate functions.
    #[prost(message, repeated, tag = "2")]
    pub agg_func: ::std::vec::Vec<Expr>,
    /// If it is a stream aggregation.
    #[prost(bool, optional, tag = "3")]
    pub streamed: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopN {
    /// Order by clause.
    #[prost(message, repeated, tag = "1")]
    pub order_by: ::std::vec::Vec<ByItem>,
    #[prost(uint64, optional, tag = "2")]
    pub limit: ::std::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Limit {
    /// Limit the result to be returned.
    #[prost(uint64, optional, tag = "1")]
    pub limit: ::std::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutorExecutionSummary {
    /// Total time cost in this executor. Includes self time cost and children time cost.
    #[prost(uint64, optional, tag = "1")]
    pub time_processed_ns: ::std::option::Option<u64>,
    /// How many rows this executor produced totally.
    #[prost(uint64, optional, tag = "2")]
    pub num_produced_rows: ::std::option::Option<u64>,
    /// How many times executor's `next()` is called.
    #[prost(uint64, optional, tag = "3")]
    pub num_iterations: ::std::option::Option<u64>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecType {
    TypeTableScan = 0,
    TypeIndexScan = 1,
    TypeSelection = 2,
    /// TODO: Rename it to hash aggregation after support stream aggregation in TiKV.
    TypeAggregation = 3,
    TypeTopN = 4,
    TypeLimit = 5,
    TypeStreamAgg = 6,
}
/// values are all in text format.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Row {
    #[prost(bytes, optional, tag = "1")]
    pub handle: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bytes, optional, tag = "2")]
    pub data: ::std::option::Option<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(int32, optional, tag = "1")]
    pub code: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub msg: ::std::option::Option<std::string::String>,
}
/// Response for SelectRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    /// Result rows.
    #[prost(message, repeated, tag = "2")]
    pub rows: ::std::vec::Vec<Row>,
    /// Use multiple chunks to reduce memory allocation and
    /// avoid allocating large contiguous memory.
    #[prost(message, repeated, tag = "3")]
    pub chunks: ::std::vec::Vec<Chunk>,
    #[prost(message, repeated, tag = "4")]
    pub warnings: ::std::vec::Vec<Error>,
    #[prost(int64, repeated, packed = "false", tag = "5")]
    pub output_counts: ::std::vec::Vec<i64>,
    #[prost(int64, optional, tag = "6")]
    pub warning_count: ::std::option::Option<i64>,
    #[prost(bytes, optional, tag = "7")]
    pub row_batch_data: ::std::option::Option<std::vec::Vec<u8>>,
    /// The execution summary of each executor, in the order in request.
    #[prost(message, repeated, tag = "8")]
    pub execution_summaries: ::std::vec::Vec<ExecutorExecutionSummary>,
}
/// Chunk contains multiple rows data and rows meta.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chunk {
    /// Data for all rows in the chunk.
    #[prost(bytes, optional, tag = "3")]
    pub rows_data: ::std::option::Option<std::vec::Vec<u8>>,
    /// Meta data for every row.
    #[prost(message, repeated, tag = "4")]
    pub rows_meta: ::std::vec::Vec<RowMeta>,
}
/// RowMeta contains row handle and length of a row.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowMeta {
    #[prost(int64, optional, tag = "1")]
    pub handle: ::std::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub length: ::std::option::Option<i64>,
}
/// DAGRequest represents the request that will be handled with DAG mode.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DagRequest {
    /// Transaction start timestamp.
    #[prost(uint64, optional, tag = "1")]
    pub start_ts: ::std::option::Option<u64>,
    /// It represents push down Executors.
    #[prost(message, repeated, tag = "2")]
    pub executors: ::std::vec::Vec<Executor>,
    /// time zone offset in seconds
    #[prost(int64, optional, tag = "3")]
    pub time_zone_offset: ::std::option::Option<i64>,
    /// flags are used to store flags that change the execution mode, it contains:
    ///	ignore_truncate = 1
    ///		truncate error should be ignore if set.
    ///	truncate_as_warning = 1 << 1
    ///		when ignored_truncate is not set, return warning instead of error if this flag is set.
    ///	...
    ///	add more when needed.
    #[prost(uint64, optional, tag = "4")]
    pub flags: ::std::option::Option<u64>,
    /// It represents which columns we should output.
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub output_offsets: ::std::vec::Vec<u32>,
    /// It represents whether we collect the detailed scan counts in each range.
    #[prost(bool, optional, tag = "6")]
    pub collect_range_counts: ::std::option::Option<bool>,
    /// It indicates the maximum number of warning,
    /// which is the number of messages that SHOW WARNINGS displays.
    #[prost(uint64, optional, tag = "7")]
    pub max_warning_count: ::std::option::Option<u64>,
    /// It indicates the encode type of response.
    #[prost(enumeration = "EncodeType", optional, tag = "8")]
    pub encode_type: ::std::option::Option<i32>,
    /// It indicates the sql_mode.
    #[prost(uint64, optional, tag = "9")]
    pub sql_mode: ::std::option::Option<u64>,
    // It indicates whether the sql mode is strict.
    // Deprecated. Don't use.
    // optional bool is_strict_sql_mode = 10;
    /// supply offset is not enough since we have daylight saving time present in some regions
    #[prost(string, optional, tag = "11")]
    pub time_zone_name: ::std::option::Option<std::string::String>,
    /// It represents whether or not TiKV should collect execution summaries.
    /// Execution summaries will be collected into `execution_summaries` field
    /// in the response.
    #[prost(bool, optional, tag = "12")]
    pub collect_execution_summaries: ::std::option::Option<bool>,
    /// Represents the maximum size of one packet, any generated string, or any parameter sent as long data.
    #[prost(uint64, optional, tag = "13")]
    pub max_allowed_packet: ::std::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    /// Data for all rows
    #[prost(bytes, optional, tag = "3")]
    pub data: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "4")]
    pub warnings: ::std::vec::Vec<Error>,
    /// output row count for each executor
    #[prost(int64, repeated, packed = "false", tag = "5")]
    pub output_counts: ::std::vec::Vec<i64>,
    #[prost(int64, optional, tag = "6")]
    pub warning_count: ::std::option::Option<i64>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EncodeType {
    TypeDefault = 0,
    TypeArrow = 1,
}
