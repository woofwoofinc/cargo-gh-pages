# ![Cargo Sphinx](https://raw.githubusercontent.com/woofwoofinc/cargo-sphinx/master/docs/assets/title.png)

[![Crates.io](https://img.shields.io/crates/v/cargo-sphinx.svg)](https://crates.io/crates/cargo-sphinx)
[![Build Status](https://travis-ci.org/woofwoofinc/cargo-sphinx.svg?branch=master)](https://travis-ci.org/woofwoofinc/cargo-sphinx)
[![Windows Build Status](https://ci.appveyor.com/api/projects/status/395nrdfq1aqdjwd8/branch/master?svg=true)](https://ci.appveyor.com/project/passy/cargo-sphinx)
[![Dependency Status](https://dependencyci.com/github/woofwoofinc/cargo-sphinx/badge)](https://dependencyci.com/github/woofwoofinc/cargo-sphinx)
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](https://github.com/woofwoofinc/cargo-sphinx#license)

Cargo subcommand for building and publishing Sphinx documentation. Includes
support for publishing to [GitHub Pages].

[GitHub Pages]: https://pages.github.com

Detailed documentation is provided in the [docs] directories and at
[woofwoofinc.github.io/cargo-sphinx].

[docs]: docs
[woofwoofinc.github.io/cargo-sphinx]: https://woofwoofinc.github.io/cargo-sphinx

Cargo Sphinx was originally forked from the [cargo-release] project by 
[Ning Sun].

[cargo-release]: https://github.com/sunng87/cargo-release
[Ning Sun]: https://github.com/sunng87


Installing
----------
Install Cargo Sphinx in a Git managed Cargo-based Rust project using:

    cargo install cargo-sphinx

Build the project Sphinx documentation using:

    cargo sphinx

To also push the documentation to the GitHub Pages branch include the `--push`
option:

    cargo sphinx --push

This will generate Sphinx documentation and commit it to the `gh-pages` branch
of the repository. If GitHub Pages are enabled on the repository then you will
be able to view your documentation at
https://YOUR-GITHUB-USERNAME.github.io/YOUR-REPOSITORY-NAME.

WARNING: This will override your existing `gh-pages` branch, use at your own
risk.

Include the `--dry-run` option to print all the commands to execute instead of
performing the generate and upload.

See further user documentation in the Sphinx documentation for this project
published at [woofwoofinc.github.io/cargo-sphinx].

[woofwoofinc.github.io/cargo-sphinx]: https://woofwoofinc.github.io/cargo-sphinx


Development
-----------
Install the [Rust] development tools on your system with [rustup] if they are
not already available. Then build and test the project using:

    cargo test

[Rust]: https://www.rust-lang.org
[rustup]: https://www.rustup.rs

Install a development version of the plugin locally from latest source using:

    cargo install
    
(`--force` is necessary if Cargo Sphinx is already installed.)

Then test with a dry run:

    cargo sphinx --push --dry-run

See further development documentation in the Sphinx documentation for this
project published at [woofwoofinc.github.io/cargo-sphinx].

[woofwoofinc.github.io/cargo-sphinx]: https://woofwoofinc.github.io/cargo-sphinx


License
-------
This work is dual-licensed under the Apache License, Version 2.0 and under the
MIT Licence.

You may license this work under the Apache License, Version 2.0.

    Copyright 2016 Ning Sun and tojson_macros contributors

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.

Alternatively, you may license this work under the MIT Licence at your option.

    Copyright (c) 2016 Ning Sun and tojson_macros contributors

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.

The license explainers at [Choose a License] may be helpful. They have
descriptions for both the [Apache 2.0 Licence] and [MIT Licence] conditions.

[Choose a License]: http://choosealicense.com
[Apache 2.0 Licence]: http://choosealicense.com/licenses/apache-2.0/
[MIT Licence]: http://choosealicense.com/licenses/mit/


Contributing
------------
Please note that this project is released with a [Contributor Code of Conduct].
By participating in this project you agree to abide by its terms. Instances of
abusive, harassing, or otherwise unacceptable behavior may be reported by
contacting the project team at woofwoofinc@gmail.com.

[Contributor Code of Conduct]: docs/conduct.rst

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
