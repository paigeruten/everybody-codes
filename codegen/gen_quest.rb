require 'fileutils'

TEMPLATE_PATH = "#{__dir__}/quest_template.rs"
SRC_PATH = "#{__dir__}/../src"
NOTES_PATH = "#{__dir__}/../notes"
QUESTS_PATH = "#{SRC_PATH}/quests.rs"

QUESTS_TEMPLATE = <<-TEMPLATE
use std::io::BufRead;

use crate::common::{Config, Part};

{{modules}}

pub const NUM_QUESTS: usize = {{num_quests}};

pub fn solve(
    quest_number: usize,
    part: Part,
    input: impl BufRead,
    config: &Config,
) -> color_eyre::Result<String> {
    match quest_number {
{{match_arms}}
        _ => Err(color_eyre::eyre::eyre!(
            "That quest has not been solved yet."
        )),
    }
}
TEMPLATE

quest_num = ARGV.first.to_i
quest_num_padded = "%02d" % quest_num

template = File.read(TEMPLATE_PATH)

content = template.gsub('{{quest_num_padded}}', quest_num_padded)
target_path = "#{SRC_PATH}/quests/quest#{quest_num_padded}.rs"

(1..3).each do |part_num|
  FileUtils.touch("#{NOTES_PATH}/q#{quest_num_padded}p0#{part_num}")
  puts "\x1b[1mTouched\x1b[0m  notes/q#{quest_num_padded}p0#{part_num}"
end

if File.exist? target_path
  puts "Error: File '#{target_path}' already exists"
  exit 1
end

File.write(target_path, content)
puts "\x1b[1mCreated\x1b[0m  src/quests/quest#{quest_num_padded}.rs"

existing_quests = Dir["#{SRC_PATH}/quests/quest??.rs"].map do |path|
  File.basename(path, ".rs")[5..6]
end

num_quests = existing_quests.map(&:to_i).max
if existing_quests.map(&:to_i).sort != (1..num_quests).to_a
  puts "Error: Quests do not consecutively increase from 1"
  exit 1
end

quests_modules = existing_quests.map { |quest_num| "mod quest#{quest_num};"}.join("\n")
quests_match_arms = existing_quests.map do |quest_num|
  "        #{quest_num.to_i} => quest#{quest_num}::solve(part, input, config),"
end.join("\n")

quests_content = QUESTS_TEMPLATE
  .sub('{{num_quests}}', existing_quests.map(&:to_i).max.to_s)
  .sub('{{modules}}', quests_modules)
  .sub('{{match_arms}}', quests_match_arms)

File.write(QUESTS_PATH, quests_content)
puts "\x1b[1mUpdated\x1b[0m  src/quests.rs"

