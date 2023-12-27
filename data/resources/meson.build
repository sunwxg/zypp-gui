blueprints = custom_target('blueprints',
  input: files(
    'ui/additional_row.blp',
    'ui/mirror_row.blp',
    'ui/mirror_window.blp',
    'ui/repo_add.blp',
    'ui/repo_row.blp',
    'ui/search_row.blp',
    'ui/list_row.blp',
    'ui/window.blp',
    'ui/page_settings.blp',
    ),
  output: '.',
  command: [find_program('blueprint-compiler'), 'batch-compile', '@OUTPUT@', '@CURRENT_SOURCE_DIR@', '@INPUT@'],
  )

# Resources
resources = gnome.compile_resources(
  'resources',
  'resources.gresource.xml',
  gresource_bundle: true,
  source_dir: meson.current_build_dir(),
  install: true,
  install_dir: pkgdatadir,
  dependencies: blueprints,
)

