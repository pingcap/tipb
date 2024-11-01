// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: checksum.proto

package tipb

import (
	fmt "fmt"
	io "io"
	math "math"
	math_bits "math/bits"

	_ "github.com/gogo/protobuf/gogoproto"
	proto "github.com/golang/protobuf/proto"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.ProtoPackageIsVersion3 // please upgrade the proto package

type ChecksumScanOn int32

const (
	ChecksumScanOn_Table ChecksumScanOn = 0
	ChecksumScanOn_Index ChecksumScanOn = 1
)

var ChecksumScanOn_name = map[int32]string{
	0: "Table",
	1: "Index",
}

var ChecksumScanOn_value = map[string]int32{
	"Table": 0,
	"Index": 1,
}

func (x ChecksumScanOn) Enum() *ChecksumScanOn {
	p := new(ChecksumScanOn)
	*p = x
	return p
}

func (x ChecksumScanOn) String() string {
	return proto.EnumName(ChecksumScanOn_name, int32(x))
}

func (x *ChecksumScanOn) UnmarshalJSON(data []byte) error {
	value, err := proto.UnmarshalJSONEnum(ChecksumScanOn_value, data, "ChecksumScanOn")
	if err != nil {
		return err
	}
	*x = ChecksumScanOn(value)
	return nil
}

func (ChecksumScanOn) EnumDescriptor() ([]byte, []int) {
	return fileDescriptor_843938c28b799986, []int{0}
}

type ChecksumAlgorithm int32

const (
	ChecksumAlgorithm_Crc64_Xor ChecksumAlgorithm = 0
)

var ChecksumAlgorithm_name = map[int32]string{
	0: "Crc64_Xor",
}

var ChecksumAlgorithm_value = map[string]int32{
	"Crc64_Xor": 0,
}

func (x ChecksumAlgorithm) Enum() *ChecksumAlgorithm {
	p := new(ChecksumAlgorithm)
	*p = x
	return p
}

func (x ChecksumAlgorithm) String() string {
	return proto.EnumName(ChecksumAlgorithm_name, int32(x))
}

func (x *ChecksumAlgorithm) UnmarshalJSON(data []byte) error {
	value, err := proto.UnmarshalJSONEnum(ChecksumAlgorithm_value, data, "ChecksumAlgorithm")
	if err != nil {
		return err
	}
	*x = ChecksumAlgorithm(value)
	return nil
}

func (ChecksumAlgorithm) EnumDescriptor() ([]byte, []int) {
	return fileDescriptor_843938c28b799986, []int{1}
}

type ChecksumRewriteRule struct {
	OldPrefix []byte `protobuf:"bytes,1,opt,name=old_prefix,json=oldPrefix" json:"old_prefix,omitempty"`
	NewPrefix []byte `protobuf:"bytes,2,opt,name=new_prefix,json=newPrefix" json:"new_prefix,omitempty"`
}

func (m *ChecksumRewriteRule) Reset()         { *m = ChecksumRewriteRule{} }
func (m *ChecksumRewriteRule) String() string { return proto.CompactTextString(m) }
func (*ChecksumRewriteRule) ProtoMessage()    {}
func (*ChecksumRewriteRule) Descriptor() ([]byte, []int) {
	return fileDescriptor_843938c28b799986, []int{0}
}
func (m *ChecksumRewriteRule) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *ChecksumRewriteRule) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_ChecksumRewriteRule.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *ChecksumRewriteRule) XXX_Merge(src proto.Message) {
	xxx_messageInfo_ChecksumRewriteRule.Merge(m, src)
}
func (m *ChecksumRewriteRule) XXX_Size() int {
	return m.Size()
}
func (m *ChecksumRewriteRule) XXX_DiscardUnknown() {
	xxx_messageInfo_ChecksumRewriteRule.DiscardUnknown(m)
}

var xxx_messageInfo_ChecksumRewriteRule proto.InternalMessageInfo

func (m *ChecksumRewriteRule) GetOldPrefix() []byte {
	if m != nil {
		return m.OldPrefix
	}
	return nil
}

func (m *ChecksumRewriteRule) GetNewPrefix() []byte {
	if m != nil {
		return m.NewPrefix
	}
	return nil
}

