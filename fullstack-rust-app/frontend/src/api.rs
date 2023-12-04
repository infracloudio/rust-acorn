use common::{ErrorResponse, Feedback, FeedbackListResponse, FeedbackResponse};
use reqwasm::http;
//use dotenv::dotenv;
//use std::env;
//use lazy_static::lazy_static;

// lazy_static! {
//     static ref BACKEND_SERVER: String = env::var("BACKEND_SERVER").expect("Error: BACKEND_SERVER not found");
// }

//let backend_server: &'static str = std::env!("BACKEND_SERVER");

//const backend_server = env::var("BACKEND_SERVER").expect("Error: BACKEND_SERVER not found").to_string();

pub async fn api_create_feedback(feedback_data: &str) -> Result<Feedback, String> {
    let backend_server = std::env!("BACKEND_SERVER");
    let response = match http::Request::post(format!("https://{}/api/feedbacks/", backend_server).as_str())
        .header("Content-Type", "application/json")
        .body(feedback_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<FeedbackResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.feedback),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn _api_fetch_single_feedback(feedback_id: &str) -> Result<Feedback, String> {
    let backend_server = std::env!("BACKEND_SERVER");
    let response = match http::Request::get(
        format!("https://{}/api/feedbacks/{}", backend_server, feedback_id).as_str(),
    )
    .send()
    .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<FeedbackResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.feedback),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_fetch_feedbacks((page, limit): (i32, i32)) -> Result<Vec<Feedback>, String> {
    //dotenv().ok();
    let backend_server = std::env!("BACKEND_SERVER");
    //let backend_server = env::var("BACKEND_SERVER").is_err();
    let response = match http::Request::get(
        format!("https://{}/api/feedbacks?page={}&limit={}", backend_server, page, limit).as_str(),
    )
    .send()
    .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<FeedbackListResponse>().await;
    match res_json {
        Ok(data) => Ok(data.feedbacks),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_delete_feedback(feedback_id: &str) -> Result<(), String> {
    let backend_server = std::env!("BACKEND_SERVER");
    let response = match http::Request::delete(
        format!("https://{}/api/feedbacks/{}", backend_server, feedback_id).as_str(),
    )
    .send()
    .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 204 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    Ok(())
}
