    
    MKT
    - Compare speed to install using the application and doing 
        it manually: "This tool reduced 90% of time spent installing the IDE manually"

    TODO
        - Separate into modules
        - Write Tests

    ## V1

    - Create jetbrains main dir at /op/
    - Unzip IDE into jetbrains main dir
    - Create symlink to enable the IDE to be called through CLI
    - Create the desktop entry file
    - Resume of operations

    ## V2
    
    - Flags and Arguments:
        - Argument to change default JetBrains folder path
        - Flag to set if creates or not symlink
        - Flag to open IDE after installation

    - Add support to other JetBrains IDE's