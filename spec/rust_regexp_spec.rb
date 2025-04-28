# frozen_string_literal: true

RSpec.describe RustRegexp do
  it "has a version number" do
    expect(RustRegexp::VERSION).not_to be nil
  end

  describe ".new" do
    it "returns a compiled regexp" do
      re = described_class.new('\w+')
      expect(re).to be_a(described_class)
    end
  end

  describe "#match" do
    examples = [
      ['\w+:\d+', "ruby:123, rust:456", {}, ["ruby:123"]],
      ['(\w+):(\d+)', 'ruby:123, rust:456', {}, ["ruby", "123"]],
      ['(\w+):(\d+)', '123', {}, []],
      ['\w+', "абв", {}, ["абв"]],
      ['\w+', "абв", {unicode: false}, []],
    ]

    examples.each do |pattern, haystack, options, expected_matches|
      context "with pattern: #{pattern.inspect}, haystack: #{haystack.inspect}" do
        it "returns #{expected_matches.inspect}" do
          re = described_class.new(pattern, **options)
          matches = re.match(haystack)

          expect(matches).to eq expected_matches
        end
      end
    end
  end

  describe "#scan" do
    examples = [
      ['\w+:\d+', "ruby:123, rust:456", {}, ["ruby:123", "rust:456"]],
      ['(\w+):(\d+)', 'ruby:123, rust:456', {}, [["ruby", "123"], ["rust", "456"]]],
      ['(\w+):(\d+)', '123', {}, []],
      ['\w:\w', "а:б", {}, ["а:б"]],
      ['\w:\w', "а:б", {unicode: false}, []],
    ]

    examples.each do |pattern, haystack, options, expected_matches|
      context "with pattern: #{pattern.inspect}, haystack: #{haystack.inspect}" do
        it "returns #{expected_matches.inspect}" do
          re = described_class.new(pattern, **options)
          matches = re.scan(haystack)

          expect(matches).to eq expected_matches
        end
      end
    end
  end

  describe "#match?" do
    it "checks whether regexp is matched" do
      re = described_class.new('\d+')

      expect(re.match?("123")).to eq true
      expect(re.match?("abc")).to eq false
    end
  end

  describe "#pattern" do
    it "returns original regular expression pattern" do
      re = described_class.new('\w+')
      expect(re.pattern).to eq '\w+'
    end
  end
end
