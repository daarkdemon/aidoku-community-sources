use aidoku::{
	error::Result,
	prelude::{format, println},
	std::net::HttpMethod,
	std::net::Request,
	std::String,
	std::Vec,
	Chapter, Filter, FilterType, Manga, MangaContentRating, MangaPageResult, MangaStatus,
	MangaViewer, Page,
};
use alloc::string::ToString;
extern crate alloc;
use crate::helper::*;

pub fn parse_manga_list(
	base_url: String,
	filters: Vec<Filter>,
	page: i32,
) -> Result<MangaPageResult> {
	let mut tag: String = String::new();
	let mut title: String = String::new();
	let mut status: String = String::new();
	let status_options = ["", "0", "1"];
	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				title = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				if filter.name.as_str() == "Genres" {
					let index = filter.value.as_int()? as usize;
					match index {
						0 => continue,
						_ => tag = get_tag_id(index as i32),
					}
				} else if filter.name.as_str() == "Status" {
					let index = filter.value.as_int().unwrap_or(-1) as usize;
					status = String::from(status_options[index])
				}
			}
			_ => continue,
		};
	}
	let url = if title.is_empty() && tag.is_empty() && status.is_empty() {
		format!("{}/twirp/comic.v1.Comic/ClassPage?device=phone&platform=app&lang={}&sys_lang=en&style_id=-1&area_id=-1&is_finish=-1&order=1&is_free=-1&page_num={}&page_size=15&style_prefer=[]", base_url, get_lang_code(), page)
	} 
	else {
		get_search_url(base_url, title, tag, status, page)
	};
	if url.contains("Search") {
		let mut mangas: Vec<Manga> = Vec::new();
	let raw_json = Request::new(&url, HttpMethod::Post).json().as_object()?;
	let json = raw_json.get("data").as_object()?;
	let arr = json.get("list").as_array()?;
	for data in arr {
		if let Ok(manga_obj) = data.as_object() {
			let title = manga_obj.get("title").as_string()?.read().replace(r#"<em class="keyword">"#,"").replace("</em>","");
			let id = manga_obj
				.get("id")
				.as_int()
				.unwrap_or(-1)
				.to_string();
			let cover = format!(
				"{}@512w.jpg",
				manga_obj.get("vertical_cover").as_string()?.read()
			);
			mangas.push(Manga {
				id,
				cover,
				title,
				author: String::new(),
				artist: String::new(),
				description: String::new(),
				url: String::new(),
				categories: Vec::new(),
				status: MangaStatus::Unknown,
				nsfw: MangaContentRating::Safe,
				viewer: MangaViewer::Rtl,
			});
		}
	}
	Ok(MangaPageResult {
		manga: mangas,
		has_more: true,
	})
	}else{
		parse_manga_listing(url, String::from(""), page)
	}
	
}

pub fn parse_manga_listing(
	base_url: String,
	list_type: String,
	page: i32,
) -> Result<MangaPageResult> {
	let mut url = match list_type.as_str() {
        "Popular" => format!("{}/twirp/comic.v1.Comic/ClassPage?platform=ios&device=phone&lang={}&style_id=-1&area_id=-1&is_finish=-1&order=1&page_num={}&page_size=18&is_free=-1&style_prefer=[]",base_url, get_lang_code(), page),
        "Latest" => format!("{}/twirp/comic.v1.Comic/ClassPage?platform=ios&device=phone&lang={}&style_id=-1&area_id=-1&is_finish=-1&order=2&page_num={}&page_size=18&is_free=-1&style_prefer=[]", base_url,get_lang_code(), page),
        _ => format!("{}/twirp/comic.v1.Comic/ClassPage?platform=ios&device=phone&lang={}&sys_lang=en&style_id=-1&area_id=-1&is_finish=-1&order=1&is_free=-1&page_num={}&page_size=15&style_prefer=[]", base_url, get_lang_code(), page),
    };
	if base_url.contains("twirp") {
		url = base_url;
	}
	println!("{}", url);
	let mut mangas: Vec<Manga> = Vec::new();
	let raw_json = Request::new(&url, HttpMethod::Post).json().as_object()?;
	let json = raw_json.get("data").as_array()?;
	for data in json {
		if let Ok(manga_obj) = data.as_object() {
			let title = manga_obj.get("title").as_string()?.read();
			let id = manga_obj
				.get("season_id")
				.as_int()
				.unwrap_or(-1)
				.to_string();
			let cover = format!(
				"{}@512w.jpg",
				manga_obj.get("vertical_cover").as_string()?.read()
			);
			mangas.push(Manga {
				id,
				cover,
				title,
				author: String::new(),
				artist: String::new(),
				description: String::new(),
				url: String::new(),
				categories: Vec::new(),
				status: MangaStatus::Unknown,
				nsfw: MangaContentRating::Safe,
				viewer: MangaViewer::Rtl,
			});
		}
	}
	Ok(MangaPageResult {
		manga: mangas,
		has_more: true,
	})
}

