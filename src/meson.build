guess_sources = [
  cargo_sources,
  'main.rs',
]

rust_builder = find_program(join_paths(meson.project_source_root(), 'build_aux/build.sh'))

main = custom_target(
    'cargo_build',
    build_by_default: true,
    input: guess_sources,
    output: meson.project_name(),
    console: true,
    install: true,
    install_dir: get_option('bindir'),
    command : [
        rust_builder,
        meson.project_build_root(),
        meson.project_source_root(),
        '@OUTPUT@',
        meson.project_name(),
    ],
)
