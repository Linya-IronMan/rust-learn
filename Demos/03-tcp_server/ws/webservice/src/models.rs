use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub teacher_id: usize,
    // id 可以为空，因为在post新增的时候这个字段是没有的
    pub id: Option<usize>,
    pub name: String,
    // 也是空的，日期时间类型
    pub time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course {
    // 将JSON格式的数据转化为 Course 类型
    fn from(course: web::Json<Course>) -> Self {
        Course {
            teacher_id: course.teacher_id,
            id: course.id,
            name: course.name.clone(),
            time: course.time,
        }
    }
}

