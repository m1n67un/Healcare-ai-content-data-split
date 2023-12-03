use serde_json::{json, Number};
use walkdir::WalkDir;
use std::io::{BufReader, Write};
use std::path::Path;
use std::fs::{self, File};
use rand::seq::SliceRandom;
use rand::thread_rng;

//  test data true: 200개, false: 400개 분류
pub fn run() {
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