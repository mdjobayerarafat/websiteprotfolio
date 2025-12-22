use rusqlite::{Connection, Result};
use bcrypt::{hash, DEFAULT_COST};
use crate::models::{EmailSettings, EmailSettingsForm, SiteContentItem};

pub fn init_db() -> Result<Connection> {
    let db_path = std::env::var("DATABASE_URL").unwrap_or_else(|_| "portfolio.db".to_string());
    log::info!("Using database at: {}", db_path);
    let conn = Connection::open(&db_path)?;
    
    // Create tables
    conn.execute_batch(
        "
        -- Profile table
        CREATE TABLE IF NOT EXISTS profile (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            title TEXT NOT NULL,
            bio TEXT NOT NULL,
            email TEXT NOT NULL,
            phone TEXT,
            location TEXT,
            github_url TEXT,
            linkedin_url TEXT,
            twitter_url TEXT,
            resume_url TEXT,
            avatar_url TEXT
        );

        -- Skills table
        CREATE TABLE IF NOT EXISTS skills (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            category TEXT NOT NULL,
            proficiency INTEGER DEFAULT 80,
            icon TEXT,
            icon_url TEXT
        );

        -- Projects table
        CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            slug TEXT NOT NULL UNIQUE,
            description TEXT NOT NULL,
            content TEXT,
            image_url TEXT,
            demo_url TEXT,
            github_url TEXT,
            technologies TEXT,
            featured INTEGER DEFAULT 0,
            created_at TEXT NOT NULL
        );

        -- Blogs table
        CREATE TABLE IF NOT EXISTS blogs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            slug TEXT NOT NULL UNIQUE,
            excerpt TEXT NOT NULL,
            content TEXT NOT NULL,
            image_url TEXT,
            tags TEXT,
            published INTEGER DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        -- Experience table
        CREATE TABLE IF NOT EXISTS experience (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            company TEXT NOT NULL,
            position TEXT NOT NULL,
            description TEXT,
            start_date TEXT NOT NULL,
            end_date TEXT,
            current INTEGER DEFAULT 0
        );

        -- Education table
        CREATE TABLE IF NOT EXISTS education (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            institution TEXT NOT NULL,
            degree TEXT NOT NULL,
            field TEXT NOT NULL,
            start_date TEXT NOT NULL,
            end_date TEXT,
            description TEXT
        );

        -- Messages table (contact form)
        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            subject TEXT NOT NULL,
            message TEXT NOT NULL,
            read INTEGER DEFAULT 0,
            created_at TEXT NOT NULL
        );

        -- Admin table
        CREATE TABLE IF NOT EXISTS admin (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL
        );

        -- Images table for storing uploaded images
        CREATE TABLE IF NOT EXISTS images (
            id TEXT PRIMARY KEY,
            filename TEXT NOT NULL,
            content_type TEXT NOT NULL,
            data BLOB NOT NULL,
            created_at TEXT NOT NULL
        );

        -- Services table
        CREATE TABLE IF NOT EXISTS services (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            image_url TEXT,
            icon TEXT,
            order_index INTEGER DEFAULT 0
        );

        -- Email Settings table
        CREATE TABLE IF NOT EXISTS email_settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            smtp_server TEXT NOT NULL DEFAULT '',
            smtp_port INTEGER NOT NULL DEFAULT 587,
            smtp_username TEXT NOT NULL DEFAULT '',
            smtp_password TEXT NOT NULL DEFAULT '',
            notification_email TEXT NOT NULL DEFAULT '',
            enabled INTEGER NOT NULL DEFAULT 0
        );

        -- Site Content table for dynamic text
        CREATE TABLE IF NOT EXISTS site_content (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL DEFAULT '',
            section TEXT NOT NULL DEFAULT 'general',
            description TEXT DEFAULT ''
        );
        "
    )?;

    // Insert default site content
    init_default_site_content(&conn)?;

    // Insert default profile if not exists
    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM profile",
        [],
        |row| row.get(0)
    )?;

    if count == 0 {
        conn.execute(
            "INSERT INTO profile (id, name, title, bio, email, phone, location, github_url, linkedin_url, twitter_url, resume_url, avatar_url)
             VALUES (1, 'Md Jobayer Arafat', 'Full Stack Developer & AI Enthusiast', 
             'Passionate software developer with expertise in web development, machine learning, and building innovative solutions. I love creating efficient, scalable applications and exploring cutting-edge technologies.',
             'jobayerarafat@example.com', '+880 1234567890', 'Bangladesh',
             'https://github.com/mdjobayerarafat', 'https://linkedin.com/in/mdjobayerarafat', 
             'https://twitter.com/mdjobayerarafat', '/static/resume.pdf', '/static/avatar.jpg')",
            [],
        )?;
    }

    // Insert default admin if not exists
    let admin_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM admin",
        [],
        |row| row.get(0)
    )?;

    if admin_count == 0 {
        let password_hash = hash("admin123", DEFAULT_COST).expect("Failed to hash password");
        conn.execute(
            "INSERT INTO admin (id, username, password_hash) VALUES (1, 'admin', ?1)",
            [&password_hash],
        )?;
        log::info!("Default admin created - username: admin, password: admin123");
    }

    // Insert sample skills if empty
    let skills_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM skills",
        [],
        |row| row.get(0)
    )?;

    if skills_count == 0 {
        let skills = vec![
            ("Rust", "Backend", 85, "🦀"),
            ("Python", "Backend", 90, "🐍"),
            ("JavaScript", "Frontend", 88, "📜"),
            ("TypeScript", "Frontend", 85, "💙"),
            ("React", "Frontend", 82, "⚛️"),
            ("Node.js", "Backend", 85, "💚"),
            ("PostgreSQL", "Database", 80, "🐘"),
            ("SQLite", "Database", 85, "📦"),
            ("Docker", "DevOps", 78, "🐳"),
            ("Git", "Tools", 90, "📚"),
            ("Linux", "Tools", 85, "🐧"),
            ("Machine Learning", "AI/ML", 80, "🤖"),
        ];

        for (name, category, proficiency, icon) in skills {
            conn.execute(
                "INSERT INTO skills (name, category, proficiency, icon) VALUES (?1, ?2, ?3, ?4)",
                [name, category, &proficiency.to_string(), icon],
            )?;
        }
    }

    // Insert sample projects if empty
    let projects_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM projects",
        [],
        |row| row.get(0)
    )?;

    if projects_count == 0 {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        conn.execute(
            "INSERT INTO projects (title, slug, description, content, image_url, demo_url, github_url, technologies, featured, created_at)
             VALUES ('Portfolio Website', 'portfolio-website', 
             'A modern portfolio website built with Rust Actix Web and Tailwind CSS',
             '## Overview\n\nThis portfolio website showcases my projects, skills, and blog posts. Built with modern technologies for optimal performance.\n\n## Features\n\n- Responsive dark theme design\n- Admin panel for content management\n- Blog with markdown support\n- Project showcase\n- Contact form',
             '/static/images/portfolio.jpg', 'https://example.com', 'https://github.com/mdjobayerarafat/portfolio',
             'Rust, Actix Web, SQLite, Tailwind CSS, Tera', 1, ?1)",
            [&now],
        )?;

        conn.execute(
            "INSERT INTO projects (title, slug, description, content, image_url, demo_url, github_url, technologies, featured, created_at)
             VALUES ('AI Chat Application', 'ai-chat-app', 
             'An intelligent chat application powered by machine learning',
             '## Overview\n\nA real-time chat application with AI-powered responses and natural language processing capabilities.\n\n## Features\n\n- Real-time messaging\n- AI-powered responses\n- Natural language understanding\n- Multi-language support',
             '/static/images/ai-chat.jpg', 'https://example.com/chat', 'https://github.com/mdjobayerarafat/ai-chat',
             'Python, FastAPI, React, TensorFlow, WebSocket', 1, ?1)",
            [&now],
        )?;
    }

    // Insert sample blog posts if empty
    let blogs_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM blogs",
        [],
        |row| row.get(0)
    )?;

    if blogs_count == 0 {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        conn.execute(
            "INSERT INTO blogs (title, slug, excerpt, content, image_url, tags, published, created_at, updated_at)
             VALUES ('Getting Started with Rust Web Development', 'getting-started-rust-web',
             'Learn how to build fast and reliable web applications using Rust and Actix Web framework.',
             '## Introduction\n\nRust is a systems programming language that offers memory safety, concurrency, and performance. In this blog post, we''ll explore how to build web applications using Rust.\n\n## Why Rust?\n\n- Memory safety without garbage collection\n- Zero-cost abstractions\n- Fearless concurrency\n- Great performance\n\n## Setting Up\n\nFirst, install Rust using rustup:\n\n```bash\ncurl --proto ''=https'' --tlsv1.2 -sSf https://sh.rustup.rs | sh\n```\n\n## Creating Your First Actix Web App\n\nCreate a new project and add dependencies...',
             '/static/images/rust-blog.jpg', 'Rust, Web Development, Actix', 1, ?1, ?2)",
            [&now, &now],
        )?;

        conn.execute(
            "INSERT INTO blogs (title, slug, excerpt, content, image_url, tags, published, created_at, updated_at)
             VALUES ('Building Modern UIs with Tailwind CSS', 'modern-ui-tailwind-css',
             'Discover how Tailwind CSS can speed up your frontend development with utility-first approach.',
             '## What is Tailwind CSS?\n\nTailwind CSS is a utility-first CSS framework that allows you to build custom designs without leaving your HTML.\n\n## Benefits\n\n- No need to write custom CSS\n- Consistent design system\n- Responsive design made easy\n- Dark mode support\n\n## Getting Started\n\nInstall Tailwind via npm:\n\n```bash\nnpm install -D tailwindcss\nnpx tailwindcss init\n```\n\n## Building Components\n\nCreate beautiful components with utility classes...',
             '/static/images/tailwind-blog.jpg', 'CSS, Tailwind, Frontend', 1, ?1, ?2)",
            [&now, &now],
        )?;
    }

    // Insert sample experience
    let exp_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM experience",
        [],
        |row| row.get(0)
    )?;

    if exp_count == 0 {
        conn.execute(
            "INSERT INTO experience (company, position, description, start_date, end_date, current)
             VALUES ('Tech Company', 'Senior Software Developer', 
             'Building scalable web applications and leading development teams.',
             '2022-01', NULL, 1)",
            [],
        )?;

        conn.execute(
            "INSERT INTO experience (company, position, description, start_date, end_date, current)
             VALUES ('Startup Inc', 'Full Stack Developer', 
             'Developed full-stack applications using modern technologies.',
             '2020-06', '2022-01', 0)",
            [],
        )?;
    }

    // Insert sample education
    let edu_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM education",
        [],
        |row| row.get(0)
    )?;

    if edu_count == 0 {
        conn.execute(
            "INSERT INTO education (institution, degree, field, start_date, end_date, description)
             VALUES ('University of Technology', 'Bachelor of Science', 
             'Computer Science and Engineering',
             '2016', '2020', 'Focused on software engineering and machine learning.')",
            [],
        )?;
    }

    // Insert sample services if empty
    let services_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM services",
        [],
        |row| row.get(0)
    )?;

    if services_count == 0 {
        conn.execute(
            "INSERT INTO services (name, description, image_url, icon, order_index)
             VALUES ('UI/UX Design', 'Creating beautiful and intuitive user interfaces with modern design principles.', '', '🎨', 1)",
            [],
        )?;
        
        conn.execute(
            "INSERT INTO services (name, description, image_url, icon, order_index)
             VALUES ('Web Design', 'Building responsive, modern websites that look great on all devices.', '', '🌐', 2)",
            [],
        )?;
        
        conn.execute(
            "INSERT INTO services (name, description, image_url, icon, order_index)
             VALUES ('Landing Page Design', 'High-converting landing pages designed to maximize your business goals.', '', '📄', 3)",
            [],
        )?;
    }

    Ok(conn)
}

