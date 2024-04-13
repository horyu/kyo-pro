(0..16).each do |l|
  ((l + 1)..16).each do |r|

    puts "@#{l} #{r}", `echo #{l} #{r} | cargo run -q --bin abc349-d`
  end
end
