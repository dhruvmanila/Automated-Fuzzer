use crate::broken_files::{create_broken_files, LANGS};
use std::process::{Child, Command, Stdio};

use crate::obj::ProgramConfig;
use crate::settings::Setting;

pub struct StaticCheckGoStruct {
    pub settings: Setting,
}

impl ProgramConfig for StaticCheckGoStruct {
    fn is_broken(&self, content: &str) -> bool {
        content.contains("internal error")
            || content.contains("panic:")
            || content.contains("fatal error:")
    }
    fn get_run_command(&self, full_name: &str) -> Child {
        Command::new(&self.settings.app_binary)
            .env("PATH", "/usr/local/go/bin")
            .arg("-checks")
            .arg("S1000,S1001,S1002,S1003,S1004,S1005,S1006,S1007,S1008,S1009,S1010,S1011,S1012,S1016,S1017,S1018,S1019,S1020,S1021,S1023,S1024,S1025,S1028,S1029,S1030,S1031,S1032,S1033,S1034,S1035,S1036,S1037,S1038,S1039,S1040,SA1000,SA1001,SA1002,SA1003,SA1004,SA1005,SA1006,SA1007,SA1008,SA1010,SA1011,SA1012,SA1013,SA1014,SA1015,SA1016,SA1017,SA1018,SA1019,SA1020,SA1021,SA1023,SA1024,SA1025,SA1026,SA1027,SA1028,SA1029,SA1030,SA2000,SA2001,SA2002,SA2003,SA3000,SA3001,SA4000,SA4001,SA4003,SA4004,SA4005,SA4006,SA4008,SA4009,SA4010,SA4011,SA4012,SA4013,SA4014,SA4015,SA4016,SA4017,SA4018,SA4019,SA4020,SA4021,SA4022,SA4023,SA4024,SA4025,SA4026,SA4027,SA4028,SA4029,SA4030,SA4031,SA5000,SA5001,SA5002,SA5003,SA5004,SA5005,SA5007,SA5008,SA5009,SA5010,SA5011,SA5012,SA6000,SA6001,SA6002,SA6003,SA6005,SA9001,SA9002,SA9003,SA9004,SA9005,SA9006,SA9007,SA9008,ST1000,ST1001,ST1003,ST1005,ST1006,ST1008,ST1011,ST1012,ST1013,ST1015,ST1016,ST1017,ST1018,ST1019,ST1020,ST1021,ST1022,ST1023,U1000
")
            .arg(full_name)
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
    }
    fn broken_file_creator(&self) -> Child {
        create_broken_files(self, LANGS::GO)
    }
    fn get_settings(&self) -> &Setting {
        &self.settings
    }
}
