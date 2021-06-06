cmake_minimum_required (VERSION 2.8.12)

project(tipb)

set (CMAKE_CXX_STANDARD 17)

if (CMAKE_VERSION VERSION_LESS "3.8.0")
    set (CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++1z")
else ()
    set (CMAKE_CXX_STANDARD 17)
    set (CMAKE_CXX_EXTENSIONS 0) # https://cmake.org/cmake/help/latest/prop_tgt/CXX_EXTENSIONS.html#prop_tgt:CXX_EXTENSIONS
    set (CMAKE_CXX_STANDARD_REQUIRED ON)
    set (CXX_FLAGS_INTERNAL_COMPILER "-std=c++1z")
    # This needs to propagate to vendored projects in contrib
    set (CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++17")
endif ()

find_package(Protobuf REQUIRED)
message(STATUS "Using protobuf: ${Protobuf_VERSION} : ${Protobuf_INCLUDE_DIRS}, ${Protobuf_LIBRARIES}")

find_package(gRPC CONFIG REQUIRED)
message(STATUS "Using gRPC: ${gRPC_VERSION}")

# .proto files
set (PROTO_DEF_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../proto")
file (GLOB PROTO_DEFS "${PROTO_DEF_DIR}/*.proto")
# The output dir for .pb.h, .pb.cc
set (PROTO_OUTPUT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/tipb")
file (MAKE_DIRECTORY "${PROTO_OUTPUT_DIR}")
# A temporary directory for proto file without gogoproto marks
set (PROTO_DEF_TMP_DIR "${CMAKE_CURRENT_BINARY_DIR}/protos")
file (MAKE_DIRECTORY "${PROTO_DEF_TMP_DIR}")

set (CLEAN_PROTO_DEFS "")
foreach (F ${PROTO_DEFS})
    get_filename_component (ABS_FIL ${F} ABSOLUTE)
    get_filename_component (FIL_WE ${F} NAME_WE)
    list (APPEND CLEAN_PROTO_DEFS "${PROTO_DEF_TMP_DIR}/${FIL_WE}.proto")
    # Generate a output file by copying and cleaning up gogoproto marks.
    # Will be re-generated if the original .proto file is changed.
    add_custom_command(
        OUTPUT "${PROTO_DEF_TMP_DIR}/${FIL_WE}.proto"
        WORKING_DIRECTORY ${PROTO_DEF_DIR}
        COMMAND cp ${ABS_FIL} ${PROTO_DEF_TMP_DIR}/${FIL_WE}.proto
        COMMAND "${CMAKE_CURRENT_SOURCE_DIR}/../generate-cpp.sh"
        ARGS clean ${PROTO_DEF_TMP_DIR}/${FIL_WE}.proto
        DEPENDS ${ABS_FIL}
        COMMENT "Generating clean proto file on ${ABS_FIL}"
        VERBATIM)
endforeach()
# Grab all clean .proto files as a target
add_custom_target (__clean_tipb_defs
    DEPENDS ${CLEAN_PROTO_DEFS}
    VERBATIM)

set (PROTO_SRCS "")
set (PROTO_HDRS "")
foreach (F ${PROTO_DEFS})
    get_filename_component (ABS_FIL ${F} ABSOLUTE)
    get_filename_component (FIL_WE ${F} NAME_WE)

    list (APPEND PROTO_SRCS "${PROTO_OUTPUT_DIR}/${FIL_WE}.pb.cc")
    list (APPEND PROTO_HDRS "${PROTO_OUTPUT_DIR}/${FIL_WE}.pb.h")
    # Create the generated cpp files for .proto file.
    # Add dependency on the original .proto file and `__clean_tipb_defs`
    # so that it can re-generate source files if .proto changed.
    add_custom_command(
        OUTPUT "${PROTO_OUTPUT_DIR}/${FIL_WE}.pb.cc"
               "${PROTO_OUTPUT_DIR}/${FIL_WE}.pb.h"
        WORKING_DIRECTORY ${PROTO_DEF_TMP_DIR}
        COMMAND protobuf::protoc
        ARGS --cpp_out=${PROTO_OUTPUT_DIR} ${PROTO_DEF_TMP_DIR}/${FIL_WE}.proto -I ${PROTO_DEF_TMP_DIR}
        DEPENDS ${ABS_FIL} protobuf::protoc __clean_tipb_defs
        COMMENT "Running C++ protocol buffer compiler on ${ABS_FIL}"
        VERBATIM)
endforeach()

# Mark those clean .proto, .pb.h, .pb.cc files are generated
# so that they can be re-build if changed and clean by `make clean`
set_source_files_properties(${CLEAN_PROTO_DEFS} ${PROTO_SRCS} ${PROTO_HDRS} PROPERTIES GENERATED TRUE)

add_library(tipb ${PROTO_SRCS} ${PROTO_HDRS})
target_include_directories(tipb PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${PROTOBUF_INCLUDE_DIRS}
)
