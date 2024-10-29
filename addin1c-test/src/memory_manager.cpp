#include "types.h"
#include "IMemoryManager.h"

class MemoryManager : public IMemoryManager
{

public:
    virtual bool ADDIN_API AllocMemory(void **pMemory, unsigned long ulCountByte)
    {
        auto ptr = new char[ulCountByte];
        *pMemory = ptr;
        return true;
    }

    virtual void ADDIN_API FreeMemory(void **pMemory)
    {
        delete[] (char *)*pMemory;
        pMemory = 0;
    }
};

extern "C" IMemoryManager *CreateMemoryManager()
{
    return new MemoryManager();
}

extern "C" void DeleteMemoryManager(IMemoryManager *mem)
{
    delete mem;
}

extern "C" bool AllocMemory(IMemoryManager *mem, void **pMemory, unsigned long ulCountByte)
{
    return mem->AllocMemory(pMemory, ulCountByte);
}

extern "C" void FreeMemory(IMemoryManager *mem, void ** pMemory)
{
    mem->FreeMemory(pMemory);
}