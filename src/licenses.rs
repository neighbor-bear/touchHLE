/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
/// Prints copyright, authorship and license information.

const MAIN_COPYRIGHT: &str = "
touchHLE © 2023 hikari_no_yume and other contributors.
";

const MAIN_CAVEAT: &str = "
The following authorship, copyright and license information relates to this
touchHLE executable. Please note that different licensing terms apply to source
files and to the bundled dynamic libraries (in touchHLE_dylibs/) and fonts (in
touchHLE_fonts/). Please consult the respective files/directories for more
information.
";

const MAIN_LICENSE: &str = "
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
";

const RUST_DESCRIPTION: &str = "
touchHLE, and therefore this executable, incorporates the following Rust
libraries, which are copyright their respective authors and other contributors,
and licensed as follows:
";

/// Names, versions, authors and licenses of Rust crates depended on by release
/// binaries. See `build.rs` for how this is generated.
const RUST_DEPENDENCIES: &str = include_str!(concat!(env!("OUT_DIR"), "/rust_dependencies.txt"));

const DYNARMIC_DESCRIPTION: &str = "
touchHLE, and therefore this executable, incorporates the library Dynarmic:
";

/// Copyright, authorship and license information for Dynarmic. See `build.rs`
/// for how this is generated.
const DYNARMIC_LICENSE: &str = include_str!(concat!(env!("OUT_DIR"), "/dynarmic_license.txt"));

const DYNARMIC_BOOST_DESCRIPTION: &str = "
Dynarmic, and therefore touchHLE, and therefore this executable, incorporates
the library Boost, which is available under the following license:
";

const DYNARMIC_BOOST_LICENSE: &str = "
Boost Software License - Version 1.0 - August 17th, 2003

Permission is hereby granted, free of charge, to any person or organization
obtaining a copy of the software and accompanying documentation covered by
this license (the \"Software\") to use, reproduce, display, distribute,
execute, and transmit the Software, and to prepare derivative works of the
Software, and to permit third-parties to whom the Software is furnished to
do so, all subject to the following:

The copyright notices in the Software and this entire statement, including
the above license grant, this restriction and the following disclaimer,
must be included in all copies of the Software, in whole or in part, and
all derivative works of the Software, unless such copies or derivative
works are solely in the form of machine-executable object code generated by
a source language processor.

THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE, TITLE AND NON-INFRINGEMENT. IN NO EVENT
SHALL THE COPYRIGHT HOLDERS OR ANYONE DISTRIBUTING THE SOFTWARE BE LIABLE
FOR ANY DAMAGES OR OTHER LIABILITY, WHETHER IN CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
";

const SDL2_DESCRIPTION: &str = "
touchHLE, and therefore this executable, incorporates the library SDL 2, which
is available under the following license:
";

const SDL2_LICENSE: &str = "
This software is provided 'as-is', without any express or implied
warranty.  In no event will the authors be held liable for any damages
arising from the use of this software.

Permission is granted to anyone to use this software for any purpose,
including commercial applications, and to alter it and redistribute it
freely, subject to the following restrictions:

1. The origin of this software must not be misrepresented; you must not
   claim that you wrote the original software. If you use this software
   in a product, an acknowledgment in the product documentation would be
   appreciated but is not required.
2. Altered source versions must be plainly marked as such, and must not be
   misrepresented as being the original software.
3. This notice may not be removed or altered from any source distribution.
";

const OPENAL_SOFT: &str = "
touchHLE, and therefore this executable, incorporates the library OpenAL Soft,
which is available under the terms of the GNU Library Public License version 2
or any later version.
";

const STB_IMAGE: &str = "
touchHLE, and therefore this executable, incorporates the library stb_image,
which is available either as Public Domain or under the terms of the MIT
license.
";

const DR_MP3: &str = "
touchHLE, and therefore this executable, incorporates the library dr_mp3,
which is available either under The Unlicense (which is a public domain
dedication) or under the terms of the MIT license. dr_mp3 is in turn derived
from the library minimp3, which is licensed under the Creative Commons CC0 1.0
Universal Public Domain Dedication.
";

pub fn divider() {
    echo!("---");
}

pub fn print() {
    echo!("{}", MAIN_CAVEAT);
    divider();
    echo!("{}", MAIN_COPYRIGHT);
    divider();
    echo!("{}", MAIN_LICENSE);
    divider();
    echo!("{}", RUST_DESCRIPTION);
    echo!("{}", RUST_DEPENDENCIES);
    divider();
    echo!("{}", DYNARMIC_DESCRIPTION);
    echo!("{}", DYNARMIC_LICENSE);
    divider();
    echo!("{}", DYNARMIC_BOOST_DESCRIPTION);
    echo!("{}", DYNARMIC_BOOST_LICENSE);
    divider();
    echo!("{}", SDL2_DESCRIPTION);
    echo!("{}", SDL2_LICENSE);
    divider();
    echo!("{}", OPENAL_SOFT);
    divider();
    echo!("{}", STB_IMAGE);
    divider();
    echo!("{}", DR_MP3);
}
