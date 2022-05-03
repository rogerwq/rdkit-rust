#[cxx::bridge(namespace = "RDKit")]
pub mod rd {
    unsafe extern "C++" {
        include!("rdkit-ffi/src/wrapper.h");

        // SmilesParse
        fn SmilesParse_smi_to_mol(smiles: &CxxString) -> SharedPtr<RWMol>;

        // RWmol
        type RWMol;
        fn RWMol_get_num_atoms(mol: &SharedPtr<RWMol>) -> u32;
        fn RWMol_get_num_heavy_atoms(mol: &SharedPtr<RWMol>) -> u32;
        fn RWMol_get_num_bonds(mol: &SharedPtr<RWMol>) -> u32;
        type ROMolPtrVec;
        fn ROMolPtrVec_new() -> UniquePtr<ROMolPtrVec>;
        fn ROMolPtrVec_emplace_back(mols: &UniquePtr<ROMolPtrVec>, mol: &SharedPtr<RWMol>);
        fn ROMolPtrVec_ptrs_count(mols: &UniquePtr<ROMolPtrVec>) -> u32;
        fn ROMolPtrVec_first_ptr_atom_count(mols: &UniquePtr<ROMolPtrVec>, index: usize) -> u32;

        // MorganFingerprint
        type MorganFingerprint;
        fn MorganFP_get_fingerprint(mol: &SharedPtr<RWMol>, radius: u32, useChirality: bool, useBondTypes: bool, useCounts: bool, onlyNonzeroInvariants: bool, includeRedundantEnvironments: bool) -> UniquePtr<MorganFingerprint>;
        fn MorganFP_get_nonzero_elements_size(mfp: &UniquePtr<MorganFingerprint>) -> usize;

        // FMCS
        type MCSResult;
        fn FMCS_find_mcs(mols: &UniquePtr<ROMolPtrVec>) -> UniquePtr<MCSResult>;
        fn MCSResult_num_atoms(mcsr: &UniquePtr<MCSResult>) -> u32;
        fn MCSResult_num_bonds(mcsr: &UniquePtr<MCSResult>) -> u32;
        fn MCSResult_smarts_string(mcsr: &UniquePtr<MCSResult>) -> &CxxString;
    }
}