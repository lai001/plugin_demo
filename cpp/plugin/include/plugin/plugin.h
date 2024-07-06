
#pragma once

#include "core/core.h"

class PLUGIN_API Plugin
{
public:
    virtual ~Plugin();
    virtual void tick(Core* core) = 0;

};
