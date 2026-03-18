for running with local build directly (no install)
    cd /home/hugo/rust/rust_solution/catr
    ./target/debug/catr -n tests/inputs/empty.txt tests/inputs/fox.txt tests/inputs/spiders.txt tests/inputs/the-bustle.txt
    cargo test
    cargo run -n
    cargo run -b


for install as directory
    cargo install --path .
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
    source ~/.bashrc
    test: 
        catr --help
    run:
        catr -n fox.txt spiders.txt the-bustle.txt