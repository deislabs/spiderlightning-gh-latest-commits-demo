use anyhow::Result;

wit_bindgen_rust::import!("wit/keyvalue_0.4.1/keyvalue.wit");
wit_error_rs::impl_error!(keyvalue::KeyvalueError);

use http_server::*;
use slight_http_handler_macro::register_handler;
use slight_http_server_macro::on_server_init;

use substring::Substring;

wit_bindgen_rust::import!("http-client_0.4.1/http-client.wit");
wit_bindgen_rust::import!("http-server_0.4.1/http-server.wit");
wit_bindgen_rust::export!("http-server_0.4.1/http-server-export.wit");

wit_error_rs::impl_error!(http_server::HttpRouterError);
wit_error_rs::impl_error!(http_client::HttpError);

use sql::*;
wit_bindgen_rust::import!("sql_0.4.1/sql.wit");
wit_error_rs::impl_error!(sql::SqlError);


//intentionally doing nothing here
fn main() -> Result<()> {
    Ok(())
}

#[on_server_init]
fn server() -> Result<()> {
    let router = Router::new()?;
    let router_with_route = router
        .get("/init_db", "handle_init_db")?
        .get("/show_feeds", "handle_show_feeds")?;
    println!("Server is running on port 3000");
    let _ = Server::serve("0.0.0.0:3000", &router_with_route)?;
    Ok(())
}

#[register_handler]
fn handle_show_feeds(_req: Request) -> Result<Response, HttpError> {
    
    let mut commits: String = "<link rel=\"stylesheet\" href=\"https://unpkg.com/@picocss/pico@1.*/css/pico.classless.min.css\"><main class=\"container\"><h1>Most recent commits to the GitHub repos listed</h1>".to_owned();
    
    let sql = Sql::open("my-db").expect("There was and error connected to the database");

    let repos = sql.query(&sql::Statement::prepare(
        "SELECT name FROM repos",
        &[],
    )).expect("there was a problem running the query");
    
    //shows the latest additions to table first, making it easier for demo purposes
    let mut inv_repos = repos;
    inv_repos.reverse();

    for repo in inv_repos {

        let r = if let DataType::Str(inner) = repo.value.clone() {
            inner
            } else {
            panic!("expected string");
            };

        commits.push_str("<h2>");
        commits.push_str(&r);
        commits.push_str("</h2>");

        let mut uri: String = "https://api.github.com/repos/".to_owned();
        uri.push_str(&r);
        uri.push_str("/commits");

        let req = crate::http_client::Request {
            method: crate::http_client::Method::Get,
            uri: uri.as_str(),
            headers: &[("User-Agent", "Slight HTTP Client")],
            body: None,
            params: &[],
        };
        let res = crate::http_client::request(req).unwrap();
        
        
        //parsing JSON
        let body = res.body.unwrap();
        let json_string = String::from_utf8(body).unwrap();
        
        let gh_json: serde_json::Value = serde_json::from_str(&json_string).expect("we had a problem parsing the JSON");
        
        commits.push_str("<p><ul>");
        //show the 5 most recent commits
        for i in 0..5 {
            let msg = &gh_json[i]["commit"]["message"];
            let url = &gh_json[i]["html_url"];
            let date = &gh_json[i]["commit"]["author"]["date"];

            //temporary hack to re-use quotes from json strings
            commits.push_str("<li>");
            commits.push_str(date.to_string().as_str().substring(1, 11));
            commits.push_str(" - <a href=");
            commits.push_str(url.to_string().as_str());
            commits.push_str(">");
            commits.push_str(msg.to_string().as_str().substring(1, 40));
            commits.push_str("...");
            commits.push_str("</a></li>");
        }
        commits.push_str("</ul></p>");
        
    }
    commits.push_str("</main>");

    let ct: String = "Content-Type".to_owned();
    let ct_val: String = "text/html; charset=utf-8".to_owned();
    let headers: Option<Vec<(String, String)>> = Some(Vec::from([(ct, ct_val)]));

    let res = Response {
        status: 200,
        headers: headers,
        body: Some(commits.as_bytes().to_vec()),
    };
    Ok(res)
}


#[register_handler]
fn handle_init_db(_req: Request) -> Result<Response, HttpError> {
    
    println!("initializing database");

    let _db = init_db().map_err(|e| match e {
        e => HttpError::UnexpectedError(e.to_string())
    });

    let ct: String = "Content-Type".to_owned();
    let ct_val: String = "text/plain".to_owned();
    let headers: Option<Vec<(String, String)>> = Some(Vec::from([(ct, ct_val)]));
    
    let res = Response {
        status: 200,
        headers: headers,
        body: Some("database initialized".as_bytes().to_vec()),
    };
    Ok(res)
}
    

fn init_db() -> Result<()> {
    
    let sql = Sql::open("my-db")?;

    sql.exec(&sql::Statement::prepare(
        "CREATE TABLE IF NOT EXISTS repos (name TEXT NOT NULL)",
        &[],
    ))?;

    let repos = vec![
        "deislabs/spiderlightning",
        "containerd/runwasi",
        "oras-project/oras",
        "deislabs/wagi",
        "notaryproject/notaryproject.dev",
        ];
        
    for repo in repos {
        sql.exec(&sql::Statement::prepare(
            "INSERT INTO repos (name) VALUES (?)",
            &[&repo],
        ))?;
    }

    Ok(())
}