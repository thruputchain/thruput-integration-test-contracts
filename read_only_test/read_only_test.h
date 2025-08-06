#pragma once

#include <array>
#include <cstddef>
#include <string_view>
#include <tuple>

// TODO(geoff): It would be nice if we could figure out a convenient
// way to autogenerate this from the contract.

struct ReadOnlyTest {
    constexpr static std::string_view kTraitName = "ReadOnlyTest";
    constexpr static std::string_view kVersion = "0.0.0";

    // Specify serialization format, if non-default, with a line like the below.
    // constexpr static std::string_view kSerializationFormat = "txs";

    // These will be sorted before being passed as indices to method id derivation.
    constexpr static auto kExportNames = std::array<std::string_view, 1>{
            "read_only_bytestring_set",
        };

    using export_types = std::tuple<void(*)(bool)>;
};

