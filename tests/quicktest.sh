ls ./tests/assets | xargs printf "./tests/assets/%s\n" | cargo run