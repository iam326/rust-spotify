# rust-spotify

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
// アーティストを検索する
$ cargo run --bin search_artist

// 指定したアーティストの人気 TOP10 を表示する
$ cargo run --bin artist_top_tracks

// 指定した楽曲を再生する
$ cargo run --bin play_track
```
