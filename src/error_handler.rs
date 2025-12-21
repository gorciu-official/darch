use crate::printer::print_error;

pub enum Error {
    PackageEmptyString,
    ShellEmptyString,
}

pub trait ErrorHandler {
    fn handle(&self, error: &Error) {
        match error {
            Error::PackageEmptyString => self.package_empty_string(),
            Error::ShellEmptyString => self.shell_empty_string(),
        }
    }

    fn package_empty_string(&self) {
        print_error("Package list contains empty string", None);
        panic!()
    }
    fn shell_empty_string(&self) {
        print_error("Shell list contains empty string", None);
        panic!()
    }
}
