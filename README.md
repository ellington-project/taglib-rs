# Rust bindings to taglib

[![Build Status](https://travis-ci.org/AdamHarries/taglib-rs.svg?branch=master)](https://travis-ci.org/AdamHarries/taglib-rs)

**NOTE:** This repository is currently designed to circumvent some limitations with the default taglib behaviour/interface. To this end, it includes a fork of taglib as a submodule, and so must either be cloned with the `--recursive` argument (i.e. `git clone --recursive <repo>`), or the submodule must be initialised after this repo is cloned using `git submodule update --init --recursive`. Adding the ability for end users to actively configure this is a feature that is planned for the future. At present, however, using any other version of taglib will result in compile errors at the `tag_c` interface.
