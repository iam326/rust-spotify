extern crate rspotify;

use rspotify::client::Spotify;
use rspotify::oauth2::SpotifyClientCredentials;
use rspotify::senum::Country;

#[tokio::main]
async fn main() {
    let client_credential = SpotifyClientCredentials::default().build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    let result = spotify
        .artist_top_tracks(birdy_uri, Country::UnitedStates)
        .await
        .unwrap();
    
    println!("# {} Top Tracks", result.tracks[0].artists[0].name);
    for track in result.tracks.iter() {
        println!("{0}: {1}", track.name, track.popularity);
    }
}
