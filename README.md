
# Blog API in Rust (Rocket + Diesel + SQLite)

This is a simple blog backend written in **Rust**, using:
- [Rocket](https://rocket.rs/) for the web framework
- [Diesel](https://diesel.rs/) as the ORM
- SQLite as the database

---

## 🚀 Features Implemented

### ✅1: User and Post Management with Pagination
- **POST /api/create_user**: Create a new user.
- **POST /api/create_post**: Create a new post with tags.
- **GET /api/posts?page=1&limit=10&search=optional_term**: List paginated posts with author info.

### ✅2: Tag Support
- `posts_tags` table links tags to posts (many-to-many).
- On post creation, tags are inserted into `posts_tags`.
- Each post in the response includes `tags: Vec<String>`.

### ✅3: Post Listing with Author Info
- Author details (username, first name, last name) are returned as `created_by` field.
- Uses a `LEFT JOIN` on `users` table.

---

## 📦 Tech Stack

| Component | Technology |
|----------|------------|
| Backend  | Rocket (Rust) |
| ORM      | Diesel |
| DB       | SQLite |
| Language | Rust 2021 |

---

## 📁 Project Structure

```
.
├── main.rs               # Rocket app launcher
├── db.rs                 # Database pool setup
├── models.rs             # Diesel models & request/response structs
├── routes/
│   ├── user.rs           # User-related routes
│   └── post.rs           # Post-related routes
├── schema.rs             # Diesel-generated schema (via CLI)
├── Cargo.toml
└── .env                  # DATABASE_URL (SQLite)
```

---

## 🛠️ Setup Instructions

### 1. Clone the Repository

```bash
git clone <your-repo-url>
cd blog-api-rust
```

### 2. Install Diesel CLI

```bash
cargo install diesel_cli --no-default-features --features sqlite
```

### 3. Setup Environment

Create a `.env` file:

```
DATABASE_URL=blog.db
```

Run migrations:

```bash
diesel setup
diesel migration run
```

### 4. Run the Server

```bash
cargo run
```

The server runs at `http://localhost:8000`.

---

## 📮 API Endpoints

### Create User
**POST** `/api/create_user`
```json
{
  "username": "devilalk33",
  "first_name": "Devilal",
  "last_name": "Kumawat"
}
```

### Create Post with Tags
**POST** `/api/create_post`
```json
{
  "created_by": 1,
  "title": "My First Post",
  "body": "This is a post body.",
  "tags": ["rust", "diesel", "blog"]
}
```

### Get Paginated Posts
**GET** `/api/posts?page=1&limit=5&search=rust`

**Response:**
```json
{
  "records": [
    {
      "id": 1,
      "title": "My First Post",
      "body": "This is a post body.",
      "tags": ["rust", "diesel", "blog"],
      "created_by": {
        "user_id": 1,
        "username": "john_doe",
        "first_name": "John",
        "last_name": "Doe"
      }
    }
  ],
  "meta": {
    "current_page": 1,
    "per_page": 5,
    "from": 1,
    "to": 1,
    "total_pages": 0,
    "total_docs": 0
  }
}
```

---

## 🧪 Testing Tips

- Use Postman or curl to test API endpoints.
- Run `cargo check` to verify build.
- Add unit tests to `routes/` modules if needed.

---

## 📌 To Do (Future Scope)
- Update/Delete for posts and users
- JWT authentication
- Tag filtering via query param
- Total count in pagination
- Rate limiting or API key support
- Dockerize app

---

## 🧑‍💻 Author
Built with ❤️ in Rust by [Devilal Kumawat]
