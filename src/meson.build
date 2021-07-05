global_conf = configuration_data()
global_conf.set_quoted('APP_ID', application_id)
global_conf.set_quoted('PKGDATADIR', pkgdatadir)
global_conf.set_quoted('PROFILE', profile)
global_conf.set_quoted('VERSION', version + version_suffix)
global_conf.set_quoted('GETTEXT_PACKAGE', gettext_package)
global_conf.set_quoted('LOCALEDIR', localedir)
config = configure_file(
  input: 'config.rs.in',
  output: 'config.rs',
  configuration: global_conf
)
# Copy the config.rs output to the source directory.
run_command(
  'cp',
  meson.build_root() / 'src' / 'config.rs',
  meson.source_root() / 'src' / 'config.rs',
  check: true
)

sources = files(
  'config.rs',
  'config.rs.in',
  'connect_packagekit.rs',
  'list_row.rs',
  'main.rs',
  'mirror.rs',
  'mirror_row.rs',
  'mirror_window.rs',
  'notification.rs',
  'package_meta.rs',
  'packagekit.rs',
  'page_settings.rs',
  'repo_row.rs',
  'search.rs',
  'search_row.rs',
  'additional.rs',
  'util.rs',
  'window.rs',
  'zypper.rs',
)

custom_target(
  'cargo-build',
  build_by_default: true,
  input: sources,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: bindir,
  depends: resources,
  command: [
    cargo_script,
    meson.build_root(),
    meson.source_root(),
    '@OUTPUT@',
    profile,
	'opensuse_software',
  ]
)