use crate::models::*;

pub fn get_profile(conn: &Connection) -> Result<Profile> {
    conn.query_row(
        "SELECT id, name, title, bio, email, phone, location, github_url, linkedin_url, twitter_url, resume_url, avatar_url FROM profile WHERE id = 1",
        [],
        |row| {
            Ok(Profile {
                id: row.get(0)?,
                name: row.get(1)?,
                title: row.get(2)?,
                bio: row.get(3)?,
                email: row.get(4)?,
                phone: row.get(5)?,
                location: row.get(6)?,
                github_url: row.get(7)?,
                linkedin_url: row.get(8)?,
                twitter_url: row.get(9)?,
                resume_url: row.get(10)?,
                avatar_url: row.get(11)?,
            })
        },
    )
}

pub fn update_profile(conn: &Connection, profile: &ProfileForm) -> Result<()> {
    conn.execute(
        "UPDATE profile SET name = ?1, title = ?2, bio = ?3, email = ?4, phone = ?5, 
         location = ?6, github_url = ?7, linkedin_url = ?8, twitter_url = ?9, 
         resume_url = ?10, avatar_url = ?11 WHERE id = 1",
        [
            &profile.name, &profile.title, &profile.bio, &profile.email, &profile.phone,
            &profile.location, &profile.github_url, &profile.linkedin_url, &profile.twitter_url,
            &profile.resume_url, &profile.avatar_url,
        ],
    )?;
    Ok(())
}

