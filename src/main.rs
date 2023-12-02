extern crate walkdir;

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
fn split_init() {
    // 초기 image 디렉토리
    let origin_dir = r#"C:\Dataset\test_data\image\"#;

    // 전체 JSON 디렉토리
    let find_dir = r#"C:\storage\json\"#;
    let output_dir = r#"C:\Dataset\test_data\json\"#;

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

// 분류된 json 파일로부터 decayed_count [true:false] count 
fn decayed_count() {
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

fn data_split() {
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



//  test data true: 200개, false: 400개
fn data_split_test() {
    let image_dir = r#"C:\storage\image\"#;
    let json_dir = r#"C:\storage_copy\json\"#;
    let output_dir = r#"C:\Dataset\test_data\image\"#;

    let front_decayed_data = r#"C:\storage\front_decayed.json"#;
    let left_decayed_data = r#"C:\storage\left_decayed.json"#;
    let lower_decayed_data = r#"C:\storage\lower_decayed.json"#;
    let right_decayed_data = r#"C:\storage\right_decayed.json"#;
    let upper_decayed_data = r#"C:\storage\upper_decayed.json"#;

    let not_front_decayed_data = r#"C:\storage\not_front_decayed.json"#;
    let not_left_decayed_data = r#"C:\storage\not_left_decayed.json"#;
    let not_lower_decayed_data = r#"C:\storage\not_lower_decayed.json"#;
    let not_right_decayed_data = r#"C:\storage\not_right_decayed.json"#;
    let not_upper_decayed_data = r#"C:\storage\not_upper_decayed.json"#;

    let mut test_data = Vec::new();

    // front_decayed
    let front_decayed_file = fs::File::open(front_decayed_data)
        .expect("file should open read only");
    let front_decayed_reader = BufReader::new(front_decayed_file);
    let front_decayed_json: serde_json::Value = serde_json::from_reader(front_decayed_reader)
        .expect("file should be proper JSON");
    println!("front_decayed_json.len():: {}", front_decayed_json.as_array().unwrap().len());

    let mut front_rng = thread_rng();
    let mut front_numbers: Vec<u32> = (0..=2999).collect(); // 1부터 10까지의 숫자를 포함하는 벡터 생성
    front_numbers.shuffle(&mut front_rng); // 벡터를 섞음

    // 처음부터 원하는 개수만큼 랜덤 숫자를 추출

    let front_num_to_extract = 200;
    let front_selected_numbers: Vec<u32> = front_numbers.iter().take(front_num_to_extract).copied().collect();
    println!("front_decayed:: {:?}", front_selected_numbers);

    for (index, &value) in front_selected_numbers.iter().enumerate() {
        // println!("{}", index);
        println!("{}", front_decayed_json[index]);
        let file_name = front_decayed_json[index].to_string().replace("\"", "");
        let path = format!("{}{}{}", image_dir, file_name, ".png");
        println!("{}", path);

        let path = Path::new(&path);

        if path.is_file() {
            println!("파일이 존재합니다.");
            let _ = fs::create_dir_all(Path::new(&format!("{}{}", output_dir, "front")));
            let _ = fs::copy(path.to_str().unwrap(), format!("{}{}{}{}", output_dir, "front\\", file_name, ".png"));
            test_data.push(file_name);
        }
    }
    // -- END front_decayed

    // left_decayed
    let left_decayed_file = fs::File::open(left_decayed_data)
        .expect("file should open read only");
    let left_decayed_reader = BufReader::new(left_decayed_file);
    let left_decayed_json: serde_json::Value = serde_json::from_reader(left_decayed_reader)
        .expect("file should be proper JSON");
    println!("left_decayed_json.len():: {}", left_decayed_json.as_array().unwrap().len());

    let mut left_rng = thread_rng();
    let mut left_numbers: Vec<u32> = (0..=2999).collect(); // 1부터 10까지의 숫자를 포함하는 벡터 생성
    left_numbers.shuffle(&mut left_rng); // 벡터를 섞음

    // 처음부터 원하는 개수만큼 랜덤 숫자를 추출
    let left_num_to_extract = 200;
    let left_selected_numbers: Vec<u32> = left_numbers.iter().take(left_num_to_extract).copied().collect();
    println!("left_decayed:: {:?}", left_selected_numbers);

    for (index, &value) in left_selected_numbers.iter().enumerate() {
        // println!("{}", index);
        println!("{}", left_decayed_json[index]);
        let file_name = left_decayed_json[index].to_string().replace("\"", "");
        let path = format!("{}{}{}", image_dir, file_name, ".png");
        println!("{}", path);

        let path = Path::new(&path);

        if path.is_file() {
            println!("파일이 존재합니다.");
            let _ = fs::create_dir_all(Path::new(&format!("{}{}", output_dir, "left")));
            let _ = fs::copy(path.to_str().unwrap(), format!("{}{}{}{}", output_dir, "left\\", file_name, ".png"));
            test_data.push(file_name);
        }
    }
    // -- END left_decayed

    // lower_decayed
    let lower_decayed_file = fs::File::open(lower_decayed_data)
        .expect("file should open read only");
    let lower_decayed_reader = BufReader::new(lower_decayed_file);
    let lower_decayed_json: serde_json::Value = serde_json::from_reader(lower_decayed_reader)
        .expect("file should be proper JSON");
    println!("lower_decayed_json.len():: {}", lower_decayed_json.as_array().unwrap().len());

    let mut lower_rng = thread_rng();
    let mut lower_numbers: Vec<u32> = (0..=3999).collect(); // 1부터 10까지의 숫자를 포함하는 벡터 생성
    lower_numbers.shuffle(&mut lower_rng); // 벡터를 섞음

    // 처음부터 원하는 개수만큼 랜덤 숫자를 추출
    let lower_num_to_extract = 200;
    let lower_selected_numbers: Vec<u32> = lower_numbers.iter().take(lower_num_to_extract).copied().collect();
    println!("lower_decayed:: {:?}", lower_selected_numbers);

    for (index, &value) in lower_selected_numbers.iter().enumerate() {
        // println!("{}", index);
        println!("{}", lower_decayed_json[index]);
        let file_name = lower_decayed_json[index].to_string().replace("\"", "");
        let path = format!("{}{}{}", image_dir, file_name, ".png");
        println!("{}", path);

        let path = Path::new(&path);

        if path.is_file() {
            println!("파일이 존재합니다.");
            let _ = fs::create_dir_all(Path::new(&format!("{}{}", output_dir, "lower")));
            let _ = fs::copy(path.to_str().unwrap(), format!("{}{}{}{}", output_dir, "lower\\", file_name, ".png"));
            test_data.push(file_name);
        }
    }
    // -- END lower_decayed

    // right_decayed
    let right_decayed_file = fs::File::open(right_decayed_data)
        .expect("file should open read only");
    let right_decayed_reader = BufReader::new(right_decayed_file);
    let right_decayed_json: serde_json::Value = serde_json::from_reader(right_decayed_reader)
        .expect("file should be proper JSON");
    println!("right_decayed_json.len():: {}", right_decayed_json.as_array().unwrap().len());

    let mut right_rng = thread_rng();
    let mut right_numbers: Vec<u32> = (0..=2999).collect(); // 1부터 10까지의 숫자를 포함하는 벡터 생성
    right_numbers.shuffle(&mut right_rng); // 벡터를 섞음

    // 처음부터 원하는 개수만큼 랜덤 숫자를 추출
    let right_num_to_extract = 200;
    let right_selected_numbers: Vec<u32> = right_numbers.iter().take(right_num_to_extract).copied().collect();
    println!("right_decayed:: {:?}", right_selected_numbers);

    for (index, &value) in right_selected_numbers.iter().enumerate() {
        // println!("{}", index);
        println!("{}", right_decayed_json[index]);
        let file_name = right_decayed_json[index].to_string().replace("\"", "");
        let path = format!("{}{}{}", image_dir, file_name, ".png");
        println!("{}", path);

        let path = Path::new(&path);

        if path.is_file() {
            println!("파일이 존재합니다.");
            let _ = fs::create_dir_all(Path::new(&format!("{}{}", output_dir, "right")));
            let _ = fs::copy(path.to_str().unwrap(), format!("{}{}{}{}", output_dir, "right\\", file_name, ".png"));
            test_data.push(file_name);
        }
    }
    // -- END right_decayed

    // upper_decayed
    let upper_decayed_file = fs::File::open(upper_decayed_data)
        .expect("file should open read only");
    let upper_decayed_reader = BufReader::new(upper_decayed_file);
    let upper_decayed_json: serde_json::Value = serde_json::from_reader(upper_decayed_reader)
        .expect("file should be proper JSON");
    println!("upper_decayed_json.len():: {}", upper_decayed_json.as_array().unwrap().len());

    let mut upper_rng = thread_rng();
    let mut upper_numbers: Vec<u32> = (0..=4999).collect(); // 1부터 10까지의 숫자를 포함하는 벡터 생성
    upper_numbers.shuffle(&mut upper_rng); // 벡터를 섞음

    // 처음부터 원하는 개수만큼 랜덤 숫자를 추출
    let upper_num_to_extract = 200;
    let upper_selected_numbers: Vec<u32> = upper_numbers.iter().take(upper_num_to_extract).copied().collect();
    println!("upper_decayed:: {:?}", upper_selected_numbers);

    for (index, &value) in upper_selected_numbers.iter().enumerate() {
        // println!("{}", index);
        println!("{}", upper_decayed_json[index]);
        let file_name = upper_decayed_json[index].to_string().replace("\"", "");
        let path = format!("{}{}{}", image_dir, file_name, ".png");
        println!("{}", path);

        let path = Path::new(&path);

        if path.is_file() {
            println!("파일이 존재합니다.");
            let _ = fs::create_dir_all(Path::new(&format!("{}{}", output_dir, "upper")));
            let _ = fs::copy(path.to_str().unwrap(), format!("{}{}{}{}", output_dir, "upper\\", file_name, ".png"));
            test_data.push(file_name);
        }
    }
    // -- END upper_decayed



//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////




    // not_front_decayed
    let not_front_decayed_file = fs::File::open(not_front_decayed_data)
        .expect("file should open read only");
    let not_front_decayed_reader = BufReader::new(not_front_decayed_file);
    let not_front_decayed_json: serde_json::Value = serde_json::from_reader(not_front_decayed_reader)
        .expect("file should be proper JSON");
    println!("not_front_decayed_json.len():: {}", not_front_decayed_json.as_array().unwrap().len());

    let mut not_front_rng = thread_rng();
    let mut not_front_numbers: Vec<u32> = (0..=2999).collect(); // 1부터 10까지의 숫자를 포함하는 벡터 생성
    not_front_numbers.shuffle(&mut not_front_rng); // 벡터를 섞음

    // 처음부터 원하는 개수만큼 랜덤 숫자를 추출

    let not_front_num_to_extract = 400;
    let not_front_selected_numbers: Vec<u32> = not_front_numbers.iter().take(not_front_num_to_extract).copied().collect();
    println!("not_front_decayed:: {:?}", not_front_selected_numbers);

    for (index, &value) in not_front_selected_numbers.iter().enumerate() {
        // println!("{}", index);
        println!("{}", not_front_decayed_json[index]);
        let file_name = not_front_decayed_json[index].to_string().replace("\"", "");
        let path = format!("{}{}{}", image_dir, file_name, ".png");
        println!("{}", path);

        let path = Path::new(&path);

        if path.is_file() {
            println!("파일이 존재합니다.");
            let _ = fs::create_dir_all(Path::new(&format!("{}{}", output_dir, "not_front")));
            let _ = fs::copy(path.to_str().unwrap(), format!("{}{}{}{}", output_dir, "not_front\\", file_name, ".png"));
            test_data.push(file_name);
        }
    }
    // -- END not_front_decayed

    // not_left_decayed
    let not_left_decayed_file = fs::File::open(not_left_decayed_data)
        .expect("file should open read only");
    let not_left_decayed_reader = BufReader::new(not_left_decayed_file);
    let not_left_decayed_json: serde_json::Value = serde_json::from_reader(not_left_decayed_reader)
        .expect("file should be proper JSON");
    println!("not_left_decayed_json.len():: {}", not_left_decayed_json.as_array().unwrap().len());

    let mut not_left_rng = thread_rng();
    let mut not_left_numbers: Vec<u32> = (0..=2999).collect(); // 1부터 10까지의 숫자를 포함하는 벡터 생성
    not_left_numbers.shuffle(&mut not_left_rng); // 벡터를 섞음

    // 처음부터 원하는 개수만큼 랜덤 숫자를 추출
    let not_left_num_to_extract = 400;
    let not_left_selected_numbers: Vec<u32> = not_left_numbers.iter().take(not_left_num_to_extract).copied().collect();
    println!("not_left_decayed:: {:?}", not_left_selected_numbers);

    for (index, &value) in not_left_selected_numbers.iter().enumerate() {
        // println!("{}", index);
        println!("{}", not_left_decayed_json[index]);
        let file_name = not_left_decayed_json[index].to_string().replace("\"", "");
        let path = format!("{}{}{}", image_dir, file_name, ".png");
        println!("{}", path);

        let path = Path::new(&path);

        if path.is_file() {
            println!("파일이 존재합니다.");
            let _ = fs::create_dir_all(Path::new(&format!("{}{}", output_dir, "not_left")));
            let _ = fs::copy(path.to_str().unwrap(), format!("{}{}{}{}", output_dir, "not_left\\", file_name, ".png"));
            test_data.push(file_name);
        }
    }
    // -- END not_left_decayed

    // not_lower_decayed
    let not_lower_decayed_file = fs::File::open(not_lower_decayed_data)
        .expect("file should open read only");
    let not_lower_decayed_reader = BufReader::new(not_lower_decayed_file);
    let not_lower_decayed_json: serde_json::Value = serde_json::from_reader(not_lower_decayed_reader)
        .expect("file should be proper JSON");
    println!("not_lower_decayed_json.len():: {}", not_lower_decayed_json.as_array().unwrap().len());

    let mut not_lower_rng = thread_rng();
    let mut not_lower_numbers: Vec<u32> = (0..=1999).collect(); // 1부터 10까지의 숫자를 포함하는 벡터 생성
    not_lower_numbers.shuffle(&mut not_lower_rng); // 벡터를 섞음

    // 처음부터 원하는 개수만큼 랜덤 숫자를 추출
    let not_lower_num_to_extract = 400;
    let not_lower_selected_numbers: Vec<u32> = not_lower_numbers.iter().take(not_lower_num_to_extract).copied().collect();
    println!("not_lower_decayed:: {:?}", not_lower_selected_numbers);

    for (index, &value) in not_lower_selected_numbers.iter().enumerate() {
        // println!("{}", index);
        println!("{}", not_lower_decayed_json[index]);
        let file_name = not_lower_decayed_json[index].to_string().replace("\"", "");
        let path = format!("{}{}{}", image_dir, file_name, ".png");
        println!("{}", path);

        let path = Path::new(&path);

        if path.is_file() {
            println!("파일이 존재합니다.");
            let _ = fs::create_dir_all(Path::new(&format!("{}{}", output_dir, "not_lower")));
            let _ = fs::copy(path.to_str().unwrap(), format!("{}{}{}{}", output_dir, "not_lower\\", file_name, ".png"));
            test_data.push(file_name);
        }
    }
    // -- END not_lower_decayed

    // not_right_decayed
    let not_right_decayed_file = fs::File::open(not_right_decayed_data)
        .expect("file should open read only");
    let not_right_decayed_reader = BufReader::new(not_right_decayed_file);
    let not_right_decayed_json: serde_json::Value = serde_json::from_reader(not_right_decayed_reader)
        .expect("file should be proper JSON");
    println!("not_right_decayed_json.len():: {}", not_right_decayed_json.as_array().unwrap().len());

    let mut not_right_rng = thread_rng();
    let mut not_right_numbers: Vec<u32> = (0..=2999).collect(); // 1부터 10까지의 숫자를 포함하는 벡터 생성
    not_right_numbers.shuffle(&mut not_right_rng); // 벡터를 섞음

    // 처음부터 원하는 개수만큼 랜덤 숫자를 추출
    let not_right_num_to_extract = 400;
    let not_right_selected_numbers: Vec<u32> = not_right_numbers.iter().take(not_right_num_to_extract).copied().collect();
    println!("not_right_decayed:: {:?}", not_right_selected_numbers);

    for (index, &value) in not_right_selected_numbers.iter().enumerate() {
        // println!("{}", index);
        println!("{}", not_right_decayed_json[index]);
        let file_name = not_right_decayed_json[index].to_string().replace("\"", "");
        let path = format!("{}{}{}", image_dir, file_name, ".png");
        println!("{}", path);

        let path = Path::new(&path);

        if path.is_file() {
            println!("파일이 존재합니다.");
            let _ = fs::create_dir_all(Path::new(&format!("{}{}", output_dir, "not_right")));
            let _ = fs::copy(path.to_str().unwrap(), format!("{}{}{}{}", output_dir, "not_right\\", file_name, ".png"));
            test_data.push(file_name);
        }
    }
    // -- END not_right_decayed

    // not_upper_decayed
    let not_upper_decayed_file = fs::File::open(not_upper_decayed_data)
        .expect("file should open read only");
    let not_upper_decayed_reader = BufReader::new(not_upper_decayed_file);
    let not_upper_decayed_json: serde_json::Value = serde_json::from_reader(not_upper_decayed_reader)
        .expect("file should be proper JSON");
    println!("not_upper_decayed_json.len():: {}", not_upper_decayed_json.as_array().unwrap().len());

    let mut not_upper_rng = thread_rng();
    let mut not_upper_numbers: Vec<u32> = (0..=999).collect(); // 1부터 10까지의 숫자를 포함하는 벡터 생성
    not_upper_numbers.shuffle(&mut not_upper_rng); // 벡터를 섞음

    // 처음부터 원하는 개수만큼 랜덤 숫자를 추출
    let not_upper_num_to_extract = 400;
    let not_upper_selected_numbers: Vec<u32> = not_upper_numbers.iter().take(not_upper_num_to_extract).copied().collect();
    println!("not_upper_decayed:: {:?}", not_upper_selected_numbers);

    for (index, &value) in not_upper_selected_numbers.iter().enumerate() {
        // println!("{}", index);
        println!("{}", not_upper_decayed_json[index]);
        let file_name = not_upper_decayed_json[index].to_string().replace("\"", "");
        let path = format!("{}{}{}", image_dir, file_name, ".png");
        println!("{}", path);

        let path = Path::new(&path);

        if path.is_file() {
            println!("파일이 존재합니다.");
            let _ = fs::create_dir_all(Path::new(&format!("{}{}", output_dir, "not_upper")));
            let _ = fs::copy(path.to_str().unwrap(), format!("{}{}{}{}", output_dir, "not_upper\\", file_name, ".png"));
            test_data.push(file_name);
        }
    }
    // -- END not_upper_decayed

    let test_data_str = serde_json::to_string(&test_data).unwrap();
    let mut test_data_file = File::create(r#"C:\Dataset\test_data.json"#)
    .expect("Error encountered while creating file!");
    test_data_file.write_all(test_data_str.as_bytes())
    .expect("Error while writing to file");
}

// Dataset/
// |-- train_data/
// |   |-- image/
// |   |-- json/
// |-- test_data/
// |   |-- image/
// |   |-- json/



fn data_split_train() {
    let origin_dir = r#"C:\storage\image\"#;
    let output_dir = r#"C:\Dataset\train_data\image\"#;
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


    let test_data = r#"C:\Dataset\test_data.json"#;

    let test_data_file = fs::File::open(test_data)
        .expect("file should open read only");
    let test_data_reader = BufReader::new(test_data_file);
    let test_data_json: serde_json::Value = serde_json::from_reader(test_data_reader)
        .expect("file should be proper JSON");

    for file in files {
        let path = Path::new(&file);
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

        let mut door = false;
        for row in test_data_json.as_array().unwrap() {
            let cont = format!("{}{}", &row.to_string().replace("\"", ""), ".");
            if file_name.contains(&cont){
                println!("{} :: {}", file_name, cont);
               door = true; 
               break;
            }
        }

        let file_name = file_name.split(".").collect::<Vec<_>>();

        if !door {
            let origin_path = format!("{}{}{}", origin_dir, file_name[0], ".png");
            println!("Origin:: {}", origin_path);
            
            let _ = fs::create_dir_all(Path::new(&format!("{}", output_dir)));
            let _ = fs::copy(origin_path.to_string(), format!("{}{}{}", output_dir, file_name[0], ".png"));
        }
    }
}

fn main() {
    split_init();
}

/*
충치 18000개, 정상 12000개가 맞다고하면
아래의 구조로 데이터셋을 만들면 될 것 같습니다.

우리가 가지고 있을 데이터
Dataset/
|-- train_data/
|   |-- image/
|   |-- json/
|-- test_data/
|   |-- image/
|   |-- json/

유저에게 제공할 데이터
Dataset/
|-- train_data/
|   |-- image/
|   |-- json/
|-- test_data/
|   |-- image/

test_data폴더로 이동할 데이터의 개수
json의  "decayed": true값의 각 클래스별 200개씩 총 1000장
json의  "decayed": false값의  각 클래스별 400개씩 총 2000장

* 우리가 가지고 있어야하는 것은 image와 json 유저에게 제공할것은 image입니다.
*/