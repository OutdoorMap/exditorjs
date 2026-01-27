import Config

config :exditorjs, json_library: JSON
config :rustler_precompiled, :force_build_all, exditorjs: true
