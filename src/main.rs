use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
#[derive(Serialize, Deserialize, Debug)]
struct Dog{
    name:String,
    year_born:i32,
    owner:Owner,
    #[serde(rename="born-breed")]
    breed:String,
}
#[derive(Serialize,Debug,Deserialize)]
struct Owner{
    first_name:String,
    last_name:String,
}

fn main() {
   test_serialize();
   test_deserialize();
    
}

fn test_serialize(){
    let owner = Owner{first_name:"Jim".to_string(),last_name:"Du".to_string()};
    let dog = Dog{name:"chimmy".to_string(),year_born:2021,owner,breed:"jinMao".to_string()};
    let json = to_string_pretty(&dog);
    if json.is_ok(){
        println!("{}",json.ok().unwrap() );
    } else {
        println!("{:#?}",json.err() );
    }
}

fn test_deserialize(){
    let str = r#"{
  "name": "chimmy",
  "year_born": 2021,
  "owner": {
    "first_name": "Jim",
    "last_name": "Du"
  },
  "born-breed": "jinMao"
}"#;
    let object = from_str::<Dog>(str);
    if object.is_ok(){
        println!("{:?}",object.ok().unwrap() );
    } else {
        println!("{:?}",object.err() );
    }
}
