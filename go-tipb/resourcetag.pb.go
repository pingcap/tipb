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

type ResourceGroupTag struct {
	SqlDigest        []byte `protobuf:"bytes,1,opt,name=sql_digest,json=sqlDigest" json:"sql_digest,omitempty"`
	PlanDigest       []byte `protobuf:"bytes,2,opt,name=plan_digest,json=planDigest" json:"plan_digest,omitempty"`
	XXX_unrecognized []byte `json:"-"`
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

func init() {
	proto.RegisterType((*ResourceGroupTag)(nil), "tipb.ResourceGroupTag")
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
	// 164 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xe2, 0x12, 0x2c, 0x4a, 0x2d, 0xce,
	0x2f, 0x2d, 0x4a, 0x4e, 0x2d, 0x49, 0x4c, 0xd7, 0x2b, 0x28, 0xca, 0x2f, 0xc9, 0x17, 0x62, 0x29,
	0xc9, 0x2c, 0x48, 0x92, 0x12, 0x49, 0xcf, 0x4f, 0xcf, 0x07, 0x0b, 0xe8, 0x83, 0x58, 0x10, 0x39,
	0xa5, 0x20, 0x2e, 0x81, 0x20, 0xa8, 0x06, 0xf7, 0xa2, 0xfc, 0xd2, 0x82, 0x90, 0xc4, 0x74, 0x21,
	0x59, 0x2e, 0xae, 0xe2, 0xc2, 0x9c, 0xf8, 0x94, 0xcc, 0xf4, 0xd4, 0xe2, 0x12, 0x09, 0x46, 0x05,
	0x46, 0x0d, 0x9e, 0x20, 0xce, 0xe2, 0xc2, 0x1c, 0x17, 0xb0, 0x80, 0x90, 0x3c, 0x17, 0x77, 0x41,
	0x4e, 0x62, 0x1e, 0x4c, 0x9e, 0x09, 0x2c, 0xcf, 0x05, 0x12, 0x82, 0x28, 0x70, 0xd2, 0x3c, 0xf1,
	0x48, 0x8e, 0xf1, 0xc2, 0x23, 0x39, 0xc6, 0x07, 0x8f, 0xe4, 0x18, 0x67, 0x3c, 0x96, 0x63, 0xe0,
	0x12, 0x4d, 0xce, 0xcf, 0xd5, 0x2b, 0xc8, 0xcc, 0x4b, 0x4f, 0x4e, 0x2c, 0xd0, 0x2b, 0xc9, 0x4c,
	0x49, 0xd2, 0x03, 0x39, 0x29, 0x80, 0x11, 0x10, 0x00, 0x00, 0xff, 0xff, 0x9a, 0x82, 0x2e, 0x8f,
	0xae, 0x00, 0x00, 0x00,
}
