require "alphred/tasks"

task :release do
  require_relative 'version'
  Rake::Task['alphred:release'].invoke("v#{VERSION}")
end
