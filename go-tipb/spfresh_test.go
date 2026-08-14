package tipb

import (
	"bytes"
	"testing"

	"github.com/gogo/protobuf/proto"
)

func TestSPFreshFilterExprColumnSourceWireValues(t *testing.T) {
	if got := int32(SPFreshFilterExprColumnSource_SPFreshFilterExprColumnSourceUnspecified); got != 0 {
		t.Fatalf("unspecified source wire value = %d, want 0", got)
	}
	if got := int32(SPFreshFilterExprColumnSource_Stored); got != 1 {
		t.Fatalf("stored source wire value = %d, want 1", got)
	}
	if got := int32(SPFreshFilterExprColumnSource_Handle); got != 2 {
		t.Fatalf("handle source wire value = %d, want 2", got)
	}
}

func TestSPFreshFilterExprColumnMissingSourceIsUnspecified(t *testing.T) {
	// Encodes only column_id = 42, leaving source absent from the wire payload.
	var column SPFreshFilterExprColumn
	if err := proto.Unmarshal([]byte{0x08, 0x2a}, &column); err != nil {
		t.Fatalf("unmarshal column: %v", err)
	}
	if got := column.GetSource(); got != SPFreshFilterExprColumnSource_SPFreshFilterExprColumnSourceUnspecified {
		t.Fatalf("missing source = %v, want unspecified", got)
	}
}

func TestSPFreshEvalContextPreservesSQLMode(t *testing.T) {
	const sqlMode = uint64(1<<6 | 1<<21 | 1<<26)

	encoded, err := proto.Marshal(&SPFreshEvalContext{SqlMode: sqlMode})
	if err != nil {
		t.Fatalf("proto.Marshal(SPFreshEvalContext) failed: %v", err)
	}
	var decoded SPFreshEvalContext
	if err := proto.Unmarshal(encoded, &decoded); err != nil {
		t.Fatalf("proto.Unmarshal(SPFreshEvalContext) failed: %v", err)
	}
	if got := decoded.GetSqlMode(); got != sqlMode {
		t.Fatalf("SPFreshEvalContext.GetSqlMode() = %#x, want %#x", got, sqlMode)
	}
}

func TestSPFreshSearchRequestMaxResponseBytesWireField(t *testing.T) {
	const maxResponseBytes = uint64(1 << 63)

	encoded, err := proto.Marshal(&SPFreshSearchRequest{MaxResponseBytes: maxResponseBytes})
	if err != nil {
		t.Fatalf("proto.Marshal(SPFreshSearchRequest) failed: %v", err)
	}
	// Field 15 with wire type 0, followed by the uint64 varint.
	want := []byte{0x78, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01}
	if !bytes.Equal(encoded, want) {
		t.Fatalf("max_response_bytes wire encoding = %x, want %x", encoded, want)
	}

	var decoded SPFreshSearchRequest
	if err := proto.Unmarshal(encoded, &decoded); err != nil {
		t.Fatalf("proto.Unmarshal(SPFreshSearchRequest) failed: %v", err)
	}
	if got := decoded.GetMaxResponseBytes(); got != maxResponseBytes {
		t.Fatalf("SPFreshSearchRequest.GetMaxResponseBytes() = %d, want %d", got, maxResponseBytes)
	}
}

func TestSPFreshSearchResponseResultIsExclusive(t *testing.T) {
	success := &SPFreshSearchResponse{
		Result: &SPFreshSearchResponse_Success{Success: &SPFreshSearchResult{
			WarningCount: 1 << 63,
		}},
	}
	encoded, err := proto.Marshal(success)
	if err != nil {
		t.Fatalf("marshal success response: %v", err)
	}
	var decoded SPFreshSearchResponse
	if err := proto.Unmarshal(encoded, &decoded); err != nil {
		t.Fatalf("unmarshal success response: %v", err)
	}
	if decoded.GetSuccess() == nil || decoded.GetError() != nil {
		t.Fatalf("success response result = %T, want success only", decoded.GetResult())
	}
	if got := decoded.GetSuccess().GetWarningCount(); got != uint64(1)<<63 {
		t.Fatalf("warning_count = %d, want %d", got, uint64(1)<<63)
	}

	errorResponse := &SPFreshSearchResponse{
		Result: &SPFreshSearchResponse_Error{Error: &Error{Code: int32(SPFreshErrorCode_SPFreshIndexCorruption)}},
	}
	encoded, err = proto.Marshal(errorResponse)
	if err != nil {
		t.Fatalf("marshal error response: %v", err)
	}
	decoded.Reset()
	if err := proto.Unmarshal(encoded, &decoded); err != nil {
		t.Fatalf("unmarshal error response: %v", err)
	}
	if decoded.GetSuccess() != nil || decoded.GetError() == nil {
		t.Fatalf("error response result = %T, want error only", decoded.GetResult())
	}

	decoded.Reset()
	if decoded.GetResult() != nil || decoded.GetSuccess() != nil || decoded.GetError() != nil {
		t.Fatalf("empty response result = %T, want unset", decoded.GetResult())
	}
}

func TestSPFreshIndexCorruptionCode(t *testing.T) {
	if got := int32(SPFreshErrorCode_SPFreshIndexCorruption); got != 9015 {
		t.Fatalf("SPFreshIndexCorruption code = %d, want 9015", got)
	}
}

func TestSPFreshResponseTooLargeCode(t *testing.T) {
	if got := int32(SPFreshErrorCode_SPFreshResponseTooLarge); got != 9016 {
		t.Fatalf("SPFreshResponseTooLarge code = %d, want 9016", got)
	}
}
