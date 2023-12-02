use serde_json::{json, Number};
use walkdir::WalkDir;
use std::io::{BufReader, Write};
use std::path::Path;
use std::fs::{self, File};
use rand::seq::SliceRandom;
use rand::thread_rng;

// 데이터 카운트
pub fn data_split() {
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

    let mut front_decayed_data = Vec::new();
    let mut left_decayed_data = Vec::new();
    let mut lower_decayed_data = Vec::new();
    let mut right_decayed_data = Vec::new();
    let mut upper_decayed_data = Vec::new();

    let mut not_front_decayed_data = Vec::new();
    let mut not_left_decayed_data = Vec::new();
    let mut not_lower_decayed_data = Vec::new();
    let mut not_right_decayed_data = Vec::new();
    let mut not_upper_decayed_data = Vec::new();

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

        let mut door = false;
        for row in array.as_array().unwrap() {
            println!("teeth_num:: {}", row.get("teeth_num").unwrap());
            println!("decayed:: {}", row.get("decayed").unwrap());
            
            if row.get("decayed").unwrap() == true {
                decayed_true += 1;
                door = true;
                break;
            }
        }
        
        if door {
            if file_name[0].to_string().contains("front") {
                front_decayed_data.push(file_name[0].to_string());
            } else if file_name[0].to_string().contains("left") {
                left_decayed_data.push(file_name[0].to_string());
            } else if file_name[0].to_string().contains("lower") {
                lower_decayed_data.push(file_name[0].to_string());
            } else if file_name[0].to_string().contains("right") {
                right_decayed_data.push(file_name[0].to_string());
            } else if file_name[0].to_string().contains("upper") {
                upper_decayed_data.push(file_name[0].to_string());
            }
        } else {
            if file_name[0].to_string().contains("front") {
                not_front_decayed_data.push(file_name[0].to_string());
            } else if file_name[0].to_string().contains("left") {
                not_left_decayed_data.push(file_name[0].to_string());
            } else if file_name[0].to_string().contains("lower") {
                not_lower_decayed_data.push(file_name[0].to_string());
            } else if file_name[0].to_string().contains("right") {
                not_right_decayed_data.push(file_name[0].to_string());
            } else if file_name[0].to_string().contains("upper") {
                not_upper_decayed_data.push(file_name[0].to_string());
            }
        }

        total_cnt += 1;
        println!("검색 수:: {}", total_cnt);
        println!("decayed_true_count:: {}", decayed_true);
    }

    let front_decayed_str = serde_json::to_string(&front_decayed_data).unwrap();
    let mut front_decayed_file = File::create(r#"C:\storage\front_decayed.json"#)
    .expect("Error encountered while creating file!");
    front_decayed_file.write_all(front_decayed_str.as_bytes())
    .expect("Error while writing to file");

    let left_decayed_str = serde_json::to_string(&left_decayed_data).unwrap();
    let mut left_decayed_file = File::create(r#"C:\storage\left_decayed.json"#)
    .expect("Error encountered while creating file!");
    left_decayed_file.write_all(left_decayed_str.as_bytes())
    .expect("Error while writing to file");

    let lower_decayed_str = serde_json::to_string(&lower_decayed_data).unwrap();
    let mut lower_decayed_file = File::create(r#"C:\storage\lower_decayed.json"#)
    .expect("Error encountered while creating file!");
    lower_decayed_file.write_all(lower_decayed_str.as_bytes())
    .expect("Error while writing to file");

    let right_decayed_str = serde_json::to_string(&right_decayed_data).unwrap();
    let mut right_decayed_file = File::create(r#"C:\storage\right_decayed.json"#)
    .expect("Error encountered while creating file!");
    right_decayed_file.write_all(right_decayed_str.as_bytes())
    .expect("Error while writing to file");

    let upper_decayed_str = serde_json::to_string(&upper_decayed_data).unwrap();
    let mut upper_decayed_file = File::create(r#"C:\storage\upper_decayed.json"#)
    .expect("Error encountered while creating file!");
    upper_decayed_file.write_all(upper_decayed_str.as_bytes())
    .expect("Error while writing to file");

    let not_front_decayed_str = serde_json::to_string(&not_front_decayed_data).unwrap();
    let mut not_front_decayed_file = File::create(r#"C:\storage\not_front_decayed.json"#)
    .expect("Error encountered while creating file!");
    not_front_decayed_file.write_all(not_front_decayed_str.as_bytes())
    .expect("Error while writing to file");

    let not_left_decayed_str = serde_json::to_string(&not_left_decayed_data).unwrap();
    let mut not_left_decayed_file = File::create(r#"C:\storage\not_left_decayed.json"#)
    .expect("Error encountered while creating file!");
    not_left_decayed_file.write_all(not_left_decayed_str.as_bytes())
    .expect("Error while writing to file");

    let not_lower_decayed_str = serde_json::to_string(&not_lower_decayed_data).unwrap();
    let mut not_lower_decayed_file = File::create(r#"C:\storage\not_lower_decayed.json"#)
    .expect("Error encountered while creating file!");
    not_lower_decayed_file.write_all(not_lower_decayed_str.as_bytes())
    .expect("Error while writing to file");

    let not_right_decayed_str = serde_json::to_string(&not_right_decayed_data).unwrap();
    let mut not_right_decayed_file = File::create(r#"C:\storage\not_right_decayed.json"#)
    .expect("Error encountered while creating file!");
    not_right_decayed_file.write_all(not_right_decayed_str.as_bytes())
    .expect("Error while writing to file");

    let not_upper_decayed_str = serde_json::to_string(&not_upper_decayed_data).unwrap();
    let mut not_upper_decayed_file = File::create(r#"C:\storage\not_upper_decayed.json"#)
    .expect("Error encountered while creating file!");
    not_upper_decayed_file.write_all(not_upper_decayed_str.as_bytes())
    .expect("Error while writing to file");

    println!("front_decayed_data.len():: {}", front_decayed_data.len());
    println!("left_decayed_data.len():: {}", left_decayed_data.len());
    println!("lower_decayed_data.len():: {}", lower_decayed_data.len());
    println!("right_decayed_data.len():: {}", right_decayed_data.len());
    println!("upper_decayed_data.len():: {}", upper_decayed_data.len());

    println!("not_front_decayed_data.len():: {}", not_front_decayed_data.len());
    println!("not_left_decayed_data.len():: {}", not_left_decayed_data.len());
    println!("not_lower_decayed_data.len():: {}", not_lower_decayed_data.len());
    println!("not_right_decayed_data.len():: {}", not_right_decayed_data.len());
    println!("not_upper_decayed_data.len():: {}", not_upper_decayed_data.len());
}