pub fn get_skills(conn: &Connection) -> Result<Vec<Skill>> {
    let mut stmt = conn.prepare("SELECT id, name, category, proficiency, icon, COALESCE(icon_url, '') FROM skills ORDER BY category, name")?;
    let skills = stmt.query_map([], |row| {
        Ok(Skill {
            id: row.get(0)?,
            name: row.get(1)?,
            category: row.get(2)?,
            proficiency: row.get(3)?,
            icon: row.get(4)?,
            icon_url: row.get(5)?,
        })
    })?;
    skills.collect()
}

pub fn add_skill(conn: &Connection, skill: &SkillForm) -> Result<()> {
    conn.execute(
        "INSERT INTO skills (name, category, proficiency, icon, icon_url) VALUES (?1, ?2, ?3, ?4, ?5)",
        [&skill.name, &skill.category, &skill.proficiency.to_string(), &skill.icon, &skill.icon_url],
    )?;
    Ok(())
}

pub fn delete_skill(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM skills WHERE id = ?1", [id])?;
    Ok(())
}

pub fn get_projects(conn: &Connection) -> Result<Vec<Project>> {
    let mut stmt = conn.prepare("SELECT id, title, slug, description, content, image_url, demo_url, github_url, technologies, featured, created_at FROM projects ORDER BY created_at DESC")?;
    let projects = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            title: row.get(1)?,
            slug: row.get(2)?,
            description: row.get(3)?,
            content: row.get(4)?,
            image_url: row.get(5)?,
            demo_url: row.get(6)?,
            github_url: row.get(7)?,
            technologies: row.get(8)?,
            featured: row.get(9)?,
            created_at: row.get(10)?,
        })
    })?;
    projects.collect()
}

