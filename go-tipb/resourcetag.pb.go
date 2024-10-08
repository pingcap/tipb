// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: resourcetag.proto

package tipb

import (
	"fmt"

	proto "github.com/golang/protobuf/proto"

	math "math"

	io "io"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

type ResourceGroupTagLabel int32

const (
	ResourceGroupTagLabel_ResourceGroupTagLabelUnknown ResourceGroupTagLabel = 0
	ResourceGroupTagLabel_ResourceGroupTagLabelRow     ResourceGroupTagLabel = 1
	ResourceGroupTagLabel_ResourceGroupTagLabelIndex   ResourceGroupTagLabel = 2
)

var ResourceGroupTagLabel_name = map[int32]string{
	0: "ResourceGroupTagLabelUnknown",
	1: "ResourceGroupTagLabelRow",
	2: "ResourceGroupTagLabelIndex",
}
var ResourceGroupTagLabel_value = map[string]int32{
	"ResourceGroupTagLabelUnknown": 0,
	"ResourceGroupTagLabelRow":     1,
	"ResourceGroupTagLabelIndex":   2,
}

func (x ResourceGroupTagLabel) Enum() *ResourceGroupTagLabel {
	p := new(ResourceGroupTagLabel)
	*p = x
	return p
}
func (x ResourceGroupTagLabel) String() string {
	return proto.EnumName(ResourceGroupTagLabel_name, int32(x))
}
func (x *ResourceGroupTagLabel) UnmarshalJSON(data []byte) error {
	value, err := proto.UnmarshalJSONEnum(ResourceGroupTagLabel_value, data, "ResourceGroupTagLabel")
	if err != nil {
		return err
	}
	*x = ResourceGroupTagLabel(value)
	return nil
}
func (ResourceGroupTagLabel) EnumDescriptor() ([]byte, []int) {
	return fileDescriptorResourcetag, []int{0}
}

type ResourceGroupTag struct {
	SqlDigest  []byte `protobuf:"bytes,1,opt,name=sql_digest,json=sqlDigest" json:"sql_digest,omitempty"`
	PlanDigest []byte `protobuf:"bytes,2,opt,name=plan_digest,json=planDigest" json:"plan_digest,omitempty"`
	// Use to label the handling kv type of the request.
	// This is for TiKV resource_metering to collect execution information by the key label.
	Label            *ResourceGroupTagLabel `protobuf:"varint,3,opt,name=label,enum=tipb.ResourceGroupTagLabel" json:"label,omitempty"`
	TableId          int64                  `protobuf:"varint,4,opt,name=table_id,json=tableId" json:"table_id"`
	XXX_unrecognized []byte                 `json:"-"`
}

func (m *ResourceGroupTag) Reset()                    { *m = ResourceGroupTag{} }
func (m *ResourceGroupTag) String() string            { return proto.CompactTextString(m) }
func (*ResourceGroupTag) ProtoMessage()               {}
func (*ResourceGroupTag) Descriptor() ([]byte, []int) { return fileDescriptorResourcetag, []int{0} }

func (m *ResourceGroupTag) GetSqlDigest() []byte {
	if m != nil {
		return m.SqlDigest
	}
	return nil
}

func (m *ResourceGroupTag) GetPlanDigest() []byte {
	if m != nil {
		return m.PlanDigest
	}
	return nil
}

func (m *ResourceGroupTag) GetLabel() ResourceGroupTagLabel {
	if m != nil && m.Label != nil {
		return *m.Label
	}
	return ResourceGroupTagLabel_ResourceGroupTagLabelUnknown
}

func (m *ResourceGroupTag) GetTableId() int64 {
	if m != nil {
		return m.TableId
	}
	return 0
}

func init() {
	proto.RegisterType((*ResourceGroupTag)(nil), "tipb.ResourceGroupTag")
	proto.RegisterEnum("tipb.ResourceGroupTagLabel", ResourceGroupTagLabel_name, ResourceGroupTagLabel_value)
}
func (m *ResourceGroupTag) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *ResourceGroupTag) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.SqlDigest != nil {
		dAtA[i] = 0xa
		i++
		i = encodeVarintResourcetag(dAtA, i, uint64(len(m.SqlDigest)))
		i += copy(dAtA[i:], m.SqlDigest)
	}
	if m.PlanDigest != nil {
		dAtA[i] = 0x12
		i++
		i = encodeVarintResourcetag(dAtA, i, uint64(len(m.PlanDigest)))
		i += copy(dAtA[i:], m.PlanDigest)
	}
	if m.Label != nil {
		dAtA[i] = 0x18
		i++
		i = encodeVarintResourcetag(dAtA, i, uint64(*m.Label))
	}
	dAtA[i] = 0x20
	i++
	i = encodeVarintResourcetag(dAtA, i, uint64(m.TableId))
	if m.XXX_unrecognized != nil {
		i += copy(dAtA[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func encodeVarintResourcetag(dAtA []byte, offset int, v uint64) int {
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return offset + 1
}
func (m *ResourceGroupTag) Size() (n int) {
	var l int
	_ = l
	if m.SqlDigest != nil {
		l = len(m.SqlDigest)
		n += 1 + l + sovResourcetag(uint64(l))
	}
	if m.PlanDigest != nil {
		l = len(m.PlanDigest)
		n += 1 + l + sovResourcetag(uint64(l))
	}
	if m.Label != nil {
		n += 1 + sovResourcetag(uint64(*m.Label))
	}
	n += 1 + sovResourcetag(uint64(m.TableId))
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func sovResourcetag(x uint64) (n int) {
	for {
		n++
		x >>= 7
		if x == 0 {
			break
		}
	}
	return n
}
func sozResourcetag(x uint64) (n int) {
	return sovResourcetag(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *ResourceGroupTag) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowResourcetag
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: ResourceGroupTag: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: ResourceGroupTag: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field SqlDigest", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowResourcetag
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				byteLen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if byteLen < 0 {
				return ErrInvalidLengthResourcetag
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.SqlDigest = append(m.SqlDigest[:0], dAtA[iNdEx:postIndex]...)
			if m.SqlDigest == nil {
				m.SqlDigest = []byte{}
			}
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field PlanDigest", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowResourcetag
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				byteLen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if byteLen < 0 {
				return ErrInvalidLengthResourcetag
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.PlanDigest = append(m.PlanDigest[:0], dAtA[iNdEx:postIndex]...)
			if m.PlanDigest == nil {
				m.PlanDigest = []byte{}
			}
			iNdEx = postIndex
		case 3:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Label", wireType)
			}
			var v ResourceGroupTagLabel
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowResourcetag
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				v |= (ResourceGroupTagLabel(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.Label = &v
		case 4:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field TableId", wireType)
			}
			m.TableId = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowResourcetag
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.TableId |= (int64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		default:
			iNdEx = preIndex
			skippy, err := skipResourcetag(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthResourcetag
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			m.XXX_unrecognized = append(m.XXX_unrecognized, dAtA[iNdEx:iNdEx+skippy]...)
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func skipResourcetag(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowResourcetag
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
					return 0, ErrIntOverflowResourcetag
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				iNdEx++
				if dAtA[iNdEx-1] < 0x80 {
					break
				}
			}
			return iNdEx, nil
		case 1:
			iNdEx += 8
			return iNdEx, nil
		case 2:
			var length int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowResourcetag
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
			iNdEx += length
			if length < 0 {
				return 0, ErrInvalidLengthResourcetag
			}
			return iNdEx, nil
		case 3:
			for {
				var innerWire uint64
				var start int = iNdEx
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return 0, ErrIntOverflowResourcetag
					}
					if iNdEx >= l {
						return 0, io.ErrUnexpectedEOF
					}
					b := dAtA[iNdEx]
					iNdEx++
					innerWire |= (uint64(b) & 0x7F) << shift
					if b < 0x80 {
						break
					}
				}
				innerWireType := int(innerWire & 0x7)
				if innerWireType == 4 {
					break
				}
				next, err := skipResourcetag(dAtA[start:])
				if err != nil {
					return 0, err
				}
				iNdEx = start + next
			}
			return iNdEx, nil
		case 4:
			return iNdEx, nil
		case 5:
			iNdEx += 4
			return iNdEx, nil
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
	}
	panic("unreachable")
}

var (
	ErrInvalidLengthResourcetag = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowResourcetag   = fmt.Errorf("proto: integer overflow")
)

func init() { proto.RegisterFile("resourcetag.proto", fileDescriptorResourcetag) }

var fileDescriptorResourcetag = []byte{
	// 268 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xe2, 0x12, 0x2c, 0x4a, 0x2d, 0xce,
	0x2f, 0x2d, 0x4a, 0x4e, 0x2d, 0x49, 0x4c, 0xd7, 0x2b, 0x28, 0xca, 0x2f, 0xc9, 0x17, 0x62, 0x29,
	0xc9, 0x2c, 0x48, 0x92, 0x12, 0x49, 0xcf, 0x4f, 0xcf, 0x07, 0x0b, 0xe8, 0x83, 0x58, 0x10, 0x39,
	0xa5, 0x65, 0x8c, 0x5c, 0x02, 0x41, 0x50, 0x1d, 0xee, 0x45, 0xf9, 0xa5, 0x05, 0x21, 0x89, 0xe9,
	0x42, 0xb2, 0x5c, 0x5c, 0xc5, 0x85, 0x39, 0xf1, 0x29, 0x99, 0xe9, 0xa9, 0xc5, 0x25, 0x12, 0x8c,
	0x0a, 0x8c, 0x1a, 0x3c, 0x41, 0x9c, 0xc5, 0x85, 0x39, 0x2e, 0x60, 0x01, 0x21, 0x79, 0x2e, 0xee,
	0x82, 0x9c, 0xc4, 0x3c, 0x98, 0x3c, 0x13, 0x58, 0x9e, 0x0b, 0x24, 0x04, 0x55, 0x60, 0xc8, 0xc5,
	0x9a, 0x93, 0x98, 0x94, 0x9a, 0x23, 0xc1, 0xac, 0xc0, 0xa8, 0xc1, 0x67, 0x24, 0xad, 0x07, 0x72,
	0x80, 0x1e, 0xba, 0x35, 0x3e, 0x20, 0x25, 0x41, 0x10, 0x95, 0x42, 0xf2, 0x5c, 0x1c, 0x25, 0x89,
	0x49, 0x39, 0xa9, 0xf1, 0x99, 0x29, 0x12, 0x2c, 0x0a, 0x8c, 0x1a, 0xcc, 0x4e, 0x2c, 0x27, 0xee,
	0xc9, 0x33, 0x04, 0xb1, 0x83, 0x45, 0x3d, 0x53, 0xb4, 0xca, 0xb9, 0x44, 0xb1, 0x1a, 0x20, 0xa4,
	0xc0, 0x25, 0x83, 0x55, 0x22, 0x34, 0x2f, 0x3b, 0x2f, 0xbf, 0x3c, 0x4f, 0x80, 0x41, 0x48, 0x86,
	0x4b, 0x02, 0xbb, 0xdd, 0xf9, 0xe5, 0x02, 0x8c, 0x42, 0x72, 0x5c, 0x52, 0x58, 0x65, 0x3d, 0xf3,
	0x52, 0x52, 0x2b, 0x04, 0x98, 0x9c, 0x34, 0x4f, 0x3c, 0x92, 0x63, 0xbc, 0xf0, 0x48, 0x8e, 0xf1,
	0xc1, 0x23, 0x39, 0xc6, 0x19, 0x8f, 0xe5, 0x18, 0xb8, 0x44, 0x93, 0xf3, 0x73, 0xf5, 0x0a, 0x32,
	0xf3, 0xd2, 0x93, 0x13, 0x0b, 0xf4, 0x4a, 0x32, 0x53, 0x92, 0xc0, 0xfe, 0x0b, 0x60, 0x04, 0x04,
	0x00, 0x00, 0xff, 0xff, 0xf7, 0xe8, 0xcf, 0x01, 0x7c, 0x01, 0x00, 0x00,
}
