#pragma once

#include <array>
#include <cstddef>
#include <string_view>
#include <tuple>
#include <utility>

#include "hogd/hogexec/runtime/serialize_calldata.h"
#include "hogd/base/types.h"
#include "types.hh"

namespace hogd {

struct DelegateCallTest {
    constexpr static std::string_view kTraitName = "DelegateCallTest";
    constexpr static std::string_view kVersion = "0.0.0";

    // Specify serialization format, if non-default, with a line like the below.
    // constexpr static std::string_view kSerializationFormat = "txs";

    // These will be sorted before being passed as indices to method id derivation.
    constexpr static auto kExportNames = std::array<std::string_view, 4>{
            "test_delegate_call_context",
            "test_normal_invoke_context",
            "test_delegate_call",
            "test_normal_invoke",
        };

    using export_types = std::tuple<
        Result<std::tuple<ContractAddress, ContractAddress>> (*)(),
        Result<std::tuple<ContractAddress, ContractAddress>> (*)(),
        void(*)(uint64_t),
        void(*)(uint64_t)
    >;
};

} // namespace hogd 