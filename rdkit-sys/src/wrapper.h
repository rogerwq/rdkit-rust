#pragma once
#include <memory>
#include <GraphMol/RWMol.h>


namespace RDKit {
    class RWMol;

    // SmilesParse
    std::unique_ptr<RWMol> SmilesParse_smi_to_mol(const std::string &smiles);

    // RWMol
    unsigned int RWMol_get_num_atoms(const std::unique_ptr<RWMol> & pMol);
    unsigned int RWMol_get_num_heavy_atoms(const std::unique_ptr<RWMol> & pMol);
    unsigned int RWMol_get_num_bonds(const std::unique_ptr<RWMol> & pMol);
}