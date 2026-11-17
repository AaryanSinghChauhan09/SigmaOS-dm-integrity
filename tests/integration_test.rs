// Integration test for SigmaOS-dm-integrity
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use dm_integrity as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
