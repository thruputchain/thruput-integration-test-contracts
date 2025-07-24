#pragma once


#include <array>
#include <cstddef>
#include <cstdint>
#include <string_view>
#include <tuple>

#include "x23/vec.h"
#include "transaction.hh"

// TODO(geoff): It would be nice if we could figure out a convenient
// way to autogenerate this from the contract.

struct CxsRoundtrip {
    constexpr static std::string_view kTraitName = "CxsRoundtrip";
    constexpr static std::string_view kVersion = "0.0.0";

    // Specify serialization format, if non-default, with a line like the below.
    // constexpr static std::string_view kSerializationFormat = "txs";

    // These will be sorted before being passed as indices to method id derivation.
    constexpr static auto kExportNames = std::array<std::string_view, 11>{
            "roundtrip_foo",
            "roundtrip_senum",
            "roundtrip_sunion",
            "roundtrip_sunion_no_void",
            "roundtrip_uunion",
            "roundtrip_uunion_bool",
            "roundtrip_test_recursive",
            "roundtrip_bytestring",
            "roundtrip_sha256hash",
            "roundtrip_storage_delta",
            "roundtrip_storage_delta_vec",
        };

    using roundtrip_signature = x23::Vec<uint8_t>(*)(hogd::Calldata);

    using export_types = std::tuple<roundtrip_signature, roundtrip_signature, roundtrip_signature, roundtrip_signature, roundtrip_signature, roundtrip_signature, roundtrip_signature, roundtrip_signature, roundtrip_signature, roundtrip_signature, roundtrip_signature>;
};

