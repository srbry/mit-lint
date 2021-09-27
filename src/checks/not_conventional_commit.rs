use indoc::indoc;
use mit_commit::CommitMessage;

use crate::model::{Code, Problem};

/// Canonical lint ID
pub(crate) const CONFIG: &str = "not-conventional-commit";

/// Advice on how to correct the problem
const HELP_MESSAGE: &str = indoc!(
    "
    It's important to follow the conventional commit style when creating your commit message. By \
    using this style we can automatically calculate the version of software using deployment \
    pipelines, and also generate changelogs and other useful information without human interaction.

    You can fix it by following style

    <type>[optional scope]: <description>

    [optional body]

    [optional footer(s)]

    You can read more at https://www.conventionalcommits.org/"
);
/// Description of the problem
const ERROR: &str = "Your commit message isn't in conventional style";

lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new("^[^()\\s]+(\\(\\w+\\))?!?: ").unwrap();
}

fn has_problem(commit_message: &CommitMessage) -> bool {
    let subject: String = commit_message.get_subject().into();

    !RE.is_match(&subject)
}

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if has_problem(commit_message) {
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::NotConventionalCommit,
            commit_message,
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::wildcard_imports)]

    use indoc::indoc;

    use super::*;
    use crate::model::Code;

    // Examples from https://www.conventionalcommits.org/en/v1.0.0/

    #[test]
    fn commit_message_with_description_and_breaking_change_footer() {
        test_subject_not_separate_from_body(
            indoc!(
                "
                feat: allow provided config object to extend other configs

                BREAKING CHANGE: `extends` key in config file is now used for extending other \
                 config files
                "
            ),
            &None,
        );
    }

    #[test]
    fn commit_message_with_bang_to_draw_attention_to_breaking_change() {
        test_subject_not_separate_from_body(
            indoc!(
                "
                refactor!: drop support for Node 6
                "
            ),
            &None,
        );
    }

    #[test]
    fn commit_message_with_both_bang_and_breaking_change_footer() {
        test_subject_not_separate_from_body(
            indoc!(
                "
                refactor!: drop support for Node 6

                BREAKING CHANGE: refactor to use JavaScript features not available in Node 6.
                "
            ),
            &None,
        );
    }

    #[test]
    fn commit_message_with_no_body() {
        test_subject_not_separate_from_body(
            indoc!(
                "
                docs: correct spelling of CHANGELOG
                "
            ),
            &None,
        );
    }

    #[test]
    fn commit_message_with_scope() {
        test_subject_not_separate_from_body(
            indoc!(
                "
                feat(lang): add polish language
                "
            ),
            &None,
        );
    }

    #[test]
    fn commit_message_with_multi_paragraph_body_and_multiple_footers() {
        test_subject_not_separate_from_body(
            indoc!(
                "
                fix: correct minor typos in code

                see the issue for details

                on typos fixed.

                Reviewed-by: Z
                Refs #133
                "
            ),
            &None,
        );
    }

    #[test]
    fn revert_example() {
        test_subject_not_separate_from_body(
            indoc!(
                "
                revert: let us never again speak of the noodle incident

                Refs: 676104e, a215868
                "
            ),
            &None,
        );
    }

    #[test]
    fn non_conventional() {
        let message = indoc!(
            "
                An example commit

                This is an example commit
                "
        );
        test_subject_not_separate_from_body(
            message,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotConventionalCommit,
                &message.into(),
            )),
        );
    }

    #[test]
    fn missing_bracket() {
        let message = indoc!(
            "
                fix(example: An example commit

                This is an example commit
                "
        );
        test_subject_not_separate_from_body(
            message,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotConventionalCommit,
                &message.into(),
            )),
        );
    }

    #[test]
    fn missing_space() {
        let message = indoc!(
            "
                fix(example):An example commit

                This is an example commit
                "
        );
        test_subject_not_separate_from_body(
            message,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotConventionalCommit,
                &message.into(),
            )),
        );
    }

    fn test_subject_not_separate_from_body(message: &str, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }
}