pub fn get_featured_projects(conn: &Connection) -> Result<Vec<Project>> {
    let mut stmt = conn.prepare("SELECT id, title, slug, description, content, image_url, demo_url, github_url, technologies, featured, created_at FROM projects WHERE featured = 1 ORDER BY created_at DESC LIMIT 4")?;
    let projects = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            title: row.get(1)?,
            slug: row.get(2)?,
            description: row.get(3)?,
            content: row.get(4)?,
            image_url: row.get(5)?,
            demo_url: row.get(6)?,
            github_url: row.get(7)?,
            technologies: row.get(8)?,
            featured: row.get(9)?,
            created_at: row.get(10)?,
        })
    })?;
    projects.collect()
}

pub fn get_project_by_slug(conn: &Connection, slug: &str) -> Result<Project> {
    conn.query_row(
        "SELECT id, title, slug, description, content, image_url, demo_url, github_url, technologies, featured, created_at FROM projects WHERE slug = ?1",
        [slug],
        |row| {
            Ok(Project {
                id: row.get(0)?,
                title: row.get(1)?,
                slug: row.get(2)?,
                description: row.get(3)?,
                content: row.get(4)?,
                image_url: row.get(5)?,
                demo_url: row.get(6)?,
                github_url: row.get(7)?,
                technologies: row.get(8)?,
                featured: row.get(9)?,
                created_at: row.get(10)?,
            })
        },
    )
}

pub fn get_project_by_id(conn: &Connection, id: i32) -> Result<Project> {
    conn.query_row(
        "SELECT id, title, slug, description, content, image_url, demo_url, github_url, technologies, featured, created_at FROM projects WHERE id = ?1",
        [id],
        |row| {
            Ok(Project {
                id: row.get(0)?,
                title: row.get(1)?,
                slug: row.get(2)?,
                description: row.get(3)?,
                content: row.get(4)?,
                image_url: row.get(5)?,
                demo_url: row.get(6)?,
                github_url: row.get(7)?,
                technologies: row.get(8)?,
                featured: row.get(9)?,
                created_at: row.get(10)?,
            })
        },
    )
}

pub fn add_project(conn: &Connection, project: &ProjectForm) -> Result<()> {
    let slug = slug::slugify(&project.title);
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let featured: i32 = if project.featured.is_some() { 1 } else { 0 };
    
    conn.execute(
        "INSERT INTO projects (title, slug, description, content, image_url, demo_url, github_url, technologies, featured, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        rusqlite::params![
            &project.title, &slug, &project.description, &project.content,
            &project.image_url, &project.demo_url, &project.github_url,
            &project.technologies, featured, &now,
        ],
    )?;
    Ok(())
}

pub fn update_project(conn: &Connection, id: i32, project: &ProjectForm) -> Result<()> {
    let slug = slug::slugify(&project.title);
    let featured: i32 = if project.featured.is_some() { 1 } else { 0 };
    
    conn.execute(
        "UPDATE projects SET title = ?1, slug = ?2, description = ?3, content = ?4, 
         image_url = ?5, demo_url = ?6, github_url = ?7, technologies = ?8, featured = ?9 
         WHERE id = ?10",
        rusqlite::params![
            &project.title, &slug, &project.description, &project.content,
            &project.image_url, &project.demo_url, &project.github_url,
            &project.technologies, featured, id,
        ],
    )?;
    Ok(())
}

pub fn delete_project(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM projects WHERE id = ?1", [id])?;
    Ok(())
}

