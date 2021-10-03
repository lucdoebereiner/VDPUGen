#include "SC_PlugIn.h"
#include <stdio.h>
#include "./bindings.h"

static InterfaceTable *ft;

struct VDPUGen : public Unit {
  struct UGenState* state;
};

extern "C" {
  void load(InterfaceTable *inTable);

  void VDPUGen_Ctor(VDPUGen* unit);
  void VDPUGen_next_a(VDPUGen* unit, int inNumSamples);
  void VDPUGen_Dtor(VDPUGen* unit);
  
};


void VDPUGen_Ctor(VDPUGen* unit) {
  SETCALC(VDPUGen_next_a);

  unit->state = new_state(SAMPLERATE);

  set_graph(unit->state);
    
  VDPUGen_next_a(unit, 1);
}


void VDPUGen_next_a(VDPUGen* unit, int inNumSamples) {
  
  UGenState* state = unit->state;

  process(state, unit->mInBuf, unit->mOutBuf, inNumSamples);

}

void VDPUGen_Dtor(VDPUGen* unit)
{

  state_free(unit->state);
  
}


PluginLoad(VDPUGen)
{
  ft = inTable;

  DefineDtorUnit(VDPUGen);
}
