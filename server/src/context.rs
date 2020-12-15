use actix_web::error::ErrorInternalServerError;
use actix_web::Error;
use askama::Template;
use rustgym_schema::leetcode_description::LeetcodeDescription;
use rustgym_schema::leetcode_question::LeetcodeQuestion;

#[derive(Template)]
#[template(path = "home.j2")]
pub struct HomeContext<'a> {
    pub title: &'a str,
    pub tag: &'a str,
}

#[derive(Queryable)]
pub struct LeetcodeIndexRow {
    pub frontend_id: i32,
    pub title: String,
}

#[derive(Template)]
#[template(path = "leetcode-index.j2")]
pub struct LeetcodeIndexContext {
    pub rows: Vec<LeetcodeIndexRow>,
}

#[derive(Template)]
#[template(path = "leetcode-detail.j2")]
pub struct LeetcodeDetailContext {
    pub question: LeetcodeQuestion,
    pub description: LeetcodeDescription,
}

macro_rules! impl_render_wrapper {
    ($type: ty) => {
        impl $type {
            pub fn render_wrapper(&self) -> Result<String, Error> {
                self.render().map_err(ErrorInternalServerError)
            }
        }
    };
}

impl_render_wrapper!(HomeContext<'_>);
impl_render_wrapper!(LeetcodeIndexContext);
impl_render_wrapper!(LeetcodeDetailContext);
