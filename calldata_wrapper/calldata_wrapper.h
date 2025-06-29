#pragma once

#include <array>
#include <cstddef>
#include <cstdint>
#include <string_view>
#include <tuple>
#include <vector>
#include "types.hh"

// TODO(geoff): It would be nice if we could figure out a convenient
// way to autogenerate this from the contract.

struct Sha256Wrapper {
    constexpr static std::string_view kTraitName = "CalldataWrapper";
    constexpr static std::string_view kVersion = "0.0.0";

    // These will be sorted before being passed as indices to method id derivation.
    constexpr static auto kExportNames = std::array<std::string_view, 1>{
            "calldata_wrapper"
        };

    using export_types = std::tuple<std::vector<uint8_t>(*)(std::vector<uint8_t>)>;
};

