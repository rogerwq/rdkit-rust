use rdkit_ffi::rd;
use super::molecule;

pub struct MorganFingeprint {
    pub rd_mfp: cxx::UniquePtr<rd::MorganFingerprint>
}

impl MorganFingeprint {
    pub fn get_morgan_fp(mol: &molecule::Molecule, radius: u32, use_chirality: bool) -> Self {
        Self{ rd_mfp: rd::MorganFP_get_fingerprint(&mol.rd_mol, radius, use_chirality, true, true, false, false) }
    }

    pub fn non_zero_elements_size(&self) -> usize { 
        rd::MorganFP_get_nonzero_elements_size(&self.rd_mfp) 
    }
}

#[cfg(test)]
mod test_mod_fingerprint {
    use super::*;

    #[test]
    fn test_molecule() {
        let mol = molecule::Molecule::new_from_smiles("CC(F)(Cl)[C@](F)(Cl)C");
        assert_eq!(MorganFingeprint::get_morgan_fp(&mol, 0, false).non_zero_elements_size(), 4);
        assert_eq!(MorganFingeprint::get_morgan_fp(&mol, 1, false).non_zero_elements_size(), 8);
        assert_eq!(MorganFingeprint::get_morgan_fp(&mol, 0, true).non_zero_elements_size(), 4);
        assert_eq!(MorganFingeprint::get_morgan_fp(&mol, 1, true).non_zero_elements_size(), 9);
    }
}