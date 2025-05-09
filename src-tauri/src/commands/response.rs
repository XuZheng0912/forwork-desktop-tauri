use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum Response<T> {
    Success { result: String, data: T },
    Failure { result: String, message: String },
}

impl<T> Response<T> {
    pub fn success(data: T) -> Response<T> {
        Response::Success {
            result: "success".to_string(),
            data,
        }
    }

    pub fn failure(message: String) -> Response<T> {
        Response::Failure {
            result: "failure".to_string(),
            message,
        }
    }
}

pub fn success<T>(data: T) -> Response<T> {
    Response::success(data)
}

pub fn failure<T>(message: String) -> Response<T> {
    Response::failure(message)
}
