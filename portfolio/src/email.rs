use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use crate::models::EmailSettings;

pub fn send_notification_email(
    settings: &EmailSettings,
    sender_name: &str,
    sender_email: &str,
    subject: &str,
    message_body: &str,
) -> Result<(), String> {
    if !settings.enabled {
        return Err("Email notifications are disabled".to_string());
    }
    
    if settings.smtp_username.is_empty() || settings.smtp_password.is_empty() {
        return Err("Email settings not configured".to_string());
    }
    
    let email_subject = format!("New Contact Form Message: {}", subject);
    
    let email_body = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <style>
        body {{ font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background-color: #0a0a14; color: #f3f4f6; padding: 20px; }}
        .container {{ max-width: 600px; margin: 0 auto; background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%); border-radius: 16px; padding: 30px; border: 1px solid #374151; }}
        .header {{ text-align: center; margin-bottom: 30px; }}
        .header h1 {{ color: #f97316; margin: 0; font-size: 24px; }}
        .header p {{ color: #9ca3af; margin-top: 8px; }}
        .info-box {{ background: rgba(249, 115, 22, 0.1); border: 1px solid rgba(249, 115, 22, 0.3); border-radius: 12px; padding: 20px; margin: 20px 0; }}
        .info-row {{ display: flex; margin-bottom: 12px; }}
        .info-label {{ color: #f97316; font-weight: 600; min-width: 100px; }}
        .info-value {{ color: #f3f4f6; }}
        .message-box {{ background: rgba(255, 255, 255, 0.05); border-radius: 12px; padding: 20px; margin-top: 20px; }}
        .message-box h3 {{ color: #f97316; margin-top: 0; }}
        .message-content {{ color: #d1d5db; line-height: 1.6; white-space: pre-wrap; }}
        .footer {{ text-align: center; margin-top: 30px; padding-top: 20px; border-top: 1px solid #374151; color: #6b7280; font-size: 12px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>📬 New Contact Message</h1>
            <p>Someone reached out through your portfolio!</p>
        </div>
        
        <div class="info-box">
            <div class="info-row">
                <span class="info-label">From:</span>
                <span class="info-value">{}</span>
            </div>
            <div class="info-row">
                <span class="info-label">Email:</span>
                <span class="info-value"><a href="mailto:{}" style="color: #60a5fa;">{}</a></span>
            </div>
            <div class="info-row">
                <span class="info-label">Subject:</span>
                <span class="info-value">{}</span>
            </div>
        </div>
        
        <div class="message-box">
            <h3>Message</h3>
            <div class="message-content">{}</div>
        </div>
        
        <div class="footer">
            <p>This notification was sent from your Portfolio website.</p>
            <p>Reply directly to the sender's email address above.</p>
        </div>
    </div>
</body>
</html>"#,
        sender_name,
        sender_email, sender_email,
        subject,
        message_body
    );

    let email = Message::builder()
        .from(settings.smtp_username.parse().map_err(|e| format!("Invalid from address: {}", e))?)
        .reply_to(sender_email.parse().map_err(|e| format!("Invalid reply-to address: {}", e))?)
        .to(settings.notification_email.parse().map_err(|e| format!("Invalid to address: {}", e))?)
        .subject(email_subject)
        .header(ContentType::TEXT_HTML)
        .body(email_body)
        .map_err(|e| format!("Failed to build email: {}", e))?;

    let creds = Credentials::new(
        settings.smtp_username.clone(),
        settings.smtp_password.clone(),
    );

    let mailer = SmtpTransport::starttls_relay(&settings.smtp_server)
        .map_err(|e| format!("Failed to create SMTP transport: {}", e))?
        .port(settings.smtp_port as u16)
        .credentials(creds)
        .build();

    mailer.send(&email)
        .map_err(|e| format!("Failed to send email: {}", e))?;

    Ok(())
}

// Async version for use in actix-web handlers
pub async fn send_notification_email_async(
    settings: EmailSettings,
    sender_name: String,
    sender_email: String,
    subject: String,
    message_body: String,
) {
    tokio::task::spawn_blocking(move || {
        match send_notification_email(&settings, &sender_name, &sender_email, &subject, &message_body) {
            Ok(_) => log::info!("Email notification sent successfully to {}", settings.notification_email),
            Err(e) => log::warn!("Email notification not sent: {}", e),
        }
    });
}
