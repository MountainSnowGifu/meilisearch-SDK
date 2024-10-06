mod directory_helper;

use lazy_static::lazy_static;
use meilisearch_sdk::client::Client;
use meilisearch_sdk::settings::Settings;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::Duration;
use uuid::Uuid;
use walkdir::WalkDir;

// instantiate the client. load it once
lazy_static! {
    static ref CLIENT: Client = Client::new(
        "http://localhost:7700",
        Some("VeHGCo5ckA53sUuh64Xa5ES8Lq0zKdE_Y3xTTwRVbXo")
    )
    .unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct File {
    id: Box<usize>,
    file_path: Box<String>,
    file_name: Box<String>,
    file_extension: Box<String>,
    created_date: Box<String>,
    updated_date: Box<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExcelFile {
    id: Box<usize>,
    file_path: Box<String>,
    file_name: Box<String>,
    sheets: Box<Vec<String>>,
    created_date: Box<String>,
    updated_date: Box<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    id: usize,
    title: String,
    genres: Vec<String>,
}

#[tokio::main]
async fn main() {
    //build_index().await;
    //build_index_2().await;
    //build_index_3().await;
    build_index_4().await;
}
async fn build_index_4() {
    let dir = r"C:\Users\oasis\OneDrive\Desktop\RUST"; // 走査したいディレクトリパス
    let displayed_attributes = [
        "created_date",
        "updated_date",
        "file_name",
        "file_path",
        "sheets",
    ];
    let searchable_attributes = ["file_name", "sheets"];
    let ranking_rules = [
        "words",
        "typo",
        "attribute",
        "exactness",
        "updated_date:desc",
    ];

    let _ = CLIENT.index("excels").delete().await.unwrap();

    let settings = Settings::new()
        .with_searchable_attributes(searchable_attributes)
        .with_displayed_attributes(displayed_attributes)
        .with_ranking_rules(ranking_rules);

    let result = CLIENT
        .index("excels")
        .set_settings(&settings)
        .await
        .unwrap()
        .wait_for_completion(&CLIENT, None, None)
        .await
        .unwrap();

    if result.is_failure() {
        panic!(
            "インデックス設定中にエラーが発生しました: {:?}",
            result.unwrap_failure()
        );
    }

    let mut count = 0;
    let mut id = 0;
    let mut excel_file_v: Vec<ExcelFile> = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let entry = Box::new(entry);
        let path = Box::new(entry.path());
        let mut v_sheets: Vec<String> = Vec::new();
        if path.is_file() {
            id += 1;
            let created_date = Box::new(directory_helper::get_created_date_by_path(&path));
            let modified_date = Box::new(directory_helper::get_modified_date_by_path(&path));
            let file_name = Box::new(entry.file_name().to_str().unwrap_or("unknown").to_string());
            let file_path = Box::new(path.to_str().unwrap_or("unknown").to_string());
            v_sheets.push("sheet1".to_string());
            v_sheets.push(Uuid::new_v4().to_string());
            v_sheets.push(Uuid::new_v4().to_string());
            v_sheets.push(Uuid::new_v4().to_string());
            v_sheets.push(Uuid::new_v4().to_string());
            v_sheets.push(Uuid::new_v4().to_string());
            v_sheets.push(Uuid::new_v4().to_string());
            v_sheets.push(Uuid::new_v4().to_string());
            v_sheets.push(Uuid::new_v4().to_string());
            v_sheets.push(Uuid::new_v4().to_string());
            v_sheets.push(Uuid::new_v4().to_string());

            let excel = Box::new(ExcelFile {
                id: Box::new(id),
                file_path: file_path,
                file_name: file_name,
                sheets: Box::new(v_sheets),
                created_date: created_date,
                updated_date: modified_date,
            });

            excel_file_v.push(*excel);
            count += 1;
            println!("id:{}", id);

            if count >= 1000 {
                let duration = Some(Duration::from_millis(5000));
                let time_out = Some(Duration::from_millis(30000));

                let result = CLIENT
                    .index("excels")
                    .add_or_update(&excel_file_v, Some("id"))
                    .await
                    .unwrap()
                    .wait_for_completion(&CLIENT, duration, time_out)
                    .await;

                match result {
                    Ok(_) => {
                        println!("success");
                        //files.add_documents(&[file], Some("id")).await.unwrap();
                        excel_file_v.clear();
                        count = 0;
                    }
                    Err(e) => {
                        println!("error:{},{}", count, e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                    }
                }
            }
        }
    }
}

async fn build_index_3() {
    //let dir = "./"; // 走査したいディレクトリパス
    let dir = r"C:\Users\oasis\OneDrive\Desktop\RUST"; // 走査したいディレクトリパス
    let dir = r"C:\Users"; // 走査したいディレクトリパス

    let displayed_attributes = [
        "created_date",
        "updated_date",
        "file_name",
        "file_path",
        "file_extension",
    ];
    let searchable_attributes = ["file_name"];
    let ranking_rules = [
        "words",
        "typo",
        "attribute",
        "exactness",
        "updated_date:desc",
    ];

    let files = CLIENT.index("files");
    files.clone().delete().await.unwrap();

    let settings = Settings::new()
        .with_searchable_attributes(searchable_attributes)
        .with_displayed_attributes(displayed_attributes)
        .with_ranking_rules(ranking_rules);

    let result = CLIENT
        .index("files")
        .set_settings(&settings)
        .await
        .unwrap()
        .wait_for_completion(&CLIENT, None, None)
        .await
        .unwrap();

    if result.is_failure() {
        panic!(
            "インデックス設定中にエラーが発生しました: {:?}",
            result.unwrap_failure()
        );
    }

    let mut count = 0;
    let mut id = 0;
    let mut files_v: Vec<File> = Vec::new();

    // WalkDir を使ってディレクトリを再帰的に巡回
    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let entry = Box::new(entry);
        //println!("path:{}", entry.path().display());
        // println!(
        //     "file_name:{}",
        //     entry.file_name().to_str().unwrap_or("unknown")
        // );

        let path = Box::new(entry.path());
        let file_extension = Box::new(directory_helper::get_extension_by_path(&path));

        if path.is_file() && *file_extension != "none".to_string() {
            id += 1;
            let created_date = Box::new(directory_helper::get_created_date_by_path(&path));
            let modified_date = Box::new(directory_helper::get_modified_date_by_path(&path));
            //let file_extension = directory_helper::get_extension_by_path(&path);
            let file_name = Box::new(entry.file_name().to_str().unwrap_or("unknown").to_string());
            let file_path = Box::new(path.to_str().unwrap_or("unknown").to_string());

            let file = Box::new(File {
                id: Box::new(id),
                file_path: file_path,
                file_name: file_name,
                file_extension: file_extension,
                created_date: created_date,
                updated_date: modified_date,
            });

            files_v.push(*file);
            count += 1;
            println!("id:{}", id);

            if count == 10000 {
                //let json = serde_json::to_string(&files_v).unwrap();
                // 結果を出力
                //println!("{:#?}", json);
                //let files_json: Vec<FileJson> = serde_json::from_str(&json).unwrap();
                //println!("{:#?}", files_json);

                let duration = Some(Duration::from_millis(1000));

                let result = CLIENT
                    .index("files")
                    .add_or_update(&files_v, Some("id"))
                    .await
                    .unwrap()
                    .wait_for_completion(&CLIENT, duration, None)
                    .await;

                match result {
                    Ok(_) => {
                        //println!("success");
                        //files.add_documents(&[file], Some("id")).await.unwrap();
                        files_v.clear();
                        count = 0;
                    }
                    Err(e) => {
                        println!("error:{}", e);
                    }
                }
            }
        }
    }
}

async fn build_index_2() {
    // reading and parsing the file
    let content = include_str!("../assets/clothes_2.json");

    // serialize the string to clothes objects
    let clothes: Vec<Clothes> = serde_json::from_str(content).unwrap();

    println!("{:#?}", clothes);

    // create displayed attributes
    let displayed_attributes = ["article", "cost", "size", "pattern"];

    // Create ranking rules
    let ranking_rules = ["words", "typo", "attribute", "exactness", "cost:asc"];

    // create searchable attributes
    let searchable_attributes = ["season", "article", "size", "pattern"];

    // create the synonyms hashmap
    let mut synonyms = std::collections::HashMap::new();
    synonyms.insert("sweater", vec!["cardigan", "long-sleeve"]);
    synonyms.insert("sweat pants", vec!["joggers", "gym pants"]);
    synonyms.insert("t-shirt", vec!["tees", "tshirt"]);

    // create the settings struct
    let settings = Settings::new()
        .with_ranking_rules(ranking_rules)
        .with_searchable_attributes(searchable_attributes)
        .with_displayed_attributes(displayed_attributes)
        .with_synonyms(synonyms);

    // add the settings to the index
    let result = CLIENT
        .index("test")
        .set_settings(&settings)
        .await
        .unwrap()
        .wait_for_completion(&CLIENT, None, None)
        .await
        .unwrap();

    if result.is_failure() {
        panic!(
            "Encountered an error while setting settings for index: {:?}",
            result.unwrap_failure()
        );
    }

    // add the documents
    let result = CLIENT
        .index("test")
        .add_or_update(&clothes, Some("id"))
        .await
        .unwrap()
        .wait_for_completion(&CLIENT, None, None)
        .await
        .unwrap();

    if result.is_failure() {
        panic!(
            "Encountered an error while sending the documents: {:?}",
            result.unwrap_failure()
        );
    }
}

async fn build_index() {
    // reading and parsing the file
    let content = include_str!("../assets/clothes.json");

    // serialize the string to clothes objects
    let clothes: Vec<Clothes> = serde_json::from_str(content).unwrap();

    // create displayed attributes
    let displayed_attributes = ["article", "cost", "size", "pattern"];

    // Create ranking rules
    let ranking_rules = ["words", "typo", "attribute", "exactness", "cost:asc"];

    // create searchable attributes
    let searchable_attributes = ["season", "article", "size", "pattern"];

    // create the synonyms hashmap
    let mut synonyms = std::collections::HashMap::new();
    synonyms.insert("sweater", vec!["cardigan", "long-sleeve"]);
    synonyms.insert("sweat pants", vec!["joggers", "gym pants"]);
    synonyms.insert("t-shirt", vec!["tees", "tshirt"]);

    // create the settings struct
    let settings = Settings::new()
        .with_ranking_rules(ranking_rules)
        .with_searchable_attributes(searchable_attributes)
        .with_displayed_attributes(displayed_attributes)
        .with_synonyms(synonyms);

    // add the settings to the index
    let result = CLIENT
        .index("clothes")
        .set_settings(&settings)
        .await
        .unwrap()
        .wait_for_completion(&CLIENT, None, None)
        .await
        .unwrap();

    if result.is_failure() {
        panic!(
            "Encountered an error while setting settings for index: {:?}",
            result.unwrap_failure()
        );
    }

    // add the documents
    let result = CLIENT
        .index("clothes")
        .add_or_update(&clothes, Some("id"))
        .await
        .unwrap()
        .wait_for_completion(&CLIENT, None, None)
        .await
        .unwrap();

    if result.is_failure() {
        panic!(
            "Encountered an error while sending the documents: {:?}",
            result.unwrap_failure()
        );
    }
}

/// Base search object.
#[derive(Serialize, Deserialize, Debug)]
pub struct Clothes {
    id: usize,
    season: String,
    article: String,
    cost: f32,
    size: String,
    pattern: String,
}

/// Search results get serialized to this struct
#[derive(Serialize, Deserialize, Debug)]
pub struct ClothesDisplay {
    article: String,
    cost: f32,
    size: String,
    pattern: String,
}

impl fmt::Display for ClothesDisplay {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "result\n article: {},\n price: {},\n size: {},\n pattern: {}\n",
            self.article, self.cost, self.size, self.pattern
        )
    }
}

async fn search(query: &str) {
    // make the search query, which executes and serializes hits into the
    // ClothesDisplay struct
    let query_results = CLIENT
        .index("clothes")
        .search()
        .with_query(query)
        .execute::<ClothesDisplay>()
        .await
        .unwrap()
        .hits;

    // display the query results
    if query_results.is_empty() {
        println!("no results...");
    } else {
        for clothes in query_results {
            let display = clothes.result;
            println!("{}", format_args!("{}", display));
        }
    }
}
