use actix_web::{web, HttpResponse};
use actix_session::Session;
use actix_multipart::Multipart;
use futures_util::StreamExt;
use crate::AppState;
use crate::db;
use crate::auth;
use crate::models::*;

fn require_auth(session: &Session) -> Option<HttpResponse> {
    if !auth::is_authenticated(session) {
        Some(HttpResponse::Found()
            .append_header(("Location", "/admin/login"))
            .finish())
    } else {
        None
    }
}

pub async fn dashboard(data: web::Data<AppState>, session: Session) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    
    let profile = db::get_profile(&conn).unwrap();
    let projects = db::get_projects(&conn).unwrap_or_default();
    let blogs = db::get_blogs(&conn).unwrap_or_default();
    let skills = db::get_skills(&conn).unwrap_or_default();
    let messages = db::get_messages(&conn).unwrap_or_default();
    let unread_count = db::get_unread_message_count(&conn).unwrap_or(0);
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("projects_count", &projects.len());
    context.insert("blogs_count", &blogs.len());
    context.insert("skills_count", &skills.len());
    context.insert("messages_count", &messages.len());
    context.insert("unread_count", &unread_count);
    context.insert("page_title", "Admin Dashboard");
    
    let rendered = data.tera.render("admin/dashboard.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn profile_page(data: web::Data<AppState>, session: Session) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let profile = db::get_profile(&conn).unwrap();
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("page_title", "Edit Profile");
    context.insert("success", &false);
    
    let rendered = data.tera.render("admin/profile.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn update_profile(
    data: web::Data<AppState>,
    form: web::Form<ProfileForm>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let _ = db::update_profile(&conn, &form);
    let profile = db::get_profile(&conn).unwrap();
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("page_title", "Edit Profile");
    context.insert("success", &true);
    
    let rendered = data.tera.render("admin/profile.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn skills_page(data: web::Data<AppState>, session: Session) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let profile = db::get_profile(&conn).unwrap();
    let skills = db::get_skills(&conn).unwrap_or_default();
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("skills", &skills);
    context.insert("page_title", "Manage Skills");
    
    let rendered = data.tera.render("admin/skills.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn add_skill(
    data: web::Data<AppState>,
    mut payload: Multipart,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let mut skill_form = SkillForm::default();
    let mut icon_url = String::new();
    
    // Process multipart form
    while let Some(item) = payload.next().await {
        if let Ok(mut field) = item {
            // Get field name from content disposition
            let field_name = field.content_disposition()
                .and_then(|cd| cd.get_name().map(|s| s.to_string()))
                .unwrap_or_default();
            
            // Check if it's a file field with actual content
            let filename = field.content_disposition()
                .and_then(|cd| cd.get_filename().map(|s| s.to_string()))
                .unwrap_or_default();
            
            let is_file_with_content = !filename.is_empty();
            
            if field_name == "icon_file" && is_file_with_content {
                // Handle file upload
                let content_type = field.content_type()
                    .map(|ct| ct.to_string())
                    .unwrap_or_else(|| "image/png".to_string());
                
                let mut file_data = Vec::new();
                while let Some(chunk) = field.next().await {
                    if let Ok(bytes) = chunk {
                        file_data.extend_from_slice(&bytes);
                    }
                }
                
                if !file_data.is_empty() {
                    // Save to database as image
                    let conn = data.db.lock().unwrap();
                    let image_id = uuid::Uuid::new_v4().to_string();
                    let _ = db::save_image(&conn, &image_id, &filename, &content_type, &file_data);
                    icon_url = format!("/images/{}", image_id);
                }
            } else {
                // Handle text fields
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    if let Ok(bytes) = chunk {
                        if let Ok(s) = std::str::from_utf8(&bytes) {
                            value.push_str(s);
                        }
                    }
                }
                
                match field_name.as_str() {
                    "name" => skill_form.name = value,
                    "category" => skill_form.category = value,
                    "proficiency" => skill_form.proficiency = value.parse().unwrap_or(80),
                    "icon" => skill_form.icon = value,
                    _ => {}
                }
            }
        }
    }
    
    // Set icon_url if file was uploaded
    skill_form.icon_url = icon_url;
    
    let conn = data.db.lock().unwrap();
    let _ = db::add_skill(&conn, &skill_form);
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/skills"))
        .finish()
}

pub async fn delete_skill(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let _ = db::delete_skill(&conn, path.into_inner());
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/skills"))
        .finish()
}

pub async fn projects_page(data: web::Data<AppState>, session: Session) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let profile = db::get_profile(&conn).unwrap();
    let projects = db::get_projects(&conn).unwrap_or_default();
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("projects", &projects);
    context.insert("page_title", "Manage Projects");
    
    let rendered = data.tera.render("admin/projects.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn add_project_page(data: web::Data<AppState>, session: Session) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let profile = db::get_profile(&conn).unwrap();
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("page_title", "Add Project");
    context.insert("editing", &false);
    
    let rendered = data.tera.render("admin/project_form.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn add_project(
    data: web::Data<AppState>,
    form: web::Form<ProjectForm>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let _ = db::add_project(&conn, &form);
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/projects"))
        .finish()
}

pub async fn edit_project_page(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let profile = db::get_profile(&conn).unwrap();
    
    match db::get_project_by_id(&conn, path.into_inner()) {
        Ok(project) => {
            let mut context = tera::Context::new();
            context.insert("profile", &profile);
            context.insert("project", &project);
            context.insert("page_title", "Edit Project");
            context.insert("editing", &true);
            
            let rendered = data.tera.render("admin/project_form.html", &context).unwrap();
            HttpResponse::Ok().content_type("text/html").body(rendered)
        }
        Err(_) => HttpResponse::NotFound().body("Project not found"),
    }
}

pub async fn update_project(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    form: web::Form<ProjectForm>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let _ = db::update_project(&conn, path.into_inner(), &form);
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/projects"))
        .finish()
}

pub async fn delete_project(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let _ = db::delete_project(&conn, path.into_inner());
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/projects"))
        .finish()
}

pub async fn blogs_page(data: web::Data<AppState>, session: Session) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let profile = db::get_profile(&conn).unwrap();
    let blogs = db::get_blogs(&conn).unwrap_or_default();
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("blogs", &blogs);
    context.insert("page_title", "Manage Blogs");
    
    let rendered = data.tera.render("admin/blogs.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn add_blog_page(data: web::Data<AppState>, session: Session) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let profile = db::get_profile(&conn).unwrap();
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("page_title", "Add Blog Post");
    context.insert("editing", &false);
    
    let rendered = data.tera.render("admin/blog_form.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn add_blog(
    data: web::Data<AppState>,
    form: web::Form<BlogForm>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let _ = db::add_blog(&conn, &form);
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/blogs"))
        .finish()
}

pub async fn edit_blog_page(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let profile = db::get_profile(&conn).unwrap();
    
    match db::get_blog_by_id(&conn, path.into_inner()) {
        Ok(blog) => {
            let mut context = tera::Context::new();
            context.insert("profile", &profile);
            context.insert("blog", &blog);
            context.insert("page_title", "Edit Blog Post");
            context.insert("editing", &true);
            
            let rendered = data.tera.render("admin/blog_form.html", &context).unwrap();
            HttpResponse::Ok().content_type("text/html").body(rendered)
        }
        Err(_) => HttpResponse::NotFound().body("Blog post not found"),
    }
}

pub async fn update_blog(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    form: web::Form<BlogForm>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let _ = db::update_blog(&conn, path.into_inner(), &form);
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/blogs"))
        .finish()
}

pub async fn delete_blog(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let _ = db::delete_blog(&conn, path.into_inner());
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/blogs"))
        .finish()
}

pub async fn messages_page(data: web::Data<AppState>, session: Session) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let profile = db::get_profile(&conn).unwrap();
    let messages = db::get_messages(&conn).unwrap_or_default();
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("messages", &messages);
    context.insert("page_title", "Messages");
    
    let rendered = data.tera.render("admin/messages.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn delete_message(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let _ = db::delete_message(&conn, path.into_inner());
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/messages"))
        .finish()
}

// Helper function to extract multipart form data
async fn extract_multipart_data(mut payload: Multipart) -> (std::collections::HashMap<String, String>, Option<(String, String, Vec<u8>)>) {
    let mut fields: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut image_data: Option<(String, String, Vec<u8>)> = None;
    
    while let Some(item) = payload.next().await {
        if let Ok(mut field) = item {
            // Get field name from content disposition
            let field_name = field.content_disposition()
                .and_then(|cd| cd.get_name().map(|s| s.to_string()))
                .unwrap_or_default();
            
            // Check if it's a file field with actual content
            let filename = field.content_disposition()
                .and_then(|cd| cd.get_filename().map(|s| s.to_string()))
                .unwrap_or_default();
            
            let is_file_with_content = !filename.is_empty();
            
            if is_file_with_content {
                let content_type = field.content_type()
                    .map(|ct| ct.to_string())
                    .unwrap_or_else(|| "application/octet-stream".to_string());
                
                let mut data = Vec::new();
                while let Some(chunk) = field.next().await {
                    if let Ok(bytes) = chunk {
                        data.extend_from_slice(&bytes);
                    }
                }
                
                if !data.is_empty() {
                    log::info!("Received file upload: {} ({} bytes)", filename, data.len());
                    image_data = Some((filename, content_type, data));
                }
            } else {
                // Regular text field (or empty file field - still read to drain it)
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    if let Ok(bytes) = chunk {
                        if let Ok(text) = std::str::from_utf8(&bytes) {
                            value.push_str(text);
                        }
                    }
                }
                if !field_name.is_empty() {
                    log::debug!("Form field: {} = {}", field_name, &value);
                    fields.insert(field_name, value);
                }
            }
        }
    }
    
    log::info!("Extracted {} form fields", fields.len());
    (fields, image_data)
}

// Extract multiple files from multipart data
async fn extract_multipart_with_files(mut payload: Multipart) -> (std::collections::HashMap<String, String>, std::collections::HashMap<String, (String, String, Vec<u8>)>) {
    let mut fields: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut files: std::collections::HashMap<String, (String, String, Vec<u8>)> = std::collections::HashMap::new();
    
    while let Some(item) = payload.next().await {
        if let Ok(mut field) = item {
            let field_name = field.content_disposition()
                .and_then(|cd| cd.get_name().map(|s| s.to_string()))
                .unwrap_or_default();
            
            let filename = field.content_disposition()
                .and_then(|cd| cd.get_filename().map(|s| s.to_string()))
                .unwrap_or_default();
            
            let is_file_with_content = !filename.is_empty();
            
            if is_file_with_content {
                let content_type = field.content_type()
                    .map(|ct| ct.to_string())
                    .unwrap_or_else(|| "application/octet-stream".to_string());
                
                let mut data = Vec::new();
                while let Some(chunk) = field.next().await {
                    if let Ok(bytes) = chunk {
                        data.extend_from_slice(&bytes);
                    }
                }
                
                if !data.is_empty() {
                    log::info!("Received file upload for field '{}': {} ({} bytes)", field_name, filename, data.len());
                    files.insert(field_name, (filename, content_type, data));
                }
            } else {
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    if let Ok(bytes) = chunk {
                        if let Ok(text) = std::str::from_utf8(&bytes) {
                            value.push_str(text);
                        }
                    }
                }
                if !field_name.is_empty() {
                    fields.insert(field_name, value);
                }
            }
        }
    }
    
    log::info!("Extracted {} form fields and {} files", fields.len(), files.len());
    (fields, files)
}

// Generic image upload endpoint (AJAX)
pub async fn upload_image(
    data: web::Data<AppState>,
    payload: Multipart,
    session: Session,
) -> HttpResponse {
    if !auth::is_authenticated(&session) {
        return HttpResponse::Unauthorized().json(serde_json::json!({"error": "Unauthorized"}));
    }
    
    let (_, image_data) = extract_multipart_data(payload).await;
    
    if let Some((filename, content_type, bytes)) = image_data {
        let image_id = uuid::Uuid::new_v4().to_string();
        let conn = data.db.lock().unwrap();
        
        match db::save_image(&conn, &image_id, &filename, &content_type, &bytes) {
            Ok(_) => {
                let image_url = format!("/images/{}", image_id);
                HttpResponse::Ok().json(serde_json::json!({
                    "success": true,
                    "image_url": image_url,
                    "image_id": image_id
                }))
            }
            Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to save image: {}", e)
            }))
        }
    } else {
        HttpResponse::BadRequest().json(serde_json::json!({"error": "No image provided"}))
    }
}

