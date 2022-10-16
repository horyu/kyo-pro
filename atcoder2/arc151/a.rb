arr = [0, 1]
(1..4).each do |n|
    vv = arr.repeated_permutation(n).to_a
    len = vv.size
    (0...len).each do |i|
        ((i + 1)...len).each do |j|
            s = vv[i].join()
            t = vv[j].join()
            arg = "#{n} #{s} #{t}"
            rs = `echo #{arg}| cargo run --bin arc151-a --quiet`.chomp
            add = ""
            if rs != "-1"
                ds = dt = 0
                (0..n).each do |k|
                    if s[k] != t[k]
                        if rs[k] != s[k]
                            ds += 1
                        else
                            dt += 1
                        end
                    end
                end
                add = "ds:#{ds} dt:#{dt}"
            end
            puts arg + " :" + rs + "\t#{add}"
        end
    end
    # arr.repeated_permutation(n).each do |s|
    #     arr.repeated_permutation(n).each do |t|
    #         arg = "#{n} #{s.join()} #{t.join()}"
    #         puts arg + " :" + `echo #{arg}| cargo run --bin arc151-a --quiet`

    #         # p s.join(), t.join()
    #     end
    # end
end
# `cargo run --bin arc151-a`
