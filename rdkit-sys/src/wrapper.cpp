#include "wrapper.h"
#include <GraphMol/SmilesParse/SmilesParse.h>

namespace RDKit {
    // SmilesParse
    std::unique_ptr<RWMol> SmilesParse_smi_to_mol(const std::string &smiles) {
        SmilesParserParams params;
        params.allowCXSMILES = true;
        return std::make_unique<RWMol>(std::move(*SmilesToMol(smiles, params)));
    }

    // RWMol
    unsigned int RWMol_get_num_atoms(const std::unique_ptr<RWMol> & pMol) { return pMol->getNumAtoms(); }
    unsigned int RWMol_get_num_heavy_atoms(const std::unique_ptr<RWMol> & pMol) { return pMol->getNumHeavyAtoms(); }
    unsigned int RWMol_get_num_bonds(const std::unique_ptr<RWMol> & pMol) { return pMol->getNumBonds(); }
}