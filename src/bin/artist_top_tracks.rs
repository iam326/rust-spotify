extern crate rspotify;

use std::env;
use rspotify::client::Spotify;
use rspotify::oauth2::SpotifyClientCredentials;
use rspotify::senum::Country;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: cargo run --bin artist_top_tracks <ARTIST_ID>");
        return;
    }
    let artist_id = &args[1];

    let client_credential = SpotifyClientCredentials::default().build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();

    // Reference: https://developer.spotify.com/documentation/web-api/reference/artists/get-artists-top-tracks/
    let result = spotify
        .artist_top_tracks(
            // id: artist の Spotify ID
            artist_id,
            // country: 国名コード
            Country::Japan
        )
        .await
        .unwrap();
    
    println!("# {} Top Tracks", result.tracks[0].artists[0].name);
    for track in result.tracks.iter() {
        println!("{}: {} ({})", track.name, track.popularity, track.id.as_deref().unwrap_or("none"));
    }
}