project('gst-plugins-rs',
        'rust',
        'c',
        version: '0.9.0-alpha.1',
        meson_version : '>= 0.60')

python = import('python').find_installation()
fs = import('fs')

if get_option('debug')
  target = 'debug'
else
  target = 'release'
endif

cargo = find_program('cargo', version:'>=1.40')
cargo_wrapper = find_program('cargo_wrapper.py')
cargo_c = find_program('cargo-cbuild', version:'>=0.9.3', required: false)
rustc = find_program('rustc', version:'>=1.52')

if not cargo_c.found()
  error('cargo-c missing, install it with: \'cargo install cargo-c\'')
endif

system = build_machine.system()
ext_exe = ''
if system == 'windows'
  ext_exe = 'exe'
  ext_dynamic = 'dll'
  ext_static = 'lib'
elif system == 'darwin'
  ext_dynamic = 'dylib'
  ext_static = 'a'
else
  ext_dynamic = 'so'
  ext_static = 'a'
endif

# workspace name -> lib name
# kept in the same order as the `members` list in Cargo.toml
plugins = {
  'rtmpserver': 'libgstrtmpserver',
}

extra_env = {}

cc = meson.get_compiler('c')


output = []

extensions = []

# Add the plugin file as output
if get_option('default_library') == 'shared' or get_option('default_library') == 'both'
  extensions += [ext_dynamic]
  foreach p, lib : plugins
    output += [lib + '.' + ext_dynamic]
  endforeach
endif

static_build = get_option('default_library') == 'static'
if static_build or get_option('default_library') == 'both'
  extensions += [ext_static]
  foreach p, lib : plugins
    output += [lib + '.' + ext_static]
  endforeach
endif

# Need to depends on all gstreamer-rs deps to ensure they are built
# before gstreamer-rs when building with gst-build.
# Custom targets can't depend on dependency() objects so we have to depend
# on the library variable from the subproject instead.
gst_req = '>= 1.18.0'
depends = []

deps = [
  # name, subproject name, subproject dep, library object
  ['gstreamer-1.0', 'gstreamer', 'gst_dep', 'libgst'],
  ['gstreamer-base-1.0', 'gstreamer', 'gst_base_dep', 'gst_base'],
]

# Used to not lookup the same dependency multiple times which clutters logs
deps_cache = {}

foreach d: deps
  dep = dependency(d[0], version : gst_req,
    fallback : [d[1], d[2]])
  set_variable(d[2], dep)
  deps_cache += {d[0]: dep}
  if dep.type_name() == 'internal'
    lib = subproject(d[1]).get_variable(d[3])
    depends += lib
  endif
endforeach

include = ','.join(plugins.keys())

# serialize extra_env
extra_env_list = []
foreach key, value : extra_env
  extra_env_list += key + ':' + value
endforeach
extra_env_str = ','.join(extra_env_list)

plugins_install_dir = get_option('libdir') / 'gstreamer-1.0'
pkgconfig_install_dir = get_option('libdir') / 'pkgconfig'

if get_option('doc').disabled()
  disable_doc = ['--disable-doc']
else
  disable_doc = []
endif

rs_studio_plugins = custom_target('gst-plugins-studio-rs',
  build_by_default: true,
  output: output,
  console: true,
  install: true,
  install_dir: plugins_install_dir,
  depends: depends,
  depfile: 'gst-plugins-studio-rs.dep',
  command: [cargo_wrapper,
    'build',
    meson.current_build_dir(),
    meson.current_source_dir(),
    meson.global_build_root(),
    target,
    include,
    extra_env_str,
    get_option('prefix'),
    get_option('libdir'),
    '--depfile', '@DEPFILE@',
    '--exts', extensions,
  ] + disable_doc)

plugins = rs_studio_plugins.to_list()

# This is used by GStreamer to static link Rust plugins into gst-full
gst_plugins = []
pc_files = []
plugin_names = []
foreach plugin : plugins
  # skip the 'lib' prefix and extension from plugin path
  plugin_name = fs.stem(plugin.full_path()).substring(3)

  # Extract plugin dependencies from their Cargo.toml file
  plugin_deps = []
  p = run_command('dependencies.py', meson.current_source_dir(), plugin_name,
    capture: true,
    check: true)
  foreach dep_name : p.stdout().strip().split(',')
    dep_name_version = dep_name.split('|')
    dep_name = dep_name_version.get(0)
    if dep_name_version.length() > 1
      extras = {'version': dep_name_version.get(1)}
    else
      extras = {}
    endif
    if deps_cache.has_key(dep_name)
      plugin_deps += deps_cache[dep_name]
    else
      dep = dependency(dep_name, required: false, kwargs: extras)
      plugin_deps += dep
      deps_cache += {dep_name: dep}
    endif
  endforeach

  dep = declare_dependency(
    link_with: plugin,
    dependencies: plugin_deps,
    variables: {'full_path': plugin.full_path()},
  )
  meson.override_dependency(plugin_name, dep)

  if static_build and plugin_name in ['gstcsound', 'gstthreadshare']
    warning('Static plugin @0@ is known to fail. It will not be included in libgstreamer-full.'.format(plugin_name))
  else
    gst_plugins += dep

    pc_files += [plugin_name + '.pc']
    if plugin_name.startswith('gst')
      plugin_names += [plugin_name.substring(3)]
    else
      plugin_names += [plugin_name]
    endif
  endif
endforeach

subdir('docs')

# We don't need to pass a command as we depends on the target above
# but it is currently mandatory ( https://github.com/mesonbuild/meson/issues/8059 )
# so use python as it's guaranteed to be present on any setup
custom_target('gst-plugins-studio-rs-pc-files',
  build_by_default: true,
  output: pc_files,
  console: true,
  install: true,
  install_dir: pkgconfig_install_dir,
  depends: rs_studio_plugins,
  command: [python, '-c', '""'])


test('tests',
  cargo_wrapper,
  args: ['test',
    meson.current_build_dir(),
    meson.current_source_dir(),
    meson.global_build_root(),
    target,
    include,
    extra_env_str,
    get_option('prefix'),
    get_option('libdir')],
  timeout: 600)

summary({
    'Plugins': plugin_names,
}, list_sep: ', ')
