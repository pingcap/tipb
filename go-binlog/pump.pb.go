// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: pump.proto

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

	context "golang.org/x/net/context"

	grpc "google.golang.org/grpc"

	io "io"
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

func (m *WriteBinlogReq) GetClusterID() uint64 {
	if m != nil {
		return m.ClusterID
	}
	return 0
}

func (m *WriteBinlogReq) GetPayload() []byte {
	if m != nil {
		return m.Payload
	}
	return nil
}

type WriteBinlogResp struct {
	// An empty errmsg returned means a successful write.
	// Otherwise return the error description.
	Errmsg string `protobuf:"bytes,1,opt,name=errmsg,proto3" json:"errmsg,omitempty"`
}

func (m *WriteBinlogResp) Reset()                    { *m = WriteBinlogResp{} }
func (m *WriteBinlogResp) String() string            { return proto.CompactTextString(m) }
func (*WriteBinlogResp) ProtoMessage()               {}
func (*WriteBinlogResp) Descriptor() ([]byte, []int) { return fileDescriptorPump, []int{1} }

func (m *WriteBinlogResp) GetErrmsg() string {
	if m != nil {
		return m.Errmsg
	}
	return ""
}

type PullBinlogReq struct {
	// Specifies which clusterID of binlog to pull.
	ClusterID uint64 `protobuf:"varint,1,opt,name=clusterID,proto3" json:"clusterID,omitempty"`
	// The position from which the binlog will be sent.
	StartFrom Pos `protobuf:"bytes,2,opt,name=startFrom" json:"startFrom"`
}

func (m *PullBinlogReq) Reset()                    { *m = PullBinlogReq{} }
func (m *PullBinlogReq) String() string            { return proto.CompactTextString(m) }
func (*PullBinlogReq) ProtoMessage()               {}
func (*PullBinlogReq) Descriptor() ([]byte, []int) { return fileDescriptorPump, []int{2} }

func (m *PullBinlogReq) GetClusterID() uint64 {
	if m != nil {
		return m.ClusterID
	}
	return 0
}

func (m *PullBinlogReq) GetStartFrom() Pos {
	if m != nil {
		return m.StartFrom
	}
	return Pos{}
}

type PullBinlogResp struct {
	// The binlog entity that send in a stream
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
	// The binlog's ts, start ts for prewrite binlog, and commit ts for commit binlog.
	Ts int64 `protobuf:"varint,3,opt,name=ts,proto3" json:"ts,omitempty"`
}

func (m *Pos) Reset()                    { *m = Pos{} }
func (m *Pos) String() string            { return proto.CompactTextString(m) }
func (*Pos) ProtoMessage()               {}
func (*Pos) Descriptor() ([]byte, []int) { return fileDescriptorPump, []int{4} }

func (m *Pos) GetSuffix() uint64 {
	if m != nil {
		return m.Suffix
	}
	return 0
}

func (m *Pos) GetOffset() int64 {
	if m != nil {
		return m.Offset
	}
	return 0
}

func (m *Pos) GetTs() int64 {
	if m != nil {
		return m.Ts
	}
	return 0
}

type Entity struct {
	// The position of the binlog entity.
	Pos Pos `protobuf:"bytes,1,opt,name=pos" json:"pos"`
	// The payload of binlog entity.
	Payload []byte `protobuf:"bytes,2,opt,name=payload,proto3" json:"payload,omitempty"`
	// checksum of binlog payload.
	Checksum []byte `protobuf:"bytes,3,opt,name=checksum,proto3" json:"checksum,omitempty"`
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

func (m *Entity) GetPayload() []byte {
	if m != nil {
		return m.Payload
	}
	return nil
}

func (m *Entity) GetChecksum() []byte {
	if m != nil {
		return m.Checksum
	}
	return nil
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
const _ = grpc.SupportPackageIsVersion4

// Client API for Pump service

type PumpClient interface {
	// Writes a binlog to the local file on the pump machine.
	// A response with an empty errmsg is returned if the binlog is written successfully.
	WriteBinlog(ctx context.Context, in *WriteBinlogReq, opts ...grpc.CallOption) (*WriteBinlogResp, error)
	// Sends binlog stream from a given location.
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
	// Sends binlog stream from a given location.
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
	Metadata: "pump.proto",
}

func (m *WriteBinlogReq) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *WriteBinlogReq) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.ClusterID != 0 {
		dAtA[i] = 0x8
		i++
		i = encodeVarintPump(dAtA, i, uint64(m.ClusterID))
	}
	if len(m.Payload) > 0 {
		dAtA[i] = 0x12
		i++
		i = encodeVarintPump(dAtA, i, uint64(len(m.Payload)))
		i += copy(dAtA[i:], m.Payload)
	}
	return i, nil
}

