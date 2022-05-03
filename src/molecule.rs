use rdkit_ffi::rd;

pub struct Molecule {
    pub rd_mol: cxx::SharedPtr<rd::RWMol>
}

impl Molecule {
    pub fn new_from_smiles(smiles: &str) -> Self {
        cxx::let_cxx_string!(smiles_cxx = smiles);
        Self { rd_mol: rd::SmilesParse_smi_to_mol(&smiles_cxx) }
    }
    
    pub fn num_atoms(&self) -> u32 { rd::RWMol_get_num_atoms(&self.rd_mol) }
    pub fn num_heavy_atoms(&self) -> u32 { rd::RWMol_get_num_heavy_atoms(&self.rd_mol) }
    pub fn num_bonds(&self) -> u32 { rd::RWMol_get_num_bonds(&self.rd_mol) }

   
}

#[cfg(test)]
mod test_mod_molecule {
    use super::*;

    #[test]
    fn test_molecule() {
        let mol = Molecule::new_from_smiles("CCNCC");
        assert_eq!(mol.num_atoms(), 5);
        assert_eq!(mol.num_bonds(), 4);
        assert_eq!(mol.num_heavy_atoms(), 5);
    }
}