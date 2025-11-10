package load

import (
	"fmt"
	"os"
	"strconv"

	"github.com/joho/godotenv"
)

type AppEnv struct {
	AppPort       string
	PubKeyPemPath string
	RedisURL      string
	RedisPass     string
	RedisDB       int
}

func LoadAppEnv() (AppEnv, error) {
	if err := godotenv.Load(); err != nil {
		return AppEnv{}, fmt.Errorf("godotenv fail :%v", err)
	}
	pubkeypempath := os.Getenv("JWT_PUBLIC_KEY_PATH")
	app_port := os.Getenv("APP_PORT")
	redisurl := os.Getenv("REDIS_URL")
	redispass := os.Getenv("REDIS_PASS")
	redisdb_str := os.Getenv("REDIS_DB")
	redisDB := 0
	if redisdb_str != "" {
		if v, err := strconv.Atoi(redisdb_str); err == nil {
			redisDB = v
		} else {
			return AppEnv{}, fmt.Errorf("invalid REDIS_DB: %v", err)
		}
	}
	if pubkeypempath == "" {
		return AppEnv{}, fmt.Errorf("missing JWT_PUBLIC_KEY_PATH")
	}
	if app_port == "" {
		app_port = "8080" // default
	}
	return AppEnv{
		app_port,
		pubkeypempath,
		redisurl,
		redispass,
		redisDB,
	}, nil
}
