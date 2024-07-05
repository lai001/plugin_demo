#include "core/core.h"
#include "spdlog/spdlog.h"

Core::Core()
    :value(0)
{
}

Core::~Core()
{
}

void Core::dump()
{
    spdlog::info("Core::dump {}", "Test111111111");
}
