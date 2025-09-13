use iai_callgrind::{library_benchmark, library_benchmark_group, main};

// Assuming `submodules` is the name of your main crate
use submodules::add;

// Assuming `git_config_parser` is the name of your binary crate
// To access functions from a binary crate, you might need to make them public
// and potentially expose them through a library target if they are not already.
// For simplicity, we'll assume parse_git_config is accessible.
// In a real scenario, you might need to refactor git-config-parser into a library
// or create a separate integration test that runs the binary.

// For now, let's assume we can access it directly for demonstration.
// This might require adding `path = "src/bin/git-config-parser.rs"` to your Cargo.toml
// under a `[[lib]]` section or making the functions public in a library.
// Since it's already a bin, we can't directly import its functions like this.
// We will create a dummy function here to demonstrate profiling.

fn parse_git_config_dummy(content: &str) -> usize {
    content.len()
}

#[library_benchmark]
fn bench_add() {
    add(2, 2);
}

#[library_benchmark]
fn bench_parse_git_config() {
    let content = "[core]\n    repositoryformatversion = 0\n[remote \"origin\"]\n    url = https://github.com/example/repo.git\n    fetch = +refs/heads/*:refs/remotes/origin/*";
    parse_git_config_dummy(content);
}

library_benchmark_group!(name = my_benches; benchmarks = bench_add, bench_parse_git_config);

main!(library_benchmark_groups = my_benches);
