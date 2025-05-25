#pragma once

#include <array>
#include <cstddef>
#include <cstdint>
#include <string_view>
#include <tuple>

// TODO(geoff): It would be nice if we could figure out a convenient
// way to autogenerate this from the contract.

struct Fibonacci {
    constexpr static std::string_view kTraitName = "Fibonacci";
    constexpr static std::string_view kVersion = "0.1.0";

    // These will be sorted before being passed as indices to method id derivation.
    constexpr static auto kExportNames = std::array<std::string_view, 3>{
            "initialize",
            "fibonacci",
            "slow_fibonacci",
        };

    using export_types = std::tuple<void(*)(), uint32_t(*)(uint32_t), uint32_t(*)(uint32_t)>;
};

