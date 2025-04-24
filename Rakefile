# frozen_string_literal: true

require "rake/extensiontask"

Rake::ExtensionTask.new("rust_regexp") do |c|
  c.lib_dir = "lib/rust_regexp"
end

require "rspec/core/rake_task"
RSpec::Core::RakeTask.new(:spec)
