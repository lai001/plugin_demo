#include <iostream>
#include <filesystem>
#include <memory>
#include "dylib.hpp"
#include "spdlog/spdlog.h"
#include "plugin/plugin.h"

int main(int argc, char** argv)
{
    int index = 0;
    Core* core = new Core();

    while (true)
    {
        std::cin >> std::string();
        try
        {
#ifdef DEBUG
            dylib* lib = new dylib(std::string(ENGINE_ROOT_DIR) + fmt::format("/build/windows/x64/debug/plugin_impl{}", index));
#else
            dylib* lib = new dylib(std::string(ENGINE_ROOT_DIR) + fmt::format("/build/windows/x64/release/plugin_impl{}", index));
#endif // DEBUG

            Plugin* (*createPlugin)() = lib->get_function<Plugin * ()>("CreatePlugin");
            if (createPlugin)
            {
                Plugin* plugin = createPlugin();
                plugin->tick(core);
                delete plugin;
            }
            delete lib;
            index += 1;
        }
        catch (const dylib::load_error& e) {
            spdlog::error("failed to load 'plugin' library, {}", e.what());
        }
        catch (const dylib::symbol_error& e) {
            spdlog::error("failed to get 'CreatePlugin' symbol, {}", e.what());
        }
    }
    delete core;

    return 0;
}
