function(add_wasm_contract target_name)

  add_custom_command(
    OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/${target_name}.wasm
    COMMAND cargo +nightly build
        -p ${target_name}
        --target wasm32-unknown-unknown
        --release
        --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/${target_name}/Cargo.toml
        --target-dir ${CMAKE_CURRENT_BINARY_DIR} &&
        cp ${CMAKE_CURRENT_BINARY_DIR}/wasm32-unknown-unknown/release/${target_name}.wasm ${CMAKE_CURRENT_BINARY_DIR}/${target_name}.wasm
    DEPENDS 
        ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.lock
        ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
        ${CMAKE_CURRENT_SOURCE_DIR}/${target_name}/Cargo.toml 
        ${CMAKE_CURRENT_SOURCE_DIR}/${target_name}/src/*.rs
    COMMENT "Building ${target_name}.wasm"
  )

  add_custom_target(${target_name}_wasm ALL DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/${target_name}.wasm)
endfunction()

add_wasm_contract(fibonacci)

add_custom_target(all_wasm_contracts DEPENDS fibonacci_wasm)