func (m *WriteBinlogResp) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *WriteBinlogResp) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if len(m.Errmsg) > 0 {
		dAtA[i] = 0xa
		i++
		i = encodeVarintPump(dAtA, i, uint64(len(m.Errmsg)))
		i += copy(dAtA[i:], m.Errmsg)
	}
	return i, nil
}

func (m *PullBinlogReq) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *PullBinlogReq) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.ClusterID != 0 {
		dAtA[i] = 0x8
		i++
		i = encodeVarintPump(dAtA, i, uint64(m.ClusterID))
	}
	dAtA[i] = 0x12
	i++
	i = encodeVarintPump(dAtA, i, uint64(m.StartFrom.Size()))
	n1, err := m.StartFrom.MarshalTo(dAtA[i:])
	if err != nil {
		return 0, err
	}
	i += n1
	return i, nil
}

func (m *PullBinlogResp) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *PullBinlogResp) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	dAtA[i] = 0xa
	i++
	i = encodeVarintPump(dAtA, i, uint64(m.Entity.Size()))
	n2, err := m.Entity.MarshalTo(dAtA[i:])
	if err != nil {
		return 0, err
	}
	i += n2
	return i, nil
}

func (m *Pos) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *Pos) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.Suffix != 0 {
		dAtA[i] = 0x8
		i++
		i = encodeVarintPump(dAtA, i, uint64(m.Suffix))
	}
	if m.Offset != 0 {
		dAtA[i] = 0x10
		i++
		i = encodeVarintPump(dAtA, i, uint64(m.Offset))
	}
	if m.Ts != 0 {
		dAtA[i] = 0x18
		i++
		i = encodeVarintPump(dAtA, i, uint64(m.Ts))
	}
	return i, nil
}

func (m *Entity) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *Entity) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	dAtA[i] = 0xa
	i++
	i = encodeVarintPump(dAtA, i, uint64(m.Pos.Size()))
	n3, err := m.Pos.MarshalTo(dAtA[i:])
	if err != nil {
		return 0, err
	}
	i += n3
	if len(m.Payload) > 0 {
		dAtA[i] = 0x12
		i++
		i = encodeVarintPump(dAtA, i, uint64(len(m.Payload)))
		i += copy(dAtA[i:], m.Payload)
	}
	if len(m.Checksum) > 0 {
		dAtA[i] = 0x1a
		i++
		i = encodeVarintPump(dAtA, i, uint64(len(m.Checksum)))
		i += copy(dAtA[i:], m.Checksum)
	}
	return i, nil
}

