// export the home route handler
use std::fs;

pub async fn m_mods(m_type:String, m_base_path:String) -> String {
    let py_mod_list = get_all_files_in_folder(m_base_path).await;
    println!("{:?}", py_mod_list);

    let mut py_str = String::new();
    let base = fs::read_to_string("html/mmods/files/mods.html").unwrap();

    for item in &py_mod_list {
        let content = read_ten_first_line(item.to_string());
        let name = item.split("/").collect::<Vec<&str>>()[item.split("/").collect::<Vec<&str>>().len()-1].to_string();

        let mut mod_html = base.clone();
        mod_html = mod_html.replace("{{file_name}}", name.as_str());
        mod_html = mod_html.replace("{{name}}", get_parameter(content.clone(), "name".to_string()).as_str());
        mod_html = mod_html.replace("{{description}}", get_parameter(content.clone(), "description".to_string()).as_str());
        mod_html = mod_html.replace("{{time}}", get_parameter(content.clone(), "time".to_string()).as_str());
        mod_html = mod_html.replace("{{author}}", get_parameter(content.clone(), "author".to_string()).as_str());
        mod_html = mod_html.replace("{{status}}", &get_status(get_parameter(content.clone(), "status".to_string())));

        py_str.push_str(mod_html.as_str());
    }

    return fs::read_to_string("html/mmods/index.html").unwrap()
        .replace("{{type}}", m_type.as_str())
        .replace("{{py_table}}", py_str.as_str());
}

pub async fn m_front() -> String {
    let front_mod_list = get_all_dir_in_folder("maker/python/ModuleFront".to_string()).await;
    println!("{:?}", front_mod_list);

    let mut py_str = String::new();
    let base = fs::read_to_string("html/mmods/files/mods.html").unwrap();


    for item in &front_mod_list {
        let content = read_front_info_files(item.to_string());
        let name = item.split("/").collect::<Vec<&str>>()[item.split("/").collect::<Vec<&str>>().len()-1].to_string();

        let mut mod_html = base.clone();
        mod_html = mod_html.replace("{{file_name}}", name.as_str());
        mod_html = mod_html.replace("{{name}}", get_parameter(content.clone(), "name".to_string()).as_str());
        mod_html = mod_html.replace("{{description}}", get_parameter(content.clone(), "description".to_string()).as_str());
        mod_html = mod_html.replace("{{time}}", get_parameter(content.clone(), "time".to_string()).as_str());
        mod_html = mod_html.replace("{{author}}", get_parameter(content.clone(), "author".to_string()).as_str());
        mod_html = mod_html.replace("{{status}}", &get_status(get_parameter(content.clone(), "status".to_string())));

        py_str.push_str(mod_html.as_str());
    }

    return fs::read_to_string("html/mmods/index.html").unwrap()
        .replace("{{type}}", "front")
        .replace("{{py_table}}", py_str.as_str());
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