pub fn get_blogs(conn: &Connection) -> Result<Vec<Blog>> {
    let mut stmt = conn.prepare("SELECT id, title, slug, excerpt, content, image_url, tags, published, created_at, updated_at FROM blogs ORDER BY created_at DESC")?;
    let blogs = stmt.query_map([], |row| {
        Ok(Blog {
            id: row.get(0)?,
            title: row.get(1)?,
            slug: row.get(2)?,
            excerpt: row.get(3)?,
            content: row.get(4)?,
            image_url: row.get(5)?,
            tags: row.get(6)?,
            published: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;
    blogs.collect()
}

pub fn get_published_blogs(conn: &Connection) -> Result<Vec<Blog>> {
    let mut stmt = conn.prepare("SELECT id, title, slug, excerpt, content, image_url, tags, published, created_at, updated_at FROM blogs WHERE published = 1 ORDER BY created_at DESC")?;
    let blogs = stmt.query_map([], |row| {
        Ok(Blog {
            id: row.get(0)?,
            title: row.get(1)?,
            slug: row.get(2)?,
            excerpt: row.get(3)?,
            content: row.get(4)?,
            image_url: row.get(5)?,
            tags: row.get(6)?,
            published: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;
    blogs.collect()
}

pub fn get_recent_blogs(conn: &Connection, limit: i32) -> Result<Vec<Blog>> {
    let mut stmt = conn.prepare("SELECT id, title, slug, excerpt, content, image_url, tags, published, created_at, updated_at FROM blogs WHERE published = 1 ORDER BY created_at DESC LIMIT ?1")?;
    let blogs = stmt.query_map([limit], |row| {
        Ok(Blog {
            id: row.get(0)?,
            title: row.get(1)?,
            slug: row.get(2)?,
            excerpt: row.get(3)?,
            content: row.get(4)?,
            image_url: row.get(5)?,
            tags: row.get(6)?,
            published: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;
    blogs.collect()
}

pub fn get_blog_by_slug(conn: &Connection, slug: &str) -> Result<Blog> {
    conn.query_row(
        "SELECT id, title, slug, excerpt, content, image_url, tags, published, created_at, updated_at FROM blogs WHERE slug = ?1",
        [slug],
        |row| {
            Ok(Blog {
                id: row.get(0)?,
                title: row.get(1)?,
                slug: row.get(2)?,
                excerpt: row.get(3)?,
                content: row.get(4)?,
                image_url: row.get(5)?,
                tags: row.get(6)?,
                published: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        },
    )
}

pub fn get_blog_by_id(conn: &Connection, id: i32) -> Result<Blog> {
    conn.query_row(
        "SELECT id, title, slug, excerpt, content, image_url, tags, published, created_at, updated_at FROM blogs WHERE id = ?1",
        [id],
        |row| {
            Ok(Blog {
                id: row.get(0)?,
                title: row.get(1)?,
                slug: row.get(2)?,
                excerpt: row.get(3)?,
                content: row.get(4)?,
                image_url: row.get(5)?,
                tags: row.get(6)?,
                published: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        },
    )
}

pub fn add_blog(conn: &Connection, blog: &BlogForm) -> Result<()> {
    let slug = slug::slugify(&blog.title);
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let published: i32 = if blog.published.is_some() { 1 } else { 0 };
    
    conn.execute(
        "INSERT INTO blogs (title, slug, excerpt, content, image_url, tags, published, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![
            &blog.title, &slug, &blog.excerpt, &blog.content,
            &blog.image_url, &blog.tags, published, &now, &now,
        ],
    )?;
    Ok(())
}

pub fn update_blog(conn: &Connection, id: i32, blog: &BlogForm) -> Result<()> {
    let slug = slug::slugify(&blog.title);
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let published: i32 = if blog.published.is_some() { 1 } else { 0 };
    
    conn.execute(
        "UPDATE blogs SET title = ?1, slug = ?2, excerpt = ?3, content = ?4, 
         image_url = ?5, tags = ?6, published = ?7, updated_at = ?8 WHERE id = ?9",
        rusqlite::params![
            &blog.title, &slug, &blog.excerpt, &blog.content,
            &blog.image_url, &blog.tags, published, &now, id,
        ],
    )?;
    Ok(())
}

pub fn delete_blog(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM blogs WHERE id = ?1", [id])?;
    Ok(())
}

pub fn get_experience(conn: &Connection) -> Result<Vec<Experience>> {
    let mut stmt = conn.prepare("SELECT id, company, position, description, start_date, end_date, current FROM experience ORDER BY start_date DESC")?;
    let experience = stmt.query_map([], |row| {
        Ok(Experience {
            id: row.get(0)?,
            company: row.get(1)?,
            position: row.get(2)?,
            description: row.get(3)?,
            start_date: row.get(4)?,
            end_date: row.get::<_, Option<String>>(5)?.unwrap_or_default(),
            current: row.get(6)?,
        })
    })?;
    experience.collect()
}

pub fn get_education(conn: &Connection) -> Result<Vec<Education>> {
    let mut stmt = conn.prepare("SELECT id, institution, degree, field, start_date, end_date, description FROM education ORDER BY start_date DESC")?;
    let education = stmt.query_map([], |row| {
        Ok(Education {
            id: row.get(0)?,
            institution: row.get(1)?,
            degree: row.get(2)?,
            field: row.get(3)?,
            start_date: row.get(4)?,
            end_date: row.get::<_, Option<String>>(5)?.unwrap_or_default(),
            description: row.get::<_, Option<String>>(6)?.unwrap_or_default(),
        })
    })?;
    education.collect()
}

pub fn get_education_by_id(conn: &Connection, id: i32) -> Result<Education> {
    conn.query_row(
        "SELECT id, institution, degree, field, start_date, end_date, description FROM education WHERE id = ?1",
        rusqlite::params![id],
        |row| {
            Ok(Education {
                id: row.get(0)?,
                institution: row.get(1)?,
                degree: row.get(2)?,
                field: row.get(3)?,
                start_date: row.get(4)?,
                end_date: row.get::<_, Option<String>>(5)?.unwrap_or_default(),
                description: row.get::<_, Option<String>>(6)?.unwrap_or_default(),
            })
        },
    )
}

pub fn add_education(conn: &Connection, edu: &Education) -> Result<()> {
    conn.execute(
        "INSERT INTO education (institution, degree, field, start_date, end_date, description) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![edu.institution, edu.degree, edu.field, edu.start_date, edu.end_date, edu.description],
    )?;
    Ok(())
}

pub fn update_education(conn: &Connection, edu: &Education) -> Result<()> {
    conn.execute(
        "UPDATE education SET institution = ?1, degree = ?2, field = ?3, start_date = ?4, end_date = ?5, description = ?6 WHERE id = ?7",
        rusqlite::params![edu.institution, edu.degree, edu.field, edu.start_date, edu.end_date, edu.description, edu.id],
    )?;
    Ok(())
}

pub fn delete_education(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM education WHERE id = ?1", rusqlite::params![id])?;
    Ok(())
}

pub fn add_message(conn: &Connection, message: &ContactForm) -> Result<()> {
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO messages (name, email, subject, message, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        [&message.name, &message.email, &message.subject, &message.message, &now],
    )?;
    Ok(())
}

pub fn get_messages(conn: &Connection) -> Result<Vec<Message>> {
    let mut stmt = conn.prepare("SELECT id, name, email, subject, message, read, created_at FROM messages ORDER BY created_at DESC")?;
    let messages = stmt.query_map([], |row| {
        Ok(Message {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            subject: row.get(3)?,
            message: row.get(4)?,
            read: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    messages.collect()
}

pub fn get_unread_message_count(conn: &Connection) -> Result<i32> {
    conn.query_row(
        "SELECT COUNT(*) FROM messages WHERE read = 0",
        [],
        |row| row.get(0)
    )
}

pub fn delete_message(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM messages WHERE id = ?1", [id])?;
    Ok(())
}

pub fn get_admin(conn: &Connection, username: &str) -> Result<Admin> {
    conn.query_row(
        "SELECT id, username, password_hash FROM admin WHERE username = ?1",
        [username],
        |row| {
            Ok(Admin {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: row.get(2)?,
            })
        },
    )
}

// Image functions
pub fn save_image(conn: &Connection, id: &str, filename: &str, content_type: &str, data: &[u8]) -> Result<()> {
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO images (id, filename, content_type, data, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![id, filename, content_type, data, now],
    )?;
    Ok(())
}

pub fn get_image(conn: &Connection, id: &str) -> Result<(String, Vec<u8>)> {
    conn.query_row(
        "SELECT content_type, data FROM images WHERE id = ?1",
        [id],
        |row| {
            let content_type: String = row.get(0)?;
            let data: Vec<u8> = row.get(1)?;
            Ok((content_type, data))
        },
    )
}

pub fn get_image_with_filename(conn: &Connection, id: &str) -> Result<(String, String, Vec<u8>)> {
    conn.query_row(
        "SELECT filename, content_type, data FROM images WHERE id = ?1",
        [id],
        |row| {
            let filename: String = row.get(0)?;
            let content_type: String = row.get(1)?;
            let data: Vec<u8> = row.get(2)?;
            Ok((filename, content_type, data))
        },
    )
}

pub fn delete_image(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM images WHERE id = ?1", [id])?;
    Ok(())
}

// Service functions
pub fn get_services(conn: &Connection) -> Result<Vec<Service>> {
    let mut stmt = conn.prepare("SELECT id, name, description, image_url, icon, order_index FROM services ORDER BY order_index")?;
    let services = stmt.query_map([], |row| {
        Ok(Service {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            image_url: row.get(3)?,
            icon: row.get(4)?,
            order_index: row.get(5)?,
        })
    })?;
    services.collect()
}

pub fn get_service(conn: &Connection, id: i32) -> Result<Service> {
    conn.query_row(
        "SELECT id, name, description, image_url, icon, order_index FROM services WHERE id = ?1",
        [id],
        |row| {
            Ok(Service {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                image_url: row.get(3)?,
                icon: row.get(4)?,
                order_index: row.get(5)?,
            })
        },
    )
}

pub fn add_service(conn: &Connection, form: &ServiceForm) -> Result<i32> {
    conn.execute(
        "INSERT INTO services (name, description, image_url, icon, order_index) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![form.name, form.description, form.image_url, form.icon, form.order_index],
    )?;
    Ok(conn.last_insert_rowid() as i32)
}

pub fn update_service(conn: &Connection, id: i32, form: &ServiceForm) -> Result<()> {
    conn.execute(
        "UPDATE services SET name = ?1, description = ?2, image_url = ?3, icon = ?4, order_index = ?5 WHERE id = ?6",
        rusqlite::params![form.name, form.description, form.image_url, form.icon, form.order_index, id],
    )?;
    Ok(())
}

pub fn delete_service(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM services WHERE id = ?1", [id])?;
    Ok(())
}

// Email Settings functions
pub fn get_email_settings(conn: &Connection) -> Result<EmailSettings> {
    // First ensure the settings row exists
    conn.execute(
        "INSERT OR IGNORE INTO email_settings (id, smtp_server, smtp_port, smtp_username, smtp_password, notification_email, enabled)
         VALUES (1, 'smtp.gmail.com', 587, '', '', '', 0)",
        [],
    )?;
    
    conn.query_row(
        "SELECT id, smtp_server, smtp_port, smtp_username, smtp_password, notification_email, enabled FROM email_settings WHERE id = 1",
        [],
        |row| {
            Ok(EmailSettings {
                id: row.get(0)?,
                smtp_server: row.get(1)?,
                smtp_port: row.get(2)?,
                smtp_username: row.get(3)?,
                smtp_password: row.get(4)?,
                notification_email: row.get(5)?,
                enabled: row.get::<_, i32>(6)? != 0,
            })
        },
    )
}

pub fn update_email_settings(conn: &Connection, settings: &EmailSettingsForm) -> Result<()> {
    let enabled = settings.enabled.as_ref().map(|v| v == "on" || v == "1").unwrap_or(false);
    
    conn.execute(
        "INSERT OR REPLACE INTO email_settings (id, smtp_server, smtp_port, smtp_username, smtp_password, notification_email, enabled)
         VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![
            settings.smtp_server,
            settings.smtp_port,
            settings.smtp_username,
            settings.smtp_password,
            settings.notification_email,
            enabled as i32
        ],
    )?;
    Ok(())
}

// Site Content functions
fn init_default_site_content(conn: &Connection) -> Result<()> {
    let defaults = vec![
        // Hero Section
        ("hero_greeting", "Hello!", "hero", "Hero section greeting badge"),
        ("hero_intro", "I'm", "hero", "Text before name"),
        ("hero_subtitle", "Full Stack Developer & AI Enthusiast", "hero", "Hero subtitle/title"),
        ("hero_description", "Passionate about building innovative solutions and exploring cutting-edge technologies.", "hero", "Hero description text"),
        ("hero_btn_portfolio", "Portfolio", "hero", "Portfolio button text"),
        ("hero_btn_hire", "Hire me", "hero", "Hire me button text"),
        ("hero_btn_resume", "Download Resume", "hero", "Download resume button text"),
        ("hero_experience_years", "10", "hero", "Years of experience number"),
        ("hero_experience_label", "Years Experience", "hero", "Experience label text"),
        
        // About Section
        ("about_badge", "About Me", "about", "About section badge text"),
        ("about_heading", "Get a website that will make a lasting impression on your audience!!!", "about", "About section main heading"),
        
        // Tech Stack Section
        ("tech_title", "Tech Stack", "tech", "Tech stack section title"),
        ("tech_items", "Rust,Python,Microsoft,AI,Automation", "tech", "Comma-separated tech items"),
        
        // Skills Section
        ("skills_title", "My Skills", "skills", "Skills section title"),
        ("skills_subtitle", "Technologies and tools I work with", "skills", "Skills section subtitle"),
        
        // Services Section
        ("services_title_prefix", "My", "services", "Services title prefix"),
        ("services_title", "Services", "services", "Services section title"),
        ("services_subtitle", "Delivering exceptional digital experiences tailored to your needs", "services", "Services section subtitle"),
        
        // Why Hire Me Section
        ("hire_title_prefix", "Why", "hire", "Hire title prefix"),
        ("hire_title", "Hire me", "hire", "Hire section title"),
        ("hire_btn", "Hire Me", "hire", "Hire button text"),
        ("stats_experience", "10+", "hire", "Years of experience stat"),
        ("stats_experience_label", "Years of Experience", "hire", "Experience stat label"),
        ("stats_projects", "4K", "hire", "Projects completed stat"),
        ("stats_projects_label", "Projects Completed", "hire", "Projects stat label"),
        ("stats_customers", "12K", "hire", "Happy customers stat"),
        ("stats_customers_label", "Happy Customers", "hire", "Customers stat label"),
        
        // Work Process Section
        ("process_title_prefix", "Our Work", "process", "Process title prefix"),
        ("process_title", "Process", "process", "Process section title"),
        ("process_subtitle", "A streamlined approach to bringing your ideas to life", "process", "Process section subtitle"),
        
        // Portfolio Section
        ("portfolio_title_prefix", "Look at my", "portfolio", "Portfolio title prefix"),
        ("portfolio_title", "Portfolio", "portfolio", "Portfolio section title"),
        ("portfolio_subtitle", "Some of my recent work", "portfolio", "Portfolio section subtitle"),
        ("portfolio_btn", "View All Projects", "portfolio", "View all projects button"),
        
        // CTA Section
        ("cta_title_prefix", "Let's Work", "cta", "CTA title prefix"),
        ("cta_title", "Together", "cta", "CTA section title"),
        ("cta_subtitle", "Have a project in mind? Let's create something amazing together.", "cta", "CTA section subtitle"),
        ("cta_btn", "Get in Touch", "cta", "CTA button text"),
        
        // Projects Section
        ("projects_title", "Featured Projects", "projects", "Projects section title"),
        ("projects_subtitle", "Some of my recent work", "projects", "Projects section subtitle"),
        ("projects_btn_view", "View Project", "projects", "View project button text"),
        ("projects_btn_all", "View All Projects", "projects", "View all projects button text"),
        
        // Blog Section
        ("blog_title", "Latest Articles", "blog", "Blog section title"),
        ("blog_subtitle", "Thoughts, tutorials, and insights", "blog", "Blog section subtitle"),
        ("blog_btn_read", "Read More", "blog", "Read more button text"),
        ("blog_btn_all", "View All Articles", "blog", "View all articles button text"),
        
        // Contact Section
        ("contact_label", "Contact", "contact", "Contact page label"),
        ("contact_title_prefix", "Get in", "contact", "Contact title prefix"),
        ("contact_title", "Touch", "contact", "Contact section title"),
        ("contact_subtitle", "Have a question or want to work together? Feel free to reach out!", "contact", "Contact section subtitle"),
        ("contact_form_title", "Send a Message", "contact", "Contact form title"),
        ("contact_btn", "Send Message", "contact", "Send message button text"),
        ("contact_success", "Thank you! Your message has been sent successfully.", "contact", "Success message after form submission"),
        
        // Form Labels
        ("form_name_label", "Name", "form", "Name field label"),
        ("form_name_placeholder", "Your name", "form", "Name field placeholder"),
        ("form_email_label", "Email", "form", "Email field label"),
        ("form_email_placeholder", "your@email.com", "form", "Email field placeholder"),
        ("form_subject_label", "Subject", "form", "Subject field label"),
        ("form_subject_placeholder", "What's this about?", "form", "Subject field placeholder"),
        ("form_message_label", "Message", "form", "Message field label"),
        ("form_message_placeholder", "Your message...", "form", "Message field placeholder"),
        
        // Roadmap Section
        ("roadmap_step1_title", "Concept", "roadmap", "Step 1 title"),
        ("roadmap_step1_desc", "Understanding your vision, goals, and requirements to create a solid foundation.", "roadmap", "Step 1 description"),
        ("roadmap_step2_title", "Design", "roadmap", "Step 2 title"),
        ("roadmap_step2_desc", "Creating beautiful, intuitive interfaces that engage and delight users.", "roadmap", "Step 2 description"),
        ("roadmap_step3_title", "Development", "roadmap", "Step 3 title"),
        ("roadmap_step3_desc", "Building robust, scalable solutions with clean, maintainable code.", "roadmap", "Step 3 description"),
        
        // Footer
        ("footer_copyright", "© 2024", "footer", "Footer copyright year"),
        ("footer_rights", "All rights reserved.", "footer", "Footer rights text"),
        ("footer_quick_links", "Quick Links", "footer", "Footer quick links title"),
        ("footer_connect", "Connect", "footer", "Footer connect title"),
        ("footer_tagline", "Built with passion using Rust & Actix Web", "footer", "Footer tagline"),
        
        // Education Section
        ("education_label", "Learning", "education", "Education section label"),
        ("education_title", "Education", "education", "Education section title"),
        
        // Experience Section
        ("experience_label", "Career", "experience", "Experience section label"),
        ("experience_title_prefix", "Work", "experience", "Experience title prefix"),
        ("experience_title", "Experience", "experience", "Experience section title"),
        
        // Skills Section (About page)
        ("skills_label", "Expertise", "skills", "Skills section label"),
        ("skills_title_prefix", "Technical", "skills", "Skills title prefix"),
        
        // About Page Header
        ("about_label", "Introduction", "about", "About page label"),
        ("about_title_prefix", "About", "about", "About page title prefix"),
        ("about_title", "Me", "about", "About page title"),
        ("about_subtitle", "Get to know more about my background, skills, and experience", "about", "About page subtitle"),
        ("about_experience_years", "5+", "about", "Years of experience badge"),
        ("about_experience_label", "Years Experience", "about", "Experience badge label"),
        ("about_resume_btn", "Download Resume", "about", "Download resume button text"),
        ("about_contact_btn", "Contact Me", "about", "Contact me button text"),
        
        // Navigation
        ("nav_home", "Home", "nav", "Home navigation link"),
        ("nav_about", "About", "nav", "About navigation link"),
        ("nav_projects", "Projects", "nav", "Projects navigation link"),
        ("nav_blog", "Blog", "nav", "Blog navigation link"),
        ("nav_contact", "Contact", "nav", "Contact navigation link"),
    ];
    
    for (key, value, section, description) in defaults {
        conn.execute(
            "INSERT OR IGNORE INTO site_content (key, value, section, description) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![key, value, section, description],
        )?;
    }
    
    Ok(())
}

pub fn get_site_content(conn: &Connection) -> Result<std::collections::HashMap<String, String>> {
    let mut stmt = conn.prepare("SELECT key, value FROM site_content")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    
    let mut content = std::collections::HashMap::new();
    for row in rows {
        if let Ok((key, value)) = row {
            content.insert(key, value);
        }
    }
    Ok(content)
}

pub fn get_site_content_by_section(conn: &Connection) -> Result<std::collections::HashMap<String, Vec<SiteContentItem>>> {
    let mut stmt = conn.prepare("SELECT key, value, section, description FROM site_content ORDER BY section, key")?;
    let rows = stmt.query_map([], |row| {
        Ok(SiteContentItem {
            key: row.get(0)?,
            value: row.get(1)?,
            section: row.get(2)?,
            description: row.get(3)?,
        })
    })?;
    
    let mut by_section: std::collections::HashMap<String, Vec<SiteContentItem>> = std::collections::HashMap::new();
    for row in rows {
        if let Ok(item) = row {
            by_section.entry(item.section.clone()).or_insert_with(Vec::new).push(item);
        }
    }
    Ok(by_section)
}

pub fn update_site_content(conn: &Connection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "UPDATE site_content SET value = ?1 WHERE key = ?2",
        rusqlite::params![value, key],
    )?;
    Ok(())
}

pub fn update_site_content_batch(conn: &Connection, updates: &std::collections::HashMap<String, String>) -> Result<()> {
    for (key, value) in updates {
        conn.execute(
            "UPDATE site_content SET value = ?1 WHERE key = ?2",
            rusqlite::params![value, key],
        )?;
    }
    Ok(())
}
