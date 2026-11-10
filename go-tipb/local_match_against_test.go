package tipb

import (
	"testing"

	"github.com/gogo/protobuf/proto"
)

func TestLocalMatchAgainstBooleanQueryWireRoundTrip(t *testing.T) {
	want := &LocalMatchAgainstBooleanQuery{
		Nodes: []*LocalMatchAgainstBooleanNode{{
			Occur:    LocalMatchAgainstBooleanOccur_LocalMatchAgainstBooleanOccurMust,
			TermType: LocalMatchAgainstBooleanTermType_LocalMatchAgainstBooleanTermPhrase,
			Text:     "hello world",
		}},
		Parser:               LocalMatchAgainstParser_LocalMatchAgainstParserNgram,
		NgramTokenSize:       2,
		InnodbFtMinTokenSize: 1,
		InnodbFtMaxTokenSize: 84,
		StopwordMode:         LocalMatchAgainstStopwordMode_LocalMatchAgainstStopwordModeBuiltin,
		StopwordCollation:    "utf8mb4_unicode_ci",
		Version:              2,
	}

	encoded, err := proto.Marshal(want)
	if err != nil {
		t.Fatalf("marshal LocalMatchAgainstBooleanQuery: %v", err)
	}

	var got LocalMatchAgainstBooleanQuery
	if err := proto.Unmarshal(encoded, &got); err != nil {
		t.Fatalf("unmarshal LocalMatchAgainstBooleanQuery: %v", err)
	}
	if !proto.Equal(want, &got) {
		t.Fatalf("LocalMatchAgainstBooleanQuery round trip mismatch: got %v, want %v", &got, want)
	}
}

func TestLocalMatchAgainstBooleanScalarSignature(t *testing.T) {
	if got := int32(ScalarFuncSig_LocalMatchAgainstBoolean); got != 5250 {
		t.Fatalf("LocalMatchAgainstBoolean scalar signature = %d, want 5250", got)
	}
}