// Update profile with image upload support
pub async fn update_profile_with_image(
    data: web::Data<AppState>,
    payload: Multipart,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let (fields, files) = extract_multipart_with_files(payload).await;
    
    log::info!("Updating profile with fields: {:?}", fields.keys().collect::<Vec<_>>());
    
    // Get the connection once
    let conn = data.db.lock().unwrap();
    
    // Handle avatar image upload if present
    let avatar_url = if let Some((filename, content_type, bytes)) = files.get("avatar_file") {
        let image_id = uuid::Uuid::new_v4().to_string();
        let _ = db::save_image(&conn, &image_id, filename, content_type, bytes);
        format!("/images/{}", image_id)
    } else {
        fields.get("avatar_url").cloned().unwrap_or_default()
    };
    
    // Handle resume file upload if present
    let resume_url = if let Some((filename, content_type, bytes)) = files.get("resume_file") {
        let file_id = uuid::Uuid::new_v4().to_string();
        let _ = db::save_image(&conn, &file_id, filename, content_type, bytes);
        format!("/files/{}", file_id)
    } else {
        fields.get("resume_url").cloned().unwrap_or_default()
    };
    
    let form = ProfileForm {
        name: fields.get("name").cloned().unwrap_or_default(),
        title: fields.get("title").cloned().unwrap_or_default(),
        bio: fields.get("bio").cloned().unwrap_or_default(),
        email: fields.get("email").cloned().unwrap_or_default(),
        phone: fields.get("phone").cloned().unwrap_or_default(),
        location: fields.get("location").cloned().unwrap_or_default(),
        github_url: fields.get("github_url").cloned().unwrap_or_default(),
        linkedin_url: fields.get("linkedin_url").cloned().unwrap_or_default(),
        twitter_url: fields.get("twitter_url").cloned().unwrap_or_default(),
        resume_url,
        avatar_url,
    };
    
    match db::update_profile(&conn, &form) {
        Ok(_) => log::info!("Profile updated successfully"),
        Err(e) => log::error!("Failed to update profile: {}", e),
    }
    
    let profile = db::get_profile(&conn).unwrap();
    
    drop(conn);
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("page_title", "Edit Profile");
    context.insert("success", &true);
    
    let rendered = data.tera.render("admin/profile.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

// Add project with image upload support
pub async fn add_project_with_image(
    data: web::Data<AppState>,
    payload: Multipart,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let (fields, image_data) = extract_multipart_data(payload).await;
    
    log::info!("Adding project with fields: {:?}", fields.keys().collect::<Vec<_>>());
    
    // Get the connection once and use it for both operations
    let conn = data.db.lock().unwrap();
    
    // Handle image upload if present
    let image_url = if let Some((filename, content_type, bytes)) = image_data {
        let image_id = uuid::Uuid::new_v4().to_string();
        let _ = db::save_image(&conn, &image_id, &filename, &content_type, &bytes);
        format!("/images/{}", image_id)
    } else {
        fields.get("image_url").cloned().unwrap_or_default()
    };
    
    let form = ProjectForm {
        title: fields.get("title").cloned().unwrap_or_default(),
        description: fields.get("description").cloned().unwrap_or_default(),
        content: fields.get("content").cloned().unwrap_or_default(),
        image_url,
        demo_url: fields.get("demo_url").cloned().unwrap_or_default(),
        github_url: fields.get("github_url").cloned().unwrap_or_default(),
        technologies: fields.get("technologies").cloned().unwrap_or_default(),
        featured: fields.get("featured").map(|s| s.to_string()),
    };
    
    log::info!("Adding project: {}", form.title);
    
    match db::add_project(&conn, &form) {
        Ok(_) => log::info!("Project added successfully"),
        Err(e) => log::error!("Failed to add project: {}", e),
    }
    
    drop(conn); // Explicitly drop the lock
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/projects"))
        .finish()
}

// Update project with image upload support
pub async fn update_project_with_image(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    payload: Multipart,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let project_id = path.into_inner();
    let (fields, image_data) = extract_multipart_data(payload).await;
    
    log::info!("Updating project {} with fields: {:?}", project_id, fields.keys().collect::<Vec<_>>());
    
    // Get the connection once and use it for both operations
    let conn = data.db.lock().unwrap();
    
    // Handle image upload if present
    let image_url = if let Some((filename, content_type, bytes)) = image_data {
        let image_id = uuid::Uuid::new_v4().to_string();
        let _ = db::save_image(&conn, &image_id, &filename, &content_type, &bytes);
        format!("/images/{}", image_id)
    } else {
        fields.get("image_url").cloned().unwrap_or_default()
    };
    
    let form = ProjectForm {
        title: fields.get("title").cloned().unwrap_or_default(),
        description: fields.get("description").cloned().unwrap_or_default(),
        content: fields.get("content").cloned().unwrap_or_default(),
        image_url,
        demo_url: fields.get("demo_url").cloned().unwrap_or_default(),
        github_url: fields.get("github_url").cloned().unwrap_or_default(),
        technologies: fields.get("technologies").cloned().unwrap_or_default(),
        featured: fields.get("featured").map(|s| s.to_string()),
    };
    
    match db::update_project(&conn, project_id, &form) {
        Ok(_) => log::info!("Project {} updated successfully", project_id),
        Err(e) => log::error!("Failed to update project: {}", e),
    }
    
    drop(conn);
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/projects"))
        .finish()
}

// Add blog with image upload support
pub async fn add_blog_with_image(
    data: web::Data<AppState>,
    payload: Multipart,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let (fields, image_data) = extract_multipart_data(payload).await;
    
    log::info!("Adding blog with fields: {:?}", fields.keys().collect::<Vec<_>>());
    
    // Get the connection once
    let conn = data.db.lock().unwrap();
    
    // Handle image upload if present
    let image_url = if let Some((filename, content_type, bytes)) = image_data {
        let image_id = uuid::Uuid::new_v4().to_string();
        let _ = db::save_image(&conn, &image_id, &filename, &content_type, &bytes);
        format!("/images/{}", image_id)
    } else {
        fields.get("image_url").cloned().unwrap_or_default()
    };
    
    let form = BlogForm {
        title: fields.get("title").cloned().unwrap_or_default(),
        excerpt: fields.get("excerpt").cloned().unwrap_or_default(),
        content: fields.get("content").cloned().unwrap_or_default(),
        image_url,
        tags: fields.get("tags").cloned().unwrap_or_default(),
        published: fields.get("published").map(|s| s.to_string()),
    };
    
    log::info!("Adding blog: {}", form.title);
    
    match db::add_blog(&conn, &form) {
        Ok(_) => log::info!("Blog added successfully"),
        Err(e) => log::error!("Failed to add blog: {}", e),
    }
    
    drop(conn);
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/blogs"))
        .finish()
}

// Update blog with image upload support
pub async fn update_blog_with_image(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    payload: Multipart,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let blog_id = path.into_inner();
    let (fields, image_data) = extract_multipart_data(payload).await;
    
    log::info!("Updating blog {} with fields: {:?}", blog_id, fields.keys().collect::<Vec<_>>());
    
    // Get the connection once
    let conn = data.db.lock().unwrap();
    
    // Handle image upload if present
    let image_url = if let Some((filename, content_type, bytes)) = image_data {
        let image_id = uuid::Uuid::new_v4().to_string();
        let _ = db::save_image(&conn, &image_id, &filename, &content_type, &bytes);
        format!("/images/{}", image_id)
    } else {
        fields.get("image_url").cloned().unwrap_or_default()
    };
    
    let form = BlogForm {
        title: fields.get("title").cloned().unwrap_or_default(),
        excerpt: fields.get("excerpt").cloned().unwrap_or_default(),
        content: fields.get("content").cloned().unwrap_or_default(),
        image_url,
        tags: fields.get("tags").cloned().unwrap_or_default(),
        published: fields.get("published").map(|s| s.to_string()),
    };
    
    match db::update_blog(&conn, blog_id, &form) {
        Ok(_) => log::info!("Blog {} updated successfully", blog_id),
        Err(e) => log::error!("Failed to update blog: {}", e),
    }
    
    drop(conn);
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/blogs"))
        .finish()
}

// Services management
pub async fn services_page(data: web::Data<AppState>, session: Session) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let profile = db::get_profile(&conn).unwrap();
    let services = db::get_services(&conn).unwrap_or_default();
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("services", &services);
    context.insert("page_title", "Manage Services");
    
    let rendered = data.tera.render("admin/services.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn add_service_page(data: web::Data<AppState>, session: Session) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let profile = db::get_profile(&conn).unwrap();
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("page_title", "Add Service");
    
    let rendered = data.tera.render("admin/service_form.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn edit_service_page(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let service_id = path.into_inner();
    let conn = data.db.lock().unwrap();
    let profile = db::get_profile(&conn).unwrap();
    
    match db::get_service(&conn, service_id) {
        Ok(service) => {
            let mut context = tera::Context::new();
            context.insert("profile", &profile);
            context.insert("service", &service);
            context.insert("page_title", "Edit Service");
            
            let rendered = data.tera.render("admin/service_form.html", &context).unwrap();
            HttpResponse::Ok().content_type("text/html").body(rendered)
        }
        Err(_) => HttpResponse::Found()
            .append_header(("Location", "/admin/services"))
            .finish(),
    }
}

pub async fn add_service_with_image(
    data: web::Data<AppState>,
    payload: Multipart,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let (fields, image_data) = extract_multipart_data(payload).await;
    
    log::info!("Adding service with fields: {:?}", fields.keys().collect::<Vec<_>>());
    
    let conn = data.db.lock().unwrap();
    
    // Handle image upload if present
    let image_url = if let Some((filename, content_type, bytes)) = image_data {
        let image_id = uuid::Uuid::new_v4().to_string();
        let _ = db::save_image(&conn, &image_id, &filename, &content_type, &bytes);
        format!("/images/{}", image_id)
    } else {
        fields.get("image_url").cloned().unwrap_or_default()
    };
    
    let form = ServiceForm {
        name: fields.get("name").cloned().unwrap_or_default(),
        description: fields.get("description").cloned().unwrap_or_default(),
        image_url,
        icon: fields.get("icon").cloned().unwrap_or_default(),
        order_index: fields.get("order_index").and_then(|s| s.parse().ok()).unwrap_or(0),
    };
    
    match db::add_service(&conn, &form) {
        Ok(_) => log::info!("Service added successfully"),
        Err(e) => log::error!("Failed to add service: {}", e),
    }
    
    drop(conn);
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/services"))
        .finish()
}

pub async fn update_service_with_image(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    payload: Multipart,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let service_id = path.into_inner();
    let (fields, image_data) = extract_multipart_data(payload).await;
    
    log::info!("Updating service {} with fields: {:?}", service_id, fields.keys().collect::<Vec<_>>());
    
    let conn = data.db.lock().unwrap();
    
    // Handle image upload if present
    let image_url = if let Some((filename, content_type, bytes)) = image_data {
        let image_id = uuid::Uuid::new_v4().to_string();
        let _ = db::save_image(&conn, &image_id, &filename, &content_type, &bytes);
        format!("/images/{}", image_id)
    } else {
        fields.get("image_url").cloned().unwrap_or_default()
    };
    
    let form = ServiceForm {
        name: fields.get("name").cloned().unwrap_or_default(),
        description: fields.get("description").cloned().unwrap_or_default(),
        image_url,
        icon: fields.get("icon").cloned().unwrap_or_default(),
        order_index: fields.get("order_index").and_then(|s| s.parse().ok()).unwrap_or(0),
    };
    
    match db::update_service(&conn, service_id, &form) {
        Ok(_) => log::info!("Service {} updated successfully", service_id),
        Err(e) => log::error!("Failed to update service: {}", e),
    }
    
    drop(conn);
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/services"))
        .finish()
}

pub async fn delete_service(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let service_id = path.into_inner();
    let conn = data.db.lock().unwrap();
    
    match db::delete_service(&conn, service_id) {
        Ok(_) => log::info!("Service {} deleted successfully", service_id),
        Err(e) => log::error!("Failed to delete service: {}", e),
    }
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/services"))
        .finish()
}

// Email Settings
pub async fn email_settings_page(
    data: web::Data<AppState>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let email_settings = db::get_email_settings(&conn).unwrap_or_default();
    
    let mut context = tera::Context::new();
    context.insert("email_settings", &email_settings);
    context.insert("page_title", "Email Settings");
    
    let rendered = data.tera.render("admin/email_settings.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn update_email_settings(
    data: web::Data<AppState>,
    form: web::Form<EmailSettingsForm>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    
    match db::update_email_settings(&conn, &form) {
        Ok(_) => log::info!("Email settings updated successfully"),
        Err(e) => log::error!("Failed to update email settings: {}", e),
    }
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/email-settings"))
        .finish()
}

pub async fn test_email(
    data: web::Data<AppState>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    
    if let Ok(settings) = db::get_email_settings(&conn) {
        if !settings.enabled {
            return HttpResponse::Ok()
                .content_type("application/json")
                .body(r#"{"success": false, "message": "Email notifications are disabled"}"#);
        }
        
        let test_result = crate::email::send_notification_email(
            &settings,
            "Test User",
            &settings.notification_email,
            "Test Email",
            "This is a test email from your portfolio website. If you received this, email notifications are working correctly!",
        );
        
        match test_result {
            Ok(_) => HttpResponse::Ok()
                .content_type("application/json")
                .body(r#"{"success": true, "message": "Test email sent successfully!"}"#),
            Err(e) => HttpResponse::Ok()
                .content_type("application/json")
                .body(format!(r#"{{"success": false, "message": "{}"}}"#, e.replace('"', "\\\""))),
        }
    } else {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(r#"{"success": false, "message": "Could not load email settings"}"#)
    }
}

// Site Content Management
pub async fn site_content_page(
    data: web::Data<AppState>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let content_by_section = db::get_site_content_by_section(&conn).unwrap_or_default();
    
    let mut context = tera::Context::new();
    context.insert("content_by_section", &content_by_section);
    context.insert("page_title", "Site Content");
    
    let rendered = data.tera.render("admin/site_content.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn update_site_content(
    data: web::Data<AppState>,
    form: web::Form<std::collections::HashMap<String, String>>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    
    match db::update_site_content_batch(&conn, &form.into_inner()) {
        Ok(_) => log::info!("Site content updated successfully"),
        Err(e) => log::error!("Failed to update site content: {}", e),
    }
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/site-content"))
        .finish()
}

// Education Management
pub async fn education_page(
    data: web::Data<AppState>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let education = db::get_education(&conn).unwrap_or_default();
    
    let mut context = tera::Context::new();
    context.insert("education", &education);
    context.insert("page_title", "Education");
    
    let rendered = data.tera.render("admin/education.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn add_education_page(
    data: web::Data<AppState>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let mut context = tera::Context::new();
    context.insert("page_title", "Add Education");
    
    let rendered = data.tera.render("admin/education_form.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn add_education(
    data: web::Data<AppState>,
    form: web::Form<Education>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    
    match db::add_education(&conn, &form.into_inner()) {
        Ok(_) => log::info!("Education added successfully"),
        Err(e) => log::error!("Failed to add education: {}", e),
    }
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/education"))
        .finish()
}

pub async fn edit_education_page(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let id = path.into_inner();
    
    match db::get_education_by_id(&conn, id) {
        Ok(edu) => {
            let mut context = tera::Context::new();
            context.insert("education", &edu);
            context.insert("page_title", "Edit Education");
            
            let rendered = data.tera.render("admin/education_form.html", &context).unwrap();
            HttpResponse::Ok().content_type("text/html").body(rendered)
        }
        Err(_) => HttpResponse::NotFound().body("Education not found"),
    }
}

pub async fn update_education(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    form: web::Form<Education>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let id = path.into_inner();
    
    let mut edu = form.into_inner();
    edu.id = id;
    
    match db::update_education(&conn, &edu) {
        Ok(_) => log::info!("Education updated successfully"),
        Err(e) => log::error!("Failed to update education: {}", e),
    }
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/education"))
        .finish()
}

pub async fn delete_education(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    session: Session,
) -> HttpResponse {
    if let Some(redirect) = require_auth(&session) {
        return redirect;
    }
    
    let conn = data.db.lock().unwrap();
    let id = path.into_inner();
    
    match db::delete_education(&conn, id) {
        Ok(_) => log::info!("Education deleted successfully"),
        Err(e) => log::error!("Failed to delete education: {}", e),
    }
    
    HttpResponse::Found()
        .append_header(("Location", "/admin/education"))
        .finish()
}
