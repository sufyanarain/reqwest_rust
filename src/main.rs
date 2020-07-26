// extern crate reqwest;

// fn main(){
//     match reqwest::get("https://www.google.com/"){
//         Ok(mut response) => {
//             match response.text(){
//                 Ok(texxt) => println!("{}",texxt),
//                 Err(_) => println!("the error"),
//             }
//         }
//         Err(_) => println!("error")
//     }
// }


// use std::collections::HashMap;
// use async_std::task;

// fn main() -> Result<(),Box<dyn  std::error::Error + Send + Sync>>{
//     task::block_on(async{
//         let response = reqwest::blocking::get("https://httpbin.org/ip")?
//         .json::<HashMap<String,String>>()?;
//         println!("{:#?}",response);
//         Ok(())
//     })
    
// }


use std::collections::HashMap;

#[tokio::main]
async fn main()-> Result<(),Box<dyn std::error::Error>>{

    let response = reqwest::get("https://httpbin.org/ip").await?
    .json::<HashMap<String,String>>().await?;
    println!("{:?}",response);
    Ok(())
}

// use std::collections::HashMap;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let resp = reqwest::get("https://httpbin.org/ip")
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{:#?}", resp);
//     Ok(())
// }