type ChecksumRequest struct {
	// Deprecated. Start Ts has been moved to coprocessor.Request.
	StartTsFallback *uint64              `protobuf:"varint,1,opt,name=start_ts_fallback,json=startTsFallback" json:"start_ts_fallback,omitempty"`
	ScanOn          ChecksumScanOn       `protobuf:"varint,2,opt,name=scan_on,json=scanOn,enum=tipb.ChecksumScanOn" json:"scan_on"`
	Algorithm       ChecksumAlgorithm    `protobuf:"varint,3,opt,name=algorithm,enum=tipb.ChecksumAlgorithm" json:"algorithm"`
	Rule            *ChecksumRewriteRule `protobuf:"bytes,4,opt,name=rule" json:"rule,omitempty"`
}

func (m *ChecksumRequest) Reset()         { *m = ChecksumRequest{} }
func (m *ChecksumRequest) String() string { return proto.CompactTextString(m) }
func (*ChecksumRequest) ProtoMessage()    {}
func (*ChecksumRequest) Descriptor() ([]byte, []int) {
	return fileDescriptor_843938c28b799986, []int{1}
}
func (m *ChecksumRequest) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *ChecksumRequest) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_ChecksumRequest.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *ChecksumRequest) XXX_Merge(src proto.Message) {
	xxx_messageInfo_ChecksumRequest.Merge(m, src)
}
func (m *ChecksumRequest) XXX_Size() int {
	return m.Size()
}
func (m *ChecksumRequest) XXX_DiscardUnknown() {
	xxx_messageInfo_ChecksumRequest.DiscardUnknown(m)
}

var xxx_messageInfo_ChecksumRequest proto.InternalMessageInfo

func (m *ChecksumRequest) GetStartTsFallback() uint64 {
	if m != nil && m.StartTsFallback != nil {
		return *m.StartTsFallback
	}
	return 0
}

func (m *ChecksumRequest) GetScanOn() ChecksumScanOn {
	if m != nil {
		return m.ScanOn
	}
	return ChecksumScanOn_Table
}

func (m *ChecksumRequest) GetAlgorithm() ChecksumAlgorithm {
	if m != nil {
		return m.Algorithm
	}
	return ChecksumAlgorithm_Crc64_Xor
}

func (m *ChecksumRequest) GetRule() *ChecksumRewriteRule {
	if m != nil {
		return m.Rule
	}
	return nil
}

type ChecksumResponse struct {
	Checksum   uint64 `protobuf:"varint,1,opt,name=checksum" json:"checksum"`
	TotalKvs   uint64 `protobuf:"varint,2,opt,name=total_kvs,json=totalKvs" json:"total_kvs"`
	TotalBytes uint64 `protobuf:"varint,3,opt,name=total_bytes,json=totalBytes" json:"total_bytes"`
}

func (m *ChecksumResponse) Reset()         { *m = ChecksumResponse{} }
func (m *ChecksumResponse) String() string { return proto.CompactTextString(m) }
func (*ChecksumResponse) ProtoMessage()    {}
func (*ChecksumResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_843938c28b799986, []int{2}
}
func (m *ChecksumResponse) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *ChecksumResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_ChecksumResponse.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *ChecksumResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_ChecksumResponse.Merge(m, src)
}
func (m *ChecksumResponse) XXX_Size() int {
	return m.Size()
}
func (m *ChecksumResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_ChecksumResponse.DiscardUnknown(m)
}

var xxx_messageInfo_ChecksumResponse proto.InternalMessageInfo

func (m *ChecksumResponse) GetChecksum() uint64 {
	if m != nil {
		return m.Checksum
	}
	return 0
}

func (m *ChecksumResponse) GetTotalKvs() uint64 {
	if m != nil {
		return m.TotalKvs
	}
	return 0
}

func (m *ChecksumResponse) GetTotalBytes() uint64 {
	if m != nil {
		return m.TotalBytes
	}
	return 0
}

func init() {
	proto.RegisterEnum("tipb.ChecksumScanOn", ChecksumScanOn_name, ChecksumScanOn_value)
	proto.RegisterEnum("tipb.ChecksumAlgorithm", ChecksumAlgorithm_name, ChecksumAlgorithm_value)
	proto.RegisterType((*ChecksumRewriteRule)(nil), "tipb.ChecksumRewriteRule")
	proto.RegisterType((*ChecksumRequest)(nil), "tipb.ChecksumRequest")
	proto.RegisterType((*ChecksumResponse)(nil), "tipb.ChecksumResponse")
}

