extern crate rspotify;

use rspotify::client::Spotify;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::senum::{Country, SearchType};
use rspotify::util::get_token;

#[tokio::main]
async fn main() {
    let mut oauth = SpotifyOAuth::default().scope("user-read-private").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();

            // Reference: https://developer.spotify.com/documentation/web-api/reference/search/search/
            let result = spotify
                .search(
                    // q: 検索キーワード
                    "DA PUMP",
                    // type: 検索タイプ（album, artist, playlist, track, etc.）
                    SearchType::Artist,
                    // limit: 取得するデータの最大数
                    1,
                    // offset: 取得するデータの開始位置
                    0,
                    // market: 指定した国名コードで再生可能なコンテンツのみを返す
                    Some(Country::Japan),
                    // include_external: audio を指定すると関連するオーディオコンテンツがレスポンスに含まれる
                    None,
                )
                .await
                .unwrap();

            match result {
                rspotify::model::search::SearchResult::Artists(artists) => {
                    let artist = &artists.items[0];
                    println!("artist: {}\nid: {}\npopularity: {}", artist.name, artist.id, artist.popularity);
                },
                err => println!("search error!{:?}", err),
            }
        }
        None => println!("auth failed"),
    };
}