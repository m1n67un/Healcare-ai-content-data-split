mod data;
mod search;
mod model;

use model::RunType;

fn main() {
    const RUN_TYPE: RunType = RunType::DECAYED_COUNT;
    
    match RUN_TYPE {
        // IMAGE 3만개에 따른 JSON 14만개 분류, JSON 3만개로 추리기
        RunType::SPLIT_INIT => {
            data::split_init::run();
        },
        // 총 json에 기입된 decayed 카운트
        RunType::DECAYED_COUNT => {
            search::decayed_count::run();
        },
        // 각각의 클래스에 충치, 정상치아 카운트하여 json으로 추출
        RunType::DATA_SPLIT => {
            search::data_split::run();
        },
        // TEST 디렉토리에 분류된 JSON 파일을 바탕으로 TEST 디렉트리에 분류
        RunType::DATA_SPLIT_TEST_DIRECTORY => {
            search::data_split_test_directory::run();
        },
        // test_data.json 파일을 보고, 이외의 이미지들을 train_data 디렉토리에 분류
        RunType::DATA_SPLIT_TRAIN_DIRECTORY => {
            search::data_split_train_directory::run();
        },
        _ => {
            println!("ignored run.");
        },
    }
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