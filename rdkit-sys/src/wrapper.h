#pragma once
#include <memory>
#include <GraphMol/RWMol.h>
#include <DataStructs/SparseIntVect.h>
#include <GraphMol/FMCS/FMCS.h>


namespace RDKit {
    // SmilesParse
    std::unique_ptr<RWMol> SmilesParse_smi_to_mol(const std::string &smiles);

    // RWMol
    class RWMol;
    unsigned int RWMol_get_num_atoms(const std::unique_ptr<RWMol> & pMol);
    unsigned int RWMol_get_num_heavy_atoms(const std::unique_ptr<RWMol> & pMol);
    unsigned int RWMol_get_num_bonds(const std::unique_ptr<RWMol> & pMol);
    struct ROMolPtrVec {
        std::vector<ROMOL_SPTR> ptrs{};

        public:
            void emplace_back(RWMol * pMol) {
                ptrs.emplace_back(new RWMol(*pMol));
            }
    };
    std::unique_ptr<ROMolPtrVec> ROMolPtrVec_new();
    void ROMolPtrVec_emplace_back(const std::unique_ptr<ROMolPtrVec> & mols, const std::unique_ptr<RWMol> & mol);

    // MorganFingerprints
    typedef SparseIntVect<uint32_t> MorganFingerprint;
    std::unique_ptr<MorganFingerprint> MorganFP_get_fingerprint(const std::unique_ptr<RWMol> & pMol, unsigned int radius, bool useChirality, bool useBondTypes, bool useCounts, bool onlyNonzeroInvariants, bool includeRedundantEnvironments);
    size_t MorganFP_get_nonzero_elements_size(const std::unique_ptr<MorganFingerprint> & pMFP);

    // FMCS
    struct MCSResult;
    std::unique_ptr<MCSResult> FMCS_find_mcs(const std::unique_ptr<ROMolPtrVec> & mols);
    unsigned int MCSResult_num_atoms(const std::unique_ptr<MCSResult> & mcsr);
    unsigned int MCSResult_num_bonds(const std::unique_ptr<MCSResult> & mcsr);
    const std::string & MCSResult_smarts_string(const std::unique_ptr<MCSResult> & mcsr);
}