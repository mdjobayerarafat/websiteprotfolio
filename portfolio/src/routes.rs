use actix_web::{web, HttpResponse};
use crate::AppState;
use crate::db;
use crate::models::ContactForm;
use pulldown_cmark::{Parser, html};

fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

pub async fn index(data: web::Data<AppState>) -> HttpResponse {
    let conn = data.db.lock().unwrap();
    
    let profile = db::get_profile(&conn).unwrap();
    let skills = db::get_skills(&conn).unwrap_or_default();
    let featured_projects = db::get_featured_projects(&conn).unwrap_or_default();
    let recent_blogs = db::get_recent_blogs(&conn, 3).unwrap_or_default();
    let experience = db::get_experience(&conn).unwrap_or_default();
    let services = db::get_services(&conn).unwrap_or_default();
    let content = db::get_site_content(&conn).unwrap_or_default();
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("skills", &skills);
    context.insert("featured_projects", &featured_projects);
    context.insert("recent_blogs", &recent_blogs);
    context.insert("experience", &experience);
    context.insert("services", &services);
    context.insert("content", &content);
    context.insert("page_title", "Home");
    
    let rendered = data.tera.render("index.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn about(data: web::Data<AppState>) -> HttpResponse {
    let conn = data.db.lock().unwrap();
    
    let profile = db::get_profile(&conn).unwrap();
    let skills = db::get_skills(&conn).unwrap_or_default();
    let experience = db::get_experience(&conn).unwrap_or_default();
    let education = db::get_education(&conn).unwrap_or_default();
    let content = db::get_site_content(&conn).unwrap_or_default();
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("skills", &skills);
    context.insert("experience", &experience);
    context.insert("education", &education);
    context.insert("content", &content);
    context.insert("page_title", "About");
    
    let rendered = data.tera.render("about.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn projects(data: web::Data<AppState>) -> HttpResponse {
    let conn = data.db.lock().unwrap();
    
    let profile = db::get_profile(&conn).unwrap();
    let projects = db::get_projects(&conn).unwrap_or_default();
    let content = db::get_site_content(&conn).unwrap_or_default();
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("projects", &projects);
    context.insert("content", &content);
    context.insert("page_title", "Projects");
    
    let rendered = data.tera.render("projects.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn project_detail(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let conn = data.db.lock().unwrap();
    let slug = path.into_inner();
    
    let profile = db::get_profile(&conn).unwrap();
    
    match db::get_project_by_slug(&conn, &slug) {
        Ok(project) => {
            let content_html = markdown_to_html(&project.content);
            
            let mut context = tera::Context::new();
            context.insert("profile", &profile);
            context.insert("project", &project);
            context.insert("content_html", &content_html);
            context.insert("page_title", &project.title);
            
            let rendered = data.tera.render("project_detail.html", &context).unwrap();
            HttpResponse::Ok().content_type("text/html").body(rendered)
        }
        Err(_) => HttpResponse::NotFound().body("Project not found"),
    }
}

pub async fn blogs(data: web::Data<AppState>) -> HttpResponse {
    let conn = data.db.lock().unwrap();
    
    let profile = db::get_profile(&conn).unwrap();
    let blogs = db::get_published_blogs(&conn).unwrap_or_default();
    let content = db::get_site_content(&conn).unwrap_or_default();
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("blogs", &blogs);
    context.insert("content", &content);
    context.insert("page_title", "Blog");
    
    let rendered = data.tera.render("blogs.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn blog_detail(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let conn = data.db.lock().unwrap();
    let slug = path.into_inner();
    
    let profile = db::get_profile(&conn).unwrap();
    
    match db::get_blog_by_slug(&conn, &slug) {
        Ok(blog) => {
            let content_html = markdown_to_html(&blog.content);
            
            let mut context = tera::Context::new();
            context.insert("profile", &profile);
            context.insert("blog", &blog);
            context.insert("content_html", &content_html);
            context.insert("page_title", &blog.title);
            
            let rendered = data.tera.render("blog_detail.html", &context).unwrap();
            HttpResponse::Ok().content_type("text/html").body(rendered)
        }
        Err(_) => HttpResponse::NotFound().body("Blog post not found"),
    }
}

pub async fn contact(data: web::Data<AppState>) -> HttpResponse {
    let conn = data.db.lock().unwrap();
    
    let profile = db::get_profile(&conn).unwrap();
    let content = db::get_site_content(&conn).unwrap_or_default();
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("content", &content);
    context.insert("page_title", "Contact");
    context.insert("success", &false);
    
    let rendered = data.tera.render("contact.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn submit_contact(
    data: web::Data<AppState>,
    form: web::Form<ContactForm>,
) -> HttpResponse {
    let conn = data.db.lock().unwrap();
    
    let profile = db::get_profile(&conn).unwrap();
    
    // Save message to database
    let _ = db::add_message(&conn, &form);
    
    // Get email settings and send notification asynchronously
    if let Ok(email_settings) = db::get_email_settings(&conn) {
        if email_settings.enabled {
            let sender_name = form.name.clone();
            let sender_email = form.email.clone();
            let subject = form.subject.clone();
            let message_body = form.message.clone();
            
            tokio::spawn(async move {
                crate::email::send_notification_email_async(
                    email_settings,
                    sender_name,
                    sender_email,
                    subject,
                    message_body,
                ).await;
            });
        }
    }
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("page_title", "Contact");
    context.insert("success", &true);
    
    let rendered = data.tera.render("contact.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn serve_image(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let conn = data.db.lock().unwrap();
    let image_id = path.into_inner();
    
    match db::get_image(&conn, &image_id) {
        Ok((content_type, image_data)) => {
            HttpResponse::Ok()
                .content_type(content_type)
                .insert_header(("Cache-Control", "public, max-age=31536000"))
                .body(image_data)
        }
        Err(_) => HttpResponse::NotFound().body("Image not found"),
    }
}

pub async fn serve_file(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let conn = data.db.lock().unwrap();
    let file_id = path.into_inner();
    
    match db::get_image_with_filename(&conn, &file_id) {
        Ok((filename, content_type, file_data)) => {
            let disposition = format!("attachment; filename=\"{}\"", filename);
            HttpResponse::Ok()
                .content_type(content_type)
                .insert_header(("Content-Disposition", disposition))
                .body(file_data)
        }
        Err(_) => HttpResponse::NotFound().body("File not found"),
    }
}
