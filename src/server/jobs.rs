//! Builds the requests of a translation job. Running them is up to the screen,
//! which streams each reply into its pages.

use crate::{
    client::{SharedHistory, shared_history},
    error::{Error, Result},
    page::{Page, Section},
    server::{Method, Server},
};
use gpui_kit::SharedString;
use rig::{Agent, message::Message};

/// One job's requests, ready to stream.
pub struct TranslationRun {
    pub agent: Agent,
    /// The conversation the agent loads and saves, in History mode.
    pub history: Option<SharedHistory>,
    /// Earlier translations to seed `history` with before the first part.
    pub seed: Vec<Message>,
    /// The parts to translate, by index, with their Japanese text.
    pub parts: Vec<(usize, SharedString)>,
}

impl Server {
    /// Translates every part of the last of `pages`. A `carried` history, from
    /// the page before, is continued instead of seeding a new one.
    pub fn translate(
        &self,
        pages: &[Page],
        model: &str,
        glossary: Option<&str>,
        carried: Option<SharedHistory>,
    ) -> Result<TranslationRun> {
        let (current, pages) = pages.split_last().ok_or(Error::Error("No such page"))?;
        let (seed, history) = match carried.filter(|_| self.method == Method::History) {
            Some(history) => (Vec::new(), Some(history)),
            None => self.seeded_history(newest_first(pages)),
        };
        let agent =
            self.client
                .translation_agent(model, &self.settings, history.clone(), glossary)?;
        let parts = current
            .sections
            .iter()
            .map(|section| section.japanese.clone())
            .enumerate()
            .collect();

        Ok(TranslationRun {
            agent,
            history,
            seed,
            parts,
        })
    }

    /// Translates one part of the last of `pages`.
    pub fn translate_part(
        &self,
        pages: &[Page],
        model: &str,
        part: usize,
        glossary: Option<&str>,
    ) -> Result<TranslationRun> {
        let (current, pages) = pages.split_last().ok_or(Error::Error("No such page"))?;
        let (section, before) = current
            .sections
            .get(..=part)
            .and_then(<[_]>::split_last)
            .ok_or(Error::Error("No such part"))?;
        let recent = before.iter().rev().chain(newest_first(pages));
        let (seed, history) = self.seeded_history(recent);
        let agent =
            self.client
                .translation_agent(model, &self.settings, history.clone(), glossary)?;

        Ok(TranslationRun {
            agent,
            history,
            seed,
            parts: vec![(part, section.japanese.clone())],
        })
    }

    /// In History mode, a new shared history and the messages to seed it with
    /// from `recent` (sections ordered newest first).
    fn seeded_history<'a>(
        &self,
        recent: impl Iterator<Item = &'a Section>,
    ) -> (Vec<Message>, Option<SharedHistory>) {
        if self.method != Method::History {
            return (Vec::new(), None);
        }
        let context_window = self.settings.context_window;
        let messages = build_history(recent, context_window);
        (messages, Some(shared_history(context_window)))
    }
}

/// Sections of `pages` ordered newest first.
fn newest_first(pages: &[Page]) -> impl Iterator<Item = &Section> {
    pages.iter().rev().flat_map(|p| p.sections.iter().rev())
}

/// Builds history messages from the latest `context_window` translated sections,
/// given `recent` ordered newest first. Stops walking once the window is full.
fn build_history<'a>(
    recent: impl Iterator<Item = &'a Section>,
    context_window: usize,
) -> Vec<Message> {
    let recent: Vec<_> = recent
        .filter(|s| !s.content.is_empty())
        .take(context_window)
        .collect();

    recent
        .into_iter()
        .rev()
        .flat_map(Section::history_message)
        .collect()
}
