use crate::commands::response::{failure, success, Response};
use crate::core::domain::project::Project;
use crate::persistence::save::Savable;

#[tauri::command]
pub fn get_all_projects() {
    // match Project::query_all() {
    //     Ok(projects) => {
    //         let mut data: Vec<PrioritizedProject> = Vec::new();
    //         for project in projects {
    //             data.push(PrioritizedProject::from(project));
    //         }
    //         Response::success(data)
    //     }
    //     Err(error) => Response::failure(error.to_string().as_str()),
    // }
}

#[tauri::command]
pub fn save_project(project: Project) -> Response<()> {
    match project.save() {
        Ok(_) => success(()),
        Err(e) => failure(e.to_string()),
    }
}