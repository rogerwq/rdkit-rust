#include "wrapper.h"
#include <GraphMol/SmilesParse/SmilesParse.h>
#include <GraphMol/Fingerprints/MorganFingerprints.h>
#include <GraphMol/ROMol.h>

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

    // ROMolPtrVector
    std::unique_ptr<ROMolPtrVec> ROMolPtrVec_new() {
        return std::make_unique<ROMolPtrVec>();
    }
    void ROMolPtrVec_emplace_back(const std::unique_ptr<ROMolPtrVec> & mols, const std::unique_ptr<RWMol> & mol) {
        mols->ptrs.emplace_back(mol.get());
    }

    // MorganFingerprints
    std::unique_ptr<MorganFingerprint> MorganFP_get_fingerprint(const std::unique_ptr<RWMol> & pMol, unsigned int radius, bool useChirality, bool useBondTypes, bool useCounts, bool onlyNonzeroInvariants, bool includeRedundantEnvironments) {
        return std::unique_ptr<MorganFingerprint>(MorganFingerprints::getFingerprint(*pMol, radius, nullptr, nullptr, useChirality, useBondTypes, useCounts, onlyNonzeroInvariants, nullptr, includeRedundantEnvironments));
    }

    size_t MorganFP_get_nonzero_elements_size(const std::unique_ptr<MorganFingerprint> & pMFP) {
        return pMFP->getNonzeroElements().size();
    }

    // FMCS
    std::unique_ptr<MCSResult> FMCS_find_mcs(const std::unique_ptr<ROMolPtrVec> & mols) {
        MCSParameters p;

        return std::make_unique<MCSResult>(findMCS(mols->ptrs, &p));
    }

    unsigned int MCSResult_num_atoms(const std::unique_ptr<MCSResult> & mcsr) { return mcsr->NumAtoms; }
    unsigned int MCSResult_num_bonds(const std::unique_ptr<MCSResult> & mcsr) { return mcsr->NumBonds; }
    const std::string & MCSResult_smarts_string(const std::unique_ptr<MCSResult> & mcsr) { return mcsr->SmartsString; }
}