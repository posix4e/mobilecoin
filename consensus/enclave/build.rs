// Copyright (c) 2018-2020 MobileCoin Inc.

//! Build script for consensus service daemon

use cargo_emit::{rustc_link_lib, rustc_link_search};
use mcbuild_sgx_utils::{Edger8r, SgxEnvironment, SgxMode};
use mcbuild_utils::Environment;

fn main() {
    let env = Environment::default();
    let sgx = SgxEnvironment::new(&env).expect("Could not read SGX build environment");

    let mut edger8r = Edger8r::new(&env, &sgx);

    for edl_data in [
        "SGX_BACKTRACE_EDL_SEARCH_PATH",
        "SGX_DEBUG_EDL_SEARCH_PATH",
        "SGX_PANIC_EDL_SEARCH_PATH",
        "SGX_SLOG_EDL_SEARCH_PATH",
    ]
    .iter()
    {
        for path_str in env
            .depvar(edl_data)
            .expect("Could not read EDL dep var")
            .split(':')
        {
            edger8r.search_path(path_str.as_ref());
        }
    }

    let enclave_edl = env
        .depvar("CONSENSUS_ENCLAVE_EDL_FILE")
        .expect("Could not read EDL file");

    edger8r
        .edl(enclave_edl.as_ref())
        .untrusted()
        .generate()
        .build();

    rustc_link_search!(sgx
        .libdir()
        .as_os_str()
        .to_str()
        .expect("Bad UTF-8 in SGX libdir"));
    rustc_link_lib!("sgx_capable");

    if sgx.sgx_mode() == SgxMode::Simulation {
        rustc_link_lib! {
            "sgx_epid_sim",
            "sgx_quote_ex_sim",
            "sgx_uae_service_sim",
            "sgx_urts_sim",
        };
    } else {
        rustc_link_lib! {
            "sgx_epid",
            "sgx_quote_ex",
            "sgx_uae_service",
            "sgx_urts",
        };
    }
}
