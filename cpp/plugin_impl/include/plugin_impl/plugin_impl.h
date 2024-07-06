#pragma once
#include "plugin/plugin.h"

class PLUGIN_IMPL_API PluginImpl :public Plugin
{
public:
    PluginImpl();
    ~PluginImpl();

    virtual void tick(Core* core) override;

private:

};

extern "C"  {
    PLUGIN_IMPL_API Plugin*  CreatePlugin();
}