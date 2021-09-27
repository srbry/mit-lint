use indoc::indoc;
use mit_commit::CommitMessage;

use crate::model::{Code, Problem};

/// Canonical lint ID
pub(crate) const CONFIG: &str = "github-id-missing";

/// Advice on how to correct the problem
const HELP_MESSAGE: &str = indoc!(
    "
    It's important to add the issue ID because it allows us to link code back to the motivations \
     for doing it, and because we can help people exploring the repository link their issues to \
     specific bits of code.

    You can fix this by adding a ID like the following examples:

    #642
    GH-642
    AnUser/git-mit#642
    AnOrganisation/git-mit#642
    fixes #642

    Be careful just putting '#642' on a line by itself, as '#' is the default comment character"
);
/// Description of the problem
const ERROR: &str = "Your commit message is missing a GitHub ID";

lazy_static! {
    static ref RE: regex::Regex =
        regex::Regex::new(r"(?m)(^| )([a-zA-Z0-9_-]{3,39}/[a-zA-Z0-9-]+#|GH-|#)[0-9]+( |$)")
            .unwrap();
}

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if commit_message.matches_pattern(&*RE) {
        None
    } else {
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::GitHubIdMissing,
            commit_message,
        ))
    }
}

#[cfg(test)]
mod tests_has_missing_github_id {
    #![allow(clippy::wildcard_imports)]

    use indoc::indoc;

    use super::*;
    use crate::model::{Code, Problem};

    #[test]
    fn id_and_close() {
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                close #642
                "
            ),
            &None,
        );
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                closes: #642
                "
            ),
            &None,
        );
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                Closed GH-642
                "
            ),
            &None,
        );
    }

    #[test]
    fn id_and_fix() {
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                fix #642
                "
            ),
            &None,
        );
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                This fixes #642
                "
            ),
            &None,
        );
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                fixed #642
                "
            ),
            &None,
        );
    }

    #[test]
    fn id_and_resolve() {
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                resolve #642
                "
            ),
            &None,
        );
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                resolves #642
                "
            ),
            &None,
        );
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                resolved #642
                "
            ),
            &None,
        );
    }

    #[test]
    fn issue() {
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                Issue #642
                "
            ),
            &None,
        );
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                Issue #642
                "
            ),
            &None,
        );
    }

    #[test]
    fn gh_id_variant() {
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                GH-642
                "
            ),
            &None,
        );
    }

    #[test]
    fn hash_alone_variant() {
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                #642
                ; Comment character is set to something else like ';'
                "
            ),
            &None,
        );
    }

    #[test]
    fn long_variant() {
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                AnUser/git-mit#642
                "
            ),
            &None,
        );

        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                AnOrganisation/git-mit#642
                "
            ),
            &None,
        );
    }

    #[test]
    fn id_missing() {
        let message = indoc!(
            "
                An example commit

                This is an example commit
                "
        );
        test_has_missing_github_id(
            message,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::GitHubIdMissing,
                &message.into(),
            )),
        );
    }

    #[test]
    fn id_malformed() {
        let message_1 = indoc!(
            "
                An example commit

                This is an example commit

                H-123
                "
        );
        test_has_missing_github_id(
            message_1,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::GitHubIdMissing,
                &message_1.into(),
            )),
        );
        let message_2 = indoc!(
            "
                An example commit

                This is an example commit

                git-mit#123
                "
        );
        test_has_missing_github_id(
            message_2,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::GitHubIdMissing,
                &message_2.into(),
            )),
        );
    }

    fn test_has_missing_github_id(message: &str, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }
}
