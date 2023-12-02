mod data;
mod search;

fn main() {
    data::split_init::split_init();
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