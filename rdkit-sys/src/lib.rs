#[cxx::bridge(namespace = "RDKit")]
pub mod rd {
    unsafe extern "C++" {
        include!("rdkit-ffi/src/wrapper.h");
        type RWMol;

        // SmilesParse
        fn SmilesParse_smi_to_mol(smiles: &CxxString) -> UniquePtr<RWMol>;

        // RWmol
        fn RWMol_get_num_atoms(mol: &UniquePtr<RWMol>) -> u32;
        fn RWMol_get_num_heavy_atoms(mol: &UniquePtr<RWMol>) -> u32;
        fn RWMol_get_num_bonds(mol: &UniquePtr<RWMol>) -> u32;
    }
}