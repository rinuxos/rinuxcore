//
// MIT License
//
// Copyright (c) 2022 AtomicGamer9523
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

//! Custom configuration for the kernel.

#[doc(hidden)]
mod file;

/// Specifies the location to load the configuration from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[unstable(feature = "rinuxcore_custom_config", issue = "none")]
pub enum ConfigType {
    /// Loads the configuration from a file.
    File,
    /// Loads the configuration from the environment.
    UserDefined(Config),
}

/// A Struct used for serializing the project's configuration.
#[unstable(feature = "rinuxcore_custom_config", issue = "none")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Config {
    /// The name of the project.
    pub project_name: &'static str,

    /// The version of the project.
    pub project_version: &'static str,

    /// If the project should boot quietly.
    /// Set to false if you are building a library.
    pub quiet_boot: bool,
}

#[unstable(feature = "rinuxcore_custom_config", issue = "none")]
impl Config {
    /// Creates a new constant empty mutable config
    pub const fn cnst() -> Self {
        Config {
            project_name: "",
            project_version: "",
            quiet_boot: false,
        }
    }

    /// Creates a new constant mutable config
    pub const fn new(
        project_name: &'static str,
        project_version: &'static str,
        quiet_boot: bool,
    ) -> Self {
        Config {
            project_name,
            project_version,
            quiet_boot,
        }
    }

    #[unstable(feature = "rinuxcore_custom_config", issue = "none")]
    pub(crate) fn get_config(self, config_type: ConfigType) -> Self {
        match config_type {
            ConfigType::File => { unsafe {
                Config {
                    project_name: file::PROJECT_NAME,
                    project_version: file::PROJECT_VERSION,
                    quiet_boot: file::QUIET_BOOT,
                }
            }},
            ConfigType::UserDefined(data) => data
        }
    }
}
