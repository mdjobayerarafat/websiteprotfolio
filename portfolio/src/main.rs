mod db;
mod models;
mod routes;
mod admin;
mod auth;
pub mod email;

use actix_files as fs;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{web, App, HttpServer, middleware::Logger, cookie::Key};
use std::sync::Mutex;
use tera::Tera;

pub struct AppState {
    pub db: Mutex<rusqlite::Connection>,
    pub tera: Tera,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let conn = db::init_db().expect("Failed to initialize database");
    
    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");
    
    let app_state = web::Data::new(AppState {
        db: Mutex::new(conn),
        tera,
    });

    let secret_key = Key::generate();
    
    // Get host and port from environment, defaulting to 0.0.0.0:8080 for Docker
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", host, port);

    log::info!("Starting server at http://{}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false)
                    .build()
            )
            // Static files
            .service(fs::Files::new("/static", "static").show_files_listing())
            // Public routes
            .route("/", web::get().to(routes::index))
            .route("/about", web::get().to(routes::about))
            .route("/projects", web::get().to(routes::projects))
            .route("/projects/{slug}", web::get().to(routes::project_detail))
            .route("/blogs", web::get().to(routes::blogs))
            .route("/blogs/{slug}", web::get().to(routes::blog_detail))
            .route("/contact", web::get().to(routes::contact))
            .route("/contact", web::post().to(routes::submit_contact))
            // Auth routes
            .route("/admin/login", web::get().to(auth::login_page))
            .route("/admin/login", web::post().to(auth::login))
            .route("/admin/logout", web::get().to(auth::logout))
            // Admin routes
            .route("/admin", web::get().to(admin::dashboard))
            .route("/admin/profile", web::get().to(admin::profile_page))
            .route("/admin/profile", web::post().to(admin::update_profile_with_image))
            .route("/admin/skills", web::get().to(admin::skills_page))
            .route("/admin/skills/add", web::post().to(admin::add_skill))
            .route("/admin/skills/delete/{id}", web::post().to(admin::delete_skill))
            .route("/admin/projects", web::get().to(admin::projects_page))
            .route("/admin/projects/add", web::get().to(admin::add_project_page))
            .route("/admin/projects/add", web::post().to(admin::add_project_with_image))
            .route("/admin/projects/edit/{id}", web::get().to(admin::edit_project_page))
            .route("/admin/projects/edit/{id}", web::post().to(admin::update_project_with_image))
            .route("/admin/projects/delete/{id}", web::post().to(admin::delete_project))
            .route("/admin/blogs", web::get().to(admin::blogs_page))
            .route("/admin/blogs/add", web::get().to(admin::add_blog_page))
            .route("/admin/blogs/add", web::post().to(admin::add_blog_with_image))
            .route("/admin/blogs/edit/{id}", web::get().to(admin::edit_blog_page))
            .route("/admin/blogs/edit/{id}", web::post().to(admin::update_blog_with_image))
            .route("/admin/blogs/delete/{id}", web::post().to(admin::delete_blog))
            .route("/admin/services", web::get().to(admin::services_page))
            .route("/admin/services/add", web::get().to(admin::add_service_page))
            .route("/admin/services/add", web::post().to(admin::add_service_with_image))
            .route("/admin/services/edit/{id}", web::get().to(admin::edit_service_page))
            .route("/admin/services/edit/{id}", web::post().to(admin::update_service_with_image))
            .route("/admin/services/delete/{id}", web::post().to(admin::delete_service))
            .route("/admin/messages", web::get().to(admin::messages_page))
            .route("/admin/messages/delete/{id}", web::post().to(admin::delete_message))
            // Email settings routes
            .route("/admin/email-settings", web::get().to(admin::email_settings_page))
            .route("/admin/email-settings", web::post().to(admin::update_email_settings))
            .route("/admin/email-settings/test", web::post().to(admin::test_email))
            // Site content routes
            .route("/admin/site-content", web::get().to(admin::site_content_page))
            .route("/admin/site-content", web::post().to(admin::update_site_content))
            // Education routes
            .route("/admin/education", web::get().to(admin::education_page))
            .route("/admin/education/add", web::get().to(admin::add_education_page))
            .route("/admin/education/add", web::post().to(admin::add_education))
            .route("/admin/education/edit/{id}", web::get().to(admin::edit_education_page))
            .route("/admin/education/edit/{id}", web::post().to(admin::update_education))
            .route("/admin/education/delete/{id}", web::post().to(admin::delete_education))
            // Image upload routes
            .route("/admin/upload-image", web::post().to(admin::upload_image))
            // Image serving route
            .route("/images/{id}", web::get().to(routes::serve_image))
            // File serving route (for resume downloads)
            .route("/files/{id}", web::get().to(routes::serve_file))
    })
    .bind(&bind_address)?
    .run()
    .await
}
