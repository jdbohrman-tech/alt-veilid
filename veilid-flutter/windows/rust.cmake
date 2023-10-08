# We include Corrosion inline here, but ideally in a project with
# many dependencies we would need to install Corrosion on the system.
# See instructions on https://github.com/AndrewGaspar/corrosion#cmake-install
# Once done, uncomment this line:
# find_package(Corrosion REQUIRED)

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/AndrewGaspar/corrosion.git
    GIT_TAG v0.4.4 # Optionally specify a version tag or branch here
)

FetchContent_MakeAvailable(Corrosion)

execute_process(COMMAND git rev-parse --show-cdup
    WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}"
    OUTPUT_VARIABLE relative_path_to_repository_root)
string(STRIP ${relative_path_to_repository_root} relative_path_to_repository_root)

get_filename_component(repository_root
    "${CMAKE_SOURCE_DIR}/${relative_path_to_repository_root}"
    ABSOLUTE)

corrosion_import_crate(MANIFEST_PATH ${repository_root}/../veilid/Cargo.toml CRATES veilid-flutter)

# Flutter-specific

set(CRATE_NAME "veilid-flutter")
target_link_libraries(${PLUGIN_NAME} PUBLIC ${CRATE_NAME})
# list(APPEND PLUGIN_BUNDLED_LIBRARIES $<TARGET_FILE:${CRATE_NAME}-shared>)