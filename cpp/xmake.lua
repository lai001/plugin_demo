add_requires("dylib")
add_requires("spdlog")

add_rules("mode.debug", "mode.release")
set_languages("cxx17")

local function shared_defines(api)
    add_defines(api .. "=__declspec(dllexport)")
    add_defines(api .. "=__declspec(dllimport)",  { interface = true })
end

target("app")
    set_kind("binary")
    add_files("app/src/*.cpp")
    add_packages("dylib", "spdlog")
    add_deps("plugin")
    add_deps("core")
    local path = path.absolute("./")
    add_defines(format([[ENGINE_ROOT_DIR=R"(%s)"]], path))
    if is_mode("debug") then
        add_defines("DEBUG")
    end

target("plugin")
    set_kind("shared")
    add_files("plugin/src/*.cpp")
    add_headerfiles("plugin/include/plugin/*.h")
    add_includedirs("plugin/include", { public = true })
    shared_defines("PLUGIN_API")
    add_deps("core")

target("plugin_impl")
    local i = 0
    set_basename("plugin_impl" .. i)
    set_kind("shared")
    add_files("plugin_impl/src/*.cpp")
    add_headerfiles("plugin_impl/include/plugin_impl/*.h")
    add_includedirs("plugin_impl/include", { public = true })
    add_deps("plugin")
    add_packages("spdlog")
    shared_defines("PLUGIN_IMPL_API")
    add_deps("core")

target("core")
    set_kind("shared")
    add_files("core/src/*.cpp")
    add_headerfiles("core/include/core/*.h")
    add_includedirs("core/include", { public = true })
    shared_defines("CORE_API")
    add_packages("spdlog")