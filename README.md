# rust-spotify

[rspotify](https://github.com/ramsayleung/rspotify) を使用しています。下記を試してみました。

- アーティストの検索
- 指定したアーティストの楽曲 TOP10 を表示する
- 指定した楽曲を再生する

## Environment

```
$ sw_vers
ProductName:    Mac OS X
ProductVersion: 10.15.6
BuildVersion:   19G2021

$ cargo version
cargo 1.46.0 (149022b1d 2020-07-17)

$ rustc --version
rustc 1.46.0 (04488afe3 2020-08-24)
```

## Usage

```
// アーティストの情報を検索する
$ cargo run --bin search_artist <ARTIST_NAME>

// 指定したアーティストの楽曲人気 TOP10 を表示する
$ cargo run --bin artist_top_tracks <ARTIST_ID>

// 指定した楽曲を再生する
$ cargo run --bin play_track <TRACK_ID>
```
