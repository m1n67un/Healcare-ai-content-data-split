use serde_json::{json, Number};
use walkdir::WalkDir;
use std::io::{BufReader, Write};
use std::path::Path;
use std::fs::{self, File};
use rand::seq::SliceRandom;
use rand::thread_rng;

// 분류된 json 파일로부터 decayed_count [true:false] count 
pub fn decayed_count() {
    let image_dir = r#"C:\storage\image\"#;
    let json_dir = r#"C:\storage_copy\json\"#;

    println!("path :: {}", image_dir);

    let mut files = Vec::new();

    // path 디렉토리의 모든 파일과 하위 디렉토리를 재귀적으로 탐색한다.
    let walker = WalkDir::new(image_dir);
     // walker의 결과를 반복자로 변환한다.
    let iterator = walker.into_iter();
     // iterator의 각 항목(`e`)에 대해 오류가 발생한 항목을 필터링하고 성공한 항목만 반환한다.
    let filtered_iterator = iterator.filter_map(|e| e.ok());

     // 필터링된 결과를 entry로 하나씩 꺼내서 처리한다.
    for entry in filtered_iterator {

        // 항목의 유형이 파일인지 확인한다.
        if entry.file_type().is_file() {

            // 해당 항목의 경로를 문자열로 변환하여 files 벡터에 추가한다.
            let file_path = entry.path().display().to_string();
            files.push(file_path);
        }
    }

    let mut total_cnt = 0;
    let mut decayed_true = 0;

    for file in files {
        let path = Path::new(&file);
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

        let origin_path = format!("{}{}", image_dir, file_name);
        println!("Origin:: {}", origin_path);

        let file_name = file_name.split(".").collect::<Vec<_>>();
        let find_path = format!("{}{}{}", json_dir, file_name[0], ".json");
        println!("Find:: {}", find_path);

        let file = fs::File::open(find_path)
            .expect("file should open read only");
        let reader = BufReader::new(file);
        let json: serde_json::Value = serde_json::from_reader(reader)
            .expect("file should be proper JSON");
        let tooth = json.get("tooth");

        // tooth 객체
        let array = json!(tooth);

        for row in array.as_array().unwrap() {
            println!("teeth_num:: {}", row.get("teeth_num").unwrap());
            println!("decayed:: {}", row.get("decayed").unwrap());
            
            if row.get("decayed").unwrap() == true {
                decayed_true += 1;
                break;
            }
        }

        total_cnt += 1;
        println!("검색 수:: {}", total_cnt);
        println!("decayed_true_count:: {}", decayed_true);
    }
}