func init() { proto.RegisterFile("checksum.proto", fileDescriptor_843938c28b799986) }

var fileDescriptor_843938c28b799986 = []byte{
	// 422 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x64, 0x91, 0x41, 0x6b, 0xd4, 0x40,
	0x18, 0x86, 0x33, 0x1a, 0xb5, 0xf9, 0xaa, 0xbb, 0xe9, 0x58, 0x71, 0x2d, 0x18, 0xd7, 0x80, 0x52,
	0x16, 0x8c, 0x50, 0x45, 0x10, 0x4f, 0x6e, 0x41, 0x10, 0x0f, 0x96, 0xb4, 0x07, 0x6f, 0x61, 0x32,
	0x99, 0xa6, 0x61, 0x67, 0x67, 0xe2, 0xcc, 0xa4, 0x5b, 0x2f, 0xfe, 0x06, 0x8f, 0xfe, 0x04, 0x7f,
	0x4a, 0x8f, 0x3d, 0xf6, 0x54, 0x64, 0xf3, 0x47, 0x24, 0x93, 0xa4, 0xeb, 0xd2, 0xdb, 0x97, 0xf7,
	0x79, 0xdf, 0x2f, 0xbc, 0xdf, 0xc0, 0x80, 0x9e, 0x30, 0x3a, 0xd3, 0xd5, 0x3c, 0x2a, 0x95, 0x34,
	0x12, 0xbb, 0xa6, 0x28, 0xd3, 0x9d, 0xed, 0x5c, 0xe6, 0xd2, 0x0a, 0xaf, 0x9b, 0xa9, 0x65, 0x3b,
	0x43, 0x55, 0x69, 0x63, 0xc7, 0x56, 0x08, 0x0f, 0xe1, 0xe1, 0x7e, 0x17, 0x8f, 0xd9, 0x42, 0x15,
	0x86, 0xc5, 0x15, 0x67, 0xf8, 0x29, 0x80, 0xe4, 0x59, 0x52, 0x2a, 0x76, 0x5c, 0x9c, 0x8d, 0xd0,
	0x18, 0xed, 0xde, 0x8f, 0x3d, 0xc9, 0xb3, 0x03, 0x2b, 0x34, 0x58, 0xb0, 0x45, 0x8f, 0x6f, 0xb5,
	0x58, 0xb0, 0x45, 0x8b, 0xc3, 0x2b, 0x04, 0xc3, 0xd5, 0xd6, 0xef, 0x15, 0xd3, 0x06, 0x4f, 0x60,
	0x4b, 0x1b, 0xa2, 0x4c, 0x62, 0x74, 0x72, 0x4c, 0x38, 0x4f, 0x09, 0x9d, 0xd9, 0xc5, 0x6e, 0x3c,
	0xb4, 0xe0, 0x48, 0x7f, 0xea, 0x64, 0xfc, 0x06, 0xee, 0x69, 0x4a, 0x44, 0x22, 0x85, 0xdd, 0x3d,
	0xd8, 0xdb, 0x8e, 0x9a, 0x4e, 0x51, 0xbf, 0xf3, 0x90, 0x12, 0xf1, 0x55, 0x4c, 0xdd, 0xf3, 0xab,
	0x67, 0x4e, 0x7c, 0x57, 0xdb, 0x2f, 0xfc, 0x01, 0x3c, 0xc2, 0x73, 0xa9, 0x0a, 0x73, 0x32, 0x1f,
	0xdd, 0xb6, 0xb1, 0xc7, 0xeb, 0xb1, 0x8f, 0x3d, 0xee, 0x92, 0x2b, 0x3f, 0x7e, 0x05, 0xae, 0xaa,
	0x38, 0x1b, 0xb9, 0x63, 0xb4, 0xbb, 0xb9, 0xf7, 0x64, 0x3d, 0xf7, 0xdf, 0x61, 0x62, 0x6b, 0x0b,
	0x7f, 0x82, 0xbf, 0x82, 0xba, 0x94, 0x42, 0x33, 0x3c, 0x86, 0x8d, 0xfe, 0x21, 0xda, 0x5e, 0xdd,
	0x5f, 0xae, 0x55, 0xfc, 0x1c, 0x3c, 0x23, 0x0d, 0xe1, 0xc9, 0xec, 0x54, 0xdb, 0x62, 0xd7, 0x16,
	0x2b, 0x7f, 0x39, 0xd5, 0xf8, 0x05, 0x6c, 0xb6, 0x96, 0xf4, 0x87, 0x61, 0xda, 0xd6, 0xe8, 0x4d,
	0x60, 0xc1, 0xb4, 0xd1, 0x27, 0x2f, 0x61, 0xb0, 0x7e, 0x0b, 0xec, 0xc1, 0x9d, 0x23, 0x92, 0x72,
	0xe6, 0x3b, 0xcd, 0xf8, 0x59, 0x64, 0xec, 0xcc, 0x47, 0x93, 0x10, 0xb6, 0x6e, 0x94, 0xc7, 0x0f,
	0xc0, 0xdb, 0x57, 0xf4, 0xdd, 0xdb, 0xe4, 0x9b, 0x54, 0xbe, 0x33, 0x7d, 0x7f, 0xf9, 0x67, 0x03,
	0x9d, 0x2f, 0x03, 0x74, 0xb1, 0x0c, 0xd0, 0xdf, 0x65, 0x80, 0x7e, 0xd5, 0x81, 0xf3, 0xbb, 0x0e,
	0x9c, 0x8b, 0x3a, 0x70, 0x2e, 0xeb, 0xc0, 0x81, 0x47, 0x54, 0xce, 0xa3, 0xb2, 0x10, 0x39, 0x25,
	0x65, 0x64, 0x8a, 0x2c, 0xb5, 0xe7, 0x39, 0x40, 0xff, 0x02, 0x00, 0x00, 0xff, 0xff, 0xa3, 0xe1,
	0xab, 0x6b, 0x7a, 0x02, 0x00, 0x00,
}

