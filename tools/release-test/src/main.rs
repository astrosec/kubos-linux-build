/*
 * Copyright (C) 2019 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * 10/11/19 FBP Added whitespace to kickoff source scan  
 */

use app_service::*;
use failure::Error;
use kubos_app::*;
use monitor_service::*;
use std::time::Duration;
use telem_service::*;

const TELEMFILE: &str = "/home/kubos/release-test/telem-results";

mod app_service;
mod monitor_service;
mod telem_service;

fn main() -> Result<(), Error> {
    logging_setup!("release-test", log::LevelFilter::Info)?;
    
    // Monitor Service Tests
    monitor_test()?;
        
    // Telemetry Service Tests
    telemetry_test()?;

    // App Service Tests
    apps_test()?;
    
    Ok(())
}
