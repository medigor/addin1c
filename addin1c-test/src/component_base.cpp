#include "ComponentBase.h"
#include "IMemoryManager.h"

extern "C" bool RegisterExtensionAs(IComponentBase *component, char16_t **wsExtensionName)
{
    return component->RegisterExtensionAs(wsExtensionName);
}

extern "C" bool SetMemManager(IComponentBase *component, void *disp)
{
    return component->setMemManager(disp);
}

extern "C" long FindProp(IComponentBase *component, const char16_t *name)
{
    return component->FindProp(name);
}

extern "C" bool SetPropVal(IComponentBase *component, long num, tVariant *value)
{
    return component->SetPropVal(num, value);
}

extern "C" bool GetPropVal(IComponentBase *component, long num, tVariant *value)
{
    return component->GetPropVal(num, value);
}

extern "C" long FindMethod(IComponentBase *component, const char16_t *name)
{
    return component->FindMethod(name);
}

extern "C" long GetNParams(IComponentBase *component, long num)
{
    return component->GetNParams(num);
}

extern "C" bool CallAsFunc(IComponentBase *component, long num, tVariant *pvarRetValue, tVariant *params, long len)
{
    return component->CallAsFunc(num, pvarRetValue, params, len);
}
