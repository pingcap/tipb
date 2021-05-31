// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: topsql_agent.proto

package tipb

import (
	"fmt"

	proto "github.com/golang/protobuf/proto"

	math "math"

	google_protobuf1 "github.com/gogo/protobuf/types"

	context "golang.org/x/net/context"

	grpc "google.golang.org/grpc"

	io "io"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

type CPUTimeRequestTiDB struct {
	TimestampList  []uint64 `protobuf:"varint,1,rep,packed,name=timestamp_list,json=timestampList" json:"timestamp_list,omitempty"`
	CpuTimeMsList  []uint32 `protobuf:"varint,2,rep,packed,name=cpu_time_ms_list,json=cpuTimeMsList" json:"cpu_time_ms_list,omitempty"`
	SqlDigest      string   `protobuf:"bytes,3,opt,name=sql_digest,json=sqlDigest,proto3" json:"sql_digest,omitempty"`
	NormalizedSql  string   `protobuf:"bytes,4,opt,name=normalized_sql,json=normalizedSql,proto3" json:"normalized_sql,omitempty"`
	PlanDigest     string   `protobuf:"bytes,5,opt,name=plan_digest,json=planDigest,proto3" json:"plan_digest,omitempty"`
	NormalizedPlan string   `protobuf:"bytes,6,opt,name=normalized_plan,json=normalizedPlan,proto3" json:"normalized_plan,omitempty"`
}

func (m *CPUTimeRequestTiDB) Reset()                    { *m = CPUTimeRequestTiDB{} }
func (m *CPUTimeRequestTiDB) String() string            { return proto.CompactTextString(m) }
func (*CPUTimeRequestTiDB) ProtoMessage()               {}
func (*CPUTimeRequestTiDB) Descriptor() ([]byte, []int) { return fileDescriptorTopsqlAgent, []int{0} }

func (m *CPUTimeRequestTiDB) GetTimestampList() []uint64 {
	if m != nil {
		return m.TimestampList
	}
	return nil
}

func (m *CPUTimeRequestTiDB) GetCpuTimeMsList() []uint32 {
	if m != nil {
		return m.CpuTimeMsList
	}
	return nil
}

func (m *CPUTimeRequestTiDB) GetSqlDigest() string {
	if m != nil {
		return m.SqlDigest
	}
	return ""
}

func (m *CPUTimeRequestTiDB) GetNormalizedSql() string {
	if m != nil {
		return m.NormalizedSql
	}
	return ""
}

func (m *CPUTimeRequestTiDB) GetPlanDigest() string {
	if m != nil {
		return m.PlanDigest
	}
	return ""
}

func (m *CPUTimeRequestTiDB) GetNormalizedPlan() string {
	if m != nil {
		return m.NormalizedPlan
	}
	return ""
}

func init() {
	proto.RegisterType((*CPUTimeRequestTiDB)(nil), "tipb.CPUTimeRequestTiDB")
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConn

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion4

// Client API for TopSQLAgent service

type TopSQLAgentClient interface {
	// CollectTiDB is called periodically (e.g. per minute) to save the in-memory TopSQL records
	CollectTiDB(ctx context.Context, opts ...grpc.CallOption) (TopSQLAgent_CollectTiDBClient, error)
}

type topSQLAgentClient struct {
	cc *grpc.ClientConn
}

func NewTopSQLAgentClient(cc *grpc.ClientConn) TopSQLAgentClient {
	return &topSQLAgentClient{cc}
}

func (c *topSQLAgentClient) CollectTiDB(ctx context.Context, opts ...grpc.CallOption) (TopSQLAgent_CollectTiDBClient, error) {
	stream, err := grpc.NewClientStream(ctx, &_TopSQLAgent_serviceDesc.Streams[0], c.cc, "/tipb.TopSQLAgent/CollectTiDB", opts...)
	if err != nil {
		return nil, err
	}
	x := &topSQLAgentCollectTiDBClient{stream}
	return x, nil
}

type TopSQLAgent_CollectTiDBClient interface {
	Send(*CPUTimeRequestTiDB) error
	CloseAndRecv() (*google_protobuf1.Empty, error)
	grpc.ClientStream
}

type topSQLAgentCollectTiDBClient struct {
	grpc.ClientStream
}

func (x *topSQLAgentCollectTiDBClient) Send(m *CPUTimeRequestTiDB) error {
	return x.ClientStream.SendMsg(m)
}

func (x *topSQLAgentCollectTiDBClient) CloseAndRecv() (*google_protobuf1.Empty, error) {
	if err := x.ClientStream.CloseSend(); err != nil {
		return nil, err
	}
	m := new(google_protobuf1.Empty)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

// Server API for TopSQLAgent service

type TopSQLAgentServer interface {
	// CollectTiDB is called periodically (e.g. per minute) to save the in-memory TopSQL records
	CollectTiDB(TopSQLAgent_CollectTiDBServer) error
}

func RegisterTopSQLAgentServer(s *grpc.Server, srv TopSQLAgentServer) {
	s.RegisterService(&_TopSQLAgent_serviceDesc, srv)
}

func _TopSQLAgent_CollectTiDB_Handler(srv interface{}, stream grpc.ServerStream) error {
	return srv.(TopSQLAgentServer).CollectTiDB(&topSQLAgentCollectTiDBServer{stream})
}

type TopSQLAgent_CollectTiDBServer interface {
	SendAndClose(*google_protobuf1.Empty) error
	Recv() (*CPUTimeRequestTiDB, error)
	grpc.ServerStream
}

type topSQLAgentCollectTiDBServer struct {
	grpc.ServerStream
}

func (x *topSQLAgentCollectTiDBServer) SendAndClose(m *google_protobuf1.Empty) error {
	return x.ServerStream.SendMsg(m)
}

func (x *topSQLAgentCollectTiDBServer) Recv() (*CPUTimeRequestTiDB, error) {
	m := new(CPUTimeRequestTiDB)
	if err := x.ServerStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

var _TopSQLAgent_serviceDesc = grpc.ServiceDesc{
	ServiceName: "tipb.TopSQLAgent",
	HandlerType: (*TopSQLAgentServer)(nil),
	Methods:     []grpc.MethodDesc{},
	Streams: []grpc.StreamDesc{
		{
			StreamName:    "CollectTiDB",
			Handler:       _TopSQLAgent_CollectTiDB_Handler,
			ClientStreams: true,
		},
	},
	Metadata: "topsql_agent.proto",
}

func (m *CPUTimeRequestTiDB) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *CPUTimeRequestTiDB) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if len(m.TimestampList) > 0 {
		dAtA2 := make([]byte, len(m.TimestampList)*10)
		var j1 int
		for _, num := range m.TimestampList {
			for num >= 1<<7 {
				dAtA2[j1] = uint8(uint64(num)&0x7f | 0x80)
				num >>= 7
				j1++
			}
			dAtA2[j1] = uint8(num)
			j1++
		}
		dAtA[i] = 0xa
		i++
		i = encodeVarintTopsqlAgent(dAtA, i, uint64(j1))
		i += copy(dAtA[i:], dAtA2[:j1])
	}
	if len(m.CpuTimeMsList) > 0 {
		dAtA4 := make([]byte, len(m.CpuTimeMsList)*10)
		var j3 int
		for _, num := range m.CpuTimeMsList {
			for num >= 1<<7 {
				dAtA4[j3] = uint8(uint64(num)&0x7f | 0x80)
				num >>= 7
				j3++
			}
			dAtA4[j3] = uint8(num)
			j3++
		}
		dAtA[i] = 0x12
		i++
		i = encodeVarintTopsqlAgent(dAtA, i, uint64(j3))
		i += copy(dAtA[i:], dAtA4[:j3])
	}
	if len(m.SqlDigest) > 0 {
		dAtA[i] = 0x1a
		i++
		i = encodeVarintTopsqlAgent(dAtA, i, uint64(len(m.SqlDigest)))
		i += copy(dAtA[i:], m.SqlDigest)
	}
	if len(m.NormalizedSql) > 0 {
		dAtA[i] = 0x22
		i++
		i = encodeVarintTopsqlAgent(dAtA, i, uint64(len(m.NormalizedSql)))
		i += copy(dAtA[i:], m.NormalizedSql)
	}
	if len(m.PlanDigest) > 0 {
		dAtA[i] = 0x2a
		i++
		i = encodeVarintTopsqlAgent(dAtA, i, uint64(len(m.PlanDigest)))
		i += copy(dAtA[i:], m.PlanDigest)
	}
	if len(m.NormalizedPlan) > 0 {
		dAtA[i] = 0x32
		i++
		i = encodeVarintTopsqlAgent(dAtA, i, uint64(len(m.NormalizedPlan)))
		i += copy(dAtA[i:], m.NormalizedPlan)
	}
	return i, nil
}

func encodeVarintTopsqlAgent(dAtA []byte, offset int, v uint64) int {
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return offset + 1
}
func (m *CPUTimeRequestTiDB) Size() (n int) {
	var l int
	_ = l
	if len(m.TimestampList) > 0 {
		l = 0
		for _, e := range m.TimestampList {
			l += sovTopsqlAgent(uint64(e))
		}
		n += 1 + sovTopsqlAgent(uint64(l)) + l
	}
	if len(m.CpuTimeMsList) > 0 {
		l = 0
		for _, e := range m.CpuTimeMsList {
			l += sovTopsqlAgent(uint64(e))
		}
		n += 1 + sovTopsqlAgent(uint64(l)) + l
	}
	l = len(m.SqlDigest)
	if l > 0 {
		n += 1 + l + sovTopsqlAgent(uint64(l))
	}
	l = len(m.NormalizedSql)
	if l > 0 {
		n += 1 + l + sovTopsqlAgent(uint64(l))
	}
	l = len(m.PlanDigest)
	if l > 0 {
		n += 1 + l + sovTopsqlAgent(uint64(l))
	}
	l = len(m.NormalizedPlan)
	if l > 0 {
		n += 1 + l + sovTopsqlAgent(uint64(l))
	}
	return n
}

func sovTopsqlAgent(x uint64) (n int) {
	for {
		n++
		x >>= 7
		if x == 0 {
			break
		}
	}
	return n
}
func sozTopsqlAgent(x uint64) (n int) {
	return sovTopsqlAgent(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *CPUTimeRequestTiDB) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowTopsqlAgent
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
			return fmt.Errorf("proto: CPUTimeRequestTiDB: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: CPUTimeRequestTiDB: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType == 0 {
				var v uint64
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return ErrIntOverflowTopsqlAgent
					}
					if iNdEx >= l {
						return io.ErrUnexpectedEOF
					}
					b := dAtA[iNdEx]
					iNdEx++
					v |= (uint64(b) & 0x7F) << shift
					if b < 0x80 {
						break
					}
				}
				m.TimestampList = append(m.TimestampList, v)
			} else if wireType == 2 {
				var packedLen int
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return ErrIntOverflowTopsqlAgent
					}
					if iNdEx >= l {
						return io.ErrUnexpectedEOF
					}
					b := dAtA[iNdEx]
					iNdEx++
					packedLen |= (int(b) & 0x7F) << shift
					if b < 0x80 {
						break
					}
				}
				if packedLen < 0 {
					return ErrInvalidLengthTopsqlAgent
				}
				postIndex := iNdEx + packedLen
				if postIndex > l {
					return io.ErrUnexpectedEOF
				}
				for iNdEx < postIndex {
					var v uint64
					for shift := uint(0); ; shift += 7 {
						if shift >= 64 {
							return ErrIntOverflowTopsqlAgent
						}
						if iNdEx >= l {
							return io.ErrUnexpectedEOF
						}
						b := dAtA[iNdEx]
						iNdEx++
						v |= (uint64(b) & 0x7F) << shift
						if b < 0x80 {
							break
						}
					}
					m.TimestampList = append(m.TimestampList, v)
				}
			} else {
				return fmt.Errorf("proto: wrong wireType = %d for field TimestampList", wireType)
			}
		case 2:
			if wireType == 0 {
				var v uint32
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return ErrIntOverflowTopsqlAgent
					}
					if iNdEx >= l {
						return io.ErrUnexpectedEOF
					}
					b := dAtA[iNdEx]
					iNdEx++
					v |= (uint32(b) & 0x7F) << shift
					if b < 0x80 {
						break
					}
				}
				m.CpuTimeMsList = append(m.CpuTimeMsList, v)
			} else if wireType == 2 {
				var packedLen int
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return ErrIntOverflowTopsqlAgent
					}
					if iNdEx >= l {
						return io.ErrUnexpectedEOF
					}
					b := dAtA[iNdEx]
					iNdEx++
					packedLen |= (int(b) & 0x7F) << shift
					if b < 0x80 {
						break
					}
				}
				if packedLen < 0 {
					return ErrInvalidLengthTopsqlAgent
				}
				postIndex := iNdEx + packedLen
				if postIndex > l {
					return io.ErrUnexpectedEOF
				}
				for iNdEx < postIndex {
					var v uint32
					for shift := uint(0); ; shift += 7 {
						if shift >= 64 {
							return ErrIntOverflowTopsqlAgent
						}
						if iNdEx >= l {
							return io.ErrUnexpectedEOF
						}
						b := dAtA[iNdEx]
						iNdEx++
						v |= (uint32(b) & 0x7F) << shift
						if b < 0x80 {
							break
						}
					}
					m.CpuTimeMsList = append(m.CpuTimeMsList, v)
				}
			} else {
				return fmt.Errorf("proto: wrong wireType = %d for field CpuTimeMsList", wireType)
			}
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field SqlDigest", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowTopsqlAgent
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
				return ErrInvalidLengthTopsqlAgent
			}
			postIndex := iNdEx + intStringLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.SqlDigest = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field NormalizedSql", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowTopsqlAgent
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
				return ErrInvalidLengthTopsqlAgent
			}
			postIndex := iNdEx + intStringLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.NormalizedSql = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 5:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field PlanDigest", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowTopsqlAgent
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
				return ErrInvalidLengthTopsqlAgent
			}
			postIndex := iNdEx + intStringLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.PlanDigest = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 6:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field NormalizedPlan", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowTopsqlAgent
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
				return ErrInvalidLengthTopsqlAgent
			}
			postIndex := iNdEx + intStringLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.NormalizedPlan = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipTopsqlAgent(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthTopsqlAgent
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
func skipTopsqlAgent(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowTopsqlAgent
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
					return 0, ErrIntOverflowTopsqlAgent
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
					return 0, ErrIntOverflowTopsqlAgent
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
				return 0, ErrInvalidLengthTopsqlAgent
			}
			return iNdEx, nil
		case 3:
			for {
				var innerWire uint64
				var start int = iNdEx
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return 0, ErrIntOverflowTopsqlAgent
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
				next, err := skipTopsqlAgent(dAtA[start:])
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
	ErrInvalidLengthTopsqlAgent = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowTopsqlAgent   = fmt.Errorf("proto: integer overflow")
)

func init() { proto.RegisterFile("topsql_agent.proto", fileDescriptorTopsqlAgent) }

var fileDescriptorTopsqlAgent = []byte{
	// 334 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x64, 0x90, 0x51, 0x4a, 0xc3, 0x30,
	0x1c, 0xc6, 0x17, 0x37, 0x07, 0xcb, 0xa8, 0x4a, 0x50, 0x29, 0x13, 0xeb, 0x18, 0xc8, 0xea, 0x4b,
	0x06, 0x7a, 0x02, 0xb7, 0xf9, 0x36, 0x61, 0x76, 0xf3, 0xb9, 0xa4, 0x5d, 0x0c, 0x81, 0xa4, 0x49,
	0x97, 0xf4, 0x41, 0x4f, 0xe2, 0x91, 0x7c, 0xf4, 0x08, 0x32, 0xaf, 0xe0, 0x01, 0x24, 0xa9, 0xd3,
	0x81, 0x6f, 0xc9, 0xf7, 0xff, 0xf2, 0x4b, 0xf2, 0x83, 0xc8, 0x2a, 0x6d, 0x4a, 0x91, 0x12, 0x46,
	0x0b, 0x8b, 0xf5, 0x5a, 0x59, 0x85, 0x5a, 0x96, 0xeb, 0xac, 0x77, 0xcc, 0x14, 0x53, 0x3e, 0x18,
	0xb9, 0x55, 0x3d, 0xeb, 0x9d, 0x31, 0xa5, 0x98, 0xa0, 0x23, 0xbf, 0xcb, 0xaa, 0xa7, 0x11, 0x95,
	0xda, 0x3e, 0xd7, 0xc3, 0xc1, 0x17, 0x80, 0x68, 0x32, 0x7f, 0x5c, 0x72, 0x49, 0x13, 0x5a, 0x56,
	0xd4, 0xd8, 0x25, 0x9f, 0x8e, 0xd1, 0x25, 0x3c, 0xb0, 0x5c, 0x52, 0x63, 0x89, 0xd4, 0xa9, 0xe0,
	0xc6, 0x86, 0xa0, 0xdf, 0x8c, 0x5b, 0x49, 0xf0, 0x9b, 0xce, 0xb8, 0xb1, 0x68, 0x08, 0x8f, 0x72,
	0x5d, 0xa5, 0x2e, 0x4c, 0xa5, 0xa9, 0x8b, 0x7b, 0xfd, 0x66, 0x1c, 0x24, 0x41, 0xae, 0x2b, 0x07,
	0xbd, 0x37, 0xbe, 0x78, 0x0e, 0xa1, 0x7b, 0xf2, 0x8a, 0x33, 0x6a, 0x6c, 0xd8, 0xec, 0x83, 0xb8,
	0x93, 0x74, 0x4c, 0x29, 0xa6, 0x3e, 0x70, 0xd7, 0x15, 0x6a, 0x2d, 0x89, 0xe0, 0x2f, 0x74, 0x95,
	0x9a, 0x52, 0x84, 0x2d, 0x5f, 0x09, 0xfe, 0xd2, 0x45, 0x29, 0xd0, 0x05, 0xec, 0x6a, 0x41, 0x8a,
	0x2d, 0x66, 0xdf, 0x77, 0xa0, 0x8b, 0x7e, 0x38, 0x43, 0x78, 0xb8, 0xc3, 0x71, 0x83, 0xb0, 0xed,
	0x4b, 0x3b, 0xf8, 0xb9, 0x20, 0xc5, 0x75, 0x02, 0xbb, 0x4b, 0xa5, 0x17, 0x0f, 0xb3, 0x5b, 0x27,
	0x11, 0x4d, 0x60, 0x77, 0xa2, 0x84, 0xa0, 0x79, 0xfd, 0xfb, 0x10, 0x3b, 0x9d, 0xf8, 0xbf, 0x97,
	0xde, 0x29, 0xae, 0x65, 0xe2, 0xad, 0x4c, 0x7c, 0xe7, 0x64, 0x0e, 0x1a, 0x31, 0x18, 0x5f, 0xbd,
	0x6d, 0x22, 0xf0, 0xbe, 0x89, 0xc0, 0xc7, 0x26, 0x02, 0xaf, 0x9f, 0x51, 0x03, 0x9e, 0xe4, 0x4a,
	0x62, 0xcd, 0x0b, 0x96, 0x13, 0x8d, 0x2d, 0x5f, 0x65, 0x9e, 0x3b, 0x07, 0x59, 0xdb, 0x1f, 0xbf,
	0xf9, 0x0e, 0x00, 0x00, 0xff, 0xff, 0x6b, 0x73, 0x8e, 0x40, 0xcb, 0x01, 0x00, 0x00,
}
