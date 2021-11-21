use actix_web::Result;

use super::Invalid;

pub struct PostContent<TypeStr = String> {
    pub post_type: TypeStr,
    pub content: String,
}

impl PostContent<&'static str> {
    pub fn from_model(model: crate::common::PostContent) -> PostContent<&'static str> {
        match model {
            crate::common::PostContent::Post(post) => PostContent {
                post_type: "post",
                content: post,
            },
            crate::common::PostContent::Url(url) => PostContent {
                post_type: "url",
                content: url.to_string(),
            },
        }
    }
}

impl PostContent {
    pub fn try_into_model(self) -> Result<crate::common::PostContent, Invalid<String>> {
        match &*self.post_type {
            "post" => Ok(crate::common::PostContent::Post(self.content)),
            "url" => crate::common::PostContent::parse_url(self.content),
            _ => Err(Invalid::new(self.post_type, "INVALID_POST_TYPE")),
        }
    }
}
