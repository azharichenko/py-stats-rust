cd pyext-statistics
cargo build --release
cd ..
rm *.so
cp pyext-statistics/target/release/libstats.so stats.so