
sources = files(
  'main.rs',
)

custom_target(
  'cargo-build',
  build_by_default: true,
  input: sources,
  output: 'mod-repo',
  console: true,
  install: true,
  install_dir: bindir,
  command: [
    cargo_script,
    meson.build_root() / 'mod_repo',
    meson.source_root() / 'mod_repo',
    '@OUTPUT@',
    profile,
    'mod-repo',
  ]
)
