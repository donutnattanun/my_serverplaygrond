package main

import (
	"crypto/ed25519"
	"fmt"
	"os"
	"time"

	"go-gateway/jwt_repo"
	"go-gateway/load"
	"go-gateway/redisrepo"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
)

func main() {
	//----- load env------//
	appenv, err := load.LoadAppEnv()
	if err != nil {
		panic(err)
	}
	//-------////
	//-----key build-----//
	pubPEM, err := os.ReadFile(appenv.PubKeyPemPath)
	if err != nil {
		panic(fmt.Sprintf("cannot read public key: %v", err))
	}
	// พิมพ์ PEM เป็นข้อความอ่านง่าย
	fmt.Printf("pubPEM (as text):\n%s\n", string(pubPEM))
	pubKey, err := jwt_repo.ParseEdPubFromPEM(pubPEM)
	if err != nil {
		panic(fmt.Sprintf("cannot pare public key: %v", err))
	}
	fmt.Printf("pubKey (len=%d, hex): %x\n", len(pubKey), pubKey)
	//---- redis init ----//

	redisclient, err := redisrepo.NewRedisClient(appenv.RedisURL, appenv.RedisPass, appenv.RedisDB)
	if err != nil {
		panic(fmt.Sprintf("redis init fail:%v", err))
	}
	//--------//

	// app steat
	app := fiber.New()

	// CORS
	app.Use(cors.New(cors.Config{
		AllowOrigins: "*", // Adjust this to be more restrictive if needed
		AllowMethods: "GET,POST,HEAD,PUT,DELETE,PATCH",
		AllowHeaders: "Origin, Content-Type, Accept",
	}))
	// router
	app.Get("/go/check", check)
	app.Post("/whoami", func(c *fiber.Ctx) error {
		return whoami(c, pubKey, redisclient)
	})

	addr := ":8080"

	app.Listen(addr)
}

func check(c *fiber.Ctx) error {
	return c.JSON(fiber.Map{"status": "ok"})
}

type Token struct {
	Token string `json:"token"`
}

func whoami(c *fiber.Ctx, pubkey ed25519.PublicKey, r *redisrepo.RedisServiceRepo) error {
	var req Token
	if err := c.BodyParser(&req); err != nil {
		return c.JSON(fiber.Map{
			"status": "Bad Request",
			"Error":  err,
		})
	}
	claims, err := jwt_repo.Verify(req.Token, pubkey)
	if err != nil {
		panic(fmt.Sprintf("cannot Verify public key: %v", err))
	}
	now := time.Now().Unix()
	exp := claims.Exp
	if now > exp {
		return c.Status(401).JSON(fiber.Map{
			"error":   "Time Expires",
			"massage": "pls re-login",
		})
	}
	row, err := r.GetSession(claims.Jti)
	if row == nil {
		return c.Status(401).JSON(fiber.Map{"error": "session not found/expired"})
	} else if err != nil {
		return c.Status(500).JSON(fiber.Map{"error": fmt.Sprintf("redis: %v", err)})
	}
	realrtexp := row.RtExp - now
	realatexp := exp - now

	return c.Status(200).JSON(fiber.Map{
		"status":          "ok",
		"id":              row.UserID,
		"row":             row.Role,
		"acountstatus":    row.Status,
		"at_exp_absolute": realatexp,
		"rt_exp_absolute": realrtexp,
		"policy_ver":      row.PolicyVer,
	})
}