func (m *ChecksumRewriteRule) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *ChecksumRewriteRule) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *ChecksumRewriteRule) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.NewPrefix != nil {
		i -= len(m.NewPrefix)
		copy(dAtA[i:], m.NewPrefix)
		i = encodeVarintChecksum(dAtA, i, uint64(len(m.NewPrefix)))
		i--
		dAtA[i] = 0x12
	}
	if m.OldPrefix != nil {
		i -= len(m.OldPrefix)
		copy(dAtA[i:], m.OldPrefix)
		i = encodeVarintChecksum(dAtA, i, uint64(len(m.OldPrefix)))
		i--
		dAtA[i] = 0xa
	}
	return len(dAtA) - i, nil
}

func (m *ChecksumRequest) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *ChecksumRequest) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *ChecksumRequest) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.Rule != nil {
		{
			size, err := m.Rule.MarshalToSizedBuffer(dAtA[:i])
			if err != nil {
				return 0, err
			}
			i -= size
			i = encodeVarintChecksum(dAtA, i, uint64(size))
		}
		i--
		dAtA[i] = 0x22
	}
	i = encodeVarintChecksum(dAtA, i, uint64(m.Algorithm))
	i--
	dAtA[i] = 0x18
	i = encodeVarintChecksum(dAtA, i, uint64(m.ScanOn))
	i--
	dAtA[i] = 0x10
	if m.StartTsFallback != nil {
		i = encodeVarintChecksum(dAtA, i, uint64(*m.StartTsFallback))
		i--
		dAtA[i] = 0x8
	}
	return len(dAtA) - i, nil
}

func (m *ChecksumResponse) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *ChecksumResponse) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *ChecksumResponse) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	i = encodeVarintChecksum(dAtA, i, uint64(m.TotalBytes))
	i--
	dAtA[i] = 0x18
	i = encodeVarintChecksum(dAtA, i, uint64(m.TotalKvs))
	i--
	dAtA[i] = 0x10
	i = encodeVarintChecksum(dAtA, i, uint64(m.Checksum))
	i--
	dAtA[i] = 0x8
	return len(dAtA) - i, nil
}

func encodeVarintChecksum(dAtA []byte, offset int, v uint64) int {
	offset -= sovChecksum(v)
	base := offset
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return base
}
func (m *ChecksumRewriteRule) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.OldPrefix != nil {
		l = len(m.OldPrefix)
		n += 1 + l + sovChecksum(uint64(l))
	}
	if m.NewPrefix != nil {
		l = len(m.NewPrefix)
		n += 1 + l + sovChecksum(uint64(l))
	}
	return n
}

