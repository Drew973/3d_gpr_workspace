
set RUST_BACKTRACE=1
cargo flamegraph --release -- --input "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\slow\11820-11 HS C.txt" --output "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\slow\11820-11 HS C.csv"



cargo run --release -- --input "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\slow\11820-11 HS C.txt" --output "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\slow\11820-11 HS C.csv" --overwrite