#[derive(Queryable)]
pub struct Link {
    pub id: i32,
    pub src: String,
    pub dst: String,
}

use super::schema::links;

#[derive(Insertable)]
#[table_name="links"]
pub struct NewLink<'a> {
    pub src: &'a str,
    pub dst: &'a str,
}
