profiling a unit test:
cargo flamegraph --unit-test -- tests::<name_of_test>




cargo flamegraph --release --              m


cargo run --release -- --input "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\11820-60\11820-60 L2 C.txt" --output "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\11820-60\11820-60 L2 C_clusters.csv" --amplitude-threshold 10000
cargo run --release -- --input "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\11805\11805-92 L1 OS.txt" --output "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\11805\11805-92 L1 OS.csv" --amplitude-threshold 10000



cargo flamegraph --release -- --input "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\slow\11820-11 HS C.txt" --output "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\slow\11820-11 HS C.csv"




cargo flamegraph --release -- --input "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\11820-60\11820-60 L2 C.txt" --output "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\11820-60\11820-60 L2 C_clusters.csv" --amplitude-threshold 25000

cargo flamegraph --release -- --input "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\11820-60\11820-60 L2 C.txt" --output "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\11820-60\11820-60 L2 C_clusters_2500_b.csv" --amplitude-threshold 25000


cargo flamegraph --release -- --input "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\11820-60\11820-60 L2 NS.txt" --output "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\11820-60\11820-60 L2 NS_clusters.csv" --amplitude-threshold 30000



cargo flamegraph -- --input "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\11820-60 L2 C.txt" --output "C:\Users\drew.bennett\Documents\3d_gpr_poi_finder\test\11820-60 L2 C_clusters.csv"
