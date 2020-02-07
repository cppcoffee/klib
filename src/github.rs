use std::collections::HashMap;

use reqwest::blocking::{Client, ClientBuilder};
use reqwest::header::{self, HeaderValue, ACCEPT, AUTHORIZATION};
use reqwest::StatusCode;
use serde::Serialize;

use crate::errors::{Error, Result};

const BASEURL: &'static str = "https://api.github.com";

lazy_static! {
    static ref ROBOT_NAME: &'static str = "kindle robot";
    static ref ROBOT_EMAIL: &'static str = "robot@kindle.com";
    static ref ROBOT_UA: String = format!("{}/{}", *ROBOT_NAME, env!("CARGO_PKG_VERSION"));
}

// contents metadata.
#[derive(Debug)]
struct Metadata {
    // The blob SHA of the file.
    sha: String,
    // Raw content download url.
    download_url: String,
}

// github committerinformation.
#[derive(Serialize)]
struct Committer {
    // The name of the author (or committer) of the commit
    name: String,
    // The email of the author (or committer) of the commit
    email: String,
}

// github put api params.
#[derive(Serialize)]
struct PutArgs {
    // Required. The commit message.
    message: String,
    // Required. The new file content, using Base64 encoding.
    content: String,
    // The person that committed the file. Default: the authenticated user.
    committer: Committer,
    // Required if you are updating a file. The blob SHA of the file being replaced.
    sha: Option<String>,
}

/// sync_repo use github API create and update content.
/// Github API: https://developer.github.com/v3/repos/contents/
pub fn sync_repo(
    owner: &str,
    repo: &str,
    token: &str,
    notes: &HashMap<String, String>,
) -> Result<()> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("token {}", token))?,
    );

    let client = ClientBuilder::new()
        .gzip(true)
        .user_agent(ROBOT_UA.to_owned())
        .default_headers(headers)
        .build()?;

    let url = format!("{}/repos/{}/{}/contents/", BASEURL, owner, repo);
    let resp = client.get(&url).send()?;
    if resp.status() != StatusCode::OK {
        return Err(Error::InvalidResponse(url, resp));
    }

    let mut put_cnt = 0;
    let mut nmd_cnt = 0;
    let value: serde_json::Value = resp.json()?;
    let metas = collect_metadata(&value)?;

    // put all resource.
    for (name, text) in notes {
        let path = name.to_owned() + ".md";
        let url = format!("{}/repos/{}/{}/contents/{}", BASEURL, owner, repo, path);

        match metas.get(&path) {
            Some(m) => {
                if is_url_same_text(&client, &m.download_url, text)? {
                    println!("===> {}", path);
                    nmd_cnt += 1;
                    continue;
                }

                put_note(&client, &url, Some(m.sha.clone()), text)?;
                println!("+++> {}", path);
                put_cnt += 1;
            }
            None => {
                put_note(&client, &url, None, text)?;
                println!("+++> {}", path);
                put_cnt += 1;
            }
        }
    }
    println!("total {} items put, {} items no modify.", put_cnt, nmd_cnt);

    Ok(())
}

fn collect_metadata(value: &serde_json::Value) -> Result<HashMap<String, Metadata>> {
    if !value.is_array() {
        return Err(Error::InvalidMetaJson(format!("{:?}", value)));
    }

    let array = value.as_array().unwrap();
    let mut res = HashMap::new();

    for elem in array {
        if !elem["path"].is_string()
            || !elem["sha"].is_string()
            || !elem["download_url"].is_string()
        {
            return Err(Error::InvalidMetaJson(format!("{:?}", value)));
        }

        let path = elem["path"].as_str().unwrap().to_string();
        let sha = elem["sha"].as_str().unwrap().to_string();
        let download_url = elem["download_url"].as_str().unwrap().to_string();

        res.insert(path, Metadata { sha, download_url });
    }

    Ok(res)
}

fn is_url_same_text(client: &Client, url: &str, text: &str) -> Result<bool> {
    let resp = client.get(url).send()?;
    let status = resp.status();

    if status != StatusCode::OK {
        return Err(Error::InvalidResponse(url.to_owned(), resp));
    }

    Ok(resp.text()? == text)
}

fn put_note(client: &Client, url: &str, sha: Option<String>, text: &str) -> Result<()> {
    let message = match sha {
        Some(_) => "robot update note.".to_owned(),
        None => "robot create new note.".to_owned(),
    };

    let committer = Committer {
        name: ROBOT_NAME.to_owned(),
        email: ROBOT_EMAIL.to_owned(),
    };

    let s = serde_json::to_string(&PutArgs {
        message,
        sha,
        committer,
        content: base64::encode(text),
    })?;

    let resp = client.put(url).body(s).send()?;
    let status = resp.status();

    if status != StatusCode::OK && status != StatusCode::CREATED {
        return Err(Error::InvalidResponse(url.to_string(), resp));
    }

    Ok(())
}
