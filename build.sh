#!/bin/bash

cargo build --release

rm -rf release && mkdir release

cp -r resources/ ./release/resources
cp target/release/lap_tap ./release/lap_tap
cp target/release/lap_tap_audio ./release/lap_tap_audio

zip -r lap_tap.zip ./release
