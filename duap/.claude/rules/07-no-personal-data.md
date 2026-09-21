# No personal data in this repository

No real personal data appears anywhere: not in tests, fixtures, examples,
documentation, benchmark inputs, issue text or commit messages.

## Requirements

1. All fixtures are synthetic and recognisably so. Synthetic subjects are
   pseudonyms derived from literal seeds; synthetic organisations use the
   `org:duap/` authority with obviously fictional names.
2. Any string standing in for personal data is prefixed `synthetic:`.
3. No real email address, telephone number, postal address, national
   identifier, payment instrument, IP address or device identifier, including
   in a comment or a test name.
4. If a real dataset is ever needed for a research experiment, it requires a
   recorded legal basis, is referenced rather than committed, and the
   experiment states the basis in its write-up.
5. Log output in tests and examples is checked against the same rule.

## Checking

`.claude/hooks/check-no-pii.sh` scans staged changes for patterns that look
like real identifiers and blocks the commit. It is a coarse filter and
catches the accidental case; it cannot catch a determined one, so the rule
is also a review responsibility.
