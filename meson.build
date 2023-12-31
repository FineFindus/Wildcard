project(
  'wildcard',
  'rust',
  version: '0.1.0',
  meson_version: '>= 0.62.0',
  license: 'GPL-3.0-or-later',
)

i18n = import('i18n')
gnome = import('gnome')

base_id = 'io.github.fkinoshita.Wildcard'

dependency('gtk4', version: '>=4.10')
dependency('libadwaita-1', version: '>=1.3')

glib_compile_resources = find_program('glib-compile-resources', required: true)
glib_compile_schemas = find_program('glib-compile-schemas', required: true)
desktop_file_validate = find_program('desktop-file-validate', required: false)
appstreamcli = find_program('appstreamcli', required: false)
cargo = find_program('cargo', required: true)

version = meson.project_version()

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
localedir = prefix / get_option('localedir')

datadir = prefix / get_option('datadir')
pkgdatadir = datadir / meson.project_name()
iconsdir = datadir / 'icons'
podir = meson.project_source_root() / 'po'
gettext_package = meson.project_name()

# Profiles
if get_option('profile') == 'development'
    profile = 'Devel'
    application_id = '@0@.@1@'.format(base_id, profile)

    find_program('git')
    rev_txt = run_command('git', 'rev-parse', '--short', 'HEAD').stdout().strip()
    version = meson.project_version() + '-' + rev_txt
else
    application_id = base_id
    profile = 'Default'
    version = meson.project_version()
endif

meson.add_dist_script(
  'build-aux/dist-vendor.sh',
  meson.project_build_root() / 'meson-dist' / meson.project_name() + '-' + version,
  meson.project_source_root()
)

subdir('data')
subdir('po')
subdir('src')

gnome.post_install(
  gtk_update_icon_cache: true,
  glib_compile_schemas: true,
  update_desktop_database: true,
)
