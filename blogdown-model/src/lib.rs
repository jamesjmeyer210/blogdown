pub struct Group 
{
    pub id: u32,
    pub name: String,
}

impl From<&str> for Group {
    fn from(value: &str) -> Self {
        Group {
            id: 0,
            name: value.to_string()
        }
    }
}

pub struct User<'a>
{
    pub id: u32,
    pub name: String,
    pub email: String,
    pub groups: Vec<Group>,
    pub blogs: Vec<Blog<'a>>,
    pub comments: Vec<Comment<'a>>,
}

pub struct Comment<'a> 
{
    pub id: u32,
    pub content: String,
    pub reply_to: Option<&'a Comment<'a>>,
}

pub struct Blog<'a>
{
    pub id: u32,
    pub title: String,
    pub content: String,
    pub comments: Vec<Comment<'a>>,
}
