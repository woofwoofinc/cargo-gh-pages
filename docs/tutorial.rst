Tutorial
========

1 Install Cargo Sphinx
----------------------
If not already available, start by installing the Cargo Sphinx extension.

::

    cargo install cargo-sphinx


2 Make a New Cargo Project
--------------------------
Begin in a new cargo project.

::

    cargo new cargo-sphinx-tutorial
    cd cargo-sphinx-tutorial

There will be a ``Cargo.toml`` and skeleton Rust ``src`` directory. We are going
to add a new Sphinx project at ``docs`` to document this Rust project.


3 Sphinx QuickStart
-------------------
Use the sphinx-quickstart tool to get started with Sphinx.

.. CAUTION::
   By default, Cargo Sphinx assumes the Sphinx files are located in ``docs``.
   This can be overridden by configuration but it's easier to use the default
   if we can. This documentation directory path must be provided to the
   sphinx-quickstart command.

::

    $ sphinx-quickstart docs
    Welcome to the Sphinx 1.8.5 quickstart utility.

    Please enter values for the following settings (just press Enter to
    accept a default value, if one is given in brackets).

    Selected root path: docs

The tool now asks a series of questions and configures a Sphinx layout and
project for the answers.

Cargo Sphinx current assumes there is no separate build directory for Sphinx
output, i.e. output is to ``_build`` in the same directory as the source
documentation files.

Answer ``n`` to separate source and build directories when asked.

::

    You have two options for placing the build directory for Sphinx output.
    Either, you use a directory "_build" within the root path, or you separate
    "source" and "build" directories within the root path.
    > Separate source and build directories (y/n) [n]: n

This is followed by a series of questions that don't matter for Cargo Sphinx,
complete them as you prefer.

::

    Inside the root directory, two more directories will be created; "_templates"
    for custom HTML templates and "_static" for custom stylesheets and other static
    files. You can enter another prefix (such as ".") to replace the underscore.
    > Name prefix for templates and static dir [_]:

    The project name will occur in several places in the built documentation.
    > Project name: My Cargo Sphinx Project
    > Author name(s): Me!
    > Project release []: 1.0

    If the documents are to be written in a language other than English,
    you can select a language here by its language code. Sphinx will then
    translate text that it generates into that language.

    For a list of supported codes, see
    http://sphinx-doc.org/config.html#confval-language.
    > Project language [en]:

    The file name suffix for source files. Commonly, this is either ".txt"
    or ".rst".  Only files with this suffix are considered documents.
    > Source file suffix [.rst]:

The Sphinx quickstart asks about whether to include a number of extensions.
Mostly these depend on whether you will use the functionality they support.

It is a good idea to include the githubpages extension even though Cargo Sphinx
provides support for generating ``.nojekyll`` files also. These are needed so
the GitHub Pages Jekyll processor doesn't delete files needed by the Sphinx
output. (githubpages is the last extension in the block below.)

::

    Indicate which of the following Sphinx extensions should be enabled:
    > autodoc: automatically insert docstrings from modules (y/n) [n]:
    > doctest: automatically test code snippets in doctest blocks (y/n) [n]:
    > intersphinx: link between Sphinx documentation of different projects (y/n) [n]:
    > todo: write "todo" entries that can be shown or hidden on build (y/n) [n]:
    > coverage: checks for documentation coverage (y/n) [n]:
    > imgmath: include math, rendered as PNG or SVG images (y/n) [n]:
    > mathjax: include math, rendered in the browser by MathJax (y/n) [n]:
    > ifconfig: conditional inclusion of content based on config values (y/n) [n]:
    > viewcode: include links to the source code of documented Python objects (y/n) [n]:
    > githubpages: create .nojekyll file to publish the document on GitHub pages (y/n) [n]: y

Next, Sphinx quickstart asks whether a Makefile should be generated. Usually
this is a convenience but is necessary in the case of Cargo Sphinx.

Answer yes to creating a Makefile. The Windows command file is up to you.

::

    A Makefile and a Windows command file can be generated for you so that you
    only have to run e.g. `make html' instead of invoking sphinx-build
    directly.
    > Create Makefile? (y/n) [y]: y
    > Create Windows command file? (y/n) [y]:

Finally, Sphinx quickstart generates the Sphinx source files under ``docs``.

::

    Creating file docs/conf.py.
    Creating file docs/index.rst.
    Creating file docs/Makefile.
    Creating file docs/make.bat.

    Finished: An initial directory structure has been created.

    You should now populate your master file docs/index.rst and create other documentation
    source files. Use the Makefile to build the docs, like so:
       make builder
    where "builder" is one of the supported builders, e.g. html, latex or linkcheck.


4 Run Cargo Sphinx
------------------
We are now ready to run Cargo Sphinx.

::

    cargo sphinx

The generated site can be inspected by loading ``docs/_build/html/index.html``
in a browser.

You are now all setup and ready to document!


5 Find Out More
---------------
To find out more about running Cargo Sphinx, and in particular about uploading
the generated output to GitHub Pages, see the section on
:ref:`running` Cargo Sphinx.

Find out more about Sphinx and writing RestructuredText at the following:

* `Sphinx Tutorial <http://www.sphinx-doc.org/en/stable/tutorial.html>`_
* `Sphinx and RestructuredText Cheatsheet <http://openalea.gforge.inria.fr/doc/openalea/doc/_build/html/source/sphinx/rest_syntax.html>`_
* `Quick Guide to RestructuredText <http://docutils.sourceforge.net/docs/user/rst/quickref.html>`_
* `RestructuredText Primer <http://sphinx-doc.org/rest.html>`_


6 Help Make Cargo Sphinx Better
-------------------------------
We'd love you to contribute to Cargo Sphinx. We value questions, feedback,
suggestions, bug reports, and patches on our `GitHub Issues`_. This is a small
and friendly project that welcomes all contributions.

.. _GitHub Issues: https://github.com/woofwoofinc/cargo-sphinx/issues

See :ref:`contributing` for particulars about the community guidelines.

Enjoy documenting with Cargo Sphinx!
