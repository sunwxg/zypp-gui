subdir('icons')
subdir('resources')
# Desktop file
desktop_conf = configuration_data()
desktop_conf.set('icon', application_id)
desktop_file = i18n.merge_file(
  type: 'desktop',
  input: configure_file(
    input: '@0@.desktop.in.in'.format(base_id),
    output: '@BASENAME@',
    configuration: desktop_conf
  ),
  output: '@0@.desktop'.format(application_id),
  po_dir: podir,
  install: true,
  install_dir: datadir / 'applications'
)
# Validate Desktop file
if desktop_file_validate.found()
  test(
    'validate-desktop',
    desktop_file_validate,
    args: [
      desktop_file.full_path()
    ],
    depends: desktop_file,
  )
endif

# Appdata
appdata_conf = configuration_data()
appdata_conf.set('app-id', application_id)
appdata_conf.set('gettext-package', gettext_package)
appdata_file = i18n.merge_file(
  input: configure_file(
    input: '@0@.metainfo.xml.in.in'.format(base_id),
    output: '@BASENAME@',
    configuration: appdata_conf
  ),
  output: '@0@.metainfo.xml'.format(application_id),
  po_dir: podir,
  install: true,
  install_dir: datadir / 'metainfo'
)
# Validate Appdata
if appstream_util.found()
  test(
    'validate-appdata', appstream_util,
    args: [
      'validate', '--nonet', appdata_file.full_path()
    ],
    depends: appdata_file,
  )
endif

# GSchema
gschema_conf = configuration_data()
gschema_conf.set('app-id', application_id)
gschema_conf.set('gettext-package', gettext_package)
configure_file(
  input: '@0@.gschema.xml.in'.format(base_id),
  output: '@0@.gschema.xml'.format(application_id),
  configuration: gschema_conf,
  install: true,
  install_dir: datadir / 'glib-2.0' / 'schemas'
)

# Validata GSchema
if glib_compile_schemas.found()
  test(
    'validate-gschema', glib_compile_schemas,
    args: [
      '--strict', '--dry-run', meson.current_build_dir()
    ],
  )
endif

# Policy file
policy_conf = configuration_data()
policy_conf.set('path', bindir)
policy_conf.set('icon', application_id)
policy_file = i18n.merge_file(
  input: configure_file(
    input: '@0@.policy.in.in'.format(base_id),
    output: '@BASENAME@',
    configuration: policy_conf
  ),
  output: '@0@.policy'.format(application_id),
  po_dir: podir,
  install: true,
  install_dir: datadir / 'polkit-1' / 'actions'
)

# Service file
service_conf = configuration_data()
service_conf.set('app_id', application_id)
service_conf.set('bindir', bindir)
configure_file(
  input: '@0@.service.in'.format(base_id),
  output: '@0@.service'.format(application_id),
  configuration: service_conf,
  install: true,
  install_dir: datadir / 'dbus-1' / 'services'
)

# Auto start Service file
auto_conf = configuration_data()
auto_conf.set('bindir', bindir)
configure_file(
  input : 'zypp-gui-service.desktop.in',
  output : 'zypp-gui-service.desktop',
  configuration: auto_conf,
  install: true,
  install_dir: join_paths(get_option('sysconfdir'), 'xdg/autostart'),
)

# Mirror sites data file
mirror_conf = configuration_data()
mirror_conf.set('bindir', bindir)
configure_file(
  input : 'mirror.json.in',
  output : 'mirror.json',
  configuration: mirror_conf,
  install: true,
  install_dir: pkgdatadir,
)
