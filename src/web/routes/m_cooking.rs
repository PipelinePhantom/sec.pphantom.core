// export the home route handler
use std::fs;

use serde_json::{json, Value};

#[tracing::instrument(level = "info")]
pub async fn core() -> String {
  let mut list :Vec<Value> = Vec::new();

  // get python builder config
  let py_config:String = fs::read_to_string("maker/python/info.json").unwrap();

  // convert to value
  list.push(serde_json::from_str(&py_config).unwrap());

  let mut str = String::new();
  let base = fs::read_to_string("html/mcooking/files/core_base.html").unwrap();
  
  for e in list {
    let mut temp = base.clone();
    temp = temp.replace("{{name}}", e["name"].as_str().unwrap());
    temp = temp.replace("{{description}}", e["description"].as_str().unwrap());
    temp = temp.replace("{{version}}", e["version"].as_str().unwrap());
    temp = temp.replace("{{cc}}", e["cc"].as_str().unwrap());
    temp = temp.replace("{{last_update}}", e["last_update"].as_str().unwrap());
    temp = temp.replace("{{source_code}}", e["source_code"].as_str().unwrap());
    temp = temp.replace("{{team_pp}}", list_to_pp_str(e["team_pp"].as_array().unwrap()).as_str());
    temp = temp.replace("{{dev}}", list_to_str(e["dev"].as_array().unwrap()).as_str());
    str.push_str(&temp);
  }


  let index = fs::read_to_string("html/mcooking/core.html").unwrap()
    .replace("{{inject_core_list}}", &str);

  return index;
}


fn list_to_str(list: &Vec<Value>) -> String {
  let mut str = String::new();
  for e in list {
    str.push_str(format!(" {},", e.as_str().unwrap()).as_str());
  }
  // remove last comma
  str.pop();
  return str;
}

fn list_to_pp_str(list: &Vec<Value>) -> String {
  let mut str = String::new();
  for e in list {
    str.push_str(format!("<img class=\"avatar sm rounded-circle\" src=\"{}\" data-bs-toggle=\"tooltip\">", e.as_str().unwrap()).as_str());
  }
  return str;
}