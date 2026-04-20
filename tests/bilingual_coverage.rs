//! Bilingual coverage smoke test.
//!
//! For each language-sensitive rule, exercise a minimal EN trigger and a
//! minimal FR trigger through the full engine pipeline and assert the
//! rule fires for both. Guards against regressions where a rule is
//! added with an EN word list only (or vice versa).
//!
//! Rules not covered here are either language-neutral (structural /
//! punctuation / length) or intentionally single-language in v0.1
//! (`jargon-undefined` TECH list is EN-only — tracked in the roadmap).

use lucid_lint::{Engine, Profile};

fn assert_fires(rule_id: &str, text: &str) {
    let engine = Engine::with_profile(Profile::Public);
    let diagnostics = engine.lint_str(text);
    assert!(
        diagnostics.iter().any(|d| d.rule_id == rule_id),
        "{rule_id} did not fire on input: {text:?}\ngot: {:?}",
        diagnostics
            .iter()
            .map(|d| d.rule_id.as_str())
            .collect::<Vec<_>>()
    );
}

#[test]
fn weasel_words_fires_in_en_and_fr() {
    assert_fires(
        "weasel-words",
        "There are various issues with several items that may be relevant.",
    );
    assert_fires(
        "weasel-words",
        "Il existe divers problèmes avec certains éléments qui peuvent être pertinents.",
    );
}

#[test]
fn repetitive_connectors_fires_in_en_and_fr() {
    // Public defaults: max_per_window = 3, window_size = 5 — need 4 hits
    // in 5 consecutive sentences.
    assert_fires(
        "repetitive-connectors",
        "The cache was slow. However the numbers looked good. However the latency \
         regressed. However the team shipped it anyway. However the rollback plan \
         held up.",
    );
    assert_fires(
        "repetitive-connectors",
        "Le cache était lent. Cependant les chiffres paraissaient bons. Cependant la \
         latence a régressé. Cependant l'équipe a tout de même livré. Cependant le \
         plan de retour arrière a tenu bon.",
    );
}

#[test]
fn passive_voice_fires_in_en_and_fr() {
    assert_fires(
        "passive-voice",
        "The report was written by the team. The bug was fixed by the author. The \
         release was approved by the board.",
    );
    assert_fires(
        "passive-voice",
        "Le rapport a été écrit par l'équipe. Le bug a été corrigé par l'auteur. La \
         version a été approuvée par le conseil.",
    );
}

#[test]
fn unclear_antecedent_fires_in_en_and_fr() {
    // Detection threshold is 10 words; keep fixtures long enough to be
    // classified rather than returning Unknown.
    assert_fires(
        "unclear-antecedent",
        "The cache was slow and the latency had been climbing for weeks. This \
         motivated the change we shipped last Friday.",
    );
    assert_fires(
        "unclear-antecedent",
        "Les performances étaient médiocres et la latence augmentait depuis des \
         semaines. Cela a motivé le changement que nous avons livré.",
    );
}

#[test]
fn excessive_nominalization_fires_in_en_and_fr() {
    assert_fires(
        "excessive-nominalization",
        "The implementation of the optimization caused a regression in the evaluation \
         of the configuration.",
    );
    assert_fires(
        "excessive-nominalization",
        "L'implémentation de l'optimisation a causé une régression dans l'évaluation \
         de la configuration.",
    );
}