func (m *ChecksumRequest) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.StartTsFallback != nil {
		n += 1 + sovChecksum(uint64(*m.StartTsFallback))
	}
	n += 1 + sovChecksum(uint64(m.ScanOn))
	n += 1 + sovChecksum(uint64(m.Algorithm))
	if m.Rule != nil {
		l = m.Rule.Size()
		n += 1 + l + sovChecksum(uint64(l))
	}
	return n
}

func (m *ChecksumResponse) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	n += 1 + sovChecksum(uint64(m.Checksum))
	n += 1 + sovChecksum(uint64(m.TotalKvs))
	n += 1 + sovChecksum(uint64(m.TotalBytes))
	return n
}

func sovChecksum(x uint64) (n int) {
	return (math_bits.Len64(x|1) + 6) / 7
}
func sozChecksum(x uint64) (n int) {
	return sovChecksum(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *ChecksumRewriteRule) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowChecksum
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: ChecksumRewriteRule: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: ChecksumRewriteRule: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field OldPrefix", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowChecksum
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				byteLen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if byteLen < 0 {
				return ErrInvalidLengthChecksum
			}
			postIndex := iNdEx + byteLen
			if postIndex < 0 {
				return ErrInvalidLengthChecksum
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.OldPrefix = append(m.OldPrefix[:0], dAtA[iNdEx:postIndex]...)
			if m.OldPrefix == nil {
				m.OldPrefix = []byte{}
			}
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field NewPrefix", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowChecksum
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				byteLen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if byteLen < 0 {
				return ErrInvalidLengthChecksum
			}
			postIndex := iNdEx + byteLen
			if postIndex < 0 {
				return ErrInvalidLengthChecksum
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.NewPrefix = append(m.NewPrefix[:0], dAtA[iNdEx:postIndex]...)
			if m.NewPrefix == nil {
				m.NewPrefix = []byte{}
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipChecksum(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthChecksum
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *ChecksumRequest) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowChecksum
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: ChecksumRequest: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: ChecksumRequest: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field StartTsFallback", wireType)
			}
			var v uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowChecksum
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				v |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.StartTsFallback = &v
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field ScanOn", wireType)
			}
			m.ScanOn = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowChecksum
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.ScanOn |= ChecksumScanOn(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 3:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Algorithm", wireType)
			}
			m.Algorithm = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowChecksum
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.Algorithm |= ChecksumAlgorithm(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Rule", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowChecksum
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthChecksum
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthChecksum
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.Rule == nil {
				m.Rule = &ChecksumRewriteRule{}
			}
			if err := m.Rule.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipChecksum(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthChecksum
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *ChecksumResponse) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowChecksum
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: ChecksumResponse: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: ChecksumResponse: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Checksum", wireType)
			}
			m.Checksum = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowChecksum
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.Checksum |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field TotalKvs", wireType)
			}
			m.TotalKvs = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowChecksum
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.TotalKvs |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 3:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field TotalBytes", wireType)
			}
			m.TotalBytes = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowChecksum
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.TotalBytes |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		default:
			iNdEx = preIndex
			skippy, err := skipChecksum(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthChecksum
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func skipChecksum(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	depth := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowChecksum
			}
			if iNdEx >= l {
				return 0, io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		wireType := int(wire & 0x7)
		switch wireType {
		case 0:
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowChecksum
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				iNdEx++
				if dAtA[iNdEx-1] < 0x80 {
					break
				}
			}
		case 1:
			iNdEx += 8
		case 2:
			var length int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowChecksum
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				length |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if length < 0 {
				return 0, ErrInvalidLengthChecksum
			}
			iNdEx += length
		case 3:
			depth++
		case 4:
			if depth == 0 {
				return 0, ErrUnexpectedEndOfGroupChecksum
			}
			depth--
		case 5:
			iNdEx += 4
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
		if iNdEx < 0 {
			return 0, ErrInvalidLengthChecksum
		}
		if depth == 0 {
			return iNdEx, nil
		}
	}
	return 0, io.ErrUnexpectedEOF
}

var (
	ErrInvalidLengthChecksum        = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowChecksum          = fmt.Errorf("proto: integer overflow")
	ErrUnexpectedEndOfGroupChecksum = fmt.Errorf("proto: unexpected end of group")
)
