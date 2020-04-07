// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: metadata.proto

package tipb

import (
	"fmt"

	proto "github.com/golang/protobuf/proto"

	math "math"

	io "io"

	github_com_golang_protobuf_proto "github.com/golang/protobuf/proto"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

type InUnionMetadata struct {
	InUnion          bool   `protobuf:"varint,1,req,name=in_union,json=inUnion" json:"in_union"`
	XXX_unrecognized []byte `json:"-"`
}

func (m *InUnionMetadata) Reset()                    { *m = InUnionMetadata{} }
func (m *InUnionMetadata) String() string            { return proto.CompactTextString(m) }
func (*InUnionMetadata) ProtoMessage()               {}
func (*InUnionMetadata) Descriptor() ([]byte, []int) { return fileDescriptorMetadata, []int{0} }

func (m *InUnionMetadata) GetInUnion() bool {
	if m != nil {
		return m.InUnion
	}
	return false
}

type CompareInMetadata struct {
	HasNull bool `protobuf:"varint,1,req,name=has_null,json=hasNull" json:"has_null"`
	// consts represents all non-null const args in repeated Datum format.
	Consts           []byte `protobuf:"bytes,2,opt,name=consts" json:"consts,omitempty"`
	XXX_unrecognized []byte `json:"-"`
}

func (m *CompareInMetadata) Reset()                    { *m = CompareInMetadata{} }
func (m *CompareInMetadata) String() string            { return proto.CompactTextString(m) }
func (*CompareInMetadata) ProtoMessage()               {}
func (*CompareInMetadata) Descriptor() ([]byte, []int) { return fileDescriptorMetadata, []int{1} }

func (m *CompareInMetadata) GetHasNull() bool {
	if m != nil {
		return m.HasNull
	}
	return false
}

func (m *CompareInMetadata) GetConsts() []byte {
	if m != nil {
		return m.Consts
	}
	return nil
}

type KeepNullMetadata struct {
	KeepNull         bool   `protobuf:"varint,1,req,name=keep_null,json=keepNull" json:"keep_null"`
	XXX_unrecognized []byte `json:"-"`
}

func (m *KeepNullMetadata) Reset()                    { *m = KeepNullMetadata{} }
func (m *KeepNullMetadata) String() string            { return proto.CompactTextString(m) }
func (*KeepNullMetadata) ProtoMessage()               {}
func (*KeepNullMetadata) Descriptor() ([]byte, []int) { return fileDescriptorMetadata, []int{2} }

func (m *KeepNullMetadata) GetKeepNull() bool {
	if m != nil {
		return m.KeepNull
	}
	return false
}

func init() {
	proto.RegisterType((*InUnionMetadata)(nil), "tipb.InUnionMetadata")
	proto.RegisterType((*CompareInMetadata)(nil), "tipb.CompareInMetadata")
	proto.RegisterType((*KeepNullMetadata)(nil), "tipb.KeepNullMetadata")
}
func (m *InUnionMetadata) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *InUnionMetadata) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	dAtA[i] = 0x8
	i++
	if m.InUnion {
		dAtA[i] = 1
	} else {
		dAtA[i] = 0
	}
	i++
	if m.XXX_unrecognized != nil {
		i += copy(dAtA[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func (m *CompareInMetadata) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *CompareInMetadata) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	dAtA[i] = 0x8
	i++
	if m.HasNull {
		dAtA[i] = 1
	} else {
		dAtA[i] = 0
	}
	i++
	if m.Consts != nil {
		dAtA[i] = 0x12
		i++
		i = encodeVarintMetadata(dAtA, i, uint64(len(m.Consts)))
		i += copy(dAtA[i:], m.Consts)
	}
	if m.XXX_unrecognized != nil {
		i += copy(dAtA[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func (m *KeepNullMetadata) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *KeepNullMetadata) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	dAtA[i] = 0x8
	i++
	if m.KeepNull {
		dAtA[i] = 1
	} else {
		dAtA[i] = 0
	}
	i++
	if m.XXX_unrecognized != nil {
		i += copy(dAtA[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func encodeVarintMetadata(dAtA []byte, offset int, v uint64) int {
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return offset + 1
}
func (m *InUnionMetadata) Size() (n int) {
	var l int
	_ = l
	n += 2
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *CompareInMetadata) Size() (n int) {
	var l int
	_ = l
	n += 2
	if m.Consts != nil {
		l = len(m.Consts)
		n += 1 + l + sovMetadata(uint64(l))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *KeepNullMetadata) Size() (n int) {
	var l int
	_ = l
	n += 2
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func sovMetadata(x uint64) (n int) {
	for {
		n++
		x >>= 7
		if x == 0 {
			break
		}
	}
	return n
}
func sozMetadata(x uint64) (n int) {
	return sovMetadata(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *InUnionMetadata) Unmarshal(dAtA []byte) error {
	var hasFields [1]uint64
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowMetadata
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
			return fmt.Errorf("proto: InUnionMetadata: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: InUnionMetadata: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field InUnion", wireType)
			}
			var v int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMetadata
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				v |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.InUnion = bool(v != 0)
			hasFields[0] |= uint64(0x00000001)
		default:
			iNdEx = preIndex
			skippy, err := skipMetadata(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthMetadata
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			m.XXX_unrecognized = append(m.XXX_unrecognized, dAtA[iNdEx:iNdEx+skippy]...)
			iNdEx += skippy
		}
	}
	if hasFields[0]&uint64(0x00000001) == 0 {
		return new(github_com_golang_protobuf_proto.RequiredNotSetError)
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *CompareInMetadata) Unmarshal(dAtA []byte) error {
	var hasFields [1]uint64
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowMetadata
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
			return fmt.Errorf("proto: CompareInMetadata: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: CompareInMetadata: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field HasNull", wireType)
			}
			var v int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMetadata
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				v |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.HasNull = bool(v != 0)
			hasFields[0] |= uint64(0x00000001)
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Consts", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMetadata
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
				return ErrInvalidLengthMetadata
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Consts = append(m.Consts[:0], dAtA[iNdEx:postIndex]...)
			if m.Consts == nil {
				m.Consts = []byte{}
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipMetadata(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthMetadata
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			m.XXX_unrecognized = append(m.XXX_unrecognized, dAtA[iNdEx:iNdEx+skippy]...)
			iNdEx += skippy
		}
	}
	if hasFields[0]&uint64(0x00000001) == 0 {
		return new(github_com_golang_protobuf_proto.RequiredNotSetError)
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *KeepNullMetadata) Unmarshal(dAtA []byte) error {
	var hasFields [1]uint64
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowMetadata
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
			return fmt.Errorf("proto: KeepNullMetadata: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: KeepNullMetadata: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field KeepNull", wireType)
			}
			var v int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMetadata
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				v |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.KeepNull = bool(v != 0)
			hasFields[0] |= uint64(0x00000001)
		default:
			iNdEx = preIndex
			skippy, err := skipMetadata(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthMetadata
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			m.XXX_unrecognized = append(m.XXX_unrecognized, dAtA[iNdEx:iNdEx+skippy]...)
			iNdEx += skippy
		}
	}
	if hasFields[0]&uint64(0x00000001) == 0 {
		return new(github_com_golang_protobuf_proto.RequiredNotSetError)
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func skipMetadata(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowMetadata
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
					return 0, ErrIntOverflowMetadata
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
					return 0, ErrIntOverflowMetadata
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
				return 0, ErrInvalidLengthMetadata
			}
			return iNdEx, nil
		case 3:
			for {
				var innerWire uint64
				var start int = iNdEx
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return 0, ErrIntOverflowMetadata
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
				next, err := skipMetadata(dAtA[start:])
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
	ErrInvalidLengthMetadata = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowMetadata   = fmt.Errorf("proto: integer overflow")
)

func init() { proto.RegisterFile("metadata.proto", fileDescriptorMetadata) }

var fileDescriptorMetadata = []byte{
	// 217 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xe2, 0xe2, 0xcb, 0x4d, 0x2d, 0x49,
	0x4c, 0x49, 0x2c, 0x49, 0xd4, 0x2b, 0x28, 0xca, 0x2f, 0xc9, 0x17, 0x62, 0x29, 0xc9, 0x2c, 0x48,
	0x92, 0x12, 0x49, 0xcf, 0x4f, 0xcf, 0x07, 0x0b, 0xe8, 0x83, 0x58, 0x10, 0x39, 0x25, 0x23, 0x2e,
	0x7e, 0xcf, 0xbc, 0xd0, 0xbc, 0xcc, 0xfc, 0x3c, 0x5f, 0xa8, 0x26, 0x21, 0x79, 0x2e, 0x8e, 0xcc,
	0xbc, 0xf8, 0x52, 0x90, 0x98, 0x04, 0xa3, 0x02, 0x93, 0x06, 0x87, 0x13, 0xcb, 0x89, 0x7b, 0xf2,
	0x0c, 0x41, 0xec, 0x99, 0x10, 0x85, 0x4a, 0x3e, 0x5c, 0x82, 0xce, 0xf9, 0xb9, 0x05, 0x89, 0x45,
	0xa9, 0x9e, 0x28, 0xba, 0x32, 0x12, 0x8b, 0xe3, 0xf3, 0x4a, 0x73, 0x72, 0x50, 0x75, 0x65, 0x24,
	0x16, 0xfb, 0x95, 0xe6, 0xe4, 0x08, 0x89, 0x71, 0xb1, 0x25, 0xe7, 0xe7, 0x15, 0x97, 0x14, 0x4b,
	0x30, 0x29, 0x30, 0x6a, 0xf0, 0x04, 0x41, 0x79, 0x4a, 0xa6, 0x5c, 0x02, 0xde, 0xa9, 0xa9, 0x05,
	0x20, 0x35, 0x70, 0xc3, 0x14, 0xb9, 0x38, 0xb3, 0x53, 0x53, 0x0b, 0x30, 0x4d, 0xe3, 0xc8, 0x86,
	0x2a, 0x75, 0xd2, 0x3c, 0xf1, 0x48, 0x8e, 0xf1, 0xc2, 0x23, 0x39, 0xc6, 0x07, 0x8f, 0xe4, 0x18,
	0x67, 0x3c, 0x96, 0x63, 0xe0, 0x12, 0x4d, 0xce, 0xcf, 0xd5, 0x2b, 0xc8, 0xcc, 0x4b, 0x4f, 0x4e,
	0x2c, 0xd0, 0x2b, 0xc9, 0x4c, 0x49, 0xd2, 0x03, 0xf9, 0x3b, 0x80, 0x11, 0x10, 0x00, 0x00, 0xff,
	0xff, 0x36, 0x4d, 0x49, 0x73, 0x10, 0x01, 0x00, 0x00,
}
