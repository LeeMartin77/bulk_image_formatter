ls ./tests/assets | xargs -I@ printf "./tests/assets/@:./target/output/@\n" | cargo run