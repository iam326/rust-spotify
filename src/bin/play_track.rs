extern crate rspotify;

use std::env;
use rspotify::client::Spotify;
use rspotify::model::offset::for_position;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::util::get_token;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: cargo run --bin play_track <TRACK_ID>");
        return;
    }
    let track_id = &args[1];
    let track_uri = "spotify:track:".to_string() + track_id;

    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state,user-read-playback-state")
        .build();

    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();

            // Reference: https://developer.spotify.com/documentation/web-api/reference/player/get-a-users-available-devices/
            let result = spotify.device().await.unwrap();
            let mut device_id = "";
            for device in result.devices.iter() {
                if let rspotify::senum::DeviceType::Computer = device._type {
                    if device.is_active == true {
                        device_id = &device.id;
                    }
                }
            }

            // Reference: https://developer.spotify.com/documentation/web-api/reference/player/start-a-users-playback/
            match spotify
                .start_playback(
                    // device_id: 対象とするデバイスのID
                    Some(device_id.to_string()),
                    // context_uri: 再生する album, artist, playlist の指定
                    None,
                    // uris: 再生する track のリストを指定
                    Some(vec![track_uri]),
                    // offset: album, playlist, track の再生を開始する位置
                    for_position(0),
                    // position_ms: track の再生を開始する位置
                    None
                )
                .await
            {
                Ok(_) => println!("start playback successful"),
                Err(_) => eprintln!("start playback failed"),
            }
        }
        None => println!("auth failed"),
    }
}