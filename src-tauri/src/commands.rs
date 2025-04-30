mod project;

pub use project::get_all_projects;

#[derive(serde::Serialize)]
pub enum Result {
    Success,
    Failure,
}

#[derive(serde::Serialize)]
pub struct Response<T : serde::Serialize> {
    result: Result,
    data: T,
}

impl<T : serde::Serialize> Response<T> {
    fn success(data: T) -> Response<T> {
        Response {
            result: Result::Success,
            data,
        }
    }

    fn failure(message: String) -> Response<String> {
        Response {
            result: Result::Failure,
            data: message,
        }
    }
}
