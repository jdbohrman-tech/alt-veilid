#
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint veilid.podspec` to validate before publishing.
#
Pod::Spec.new do |s|
  s.name             = 'veilid'
  s.version          = '0.0.1'
  s.summary          = 'Veilid Network'
  s.description      = <<-DESC
Veilid Network Plugin
                       DESC
  s.homepage         = 'http://veilid.com'
  s.license          = 'LGPL-2.0-or-later OR MPL-2.0 OR (MIT AND BSD-3-Clause)'
  s.author           = { 'John Smith' => 'jsmith@example.com' }
  s.source           = { :path => '.' }
  s.source_files = 'Classes/**/*'
  s.dependency 'Flutter'
  s.platform = :ios, '9.0'

  # Flutter.framework does not contain a i386 slice.
  s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES', 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386' }
  s.swift_version = '5.0'

  require 'json'
  require 'pathname'

  script_dir = File.realpath(File.expand_path(__dir__))
  workspace_dir = File.dirname(JSON.parse(`(cd #{script_dir}; cargo locate-project --workspace)`)['root'])
  cargo_target_dir = File.join(workspace_dir, 'target')

  s.xcconfig = { 
    'OTHER_LDFLAGS' => "-Wl,-force_load,#{File.join(cargo_target_dir, 'lipo-ios', 'libveilid_flutter.a')}",
    "LIBRARY_SEARCH_PATHS" => File.join(cargo_target_dir, 'lipo-ios')
  }

  s.script_phase = { 
    :name => 'Cargo Build', 
    :script => File.join(workspace_dir, 'scripts', 'ios_build.sh') + ' veilid_flutter', 
    :execution_position => :before_compile
    # :output_files => [ File.join(cargo_target_dir, 'lipo-ios', 'libveilid_flutter.a') ]
  }

end
