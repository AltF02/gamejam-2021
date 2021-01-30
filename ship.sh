cargo build --release --no-default-features
pwd
rm -rf build/linux.zip
mkdir build/linux
cp ./release/gamejam-2021 build/linux
cp -avr ./assets ./build/linux
zip -r build/linux.zip ./build/linux
#rm -rf ./build/linux