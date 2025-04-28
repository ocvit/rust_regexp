# RustRegexp

[![Gem Version](https://badge.fury.io/rb/rust_regexp.svg)](https://badge.fury.io/rb/rust_regexp)
[![Test](https://github.com/ocvit/rust_regexp/workflows/CI/badge.svg)](https://github.com/ocvit/rust_regexp/actions)

Ruby bindings for [rust/regex](https://docs.rs/regex/latest/regex/) library.

## Installation

Install [Rust](https://www.rust-lang.org/) via [rustup](https://rustup.rs/) or in any other way.

Add as a dependency:

```ruby
# In your Gemfile
gem "rust_regexp"

# Or without Bundler
gem install rust_regexp
```

Include in your code:

```ruby
require "rust_regexp"
```

## Usage

Regular expressions should be pre-compiled before use:

```ruby
re = RustRegexp.new('p.t{2}ern*')
# => #<RustRegexp:...>
```

> [!TIP]
> Note the use of *single quotes* when passing the regular expression as
> a string to `rust/regex` so that the backslashes aren't interpreted as escapes.

To find a single match in the haystack:

```ruby
RustRegexp.new('\w+:\d+').match("ruby:123, rust:456")
# => ["ruby:123"]

RustRegexp.new('(\w+):(\d+)').match("ruby:123, rust:456")
# => ["ruby", "123"]
```

To find all matches in the haystack:

```ruby
RustRegexp.new('\w+:\d+').scan("ruby:123, rust:456")
# => ["ruby:123", "rust:456"]

RustRegexp.new('(\w+):(\d+)').scan("ruby:123, rust:456")
# => [["ruby", "123"], ["rust", "456"]]
```

To check whether there is at least one match in the haystack:

```ruby
RustRegexp.new('\w+:\d+').match?("ruby:123")
# => true

RustRegexp.new('\w+:\d+').match?("ruby")
# => false
```

Inspect original pattern:

```ruby
RustRegexp.new('\w+:\d+').pattern
# => "(\\w+):(\\d+)"
```

> [!WARNING]
> `rust/regex` regular expression syntax differs from Ruby's built-in
> [`Regexp`](https://docs.ruby-lang.org/en/3.4/Regexp.html) library, see the
> [official syntax page](https://docs.rs/regex/latest/regex/index.html#syntax) for more
> details.

### Searching simultaneously

`RustRegexp::Set` represents a collection of
regular expressions that can be searched for simultaneously. Calling `RustRegexp::Set#match` will return an array containing the indices of all the patterns that matched.

```ruby
set = RustRegexp::Set.new(["abc", "def", "ghi", "xyz"])

set.match("abcdefghi") # => [0, 1, 2]
set.match("ghidefabc") # => [0, 1, 2]
```

> [!NOTE]
> Matches arrive in the order the constituent patterns were declared,
> not the order they appear in the haystack.

To check whether at least one pattern from the set matches the haystack:

```ruby
RustRegexp::Set.new(["abc", "def"]).match?("abc")
# => true

RustRegexp::Set.new(["abc", "def"]).match?("123")
# => false
```

Inspect original patterns:

```ruby
RustRegexp::Set.new(["abc", "def"]).patterns
# => ["abc", "def"]
```

## Encoding

Currently, `rust_regexp` expects the haystack to be an UTF-8 string.

It also supports parsing of strings with invalid UTF-8 characters by default. It's achieved via using `regex::bytes` instead of plain `regex` under the hood, so any byte sequence can be matched. The output match is encoded as UTF-8 string.

In case unicode awarness of matchers should be disabled, both `RustRegexp` and `RustRegexp::Set` support `unicode: false` option:

```ruby
RustRegexp.new('\w+').match('ю٤夏')
# => ["ю٤夏"]

RustRegexp.new('\w+', unicode: false).match('ю٤夏')
# => []

RustRegexp::Set.new(['\w', '\d', '\s']).match("ю٤\u2000")
# => [0, 1, 2]

RustRegexp::Set.new(['\w', '\d', '\s'], unicode: false).match("ю٤\u2000")
# => []
```

## Development

```sh
bin/setup     # install deps
bin/console   # interactive prompt to play around
rake compile  # (re)compile extension
rake spec     # run tests
```

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/ocvit/rust_regexp.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
