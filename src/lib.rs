pub struct Issue {
    pub id: Option<String>,
    pub key: String,
    pub summary: String,
    pub description: String,
    pub status: IssueStatus,
}

pub struct IssueStatus {
    pub id: Option<String>,
    pub name: String,
    // url
}

// TODO: Add self-link
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
    // TODO: Add self_link/url
    // self_link: URL?
}

pub struct IssueType {
    pub id: String,
    pub name: String,
    pub description: String,
    pub subtask: bool,
}
