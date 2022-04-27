# n = 14
# l = 2
# r = 100

# p n.to_s(2)
# (l..r).each do |i|
#   p [i.to_s(2), i] if n ^ i < n
# end

(1..31).each do |n|
  (1..64).each do |l|
    (l..64).each do |r|
      expect = (l..r).filter{ n ^ _1 < n}.count
      real = `echo #{n} #{l} #{r} | ../target/debug/a`.chomp.to_i
      if expect != real
        p [n, l, r], [expect, real]
        exit
      end
    end
  end
end
