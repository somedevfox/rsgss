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
use figment::{
    providers::{Env, Format, Json, Serialized, Toml},
    Figment,
};
use rsgss_graphics::wgpu::PowerPreference;
use std::process;

#[derive(Debug, Deserialize, Serialize)]
pub struct WindowConfiguration {
    pub title: String,
    pub width: u32,
    pub height: u32,
}
impl Default for WindowConfiguration {
    fn default() -> Self {
        Self {
            title: String::from("Game"),
            width: 640,
            height: 480,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(remote = "PowerPreference")]
enum PowerPreferenceDef {
    LowPower,
    HighPerformance,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RendererConfiguration {
    #[serde(with = "PowerPreferenceDef")]
    pub power_preference: PowerPreference,
}
impl Default for RendererConfiguration {
    fn default() -> Self {
        Self {
            power_preference: PowerPreference::HighPerformance,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AppConfiguration {
    pub window: WindowConfiguration,
    pub renderer: RendererConfiguration,
}
impl AppConfiguration {
    pub fn get() -> Self {
        match Figment::from(Serialized::defaults(AppConfiguration::default()))
            .merge(Toml::file("rsgss.toml"))
            .merge(Env::prefixed("RS_"))
            .join(Json::file("rsgss.json"))
            .extract()
        {
            Ok(c) => c,
            Err(why) => {
                rfd::MessageDialog::new()
                    .set_title("rsgss")
                    .set_level(rfd::MessageLevel::Error)
                    .set_description(format!("Couldn't get configuration data: {why}").as_str())
                    .show();
                process::exit(1i32)
            }
        }
    }
}
