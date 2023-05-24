#include "llvm/IR/Instructions.h"

using namespace llvm;
using namespace llvm::sys;

enum LLVMRustAttribute {
  AlwaysInline = 0,
  Cold = 1,
  Hot = 2,
  NoAlias = 3,
  NoCapture = 4,
  NoInline = 5,
  NoUnwind = 6,
  NoReturn = 7
};

static Attribute::AttrKind fromRust(LLVMRustAttribute Kind) {
  switch (Kind) {
    case AlwaysInline:
        return Attribute::AlwaysInline;
    case Cold:
        return Attribute::Cold;
    case Hot:
        return Attribute::Hot;
    case NoAlias:
        return Attribute::NoAlias;
    case NoCapture:
        return Attribute::NoCapture;
    case NoInline:
        return Attribute::NoInline;
    case NoUnwind:
        return Attribute::NoUnwind;
    case NoReturn:
        return Attribute::NoReturn;
  }
  report_fatal_error("bad AttributeKind");
}

template<typename T> static inline void AddAttributes(T *t, unsigned Index,
                                                      LLVMAttributeRef *Attrs, size_t AttrsLen) {
  AttributeList PAL = t->getAttributes();
  AttributeList PALNew;
  AttrBuilder B(t->getContext());
  for (LLVMAttributeRef Attr : makeArrayRef(Attrs, AttrsLen))
    B.addAttribute(unwrap(Attr));
  PALNew = PAL.addAttributesAtIndex(t->getContext(), Index, B);
  t->setAttributes(PALNew);
}

extern "C" void LLVMRustAddFunctionAttributes(LLVMValueRef Fn, unsigned Index,
                                              LLVMAttributeRef *Attrs, size_t AttrsLen) {
  Function *F = unwrap<Function>(Fn);
  AddAttributes(F, Index, Attrs, AttrsLen);
}

extern "C" void LLVMRustAddCallSiteAttributes(LLVMValueRef Instr, unsigned Index,
                                              LLVMAttributeRef *Attrs, size_t AttrsLen) {
  CallBase *Call = unwrap<CallBase>(Instr);
  AddAttributes(Call, Index, Attrs, AttrsLen);
}

extern "C" LLVMAttributeRef LLVMCreateAttribute(LLVMContextRef C,
                                                      LLVMRustAttribute RustAttr) {
  return wrap(Attribute::get(*unwrap(C), fromRust(RustAttr)));
}
