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

struct Ed25519VerifyWrapper {
    constexpr static std::string_view kTraitName = "Ed25519VerifyWrapper";
    constexpr static std::string_view kVersion = "0.0.0";

    // These will be sorted before being passed as indices to method id derivation.
    constexpr static auto kExportNames = std::array<std::string_view, 1>{
            "ed25519_verify"
        };

    using export_types = std::tuple<bool(*)(std::vector<uint8_t>, hogd::Ed25519Signature, hogd::Ed25519PublicKey)>;
};

