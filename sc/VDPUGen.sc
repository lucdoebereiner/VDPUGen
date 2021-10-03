VDPUGen : MultiOutUGen {
	*ar { arg in, e, a, f;
		^this.multiNewList(['audio', in, e, a, f]);
	}

	init { arg ... theInputs;
		inputs = theInputs;
		^this.initOutputs(1, 'audio');
	}	

}