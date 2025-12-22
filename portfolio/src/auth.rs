use actix_web::{web, HttpResponse};
use actix_session::Session;
use bcrypt::verify;
use crate::AppState;
use crate::db;
use crate::models::LoginForm;

pub async fn login_page(data: web::Data<AppState>, session: Session) -> HttpResponse {
    // If already logged in, redirect to admin
    if let Ok(Some(_)) = session.get::<String>("admin") {
        return HttpResponse::Found()
            .append_header(("Location", "/admin"))
            .finish();
    }
    
    let conn = data.db.lock().unwrap();
    let profile = db::get_profile(&conn).unwrap();
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("page_title", "Admin Login");
    context.insert("error", &false);
    
    let rendered = data.tera.render("admin/login.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn login(
    data: web::Data<AppState>,
    form: web::Form<LoginForm>,
    session: Session,
) -> HttpResponse {
    let conn = data.db.lock().unwrap();
    let profile = db::get_profile(&conn).unwrap();
    
    match db::get_admin(&conn, &form.username) {
        Ok(admin) => {
            if verify(&form.password, &admin.password_hash).unwrap_or(false) {
                session.insert("admin", &admin.username).unwrap();
                return HttpResponse::Found()
                    .append_header(("Location", "/admin"))
                    .finish();
            }
        }
        Err(_) => {}
    }
    
    let mut context = tera::Context::new();
    context.insert("profile", &profile);
    context.insert("page_title", "Admin Login");
    context.insert("error", &true);
    
    let rendered = data.tera.render("admin/login.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn logout(session: Session) -> HttpResponse {
    session.purge();
    HttpResponse::Found()
        .append_header(("Location", "/admin/login"))
        .finish()
}

pub fn is_authenticated(session: &Session) -> bool {
    session.get::<String>("admin").unwrap_or(None).is_some()
}
