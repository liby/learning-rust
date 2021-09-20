use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Input {
    city: String,
}

// #[derive(Serialize, Deserialize, Debug)]
// struct Aqi {
//     status: String,
//     data: Vec<AqiData>,
// }

// impl Aqi {
//     async fn get(city: &String) -> Result<Self, ExitFailure> {
//         let url = format!(
//             "https://api.waqi.info/search/?token={}&keyword={}",
//             city
//         );

//         let url = Url::parse(&url)?;
//         let resp = reqwest::get(url).await?.json::<Aqi>().await?;
//         Ok(resp)
//     }
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct AqiData {
//     uid: u32,
//     aqi: String,
//     time: Time,
//     station: Station,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Time {
//     tz: String,
//     stime: String,
//     vtime: u32,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Station {
//     name: String,
//     geo: (f64, f64),
//     url: String,
// }

#[derive(Serialize, Deserialize, Debug)]
struct W {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Main,
}

impl W {
    async fn get(city: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid=4845f22236e074cdac59ae174aa580a3",
            city
        );

        let url = Url::parse(&url)?;
        let resp = reqwest::get(url).await?.json::<W>().await?;
        Ok(resp)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let input = Input::from_args();
    let response = W::get(&input.city).await?;
    // let aqi = Aqi::get(&input.city).await?;
    println!(
        "城市：{} \n 当前温度：{} \n 最高温度：{} \n 最低温度：{} \n 湿度：{} \n",
        input.city,
        response.main.temp,
        response.main.temp_max,
        response.main.temp_min,
        response.main.humidity,
    );
    Ok(())
}
