use actix_web::{get,web,HttpResponse,HttpRequest,Responder};
use std::fs;
use crate::helper::{find_insert::find_insert,replace_in_body::replace_in_body};
use crate::helper::trace::{trace_logs,trace_warn};
use crate::api::init::{RequestData, log_request};
use crate::helper::database::USERS;

// import the routes pages
use crate::web::routes::*;

#[get("/{path:.*}")]
#[tracing::instrument(level = "info", name = "Dispatch request", skip(path, req))]
pub async fn dispatch(path: web::Path<String>, req: HttpRequest) -> impl Responder {

  let request_data = log_request(path, req.clone(), "GET", None).await;
  let mut content_body = String::new();

  println!("Request data: {:?}", request_data);

  match request_data.path.as_str() {
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


  return HttpResponse::Ok().content_type("text/html").body(content_body)
}