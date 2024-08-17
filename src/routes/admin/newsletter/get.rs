// Handler that returns a form to publish a new newsletter.

use actix_web::http::header::ContentType;
use actix_web::HttpResponse;

use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};

/// Returns an HTML page with a form to submit a new issue of a newsletter.
///
/// REQUIRES:
///  - valid user session
pub async fn newsletter_form(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"));
    }

    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Change Password</title>
</head>
<body>
 {msg_html}
<form action="/admin/newsletter" method="post">
        <label>Title
            <input
                type="text"
                placeholder="Untitled"
                name="title"
            >
        </label>
        <br>
        <label>Content
            <input
                type="text"
                placeholder="Content..."
                name="content"
            >
        </label>
        <br>
        <br>
        <button type="submit">Change password</button>
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>"#,
    ))
}