pub fn parse_manga_details(base_url: String, id: String) -> Result<Manga> {
	let url = format!(
		"{}/twirp/comic.v1.Comic/ComicDetail?platform=ios&device=phone&lang={}&comic_id={}",
		base_url,
		get_lang_code(),
		id
	);
	let json = Request::new(&url, HttpMethod::Post).json().as_object()?;
	let data = json.get("data").as_object()?;
	let title = data_from_json(data.clone(), "title");
	let cover = format!(
		"{}@512w.jpg",
		data_from_json(data.clone(), "vertical_cover")
	);
	let author = match data.get("author_name").as_array() {
		Ok(str) => match str.get(0).as_string() {
			Ok(str) => str.read(),
			Err(_) => String::new(),
		},
		Err(_) => String::new(),
	};
	let description = data_from_json(data.clone(), "evaluate");
	let status = match data.get("is_finish").as_int().unwrap_or(0) {
		1 => MangaStatus::Completed,
		_ => MangaStatus::Ongoing,
	};
	let genres = data.get("styles2").as_array()?;
	let categories = genres
		.map(|genre| {
			let genre_obj = genre.as_object()?;
			Ok(genre_obj.get("name").as_string()?.read())
		})
		.map(|a: Result<String>| a.unwrap_or_default())
		.collect::<Vec<String>>();
	let viewer = match data.get("japan_comic").as_bool()? {
		true => MangaViewer::Rtl,
		_ => MangaViewer::Scroll,
	};
	Ok(Manga {
		id: url.replace("platform=ios&device=phone", "device=android&platform=app"),
		cover,
		title,
		author,
		artist: String::new(),
		description,
		url: format!("{}/detail/mc{}", base_url, id),
		categories,
		status,
		nsfw: MangaContentRating::Safe,
		viewer,
	})
}

pub fn parse_chapter_list(id: String) -> Result<Vec<Chapter>> {
	let mut chapters: Vec<Chapter> = Vec::new();
	let json = Request::new(&id, HttpMethod::Post).json().as_object()?;
	let data = json.get("data").as_object()?;
	let mchapters = data.get("ep_list").as_array()?;
	for chapter in mchapters {
		let chapter_obj = chapter.as_object()?;
		let paid = chapter_obj.get("pay_mode").as_int().unwrap_or(-1);
		let title = match paid {
			1 => format!("[BUY] {}", data_from_json(chapter_obj.clone(), "title")),
			_ => data_from_json(chapter_obj.clone(), "title"),
		};
		let id = chapter_obj.get("id").as_int().unwrap_or(-1).to_string();
		let chapter = chapter_obj.get("ord").as_float().unwrap_or(0.0) as f32;
		let date_updated = chapter_obj
			.get("pub_time")
			.as_date("yyyy-MM-dd'T'HH:mm:ssZ", Some("en_US"), None)
			.unwrap_or(-1.0);
		chapters.push(Chapter {
			id,
			title,
			volume: -1.0,
			chapter,
			date_updated,
			scanlator: String::new(),
			url: String::new(),
			lang: get_lang_code(),
		});
	}
	Ok(chapters)
}

pub fn parse_page_list(base_url: String, id: String) -> Result<Vec<Page>> {
	let mut pages: Vec<Page> = Vec::new();
	let request = Request::new(
		&format!(
			"{}/twirp/comic.v1.Comic/GetImageIndex?device=android&platform=app&lang={}",
			base_url,
			get_lang_code()
		),
		HttpMethod::Post,
	);
	let body_data = format!("ep_id={}", id);
	let json = request
		.header("Content-Type", "application/x-www-form-urlencoded")
		.body(body_data.as_bytes())
		.json()
		.as_object()?;
	let paid = data_from_json(json.clone(), "msg");
	if !paid.is_empty(){
		let page_url = String::from("https://placehold.jp/16/050000/ffffff/390x800.png?text=PAID%20CHAPTER%0ATo%20read%20the%20paid%20chapter%2C%20please%20utilize%20the%20Bilibilicomics%20app%20as%20this%20source%20does%20not%20handle%20app%20login%20yet.");
		pages.push(Page {
			index: 1,
			url: page_url,
			base64: String::new(),
			text: String::new(),
		});
	}else{
		let data = json.get("data").as_object()?;
		if let Ok(images) = data.get("images").as_array() {
			for (at, image) in images.enumerate() {
				//if paid.is_empty(){ continue; }
				if let Ok(image_obj) = image.as_object() {
					let path = format!(
						"{}@{}w.jpg",
						image_obj.get("path").as_string()?.read(),
						image_obj.get("x").as_int().unwrap_or(-1)
					);
					let request = Request::new(
						&format!(
							"{}/twirp/comic.v1.Comic/ImageToken?platform=ios&device=phone&lang={}",
							base_url,
							get_lang_code()
						),
						HttpMethod::Post,
					);
					let body_data = format!("urls={}", format!(r#"["{}"]"#, path));
					let image_json = request
						.header("Content-Type", "application/x-www-form-urlencoded")
						.body(body_data.as_bytes())
						.json()
						.as_object()?;
					let images_arr = image_json.get("data").as_array()?;
					let token_object = images_arr.get(0).as_object()?;
					let page_url = format!(
						"{}?token={}",
						token_object.get("url").as_string()?.read(),
						token_object.get("token").as_string()?.read()
					);
					pages.push(Page {
						index: at as i32,
						url: page_url,
						base64: String::new(),
						text: String::new(),
					});
				}
			}
		}
	}
	Ok(pages)
}

pub fn modify_image_request(base_url: String, request: Request) {
	request.header("Referer", &base_url);
}