func encodeVarintPump(dAtA []byte, offset int, v uint64) int {
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
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
	if m.Ts != 0 {
		n += 1 + sovPump(uint64(m.Ts))
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
	l = len(m.Checksum)
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
func (m *WriteBinlogReq) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
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
				b := dAtA[iNdEx]
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
				b := dAtA[iNdEx]
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
			m.Payload = append(m.Payload[:0], dAtA[iNdEx:postIndex]...)
			if m.Payload == nil {
				m.Payload = []byte{}
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipPump(dAtA[iNdEx:])
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
func (m *WriteBinlogResp) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
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
				b := dAtA[iNdEx]
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
			m.Errmsg = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipPump(dAtA[iNdEx:])
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
func (m *PullBinlogReq) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
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
				b := dAtA[iNdEx]
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
				b := dAtA[iNdEx]
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
			if err := m.StartFrom.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipPump(dAtA[iNdEx:])
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
func (m *PullBinlogResp) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
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
				b := dAtA[iNdEx]
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
			if err := m.Entity.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipPump(dAtA[iNdEx:])
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
func (m *Pos) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
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
				b := dAtA[iNdEx]
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
				b := dAtA[iNdEx]
				iNdEx++
				m.Offset |= (int64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 3:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Ts", wireType)
			}
			m.Ts = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowPump
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.Ts |= (int64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		default:
			iNdEx = preIndex
			skippy, err := skipPump(dAtA[iNdEx:])
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
func (m *Entity) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
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
				b := dAtA[iNdEx]
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
			if err := m.Pos.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
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
				b := dAtA[iNdEx]
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
			m.Payload = append(m.Payload[:0], dAtA[iNdEx:postIndex]...)
			if m.Payload == nil {
				m.Payload = []byte{}
			}
			iNdEx = postIndex
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Checksum", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowPump
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
				return ErrInvalidLengthPump
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Checksum = append(m.Checksum[:0], dAtA[iNdEx:postIndex]...)
			if m.Checksum == nil {
				m.Checksum = []byte{}
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipPump(dAtA[iNdEx:])
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
func skipPump(dAtA []byte) (n int, err error) {
	l := len(dAtA)
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
					return 0, ErrIntOverflowPump
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
					return 0, ErrIntOverflowPump
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
				next, err := skipPump(dAtA[start:])
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
	// 365 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x8c, 0x92, 0xcf, 0x4a, 0xf3, 0x40,
	0x14, 0xc5, 0x3b, 0x4d, 0xc9, 0xf7, 0xf5, 0xa6, 0x46, 0x19, 0xb4, 0x86, 0x20, 0xb1, 0xc4, 0x4d,
	0x05, 0x69, 0xa5, 0xee, 0x45, 0x8a, 0x15, 0xdd, 0x85, 0x6c, 0xdc, 0x09, 0x6d, 0x4c, 0x62, 0x30,
	0xe9, 0x8c, 0xb9, 0x13, 0xb0, 0xaf, 0xe0, 0x13, 0xf8, 0x48, 0x5d, 0xfa, 0x04, 0x22, 0xf5, 0x45,
	0x24, 0xd3, 0xe9, 0x3f, 0xa8, 0xe0, 0x2e, 0xe7, 0xcc, 0x9d, 0xdf, 0x3d, 0xb9, 0x77, 0x00, 0x78,
	0x91, 0xf1, 0x0e, 0xcf, 0x99, 0x60, 0x54, 0x1f, 0x25, 0xe3, 0x94, 0xc5, 0xf6, 0x7e, 0xcc, 0x62,
	0x26, 0xad, 0x6e, 0xf9, 0x35, 0x3f, 0x75, 0x6f, 0xc1, 0xbc, 0xcf, 0x13, 0x11, 0xf6, 0x65, 0x91,
	0x1f, 0xbe, 0xd0, 0x23, 0xa8, 0x07, 0x69, 0x81, 0x22, 0xcc, 0xef, 0xae, 0x2d, 0xd2, 0x22, 0xed,
	0x9a, 0xbf, 0x32, 0xa8, 0x05, 0xff, 0xf8, 0x70, 0x92, 0xb2, 0xe1, 0xa3, 0x55, 0x6d, 0x91, 0x76,
	0xc3, 0x5f, 0x48, 0xf7, 0x14, 0x76, 0x37, 0x48, 0xc8, 0x69, 0x13, 0xf4, 0x30, 0xcf, 0x33, 0x8c,
	0x25, 0xa7, 0xee, 0x2b, 0xe5, 0x3e, 0xc0, 0x8e, 0x57, 0xa4, 0xe9, 0x5f, 0x7b, 0x76, 0xa1, 0x8e,
	0x62, 0x98, 0x8b, 0x9b, 0x9c, 0x65, 0xb2, 0xab, 0xd1, 0x33, 0x3a, 0xf3, 0xbf, 0xea, 0x78, 0x0c,
	0xfb, 0xb5, 0xe9, 0xe7, 0x71, 0xc5, 0x5f, 0xd5, 0xb8, 0x97, 0x60, 0xae, 0xf3, 0x91, 0xd3, 0x33,
	0xd0, 0xc3, 0xb1, 0x48, 0xc4, 0x44, 0xd2, 0x8d, 0x9e, 0xb9, 0xb8, 0x3f, 0x90, 0xae, 0x42, 0xa8,
	0x1a, 0x77, 0x00, 0x9a, 0xc7, 0xb0, 0x8c, 0x8f, 0x45, 0x14, 0x25, 0xaf, 0x2a, 0x92, 0x52, 0xa5,
	0xcf, 0xa2, 0x08, 0x43, 0x21, 0xc3, 0x68, 0xbe, 0x52, 0xd4, 0x84, 0xaa, 0x40, 0x4b, 0x93, 0x5e,
	0x55, 0xa0, 0x1b, 0x80, 0x3e, 0xc7, 0xd3, 0x13, 0xd0, 0x38, 0x43, 0xd5, 0x7b, 0x4b, 0xf6, 0xf2,
	0xf4, 0xf7, 0xd1, 0x52, 0x1b, 0xfe, 0x07, 0x4f, 0x61, 0xf0, 0x8c, 0x45, 0x26, 0xf1, 0x0d, 0x7f,
	0xa9, 0x7b, 0x6f, 0x04, 0x6a, 0x5e, 0x91, 0x71, 0x7a, 0x05, 0xc6, 0xda, 0xfc, 0x69, 0x73, 0xd1,
	0x65, 0x73, 0xbd, 0xf6, 0xe1, 0x56, 0x1f, 0xb9, 0x5b, 0x29, 0x09, 0xab, 0xb1, 0x21, 0x3d, 0x58,
	0xe6, 0x5c, 0xdf, 0x95, 0xdd, 0xdc, 0x66, 0x97, 0xf7, 0xcf, 0x49, 0x7f, 0x6f, 0x3a, 0x73, 0xc8,
	0xc7, 0xcc, 0x21, 0x5f, 0x33, 0x87, 0xbc, 0x7f, 0x3b, 0x95, 0x91, 0x2e, 0x9f, 0xd9, 0xc5, 0x4f,
	0x00, 0x00, 0x00, 0xff, 0xff, 0xb1, 0xac, 0xf5, 0x23, 0x92, 0x02, 0x00, 0x00,
}
