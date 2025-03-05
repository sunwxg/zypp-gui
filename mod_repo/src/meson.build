
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
    meson.build_root(),
    meson.source_root(),
    '@OUTPUT@',
    profile,
    'mod-repo',
    'mod_repo',
  ]
)
