# @author Tobias Muench <tobias.muench@hs-osnabrueck.de>
# Is_Prime in Ruby with 41 characters.
# If you want to run use bundle install and rspec

class PrimeCalculator

  # Solution I
  # 83 Characters - same code like Solution II
  def f_long(n)
    (2...n).each do |i|
      return false if n%i==0
    end
    true
  end

  # Solution II
  # 57 Characters
  def f(n);(2...n).each{|i|return false if n%i==0};true;end

  # Solution III
  # A bit recursion - same as IV
  # 62 Characters
  def r_long(n,a=2)
    a>=n || !(n%a==0) && r_long(n,a+1)
  end

  # Solution IV
  # 41 Characters
  def r(n,a=2);a>=n||!(n%a==0)&&r(n,a+1)end
end


#output
calc = PrimeCalculator.new
(0..100).each {|i| puts "%d -> %s" %[i, calc.r(i).to_s]}
