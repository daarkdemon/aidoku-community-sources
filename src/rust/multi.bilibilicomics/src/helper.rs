use aidoku::{
	std::{String, defaults::defaults_get}, std::ObjectRef, prelude::format,
};

pub fn get_search_url(base_url: String, title:String, tag: String, status: String, page: i32 ) -> String {
    let mut url = format!("{}/twirp/comic.v1.Comic/", base_url);
	if !title.is_empty() {
		url.push_str(&format!("Search?device=phone&platform=app&lang={}&sys_lang=en&key_word={}&page_num={}&page_size=9&style_id=-1&area_id=-1&is_finish=-1&is_free=-1&order=-1&need_shield_prefer=true&style_prefer=[]", get_lang_code(), title.replace(' ', "%20"), page))
	}
	else if !tag.is_empty() {
		url.push_str(&format!("ClassPage?device=phone&platform=app&lang={}&sys_lang=en&style_id={}&area_id=-1&is_finish=-1&order=1&is_free=-1&page_num={}&page_size=15&style_prefer=[]",get_lang_code(), tag, page));
	}
	else if !status.is_empty() && title.is_empty() {
		url.push_str(&format!("ClassPage?device=phone&platform=app&lang={}&sys_lang=en&style_id=-1&area_id=-1&is_finish={}&order=1&is_free=-1&page_num={}&page_size=15&style_prefer=[]", get_lang_code(), status, page));
	}
	url
}

pub fn data_from_json(data: ObjectRef, key: &str) -> String {
	match data.get(key).as_string() {
		Ok(str) => str.read(),
		Err(_) => String::new(),
	}
}

pub fn get_lang_code() -> String {
	let mut code = String::new();
	if let Ok(languages) = defaults_get("languages").as_array() {
		if let Ok(language) = languages.get(0).as_string() {
			code = match language.clone().read().as_str(){
                "zh" => String::from("cn"),
                _ => language.read(),
            };
		}
	}
	code
}

pub fn get_tag_id(tag: i32) -> String {
	let id = match tag {
		0 => "-1",
        1 => "17",
        2 => "15",
        3 => "19",
        4 => "22",
        5 => "3",
        6 => "11",
        7 => "12",
        8 => "13",
        9 => "14",
        10 => "16",
        11 => "20",
        12 => "21",
        13 => "23",
        14 => "30",
        15 => "41",
        _ => ""
	};
    String::from(id)
}