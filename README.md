# feed-a-dev-backend
Automates getting coffee and snacks for tired devs


<img src="https://imgur.com/tsIoR7w.png" alt="feed-a-dev logo" width="200px"/>
# API Endpoints

## Auth Routes
| Method | Endpoint | Description | Auth Required | Request Body |
|--------|----------|-------------|---------------|--------------|
| POST | `/register` | Register new user (developer) | No | `{ "username": string, "password": string, "role": "developer" }` |
| POST | `/register` | Register new user (project manager) | No | `{ "username": string, "password": string, "role": "project_manager" }` |
| POST | `/login` | Authenticate user and get JWT | No | `{ "username": string, "password": string }` |

## Snack Routes
| Method | Endpoint | Description | Auth Required | Request Body |
|--------|----------|-------------|---------------|--------------|
| POST | `/snack` | Create new snack | Yes | `{ "name": string, "category": string, "price": decimal, "image_url": string }` |
| PATCH | `/snack/{id}` | Update snack (owner/admin) | Yes | `{ "name"?: string, "category"?: string, "price"?: decimal, "image_url"?: string }` |
| DELETE | `/snack/{id}` | Delete snack (owner/admin) | Yes | None |
| GET | `/snacks` | List snacks (filtered by role) | Yes | None |

## Relationship Routes
| Method | Endpoint | Description | Auth Required | Request Body |
|--------|----------|-------------|---------------|--------------|
| POST | `/invite-pm` | Developer invites PM | Yes | `{ "project_manager_id": integer }` |
| PATCH | `/respond-to-invite/{id}` | PM accepts/rejects invite | Yes | `{ "status": "accepted" \| "rejected" }` |
| GET | `/my-developers` | PM lists accepted developers | Yes | None |

## Authentication
Use Bearer token in Authorization header:
```
Authorization: Bearer <jwt_token>
```

## Response Types

### User
```typescript
{
  id: number
  username: string
  role: "developer" | "project_manager" | "admin"
  created_at: string
  updated_at: string
}
```

### Snack
```typescript
{
  id: number
  name: string
  category: string
  price: decimal
  image_url: string
  created_at: string
  updated_at: string
  user_id: number
}
```

### Relationship
```typescript
{
  id: number
  developer_id: number
  project_manager_id: number
  status: "pending" | "accepted" | "rejected"
  created_at: string
  updated_at: string
}
```
