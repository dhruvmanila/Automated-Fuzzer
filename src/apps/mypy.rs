use crate::broken_files::create_broken_python_files;
use std::process::{Child, Command, Stdio};

use crate::obj::ProgramConfig;
use crate::settings::Setting;

pub struct MypyStruct {
    pub settings: Setting,
}

impl ProgramConfig for MypyStruct {
    fn is_broken(&self, content: &str) -> bool {
        content.contains("INTERNAL ERROR") || content.contains("Traceback")
    }

    fn get_run_command(&self, full_name: &str) -> Child {
        Command::new(&self.settings.app_binary)
            .arg(full_name)
            .args("--no-incremental --ignore-missing-imports --disallow-any-unimported --disallow-any-expr --disallow-any-decorated --disallow-any-explicit --disallow-any-generics --disallow-subclassing-any --disallow-untyped-calls --disallow-untyped-defs --disallow-incomplete-defs --check-untyped-defs --disallow-untyped-decorators --warn-redundant-casts --warn-unused-ignores --no-warn-no-return --warn-return-any --warn-unreachable --strict".split(' '))
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
    }
    fn broken_file_creator(&self) -> Child {
        create_broken_python_files(self)
    }
    fn get_settings(&self) -> &Setting {
        &self.settings
    }
}
