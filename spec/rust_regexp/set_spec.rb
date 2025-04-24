# frozen_string_literal: true

RSpec.describe RustRegexp::Set do
  let(:patterns) { ["abc", "def", "xyz"] }
  let(:set) { described_class.new(patterns) }

  describe ".new" do
    it "returns a compiled set" do
      expect(set).to be_a(described_class)
    end
  end

  describe "#match" do
    it "returns indices of matched patterns" do
      expect(set.match("abcdef")).to eq [0, 1]
      expect(set.match("defabc")).to eq [0, 1]
      expect(set.match("123")).to eq []
    end
  end

  describe "#match?" do
    it "checks whether set has at least one pattern matched" do
      expect(set.match?("abc")).to eq true
      expect(set.match?("123")).to eq false
    end
  end

  describe "#patterns" do
    it "returns original regular expression patterns" do
      expect(set.patterns).to eq patterns
    end
  end
end
