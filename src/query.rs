use rdkit_ffi::rd;
use super::molecule;

pub struct MCSResult {
    pub rd_mcsr: cxx::UniquePtr<rd::MCSResult>
    // pub mol_vec: cxx::UniquePtr<rd::ROMolPtrVec>
}

impl MCSResult {
    pub fn find_mcs(mols: &Vec<molecule::Molecule>) -> Self {
        let mol_vec = rd::ROMolPtrVec_new();
        for mol in mols.iter() {
            rd::ROMolPtrVec_emplace_back(&mol_vec, &mol.rd_mol)
        }

        let rd_mcsr = rd::FMCS_find_mcs(&mol_vec);
        return Self { rd_mcsr } 
    }

    pub fn num_atoms(&self) -> u32 { rd::MCSResult_num_atoms(&self.rd_mcsr) }
    pub fn num_bonds(&self) -> u32 { rd::MCSResult_num_bonds(&self.rd_mcsr) }
    pub fn smarts_string(&self) -> String { 
        rd::MCSResult_smarts_string(&self.rd_mcsr).to_string_lossy().into_owned()
    }
}

#[cfg(test)]
mod test_mod_query {
    use super::*;

    #[test]
    fn test_fmcs() {
        let smiles_vec = vec![
            "O=C(NCc1cc(OC)c(O)cc1)CCCC/C=C/C(C)C",
            "CC(C)CCCCCC(=O)NCC1=CC(=C(C=C1)O)OC",
            "c1(C=O)cc(OC)c(O)cc1"
        ];
        let mols: Vec<molecule::Molecule> = smiles_vec
            .iter()
            .map(|smiles| molecule::Molecule::new_from_smiles(smiles))
            .collect();
        let mcrs = MCSResult::find_mcs(&mols);
        assert_eq!(mcrs.num_atoms(), 10);
        assert_eq!(mcrs.num_bonds(), 10);
        assert_eq!(mcrs.smarts_string(), "[#6]1(-[#6]):[#6]:[#6](-[#8]-[#6]):[#6](:[#6]:[#6]:1)-[#8]");
    }
}