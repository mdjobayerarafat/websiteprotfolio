use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub bio: String,
    pub email: String,
    pub phone: String,
    pub location: String,
    pub github_url: String,
    pub linkedin_url: String,
    pub twitter_url: String,
    pub resume_url: String,
    pub avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Skill {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub proficiency: i32,
    pub icon: String,
    pub icon_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub content: String,
    pub image_url: String,
    pub demo_url: String,
    pub github_url: String,
    pub technologies: String,
    pub featured: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    pub content: String,
    pub image_url: String,
    pub tags: String,
    pub published: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Experience {
    pub id: i32,
    pub company: String,
    pub position: String,
    pub description: String,
    pub start_date: String,
    pub end_date: String,
    pub current: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Education {
    pub id: i32,
    pub institution: String,
    pub degree: String,
    pub field: String,
    pub start_date: String,
    pub end_date: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
    pub read: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub icon: String,
    pub order_index: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Admin {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

// Form structs
#[derive(Debug, Deserialize)]
pub struct ProfileForm {
    pub name: String,
    pub title: String,
    pub bio: String,
    pub email: String,
    pub phone: String,
    pub location: String,
    pub github_url: String,
    pub linkedin_url: String,
    pub twitter_url: String,
    pub resume_url: String,
    pub avatar_url: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct SkillForm {
    pub name: String,
    pub category: String,
    pub proficiency: i32,
    pub icon: String,
    pub icon_url: String,
}

#[derive(Debug, Deserialize)]
pub struct ProjectForm {
    pub title: String,
    pub description: String,
    pub content: String,
    pub image_url: String,
    pub demo_url: String,
    pub github_url: String,
    pub technologies: String,
    pub featured: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BlogForm {
    pub title: String,
    pub excerpt: String,
    pub content: String,
    pub image_url: String,
    pub tags: String,
    pub published: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ServiceForm {
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub icon: String,
    pub order_index: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailSettings {
    pub id: i32,
    pub smtp_server: String,
    pub smtp_port: i32,
    pub smtp_username: String,
    pub smtp_password: String,
    pub notification_email: String,
    pub enabled: bool,
}

impl Default for EmailSettings {
    fn default() -> Self {
        EmailSettings {
            id: 1,
            smtp_server: "smtp.gmail.com".to_string(),
            smtp_port: 587,
            smtp_username: String::new(),
            smtp_password: String::new(),
            notification_email: String::new(),
            enabled: false,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct EmailSettingsForm {
    pub smtp_server: String,
    pub smtp_port: i32,
    pub smtp_username: String,
    pub smtp_password: String,
    pub notification_email: String,
    pub enabled: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SiteContentItem {
    pub key: String,
    pub value: String,
    pub section: String,
    pub description: String,
}
