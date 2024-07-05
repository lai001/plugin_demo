#include "plugin_impl/plugin_impl.h"
#include "spdlog/spdlog.h"


PluginImpl::PluginImpl()
{
}

PluginImpl::~PluginImpl()
{
}

void PluginImpl::tick(Core* core)
{
    core->dump();
}


Plugin * CreatePlugin()
{
    PluginImpl* impl = new PluginImpl();
    return impl;
}