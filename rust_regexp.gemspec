# frozen_string_literal: true

require_relative "lib/rust_regexp/version"

Gem::Specification.new do |spec|
  spec.name = "rust_regexp"
  spec.version = RustRegexp::VERSION
  spec.authors = ["Dmytro Horoshko"]
  spec.email = ["electric.molfar@gmail.com"]

  spec.summary = "Simple bindings for rust/regex library"
  spec.description = "Simple bindings to rust/regex library."
  spec.homepage = "https://github.com/ocvit/rust_regexp"
  spec.license = "MIT"
  spec.metadata = {
    "bug_tracker_uri" => "https://github.com/ocvit/rust_regexp/issues",
    "homepage_uri" => "https://github.com/ocvit/rust_regexp",
    "source_code_uri" => "https://github.com/ocvit/rust_regexp"
  }

  spec.files = Dir["lib/**/*.rb", "ext/**/*.{rs,toml,lock,rb}", "README.md", "LICENSE.txt"]
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/rust_regexp/extconf.rb"]

  spec.required_ruby_version = ">= 2.7.0"
end
