#pragma once

#include <array>
#include <cstddef>
#include <cstdint>
#include <string_view>
#include <utility>

// TODO(geoff): It would be nice if we could figure out a convenient
// way to autogenerate this from the contract.

struct Fibonacci {
    constexpr static std::string_view kTraitName = "Fibonacci";
    constexpr static std::string_view kVersion = "0.1.0";

    // These must be sorted before being passed as indices to method id derivation.
    constexpr static auto kExportNames = std::array<std::string_view, 3>{
            "initialize",
            "fibonacci",
            "slow_fibonacci",
        };

    template<size_t I> requires (I < 3)
    constexpr static auto export_types() {
        if constexpr (I == 0) {
            return std::declval<void(*)()>();
        } else if constexpr (I == 1) {
            return std::declval<uint32_t(*)(uint32_t)>();
        } else if constexpr (I == 2) {
            return std::declval<uint32_t(*)(uint32_t)>();
        }
    }
};

