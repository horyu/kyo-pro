(1..6).to_a().permutation(6) do |vv|
    puts `echo 3 #{vv.join(" ")} | cargo run -q --bin agc058-a`
end
