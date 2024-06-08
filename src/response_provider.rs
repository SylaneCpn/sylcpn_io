use axum::{
    http::header,
    response::{IntoResponse, Response},
    Json,
};
use tokio::{fs, io};

use serde::Serialize;
//###########################################################################################//

//internal file structure to send the contents of a directory as a json to the network
#[derive(Serialize)]
struct File {
    name: String,
    content_type: String,
    full_path: String,
}

//###########################################################################################//

/*read the file or the directory and provide a response for the network
it can be :
    -the content of the file ,as a string or a byte stream depending of the extention of the file
    - a json of the content of the directory structured as the File struct
*/
pub async fn respond(path: &str) -> io::Result<Response> {
    let md = fs::metadata(path).await?;
    //is a dir ?
    if md.is_dir() {
        respond_dir(path).await
    }
    //assume file
    else {
        respond_file(path).await
    }
}

//###########################################################################################//

//internal functions

//provide a response for a file
async fn respond_file(path: &str) -> io::Result<Response> {
    //try to read path
    let contents = fs::read(path).await?;
    //try to find extention
    if let Some(idx) = path.rfind('.') {
        let content_ext = &path[(idx + 1)..];
        let content_type: String;
        match content_ext {
            //simple text
            "txt" => {
                content_type = "text/plain".to_string();
                return Ok((
                    [(header::CONTENT_TYPE, &content_type)],
                    String::from_utf8_lossy(&contents).to_string(),
                )
                    .into_response());
            }

            //other text format
            "html" | "css" | "js" | "rs" => {
                content_type = format!("text/{}", content_ext);
                return Ok((
                    [(header::CONTENT_TYPE, &content_type)],
                    String::from_utf8_lossy(&contents).to_string(),
                )
                    .into_response());
            }

            //image formats
            "png" | "jpg" | "svg" | "webp" | "gif" => {
                content_type = format!("image/{}", content_ext);
                return Ok(([(header::CONTENT_TYPE, &content_type)], contents).into_response());
            }

            //pdf
            "pdf" => {
                content_type = format!("application/{}", content_ext);
                return Ok(([(header::CONTENT_TYPE, &content_type)], contents).into_response());
            }

            //video formats
            "mp4" | "webm" => {
                content_type = format!("video/{}", content_ext);
                return Ok(([(header::CONTENT_TYPE, &content_type)], contents).into_response());
            }
            //unkwown file extention
            _ => {
                return Ok(contents.into_response());
            }
        }
    }
    //no file extention
    else {
        Ok(contents.into_response())
    }
}

//provide a response for a directory
async fn respond_dir(path: &str) -> io::Result<Response> {
    let mut entries = fs::read_dir(path).await?;
    let mut fls: Vec<File> = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let metadata = entry.metadata().await?;
        //file
        if metadata.is_file() {
            let name = entry.file_name().into_string().unwrap();
            fls.push(File {
                name: name.clone(),
                content_type: resolve_extention(&name),
                full_path: format!("{}/{}", path, name.clone()),
            });
        }
        //directory
        else {
            let name = entry.file_name().into_string().unwrap();
            fls.push(File {
                name: name.clone(),
                content_type: "dir".to_string(),
                full_path: format!("{}/{}", path, name.clone()),
            });
        }
    }
    Ok(Json(fls).into_response())
}

//return the content type of a file knowing it's extention
fn resolve_extention(f_name: &str) -> String {
    //try to find extention
    if let Some(idx) = f_name.rfind('.') {
        let content_ext = &f_name[(idx + 1)..];
        match content_ext {
            //simple text
            "txt" => String::from("txt"),

            //source files
            "html" | "css" | "js" | "rs" => String::from("code_file"),

            //image formats
            "png" | "jpg" | "svg" | "webp" | "gif" => String::from("image"),

            //pdf
            "pdf" => String::from("pdf"),

            //video formats
            "mp4" | "webm" => String::from("video"),
            //unkwown file extention
            _ => String::from("default_file"),
        }
    } else {
        String::from("default_file")
    }
}

//todo : trim path in full-path for repond_dir if request ends whit "/"
