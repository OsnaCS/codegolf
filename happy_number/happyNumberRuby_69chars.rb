# @author Tobias Muench <tobias.muench@hs-osnabrueck.de>
# Codegolf Happy Numbers in Ruby with 69 characters.
# If you want to run use bundle install and rspec
class HappyNumber

  # Solution I
  # 101 Characters
  # Not Optimized - same Code as Solution II but more readable
  def f_long(n)
    x=0
    n.to_s.each_char{|c|
      x+=c.to_i**2
    }
    x<=1||x==4?x%4:f(x)
  end

  # Solution II
  # 69 Characters
  def f(n);x=0;n.to_s.each_char{|c|x+=c.to_i**2};x<=1||x==4?x%4:f(x)end

end


#output
happy = HappyNumber.new
(0..100).each {|i| puts "%d -> %s" %[i, happy.f(i).to_s]}
