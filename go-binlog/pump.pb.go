// Code generated by protoc-gen-gogo.
// source: pump.proto
// DO NOT EDIT!

/*
	Package binlog is a generated protocol buffer package.

	It is generated from these files:
		pump.proto

	It has these top-level messages:
		WriteBinlogReq
		WriteBinlogResp
		PullBinlogReq
		PullBinlogResp
		Pos
		Entity
*/
package binlog

import (
	"fmt"

	proto "github.com/golang/protobuf/proto"

	math "math"

	io "io"
)

import (
	context "golang.org/x/net/context"
	grpc "google.golang.org/grpc"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.ProtoPackageIsVersion2 // please upgrade the proto package

type WriteBinlogReq struct {
	// The identifier of tidb-cluster, which is given at tidb startup.
	// Must specify the clusterID for each binlog to write.
	ClusterID uint64 `protobuf:"varint,1,opt,name=clusterID,proto3" json:"clusterID,omitempty"`
	// Payload bytes can be decoded back to binlog struct by the protobuf.
	Payload []byte `protobuf:"bytes,2,opt,name=payload,proto3" json:"payload,omitempty"`
}

func (m *WriteBinlogReq) Reset()                    { *m = WriteBinlogReq{} }
func (m *WriteBinlogReq) String() string            { return proto.CompactTextString(m) }
func (*WriteBinlogReq) ProtoMessage()               {}
func (*WriteBinlogReq) Descriptor() ([]byte, []int) { return fileDescriptorPump, []int{0} }

type WriteBinlogResp struct {
	// An empty errmsg returned means a successful write.
	// Otherwise return the error description.
	Errmsg string `protobuf:"bytes,1,opt,name=errmsg,proto3" json:"errmsg,omitempty"`
}

func (m *WriteBinlogResp) Reset()                    { *m = WriteBinlogResp{} }
func (m *WriteBinlogResp) String() string            { return proto.CompactTextString(m) }
func (*WriteBinlogResp) ProtoMessage()               {}
func (*WriteBinlogResp) Descriptor() ([]byte, []int) { return fileDescriptorPump, []int{1} }

type PullBinlogReq struct {
	// Specifies which clusterID of binlog to pull.
	ClusterID uint64 `protobuf:"varint,1,opt,name=clusterID,proto3" json:"clusterID,omitempty"`
	// The position from which the binlog will be sent.
	StartFrom Pos `protobuf:"bytes,2,opt,name=startFrom" json:"startFrom"`
	// The max number of binlog in a batch to pull.
	Batch int32 `protobuf:"varint,3,opt,name=batch,proto3" json:"batch,omitempty"`
}

func (m *PullBinlogReq) Reset()                    { *m = PullBinlogReq{} }
func (m *PullBinlogReq) String() string            { return proto.CompactTextString(m) }
func (*PullBinlogReq) ProtoMessage()               {}
func (*PullBinlogReq) Descriptor() ([]byte, []int) { return fileDescriptorPump, []int{2} }

func (m *PullBinlogReq) GetStartFrom() Pos {
	if m != nil {
		return m.StartFrom
	}
	return Pos{}
}

type PullBinlogResp struct {
	// The binlog entities pulled in a batch
	Entity Entity `protobuf:"bytes,1,opt,name=entity" json:"entity"`
}

func (m *PullBinlogResp) Reset()                    { *m = PullBinlogResp{} }
func (m *PullBinlogResp) String() string            { return proto.CompactTextString(m) }
func (*PullBinlogResp) ProtoMessage()               {}
func (*PullBinlogResp) Descriptor() ([]byte, []int) { return fileDescriptorPump, []int{3} }

func (m *PullBinlogResp) GetEntity() Entity {
	if m != nil {
		return m.Entity
	}
	return Entity{}
}

// Binlogs are stored in a number of sequential files in a directory.
// The Pos describes the position of a binlog.
type Pos struct {
	// The suffix of binlog file, like .000001 .000002
	Suffix uint64 `protobuf:"varint,1,opt,name=suffix,proto3" json:"suffix,omitempty"`
	// The binlog offset in a file.
	Offset int64 `protobuf:"varint,2,opt,name=offset,proto3" json:"offset,omitempty"`
}

func (m *Pos) Reset()                    { *m = Pos{} }
func (m *Pos) String() string            { return proto.CompactTextString(m) }
func (*Pos) ProtoMessage()               {}
func (*Pos) Descriptor() ([]byte, []int) { return fileDescriptorPump, []int{4} }

type Entity struct {
	// The position of the binlog entity.
	Pos Pos `protobuf:"bytes,1,opt,name=pos" json:"pos"`
	// The payload of binlog entity.
	Payload []byte `protobuf:"bytes,2,opt,name=payload,proto3" json:"payload,omitempty"`
}

func (m *Entity) Reset()                    { *m = Entity{} }
func (m *Entity) String() string            { return proto.CompactTextString(m) }
func (*Entity) ProtoMessage()               {}
func (*Entity) Descriptor() ([]byte, []int) { return fileDescriptorPump, []int{5} }

func (m *Entity) GetPos() Pos {
	if m != nil {
		return m.Pos
	}
	return Pos{}
}

func init() {
	proto.RegisterType((*WriteBinlogReq)(nil), "binlog.WriteBinlogReq")
	proto.RegisterType((*WriteBinlogResp)(nil), "binlog.WriteBinlogResp")
	proto.RegisterType((*PullBinlogReq)(nil), "binlog.PullBinlogReq")
	proto.RegisterType((*PullBinlogResp)(nil), "binlog.PullBinlogResp")
	proto.RegisterType((*Pos)(nil), "binlog.Pos")
	proto.RegisterType((*Entity)(nil), "binlog.Entity")
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConn

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion3

// Client API for Pump service

type PumpClient interface {
	// Writes a binlog to the local file on the pump machine.
	// A response with an empty errmsg is returned if the binlog is written successfully.
	WriteBinlog(ctx context.Context, in *WriteBinlogReq, opts ...grpc.CallOption) (*WriteBinlogResp, error)
	// Obtains a batch of binlog from a given location.
	PullBinlogs(ctx context.Context, in *PullBinlogReq, opts ...grpc.CallOption) (Pump_PullBinlogsClient, error)
}

type pumpClient struct {
	cc *grpc.ClientConn
}

func NewPumpClient(cc *grpc.ClientConn) PumpClient {
	return &pumpClient{cc}
}

func (c *pumpClient) WriteBinlog(ctx context.Context, in *WriteBinlogReq, opts ...grpc.CallOption) (*WriteBinlogResp, error) {
	out := new(WriteBinlogResp)
	err := grpc.Invoke(ctx, "/binlog.Pump/WriteBinlog", in, out, c.cc, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *pumpClient) PullBinlogs(ctx context.Context, in *PullBinlogReq, opts ...grpc.CallOption) (Pump_PullBinlogsClient, error) {
	stream, err := grpc.NewClientStream(ctx, &_Pump_serviceDesc.Streams[0], c.cc, "/binlog.Pump/PullBinlogs", opts...)
	if err != nil {
		return nil, err
	}
	x := &pumpPullBinlogsClient{stream}
	if err := x.ClientStream.SendMsg(in); err != nil {
		return nil, err
	}
	if err := x.ClientStream.CloseSend(); err != nil {
		return nil, err
	}
	return x, nil
}

type Pump_PullBinlogsClient interface {
	Recv() (*PullBinlogResp, error)
	grpc.ClientStream
}

type pumpPullBinlogsClient struct {
	grpc.ClientStream
}

func (x *pumpPullBinlogsClient) Recv() (*PullBinlogResp, error) {
	m := new(PullBinlogResp)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

// Server API for Pump service

type PumpServer interface {
	// Writes a binlog to the local file on the pump machine.
	// A response with an empty errmsg is returned if the binlog is written successfully.
	WriteBinlog(context.Context, *WriteBinlogReq) (*WriteBinlogResp, error)
	// Obtains a batch of binlog from a given location.
	PullBinlogs(*PullBinlogReq, Pump_PullBinlogsServer) error
}

func RegisterPumpServer(s *grpc.Server, srv PumpServer) {
	s.RegisterService(&_Pump_serviceDesc, srv)
}

func _Pump_WriteBinlog_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(WriteBinlogReq)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PumpServer).WriteBinlog(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/binlog.Pump/WriteBinlog",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PumpServer).WriteBinlog(ctx, req.(*WriteBinlogReq))
	}
	return interceptor(ctx, in, info, handler)
}

func _Pump_PullBinlogs_Handler(srv interface{}, stream grpc.ServerStream) error {
	m := new(PullBinlogReq)
	if err := stream.RecvMsg(m); err != nil {
		return err
	}
	return srv.(PumpServer).PullBinlogs(m, &pumpPullBinlogsServer{stream})
}

type Pump_PullBinlogsServer interface {
	Send(*PullBinlogResp) error
	grpc.ServerStream
}

type pumpPullBinlogsServer struct {
	grpc.ServerStream
}

func (x *pumpPullBinlogsServer) Send(m *PullBinlogResp) error {
	return x.ServerStream.SendMsg(m)
}

var _Pump_serviceDesc = grpc.ServiceDesc{
	ServiceName: "binlog.Pump",
	HandlerType: (*PumpServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "WriteBinlog",
			Handler:    _Pump_WriteBinlog_Handler,
		},
	},
	Streams: []grpc.StreamDesc{
		{
			StreamName:    "PullBinlogs",
			Handler:       _Pump_PullBinlogs_Handler,
			ServerStreams: true,
		},
	},
	Metadata: fileDescriptorPump,
}

func (m *WriteBinlogReq) Marshal() (data []byte, err error) {
	size := m.Size()
	data = make([]byte, size)
	n, err := m.MarshalTo(data)
	if err != nil {
		return nil, err
	}
	return data[:n], nil
}

func (m *WriteBinlogReq) MarshalTo(data []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.ClusterID != 0 {
		data[i] = 0x8
		i++
		i = encodeVarintPump(data, i, uint64(m.ClusterID))
	}
	if len(m.Payload) > 0 {
		data[i] = 0x12
		i++
		i = encodeVarintPump(data, i, uint64(len(m.Payload)))
		i += copy(data[i:], m.Payload)
	}
	return i, nil
}

func (m *WriteBinlogResp) Marshal() (data []byte, err error) {
	size := m.Size()
	data = make([]byte, size)
	n, err := m.MarshalTo(data)
	if err != nil {
		return nil, err
	}
	return data[:n], nil
}

func (m *WriteBinlogResp) MarshalTo(data []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if len(m.Errmsg) > 0 {
		data[i] = 0xa
		i++
		i = encodeVarintPump(data, i, uint64(len(m.Errmsg)))
		i += copy(data[i:], m.Errmsg)
	}
	return i, nil
}

func (m *PullBinlogReq) Marshal() (data []byte, err error) {
	size := m.Size()
	data = make([]byte, size)
	n, err := m.MarshalTo(data)
	if err != nil {
		return nil, err
	}
	return data[:n], nil
}

func (m *PullBinlogReq) MarshalTo(data []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.ClusterID != 0 {
		data[i] = 0x8
		i++
		i = encodeVarintPump(data, i, uint64(m.ClusterID))
	}
	data[i] = 0x12
	i++
	i = encodeVarintPump(data, i, uint64(m.StartFrom.Size()))
	n1, err := m.StartFrom.MarshalTo(data[i:])
	if err != nil {
		return 0, err
	}
	i += n1
	if m.Batch != 0 {
		data[i] = 0x18
		i++
		i = encodeVarintPump(data, i, uint64(m.Batch))
	}
	return i, nil
}

func (m *PullBinlogResp) Marshal() (data []byte, err error) {
	size := m.Size()
	data = make([]byte, size)
	n, err := m.MarshalTo(data)
	if err != nil {
		return nil, err
	}
	return data[:n], nil
}

func (m *PullBinlogResp) MarshalTo(data []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	data[i] = 0xa
	i++
	i = encodeVarintPump(data, i, uint64(m.Entity.Size()))
	n2, err := m.Entity.MarshalTo(data[i:])
	if err != nil {
		return 0, err
	}
	i += n2
	return i, nil
}

func (m *Pos) Marshal() (data []byte, err error) {
	size := m.Size()
	data = make([]byte, size)
	n, err := m.MarshalTo(data)
	if err != nil {
		return nil, err
	}
	return data[:n], nil
}

func (m *Pos) MarshalTo(data []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.Suffix != 0 {
		data[i] = 0x8
		i++
		i = encodeVarintPump(data, i, uint64(m.Suffix))
	}
	if m.Offset != 0 {
		data[i] = 0x10
		i++
		i = encodeVarintPump(data, i, uint64(m.Offset))
	}
	return i, nil
}

func (m *Entity) Marshal() (data []byte, err error) {
	size := m.Size()
	data = make([]byte, size)
	n, err := m.MarshalTo(data)
	if err != nil {
		return nil, err
	}
	return data[:n], nil
}

func (m *Entity) MarshalTo(data []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	data[i] = 0xa
	i++
	i = encodeVarintPump(data, i, uint64(m.Pos.Size()))
	n3, err := m.Pos.MarshalTo(data[i:])
	if err != nil {
		return 0, err
	}
	i += n3
	if len(m.Payload) > 0 {
		data[i] = 0x12
		i++
		i = encodeVarintPump(data, i, uint64(len(m.Payload)))
		i += copy(data[i:], m.Payload)
	}
	return i, nil
}

func encodeFixed64Pump(data []byte, offset int, v uint64) int {
	data[offset] = uint8(v)
	data[offset+1] = uint8(v >> 8)
	data[offset+2] = uint8(v >> 16)
	data[offset+3] = uint8(v >> 24)
	data[offset+4] = uint8(v >> 32)
	data[offset+5] = uint8(v >> 40)
	data[offset+6] = uint8(v >> 48)
	data[offset+7] = uint8(v >> 56)
	return offset + 8
}
func encodeFixed32Pump(data []byte, offset int, v uint32) int {
	data[offset] = uint8(v)
	data[offset+1] = uint8(v >> 8)
	data[offset+2] = uint8(v >> 16)
	data[offset+3] = uint8(v >> 24)
	return offset + 4
}
func encodeVarintPump(data []byte, offset int, v uint64) int {
	for v >= 1<<7 {
		data[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	data[offset] = uint8(v)
	return offset + 1
}
func (m *WriteBinlogReq) Size() (n int) {
	var l int
	_ = l
	if m.ClusterID != 0 {
		n += 1 + sovPump(uint64(m.ClusterID))
	}
	l = len(m.Payload)
	if l > 0 {
		n += 1 + l + sovPump(uint64(l))
	}
	return n
}

func (m *WriteBinlogResp) Size() (n int) {
	var l int
	_ = l
	l = len(m.Errmsg)
	if l > 0 {
		n += 1 + l + sovPump(uint64(l))
	}
	return n
}

func (m *PullBinlogReq) Size() (n int) {
	var l int
	_ = l
	if m.ClusterID != 0 {
		n += 1 + sovPump(uint64(m.ClusterID))
	}
	l = m.StartFrom.Size()
	n += 1 + l + sovPump(uint64(l))
	if m.Batch != 0 {
		n += 1 + sovPump(uint64(m.Batch))
	}
	return n
}

func (m *PullBinlogResp) Size() (n int) {
	var l int
	_ = l
	l = m.Entity.Size()
	n += 1 + l + sovPump(uint64(l))
	return n
}

func (m *Pos) Size() (n int) {
	var l int
	_ = l
	if m.Suffix != 0 {
		n += 1 + sovPump(uint64(m.Suffix))
	}
	if m.Offset != 0 {
		n += 1 + sovPump(uint64(m.Offset))
	}
	return n
}

func (m *Entity) Size() (n int) {
	var l int
	_ = l
	l = m.Pos.Size()
	n += 1 + l + sovPump(uint64(l))
	l = len(m.Payload)
	if l > 0 {
		n += 1 + l + sovPump(uint64(l))
	}
	return n
}

func sovPump(x uint64) (n int) {
	for {
		n++
		x >>= 7
		if x == 0 {
			break
		}
	}
	return n
}
func sozPump(x uint64) (n int) {
	return sovPump(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *WriteBinlogReq) Unmarshal(data []byte) error {
	l := len(data)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowPump
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := data[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: WriteBinlogReq: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: WriteBinlogReq: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field ClusterID", wireType)
			}
			m.ClusterID = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowPump
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				m.ClusterID |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Payload", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowPump
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				byteLen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if byteLen < 0 {
				return ErrInvalidLengthPump
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Payload = append(m.Payload[:0], data[iNdEx:postIndex]...)
			if m.Payload == nil {
				m.Payload = []byte{}
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipPump(data[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthPump
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
func (m *WriteBinlogResp) Unmarshal(data []byte) error {
	l := len(data)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowPump
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := data[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: WriteBinlogResp: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: WriteBinlogResp: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Errmsg", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowPump
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				stringLen |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			intStringLen := int(stringLen)
			if intStringLen < 0 {
				return ErrInvalidLengthPump
			}
			postIndex := iNdEx + intStringLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Errmsg = string(data[iNdEx:postIndex])
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipPump(data[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthPump
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
func (m *PullBinlogReq) Unmarshal(data []byte) error {
	l := len(data)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowPump
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := data[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: PullBinlogReq: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: PullBinlogReq: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field ClusterID", wireType)
			}
			m.ClusterID = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowPump
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				m.ClusterID |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field StartFrom", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowPump
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				msglen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthPump
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if err := m.StartFrom.Unmarshal(data[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 3:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Batch", wireType)
			}
			m.Batch = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowPump
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				m.Batch |= (int32(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		default:
			iNdEx = preIndex
			skippy, err := skipPump(data[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthPump
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
func (m *PullBinlogResp) Unmarshal(data []byte) error {
	l := len(data)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowPump
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := data[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: PullBinlogResp: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: PullBinlogResp: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Entity", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowPump
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				msglen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthPump
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if err := m.Entity.Unmarshal(data[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipPump(data[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthPump
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
func (m *Pos) Unmarshal(data []byte) error {
	l := len(data)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowPump
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := data[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: Pos: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: Pos: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Suffix", wireType)
			}
			m.Suffix = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowPump
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				m.Suffix |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Offset", wireType)
			}
			m.Offset = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowPump
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				m.Offset |= (int64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		default:
			iNdEx = preIndex
			skippy, err := skipPump(data[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthPump
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
func (m *Entity) Unmarshal(data []byte) error {
	l := len(data)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowPump
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := data[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: Entity: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: Entity: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Pos", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowPump
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				msglen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthPump
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if err := m.Pos.Unmarshal(data[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Payload", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowPump
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				byteLen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if byteLen < 0 {
				return ErrInvalidLengthPump
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Payload = append(m.Payload[:0], data[iNdEx:postIndex]...)
			if m.Payload == nil {
				m.Payload = []byte{}
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipPump(data[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthPump
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
func skipPump(data []byte) (n int, err error) {
	l := len(data)
	iNdEx := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowPump
			}
			if iNdEx >= l {
				return 0, io.ErrUnexpectedEOF
			}
			b := data[iNdEx]
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
					return 0, ErrIntOverflowPump
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				iNdEx++
				if data[iNdEx-1] < 0x80 {
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
					return 0, ErrIntOverflowPump
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				length |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			iNdEx += length
			if length < 0 {
				return 0, ErrInvalidLengthPump
			}
			return iNdEx, nil
		case 3:
			for {
				var innerWire uint64
				var start int = iNdEx
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return 0, ErrIntOverflowPump
					}
					if iNdEx >= l {
						return 0, io.ErrUnexpectedEOF
					}
					b := data[iNdEx]
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
				next, err := skipPump(data[start:])
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
	ErrInvalidLengthPump = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowPump   = fmt.Errorf("proto: integer overflow")
)

func init() { proto.RegisterFile("pump.proto", fileDescriptorPump) }

var fileDescriptorPump = []byte{
	// 355 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x09, 0x6e, 0x88, 0x02, 0xff, 0x8c, 0x92, 0xcf, 0x4a, 0xf3, 0x40,
	0x14, 0xc5, 0x3b, 0x5f, 0xda, 0x7c, 0xf4, 0x46, 0xab, 0x0c, 0xb5, 0x86, 0x22, 0xb1, 0x8c, 0x9b,
	0x0a, 0xd2, 0x4a, 0xc5, 0xad, 0x48, 0xf1, 0xef, 0x2e, 0xcc, 0xc6, 0x75, 0x5a, 0x93, 0x18, 0x48,
	0x3a, 0xe3, 0xcc, 0x04, 0xec, 0x2b, 0xf8, 0x04, 0x3e, 0x52, 0x97, 0x3e, 0x81, 0x48, 0x7d, 0x11,
	0xc9, 0x64, 0x6a, 0x5b, 0xa8, 0xe0, 0x2e, 0xe7, 0xcc, 0xcd, 0xef, 0xdc, 0x7b, 0x67, 0x00, 0x78,
	0x9e, 0xf1, 0x1e, 0x17, 0x4c, 0x31, 0x6c, 0x8f, 0x92, 0x49, 0xca, 0xe2, 0x76, 0x33, 0x66, 0x31,
	0xd3, 0x56, 0xbf, 0xf8, 0x2a, 0x4f, 0xc9, 0x1d, 0x34, 0x1e, 0x44, 0xa2, 0xc2, 0xa1, 0x2e, 0xa2,
	0xe1, 0x33, 0x3e, 0x80, 0xfa, 0x38, 0xcd, 0xa5, 0x0a, 0xc5, 0xfd, 0x95, 0x8b, 0x3a, 0xa8, 0x5b,
	0xa5, 0x4b, 0x03, 0xbb, 0xf0, 0x9f, 0x07, 0xd3, 0x94, 0x05, 0x8f, 0xee, 0xbf, 0x0e, 0xea, 0x6e,
	0xd1, 0x85, 0x24, 0xc7, 0xb0, 0xb3, 0x46, 0x92, 0x1c, 0xb7, 0xc0, 0x0e, 0x85, 0xc8, 0x64, 0xac,
	0x39, 0x75, 0x6a, 0x14, 0x51, 0xb0, 0xed, 0xe7, 0x69, 0xfa, 0xd7, 0xcc, 0x3e, 0xd4, 0xa5, 0x0a,
	0x84, 0xba, 0x11, 0x2c, 0xd3, 0xa9, 0xce, 0xc0, 0xe9, 0x95, 0x53, 0xf5, 0x7c, 0x26, 0x87, 0xd5,
	0xd9, 0xc7, 0x61, 0x85, 0x2e, 0x6b, 0x70, 0x13, 0x6a, 0xa3, 0x40, 0x8d, 0x9f, 0x5c, 0xab, 0x83,
	0xba, 0x35, 0x5a, 0x0a, 0x72, 0x01, 0x8d, 0xd5, 0x54, 0xc9, 0xf1, 0x09, 0xd8, 0xe1, 0x44, 0x25,
	0x6a, 0xaa, 0x33, 0x9d, 0x41, 0x63, 0x41, 0xbd, 0xd6, 0xae, 0x01, 0x9b, 0x1a, 0x72, 0x0e, 0x96,
	0xcf, 0x64, 0x31, 0x94, 0xcc, 0xa3, 0x28, 0x79, 0x31, 0x8d, 0x1a, 0x55, 0xf8, 0x2c, 0x8a, 0x64,
	0xa8, 0x74, 0x8b, 0x16, 0x35, 0x8a, 0xdc, 0x82, 0x5d, 0xe2, 0xf0, 0x11, 0x58, 0x9c, 0x49, 0x93,
	0xb5, 0x61, 0x82, 0xe2, 0xf4, 0xf7, 0x05, 0x0f, 0x5e, 0x11, 0x54, 0xfd, 0x3c, 0xe3, 0xf8, 0x12,
	0x9c, 0x95, 0x4d, 0xe3, 0xd6, 0x82, 0xb4, 0x7e, 0x91, 0xed, 0xfd, 0x8d, 0xbe, 0xe4, 0xa4, 0x52,
	0x10, 0x96, 0xab, 0x90, 0x78, 0xef, 0xa7, 0x97, 0xd5, 0x5b, 0x69, 0xb7, 0x36, 0xd9, 0xc5, 0xff,
	0xa7, 0x68, 0xb8, 0x3b, 0x9b, 0x7b, 0xe8, 0x7d, 0xee, 0xa1, 0xcf, 0xb9, 0x87, 0xde, 0xbe, 0xbc,
	0xca, 0xc8, 0xd6, 0x0f, 0xea, 0xec, 0x3b, 0x00, 0x00, 0xff, 0xff, 0x9c, 0x91, 0x56, 0xe6, 0x7c,
	0x02, 0x00, 0x00,
}
