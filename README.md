LOD Cloud Draw
--------------

This is a tool for creating linked open data cloud diagrams, such as those 
found at http://lod-cloud.net/. 

## Installation

This project is written in Rust and can be compiled with Cargo as follows

    cargo build --release

This will create a binary under target/release/lod-cloud-draw that can be used 
to create cloud diagrams

### Requirements

To use this library you must have gfortran installed in your pc

* for windows use fortran compiler provided by mingw or TDM-GCC
* for linux you can use the package manager to install gfortran
* for Mac os you can install it form [here](http://hpc.sourceforge.net/) or 
    [here](http://sourceforge.net/projects/hpc/files/hpc/g95/gfortran-mlion.tar.gz)

## Details

Tool used to create LOD cloud diagrams as SVG.
The cloud is created as a minimization of the following function:

  f(V,E) = s * sum_{e} spring(e) + r * sum_{v1} sum_{v2} repulse(v1, v2, d) + 
                w * sum_{v} well(v, c)

Where:

* spring(e): Measures the length of a link in the cloud
* repulse(v1, v2, d): Indicates if v1 and v2 are within a distance of d
* well(v, c): Indicates if v is contained within a circle (well) of radius c

And s,r,w are tuning constants

## Usage
    
    lod-cloud-draw [OPTIONS] <data.json> <output.svg>

### FLAGS

    -h, --help       Prints help information
    -V, --version    Prints version information

### OPTIONS

        --algorithm <cg|lbfgsb>          The algorithm used to find the cloud diagram (cg=Conjugate
                                         Gradient or lbfgsb = Limited BFGS)
    -w, --well <FORCE>                   The value of the well boundary force
        --canvas-rigidity <FACTOR>       The rigidity of the well
    -c, --canvas <PIXELS>                The radius of the circle that the bubbles should be contained in
        --ident <none|neighbour|tags>    The algorithm used to identify domain (bubble colours) of unidentified datasets
    -i, --max-iters <ITERATIONS>         The maximum number of iterations to perform (default=10000)
    -n, --n-blocks <BLOCKS>              Apply an n x n blocking method to speed up the algorithm 
                                         (default=1, no blocking)
    -r, --repulse <FORCE>                The value of the repulsion force
    -d, --distance <PIXELS>              The minimal distance between bubbles
        --repulse-rigidity <FACTOR>      The rigidity of repulsion between bubbles
    -e, --settings <settings.json>       The JSON file containing the settings for the system
    -s, --spring <FORCE>                 The value of the spring force

### ARGS

    <data.json>     The data of the LOD cloud
    <output.svg>    The path of the SVG file to write to

## Settings

In order to create a cloud diagram a settings file is required, this is a JSON 
file as follows

* `legend`: The legend (categories) to show; an array of legend entries (see below)
* `fixed_points`: If any datasets should occur at a fixed position; dictionary
    of strings to an array of two floating point numbers
* `selection`: Selection method: Either "all", "dataset" or "domain"
* `selected`: The value of the dataset/domain to be selected
* `hops`: The number of hops from a dataset in dataset mode
* `datasets`: Any datasets that must be included in the data
* `rights_text`:  The description and copyright text
* `logo_link`: The logo (link) (to be included in the bottom right of the image)
* `logo_width`: The logo width

Legend entries consist of the following

* `title`: The display title to be shown to the user
* `domain`: The value of the `domain` property in data that corresponds to
* `colour`: The (HTML) colour of this legend
* `keywords`:  The keywords that identify this domain

Examples of this may be found under the `clouds` folder

## Data

Data may be obtained from the following URL

    https://lod-cloud.net/lod-data.json

Rebuilding from the live version can be done with the following command

    python3 scripts/get-data.py
