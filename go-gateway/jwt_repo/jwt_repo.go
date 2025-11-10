package jwt_repo

import (
	"crypto/ed25519"
	"crypto/x509"
	"encoding/pem"
	"errors"
	"time"

	"github.com/golang-jwt/jwt/v5"
)

type MyClaims struct {
	Iss       string `json:"iss"`
	Jti       string `json:"jti"`
	Sub       string `json:"sub"`
	Iat       int64  `json:"iat"`
	Exp       int64  `json:"exp"`
	PolicyVer int    `json:"policy_ver"`
	jwt.RegisteredClaims
}

func ParseEdPubFromPEM(pemBytes []byte) (ed25519.PublicKey, error) {
	block, _ := pem.Decode(pemBytes)
	if block == nil {
		return nil, errors.New("PEM decode failed: not a PEM or empty input")
	}
	pubAny, err := x509.ParsePKIXPublicKey(block.Bytes)
	if err != nil {
		return nil, err
	}
	pub, ok := pubAny.(ed25519.PublicKey)
	if !ok {
		return nil, errors.New("not an Ed25519 public key")
	}
	return pub, nil
	// โหลด DER → ed25519.PublicKey ตามฟอร์แมตคีย์คุณ
	// (ถ้าเป็น raw SubjectPublicKeyInfo ก็ใช้ block.Bytes ได้เลย)
}

func Verify(tokenStr string, pubKey ed25519.PublicKey) (*MyClaims, error) {
	parser := jwt.NewParser(
		jwt.WithValidMethods([]string{jwt.SigningMethodEdDSA.Alg()}),
		jwt.WithLeeway(5*time.Second), // เผื่อ clock skew
	)
	var claims MyClaims
	_, err := parser.ParseWithClaims(tokenStr, &claims, func(t *jwt.Token) (interface{}, error) {
		return pubKey, nil
	})
	if err != nil {
		return nil, err
	}
	return &claims, nil
}
