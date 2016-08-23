/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Runtime switches for the style system.

pub struct Opts {
    pub disable_share_style_cache: bool,
    pub style_sharing_stats: bool,
    pub nonincremental_layout: bool,
}

impl Opts {
    pub fn new() -> Opts {
        Opts {
            disable_share_style_cache: false,
            style_sharing_stats: false,
            nonincremental_layout: false,
        }
    }
}
