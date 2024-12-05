// export the home route handler
use std::fs;
use crate::api::init::{RequestData};
use serde_json::Value;

#[tracing::instrument(level = "info")]
pub async fn home() -> String {
  let mut list :Vec<Value> = Vec::new();

  // get python builder config
  let py_config:String = fs::read_to_string("maker/python/info.json").unwrap();

  // convert to value
  list.push(serde_json::from_str(&py_config).unwrap());

  let mut str = String::new();
  let base = fs::read_to_string("html/mcicd/files/core_base.html").unwrap();
  
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


  let index = fs::read_to_string("html/mcicd/index.html").unwrap()
    .replace("{{inject_core_list}}", &str);

  return index;
}


pub async fn build(request_data:RequestData) -> String {
    let builder_code = request_data.path.replace("/m/cicd/build/", "");

    let mut list :Vec<Value> = Vec::new();
    let mut elem = serde_json::from_str("{}").unwrap();
    let py_config:String = fs::read_to_string("maker/python/info.json").unwrap();
    list.push(serde_json::from_str(&py_config).unwrap());


    // check if one element of the list have "cc" that is equal to the builder code
    let mut found = false;
    for e in list {
      if e["cc"].as_str().unwrap() == builder_code {
        found = true;
        elem = e;
        break;
      }
    }

    if !found {
      return "__404".to_string();
    }

    // get all the builder modules list
    let mod_c2c = m_mods("C2C".to_string(), "maker/python/ModuleC2C/".to_string()).await;
    let mod_exploit = m_mods("exploit".to_string(), "maker/python/ModuleExploit/".to_string()).await;
    let mod_persistance = m_mods("persistance".to_string(), "maker/python/ModulePersistance/".to_string()).await;
    let mod_front = m_front().await;



    return fs::read_to_string("html/mcicd/build.html").unwrap()
        .replace("{{type}}", elem["name"].as_str().unwrap())
        .replace("{{inject_mod_c2c}}", &mod_c2c)
        .replace("{{inject_mod_exploit}}", &mod_exploit)
        .replace("{{inject_mod_persistance}}", &mod_persistance)
        .replace("{{inject_mod_front}}", &mod_front);
}





pub async fn m_mods(m_type:String, m_base_path:String) -> String {
    let py_mod_list = get_all_files_in_folder(m_base_path).await;

    let mut py_str = String::new();
    let base = fs::read_to_string("html/mcicd/files/build_mod.html").unwrap();

    for item in &py_mod_list {
        let content = read_ten_first_line(item.to_string());
        let name = item.split("/").collect::<Vec<&str>>()[item.split("/").collect::<Vec<&str>>().len()-1].to_string();

        let mut mod_html = base.clone();
        mod_html = mod_html.replace("{{checked}}", format!("<input type=\"radio\" name=\"{}\" value=\"{}\">", m_type.as_str(), name.as_str()).as_str());
        mod_html = mod_html.replace("{{file_name}}", name.as_str());
        mod_html = mod_html.replace("{{name}}", get_parameter(content.clone(), "name".to_string()).as_str());
        mod_html = mod_html.replace("{{description}}", get_parameter(content.clone(), "description".to_string()).as_str());
        mod_html = mod_html.replace("{{time}}", get_parameter(content.clone(), "time".to_string()).as_str());
        mod_html = mod_html.replace("{{author}}", get_parameter(content.clone(), "author".to_string()).as_str());
        mod_html = mod_html.replace("{{status}}", &get_status(get_parameter(content.clone(), "status".to_string())));

        py_str.push_str(mod_html.as_str());
    }

    return py_str;
}


pub async fn m_front() -> String {
  let front_mod_list = get_all_dir_in_folder("maker/python/ModuleFront".to_string()).await;

  let mut py_str = String::new();
  let base = fs::read_to_string("html/mcicd/files/build_mod.html").unwrap();


  for item in &front_mod_list {
      let content = read_front_info_files(item.to_string());
      let name = item.split("/").collect::<Vec<&str>>()[item.split("/").collect::<Vec<&str>>().len()-1].to_string();

      let mut mod_html = base.clone();
      mod_html = mod_html.replace("{{checked}}", format!("<input type=\"radio\" name=\"FRONT\" value=\"{}\">", name.as_str()).as_str());
      mod_html = mod_html.replace("{{file_name}}", name.as_str());
      mod_html = mod_html.replace("{{name}}", get_parameter(content.clone(), "name".to_string()).as_str());
      mod_html = mod_html.replace("{{description}}", get_parameter(content.clone(), "description".to_string()).as_str());
      mod_html = mod_html.replace("{{time}}", get_parameter(content.clone(), "time".to_string()).as_str());
      mod_html = mod_html.replace("{{author}}", get_parameter(content.clone(), "author".to_string()).as_str());
      mod_html = mod_html.replace("{{status}}", &get_status(get_parameter(content.clone(), "status".to_string())));

      py_str.push_str(mod_html.as_str());
  }

  return py_str;
}



fn get_status(name:String) -> String {
    println!("{}", name);
    match name.as_str() {
        "working" => return "<td><span class=\"badge bg-info\">working</span></td>".to_string(),
        "development" => return "<td><span class=\"badge bg-warning\">development</span></td>".to_string(),
        _ => return "<td><span class=\"badge bg-warning\">unknow</span></td>".to_string(),
    }
}

fn get_parameter(text:String, finder:String) -> String {
    if text.contains(finder.as_str()) {
        let split = text.split(format!("{}:", finder.as_str()).as_str()).collect::<Vec<&str>>();
        let split = split[1].split("\n").collect::<Vec<&str>>();
        return split[0].to_string().trim().to_string();
    }
    return "unknow".to_string();
}

async fn get_all_files_in_folder(path:String) -> Vec<String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            files.push(path.to_str().unwrap().to_string());
        }
    }
    return files;
}

async fn get_all_dir_in_folder(path:String) -> Vec<String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            files.push(path.to_str().unwrap().to_string());
        }
    }
    return files;
}

fn read_ten_first_line(file_path:String) -> String {
    let file = fs::read_to_string(file_path).unwrap();
    let split = file.split("\n").collect::<Vec<&str>>();
    let mut result = String::new();

    let max_lines = if split.len() > 10 { 10 } else { split.len() };

    for i in 0..max_lines {
        result.push_str(split[i]);
        result.push_str("\n");
    }
    return result;
}

fn read_front_info_files(file_path:String) -> String {
    // read filepath/readme.md
    return read_ten_first_line(format!("{}/readme.md", file_path));
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