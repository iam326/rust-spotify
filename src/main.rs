extern crate rspotify;


// use rspotify::client::Spotify;
// use rspotify::oauth2::SpotifyClientCredentials;
// use rspotify::senum::Country;

// #[tokio::main]
// async fn main() {
//     let client_credential = SpotifyClientCredentials::default().build();
//     let spotify = Spotify::default()
//         .client_credentials_manager(client_credential)
//         .build();
//     // Reference: https://developer.spotify.com/documentation/web-api/reference/artists/get-artists-top-tracks/
//     let result = spotify
//         .artist_top_tracks(
//             // id: アーティストの Spotify ID
//             "3NRXKeatDxKe4apH6XawKX",
//             // country: 国名コード
//             Country::Japan
//         )
//         .await
//         .unwrap();
    
//     println!("# {} Top Tracks", result.tracks[0].artists[0].name);
//     for track in result.tracks.iter() {
//         println!("{}: {} ({})", track.name, track.popularity, track.id.as_deref().unwrap_or("none"));
//     }
// }

// use rspotify::client::Spotify;
// use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
// use rspotify::senum::{Country, SearchType};
// use rspotify::util::get_token;

// #[tokio::main]
// async fn main() {
//     let mut oauth = SpotifyOAuth::default().scope("user-read-private").build();
//     match get_token(&mut oauth).await {
//         Some(token_info) => {
//             let client_credential = SpotifyClientCredentials::default()
//                 .token_info(token_info)
//                 .build();
//             let spotify = Spotify::default()
//                 .client_credentials_manager(client_credential)
//                 .build();
//             // Reference: https://developer.spotify.com/documentation/web-api/reference/search/search/
//             let result = spotify
//                 .search(
//                     // q: 検索キーワード
//                     "DA PUMP",
//                     // type: 検索タイプ（album, artist, playlist, track, etc.）
//                     SearchType::Artist,
//                     // limit: 取得するデータの最大数
//                     1,
//                     // offset: 取得するデータの開始位置
//                     0,
//                     // market: 指定した国名コードで再生可能なコンテンツのみを返す
//                     Some(Country::Japan),
//                     // include_external: `audio` を指定すると関連するオーディオコンテンツがレスポンスに含まれる
//                     None,
//                 )
//                 .await
//                 .unwrap();

//             match result {
//                 rspotify::model::search::SearchResult::Artists(artists) => {
//                     let artist = &artists.items[0];
//                     println!("artist: {}\nid: {}\npopularity: {}", artist.name, artist.id, artist.popularity);
//                 },
//                 err => println!("search error!{:?}", err),
//             }
//         }
//         None => println!("auth failed"),
//     };
// }

use rspotify::client::Spotify;
use rspotify::model::offset::for_position;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::util::get_token;

#[tokio::main]
async fn main() {
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
                    Some(vec!["spotify:track:623pmkD6sclgLBQrrPqyz4".to_owned()]),
                    // offset: album, playlist, track の再生を開始する位置
                    for_position(0),
                    // position_ms: 再生を開始する位置
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