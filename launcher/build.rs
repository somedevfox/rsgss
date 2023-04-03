/*
   Kitsune Standard License Version 1.0

   Copyright (c) 2023, Poleshko Egor Ivanovich, all rights reserved.

   Redistribution and use in source and binary forms, with or without modification,
   are permitted provided that the following conditions are met:
       1. Redistributions of source code must retain the above copyright notice,
           this list of conditions and the following disclaimer.
       2. All advertising materials mentioning features or use of this Software must
           display the following acknowledgement: This product includes software developed
           by Poleshko Egor Ivanovich.
       3. Redistributions in binary form must reproduce the above copyright notice or name of
           this Software ("rsgss") or trademark on the start up sequence of the distribution,
           unless waiver was granted by specific prior written permission.
       4. Redistributions in binary form must reproduce the above copyright notice, this list of
           conditions and the following disclaimer in the documentation and/or other materials
           provided with the distribution.
       5. Neither the name of the Poleshko Egor Ivanovich nor the names of it's contributors
           may be used to endorse or promote products derived from this software without
           specific prior written permission.
       6. Redistributions in source form must be made publicly available. This does not apply to
           any other software linked with the distribution.
       7. Redistributions in source and binary forms must state changes made to the Software.
       8. Redistributions in binary form must include the instructions on how to install
           and build the distribution.

   THIS SOFTWARE IS PROVIDED BY Poleshko Egor Ivanovich "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
   INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
   PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL  COPYRIGHT HOLDER BE LIABLE FOR ANY
   DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
   LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
   BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
   STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
   OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
use std::env;

fn main() {
    if Ok("windows") == env::var("CARGO_CFG_TARGET_OS").as_deref() {
        embed_manifest::embed_manifest_file("rsgss.manifest.xml").unwrap();
    }

    println!("cargo:rerun-if-changed=rsgss.manifest.xml");
}
