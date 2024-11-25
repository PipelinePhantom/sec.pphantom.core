use actix_web::{web, Scope,get, HttpResponse, HttpRequest, Responder};
use serde_json::Value;
use crate::helper::trace::{trace_logs,trace_warn};
use crate::helper::cookie::auth_cookie;
use crate::helper::database::USERS;


#[derive(Debug, Clone)]
pub struct RequestData {
    // Request basic information
    pub path: String,
    pub user_ip: String,
    pub method: String,
    
    // All stuff related to authentication
    pub user_data: USERS,
    pub user_logged: bool,
}

#[derive(Debug, Clone)]
pub struct Key {
    pub is_authenticated: bool,
    pub key: String,
    pub user_detail: USERS,
}


#[get("/{path:.*}")]
pub async fn handler(path: web::Path<String>) -> impl Responder {

    trace_logs(format!("api: {}", path.to_string()));

    match path.to_string().as_str() {
        "" => {
            return HttpResponse::Ok().content_type("application/json").body("{\"status\": \"OK\"}");
        }

        _ => {
            trace_warn(format!("404 Not Found: {}", path.to_string()));
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": \"path not found\"}");
        }
    }
}



pub async fn log_request(path: web::Path<String>, req: HttpRequest, method:&str, payload: Option<web::Payload>) -> RequestData {
    // get user IP address
    let connection_info = req.connection_info().clone();  // Bind the result of connection_info
    let user_ip = connection_info.realip_remote_addr();    // Use connection_info to get peer_addr
    let user_ip: String = match user_ip {
        Some(ip) => ip.to_string(),
        None => match connection_info.peer_addr() {
              Some(ip) => ip.to_string(),
              None => "unknown".to_string()
          }
    };

    trace_logs(format!("Request: {} - {}", method, path.to_string()));

    let mut user_data: USERS = USERS::default();
    let mut user_logged = false;

    let req_auth_cookie = auth_cookie(req.clone());
    if req_auth_cookie != "none" {
        let u = USERS::get_user_by_cookie(req_auth_cookie.clone()).await;
        if u.len() > 0 {
            user_data = u[0].clone();
            user_logged = true;
        }
    }

    RequestData {
        path: format!("/{}", path.to_string()),
        method: method.to_string(),
        user_ip: user_ip,
        user_data: user_data,
        user_logged: user_logged,
    }
}


pub fn init_api() -> Scope {
    web::scope("/api").service(handler)
}
