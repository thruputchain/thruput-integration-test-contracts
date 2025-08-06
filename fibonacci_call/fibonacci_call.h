#pragma once

#include <array>
#include <cstddef>
#include <cstdint>
#include <string_view>
#include <tuple>

#include "types.hh"

// TODO(geoff): It would be nice if we could figure out a convenient
// way to autogenerate this from the contract.

struct FibonacciCaller {
    constexpr static std::string_view kTraitName = "FibonacciCaller";
    constexpr static std::string_view kVersion = "0.0.0";

    // Specify serialization format, if non-default, with a line like the below.
    // constexpr static std::string_view kSerializationFormat = "txs";

    // These will be sorted before being passed as indices to method id derivation.
    constexpr static auto kExportNames = std::array<std::string_view, 3>{
            "initialize",
            "call_fibonacci",
            "call_fibonacci_expect_out_of_gas",
        };

    using export_types = std::tuple<void(*)(hogd::ContractAddress), void(*)(uint32_t, uint64_t), void(*)(uint32_t, uint64_t)>;
};

