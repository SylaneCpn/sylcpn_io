/*Handle get requests that respond with content of the files of the server, require auth */

//###########################################################################################//
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::auth::{check_user, verify_access};
use crate::reader::{respond_main_dir, respond_or_fallback};

//###########################################################################################//

//provide the server's files and directories  for the web. require auth
pub async fn files(Path((user, password, path)): Path<(String, String, String)>) -> Response {
    let complete_path = format!("files/{}", &path);

    //check if user is in the database
    let u = check_user(&user, &password).await;
    //verify if the user has access to the source
    if verify_access(&u, &path) {
        //respond if access granted
        respond_or_fallback(&complete_path).await
    } else {
        //respond to unauthorised user
        (StatusCode::UNAUTHORIZED, format!("Not Authorized\n")).into_response()
    }
}

//send directory or
pub async fn main_repo(Path((user, password)): Path<(String, String)>) -> Response {
    let u = check_user(&user, &password).await;
    if let Ok(r) = respond_main_dir(&u).await {
        r
    } else {
        (StatusCode::NOT_FOUND, format!("Something went wrong\n")).into_response()
    }
}
