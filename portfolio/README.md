# Portfolio Website

A modern, full-stack portfolio website built with **Rust Actix Web** and **Tailwind CSS** featuring a beautiful dark theme.

## 🚀 Features

- **Modern Dark Theme UI** - Sleek, professional design with glass morphism effects
- **Admin Panel** - Full content management system
- **Blog System** - Create and publish blog posts with Markdown support
- **Project Showcase** - Display your projects with details, links, and technologies
- **Skills Management** - Add and organize your technical skills
- **Contact Form** - Receive messages from visitors
- **Responsive Design** - Works perfectly on all devices
- **SQLite Database** - Lightweight, file-based database

## 🛠️ Tech Stack

- **Backend**: Rust, Actix Web 4
- **Database**: SQLite3
- **Templating**: Tera
- **Frontend**: Tailwind CSS (CDN)
- **Markdown**: pulldown-cmark

## 📦 Installation

### Prerequisites

- Rust (1.70+) - [Install Rust](https://rustup.rs/)

### Setup

1. Clone or navigate to the project directory:
```bash
cd portfolio
```

2. Build the project:
```bash
cargo build --release
```

3. Run the server:
```bash
cargo run
```

4. Open your browser and visit:
- **Portfolio**: http://127.0.0.1:8080
- **Admin Panel**: http://127.0.0.1:8080/admin

## 🔐 Default Admin Credentials

- **Username**: `admin`
- **Password**: `admin123`

> ⚠️ **Important**: Change the default password after first login!

## 📁 Project Structure

```
portfolio/
├── src/
│   ├── main.rs         # Application entry point
│   ├── db.rs           # Database operations
│   ├── models.rs       # Data models
│   ├── routes.rs       # Public routes
│   ├── admin.rs        # Admin routes
│   └── auth.rs         # Authentication
├── templates/
│   ├── base.html       # Base template
│   ├── index.html      # Homepage
│   ├── about.html      # About page
│   ├── projects.html   # Projects listing
│   ├── project_detail.html
│   ├── blogs.html      # Blog listing
│   ├── blog_detail.html
│   ├── contact.html    # Contact form
│   └── admin/          # Admin templates
├── static/
│   ├── css/
│   │   └── style.css   # Custom styles
│   └── js/
│       └── main.js     # JavaScript
├── Cargo.toml
├── .env
└── README.md
```

## 🎨 Customization

### Updating Profile
1. Login to admin panel at `/admin`
2. Go to "Profile" section
3. Update your information and save

### Adding Projects
1. Login to admin panel
2. Go to "Projects" → "Add Project"
3. Fill in project details with Markdown content
4. Check "Featured" to show on homepage

### Adding Blog Posts
1. Login to admin panel
2. Go to "Blogs" → "New Post"
3. Write content using Markdown
4. Check "Publish" to make it visible

### Managing Skills
1. Login to admin panel
2. Go to "Skills"
3. Add skills with name, category, proficiency, and emoji icon

## 📝 Markdown Support

Blog posts and project descriptions support Markdown:

```markdown
## Heading 2
### Heading 3

**Bold text** and *italic text*

- Bullet list
- Item 2

1. Numbered list
2. Item 2

`inline code`

​```rust
// Code block
fn main() {
    println!("Hello, World!");
}
​```

[Link text](https://example.com)

> Blockquote
```

## 🔒 Security Notes

- Change default admin password immediately
- Session cookies are HTTP-only
- Passwords are hashed with bcrypt
- SQLite database is local (no network exposure)

## 📜 License

MIT License - Feel free to use this for your own portfolio!

## 🤝 Contributing

Contributions are welcome! Feel free to submit issues and pull requests.

---

Built with ❤️ using Rust and Actix Web
