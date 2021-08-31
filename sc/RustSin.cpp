#include "SC_PlugIn.h"
#include <stdio.h>
#include "./bindings.h"

static InterfaceTable *ft;

struct RustSin : public Unit {
  struct UGenState* state;
};

extern "C" {
  void load(InterfaceTable *inTable);

  void RustSin_Ctor(RustSin* unit);
  void RustSin_next_a(RustSin* unit, int inNumSamples);
  void RustSin_Dtor(RustSin* unit);
  
};


void RustSin_Ctor(RustSin* unit) {
  SETCALC(RustSin_next_a);

  unit->state = new_state(SAMPLERATE);

  set_graph(unit->state);
    
  RustSin_next_a(unit, 1);
}


void RustSin_next_a(RustSin* unit, int inNumSamples) {
  
  UGenState* state = unit->state;

  process(state, unit->mInBuf, unit->mOutBuf, inNumSamples);

}

void RustSin_Dtor(RustSin* unit)
{

  state_free(unit->state);
  
}


PluginLoad(RustSin)
{
  ft = inTable;

  DefineDtorUnit(RustSin);
}
