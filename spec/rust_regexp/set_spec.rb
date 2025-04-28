# frozen_string_literal: true

RSpec.describe RustRegexp::Set do
  describe ".new" do
    it "returns a compiled set" do
      set = described_class.new(["abc", "def", "xyz"])
      expect(set).to be_a(described_class)
    end
  end

  describe "#match" do
    examples = [
      [["abc", "def", "xyz"], "abcdef", {}, [0, 1]],
      [["abc", "def", "xyz"], "defabc", {}, [0, 1]],
      [["abc", "def", "xyz"], "123", {}, []],
      [['\w', '\d', '\s'], "ю٤\u2000", {}, [0, 1, 2]],
      [['\w', '\d', '\s'], "ю٤\u2000", {unicode: false}, []],
    ]

    examples.each do |patterns, haystack, options, expected_indexes|
      context "with patterns: #{patterns.inspect}, haystack: #{haystack.inspect}" do
        it "returns #{expected_indexes.inspect}" do
          set = described_class.new(patterns, **options)
          indexes = set.match(haystack)

          expect(indexes).to eq expected_indexes
        end
      end
    end
  end

  describe "#match?" do
    it "checks whether set has at least one pattern matched" do
      set = described_class.new(["abc", "def", "xyz"])

      expect(set.match?("abc")).to eq true
      expect(set.match?("123")).to eq false
    end
  end

  describe "#patterns" do
    it "returns original regular expression patterns" do
      patterns = ["abc", "def", "xyz"]
      set = described_class.new(patterns)

      expect(set.patterns).to eq patterns
    end
  end
end
