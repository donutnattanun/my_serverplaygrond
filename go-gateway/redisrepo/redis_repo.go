package redisrepo

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/redis/go-redis/v9"
)

type SessionRow struct {
	SessionID string `json:"session_id"`
	UserID    string `json:"user_id"`
	Role      string `json:"role"`
	Status    string `json:"status"`
	CreatedAt int64  `json:"created_at"`
	ExpiresAt int64  `json:"expires_at"`
	RtHash    string `json:"rt_hash"`
	RtExp     int64  `json:"rt_exp"`
	Device    string `json:"device"`
	IP        string `json:"ip"`
	PolicyVer int    `json:"policy_ver"`
}

type RedisServiceRepo struct {
	Client *redis.Client
	Ctx    context.Context
}
type RedisService interface {
	GetSession(sessid string) (*SessionRow, error)
}

func NewRedisClient(addr string, pass string, db int) (*RedisServiceRepo, error) {
	ctx := context.Background()

	var client *redis.Client
	if strings.HasPrefix(addr, "redis://") || strings.HasPrefix(addr, "rediss://") {
		// กรณีส่งมาเป็น URL: ใช้ ParseURL แล้วจบ
		opt, err := redis.ParseURL(addr)
		if err != nil {
			return nil, fmt.Errorf("parse redis url: %v", err)
		}
		client = redis.NewClient(opt)
	} else {
		// กรณีส่งมาเป็น host:port ธรรมดา
		client = redis.NewClient(&redis.Options{
			Addr:     addr,
			Password: pass,
			DB:       db,
		})
	}

	if err := client.Ping(ctx).Err(); err != nil {
		return nil, fmt.Errorf("redis ping failed: %v", err)
	}

	fmt.Println("Redis connected:", addr)
	return &RedisServiceRepo{Client: client, Ctx: ctx}, nil
}

func (r *RedisServiceRepo) GetSession(sessid string) (*SessionRow, error) {
	raw, err := r.Client.Get(r.Ctx, sessid).Result()
	if err != nil {
		return nil, fmt.Errorf("redis get : %v", err)
	}
	var sess SessionRow
	if err := json.Unmarshal([]byte(raw), &sess); err != nil {
		return nil, fmt.Errorf("json decode: %v", err)
	}
	return &sess, nil
}
