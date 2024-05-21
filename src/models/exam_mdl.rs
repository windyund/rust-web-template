use std::fmt::Debug;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExamMdl {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub desc: String,
    pub subjects: Vec<String>,
    pub grade: i32,
    pub exam_type: i32,
    pub duration: i32,
    pub ctime: i64,
    pub mtime: i64,
}