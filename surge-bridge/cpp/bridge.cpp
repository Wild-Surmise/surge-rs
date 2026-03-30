#include "bridge.h"
#include "src/common/SurgeSynthesizer.h"

class ErrCork : public SurgeSynthesizer::PluginLayer {
	public:
		void surgeParameterUpdated(const SurgeSynthesizer::ID &id, float d) override {}
		void surgeMacroUpdated(long macroNum, float d) override {}
};

extern "C" {
	SurgeSynthesizer* create_engine(float sr) {
		auto* layer = new ErrCork();
		auto* surge = new SurgeSynthesizer(layer, "");

		surge->setSamplerate(sr);
		surge->time_data.tempo = 120;
		surge->time_data.ppqPos = 0;

		return surge;
	}

	SurgePatch* create_patch() {
		SurgeStorage::SurgeStorageConfig sconf;
		sconf.scanWavetableAndPatches = false;
		sconf.createUserDirectory = false;

		auto* storage = new SurgeStorage(sconf);
		auto* patch = new SurgePatch(storage);

		return patch;
	}

	void destroy_engine(SurgeSynthesizer* surge) {
		if (surge) delete surge;	// this just works?
	}

	void destroy_patch(SurgePatch* patch) {
		if (patch) delete patch;
	}

	// TODO: check if below and above even need the if.
	void destroy_parameter(Parameter* p) {
		if (p) delete p;
	}
}
