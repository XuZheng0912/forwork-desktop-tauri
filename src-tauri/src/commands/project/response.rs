#[derive(serde::Serialize)]
pub struct DisplayProject {
    pub name: String,
    #[serde(rename = "simpleName")]
    pub simple_name: String,
    #[serde(rename = "svnName")]
    pub svn_name: String,
}

#[derive(serde::Serialize)]
pub struct PrioritizedProject {
    pub id: i64,
    pub name: String,
    #[serde(rename = "simpleName")]
    pub simple_name: String,
    #[serde(rename = "svnName")]
    pub svn_name: String,
}
