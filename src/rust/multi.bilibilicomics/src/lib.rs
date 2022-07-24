#![no_std]

use aidoku::{
	error::Result,
	prelude::*,
	std::{String, Vec, net::Request},
	Chapter, Filter, Listing, Manga, MangaPageResult, Page
};

pub mod helper;
pub mod parser;
use parser::*;

const BASE_URL: &str = "https://www.bilibilicomics.com";

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	parse_manga_list(String::from(BASE_URL), filters, page)
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	parse_manga_listing(String::from(BASE_URL), listing.name, page)
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	parser::parse_manga_details(String::from(BASE_URL), id)
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
	parser::parse_chapter_list(id)
}

#[get_page_list]
fn get_page_list(id: String) -> Result<Vec<Page>> {
	parser::parse_page_list(String::from(BASE_URL), id)
}

#[modify_image_request]
fn modify_image_request(request: Request) {
	parser::modify_image_request(String::from(BASE_URL), request)
}
