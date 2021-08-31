RustSin : MultiOutUGen {
	*ar { arg fr;
		^this.multiNewList(['audio', fr]);
	}

	init { arg ... theInputs;
		inputs = theInputs;
		^this.initOutputs(1, 'audio');
	}	

}