if (NOT EXISTS "D:/Github_projects/game_boy/target/debug/build/sdl2-sys-a56d0b076815adf8/out/build/install_manifest.txt")
    message(FATAL_ERROR "Cannot find install manifest: \"D:/Github_projects/game_boy/target/debug/build/sdl2-sys-a56d0b076815adf8/out/build/install_manifest.txt\"")
endif(NOT EXISTS "D:/Github_projects/game_boy/target/debug/build/sdl2-sys-a56d0b076815adf8/out/build/install_manifest.txt")

file(READ "D:/Github_projects/game_boy/target/debug/build/sdl2-sys-a56d0b076815adf8/out/build/install_manifest.txt" files)
string(REGEX REPLACE "\n" ";" files "${files}")
foreach (file ${files})
    message(STATUS "Uninstalling \"$ENV{DESTDIR}${file}\"")
    execute_process(
        COMMAND C:/Program Files/Microsoft Visual Studio/2022/Community/Common7/IDE/CommonExtensions/Microsoft/CMake/CMake/bin/cmake.exe -E remove "$ENV{DESTDIR}${file}"
        OUTPUT_VARIABLE rm_out
        RESULT_VARIABLE rm_retval
    )
    if(NOT ${rm_retval} EQUAL 0)
        message(FATAL_ERROR "Problem when removing \"$ENV{DESTDIR}${file}\"")
    endif (NOT ${rm_retval} EQUAL 0)
endforeach(file)

