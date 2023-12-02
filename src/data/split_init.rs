use serde_json::{json, Number};
use walkdir::WalkDir;
use std::io::{BufReader, Write};
use std::path::Path;
use std::fs::{self, File};
use rand::seq::SliceRandom;
use rand::thread_rng;

// 1차 분류
// Dataset/
// |-- image/
// |-- json/
pub fn split_init() {
    // 초기 image 디렉토리
    let origin_dir = r#"C:\storage\image\"#;

    // 전체 JSON 디렉토리
    let find_dir = r#"C:\storage\json\"#;
    let output_dir = r#"C:\storage_copy\json\"#;

    println!("path :: {}", origin_dir);

    let mut files = Vec::new();

    // path 디렉토리의 모든 파일과 하위 디렉토리를 재귀적으로 탐색한다.
    let walker = WalkDir::new(origin_dir);
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

    let mut matching_file_cnt = 0;
    for file in files {
        let path = Path::new(&file);
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

        let origin_path = format!("{}{}", origin_dir, file_name);
        println!("Origin:: {}", origin_path);

        let file_name = file_name.split(".").collect::<Vec<_>>();
        let find_path = format!("{}{}{}", find_dir, file_name[0], ".json");
        println!("Find:: {}", find_path);
        let find_file_path = Path::new(&find_path);
        
        println!("파일 존재 여부:: {}", find_file_path.is_file());
        if find_file_path.is_file() {
            matching_file_cnt += 1;
        }

        let _ = fs::create_dir_all(Path::new(&format!("{}", output_dir)));
        let _ = fs::copy(find_path.to_string(), format!("{}{}{}", output_dir, file_name[0], ".json"));
    }

    println!("총 매칭되는 파일수:: {}", matching_file_cnt);
}