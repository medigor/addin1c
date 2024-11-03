#include "types.h"

extern "C" unsigned long SizeOfVariant()
{
    return sizeof(tVariant);
}

extern "C" unsigned short GetTypeVariant(tVariant *variant)
{
    return variant->vt;
}

extern "C" void SetEmptyVariant(tVariant *variant)
{
    if (variant->vt == VTYPE_PWSTR)
    {
        delete[] variant->pwstrVal;
        variant->pwstrVal = 0;
        variant->wstrLen = 0;
    }
    else if (variant->vt == VTYPE_BLOB)
    {
        delete[] variant->pstrVal;
        variant->pstrVal = 0;
        variant->strLen = 0;
    }
    variant->vt = VTYPE_EMPTY;
}

extern "C" bool GetValVariantBool(tVariant *variant)
{
    return TV_BOOL(variant);
}

extern "C" void SetValVariantBool(tVariant *variant, bool val)
{
    SetEmptyVariant(variant);
    TV_VT(variant) = VTYPE_BOOL;
    TV_BOOL(variant) = val;
}

extern "C" int32_t GetValVariantI4(tVariant *variant)
{
    return TV_I4(variant);
}

extern "C" void SetValVariantI4(tVariant *variant, int32_t val)
{
    SetEmptyVariant(variant);
    TV_VT(variant) = VTYPE_I4;
    TV_I4(variant) = val;
}

extern "C" double GetValVariantR8(tVariant *variant)
{
    return TV_R8(variant);
}

extern "C" void SetValVariantR8(tVariant *variant, double val)
{
    SetEmptyVariant(variant);
    TV_VT(variant) = VTYPE_R8;
    TV_R8(variant) = val;
}

extern "C" uint32_t GetLenVariantString(tVariant *variant)
{
    return variant->wstrLen;
}

extern "C" char16_t *GetValVariantString(tVariant *variant)
{
    return variant->pwstrVal;
}

extern "C" void SetValVariantString(tVariant *variant, char16_t *str, uint32_t len)
{
    SetEmptyVariant(variant);

    auto val = new char16_t[len];
    memcpy(val, str, len * sizeof(char16_t));

    TV_VT(variant) = VTYPE_PWSTR;
    variant->pwstrVal = val;
    variant->wstrLen = len;
}
