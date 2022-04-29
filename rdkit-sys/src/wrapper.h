#pragma once
#include <memory>
#include <GraphMol/RWMol.h>
#include <DataStructs/SparseIntVect.h>


namespace RDKit {
    // SmilesParse
    std::unique_ptr<RWMol> SmilesParse_smi_to_mol(const std::string &smiles);

    // RWMol
    class RWMol;
    unsigned int RWMol_get_num_atoms(const std::unique_ptr<RWMol> & pMol);
    unsigned int RWMol_get_num_heavy_atoms(const std::unique_ptr<RWMol> & pMol);
    unsigned int RWMol_get_num_bonds(const std::unique_ptr<RWMol> & pMol);

    // MorganFingerprints
    typedef SparseIntVect<uint32_t> MorganFingerprint;
    std::unique_ptr<MorganFingerprint> MorganFP_get_fingerprint(const std::unique_ptr<RWMol> & pMol, unsigned int radius, bool useChirality, bool useBondTypes, bool useCounts, bool onlyNonzeroInvariants, bool includeRedundantEnvironments);
    size_t MorganFP_get_nonzero_elements_size(const std::unique_ptr<MorganFingerprint> & pMFP);
}