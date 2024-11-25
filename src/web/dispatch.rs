use actix_web::{get,web,HttpResponse,Responder};
use std::fs;
use crate::helper::{find_insert::find_insert,replace_in_body::replace_in_body};
use crate::helper::trace::{trace_logs,trace_warn};

// import the routes pages
use crate::web::routes::*;

#[get("/{path:.*}")]
#[tracing::instrument(level = "info", name = "Dispatch request", skip(path))]
pub async fn dispatch(path: web::Path<String>) -> impl Responder {
  let path_arg = path.into_inner().clone();
  let path_clone = path_arg.clone();
  let mut content_body = String::new();

  // show the request in the tracing log, with the timestamp, level, and the request path
  trace_logs(format!("Request: {}", path_arg));
  
  match path_arg.as_str() {
    "" => { content_body = home::home().await; },
  
    // default route: 404
    _ => {      
      content_body = "__404".to_string();
    }
  }

    // inject the 404 if the content is __404
  if content_body.contains("__404") {
    content_body = fs::read_to_string("html/404/index.html").unwrap();
  }

  // [START] - Pass all the injector here
  let tab_to_insert = find_insert(content_body.clone());

  // for each tab_to_insert, we will insert the content of the file
  for(tab, file) in tab_to_insert.iter().zip(tab_to_insert.iter()){
    // check if file exists
    if fs::metadata(format!("html/inject/{}.html", file)).is_ok(){
      let file_content = fs::read_to_string(format!("html/inject/{}.html", file)).unwrap();
      let inject_name = format!("inject_{}", tab.to_string());
      let replace_vec = vec![(inject_name, file_content)];
      content_body = replace_in_body(content_body.clone(), replace_vec);
    }
  }
  // [END] - Pass all the injector here

  if path_clone.starts_with("sitemap") {
    return HttpResponse::Ok().content_type("application/xml").body(content_body)
  } else if path_clone.starts_with("robots.txt") {
    return HttpResponse::Ok().content_type("text/plain").body(content_body)
  }

  return HttpResponse::Ok().content_type("text/html").body(content_body)
}