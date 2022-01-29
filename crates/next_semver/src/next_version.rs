use semver::Version;

use crate::VersionIncrement;

pub trait NextVersion {
    /// Analyze commits and determine the next version based on
    /// [conventional commits](https://www.conventionalcommits.org/) and
    /// [semantic versioning](https://semver.org/):
    /// - If no commits are passed, the version is unchanged.
    /// - If some commits are present, but none of them match conventional commits specification,
    ///   the version is incremented as a Patch.
    /// - If some commits match conventional commits, then the next version is calculated by using
    ///   [these](https://www.conventionalcommits.org/en/v1.0.0/#how-does-this-relate-to-semverare) rules.
    fn next<I>(&self, commits: I) -> Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>;
}

impl NextVersion for Version {
    fn next<I>(&self, commits: I) -> Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let increment = VersionIncrement::from_commits(self, commits);
        match increment {
            Some(increment) => increment.bump(self),
            None => self.clone(),
        }
    }
}
