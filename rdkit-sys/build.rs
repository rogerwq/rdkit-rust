fn main() {
    let rdkit_year = "2022";
    let rdkit_month = "03";
    let rdkit_revision = "2";

    let target = std::env::var("TARGET").unwrap();
    let windows = target.contains("windows");
    let dst = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let include = dst.join("include");
    println!("cargo:root={}", dst.display());
    println!("cargo:include={}", include.display());
    if !include.exists() {
        std::fs::create_dir(&include).unwrap();
    }

    // rdkit repo as submodule
    if !std::path::Path::new("rdkit/.git").exists() {
        let _ = std::process::Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status().unwrap();
    }

    // boost from conda
    let path_conda = std::env::var("CONDA_PREFIX").unwrap();
    let incl_conda = std::path::Path::new(&path_conda).join("include");

    // RDGeneral/export.h
    let include_rd_general = include.join("RDGeneral");
    if !include_rd_general.exists() {
        std::fs::create_dir(&include_rd_general).unwrap();
    }
    std::fs::copy(
        "rdkit/External/GA/util/export.h",
        include_rd_general.join("export.h")
    ).unwrap();

    // missing cmake created header & source files
    let contents_version_h = std::fs::read_to_string("rdkit/Code/RDGeneral/versions.h.cmake").unwrap();
    if windows {
        return println!("windows not supported yet");
    } else {
        std::fs::write(
            include.join("versions.h"),
            contents_version_h
                .replace("@RDKit_Year @", rdkit_year)
                .replace("@RDKit_intMonth @", rdkit_month)
                .replace("@RDKit_Revision @", rdkit_revision)
        ).unwrap();
    }
    for header in [
        "smiles.tab.hpp",
        "smarts.tab.hpp"
    ].iter()
    {
        std::fs::copy(
            format!("rdkit/Code/GraphMol/SmilesParse/{}.cmake", header),
            include.join(header)
        ).unwrap();
    }
    for source in [
        "smiles.tab.cpp",
        "smarts.tab.cpp",
        "lex.yysmiles.cpp",
        "lex.yysmarts.cpp"
    ].iter()
    {
        std::fs::copy(
            format!("rdkit/Code/GraphMol/SmilesParse/{}.cmake", source),
            dst.join(source)
        ).unwrap();
    }

    // Compiling
    let incl_src = std::path::Path::new("./src");
    let incl_rdkit_repo = std::path::Path::new("./rdkit/Code");

    cxx_build::bridge("src/lib.rs")
        .file("rdkit/Code/GraphMol/ROMol.cpp")
        .file("rdkit/Code/GraphMol/RWMol.cpp")
        .file("rdkit/Code/GraphMol/Atom.cpp")
        .file("rdkit/Code/GraphMol/Bond.cpp")
        .file("rdkit/Code/GraphMol/Conformer.cpp")
        .file("rdkit/Code/GraphMol/SubstanceGroup.cpp")
        .file("rdkit/Code/GraphMol/QueryOps.cpp")
        .file("rdkit/Code/GraphMol/RingInfo.cpp")
        .file("rdkit/Code/GraphMol/QueryAtom.cpp")
        .file("rdkit/Code/GraphMol/QueryBond.cpp")
        .file("rdkit/Code/GraphMol/PeriodicTable.cpp")
        .file("rdkit/Code/GraphMol/atomic_data.cpp")
        .file("rdkit/Code/GraphMol/Canon.cpp")
        .file("rdkit/Code/GraphMol/MolOps.cpp")
        .file("rdkit/Code/GraphMol/AtomIterators.cpp")
        .file("rdkit/Code/GraphMol/BondIterators.cpp")
        .file("rdkit/Code/GraphMol/StereoGroup.cpp")
        .file("rdkit/Code/GraphMol/FindRings.cpp")
        .file("rdkit/Code/GraphMol/FindStereo.cpp")
        .file("rdkit/Code/GraphMol/Aromaticity.cpp")
        .file("rdkit/Code/GraphMol/Kekulize.cpp")
        .file("rdkit/Code/GraphMol/Chirality.cpp")
        .file("rdkit/Code/GraphMol/ConjugHybrid.cpp")
        .file("rdkit/Code/GraphMol/AddHs.cpp")
        .file("rdkit/Code/GraphMol/new_canon.cpp")
        .file("rdkit/Code/GraphMol/SmilesParse/SmilesParse.cpp")
        .file("rdkit/Code/GraphMol/SmilesParse/SmilesParseOps.cpp")
        .file("rdkit/Code/GraphMol/SmilesParse/CXSmilesOps.cpp")
        .file("rdkit/Code/GraphMol/Fingerprints/FingerprintUtil.cpp")
        .file("rdkit/Code/GraphMol/Fingerprints/MorganFingerprints.cpp")
        .file("rdkit/Code/RDGeneral/RDLog.cpp")
        .file("rdkit/Code/RDGeneral/Invariant.cpp")
        .file("rdkit/Code/RDGeneral/types.cpp")
        .file("rdkit/Code/RDGeneral/LocaleSwitcher.cpp")
        .file(dst.join("smiles.tab.cpp"))
        .file(dst.join("smarts.tab.cpp"))
        .file(dst.join("lex.yysmiles.cpp"))
        .file(dst.join("lex.yysmarts.cpp"))
        .file("src/wrapper.cpp")
        .include(include)
        .include(incl_src)
        .include(incl_rdkit_repo)
        .include(incl_conda)
        .define("RDKIT_RDGENERAL_EXPORT", "RDKIT_EXPORT_API")
        .define("RDKIT_GRAPHMOL_EXPORT", "RDKIT_EXPORT_API")
        .define("RDKIT_QUERY_EXPORT", "RDKIT_EXPORT_API")
        .define("RDKIT_RDGEOMETRYLIB_EXPORT", "RDKIT_EXPORT_API")
        .define("RDKIT_SMILESPARSE_EXPORT", "RDKIT_EXPORT_API")
        .define("RDKIT_DATASTRUCTS_EXPORT", "RDKIT_EXPORT_API")
        .define("RDKIT_SUBGRAPHS_EXPORT", "RDKIT_EXPORT_API")
        .define("RDKIT_FINGERPRINTS_EXPORT", "RDKIT_EXPORT_API")
        .define("RDKIT_SUBSTRUCTMATCH_EXPORT", "RDKIT_EXPORT_API")
        .flag_if_supported("-std=c++17")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-function")
        .flag("-Wno-unused-variable")
        .flag("-Wno-deprecated-declarations")
        .flag("-Wno-reorder-ctor")
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-private-field")
        .flag("-Wno-c99-extensions")
        .flag("-Wno-extra-tokens")
        .flag("-Wno-tautological-overlap-compare")
        .flag("-Wno-deprecated-copy")
        .compile("rdkit");

        println!("cargo:rerun-if-changed=src/lib.rs");
        println!("cargo:rerun-if-changed=src/wrapper.cpp");

}