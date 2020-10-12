use chrono::prelude::*;
use dotenv;
use egg_mode;
use egg_mode::tweet::DraftTweet;
use num_cpus;
use std::{thread, time};
use sysinfo::SystemExt;

#[tokio::main]
async fn main() {
    let cpus = num_cpus::get();
    let cores = num_cpus::get_physical();
    println!("Hello world!");
    println!("We have [{}] CPUs", cpus);
    println!("We have [{}] Physical cores", cores);

    let mut system = sysinfo::System::new_all();

    let con_token_key = dotenv::var("API_KEY").unwrap();
    let con_token_secret = dotenv::var("API_SECRET_KEY").unwrap();
    let access_token_key = dotenv::var("ACCESS_TOKEN").unwrap();
    let access_token_secret = dotenv::var("ACCESS_TOKEN_SECRET").unwrap();

    let con_token = egg_mode::KeyPair::new(con_token_key, con_token_secret);

    let access_token = egg_mode::KeyPair::new(access_token_key, access_token_secret);

    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };

    // let user = egg_mode::user::show("thekevinwang", &token).await.unwrap();

    // println!("{} (@{})", user.name, user.screen_name);
    loop {
        &system.refresh_all();
        let local_time = Local::now();
        let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
        let nyc_timezone = FixedOffset::west(4 * 3600);

        let date = utc_time
            .with_timezone(&nyc_timezone)
            .format("%A %B %d, %Y")
            .to_string();
        let time = utc_time
            .with_timezone(&nyc_timezone)
            .format("%r")
            .to_string();

        let clocks = [
            "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚", "🕛",
        ];
        let hour = utc_time
            .with_timezone(&nyc_timezone)
            .format("%l")
            .to_string();
        let index = hour.trim().parse::<usize>().unwrap() - 1;

        let component = &system.get_components()[0];

        let message = format!("👋 Hello from 🍒 #RaspberryPi ⚙️ #Rust 🐳 #Docker\n---\n{clock} {time}\n📅 {date}\n🌡 {:?}", component, time=time, date=date, clock=clocks[index]);
        let draft = DraftTweet::new(message);

        draft.send(&token).await.unwrap();

        thread::sleep(time::Duration::from_secs(3600)); // 1 hrs
    }
}
