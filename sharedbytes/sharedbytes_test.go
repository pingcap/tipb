package sharedbytes

import (
	"testing"
	"bytes"
)

func TestShardBytes(t *testing.T) {
	var sb SharedBytes
	sb = []byte("abc")
	x, err := sb.Marshal()
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(x, []byte("abc")) {
		t.Fatal("marshal failed")
	}
	x = make([]byte, 3)
	sb.MarshalTo(x)
	if !bytes.Equal(x, []byte("abc")) {
		t.Fatal("marshal failed")
	}
	sb = nil
	if err := sb.Unmarshal([]byte("abc")); err != nil {
		t.Fatal(err)
	}
}
