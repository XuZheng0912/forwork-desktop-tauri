use crate::commands::project::response::DisplayProject;
use crate::commands::Response;

mod response;

#[tauri::command]
pub fn get_all_projects() -> Response<Vec<DisplayProject>> {
    Response::success(vec![DisplayProject {
        name: "中能建风光水火储一体化".to_string(),
        simple_name: "中能建".to_string(),
        svn_name: "LiEMS80_ChongZuoXinNengYuan".to_string(),
    }, DisplayProject {
        name: "中能建风光水火储一体化".to_string(),
        simple_name: "中能建".to_string(),
        svn_name: "LiEMS80_ChongZuoXinNengYuan".to_string(),
    }])
}
