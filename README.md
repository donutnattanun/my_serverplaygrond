# 🔐 Rust Auth Core + Go Gateway + Frontend Demo  
A small but production-style authentication system designed for learning, experimenting, and future expansion into a scalable service architecture.

This project is intentionally structured like a real backend platform:  
Rust handles authentication & session control, Go acts as the gateway, PostgreSQL stores user data, Redis caches sessions, and a small frontend is used to test the full flow end-to-end.

---

## 🚀 Quick Start

1. **Clone repository**
   ```sh
   git clone https://github.com/donutnattanun/my_serverplaygrond
   cd my_serverplaygrond

2. **Run everything with Docker Compose**
  ```sh
  docker compose up --build
  ```

*Frontend will be available at:*
    http://localhost:5173/
  
*Rust Auth API:*
      http://localhost:3000/

*Go Gateway:*
      http://localhost:8080/


## 🧩 System Architecture Overview

This project demonstrates a real-world authentication flow:

Rust Auth Server
Issues tokens, validates sessions, controls policy versions, talks to PostgreSQL.

Redis Session Cache
Central store for valid sessions.
Rust = read/write, Go = read-only.

Go Gateway
Decodes tokens, fetches session from Redis, validates access, and forwards requests.

Frontend (Vite)
Simple UI to test login, refresh, whoami, and health checks.

Under the hood → everything communicates through container networking.

## 🖼️ Architecture Diagram

Note:
The worker servers on the right do not exist yet in code.
They represent a future direction — showing that the system can expand horizontally into microservices or background workers without redesigning the core auth flow.

<p align="center"> <img src="./Untitled%20Diagram.drawio.png" width="1600"/> </p>

## 🛠️ Technology Stack

| Layer       | Tech                       |
| ----------- | -------------------------- |
| Auth Server | Rust + Axum + SQLx + Redis |
| Gateway     | Go + Fiber                 |
| Cache       | Redis                      |
| Database    | PostgreSQL                 |
| Frontend    | Vite                       |
| Deployment  | Docker Compose             |

## 🔮 Future Expansion (Planned Architecture)

The system is designed to support:

Plug-in worker microservices

Background jobs / batch processors

Event-driven processing

Horizontal scaling of gateway & workers

gRPC communication between Gateway ↔ Workers

Key rotation & policy-versioned access

Think of this project as a foundation rather than a finished product.

## 🧪 Demo Features

Login / Logout

Copy access token

Refresh session

Service health status

Rust & Go latency monitor

Redis + PostgreSQL integration

Dockerized environment you can run anywhere

# 🔑 Master: Update User

This endpoint is reserved for master / admin users.
It allows the master account to update user information 

(for example: role, status, or basic profile fields).

ℹ️ You must call this endpoint with an access token that belongs to the master user.
Normal users cannot use this API.

ℹ️ *Login and get master token*
```sh
curl -iX POST http://localhost:3000/auth/login \
-H "Content-Type: application/json" \
-d '{"username":"master","password":"1111111111"
}'
```
[info] this project seed master row already

*Endpoint*

```http
POST /auth/master/update
```

*Headers*
```http
Content-Type: application/json
```
*Request Body*
```json
{
  {"token":{"access_token":"//ac_tokenfrom post /auth/login",
            "expires_in":1762946114,
            "refresh_token":"xQsh7tSwt5tcJ9o8SHljH77pFZYLKfUFJ2Vmmj7HJps",
            "token_type":"Bearer"
            },
  "user_id":"61c584f4-60d7-4eb5-adc1-557bbed79de2",
  "user_role":"User","user_status":"Active",
}
```

token - from resopone POST /auth/login

user_id - UUID can get form Go-gate-way at front ui

role - e.g. "User","Master","Admin"

stataus - e.g. "Pending","Active","Suspended","Disabled"

*Example curl*
```sh
curl -X POST http://localhost:3000/auth/master/update \
  -H "Content-Type: application/json" \
  -d '{
    "token":{"access_token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRldi1lZDI1NTE5In0.eyJpc3MiOiJydXN0LmF1dGguc2VydmVyIiwic3ViIjoiZ28uZ2F0ZXdheSIsImp0aSI6InNlc3NfMmJkMjBlNDAtMDU3My00YmM3LWE0NjEtMjkyNGFlNDViY2QwIiwiaWF0IjoxNzYyOTQ1MjE0LCJleHAiOjE3NjI5NDYxMTQsInBvbGljeV92ZXIiOjF9.10QVDG6SFvthWZCwPyBy3D_LWxQw0DIBa68DFNw81UT2afFWUoPHq3hfl6TBkG9uEiG9iD3vJoVkOqcPOU_cBg",
                "expires_in":1762946114,
                "refresh_token":"xQsh7tSwt5tcJ9o8SHljH77pFZYLKfUFJ2Vmmj7HJps",
                "token_type":"Bearer"
                },
    "user_id":"61c584f4-60d7-4eb5-adc1-557bbed79de2",
    "user_role":"User",
    "user_status":"Active"
}'
```

## 👤 About Me

Hi, I'm Donut — a mechanical engineer turned backend/devops developer.
I build software from scratch, love Rust  and enjoy understanding systems all the way down to how electricity becomes logic.
This project is part of my journey toward building real scalable systems.
